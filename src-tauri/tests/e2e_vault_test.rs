use obsidian_task_launcher_lib::config::{AppConfig, CustomProperty};
use obsidian_task_launcher_lib::task_capture::{create_task_note, TaskInput};
use obsidian_task_launcher_lib::worklog_capture::{append_worklog, WorklogInput};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

struct TestVault {
    vault_path: PathBuf,
}

impl TestVault {
    fn setup() -> Self {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        let vault_path = env::temp_dir().join(format!("obsidian_e2e_vault_{}_{}", std::process::id(), count));
        if vault_path.exists() {
            let _ = fs::remove_dir_all(&vault_path);
        }
        fs::create_dir_all(&vault_path).unwrap();

        // Create .obsidian directory and daily-notes.json
        let obs_dir = vault_path.join(".obsidian");
        fs::create_dir_all(&obs_dir).unwrap();

        let daily_cfg = serde_json::json!({
            "format": "YYYY-MM-DD",
            "folder": "Daily",
            "template": "Templates/DailyTemplate"
        });
        fs::write(obs_dir.join("daily-notes.json"), daily_cfg.to_string()).unwrap();

        // Create Template directory and template note
        let tpl_dir = vault_path.join("Templates");
        fs::create_dir_all(&tpl_dir).unwrap();
        let tpl_content = r#"---
type: daily
created: {{date}}
---

# {{title}}

## 今日の目標
- [ ] 目標1

## 作業ログ

## 振り返り
Good day.
"#;
        fs::write(tpl_dir.join("DailyTemplate.md"), tpl_content).unwrap();

        // Create Tasks directory
        fs::create_dir_all(vault_path.join("Tasks")).unwrap();
        fs::create_dir_all(vault_path.join("Daily")).unwrap();

        Self { vault_path }
    }

    fn config(&self) -> AppConfig {
        AppConfig {
            vault_path: self.vault_path.to_string_lossy().to_string(),
            tasks_folder: "Tasks".to_string(),
            default_mode: "worklog".to_string(),
            worklog_section_heading: "## 作業ログ".to_string(),
            worklog_use_anchor: false,
            worklog_anchor_comment: "%% worklog %%".to_string(),
            global_shortcut: "Super+Shift+Q".to_string(),
            auto_hide_on_blur: true,
            task_custom_properties: vec![
                CustomProperty {
                    key: "estimate".to_string(),
                    label: "見積もり (h)".to_string(),
                    property_type: "number".to_string(),
                    default: None,
                },
                CustomProperty {
                    key: "priority".to_string(),
                    label: "優先度".to_string(),
                    property_type: "text".to_string(),
                    default: None,
                },
            ],
        }
    }
}

impl Drop for TestVault {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.vault_path);
    }
}

#[test]
fn test_e2e_task_capture_workflow() {
    let vault = TestVault::setup();
    let config = vault.config();

    // 1. Create a task with standard & custom properties
    let mut custom = HashMap::new();
    custom.insert("estimate".to_string(), serde_json::json!(4.5));
    custom.insert("priority".to_string(), serde_json::json!("高"));

    let task_input = TaskInput {
        title: "新機能のリリース準備: 動作検証 & レビュー".to_string(),
        requester: "田中".to_string(),
        due: "2026-11-20".to_string(),
        body: "- [ ] 結合テストの実施\n- [ ] PRレビューの依頼".to_string(),
        custom_properties: custom,
    };

    let result = create_task_note(&task_input, &config).expect("Task creation must succeed");
    assert!(result.success);
    assert_eq!(result.filename, "新機能のリリース準備- 動作検証 & レビュー.md");

    // 2. Verify file content and YAML frontmatter in Obsidian format
    let file_path = PathBuf::from(&result.file_path);
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();

    assert!(content.starts_with("---\n"));
    assert!(content.contains("requester: 田中\n"));
    assert!(content.contains("due: 2026-11-20\n"));
    assert!(content.contains("completed: null\n"));
    assert!(content.contains("estimate: 4.5\n"));
    assert!(content.contains("priority: 高\n"));
    assert!(content.contains("---\n\n- [ ] 結合テストの実施\n- [ ] PRレビューの依頼"));

    // 3. Create another task with identical title to verify duplicate index handling
    let duplicate_task = TaskInput {
        title: "新機能のリリース準備: 動作検証 & レビュー".to_string(),
        requester: "佐藤".to_string(),
        due: "2026-11-25".to_string(),
        body: "追加タスク".to_string(),
        custom_properties: HashMap::new(),
    };
    let dup_result = create_task_note(&duplicate_task, &config).expect("Duplicate task creation must succeed");
    assert_eq!(dup_result.filename, "新機能のリリース準備- 動作検証 & レビュー_1.md");
    assert!(PathBuf::from(&dup_result.file_path).exists());
}

