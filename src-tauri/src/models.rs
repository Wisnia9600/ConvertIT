#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct ConversionRequest {
    pub input_path: String,
    pub preset_id: String,
    pub quality_preset: QualityPreset,
    pub open_folder_after_convert: bool,
}

#[derive(Debug, Clone)]
pub struct ConversionResult {
    pub output_path: String,
    pub tool: String,
    pub log_summary: Vec<String>,
}
