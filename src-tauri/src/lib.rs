mod cli;
mod conversion;
mod models;
mod presets;
mod settings;
mod shell;

use tauri::{AppHandle, Emitter, State};

use crate::cli::CliAction;
use crate::models::{ConversionRequest, ConversionResult, LaunchContext, Settings};

pub struct AppState {
    launch_context: LaunchContext,
}

mod commands {
    use super::*;

    #[tauri::command]
    pub(crate) fn get_launch_context(state: State<'_, AppState>) -> LaunchContext {
        state.launch_context.clone()
    }

    #[tauri::command]
    pub(crate) fn list_presets_for_path(input_path: Option<String>) -> Vec<models::ConversionPreset> {
        presets::for_input_path(input_path.as_deref())
    }

    #[tauri::command]
    pub(crate) fn load_settings() -> Result<Settings, String> {
        settings::load_settings()
    }

    #[tauri::command]
    pub(crate) fn save_settings(settings: Settings) -> Result<(), String> {
        settings::save_settings(&settings)
    }

    #[tauri::command]
    pub(crate) fn run_conversion(app: AppHandle, request: ConversionRequest) -> Result<ConversionResult, String> {
        conversion::run_conversion(&request, |event| {
            let _ = app.emit("conversion-progress", event);
        })
        .map_err(|error| error.to_string())
    }
}

pub fn run() {
    let action = match cli::parse(std::env::args()) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    };

    match action {
        CliAction::Convert(request) => {
            if let Err(error) = conversion::run_conversion(&request, |_| {}) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        CliAction::InstallShell => {
            let executable = std::env::current_exe().expect("current exe path should resolve");
            if let Err(error) = shell::install_shell_menu(&executable) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        CliAction::UninstallShell => {
            if let Err(error) = shell::uninstall_shell_menu() {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        CliAction::Launch(launch_context) => {
            tauri::Builder::default()
                .manage(AppState { launch_context })
                .invoke_handler(tauri::generate_handler![
                    commands::get_launch_context,
                    commands::list_presets_for_path,
                    commands::load_settings,
                    commands::save_settings,
                    commands::run_conversion,
                ])
                .run(tauri::generate_context!())
                .expect("error while running ConvertIT application");
        }
    }
}