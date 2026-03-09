use crate::models::{ConversionRequest, QualityPreset};

pub enum CliAction {
    Convert(ConversionRequest),
    InstallShell,
    UninstallShell,
    Help,
}

pub fn parse<I>(args: I) -> Result<CliAction, String>
where
    I: IntoIterator<Item = String>,
{
    let mut values = args.into_iter();
    let _binary = values.next();

    let Some(subcommand) = values.next() else {
        return Ok(CliAction::Help);
    };

    match subcommand.as_str() {
        "convert" => {
            let remaining = values.collect::<Vec<_>>();
            let input_path = parse_named_arg(remaining.clone(), "--input")?
                .ok_or_else(|| "Missing required --input argument".to_string())?;
            let preset_id = parse_named_arg(remaining.clone(), "--preset")?
                .ok_or_else(|| "Missing required --preset argument".to_string())?;
            let quality_preset = parse_named_arg(remaining.clone(), "--quality")?;
            Ok(CliAction::Convert(ConversionRequest {
                input_path,
                preset_id,
                quality_preset: quality_preset
                    .as_deref()
                    .map(parse_quality_preset)
                    .transpose()?
                    .unwrap_or_default(),
                open_folder_after_convert: has_flag(remaining, "--open-folder"),
            }))
        }
        "install-shell" => Ok(CliAction::InstallShell),
        "uninstall-shell" => Ok(CliAction::UninstallShell),
        "help" | "--help" | "-h" => Ok(CliAction::Help),
        other => Err(format!("Unknown subcommand: {other}")),
    }
}

pub fn print_help(program_name: &str) {
    println!("ConvertIT");
    println!();
    println!("Usage:");
    println!("  {program_name} convert --input <path> --preset <id> [--quality fast|balanced|best] [--open-folder]");
    println!("  {program_name} install-shell");
    println!("  {program_name} uninstall-shell");
    println!("  {program_name} help");
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

fn has_flag(values: Vec<String>, name: &str) -> bool {
    values.into_iter().any(|value| value == name)
}

fn parse_quality_preset(value: &str) -> Result<QualityPreset, String> {
    match value {
        "fast" => Ok(QualityPreset::Fast),
        "balanced" => Ok(QualityPreset::Balanced),
        "best" => Ok(QualityPreset::Best),
        other => Err(format!("Unsupported quality preset: {other}")),
    }
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

    #[test]
    fn defaults_to_help_without_subcommand() {
        let action = parse(vec!["convertit.exe".to_string()]).expect("command should parse");
        assert!(matches!(action, CliAction::Help));
    }

    #[test]
    fn parses_optional_convert_flags() {
        let action = parse(vec![
            "convertit.exe".to_string(),
            "convert".to_string(),
            "--input".to_string(),
            "C:/demo/file.mp4".to_string(),
            "--preset".to_string(),
            "video.mp4_to_gif".to_string(),
            "--quality".to_string(),
            "best".to_string(),
            "--open-folder".to_string(),
        ])
        .expect("convert command should parse");

        match action {
            CliAction::Convert(request) => {
                assert!(matches!(request.quality_preset, crate::models::QualityPreset::Best));
                assert!(request.open_folder_after_convert);
            }
            _ => panic!("expected convert action"),
        }
    }
}
