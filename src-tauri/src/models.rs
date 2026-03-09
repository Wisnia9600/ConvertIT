use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LaunchMode {
    Gui,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchContext {
    pub mode: LaunchMode,
    pub input_path: Option<String>,
}

impl Default for LaunchContext {
    fn default() -> Self {
        Self {
            mode: LaunchMode::Gui,
            input_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QualityPreset {
    Fast,
    Balanced,
    Best,
}

impl Default for QualityPreset {
    fn default() -> Self {
        Self::Balanced
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub open_folder_after_convert: bool,
    pub default_quality_preset: QualityPreset,
    pub advanced_last_output_mode: String,
    pub remember_settings: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            open_folder_after_convert: true,
            default_quality_preset: QualityPreset::Balanced,
            advanced_last_output_mode: "sibling".to_string(),
            remember_settings: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionPreset {
    pub id: String,
    pub label: String,
    pub source_extensions: Vec<String>,
    pub target_extension: String,
    pub category: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRequest {
    pub input_path: String,
    pub preset_id: String,
    pub quality_preset: QualityPreset,
    pub open_folder_after_convert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionResult {
    pub output_path: String,
    pub preset_id: String,
    pub tool: String,
    pub log_summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub stage: String,
    pub message: String,
}