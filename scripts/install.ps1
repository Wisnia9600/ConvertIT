param(
  [string]$Repo = "Wisnia9600/ConvertIT",
  [string]$Version = "latest",
  [string]$InstallDir = "$env:LOCALAPPDATA\Programs\ConvertIT",
  [switch]$SkipShellRegistration,
  [switch]$AddToPath
)

$ErrorActionPreference = "Stop"

function Get-ReleaseApiUrl {
  param([string]$Repository, [string]$RequestedVersion)

  if ($RequestedVersion -eq "latest") {
    return "https://api.github.com/repos/$Repository/releases/latest"
  }

  return "https://api.github.com/repos/$Repository/releases/tags/$RequestedVersion"
}

function Get-Asset {
  param(
    [pscustomobject]$Release,
    [string]$Suffix
  )

  foreach ($asset in $Release.assets) {
    if ($asset.name -like "*$Suffix") {
      return $asset
    }
  }

  throw "Release asset with suffix '$Suffix' not found."
}

function Add-DirectoryToUserPath {
  param([string]$Directory)

  $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
  $segments = @()
  if ($currentPath) {
    $segments = $currentPath.Split(';', [System.StringSplitOptions]::RemoveEmptyEntries)
  }

  if ($segments -contains $Directory) {
    return
  }

  $updatedPath = ($segments + $Directory) -join ';'
  [Environment]::SetEnvironmentVariable("Path", $updatedPath, "User")
  $env:Path = "$updatedPath;$env:Path"
}

$releaseApiUrl = Get-ReleaseApiUrl -Repository $Repo -RequestedVersion $Version
$release = Invoke-RestMethod -Uri $releaseApiUrl -Headers @{ "User-Agent" = "ConvertIT-Installer" }
$asset = Get-Asset -Release $release -Suffix "_x64.zip"

$tempRoot = Join-Path $env:TEMP "convertit-install"
New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null
$downloadPath = Join-Path $tempRoot $asset.name

Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $downloadPath -Headers @{ "User-Agent" = "ConvertIT-Installer" }
Unblock-File -Path $downloadPath -ErrorAction SilentlyContinue

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $downloadPath -DestinationPath $InstallDir -Force
$exePath = Join-Path $InstallDir "convertit.exe"

if ($AddToPath) {
  Add-DirectoryToUserPath -Directory $InstallDir
}

if (-not $SkipShellRegistration) {
  & $exePath install-shell
}

Write-Host "ConvertIT installed to $InstallDir"
if ($AddToPath) {
  Write-Host "Added $InstallDir to the current user's PATH"
}
