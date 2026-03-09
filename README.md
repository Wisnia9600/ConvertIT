# ConvertIT

**ConvertIT** is a lightweight Windows file converter with Explorer context-menu integration.

Right-click a supported file, open `Convert to`, and export it to a practical format without opening a full editor.

## What it does

- adds a classic Windows Explorer submenu: `Convert to`
- includes a small `Advanced...` window for format and quality selection
- works offline after installation
- bundles the required converter tools with the app build
- targets Windows 11 and uses the classic context menu under `Show more options`

## Supported formats in v1

### Video

- `MP4 -> GIF`
- `MP4 <-> WebM`
- `MP4 <-> MOV`
- `MP4 <-> AVI`
- `MP4 <-> MKV`
- `AVI -> MP4`
- `MKV -> MP4`
- `Video -> MP3 / WAV`

### Images

- `PNG <-> JPG / JPEG`
- `PNG / JPG -> WebP`
- `HEIC -> JPG`
- `SVG -> PNG / JPG`
- `CR2 / ARW -> JPG`

### Audio

- `MP3 <-> WAV`
- `MP3 <-> FLAC`
- `MP3 <-> OGG / AAC / M4A`
- `WAV -> MP3`

## Current status

Working locally and already verified:

- frontend production build
- Rust backend tests
- Tauri Windows bundle build
- smoke-tested conversions:
  - `MP4 -> GIF`
  - `PNG -> JPG`
- Explorer shell registration and cleanup

Current gap:

- `CR2 / ARW -> JPG` is implemented, but still needs `dcraw_emu.exe` staged for full local end-to-end verification

## Install on a new PC

### Option 1: normal user install

1. Download the latest `ConvertIT_..._x64-setup.exe` from GitHub Releases.
2. Run the installer.
3. Finish setup.
4. Right-click a supported file.
5. On Windows 11, click `Show more options`.
6. Open `Convert to` and choose a target format.

### What the installer gives you

- the `ConvertIT` desktop app
- the release binary and bundled converter tools
- Explorer context-menu support for supported file types

### First-use behavior

- converted files are saved next to the original file
- output name format is `<name>.converted.<ext>`
- if that name already exists, ConvertIT creates `<name>.converted (2).<ext>` and so on
- files are not overwritten by default

### If something does not show up in Explorer

- make sure you are right-clicking a supported file type
- on Windows 11, use `Show more options`
- if needed, launch ConvertIT once and retry

## Build from source

### Requirements

- Node.js 22+
- `pnpm` via Corepack
- Rust stable toolchain
- Microsoft Visual Studio Build Tools with C++ workload

### Development setup

```powershell
corepack enable
corepack pnpm install
corepack pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
corepack pnpm tauri:build
```

### Helper script

There is also a bootstrap helper:

```powershell
./scripts/bootstrap-dev.ps1
```

## Project structure

- `src/` - React UI for the advanced conversion window
- `src-tauri/` - Rust backend, shell registration, settings, packaging
- `scripts/` - helper scripts for local setup and vendor tools
- `vendor/` - third-party tool notes and locally staged binaries

## Bundled tools

ConvertIT is designed to package external converter tools during release builds instead of storing them directly in git history.

Planned/runtime tools:

- FFmpeg / FFprobe
- ImageMagick
- `dcraw_emu` for RAW decoding

See:

- `scripts/fetch-tools.ps1`
- `vendor/THIRD_PARTY_LICENSES.md`

## CLI examples

```powershell
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif
ConvertIT.exe advanced --input "C:\path\file.mp4"
ConvertIT.exe install-shell
ConvertIT.exe uninstall-shell
```

## Roadmap

### v1

- multimedia and image conversions
- Windows installer
- Explorer context menu
- lightweight advanced GUI

### v2

- `PDF / DOCX / Markdown` conversions
- native Windows 11 modern context-menu integration
- broader document pipeline

## Notes

- bundle identifier: `com.convertit.desktop`
- public repository: `Wisnia9600/ConvertIT`
- current local installer size is about `105 MB`