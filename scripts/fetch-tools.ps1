param(
  [string]$Destination = "./vendor/bin",
  [string]$FfmpegPath,
  [string]$FfprobePath,
  [string]$MagickPath,
  [string]$DcrawPath,
  [string]$LibRawDllPath,
  [string]$LibRawZipUrl = "https://www.libraw.org/data/LibRaw-0.22.0-Win64.zip"
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
    return $true
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
    return $true
  }

  Write-Warning "$TargetName not found."
  return $false
}

function Stage-LibRawTools {
  param(
    [string]$ZipUrl
  )

  $dcrawTarget = Join-Path $Destination 'dcraw_emu.exe'
  $librawTarget = Join-Path $Destination 'libraw.dll'
  if ((Test-Path $dcrawTarget) -and (Test-Path $librawTarget)) {
    Write-Host 'LibRaw tools already staged'
    return
  }

  $tempRoot = Join-Path $env:TEMP 'convertit-libraw-fetch'
  $zipPath = Join-Path $tempRoot 'LibRaw.zip'
  $extractDir = Join-Path $tempRoot 'extract'
  New-Item -ItemType Directory -Force -Path $tempRoot,$extractDir | Out-Null

  Write-Host "Downloading LibRaw package from $ZipUrl"
  curl.exe -L $ZipUrl -o $zipPath
  if ($LASTEXITCODE -ne 0) {
    throw 'Failed to download LibRaw package'
  }

  Expand-Archive -LiteralPath $zipPath -DestinationPath $extractDir -Force
  $dcrawExe = Get-ChildItem $extractDir -Recurse -Filter dcraw_emu.exe | Select-Object -First 1
  $librawDll = Get-ChildItem $extractDir -Recurse -Filter libraw.dll | Select-Object -First 1

  if (-not $dcrawExe -or -not $librawDll) {
    throw 'LibRaw package did not contain dcraw_emu.exe and libraw.dll'
  }

  Copy-Item $dcrawExe.FullName $dcrawTarget -Force
  Copy-Item $librawDll.FullName $librawTarget -Force
  Write-Host 'Staged LibRaw tools from official package'
}

New-Item -ItemType Directory -Force -Path $Destination | Out-Null

Write-Host "Preparing ConvertIT vendor tools in $Destination"

Copy-ToolIfAvailable -TargetName "ffmpeg.exe" -ExplicitPath $FfmpegPath -CommandNames @("ffmpeg.exe", "ffmpeg") | Out-Null
Copy-ToolIfAvailable -TargetName "ffprobe.exe" -ExplicitPath $FfprobePath -CommandNames @("ffprobe.exe", "ffprobe") | Out-Null
Copy-ToolIfAvailable -TargetName "magick.exe" -ExplicitPath $MagickPath -CommandNames @("magick.exe", "magick") | Out-Null

if (-not (Copy-ToolIfAvailable -TargetName "dcraw_emu.exe" -ExplicitPath $DcrawPath -CommandNames @("dcraw_emu.exe", "dcraw_emu"))) {
  Stage-LibRawTools -ZipUrl $LibRawZipUrl
}

if (-not (Test-Path (Join-Path $Destination 'libraw.dll'))) {
  if ($LibRawDllPath -and (Test-Path $LibRawDllPath)) {
    Copy-Item $LibRawDllPath (Join-Path $Destination 'libraw.dll') -Force
    Write-Host "Staged libraw.dll from $LibRawDllPath"
  } else {
    Stage-LibRawTools -ZipUrl $LibRawZipUrl
  }
}

Write-Host "Vendor tool check completed."