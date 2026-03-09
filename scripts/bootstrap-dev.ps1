param(
  [switch]$InstallJs,
  [switch]$StageVendorTools
)

$ErrorActionPreference = "Stop"

Write-Host "ConvertIT development bootstrap"
Write-Host "1. Install Rust from https://rustup.rs/"
Write-Host "2. Install Visual Studio Build Tools with Desktop development for C++"
Write-Host "3. Ensure Node.js 22+ is available"

if ($InstallJs) {
  corepack enable
  corepack pnpm install
}

if ($StageVendorTools) {
  ./scripts/fetch-tools.ps1 -Destination "./vendor/bin"
}

Write-Host "Bootstrap guidance finished."