# ConvertIT

Lightweight Windows file converter with Explorer context-menu integration.

## Install

### Easiest

```powershell
$script = Join-Path $env:TEMP "convertit-install.ps1"
Invoke-WebRequest "https://raw.githubusercontent.com/Wisnia9600/ConvertIT/main/scripts/install.ps1" -OutFile $script
& $script
```

This downloads the latest release, installs ConvertIT into `%LOCALAPPDATA%\Programs\ConvertIT`, and registers the right-click menu.

### From Zip

1. Download the latest `ConvertIT_<version>_x64.zip` from [GitHub Releases](https://github.com/Wisnia9600/ConvertIT/releases).
2. Extract it.
3. Run `install.cmd`.

### Uninstall

Run `uninstall.cmd`.

## Explorer Menu

1. Right-click a supported file.
2. Click `Show more options` on Windows 11.
3. Open `Convert to`.
4. Pick the target format.

Converted files are saved beside the original as `<name>.converted.<ext>`.

## Supported Formats

### Video

- `MP4` -> `GIF`
- `MP4` <-> `WebM` / `MOV` / `AVI` / `MKV`
- `AVI` / `MKV` -> `MP4`
- `Video` -> `MP3` / `WAV`

### Images

- `PNG` <-> `JPG` / `JPEG`
- `PNG` / `JPG` -> `WebP`
- `HEIC` -> `JPG`
- `SVG` -> `PNG` / `JPG`
- `CR2` / `ARW` -> `JPG`

### Audio

- `MP3` <-> `WAV` / `FLAC` / `OGG` / `AAC` / `M4A`
- `WAV` -> `MP3`

## CLI Usage

```powershell
ConvertIT.exe convert --input "C:\path\file.mp4" --preset video.mp4_to_gif
ConvertIT.exe help
```

## Build From Source

Requirements:

- Rust stable
- Visual Studio Build Tools with the C++ workload
- FFmpeg and ImageMagick available locally, or staged with `scripts/fetch-tools.ps1`

Commands:

```powershell
./scripts/fetch-tools.ps1 -Destination "./vendor/bin"
cargo test --manifest-path src-tauri/Cargo.toml
cargo build --release --manifest-path src-tauri/Cargo.toml
./scripts/package-release.ps1 -Version 0.1.1
```

## Project Structure

- `src-tauri/` - Rust conversion logic and shell integration
- `scripts/` - install, packaging, and helper scripts
- `assets/` - branding assets
- `vendor/` - third-party tool notes and staged binaries

## SmartScreen

Windows may still show `Unknown publisher` or `Windows protected your PC` for unsigned public builds. Removing that requires code signing.
