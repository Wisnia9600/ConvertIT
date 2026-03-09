#[cfg(windows)]
use std::collections::BTreeMap;
#[cfg(windows)]
use std::path::Path;

#[cfg(windows)]
use windows_sys::Win32::UI::Shell::{SHCNE_ASSOCCHANGED, SHCNF_IDLIST, SHChangeNotify};
#[cfg(windows)]
use winreg::enums::HKEY_CURRENT_USER;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
use crate::presets;

#[cfg(windows)]
const ROOT_KEY: &str = "Software\\Classes\\SystemFileAssociations";
#[cfg(windows)]
const MENU_NAME: &str = "ConvertIT";

#[cfg(windows)]
pub fn install_shell_menu(executable_path: &Path) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let mut per_extension = BTreeMap::<&str, Vec<&str>>::new();

    for preset in presets::all() {
        for extension in preset.source_extensions {
            per_extension.entry(extension).or_default().push(preset.id);
        }
    }

    for (extension, preset_ids) in per_extension {
        let base_path = format!("{ROOT_KEY}\\.{}\\shell\\{MENU_NAME}", extension);
        let (base_key, _) = hkcu
            .create_subkey(&base_path)
            .map_err(|error| format!("Failed to create shell key {base_path}: {error}"))?;
        base_key
            .set_value("MUIVerb", &"Convert to")
            .map_err(|error| format!("Failed to set menu title for {extension}: {error}"))?;
        base_key
            .set_value("SubCommands", &"")
            .map_err(|error| format!("Failed to mark submenu for {extension}: {error}"))?;
        base_key
            .set_value("Icon", &executable_path.display().to_string())
            .map_err(|error| format!("Failed to set menu icon for {extension}: {error}"))?;

        for preset_id in preset_ids {
            let preset = presets::find_by_id(preset_id).expect("preset must exist");
            let command_path = format!("{base_path}\\shell\\{}", preset.id.replace('.', "_"));
            let (menu_key, _) = hkcu
                .create_subkey(&command_path)
                .map_err(|error| format!("Failed to create preset key {command_path}: {error}"))?;
            menu_key
                .set_value("MUIVerb", &preset.label)
                .map_err(|error| format!("Failed to set menu label for {}: {error}", preset.id))?;
            let (command_key, _) = hkcu
                .create_subkey(format!("{command_path}\\command"))
                .map_err(|error| format!("Failed to create command key for {}: {error}", preset.id))?;
            let command = build_shell_command(executable_path, preset.id);
            command_key
                .set_value("", &command)
                .map_err(|error| format!("Failed to register command for {}: {error}", preset.id))?;
        }
    }

    notify_shell_change();

    Ok(())
}

#[cfg(windows)]
pub fn uninstall_shell_menu() -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for extension in presets::supported_extensions() {
        let key_path = format!("{ROOT_KEY}\\.{}\\shell\\{MENU_NAME}", extension);
        match hkcu.delete_subkey_all(&key_path) {
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!("Failed to remove shell key tree {key_path}: {error}"));
            }
        }

        let parent_path = format!("{ROOT_KEY}\\.{}\\shell", extension);
        if let Ok(parent_key) = hkcu.open_subkey_with_flags(&parent_path, winreg::enums::KEY_ALL_ACCESS) {
            match parent_key.delete_subkey(MENU_NAME) {
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(format!("Failed to remove shell key root {key_path}: {error}"));
                }
            }
        }
    }

    notify_shell_change();

    Ok(())
}

#[cfg(windows)]
fn notify_shell_change() {
    unsafe {
        SHChangeNotify(SHCNE_ASSOCCHANGED as i32, SHCNF_IDLIST, std::ptr::null(), std::ptr::null());
    }
}

#[cfg(windows)]
fn build_shell_command(executable_path: &Path, preset_id: &str) -> String {
    let packaged_vbs_helper = executable_path
        .parent()
        .map(|parent| parent.join("convert-shell.vbs"))
        .unwrap_or_else(|| Path::new("convert-shell.vbs").to_path_buf());
    let repo_vbs_helper = std::env::current_dir()
        .map(|path| path.join("scripts").join("convert-shell.vbs"))
        .unwrap_or_else(|_| Path::new("scripts").join("convert-shell.vbs"));
    let vbs_helper = if packaged_vbs_helper.exists() {
        packaged_vbs_helper
    } else {
        repo_vbs_helper
    };

    if vbs_helper.exists() {
        format!(
            "wscript.exe \"{}\" \"{}\" \"%1\" \"{}\"",
            vbs_helper.display(),
            executable_path.display(),
            preset_id
        )
    } else {
        format!(
            "\"{}\" shell-convert --input \"%1\" --preset \"{}\"",
            executable_path.display(),
            preset_id
        )
    }
}

#[cfg(not(windows))]
pub fn install_shell_menu(_: &std::path::Path) -> Result<(), String> {
    Err("Shell integration is only available on Windows".to_string())
}

#[cfg(not(windows))]
pub fn uninstall_shell_menu() -> Result<(), String> {
    Err("Shell integration is only available on Windows".to_string())
}
