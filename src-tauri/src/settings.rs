use std::fs;
use std::path::PathBuf;

use crate::models::Settings;

pub fn load_settings() -> Result<Settings, String> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(Settings::default());
    }

    let raw = fs::read_to_string(&path)
        .map_err(|error| format!("Failed to read settings file {}: {error}", path.display()))?;
    serde_json::from_str(&raw).map_err(|error| format!("Failed to parse settings: {error}"))
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Failed to create settings directory {}: {error}", parent.display()))?;
    }

    let content = serde_json::to_string_pretty(settings)
        .map_err(|error| format!("Failed to serialize settings: {error}"))?;
    fs::write(&path, content)
        .map_err(|error| format!("Failed to save settings file {}: {error}", path.display()))
}

fn settings_path() -> Result<PathBuf, String> {
    let Some(config_dir) = dirs::config_dir() else {
        return Err("Unable to resolve the user config directory".to_string());
    };

    Ok(config_dir.join("ConvertIT").join("settings.json"))
}