#[test]
fn test_e2e_worklog_capture_workflow() {
    let vault = TestVault::setup();
    let config = vault.config();

    // 1. First work log entry - should initialize daily note from template
    let log1 = WorklogInput {
        memo: "朝会参加\n- スプリント目標の確認".to_string(),
        timestamp: Some("09:30".to_string()),
    };
    let res1 = append_worklog(&log1, &config).expect("First log append must succeed");
    assert!(res1.success);

    let daily_path = PathBuf::from(&res1.daily_note_path);
    assert!(daily_path.exists());

    let content1 = fs::read_to_string(&daily_path).unwrap();
    // Verify template was applied
    assert!(content1.contains("type: daily"));
    assert!(content1.contains("## 今日の目標"));
    assert!(content1.contains("## 作業ログ"));
    assert!(content1.contains("09:30\n朝会参加\n- スプリント目標の確認"));
    assert!(content1.contains("## 振り返り\nGood day."));

    // 2. Second work log entry - append chronologically
    let log2 = WorklogInput {
        memo: "仕様書の確認と設計ドラフト作成".to_string(),
        timestamp: Some("11:15".to_string()),
    };
    let res2 = append_worklog(&log2, &config).expect("Second log append must succeed");
    assert!(res2.success);

    // 3. Third work log entry - afternoon
    let log3 = WorklogInput {
        memo: "PR #42 のコードレビュー\n- テストケース追加の指摘".to_string(),
        timestamp: Some("15:45".to_string()),
    };
    let res3 = append_worklog(&log3, &config).expect("Third log append must succeed");
    assert!(res3.success);

    let final_content = fs::read_to_string(&daily_path).unwrap();

    // Verify all 3 entries are present in chronological order under ## 作業ログ
    let pos_heading = final_content.find("## 作業ログ").unwrap();
    let pos_log1 = final_content.find("09:30\n朝会参加").unwrap();
    let pos_log2 = final_content.find("11:15\n仕様書の確認").unwrap();
    let pos_log3 = final_content.find("15:45\nPR #42").unwrap();
    let pos_reflect = final_content.find("## 振り返り").unwrap();

    assert!(pos_heading < pos_log1);
    assert!(pos_log1 < pos_log2);
    assert!(pos_log2 < pos_log3);
    assert!(pos_log3 < pos_reflect);

    // Verify existing sections were not modified or corrupted
    assert!(final_content.contains("## 今日の目標\n- [ ] 目標1"));
    assert!(final_content.contains("Good day."));
}

