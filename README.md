# ConvertIT

ConvertIT is a lightweight Windows desktop file converter with Explorer context-menu integration.

## v1 scope

- Video:
  - `MP4 -> GIF/WebM/MOV/AVI/MKV/MP3/WAV`
  - `WebM <-> MP4`
  - `MOV <-> MP4`
  - `AVI -> MP4`
  - `MKV -> MP4`
- Images:
  - `PNG <-> JPG/JPEG`
  - `PNG/JPG -> WebP`
  - `HEIC -> JPG`
  - `SVG -> PNG/JPG`
  - `CR2/ARW -> JPG`
- Audio:
  - `MP3 <-> WAV`
  - `MP3 <-> FLAC`
  - `MP3 <-> OGG/AAC/M4A`
  - `WAV -> MP3`

Document conversions are planned for v2.

## Product goals

- Lightweight desktop app built with Tauri 2
- Classic Windows Explorer submenu: `Convert to`
- Minimal advanced GUI with presets, progress, and plain-language errors
- Offline-first packaging with bundled converter binaries
- Public GitHub-ready repository with CI and release automation

## Development setup

### Prerequisites

- Node.js 22+
- `pnpm` via Corepack
- Rust stable toolchain
- Microsoft Visual Studio C++ Build Tools

### Install

```powershell
corepack enable
corepack pnpm install
cargo --version
corepack pnpm tauri:dev
```

### Build

```powershell
corepack pnpm build
corepack pnpm tauri:build
```

## Bundled tools

The app is designed to package third-party tools during release builds instead of committing binaries to git:

- FFmpeg / FFprobe
- ImageMagick portable
- `dcraw_emu` for RAW decoding

See `vendor/THIRD_PARTY_LICENSES.md` and `scripts/fetch-tools.ps1`.

## Output behavior

- Output is written next to the source file
- Filename format: `<name>.converted.<ext>`
- Conflicts become `<name>.converted (2).<ext>`, `<name>.converted (3).<ext>`, and so on
- Existing files are never overwritten by default

## Context menu behavior

On Windows 11 v1 integrates with the classic Explorer menu under `Show more options`.

Supported shell commands:

```powershell
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif
ConvertIT.exe advanced --input "C:\path\file.mp4"
ConvertIT.exe install-shell
ConvertIT.exe uninstall-shell
```

## Roadmap

- v1:
  - multimedia and image conversions
  - context menu integration
  - Windows installer and uninstaller
- v2:
  - `PDF/DOCX/Markdown` conversion pipeline
  - native Windows 11 modern context menu integration
## RAW support note

`CR2/ARW -> JPG` is implemented in the app and requires `dcraw_emu.exe` to be staged into `vendor/bin` for local packaging or release builds. The CI workflow currently auto-stages FFmpeg and ImageMagick from the runner, while RAW decoding still requires an explicit binary source.
