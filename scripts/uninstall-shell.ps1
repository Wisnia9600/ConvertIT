param(
  [string]$ExecutablePath
)

$ErrorActionPreference = "Stop"

function Resolve-ConvertITExecutable {
  param([string]$CandidatePath)

  $candidates = @()

  if ($CandidatePath) {
    $candidates += $CandidatePath
  }

  $candidates += @(
    (Join-Path $PSScriptRoot "convertit.exe"),
    (Join-Path $PSScriptRoot "..\src-tauri\target\release\convertit.exe"),
    (Join-Path $env:LOCALAPPDATA "Programs\ConvertIT\convertit.exe"),
    (Join-Path $env:LOCALAPPDATA "Programs\ConvertIT-NoGUI\convertit.exe")
  )

  foreach ($candidate in $candidates) {
    if ($candidate -and (Test-Path $candidate)) {
      return (Resolve-Path $candidate).Path
    }
  }

  throw "ConvertIT executable not found. Pass -ExecutablePath if you want to remove shell integration from a specific install."
}

$resolvedExecutable = Resolve-ConvertITExecutable -CandidatePath $ExecutablePath
& $resolvedExecutable uninstall-shell
Write-Host "ConvertIT shell menu removed using $resolvedExecutable"
