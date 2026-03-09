# 🔄 ConvertIT

[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)]()
[![Built with: Rust](https://img.shields.io/badge/Built_with-Rust-orange.svg)]()

> Lightweight Windows file converter with Explorer context-menu integration. Export files to practical formats in just two clicks.

---

## 🚀 Install

### From Zip

1. Download the latest `ConvertIT_<version>_x64.zip` from [GitHub Releases](https://github.com/Wisnia9600/ConvertIT/releases).
2. Extract the archive.
3. Run `install.cmd`.

### Uninstall

Simply run `uninstall.cmd` from your installation or extracted folder.

---

## 🖱️ Explorer Menu Usage

1. Right-click a supported file.
2. Click **`Show more options`** (if you are on Windows 11).
3. Open **`Convert to`**.
4. Pick the target format.

*Note: Converted files are safely saved beside the original as `<name>.converted.<ext>`.*

---

## 🗂️ Supported Formats

### 🎬 Video

* `MP4` ➡️ `GIF`
* `MP4` ↔️ `WebM` / `MOV` / `AVI` / `MKV`
* `AVI` / `MKV` ➡️ `MP4`
* `Video` ➡️ `MP3` / `WAV` (Audio Extraction)

### 🖼️ Images

* `PNG` ↔️ `JPG` / `JPEG`
* `PNG` / `JPG` ➡️ `WebP`
* `HEIC` / `SVG` / `CR2` / `ARW` ➡️ `JPG` (or `PNG` for SVG)

### 🎵 Audio

* `MP3` ↔️ `WAV` / `FLAC` / `OGG` / `AAC` / `M4A`
* `WAV` ➡️ `MP3`

---

## 💻 CLI Usage

You can also use ConvertIT from the command line:

```powershell
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif
ConvertIT.exe help

```

---

## 🛠️ Build From Source

### Requirements

* **Rust**: Stable toolchain
* **C++ Workload**: Microsoft Visual Studio Build Tools
* **Tools**: FFmpeg and ImageMagick available locally, or staged using the included fetch script.

### Commands

```powershell
# Fetch required vendor tools
./scripts/fetch-tools.ps1 -Destination "./vendor/bin"

# Run tests
cargo test --manifest-path src-tauri/Cargo.toml

# Build the release binary
cargo build --release --manifest-path src-tauri/Cargo.toml

# Package the release
./scripts/package-release.ps1 -Version 0.1.1

```

---

## 📁 Project Structure

* `src-tauri/` — Rust conversion logic and shell integration.
* `scripts/` — Installation, packaging, and helper scripts.
* `assets/` — Branding assets.
* `vendor/` — Third-party tool notes and staged binaries.

---

## ⚠️ SmartScreen Notice

Windows may show an **"Unknown publisher"** or **"Windows protected your PC"** warning for unsigned public builds. Removing this prompt requires a paid code-signing certificate. Simply click "More info" and "Run anyway" to proceed.
