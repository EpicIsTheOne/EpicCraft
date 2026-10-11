import os,tempfile,hashlib,secrets,base64,io,importlib.util
from pathlib import Path
from fastapi.testclient import TestClient
from PIL import Image
os.environ['EPICCRAFT_DATA']=tempfile.mkdtemp()
os.environ['EPICCRAFT_ORIGIN']='http://testserver'
salt=secrets.token_bytes(16)
os.environ['EPICCRAFT_PASSWORD_HASH']=salt.hex()+':'+hashlib.scrypt(b'test-pass',salt=salt,n=16384,r=8,p=1).hex()
os.environ['EPICCRAFT_BRIDGE_TOKEN']='test-bridge'
spec=importlib.util.spec_from_file_location('server',Path('epiccraft/feedback/server.py'));mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
c=TestClient(mod.app);base='/epiccraft/api/'
assert c.get(base+'reports').status_code==401
assert c.post(base+'login',json={'password':'wrong'}).status_code==401
p=c.post(base+'login',json={'password':'test-pass'});assert p.status_code==200
csrf={'X-CSRF-Token':p.json()['csrf']}
f=io.BytesIO();Image.new('RGB',(10,10),'red').save(f,format='PNG')
payload={'app':'FilmCraft','kind':'bug','title':'Test report','body':'Test context','images':[base64.b64encode(f.getvalue()).decode()]}
r=c.post(base+'reports',json=payload);assert r.status_code==200,r.text
rid=r.json()['id'];receipt=r.json()['url'].split('/')[-1]
assert c.post(base+'context/'+receipt+'/complete',json={'resolution':'bad'}).status_code==403
assert c.patch(base+'reports/'+rid,json={'title':'edited','body':'context'}).status_code==403
assert c.patch(base+'reports/'+rid,headers=csrf,json={'title':'edited','body':'context'}).status_code==200
s=c.post(base+'reports/'+rid+'/share',headers=csrf,json={}).json()['url'].split('/')[-1]
context=c.get(base+'context/'+s).json();assert context['title']=='edited' and c.get(context['images'][0]).status_code==200
for _ in range(2):assert c.post(base+'reports/'+rid+'/dispatch',headers=csrf,json={}).status_code==200
worker={'Authorization':'Bearer test-bridge'}
j=c.post(base+'bridge/claim',headers=worker,json={}).json()['job'];assert j['id']==rid
assert c.post(base+'bridge/claim',headers=worker,json={}).json()['job'] is None
assert c.post(base+'context/'+s+'/complete',json={'resolution':'Verified test'}).status_code==200
assert c.get(base+'context/'+s).json()['status']=='done'
payload['images']=[base64.b64encode(b'not-image').decode()];assert c.post(base+'reports',json=payload).status_code==400
print('PASS: auth, CSRF, image validation, edits, share permissions, idempotent dispatch, leasing and completion')
