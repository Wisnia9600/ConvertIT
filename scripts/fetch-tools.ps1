param(
  [string]$Destination = "./vendor/bin",
  [string]$FfmpegPath,
  [string]$FfprobePath,
  [string]$MagickPath,
  [string]$DcrawPath
)

$ErrorActionPreference = "Stop"

function Copy-ToolIfAvailable {
  param(
    [string]$TargetName,
    [string]$ExplicitPath,
    [string[]]$CommandNames
  )

  $targetPath = Join-Path $Destination $TargetName
  if (Test-Path $targetPath) {
    Write-Host "$TargetName already staged"
    return
  }

  $candidate = $null

  if ($ExplicitPath -and (Test-Path $ExplicitPath)) {
    $candidate = $ExplicitPath
  }

  if (-not $candidate) {
    foreach ($commandName in $CommandNames) {
      $command = Get-Command $commandName -ErrorAction SilentlyContinue
      if ($command -and (Test-Path $command.Source)) {
        $candidate = $command.Source
        break
      }
    }
  }

  if ($candidate) {
    Copy-Item -Path $candidate -Destination $targetPath -Force
    Write-Host "Staged $TargetName from $candidate"
  } else {
    Write-Warning "$TargetName not found. Stage it manually in vendor/bin before packaging."
  }
}

New-Item -ItemType Directory -Force -Path $Destination | Out-Null

Write-Host "Preparing ConvertIT vendor tools in $Destination"

Copy-ToolIfAvailable -TargetName "ffmpeg.exe" -ExplicitPath $FfmpegPath -CommandNames @("ffmpeg.exe", "ffmpeg")
Copy-ToolIfAvailable -TargetName "ffprobe.exe" -ExplicitPath $FfprobePath -CommandNames @("ffprobe.exe", "ffprobe")
Copy-ToolIfAvailable -TargetName "magick.exe" -ExplicitPath $MagickPath -CommandNames @("magick.exe", "magick")
Copy-ToolIfAvailable -TargetName "dcraw_emu.exe" -ExplicitPath $DcrawPath -CommandNames @("dcraw_emu.exe", "dcraw_emu")

Write-Host "Vendor tool check completed."