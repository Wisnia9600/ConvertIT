import { invoke } from "@tauri-apps/api/core";
import type {
  ConversionPreset,
  ConversionRequest,
  ConversionResult,
  LaunchContext,
  Settings,
} from "./types";

export const defaultSettings: Settings = {
  openFolderAfterConvert: true,
  defaultQualityPreset: "balanced",
  advancedLastOutputMode: "sibling",
  rememberSettings: true,
};

export async function getLaunchContext(): Promise<LaunchContext> {
  return invoke<LaunchContext>("get_launch_context");
}

export async function listPresetsForPath(
  inputPath: string | null,
): Promise<ConversionPreset[]> {
  return invoke<ConversionPreset[]>("list_presets_for_path", { inputPath });
}

export async function loadSettings(): Promise<Settings> {
  return invoke<Settings>("load_settings");
}

export async function saveSettings(settings: Settings): Promise<void> {
  return invoke("save_settings", { settings });
}

export async function runConversion(
  request: ConversionRequest,
): Promise<ConversionResult> {
  return invoke<ConversionResult>("run_conversion", { request });
}