use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomProperty {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub property_type: String,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    #[serde(default)]
    pub vault_path: String,
    #[serde(default = "default_tasks_folder")]
    pub tasks_folder: String,
    #[serde(default = "default_mode")]
    pub default_mode: String,
    #[serde(default = "default_worklog_section_heading")]
    pub worklog_section_heading: String,
    #[serde(default = "default_worklog_use_anchor")]
    pub worklog_use_anchor: bool,
    #[serde(default = "default_worklog_anchor_comment")]
    pub worklog_anchor_comment: String,
    #[serde(default = "default_daily_notes_folder")]
    pub daily_notes_folder: String,
    #[serde(default = "default_daily_note_format")]
    pub daily_note_format: String,
    #[serde(default)]
    pub daily_note_template_path: String,
    #[serde(default = "default_global_shortcut")]
    pub global_shortcut: String,
    #[serde(default = "default_auto_hide_on_blur")]
    pub auto_hide_on_blur: bool,
    #[serde(default = "default_task_custom_properties")]
    pub task_custom_properties: Vec<CustomProperty>,
}

fn default_tasks_folder() -> String {
    "Tasks".to_string()
}

fn default_mode() -> String {
    "worklog".to_string()
}

fn default_worklog_section_heading() -> String {
    "## 作業ログ".to_string()
}

fn default_worklog_use_anchor() -> bool {
    false
}

fn default_worklog_anchor_comment() -> String {
    "%% worklog %%".to_string()
}

fn default_daily_notes_folder() -> String {
    "Daily".to_string()
}

fn default_daily_note_format() -> String {
    "YYYY-MM-DD".to_string()
}

fn default_global_shortcut() -> String {
    "Super+Shift+Q".to_string()
}

fn default_auto_hide_on_blur() -> bool {
    true
}

fn default_task_custom_properties() -> Vec<CustomProperty> {
    vec![CustomProperty {
        key: "estimate".to_string(),
        label: "見積もり (h)".to_string(),
        property_type: "number".to_string(),
        default: None,
    }]
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            vault_path: String::new(),
            tasks_folder: default_tasks_folder(),
            default_mode: default_mode(),
            worklog_section_heading: default_worklog_section_heading(),
            worklog_use_anchor: default_worklog_use_anchor(),
            worklog_anchor_comment: default_worklog_anchor_comment(),
            daily_notes_folder: default_daily_notes_folder(),
            daily_note_format: default_daily_note_format(),
            daily_note_template_path: String::new(),
            global_shortcut: default_global_shortcut(),
            auto_hide_on_blur: default_auto_hide_on_blur(),
            task_custom_properties: default_task_custom_properties(),
        }
    }
}

pub fn get_config_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        config_dir.join("obsidian-task-launcher")
    } else {
        PathBuf::from(".obsidian-task-launcher")
    }
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

pub fn load_config_from_path(path: &Path) -> AppConfig {
    if !path.exists() {
        return AppConfig::default();
    }
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppConfig::default(),
    }
}

pub fn save_config_to_path(config: &AppConfig, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    let json_str = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(path, json_str).map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}

pub fn load_config() -> AppConfig {
    load_config_from_path(&get_config_path())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    save_config_to_path(config, &get_config_path())
}

pub fn validate_vault_path(path_str: &str) -> Result<PathBuf, String> {
    if path_str.trim().is_empty() {
        return Err("Vault path cannot be empty".to_string());
    }
    let path = PathBuf::from(path_str);
    if !path.exists() {
        return Err(format!("Vault directory does not exist: {}", path_str));
    }
    if !path.is_dir() {
        return Err(format!("Vault path is not a directory: {}", path_str));
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config_serialization_deserialization() {
        let default_cfg = AppConfig::default();
        let json_str = serde_json::to_string(&default_cfg).expect("Serialization failed");
        let deserialized: AppConfig = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(default_cfg, deserialized);
        assert_eq!(deserialized.default_mode, "worklog");
        assert_eq!(deserialized.worklog_section_heading, "## 作業ログ");
        assert_eq!(deserialized.worklog_use_anchor, false);
        assert_eq!(deserialized.worklog_anchor_comment, "%% worklog %%");
        assert_eq!(deserialized.tasks_folder, "Tasks");
        assert_eq!(deserialized.global_shortcut, "Super+Shift+Q");
    }

    #[test]
    fn test_custom_config_roundtrip() {
        let json_value = serde_json::json!({
            "vault_path": "C:/Vault",
            "tasks_folder": "MyTasks",
            "default_mode": "task",
            "worklog_section_heading": "## 業務ログ",
            "worklog_use_anchor": true,
            "worklog_anchor_comment": "<!-- worklog -->",
            "daily_notes_folder": "Journal",
            "daily_note_format": "YYYY/MM/DD",
            "daily_note_template_path": "Templates/Daily.md",
            "global_shortcut": "Win+Shift+Z",
            "auto_hide_on_blur": false,
            "task_custom_properties": [
                {
                    "key": "priority",
                    "label": "優先度",
                    "type": "text",
                    "default": "中"
                }
            ]
        });
        let json_str = json_value.to_string();

        let cfg: AppConfig = serde_json::from_str(&json_str).expect("Deserialization failed");
        assert_eq!(cfg.vault_path, "C:/Vault");
        assert_eq!(cfg.tasks_folder, "MyTasks");
        assert_eq!(cfg.default_mode, "task");
        assert_eq!(cfg.worklog_section_heading, "## 業務ログ");
        assert_eq!(cfg.worklog_use_anchor, true);
        assert_eq!(cfg.worklog_anchor_comment, "<!-- worklog -->");
        assert_eq!(cfg.auto_hide_on_blur, false);
        assert_eq!(cfg.task_custom_properties.len(), 1);
        assert_eq!(cfg.task_custom_properties[0].key, "priority");

        let serialized = serde_json::to_string_pretty(&cfg).expect("Serialization failed");
        let roundtrip: AppConfig = serde_json::from_str(&serialized).expect("Roundtrip deserialization failed");
        assert_eq!(cfg, roundtrip);
    }

    #[test]
    fn test_save_and_load_config() {
        let temp_dir = env::temp_dir().join(format!("obsidian_launcher_test_{}", std::process::id()));
        let config_file = temp_dir.join("test_config.json");

        let mut config = AppConfig::default();
        config.vault_path = "C:/TestVault".to_string();
        config.worklog_section_heading = "## Custom Log".to_string();

        let save_res = save_config_to_path(&config, &config_file);
        assert!(save_res.is_ok());

        let loaded = load_config_from_path(&config_file);
        assert_eq!(loaded.vault_path, "C:/TestVault");
        assert_eq!(loaded.worklog_section_heading, "## Custom Log");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_validate_vault_path() {
        let temp_dir = env::temp_dir().join(format!("obsidian_vault_val_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let valid = validate_vault_path(temp_dir.to_str().unwrap());
        assert!(valid.is_ok());

        let empty = validate_vault_path("");
        assert!(empty.is_err());

        let nonexistent = validate_vault_path("Z:/NonExistentPath12345/Vault");
        assert!(nonexistent.is_err());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
