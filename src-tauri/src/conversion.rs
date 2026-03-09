use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use thiserror::Error;

use crate::models::{ConversionRequest, ConversionResult, QualityPreset};
use crate::presets::{self, PresetDefinition, PresetKind};

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Unsupported preset: {0}")]
    UnsupportedPreset(String),
    #[error("Unsupported file extension for preset {preset}: {input}")]
    InvalidInput { preset: String, input: String },
    #[error("Missing required converter executable: {0}")]
    MissingTool(String),
    #[error("Failed to create output path: {0}")]
    OutputPath(String),
    #[error("Process failed for {tool}: {details}")]
    ProcessFailed { tool: String, details: String },
}

#[derive(Debug, Clone)]
struct QualityProfile {
    crf: &'static str,
    audio_bitrate: &'static str,
    ffmpeg_preset: &'static str,
    gif_fps: &'static str,
    gif_width: &'static str,
    image_quality: &'static str,
}

#[derive(Debug, Clone)]
struct CommandSpec {
    program: PathBuf,
    args: Vec<OsString>,
    tool: &'static str,
    log_summary: Vec<String>,
}

pub fn run_conversion<F>(request: &ConversionRequest, emit_progress: F) -> Result<ConversionResult, ConversionError>
where
    F: Fn(&str, &str),
{
    let preset = presets::find_by_id(&request.preset_id)
        .ok_or_else(|| ConversionError::UnsupportedPreset(request.preset_id.clone()))?;

    validate_input_extension(&request.input_path, preset)?;

    emit_progress("starting", &format!("Preparing {}", preset.label));

    let input_path = PathBuf::from(&request.input_path);
    let output_path = next_output_path(&input_path, preset.target_extension)?;
    let quality = quality_profile(&request.quality_preset);

    let result = if matches!(preset.kind, PresetKind::RawToJpg) {
        emit_progress("running", "Running LibRaw decode and JPG encode");
        run_raw_conversion(request, &input_path, &output_path, &quality)?
    } else {
        let command = build_command(preset, &input_path, &output_path, &quality)?;
        emit_progress("running", &format!("Running {}", command.tool));
        let output = execute_command(&command)?;
        ConversionResult {
            output_path: output_path.display().to_string(),
            tool: command.tool.to_string(),
            log_summary: summarize_output(&command, &output),
        }
    };

    if request.open_folder_after_convert {
        reveal_output_path(&output_path);
    }

    emit_progress("finished", &format!("Saved {}", output_path.display()));

    Ok(result)
}

fn validate_input_extension(input_path: &str, preset: &PresetDefinition) -> Result<(), ConversionError> {
    let extension = presets::normalized_extension(input_path).ok_or_else(|| ConversionError::InvalidInput {
        preset: preset.id.to_string(),
        input: input_path.to_string(),
    })?;

    if preset.source_extensions.iter().any(|candidate| *candidate == extension) {
        Ok(())
    } else {
        Err(ConversionError::InvalidInput {
            preset: preset.id.to_string(),
            input: input_path.to_string(),
        })
    }
}

