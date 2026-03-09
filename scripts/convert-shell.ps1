param(
  [Parameter(Mandatory = $true)]
  [string]$ExecutablePath,
  [Parameter(Mandatory = $true)]
  [string]$InputPath,
  [Parameter(Mandatory = $true)]
  [string]$PresetId
)

$ErrorActionPreference = "Stop"

& $ExecutablePath convert --input $InputPath --preset $PresetId
