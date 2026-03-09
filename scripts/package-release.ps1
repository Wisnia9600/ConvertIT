param(
  [string]$Version,
  [string]$ReleaseExePath = "./src-tauri/target/release/convertit.exe",
  [string]$VendorDir = "./vendor/bin",
  [string]$OutputDir = "./artifacts"
)

$ErrorActionPreference = "Stop"

if (-not $Version) {
  throw "Pass -Version <semver>."
}

$resolvedExe = (Resolve-Path $ReleaseExePath).Path
$resolvedVendor = (Resolve-Path $VendorDir).Path
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
$resolvedOutputDir = (Resolve-Path $OutputDir).Path

$stagingRoot = Join-Path $env:TEMP "convertit-release-$Version"
$packageRoot = Join-Path $stagingRoot "ConvertIT"
$vendorTarget = Join-Path $packageRoot "vendor\bin"
$zipPath = Join-Path $resolvedOutputDir "ConvertIT_${Version}_x64.zip"

if (Test-Path $stagingRoot) {
  Remove-Item $stagingRoot -Recurse -Force
}

if (Test-Path $zipPath) {
  Remove-Item $zipPath -Force
}

New-Item -ItemType Directory -Force -Path $vendorTarget | Out-Null

Copy-Item $resolvedExe (Join-Path $packageRoot "convertit.exe") -Force
Copy-Item (Join-Path $resolvedVendor "*") $vendorTarget -Recurse -Force
Copy-Item "./scripts/convert-shell.ps1" (Join-Path $packageRoot "convert-shell.ps1") -Force
Copy-Item "./scripts/install-shell.ps1" (Join-Path $packageRoot "install-shell.ps1") -Force
Copy-Item "./scripts/uninstall-shell.ps1" (Join-Path $packageRoot "uninstall-shell.ps1") -Force

@"
ConvertIT
=========

This package contains the CLI build of ConvertIT and the bundled converter tools.

Examples:
  .\convertit.exe convert --input "C:\path\video.mp4" --preset video.mp4_to_gif
  .\install-shell.ps1
  .\uninstall-shell.ps1
"@ | Set-Content (Join-Path $packageRoot "README.txt")

Compress-Archive -Path (Join-Path $packageRoot "*") -DestinationPath $zipPath -Force
Write-Host "Created $zipPath"