fn build_command(
    preset: &PresetDefinition,
    input_path: &Path,
    output_path: &Path,
    quality: &QualityProfile,
) -> Result<CommandSpec, ConversionError> {
    let input = input_path.as_os_str().to_os_string();
    let output = output_path.as_os_str().to_os_string();

    let command = match preset.kind {
        PresetKind::Mp4ToGif => CommandSpec {
            program: resolve_tool("ffmpeg.exe")?,
            args: vec![
                OsString::from("-y"),
                OsString::from("-i"),
                input,
                OsString::from("-vf"),
                OsString::from(format!(
                    "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen=stats_mode=diff[p];[s1][p]paletteuse=dither=bayer",
                    quality.gif_fps, quality.gif_width
                )),
                output,
            ],
            tool: "ffmpeg",
            log_summary: vec!["palette generated GIF workflow".to_string()],
        },
        PresetKind::Mp4ToWebm => video_transcode(resolve_tool("ffmpeg.exe")?, input, output, "libvpx-vp9", "libopus", quality, vec!["web-friendly VP9".to_string()]),
        PresetKind::WebmToMp4 | PresetKind::MovToMp4 | PresetKind::MkvToMp4 | PresetKind::AviToMp4 => {
            video_transcode(resolve_tool("ffmpeg.exe")?, input, output, "libx264", "aac", quality, vec!["H.264/AAC output".to_string()])
        }
        PresetKind::Mp4ToMov => video_transcode(resolve_tool("ffmpeg.exe")?, input, output, "libx264", "aac", quality, vec!["Apple-friendly MOV output".to_string()]),
        PresetKind::Mp4ToAvi => video_transcode(resolve_tool("ffmpeg.exe")?, input, output, "mpeg4", "libmp3lame", quality, vec!["legacy AVI output".to_string()]),
        PresetKind::Mp4ToMkv => video_transcode(resolve_tool("ffmpeg.exe")?, input, output, "libx264", "aac", quality, vec!["Matroska output".to_string()]),
        PresetKind::VideoToMp3 => audio_extract(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("libmp3lame"), OsString::from("-b:a"), OsString::from(quality.audio_bitrate)], vec!["audio extracted to MP3".to_string()]),
        PresetKind::VideoToWav => audio_extract(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("pcm_s16le")], vec!["audio extracted to WAV".to_string()]),
        PresetKind::PngToJpg | PresetKind::JpgToPng | PresetKind::RasterToWebp | PresetKind::HeicToJpg | PresetKind::SvgToPng | PresetKind::SvgToJpg => {
            magick_convert(resolve_tool("magick.exe")?, input, output, quality)
        }
        PresetKind::RawToJpg => unreachable!("RAW conversion is handled separately"),
        PresetKind::Mp3ToWav => audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("pcm_s16le")], vec!["decoded to WAV".to_string()]),
        PresetKind::WavToMp3 | PresetKind::FlacToMp3 | PresetKind::OggToMp3 | PresetKind::AacToMp3 | PresetKind::M4aToMp3 => {
            audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("libmp3lame"), OsString::from("-b:a"), OsString::from(quality.audio_bitrate)], vec!["encoded as MP3".to_string()])
        }
        PresetKind::Mp3ToFlac => audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("flac")], vec!["encoded as FLAC".to_string()]),
        PresetKind::Mp3ToOgg => audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("libvorbis"), OsString::from("-q:a"), OsString::from("5")], vec!["encoded as OGG".to_string()]),
        PresetKind::Mp3ToAac => audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("aac"), OsString::from("-b:a"), OsString::from(quality.audio_bitrate)], vec!["encoded as AAC".to_string()]),
        PresetKind::Mp3ToM4a => audio_convert(resolve_tool("ffmpeg.exe")?, input, output, vec![OsString::from("-c:a"), OsString::from("aac"), OsString::from("-b:a"), OsString::from(quality.audio_bitrate)], vec!["encoded as M4A".to_string()]),
    };

    Ok(command)
}

fn video_transcode(
    ffmpeg_path: PathBuf,
    input: OsString,
    output: OsString,
    video_codec: &str,
    audio_codec: &str,
    quality: &QualityProfile,
    log_summary: Vec<String>,
) -> CommandSpec {
    CommandSpec {
        program: ffmpeg_path,
        args: vec![
            OsString::from("-y"),
            OsString::from("-i"),
            input,
            OsString::from("-c:v"),
            OsString::from(video_codec),
            OsString::from("-preset"),
            OsString::from(quality.ffmpeg_preset),
            OsString::from("-crf"),
            OsString::from(quality.crf),
            OsString::from("-c:a"),
            OsString::from(audio_codec),
            OsString::from("-b:a"),
            OsString::from(quality.audio_bitrate),
            output,
        ],
        tool: "ffmpeg",
        log_summary,
    }
}

