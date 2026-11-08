use crate::config::AppConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub title: String,
    pub requester: String,
    pub due: String,
    pub body: String,
    #[serde(default)]
    pub custom_properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTaskResult {
    pub success: bool,
    pub file_path: String,
    pub filename: String,
}

pub fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            other => other,
        })
        .collect();

    let trimmed = sanitized.trim().trim_matches('.').trim();
    if trimmed.is_empty() {
        "Untitled Task".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn resolve_unique_filepath(folder: &Path, base_name: &str) -> PathBuf {
    let candidate = folder.join(format!("{}.md", base_name));
    if !candidate.exists() {
        return candidate;
    }

    let mut index = 1;
    loop {
        let indexed_candidate = folder.join(format!("{}_{}.md", base_name, index));
        if !indexed_candidate.exists() {
            return indexed_candidate;
        }
        index += 1;
    }
}

pub fn generate_task_markdown(input: &TaskInput, config: &AppConfig) -> String {
    let mut frontmatter_lines = Vec::new();
    frontmatter_lines.push("---".to_string());

    if input.requester.trim().is_empty() {
        frontmatter_lines.push("requester: null".to_string());
    } else {
        frontmatter_lines.push(format!("requester: {}", input.requester.trim()));
    }

    if input.due.trim().is_empty() {
        frontmatter_lines.push("due: null".to_string());
    } else {
        frontmatter_lines.push(format!("due: {}", input.due.trim()));
    }

    frontmatter_lines.push("completed: null".to_string());

    for prop in &config.task_custom_properties {
        let val_opt = input.custom_properties.get(&prop.key);
        match val_opt {
            Some(serde_json::Value::String(s)) => {
                if s.trim().is_empty() {
                    frontmatter_lines.push(format!("{}: null", prop.key));
                } else {
                    frontmatter_lines.push(format!("{}: {}", prop.key, s.trim()));
                }
            }
            Some(serde_json::Value::Number(n)) => {
                frontmatter_lines.push(format!("{}: {}", prop.key, n));
            }
            Some(serde_json::Value::Bool(b)) => {
                frontmatter_lines.push(format!("{}: {}", prop.key, b));
            }
            _ => {
                frontmatter_lines.push(format!("{}: null", prop.key));
            }
        }
    }

    frontmatter_lines.push("---".to_string());
    let frontmatter = frontmatter_lines.join("\n");

    let body = input.body.trim();
    if body.is_empty() {
        frontmatter
    } else {
        format!("{}\n\n{}", frontmatter, body)
    }
}

pub fn create_task_note(
    input: &TaskInput,
    config: &AppConfig,
) -> Result<CreateTaskResult, String> {
    if config.vault_path.trim().is_empty() {
        return Err("Vault path is not configured. Please set Vault path in Settings.".to_string());
    }
    let vault_dir = Path::new(&config.vault_path);
    if !vault_dir.exists() {
        return Err(format!("Vault directory does not exist: {}", config.vault_path));
    }

    let tasks_dir = vault_dir.join(&config.tasks_folder);
    if !tasks_dir.exists() {
        fs::create_dir_all(&tasks_dir)
            .map_err(|e| format!("Failed to create tasks folder '{:?}': {}", tasks_dir, e))?;
    }

    let sanitized = sanitize_filename(&input.title);
    let target_path = resolve_unique_filepath(&tasks_dir, &sanitized);
    let markdown = generate_task_markdown(input, config);

    fs::write(&target_path, markdown)
        .map_err(|e| format!("Failed to write task file '{:?}': {}", target_path, e))?;

    let filename = target_path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| sanitized.clone());

    Ok(CreateTaskResult {
        success: true,
        file_path: target_path.to_string_lossy().to_string(),
        filename,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Normal Title"), "Normal Title");
        assert_eq!(sanitize_filename("Title/With:Disallowed*Chars?"), "Title-With-Disallowed-Chars-");
        assert_eq!(sanitize_filename("...trailing and leading dots..."), "trailing and leading dots");
        assert_eq!(sanitize_filename("   "), "Untitled Task");
    }

    #[test]
    fn test_resolve_unique_filepath() {
        let temp_dir = env::temp_dir().join(format!("obsidian_uniq_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let path1 = resolve_unique_filepath(&temp_dir, "MyTask");
        assert_eq!(path1, temp_dir.join("MyTask.md"));
        fs::write(&path1, "test1").unwrap();

        let path2 = resolve_unique_filepath(&temp_dir, "MyTask");
        assert_eq!(path2, temp_dir.join("MyTask_1.md"));
        fs::write(&path2, "test2").unwrap();

        let path3 = resolve_unique_filepath(&temp_dir, "MyTask");
        assert_eq!(path3, temp_dir.join("MyTask_2.md"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_generate_task_markdown() {
        let mut custom = HashMap::new();
        custom.insert("estimate".to_string(), serde_json::json!(2.5));

        let input = TaskInput {
            title: "新規機能の設計".to_string(),
            requester: "田中".to_string(),
            due: "2026-11-15".to_string(),
            body: "仕様書に従って実装を行う。".to_string(),
            custom_properties: custom,
        };

        let config = AppConfig::default();
        let markdown = generate_task_markdown(&input, &config);

        assert!(markdown.contains("requester: 田中"));
        assert!(markdown.contains("due: 2026-11-15"));
        assert!(markdown.contains("completed: null"));
        assert!(markdown.contains("estimate: 2.5"));
        assert!(markdown.ends_with("仕様書に従って実装を行う。"));
    }

    #[test]
    fn test_create_task_note_integration() {
        let temp_dir = env::temp_dir().join(format!("obsidian_task_integ_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let mut config = AppConfig::default();
        config.vault_path = temp_dir.to_str().unwrap().to_string();
        config.tasks_folder = "CustomTasks".to_string();

        let input = TaskInput {
            title: "テストタスク 1".to_string(),
            requester: "佐藤".to_string(),
            due: "2026-12-01".to_string(),
            body: "詳細なメモ".to_string(),
            custom_properties: HashMap::new(),
        };

        let result = create_task_note(&input, &config).expect("Failed to create task note");
        assert!(result.success);
        assert_eq!(result.filename, "テストタスク 1.md");
        assert!(Path::new(&result.file_path).exists());

        let content = fs::read_to_string(&result.file_path).unwrap();
        assert!(content.contains("requester: 佐藤"));
        assert!(content.contains("due: 2026-12-01"));
        assert!(content.contains("詳細なメモ"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
