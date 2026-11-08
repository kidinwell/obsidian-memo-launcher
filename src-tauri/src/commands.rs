use crate::config::{load_config, save_config, validate_vault_path, AppConfig};
use crate::task_capture::{create_task_note, CreateTaskResult, TaskInput};
use crate::worklog_capture::{append_worklog as append_worklog_impl, AppendWorklogResult, WorklogInput};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VaultValidationResult {
    pub is_valid: bool,
    pub has_obsidian_folder: bool,
    pub message: Option<String>,
}

#[tauri::command]
pub fn get_config() -> Result<AppConfig, String> {
    Ok(load_config())
}

#[tauri::command]
pub fn set_config(config: AppConfig) -> Result<(), String> {
    save_config(&config)
}

#[tauri::command]
pub fn update_hotkey(app: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    crate::hotkey::register_launcher_shortcut(&app, &shortcut)
}

#[tauri::command]
pub fn create_task(input: TaskInput) -> Result<CreateTaskResult, String> {
    let config = load_config();
    create_task_note(&input, &config)
}

#[tauri::command]
pub fn append_worklog(input: WorklogInput) -> Result<AppendWorklogResult, String> {
    let config = load_config();
    append_worklog_impl(&input, &config)
}

#[tauri::command]
pub fn validate_vault(vault_path: String) -> Result<VaultValidationResult, String> {
    match validate_vault_path(&vault_path) {
        Ok(path) => {
            let has_obsidian = path.join(".obsidian").exists();
            Ok(VaultValidationResult {
                is_valid: true,
                has_obsidian_folder: has_obsidian,
                message: None,
            })
        }
        Err(e) => Ok(VaultValidationResult {
            is_valid: false,
            has_obsidian_folder: false,
            message: Some(e),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_validate_vault_command() {
        let temp_dir = env::temp_dir().join(format!("obsidian_cmd_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();
        fs::create_dir_all(temp_dir.join(".obsidian")).unwrap();

        let res = validate_vault(temp_dir.to_str().unwrap().to_string()).unwrap();
        assert!(res.is_valid);
        assert!(res.has_obsidian_folder);

        let res_invalid = validate_vault("X:/InvalidVaultPath12345".to_string()).unwrap();
        assert!(!res_invalid.is_valid);
        assert!(res_invalid.message.is_some());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