fn audio_extract(
    ffmpeg_path: PathBuf,
    input: OsString,
    output: OsString,
    mut codec_args: Vec<OsString>,
    log_summary: Vec<String>,
) -> CommandSpec {
    let mut args = vec![
        OsString::from("-y"),
        OsString::from("-i"),
        input,
        OsString::from("-vn"),
    ];
    args.append(&mut codec_args);
    args.push(output);
    CommandSpec {
        program: ffmpeg_path,
        args,
        tool: "ffmpeg",
        log_summary,
    }
}

fn audio_convert(
    ffmpeg_path: PathBuf,
    input: OsString,
    output: OsString,
    mut codec_args: Vec<OsString>,
    log_summary: Vec<String>,
) -> CommandSpec {
    let mut args = vec![OsString::from("-y"), OsString::from("-i"), input];
    args.append(&mut codec_args);
    args.push(output);
    CommandSpec {
        program: ffmpeg_path,
        args,
        tool: "ffmpeg",
        log_summary,
    }
}

fn magick_convert(
    magick_path: PathBuf,
    input: OsString,
    output: OsString,
    quality: &QualityProfile,
) -> CommandSpec {
    CommandSpec {
        program: magick_path,
        args: vec![
            input,
            OsString::from("-quality"),
            OsString::from(quality.image_quality),
            output,
        ],
        tool: "ImageMagick",
        log_summary: vec!["ImageMagick raster conversion".to_string()],
    }
}

fn run_raw_conversion(
    _request: &ConversionRequest,
    input_path: &Path,
    output_path: &Path,
    quality: &QualityProfile,
) -> Result<ConversionResult, ConversionError> {
    let dcraw_path = resolve_tool("dcraw_emu.exe")?;
    let libraw_path = resolve_tool("libraw.dll")?;
    let magick_path = resolve_tool("magick.exe")?;
    let temp_ppm = output_path.with_extension("ppm");

    let dcraw_output = Command::new(&dcraw_path)
        .env("PATH", format!("{};{}", libraw_path.parent().unwrap_or(Path::new(".")).display(), std::env::var("PATH").unwrap_or_default()))
        .args([
            OsString::from("-w"),
            OsString::from("-o"),
            OsString::from("1"),
            OsString::from("-6"),
            OsString::from("-Z"),
            OsString::from("-"),
            input_path.as_os_str().to_os_string(),
        ])
        .output()
        .map_err(|error| ConversionError::ProcessFailed {
            tool: "dcraw_emu".to_string(),
            details: format!("Failed to launch {}: {error}", dcraw_path.display()),
        })?;

    if !dcraw_output.status.success() {
        let details = String::from_utf8_lossy(&dcraw_output.stderr).trim().to_string();
        return Err(ConversionError::ProcessFailed {
            tool: "dcraw_emu".to_string(),
            details,
        });
    }

    fs::write(&temp_ppm, &dcraw_output.stdout)
        .map_err(|error| ConversionError::ProcessFailed {
            tool: "dcraw_emu".to_string(),
            details: format!("Failed to write temporary RAW decode output: {error}"),
        })?;

    let magick_output = Command::new(&magick_path)
        .args([
            temp_ppm.as_os_str().to_os_string(),
            OsString::from("-quality"),
            OsString::from(quality.image_quality),
            output_path.as_os_str().to_os_string(),
        ])
        .output()
        .map_err(|error| ConversionError::ProcessFailed {
            tool: "ImageMagick".to_string(),
            details: format!("Failed to launch {}: {error}", magick_path.display()),
        })?;

    let _ = fs::remove_file(&temp_ppm);

    if !magick_output.status.success() {
        let details = String::from_utf8_lossy(&magick_output.stderr).trim().to_string();
        return Err(ConversionError::ProcessFailed {
            tool: "ImageMagick".to_string(),
            details,
        });
    }

    Ok(ConversionResult {
        output_path: output_path.display().to_string(),
        tool: "dcraw_emu + ImageMagick".to_string(),
        log_summary: vec![
            "RAW decode pipeline".to_string(),
            "temporary PPM staging".to_string(),
        ],
    })
}

