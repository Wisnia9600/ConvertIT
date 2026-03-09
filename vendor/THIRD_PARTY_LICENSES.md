# Third-party tooling inventory

ConvertIT does not commit converter binaries into git. Release builds fetch or stage them into `vendor/bin` during CI or local packaging.

## Planned tools

- FFmpeg / FFprobe
  - Purpose: video and audio conversion
  - License: LGPL/GPL depending on build
  - Distribution note: keep build provenance documented per release
- ImageMagick portable
  - Purpose: raster image conversion, WebP, SVG rasterization
  - License: ImageMagick License
- `dcraw_emu`
  - Purpose: RAW photo decode for `CR2` and `ARW`
  - License: verify exact upstream package before release

Before publishing production releases, verify the exact downloaded artifacts and update this file with upstream source URLs and license texts or notices as required.