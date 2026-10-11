param([string]$Package)
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$install = Join-Path $env:LOCALAPPDATA 'Programs\EpicCraft Launcher'
$exe = Join-Path $install 'artcraft-launcher.exe'
if (Get-Process -Name artcraft-launcher -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $exe }) {
    throw 'Quit EpicCraft Launcher before installing a new copy.'
}
New-Item -ItemType Directory -Force $install | Out-Null
if ($Package) {
    $stage = Join-Path $env:TEMP ('epiccraft-install-' + [guid]::NewGuid().ToString('N'))
    Expand-Archive -LiteralPath $Package -DestinationPath $stage
    $binary = Get-ChildItem $stage -Recurse -Filter artcraft-launcher.exe | Select-Object -First 1
    if (-not $binary) { throw 'Package contains no launcher executable' }
    Copy-Item -LiteralPath $binary.FullName -Destination $exe
    Get-ChildItem -LiteralPath $binary.DirectoryName -File |
        Where-Object { $_.Name -match '^(LICENSE-|OFL-|ATTRIBUTION\.md)' } |
        Copy-Item -Destination $install
} else {
    Copy-Item -LiteralPath (Join-Path $root 'target\release\artcraft-launcher.exe') -Destination $exe
    Copy-Item (Join-Path $root 'LICENSE-MIT'), (Join-Path $root 'LICENSE-APACHE'), (Join-Path $root 'ATTRIBUTION.md') $install
    Copy-Item (Join-Path $root 'assets\fonts\OFL-*.txt') $install
    Copy-Item (Join-Path $root 'docs\brand\LICENSE-brand.txt') $install
}
New-Item -ItemType File -Force (Join-Path $install 'portable.txt') | Out-Null
$shell = New-Object -ComObject WScript.Shell
foreach ($link in @(
    (Join-Path ([Environment]::GetFolderPath('Desktop')) 'EpicCraft Launcher.lnk'),
    (Join-Path ([Environment]::GetFolderPath('Programs')) 'EpicCraft Launcher.lnk')
)) {
    $shortcut = $shell.CreateShortcut($link)
    $shortcut.TargetPath = $exe
    $shortcut.WorkingDirectory = $install
    $shortcut.Description = 'EpicCraft: install, update, and open the Craft App Suite'
    $shortcut.Save()
}
New-ItemProperty -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run' -Name 'EpicCraft Launcher' -Value ('"' + $exe + '" --hidden') -PropertyType String -Force | Out-Null
Write-Output "Installed EpicCraft Launcher: $exe"
