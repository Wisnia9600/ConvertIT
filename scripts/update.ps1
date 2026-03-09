param(
  [string]$InstallDir = "$env:LOCALAPPDATA\Programs\ConvertIT",
  [switch]$AddToPath
)

$ErrorActionPreference = "Stop"

$sourceDir = $PSScriptRoot
$vendorSource = Join-Path $sourceDir "vendor\bin"
$exeSource = Join-Path $sourceDir "convertit.exe"

if (-not (Test-Path $exeSource)) {
  throw "convertit.exe was not found next to update.ps1."
}

if (-not (Test-Path $vendorSource)) {
  throw "vendor\\bin was not found next to update.ps1."
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $InstallDir "vendor\bin") | Out-Null

$filesToCopy = @(
  "convertit.exe",
  "convert-shell.ps1",
  "convert-shell.vbs",
  "install.cmd",
  "install-shell.ps1",
  "uninstall.cmd",
  "uninstall-shell.ps1",
  "update.ps1",
  "README.txt"
)

foreach ($file in $filesToCopy) {
  $sourcePath = Join-Path $sourceDir $file
  if (Test-Path $sourcePath) {
    Copy-Item $sourcePath (Join-Path $InstallDir $file) -Force
  }
}

Copy-Item (Join-Path $vendorSource "*") (Join-Path $InstallDir "vendor\bin") -Recurse -Force

if ($AddToPath) {
  $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $segments = @()
  if ($currentPath) {
    $segments = $currentPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
  }

  if ($segments -notcontains $InstallDir) {
    $updatedPath = ($segments + $InstallDir) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $updatedPath, "User")
    $env:Path = "$updatedPath;$env:Path"
  }
}

& (Join-Path $InstallDir "convertit.exe") install-shell

Write-Host "ConvertIT installed in $InstallDir"