#[test]
fn test_e2e_worklog_with_anchor_template_workflow() {
    let vault = TestVault::setup();
    let mut config = vault.config();
    config.worklog_use_anchor = true;
    config.worklog_anchor_comment = "%% worklog %%".to_string();

    // Overwrite template with %% worklog %% anchor comment
    let tpl_path = vault.vault_path.join("Templates").join("DailyTemplate.md");
    let tpl_content = r#"---
type: daily
created: {{date}}
---

# {{title}}

## 今日の目標
- [ ] 今日の目標を達成する

## 作業ログ

%% worklog %%

## 振り返り
お疲れ様でした。
"#;
    fs::write(&tpl_path, tpl_content).unwrap();

    // 1. First entry: Initializes daily note from template and inserts before anchor
    let log1 = WorklogInput {
        memo: "朝会参加\n- スプリントタスク確認".to_string(),
        timestamp: Some("09:15".to_string()),
    };
    let res1 = append_worklog(&log1, &config).expect("First log append with anchor must succeed");
    assert!(res1.success);

    let daily_path = PathBuf::from(&res1.daily_note_path);
    assert!(daily_path.exists());

    let content1 = fs::read_to_string(&daily_path).unwrap();
    assert!(content1.contains("type: daily"));
    assert!(content1.contains("## 今日の目標\n- [ ] 今日の目標を達成する"));
    assert!(content1.contains("09:15\n朝会参加\n- スプリントタスク確認"));
    assert!(content1.contains("%% worklog %%"));
    assert!(content1.contains("## 振り返り\nお疲れ様でした。"));

    let log1_pos_init = content1.find("09:15\n朝会参加").unwrap();
    let anchor_pos_init = content1.find("%% worklog %%").unwrap();
    let reflect_pos_init = content1.find("## 振り返り").unwrap();
    assert!(log1_pos_init < anchor_pos_init);
    assert!(anchor_pos_init < reflect_pos_init);

    // 2. Second entry: Insert chronologically before anchor
    let log2 = WorklogInput {
        memo: "アンカーコメント追記のE2Eテスト実装".to_string(),
        timestamp: Some("13:30".to_string()),
    };
    let res2 = append_worklog(&log2, &config).expect("Second log append with anchor must succeed");
    assert!(res2.success);

    // 3. Third entry: Afternoon log before anchor
    let log3 = WorklogInput {
        memo: "日報作成と退勤".to_string(),
        timestamp: Some("18:00".to_string()),
    };
    let res3 = append_worklog(&log3, &config).expect("Third log append with anchor must succeed");
    assert!(res3.success);

    let final_content = fs::read_to_string(&daily_path).unwrap();

    // Verify all 3 entries are present in order before %% worklog %%
    let pos_heading = final_content.find("## 作業ログ").unwrap();
    let pos_log1 = final_content.find("09:15\n朝会参加").unwrap();
    let pos_log2 = final_content.find("13:30\nアンカーコメント追記").unwrap();
    let pos_log3 = final_content.find("18:00\n日報作成と退勤").unwrap();
    let pos_anchor = final_content.find("%% worklog %%").unwrap();
    let pos_reflect = final_content.find("## 振り返り").unwrap();

    assert!(pos_heading < pos_log1);
    assert!(pos_log1 < pos_log2);
    assert!(pos_log2 < pos_log3);
    assert!(pos_log3 < pos_anchor);
    assert!(pos_anchor < pos_reflect);

    // Verify other sections remain intact
    assert!(final_content.contains("type: daily"));
    assert!(final_content.contains("## 今日の目標\n- [ ] 今日の目標を達成する"));
    assert!(final_content.contains("## 振り返り\nお疲れ様でした。"));
}

#[test]
fn test_e2e_worklog_fallback_without_obsidian_daily_notes_config() {
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    let vault_path = env::temp_dir().join(format!("obsidian_e2e_fallback_vault_{}_{}", std::process::id(), count));
    if vault_path.exists() {
        let _ = fs::remove_dir_all(&vault_path);
    }
    fs::create_dir_all(&vault_path).unwrap();

    let config = AppConfig {
        vault_path: vault_path.to_string_lossy().to_string(),
        tasks_folder: "Tasks".to_string(),
        default_mode: "worklog".to_string(),
        worklog_section_heading: "## 作業ログ".to_string(),
        worklog_use_anchor: false,
        worklog_anchor_comment: "%% worklog %%".to_string(),
        global_shortcut: "Super+Shift+Q".to_string(),
        auto_hide_on_blur: true,
        task_custom_properties: Vec::new(),
    };

    // Notice no .obsidian/daily-notes.json is created.
    // Append worklog: should fall back to Vault root and YYYY-MM-DD.md
    let log = WorklogInput {
        memo: "フォールバックテスト\n- ルート直下に作成".to_string(),
        timestamp: Some("12:00".to_string()),
    };
    let res = append_worklog(&log, &config).expect("Worklog append without daily-notes.json should succeed");
    assert!(res.success);

    let note_path = PathBuf::from(&res.daily_note_path);
    assert!(note_path.exists());
    // Note must be in the vault root
    assert_eq!(note_path.parent().unwrap(), vault_path.as_path());

    let content = fs::read_to_string(&note_path).unwrap();
    assert!(content.contains("## 作業ログ"));
    assert!(content.contains("12:00\nフォールバックテスト\n- ルート直下に作成"));

    let _ = fs::remove_dir_all(&vault_path);
}

#[test]
fn test_e2e_config_save_and_load_workflow() {
    use obsidian_task_launcher_lib::config::{load_config_from_path, save_config_to_path};
    let temp_dir = env::temp_dir().join(format!("obsidian_e2e_config_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();
    let config_path = temp_dir.join("config.json");

    let mut cfg = AppConfig::default();
    cfg.vault_path = "C:/Test/Vault".to_string();
    cfg.tasks_folder = "CustomTasks".to_string();
    cfg.worklog_section_heading = "## 今日のログ".to_string();
    cfg.worklog_use_anchor = true;
    cfg.worklog_anchor_comment = "<!-- log-here -->".to_string();

    assert!(save_config_to_path(&cfg, &config_path).is_ok());
    let loaded = load_config_from_path(&config_path);
    assert_eq!(cfg, loaded);

    let _ = fs::remove_dir_all(&temp_dir);
}
