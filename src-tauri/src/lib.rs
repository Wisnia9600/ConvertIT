mod cli;
mod conversion;
mod models;
mod presets;
mod shell;

use crate::cli::CliAction;

pub fn run() {
    let program_name = std::env::args()
        .next()
        .unwrap_or_else(|| "convertit.exe".to_string());
    let action = match cli::parse(std::env::args()) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{error}");
            cli::print_help(&program_name);
            std::process::exit(2);
        }
    };

    match action {
        CliAction::Convert(request) => {
            match conversion::run_conversion(&request, |_, _| {}) {
                Ok(result) => {
                    println!("Saved {} via {}", result.output_path, result.tool);
                    for line in result.log_summary {
                        println!("{line}");
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
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
        CliAction::Help => {
            cli::print_help(&program_name);
        }
    }
}
