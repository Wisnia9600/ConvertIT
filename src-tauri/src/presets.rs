use std::collections::BTreeSet;
use std::path::Path;

use crate::models::ConversionPreset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetKind {
    Mp4ToGif,
    Mp4ToWebm,
    WebmToMp4,
    Mp4ToMov,
    MovToMp4,
    Mp4ToAvi,
    AviToMp4,
    Mp4ToMkv,
    MkvToMp4,
    VideoToMp3,
    VideoToWav,
    PngToJpg,
    JpgToPng,
    RasterToWebp,
    HeicToJpg,
    SvgToPng,
    SvgToJpg,
    RawToJpg,
    Mp3ToWav,
    WavToMp3,
    Mp3ToFlac,
    FlacToMp3,
    Mp3ToOgg,
    OggToMp3,
    Mp3ToAac,
    AacToMp3,
    Mp3ToM4a,
    M4aToMp3,
}

#[derive(Debug, Clone, Copy)]
pub struct PresetDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub source_extensions: &'static [&'static str],
    pub target_extension: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub kind: PresetKind,
}

const PRESETS: &[PresetDefinition] = &[
    PresetDefinition { id: "video.mp4_to_gif", label: "Animated GIF", source_extensions: &["mp4"], target_extension: "gif", category: "video", description: "Shareable animated GIF optimized for quick previews.", kind: PresetKind::Mp4ToGif },
    PresetDefinition { id: "video.mp4_to_webm", label: "WebM video", source_extensions: &["mp4"], target_extension: "webm", category: "video", description: "Web-friendly video using VP9 and Opus.", kind: PresetKind::Mp4ToWebm },
    PresetDefinition { id: "video.webm_to_mp4", label: "MP4 video", source_extensions: &["webm"], target_extension: "mp4", category: "video", description: "Convert WebM into a broadly compatible MP4 file.", kind: PresetKind::WebmToMp4 },
    PresetDefinition { id: "video.mp4_to_mov", label: "MOV video", source_extensions: &["mp4"], target_extension: "mov", category: "video", description: "Apple-friendly MOV output.", kind: PresetKind::Mp4ToMov },
    PresetDefinition { id: "video.mov_to_mp4", label: "MP4 video", source_extensions: &["mov"], target_extension: "mp4", category: "video", description: "Convert MOV into MP4 for wider sharing.", kind: PresetKind::MovToMp4 },
    PresetDefinition { id: "video.mp4_to_avi", label: "AVI video", source_extensions: &["mp4"], target_extension: "avi", category: "video", description: "Legacy AVI container output.", kind: PresetKind::Mp4ToAvi },
    PresetDefinition { id: "video.avi_to_mp4", label: "MP4 video", source_extensions: &["avi"], target_extension: "mp4", category: "video", description: "Modernize AVI into MP4.", kind: PresetKind::AviToMp4 },
    PresetDefinition { id: "video.mp4_to_mkv", label: "MKV video", source_extensions: &["mp4"], target_extension: "mkv", category: "video", description: "Matroska video output for archival or playback flexibility.", kind: PresetKind::Mp4ToMkv },
    PresetDefinition { id: "video.mkv_to_mp4", label: "MP4 video", source_extensions: &["mkv"], target_extension: "mp4", category: "video", description: "Convert MKV into a mobile-friendly MP4.", kind: PresetKind::MkvToMp4 },
    PresetDefinition { id: "audio.video_to_mp3", label: "MP3 audio", source_extensions: &["mp4", "mov", "avi", "mkv", "webm"], target_extension: "mp3", category: "audio", description: "Extract compressed audio from a video file.", kind: PresetKind::VideoToMp3 },
    PresetDefinition { id: "audio.video_to_wav", label: "WAV audio", source_extensions: &["mp4", "mov", "avi", "mkv", "webm"], target_extension: "wav", category: "audio", description: "Extract uncompressed audio from a video file.", kind: PresetKind::VideoToWav },
    PresetDefinition { id: "image.png_to_jpg", label: "JPG image", source_extensions: &["png"], target_extension: "jpg", category: "image", description: "Convert PNG into a compressed JPG.", kind: PresetKind::PngToJpg },
    PresetDefinition { id: "image.jpg_to_png", label: "PNG image", source_extensions: &["jpg", "jpeg"], target_extension: "png", category: "image", description: "Convert JPG into a lossless PNG.", kind: PresetKind::JpgToPng },
    PresetDefinition { id: "image.raster_to_webp", label: "WebP image", source_extensions: &["png", "jpg", "jpeg"], target_extension: "webp", category: "image", description: "Smaller web-ready image output.", kind: PresetKind::RasterToWebp },
    PresetDefinition { id: "image.heic_to_jpg", label: "JPG image", source_extensions: &["heic"], target_extension: "jpg", category: "image", description: "Convert iPhone HEIC photos to JPG.", kind: PresetKind::HeicToJpg },
    PresetDefinition { id: "image.svg_to_png", label: "PNG image", source_extensions: &["svg"], target_extension: "png", category: "image", description: "Rasterize SVG to PNG.", kind: PresetKind::SvgToPng },
    PresetDefinition { id: "image.svg_to_jpg", label: "JPG image", source_extensions: &["svg"], target_extension: "jpg", category: "image", description: "Rasterize SVG to JPG.", kind: PresetKind::SvgToJpg },
    PresetDefinition { id: "image.raw_to_jpg", label: "JPG image", source_extensions: &["cr2", "arw"], target_extension: "jpg", category: "image", description: "Decode RAW photo files into JPG.", kind: PresetKind::RawToJpg },
    PresetDefinition { id: "audio.mp3_to_wav", label: "WAV audio", source_extensions: &["mp3"], target_extension: "wav", category: "audio", description: "Decode MP3 into WAV.", kind: PresetKind::Mp3ToWav },
    PresetDefinition { id: "audio.wav_to_mp3", label: "MP3 audio", source_extensions: &["wav"], target_extension: "mp3", category: "audio", description: "Compress WAV into MP3.", kind: PresetKind::WavToMp3 },
    PresetDefinition { id: "audio.mp3_to_flac", label: "FLAC audio", source_extensions: &["mp3"], target_extension: "flac", category: "audio", description: "Convert MP3 into FLAC container output.", kind: PresetKind::Mp3ToFlac },
    PresetDefinition { id: "audio.flac_to_mp3", label: "MP3 audio", source_extensions: &["flac"], target_extension: "mp3", category: "audio", description: "Compress FLAC into MP3.", kind: PresetKind::FlacToMp3 },
    PresetDefinition { id: "audio.mp3_to_ogg", label: "OGG audio", source_extensions: &["mp3"], target_extension: "ogg", category: "audio", description: "Convert MP3 into OGG Vorbis.", kind: PresetKind::Mp3ToOgg },
    PresetDefinition { id: "audio.ogg_to_mp3", label: "MP3 audio", source_extensions: &["ogg"], target_extension: "mp3", category: "audio", description: "Convert OGG into MP3.", kind: PresetKind::OggToMp3 },
    PresetDefinition { id: "audio.mp3_to_aac", label: "AAC audio", source_extensions: &["mp3"], target_extension: "aac", category: "audio", description: "Convert MP3 into raw AAC.", kind: PresetKind::Mp3ToAac },
    PresetDefinition { id: "audio.aac_to_mp3", label: "MP3 audio", source_extensions: &["aac"], target_extension: "mp3", category: "audio", description: "Convert AAC into MP3.", kind: PresetKind::AacToMp3 },
    PresetDefinition { id: "audio.mp3_to_m4a", label: "M4A audio", source_extensions: &["mp3"], target_extension: "m4a", category: "audio", description: "Convert MP3 into M4A.", kind: PresetKind::Mp3ToM4a },
    PresetDefinition { id: "audio.m4a_to_mp3", label: "MP3 audio", source_extensions: &["m4a"], target_extension: "mp3", category: "audio", description: "Convert M4A into MP3.", kind: PresetKind::M4aToMp3 },
];

