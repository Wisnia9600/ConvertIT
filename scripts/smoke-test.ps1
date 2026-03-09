param(
  [string]$ExecutablePath = ".\src-tauri\target\release\convertit.exe",
  [string]$ToolsDir = ".\vendor\bin"
)

$ErrorActionPreference = "Stop"

$resolvedExe = (Resolve-Path $ExecutablePath).Path
$resolvedToolsDir = (Resolve-Path $ToolsDir).Path
$ffmpeg = Join-Path $resolvedToolsDir "ffmpeg.exe"
$magick = Join-Path $resolvedToolsDir "magick.exe"
$workDir = Join-Path $env:TEMP "convertit-smoke"

if (Test-Path $workDir) {
  cmd /c "rmdir /s /q `"$workDir`""
}

New-Item -ItemType Directory -Force -Path $workDir | Out-Null
$env:CONVERTIT_TOOL_DIR = $resolvedToolsDir

& $ffmpeg -y -f lavfi -i testsrc=size=64x64:rate=12 -f lavfi -i sine=frequency=880:sample_rate=44100 -t 1 -pix_fmt yuv420p -c:v libx264 -c:a aac (Join-Path $workDir "clip.mp4") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "clip.mp4") (Join-Path $workDir "clip.webm") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "clip.mp4") (Join-Path $workDir "clip.mov") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "clip.mp4") (Join-Path $workDir "clip.avi") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "clip.mp4") (Join-Path $workDir "clip.mkv") | Out-Null
& $ffmpeg -y -f lavfi -i sine=frequency=660:sample_rate=44100 -t 1 (Join-Path $workDir "tone.wav") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "tone.wav") (Join-Path $workDir "tone.mp3") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "tone.wav") (Join-Path $workDir "tone.flac") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "tone.wav") (Join-Path $workDir "tone.ogg") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "tone.wav") (Join-Path $workDir "tone.aac") | Out-Null
& $ffmpeg -y -i (Join-Path $workDir "tone.wav") (Join-Path $workDir "tone.m4a") | Out-Null
& $magick -size 64x64 xc:tomato (Join-Path $workDir "image.png")
& $magick (Join-Path $workDir "image.png") (Join-Path $workDir "image.jpg")
'<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64"><rect width="64" height="64" fill="#f97316"/><circle cx="32" cy="32" r="18" fill="#111827"/></svg>' | Set-Content (Join-Path $workDir "vector.svg")

$cases = @(
  @{ Input = "clip.mp4"; Preset = "video.mp4_to_gif"; Expected = "clip.converted.gif" },
  @{ Input = "clip.mp4"; Preset = "video.mp4_to_webm"; Expected = "clip.converted.webm" },
  @{ Input = "clip.webm"; Preset = "video.webm_to_mp4"; Expected = "clip.converted.mp4" },
  @{ Input = "clip.mp4"; Preset = "video.mp4_to_mov"; Expected = "clip.converted.mov" },
  @{ Input = "clip.mov"; Preset = "video.mov_to_mp4"; Expected = "clip.converted.mp4" },
  @{ Input = "clip.mp4"; Preset = "video.mp4_to_avi"; Expected = "clip.converted.avi" },
  @{ Input = "clip.avi"; Preset = "video.avi_to_mp4"; Expected = "clip.converted.mp4" },
  @{ Input = "clip.mp4"; Preset = "video.mp4_to_mkv"; Expected = "clip.converted.mkv" },
  @{ Input = "clip.mkv"; Preset = "video.mkv_to_mp4"; Expected = "clip.converted.mp4" },
  @{ Input = "clip.mp4"; Preset = "audio.video_to_mp3"; Expected = "clip.converted.mp3" },
  @{ Input = "clip.mp4"; Preset = "audio.video_to_wav"; Expected = "clip.converted.wav" },
  @{ Input = "image.png"; Preset = "image.png_to_jpg"; Expected = "image.converted.jpg" },
  @{ Input = "image.jpg"; Preset = "image.jpg_to_png"; Expected = "image.converted.png" },
  @{ Input = "image.png"; Preset = "image.raster_to_webp"; Expected = "image.converted.webp" },
  @{ Input = "vector.svg"; Preset = "image.svg_to_png"; Expected = "vector.converted.png" },
  @{ Input = "vector.svg"; Preset = "image.svg_to_jpg"; Expected = "vector.converted.jpg" },
  @{ Input = "tone.mp3"; Preset = "audio.mp3_to_wav"; Expected = "tone.converted.wav" },
  @{ Input = "tone.wav"; Preset = "audio.wav_to_mp3"; Expected = "tone.converted.mp3" },
  @{ Input = "tone.mp3"; Preset = "audio.mp3_to_flac"; Expected = "tone.converted.flac" },
  @{ Input = "tone.flac"; Preset = "audio.flac_to_mp3"; Expected = "tone.converted.mp3" },
  @{ Input = "tone.mp3"; Preset = "audio.mp3_to_ogg"; Expected = "tone.converted.ogg" },
  @{ Input = "tone.ogg"; Preset = "audio.ogg_to_mp3"; Expected = "tone.converted.mp3" },
  @{ Input = "tone.mp3"; Preset = "audio.mp3_to_aac"; Expected = "tone.converted.aac" },
  @{ Input = "tone.aac"; Preset = "audio.aac_to_mp3"; Expected = "tone.converted.mp3" },
  @{ Input = "tone.mp3"; Preset = "audio.mp3_to_m4a"; Expected = "tone.converted.m4a" },
  @{ Input = "tone.m4a"; Preset = "audio.m4a_to_mp3"; Expected = "tone.converted.mp3" }
)

$results = for ($index = 0; $index -lt $cases.Count; $index++) {
  $case = $cases[$index]
  $caseDir = Join-Path $workDir ("case-" + ($index + 1))
  New-Item -ItemType Directory -Force -Path $caseDir | Out-Null

  $sourcePath = Join-Path $workDir $case.Input
  $inputPath = Join-Path $caseDir $case.Input
  Copy-Item $sourcePath $inputPath -Force

  $expectedPath = Join-Path $caseDir $case.Expected

  $output = & $resolvedExe convert --input $inputPath --preset $case.Preset 2>&1
  $ok = ($LASTEXITCODE -eq 0) -and (Test-Path $expectedPath)

  [pscustomobject]@{
    Preset = $case.Preset
    Input = $case.Input
    Status = if ($ok) { "PASS" } else { "FAIL" }
    Details = ($output | Out-String).Trim()
  }
}

$results | Format-Table -AutoSize

$failed = @($results | Where-Object { $_.Status -ne "PASS" })
if ($failed.Count -gt 0) {
  throw "Smoke test failed for $($failed.Count) preset(s)."
}

Write-Host "Skipped presets: image.heic_to_jpg, image.raw_to_jpg"
