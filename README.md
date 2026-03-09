# 🔄 ConvertIT

[![Platform: Windows](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-blue.svg)]()
[![Bundle: Tauri](https://img.shields.io/badge/Built_with-Tauri%20%7C%20Rust%20%7C%20React-orange.svg)]()
[![Installer Size](https://img.shields.io/badge/Size-~105_MB-success.svg)]()

> A lightweight Windows file converter with Explorer context-menu integration. Export files to practical formats in just two clicks, without opening a full editor.

---

## ✨ Features

- **Context Menu Integration**: Adds a classic Windows Explorer `Convert to` submenu.
- **Advanced Options**: Includes a minimal GUI for format and quality selection when needed.
- **Fully Offline**: Works entirely offline with zero cloud dependency after installation.
- **Batteries Included**: Bundles required converter tools (FFmpeg, ImageMagick, LibRaw) directly with the app.
- **Windows 11 Ready**: Accessible via the classic context menu under `Show more options`.
- **Safe Conversions**: Never overwrites your original files. Output files are automatically named `<name>.converted.<ext>` (or `... (2).<ext>` if a file already exists).

---

## 🗂️ Supported Formats (v1)

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

## 🚀 Installation & Usage

### Standard Install
1. Download the latest `ConvertIT_..._x64-setup.exe` from [GitHub Releases](https://github.com/Wisnia9600/ConvertIT/releases).
2. Run the installer and complete the setup.
3. Right-click a supported file in Windows Explorer.
   * *Note: On Windows 11, click `Show more options` first.*
4. Open the `Convert to` menu and choose your target format!

**Troubleshooting the Explorer Menu:**
If the menu doesn't appear, ensure you are right-clicking a *supported* file type. If it still doesn't show up, launch the ConvertIT desktop app once manually to initialize settings, then retry.

---

## 💻 CLI Usage

ConvertIT can also be used via the command line for automation or advanced control:

```powershell
# Convert a file using a specific preset
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif

# Open the advanced GUI for a specific file
ConvertIT.exe advanced --input "C:\path\file.mp4"

# Manage shell registration manually
ConvertIT.exe install-shell
ConvertIT.exe uninstall-shell

```

---

## 🛠️ Build from Source

### Requirements

* **Node.js**: v22+
* **Package Manager**: `pnpm` (via Corepack)
* **Rust**: Stable toolchain
* **C++ Workload**: Microsoft Visual Studio Build Tools

### Development Setup

```powershell
# Enable Corepack and install dependencies
corepack enable
corepack pnpm install

# Build frontend and test backend
corepack pnpm build
cargo test --manifest-path src-tauri/Cargo.toml

# Build the Tauri Windows bundle
corepack pnpm tauri:build

```

**Helper Script:** Alternatively, use the included bootstrap script for a quick start:

```powershell
./scripts/bootstrap-dev.ps1

```

---

## 📁 Project Structure

* `src/` — React UI for the advanced conversion window.
* `src-tauri/` — Rust backend, shell registration, settings, and packaging.
* `scripts/` — Helper scripts for local setup and fetching vendor tools (`fetch-tools.ps1`).
* `vendor/` — Third-party tool notes and locally staged binaries (not stored in git history).

*Third-party tools packaged during release: FFmpeg/FFprobe, ImageMagick, and LibRaw (`dcraw_emu.exe` + `libraw.dll`). See `vendor/THIRD_PARTY_LICENSES.md` for license details.*

---

## 🗺️ Roadmap

**v1 (Current)**

* [x] Multimedia and image conversions
* [x] Windows installer & Explorer context menu
* [x] Lightweight advanced GUI

**v2 (Planned)**

* [ ] `PDF` / `DOCX` / `Markdown` document conversions
* [ ] Native Windows 11 modern context-menu integration
* [ ] Broader document processing pipeline

---

*Bundle identifier: `com.convertit.desktop*`
