export type QualityPreset = "fast" | "balanced" | "best";

export type Settings = {
  openFolderAfterConvert: boolean;
  defaultQualityPreset: QualityPreset;
  advancedLastOutputMode: "sibling";
  rememberSettings: boolean;
};

export type LaunchMode = "gui" | "advanced";

export type LaunchContext = {
  mode: LaunchMode;
  inputPath: string | null;
};

export type ConversionPreset = {
  id: string;
  label: string;
  sourceExtensions: string[];
  targetExtension: string;
  category: "video" | "audio" | "image";
  description: string;
};

export type ConversionRequest = {
  inputPath: string;
  presetId: string;
  qualityPreset: QualityPreset;
  openFolderAfterConvert: boolean;
};

export type ConversionResult = {
  outputPath: string;
  presetId: string;
  tool: string;
  logSummary: string[];
};

export type ConversionError = {
  message: string;
  details?: string;
};

export type ProgressEvent = {
  stage: "queued" | "starting" | "running" | "finished" | "failed";
  message: string;
};