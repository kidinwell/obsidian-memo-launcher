use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn normalize_shortcut(shortcut_str: &str) -> String {
    shortcut_str
        .replace("Win+", "Super+")
        .replace("win+", "super+")
        .replace("CmdOrCtrl+", "CommandOrControl+")
}

pub fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, String> {
    let normalized = normalize_shortcut(shortcut_str);
    Shortcut::from_str(&normalized)
        .map_err(|e| format!("Invalid shortcut combination '{}': {:?}", shortcut_str, e))
}

pub fn register_launcher_shortcut(
    app: &AppHandle,
    shortcut_str: &str,
) -> Result<(), String> {
    let shortcut = parse_shortcut(shortcut_str)?;
    let global_shortcut = app.global_shortcut();

    // Unregister all previously registered shortcuts for clean slate
    let _ = global_shortcut.unregister_all();

    let app_handle = app.clone();
    global_shortcut
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.emit("launcher-opened", ());
                    }
                }
            }
        })
        .map_err(|e| format!("Failed to register global hotkey '{}': {:?}", shortcut_str, e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_and_parse_shortcut() {
        assert_eq!(normalize_shortcut("Win+Shift+Q"), "Super+Shift+Q");
        assert_eq!(normalize_shortcut("win+shift+q"), "super+shift+q");

        let parsed = parse_shortcut("Win+Shift+Q");
        assert!(parsed.is_ok(), "Parsed Win+Shift+Q successfully");

        let parsed_super = parse_shortcut("Super+Shift+Q");
        assert!(parsed_super.is_ok(), "Parsed Super+Shift+Q successfully");

        let invalid = parse_shortcut("InvalidNonExistentKeyCombo");
        assert!(invalid.is_err(), "Invalid shortcut should fail");
    }
}
