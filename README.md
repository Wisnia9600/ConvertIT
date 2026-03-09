# 🔄 ConvertIT

[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)]()
[![Runtime: Rust](https://img.shields.io/badge/Built_with-Rust-orange.svg)]()
[![Mode: CLI only](https://img.shields.io/badge/Mode-CLI_only-success.svg)]()

> A lightweight Windows file converter with Explorer context-menu integration. Right-click a supported file, pick `Convert to`, and get a converted copy next to the original.

---

## ✨ What Changed

- **No GUI**: ConvertIT is now CLI-only.
- **Right-click stays**: the Windows Explorer `Convert to` submenu still works.
- **PowerShell install**: releases are delivered as a portable zip plus install script.
- **Offline**: conversion still runs locally with bundled FFmpeg, ImageMagick, and LibRaw tools.

---

## 🗂️ Supported Formats

### 🎬 Video
* `MP4` ➡️ `GIF`
* `MP4` ↔️ `WebM` / `MOV` / `AVI` / `MKV`
* `AVI` / `MKV` ➡️ `MP4`
* `Video` ➡️ `MP3` / `WAV`

### 🖼️ Images
* `PNG` ↔️ `JPG` / `JPEG`
* `PNG` / `JPG` ➡️ `WebP`
* `HEIC` ➡️ `JPG`
* `SVG` ➡️ `PNG` / `JPG`
* `CR2` / `ARW` ➡️ `JPG`

### 🎵 Audio
* `MP3` ↔️ `WAV` / `FLAC` / `OGG` / `AAC` / `M4A`
* `WAV` ➡️ `MP3`

---

## 🚀 Install

### PowerShell

```powershell
$script = Join-Path $env:TEMP "convertit-install.ps1"
Invoke-WebRequest "https://raw.githubusercontent.com/Wisnia9600/ConvertIT/main/scripts/install.ps1" -OutFile $script
& $script -AddToPath
```

This installs ConvertIT into `%LOCALAPPDATA%\Programs\ConvertIT` and registers the right-click menu.

### Manual

1. Download the latest `ConvertIT_<version>_x64.zip` from [GitHub Releases](https://github.com/Wisnia9600/ConvertIT/releases).
2. Extract it anywhere you want.
3. Run `install-shell.ps1`.

### Uninstall Shell Menu

```powershell
.\uninstall-shell.ps1
```

---

## 🖱️ Explorer Menu

On Windows 11:

1. Right-click a supported file.
2. Click `Show more options`.
3. Open `Convert to`.
4. Pick the target format.

Converted files are saved beside the original as `<name>.converted.<ext>`. Existing files are never overwritten.

---

## 💻 CLI Usage

```powershell
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif
ConvertIT.exe install-shell
ConvertIT.exe uninstall-shell
ConvertIT.exe help
```

---

## 🛠️ Build From Source

Requirements:

* Rust stable
* Microsoft Visual Studio Build Tools with the C++ workload
* FFmpeg and ImageMagick available locally, or staged with `scripts/fetch-tools.ps1`

Commands:

```powershell
./scripts/fetch-tools.ps1 -Destination "./vendor/bin"
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --release --manifest-path src-tauri/Cargo.toml
./scripts/package-release.ps1 -Version 0.1.1
```

---

## 📁 Project Structure

* `src-tauri/` — Rust conversion logic and shell integration.
* `scripts/` — install, packaging, and vendor-tool helper scripts.
* `vendor/` — third-party tool notes and staged binaries.

---

## ⚠️ SmartScreen

Windows may still show `Unknown publisher` or `Windows protected your PC` for public unsigned builds. That cannot be fully removed without code signing.
