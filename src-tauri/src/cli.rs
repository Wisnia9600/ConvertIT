use crate::models::{ConversionRequest, LaunchContext, LaunchMode, QualityPreset};

pub enum CliAction {
    Launch(LaunchContext),
    Convert(ConversionRequest),
    InstallShell,
    UninstallShell,
}

pub fn parse<I>(args: I) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let mut values = args.into_iter();
    let _binary = values.next();

    let Some(subcommand) = values.next() else {
        return Ok(CliAction::Launch(LaunchContext::default()));
    };

    match subcommand.as_str() {
        "advanced" => {
            let input_path = parse_named_arg(values.collect(), "--input")?;
            Ok(CliAction::Launch(LaunchContext {
                mode: LaunchMode::Advanced,
                input_path,
            }))
        }
        "convert" => {
            let remaining = values.collect::<Vec<_>>();
            let input_path = parse_named_arg(remaining.clone(), "--input")?
                .ok_or_else(|| "Missing required --input argument".to_string())?;
            let preset_id = parse_named_arg(remaining, "--preset")?
                .ok_or_else(|| "Missing required --preset argument".to_string())?;
            Ok(CliAction::Convert(ConversionRequest {
                input_path,
                preset_id,
                quality_preset: QualityPreset::Balanced,
                open_folder_after_convert: false,
            }))
        }
        "install-shell" => Ok(CliAction::InstallShell),
        "uninstall-shell" => Ok(CliAction::UninstallShell),
        other => Err(format!("Unknown subcommand: {other}")),
    }
}

fn parse_named_arg(values: Vec<String>, name: &str) -> Result<Option<String>, String> {
    let mut iter = values.into_iter();
    while let Some(current) = iter.next() {
        if current == name {
            return iter
                .next()
                .map(Some)
                .ok_or_else(|| format!("Expected a value after {name}"));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::{parse, CliAction};

    #[test]
    fn parses_convert_command() {
        let action = parse(vec![
            "convertit.exe".to_string(),
            "convert".to_string(),
            "--input".to_string(),
            "C:/demo/file.mp4".to_string(),
            "--preset".to_string(),
            "video.mp4_to_gif".to_string(),
        ])
        .expect("convert command should parse");

        match action {
            CliAction::Convert(request) => {
                assert_eq!(request.input_path, "C:/demo/file.mp4");
                assert_eq!(request.preset_id, "video.mp4_to_gif");
            }
            _ => panic!("expected convert action"),
        }
    }
}