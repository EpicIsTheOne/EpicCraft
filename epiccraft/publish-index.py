"""Merge a verified GitHub release into EpicCraft's per-app update index."""
import json
import pathlib
import subprocess
import sys

app, tag = sys.argv[1:]
release = json.loads(subprocess.check_output([
    "gh", "api", f"repos/EpicIsTheOne/EpicCraft/releases/tags/{tag}"
], text=True))
assets = [asset for asset in release["assets"] if asset["name"].startswith(app + "-")]
if release.get("draft") or not assets:
    raise SystemExit("Release must be published and contain matching app assets")
if any(not str(asset.get("digest", "")).startswith("sha256:") for asset in assets):
    raise SystemExit("Every app asset must have a GitHub SHA-256 digest")
index_path = pathlib.Path("epiccraft/releases.json")
index = json.loads(index_path.read_text(encoding="utf-8-sig"))
index[app] = release
index_path.write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")
