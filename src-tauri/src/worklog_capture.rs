use crate::config::AppConfig;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ObsidianDailyNotesConfig {
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub folder: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorklogInput {
    pub memo: String,
    #[serde(default)]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppendWorklogResult {
    pub success: bool,
    pub daily_note_path: String,
    pub entry_text: String,
}

pub fn format_date_with_pattern(dt: &DateTime<Local>, pattern: &str) -> String {
    let yyyy = dt.format("%Y").to_string();
    let mm = dt.format("%m").to_string();
    let dd = dt.format("%d").to_string();
    let hh = dt.format("%H").to_string();
    let min = dt.format("%M").to_string();

    pattern
        .replace("YYYY", &yyyy)
        .replace("yyyy", &yyyy)
        .replace("MM", &mm)
        .replace("DD", &dd)
        .replace("dd", &dd)
        .replace("HH", &hh)
        .replace("mm", &min)
}

pub fn read_obsidian_daily_notes_config(vault_path: &Path) -> Option<ObsidianDailyNotesConfig> {
    let config_file = vault_path.join(".obsidian").join("daily-notes.json");
    if config_file.exists() {
        if let Ok(content) = fs::read_to_string(config_file) {
            return serde_json::from_str(&content).ok();
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedDailySettings {
    pub folder: String,
    pub format: String,
    pub template_path: Option<PathBuf>,
}

pub fn resolve_daily_settings(vault_path: &Path, app_config: &AppConfig) -> ResolvedDailySettings {
    let obs_cfg = read_obsidian_daily_notes_config(vault_path).unwrap_or_default();

    let folder = obs_cfg
        .folder
        .filter(|f| !f.trim().is_empty())
        .unwrap_or_else(|| {
            if !app_config.daily_notes_folder.trim().is_empty() {
                app_config.daily_notes_folder.clone()
            } else {
                "Daily".to_string()
            }
        });

    let format = obs_cfg
        .format
        .filter(|f| !f.trim().is_empty())
        .unwrap_or_else(|| {
            if !app_config.daily_note_format.trim().is_empty() {
                app_config.daily_note_format.clone()
            } else {
                "YYYY-MM-DD".to_string()
            }
        });

    let template_rel = obs_cfg
        .template
        .filter(|t| !t.trim().is_empty())
        .or_else(|| {
            if !app_config.daily_note_template_path.trim().is_empty() {
                Some(app_config.daily_note_template_path.clone())
            } else {
                None
            }
        });

    let template_path = template_rel.and_then(|rel| {
        let direct = vault_path.join(&rel);
        if direct.exists() {
            Some(direct)
        } else {
            let with_md = vault_path.join(format!("{}.md", rel));
            if with_md.exists() {
                Some(with_md)
            } else {
                None
            }
        }
    });

    ResolvedDailySettings {
        folder,
        format,
        template_path,
    }
}

pub fn ensure_daily_note_exists(
    vault_path: &Path,
    settings: &ResolvedDailySettings,
    today_dt: &DateTime<Local>,
) -> Result<PathBuf, String> {
    let formatted_date = format_date_with_pattern(today_dt, &settings.format);
    let target_dir = if settings.folder.trim().is_empty() {
        vault_path.to_path_buf()
    } else {
        vault_path.join(&settings.folder)
    };

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create daily notes directory '{:?}': {}", target_dir, e))?;
    }

    let daily_note_path = target_dir.join(format!("{}.md", formatted_date));
    if !daily_note_path.exists() {
        let initial_content = if let Some(ref tpl_path) = settings.template_path {
            match fs::read_to_string(tpl_path) {
                Ok(raw) => raw
                    .replace("{{date}}", &formatted_date)
                    .replace("{{title}}", &formatted_date)
                    .replace("{{time}}", &today_dt.format("%H:%M").to_string()),
                Err(_) => format!("# {}\n\n", formatted_date),
            }
        } else {
            format!("# {}\n\n", formatted_date)
        };

        fs::write(&daily_note_path, initial_content)
            .map_err(|e| format!("Failed to create daily note '{:?}': {}", daily_note_path, e))?;
    }

    Ok(daily_note_path)
}

pub fn format_worklog_entry(timestamp: &str, memo: &str) -> String {
    format!("{}\n{}", timestamp, memo.trim())
}

pub fn insert_entry_into_content(content: &str, heading: &str, entry: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let trimmed_heading = heading.trim();
    let heading_level = trimmed_heading.chars().take_while(|&c| c == '#').count();

    let mut heading_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == trimmed_heading {
            heading_idx = Some(i);
            break;
        }
    }

    match heading_idx {
        Some(idx) => {
            let mut insert_idx = lines.len();
            for i in (idx + 1)..lines.len() {
                let line_trim = lines[i].trim();
                if line_trim.starts_with('#') {
                    let level = line_trim.chars().take_while(|&c| c == '#').count();
                    if level > 0 && level <= heading_level {
                        insert_idx = i;
                        break;
                    }
                }
            }

            let mut result = Vec::new();
            for i in 0..insert_idx {
                result.push(lines[i].to_string());
            }

            let last_line = result.last().map(|s| s.as_str()).unwrap_or("");
            if !last_line.is_empty() {
                result.push(String::new());
            }

            result.push(entry.to_string());
            result.push(String::new());

            for i in insert_idx..lines.len() {
                result.push(lines[i].to_string());
            }

            result.join("\n")
        }
        None => {
            let trimmed_content = content.trim_end();
            if trimmed_content.is_empty() {
                format!("{}\n\n{}\n", trimmed_heading, entry)
            } else {
                format!("{}\n\n{}\n\n{}\n", trimmed_content, trimmed_heading, entry)
            }
        }
    }
}

pub fn append_worklog(
    input: &WorklogInput,
    config: &AppConfig,
) -> Result<AppendWorklogResult, String> {
    if config.vault_path.trim().is_empty() {
        return Err("Vault path is not configured. Please set Vault path in Settings.".to_string());
    }
    let vault_dir = Path::new(&config.vault_path);
    if !vault_dir.exists() {
        return Err(format!("Vault directory does not exist: {}", config.vault_path));
    }

    let now: DateTime<Local> = Local::now();
    let settings = resolve_daily_settings(vault_dir, config);
    let daily_note_path = ensure_daily_note_exists(vault_dir, &settings, &now)?;

    let timestamp = input
        .timestamp
        .clone()
        .unwrap_or_else(|| now.format("%H:%M").to_string());

    let entry = format_worklog_entry(&timestamp, &input.memo);

    let existing_content = fs::read_to_string(&daily_note_path)
        .map_err(|e| format!("Failed to read daily note '{:?}': {}", daily_note_path, e))?;

    let updated_content = insert_entry_into_content(
        &existing_content,
        &config.worklog_section_heading,
        &entry,
    );

    fs::write(&daily_note_path, updated_content)
        .map_err(|e| format!("Failed to write daily note '{:?}': {}", daily_note_path, e))?;

    Ok(AppendWorklogResult {
        success: true,
        daily_note_path: daily_note_path.to_string_lossy().to_string(),
        entry_text: entry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_format_date_with_pattern() {
        let dt = Local::now();
        let formatted = format_date_with_pattern(&dt, "YYYY-MM-DD");
        assert_eq!(formatted, dt.format("%Y-%m-%d").to_string());

        let formatted_slash = format_date_with_pattern(&dt, "YYYY/MM/DD");
        assert_eq!(formatted_slash, dt.format("%Y/%m/%d").to_string());
    }

    #[test]
    fn test_insert_entry_into_existing_heading() {
        let content = r#"# 2026-11-08

## 作業ログ

10:00
朝会参加

## 振り返り
Good day!
"#;

        let entry = "11:30\nタスクAの実装";
        let updated = insert_entry_into_content(content, "## 作業ログ", entry);

        assert!(updated.contains("10:00\n朝会参加"));
        assert!(updated.contains("11:30\nタスクAの実装"));
        assert!(updated.contains("## 振り返り"));

        let log_pos = updated.find("## 作業ログ").unwrap();
        let new_entry_pos = updated.find("11:30\nタスクAの実装").unwrap();
        let reflect_pos = updated.find("## 振り返り").unwrap();

        assert!(log_pos < new_entry_pos);
        assert!(new_entry_pos < reflect_pos);
    }

    #[test]
    fn test_insert_entry_into_missing_heading() {
        let content = "# 2026-11-08\n\nDaily content.";
        let entry = "14:15\nレビュー対応";
        let updated = insert_entry_into_content(content, "## 作業ログ", entry);

        assert!(updated.contains("## 作業ログ"));
        assert!(updated.contains("14:15\nレビュー対応"));
    }

    #[test]
    fn test_resolve_daily_settings_with_obsidian_config() {
        let temp_dir = env::temp_dir().join(format!("obs_cfg_test_{}", std::process::id()));
        let obs_dir = temp_dir.join(".obsidian");
        fs::create_dir_all(&obs_dir).unwrap();

        let daily_cfg = serde_json::json!({
            "format": "YYYY_MM_DD",
            "folder": "DailyNotes",
            "template": "Templates/CustomDaily"
        });
        fs::write(obs_dir.join("daily-notes.json"), daily_cfg.to_string()).unwrap();

        let tpl_dir = temp_dir.join("Templates");
        fs::create_dir_all(&tpl_dir).unwrap();
        fs::write(tpl_dir.join("CustomDaily.md"), "# {{title}}\n\n## 作業ログ\n\n").unwrap();

        let app_cfg = AppConfig::default();
        let resolved = resolve_daily_settings(&temp_dir, &app_cfg);

        assert_eq!(resolved.format, "YYYY_MM_DD");
        assert_eq!(resolved.folder, "DailyNotes");
        assert!(resolved.template_path.is_some());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_append_worklog_integration() {
        let temp_dir = env::temp_dir().join(format!("obs_worklog_integ_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let mut app_cfg = AppConfig::default();
        app_cfg.vault_path = temp_dir.to_str().unwrap().to_string();
        app_cfg.daily_notes_folder = "Journal".to_string();
        app_cfg.daily_note_format = "YYYY-MM-DD".to_string();
        app_cfg.worklog_section_heading = "## 作業ログ".to_string();

        let input1 = WorklogInput {
            memo: "タスクの調査\n- 仕様書確認".to_string(),
            timestamp: Some("09:30".to_string()),
        };

        let res1 = append_worklog(&input1, &app_cfg).expect("First append failed");
        assert!(res1.success);
        assert!(Path::new(&res1.daily_note_path).exists());

        let content1 = fs::read_to_string(&res1.daily_note_path).unwrap();
        assert!(content1.contains("## 作業ログ"));
        assert!(content1.contains("09:30\nタスクの調査\n- 仕様書確認"));

        let input2 = WorklogInput {
            memo: "実装着手".to_string(),
            timestamp: Some("10:45".to_string()),
        };

        let res2 = append_worklog(&input2, &app_cfg).expect("Second append failed");
        assert!(res2.success);

        let content2 = fs::read_to_string(&res2.daily_note_path).unwrap();
        assert!(content2.contains("09:30\nタスクの調査\n- 仕様書確認"));
        assert!(content2.contains("10:45\n実装着手"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