pub fn all() -> &'static [PresetDefinition] {
    PRESETS
}

pub fn find_by_id(id: &str) -> Option<&'static PresetDefinition> {
    PRESETS.iter().find(|preset| preset.id == id)
}

pub fn for_input_path(input_path: Option<&str>) -> Vec<ConversionPreset> {
    let Some(path) = input_path else {
        return PRESETS.iter().map(as_api_model).collect();
    };

    let Some(extension) = normalized_extension(path) else {
        return Vec::new();
    };

    PRESETS
        .iter()
        .filter(|preset| preset.source_extensions.iter().any(|candidate| *candidate == extension))
        .map(as_api_model)
        .collect()
}

pub fn supported_extensions() -> BTreeSet<&'static str> {
    let mut values = BTreeSet::new();
    for preset in PRESETS {
        for extension in preset.source_extensions {
            values.insert(*extension);
        }
    }
    values
}

pub fn normalized_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .map(|value| value.to_string_lossy().to_ascii_lowercase())
}

fn as_api_model(preset: &PresetDefinition) -> ConversionPreset {
    ConversionPreset {
        id: preset.id.to_string(),
        label: preset.label.to_string(),
        source_extensions: preset
            .source_extensions
            .iter()
            .map(|extension| extension.to_string())
            .collect(),
        target_extension: preset.target_extension.to_string(),
        category: preset.category.to_string(),
        description: preset.description.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{for_input_path, normalized_extension, supported_extensions};

    #[test]
    fn filters_presets_by_extension() {
        let presets = for_input_path(Some("C:/demo/sample.mp4"));
        let ids = presets.into_iter().map(|preset| preset.id).collect::<Vec<_>>();
        assert!(ids.contains(&"video.mp4_to_gif".to_string()));
        assert!(ids.contains(&"audio.video_to_mp3".to_string()));
        assert!(!ids.contains(&"image.png_to_jpg".to_string()));
    }

    #[test]
    fn normalizes_mixed_case_extensions() {
        assert_eq!(normalized_extension("C:/demo/PHOTO.JPEG"), Some("jpeg".to_string()));
    }

    #[test]
    fn exposes_supported_extensions() {
        let extensions = supported_extensions();
        assert!(extensions.contains("mp4"));
        assert!(extensions.contains("heic"));
        assert!(extensions.contains("arw"));
    }
}