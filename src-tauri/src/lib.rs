pub mod commands;
pub mod config;
pub mod hotkey;
pub mod task_capture;
pub mod tray;
pub mod worklog_capture;

use tauri::{Emitter, WebviewWindow};

#[tauri::command]
fn show_launcher(window: WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    let _ = window.emit("launcher-opened", ());
    Ok(())
}

#[tauri::command]
fn hide_launcher(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            show_launcher,
            hide_launcher,
            commands::get_config,
            commands::set_config,
            commands::validate_vault,
            commands::update_hotkey,
            commands::create_task,
            commands::append_worklog
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let config = config::load_config();

            // Set up tray
            if let Err(e) = tray::setup_tray(app_handle) {
                eprintln!("Failed to setup system tray: {}", e);
            }

            // Register global shortcut
            let shortcut_str = if config.global_shortcut.is_empty() {
                "Super+Shift+Q"
            } else {
                &config.global_shortcut
            };
            if let Err(e) = hotkey::register_launcher_shortcut(app_handle, shortcut_str) {
                eprintln!("Failed to register initial global shortcut: {}", e);
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(focused) = event {
                if !focused {
                    let config = config::load_config();
                    if config.auto_hide_on_blur {
                        let _ = window.hide();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