fn execute_command(command: &CommandSpec) -> Result<Output, ConversionError> {
    let output = Command::new(&command.program)
        .args(&command.args)
        .output()
        .map_err(|error| ConversionError::ProcessFailed {
            tool: command.tool.to_string(),
            details: format!("Failed to launch {}: {error}", command.program.display()),
        })?;

    if output.status.success() {
        Ok(output)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let details = if !stderr.trim().is_empty() {
            stderr.trim().to_string()
        } else {
            stdout.trim().to_string()
        };
        Err(ConversionError::ProcessFailed {
            tool: command.tool.to_string(),
            details,
        })
    }
}

fn summarize_output(command: &CommandSpec, output: &Output) -> Vec<String> {
    let mut summary = command.log_summary.clone();
    if !output.status.success() {
        summary.push(format!("exit code {:?}", output.status.code()));
    }
    summary
}

fn reveal_output_path(output_path: &Path) {
    #[cfg(windows)]
    {
        let select_arg = OsString::from(format!(r#"/select,"{}""#, output_path.display()));
        if Command::new("explorer.exe").arg(select_arg).spawn().is_ok() {
            return;
        }
    }

    if let Some(folder) = output_path.parent() {
        let _ = open::that_detached(folder);
    }
}

fn quality_profile(quality: &QualityPreset) -> QualityProfile {
    match quality {
        QualityPreset::Fast => QualityProfile {
            crf: "28",
            audio_bitrate: "128k",
            ffmpeg_preset: "veryfast",
            gif_fps: "10",
            gif_width: "720",
            image_quality: "78",
        },
        QualityPreset::Balanced => QualityProfile {
            crf: "24",
            audio_bitrate: "192k",
            ffmpeg_preset: "medium",
            gif_fps: "12",
            gif_width: "960",
            image_quality: "86",
        },
        QualityPreset::Best => QualityProfile {
            crf: "18",
            audio_bitrate: "256k",
            ffmpeg_preset: "slow",
            gif_fps: "15",
            gif_width: "1280",
            image_quality: "92",
        },
    }
}

fn next_output_path(input_path: &Path, extension: &str) -> Result<PathBuf, ConversionError> {
    let parent = input_path.parent().ok_or_else(|| {
        ConversionError::OutputPath(format!("No parent directory for {}", input_path.display()))
    })?;
    let stem = input_path
        .file_stem()
        .ok_or_else(|| ConversionError::OutputPath(format!("No filename stem for {}", input_path.display())))?
        .to_string_lossy();

    let candidate = parent.join(format!("{}.converted.{}", stem, extension));
    if !candidate.exists() {
        return Ok(candidate);
    }

    for index in 2..1000 {
        let candidate = parent.join(format!("{}.converted ({index}).{}", stem, extension));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(ConversionError::OutputPath(format!(
        "Could not find a free output name next to {}",
        input_path.display()
    )))
}

fn resolve_tool(binary_name: &str) -> Result<PathBuf, ConversionError> {
    let mut candidates = Vec::new();

    if let Ok(tool_dir) = std::env::var("CONVERTIT_TOOL_DIR") {
        candidates.push(PathBuf::from(tool_dir).join(binary_name));
    }

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            candidates.push(parent.join("vendor").join("bin").join(binary_name));
            candidates.push(parent.join("resources").join("vendor").join("bin").join(binary_name));
        }
    }

    if let Ok(current_dir) = std::env::current_dir() {
        candidates.push(current_dir.join("vendor").join("bin").join(binary_name));
    }

    for candidate in candidates {
        if fs::metadata(&candidate).is_ok() {
            return Ok(candidate);
        }
    }

    Err(ConversionError::MissingTool(binary_name.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::next_output_path;

    #[test]
    fn creates_incremented_output_names() {
        let temp_dir = std::env::temp_dir().join(format!("convertit-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).expect("temp directory should be created");

        let input_path = temp_dir.join("demo.mp4");
        fs::write(&input_path, "source").expect("input file should exist");
        fs::write(temp_dir.join("demo.converted.gif"), "existing").expect("first collision file should exist");

        let output = next_output_path(&input_path, "gif").expect("should create a new candidate");
        assert!(output.ends_with("demo.converted (2).gif"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
