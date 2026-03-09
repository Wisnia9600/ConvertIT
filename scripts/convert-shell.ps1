param(
  [Parameter(Mandatory = $true)]
  [string]$ExecutablePath,
  [Parameter(Mandatory = $true)]
  [string]$InputPath,
  [Parameter(Mandatory = $true)]
  [string]$PresetId
)

$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Windows.Forms

$stdoutPath = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
$stderrPath = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
$toolDir = Join-Path (Split-Path -Parent $ExecutablePath) "vendor\bin"
$previousToolDir = $env:CONVERTIT_TOOL_DIR

try {
  if (Test-Path $toolDir) {
    $env:CONVERTIT_TOOL_DIR = $toolDir
  }

  $process = Start-Process `
    -FilePath $ExecutablePath `
    -ArgumentList @("convert", "--input", $InputPath, "--preset", $PresetId, "--open-folder") `
    -WindowStyle Hidden `
    -Wait `
    -PassThru `
    -RedirectStandardOutput $stdoutPath `
    -RedirectStandardError $stderrPath

  $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
  $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }

  if ($process.ExitCode -ne 0) {
    $details = if ($stderr.Trim()) {
      $stderr.Trim()
    }
    elseif ($stdout.Trim()) {
      $stdout.Trim()
    }
    else {
      "Conversion failed without an error message."
    }

    [System.Windows.Forms.MessageBox]::Show(
      "ConvertIT could not convert this file.`r`n`r`n$details",
      "ConvertIT",
      [System.Windows.Forms.MessageBoxButtons]::OK,
      [System.Windows.Forms.MessageBoxIcon]::Error
    ) | Out-Null

    exit $process.ExitCode
  }
}
catch {
  [System.Windows.Forms.MessageBox]::Show(
    "ConvertIT could not start.`r`n`r`n$($_.Exception.Message)",
    "ConvertIT",
    [System.Windows.Forms.MessageBoxButtons]::OK,
    [System.Windows.Forms.MessageBoxIcon]::Error
  ) | Out-Null

  exit 1
}
finally {
  if ($null -eq $previousToolDir) {
    Remove-Item Env:\CONVERTIT_TOOL_DIR -ErrorAction SilentlyContinue
  }
  else {
    $env:CONVERTIT_TOOL_DIR = $previousToolDir
  }

  Remove-Item $stdoutPath, $stderrPath -Force -ErrorAction SilentlyContinue
}
