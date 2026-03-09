param(
  [switch]$StageVendorTools
)

$ErrorActionPreference = "Stop"

Write-Host "ConvertIT development bootstrap"
Write-Host "1. Install Rust from https://rustup.rs/"
Write-Host "2. Install Visual Studio Build Tools with Desktop development for C++"
Write-Host "3. Ensure FFmpeg and ImageMagick are available, or use fetch-tools.ps1"

if ($StageVendorTools) {
  ./scripts/fetch-tools.ps1 -Destination "./vendor/bin"
}

Write-Host "Bootstrap guidance finished."
