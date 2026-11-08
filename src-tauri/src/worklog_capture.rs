use crate::config::AppConfig;
use chrono::{DateTime, Datelike, Local, Timelike};
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ObsidianAppConfig {
    #[serde(default, rename = "userLocale")]
    pub user_locale: Option<String>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub locale: Option<String>,
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

pub fn get_system_locale() -> String {
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn GetUserDefaultLocaleName(lpLocaleName: *mut u16, cchLocaleName: i32) -> i32;
        }
        let mut buf = [0u16; 85];
        let ret = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
        if ret > 0 {
            let len = (ret - 1) as usize;
            let name = String::from_utf16_lossy(&buf[..len]);
            if !name.trim().is_empty() {
                return name;
            }
        }
    }

    std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "en".to_string())
}

pub fn is_japanese_locale(locale: &str) -> bool {
    let lower = locale.to_lowercase();
    lower.starts_with("ja")
}

pub fn read_obsidian_app_locale(vault_path: &Path) -> Option<String> {
    let app_file = vault_path.join(".obsidian").join("app.json");
    if app_file.exists() {
        if let Ok(content) = fs::read_to_string(app_file) {
            if let Ok(cfg) = serde_json::from_str::<ObsidianAppConfig>(&content) {
                return cfg
                    .user_locale
                    .or(cfg.language)
                    .or(cfg.locale)
                    .filter(|l| !l.trim().is_empty());
            }
        }
    }
    None
}

pub fn resolve_locale(vault_path: &Path) -> String {
    if let Some(locale) = read_obsidian_app_locale(vault_path) {
        return locale;
    }
    get_system_locale()
}

pub fn format_moment_date<Tz: chrono::TimeZone>(
    dt: &DateTime<Tz>,
    pattern: &str,
    locale: Option<&str>,
) -> String {
    let loc = locale
        .map(|s| s.to_string())
        .unwrap_or_else(get_system_locale);
    let is_ja = is_japanese_locale(&loc);

    let year = dt.year();
    let month = dt.month(); // 1..=12
    let day = dt.day(); // 1..=31
    let hour24 = dt.hour(); // 0..=23
    let minute = dt.minute(); // 0..=59
    let second = dt.second(); // 0..=59
    let weekday_sunday_idx = dt.weekday().num_days_from_sunday() as usize; // 0..=6 (0 = Sunday)
    let month_idx = (month.saturating_sub(1) as usize).min(11);

    let yyyy_str = format!("{:04}", year);
    let yy_str = format!("{:02}", year.rem_euclid(100));

    let en_months_full = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ];
    let en_months_short = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let ja_months = [
        "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
    ];

    let mmmm_str = if is_ja {
        ja_months[month_idx]
    } else {
        en_months_full[month_idx]
    };
    let mmm_str = if is_ja {
        ja_months[month_idx]
    } else {
        en_months_short[month_idx]
    };
    let mm_str = format!("{:02}", month);
    let m_str = format!("{}", month);

    let dd_str = format!("{:02}", day);
    let d_str = format!("{}", day);

    let en_weekdays_full = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let en_weekdays_short = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let en_weekdays_min = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    let ja_weekdays_full = [
        "日曜日",
        "月曜日",
        "火曜日",
        "水曜日",
        "木曜日",
        "金曜日",
        "土曜日",
    ];
    let ja_weekdays_short = ["日", "月", "火", "水", "木", "金", "土"];
    let ja_weekdays_min = ["日", "月", "火", "水", "木", "金", "土"];

    let dddd_str = if is_ja {
        ja_weekdays_full[weekday_sunday_idx]
    } else {
        en_weekdays_full[weekday_sunday_idx]
    };
    let ddd_str = if is_ja {
        ja_weekdays_short[weekday_sunday_idx]
    } else {
        en_weekdays_short[weekday_sunday_idx]
    };
    let min_dd_str = if is_ja {
        ja_weekdays_min[weekday_sunday_idx]
    } else {
        en_weekdays_min[weekday_sunday_idx]
    };
    let weekday_num_str = format!("{}", weekday_sunday_idx);

    let hh24_str = format!("{:02}", hour24);
    let h24_str = format!("{}", hour24);

    let h12 = if hour24 == 0 {
        12
    } else if hour24 > 12 {
        hour24 - 12
    } else {
        hour24
    };
    let hh12_str = format!("{:02}", h12);
    let h12_str = format!("{}", h12);

    let min_str = format!("{:02}", minute);
    let m_minute_str = format!("{}", minute);

    let sec_str = format!("{:02}", second);
    let s_sec_str = format!("{}", second);

    let a_upper_str = if is_ja {
        if hour24 < 12 {
            "午前"
        } else {
            "午後"
        }
    } else if hour24 < 12 {
        "AM"
    } else {
        "PM"
    };
    let a_lower_str = if is_ja {
        if hour24 < 12 {
            "午前"
        } else {
            "午後"
        }
    } else if hour24 < 12 {
        "am"
    } else {
        "pm"
    };

    let quarter_str = format!("{}", ((month - 1) / 3) + 1);

    // List of tokens and their string replacements, ordered by descending token length
    let token_replacements: [(&str, &str); 25] = [
        // Length 4
        ("YYYY", &yyyy_str),
        ("yyyy", &yyyy_str),
        ("MMMM", mmmm_str),
        ("dddd", dddd_str),
        // Length 3
        ("MMM", mmm_str),
        ("ddd", ddd_str),
        // Length 2
        ("YY", &yy_str),
        ("yy", &yy_str),
        ("MM", &mm_str),
        ("DD", &dd_str),
        ("dd", min_dd_str),
        ("HH", &hh24_str),
        ("hh", &hh12_str),
        ("mm", &min_str),
        ("ss", &sec_str),
        // Length 1
        ("M", &m_str),
        ("D", &d_str),
        ("d", &weekday_num_str),
        ("H", &h24_str),
        ("h", &h12_str),
        ("m", &m_minute_str),
        ("s", &s_sec_str),
        ("A", a_upper_str),
        ("a", a_lower_str),
        ("Q", &quarter_str),
    ];

    let mut result = String::new();
    let mut i = 0;

    while i < pattern.len() {
        let remainder = &pattern[i..];

        // 1. Check for [...] escape bracket
        if remainder.starts_with('[') {
            if let Some(close_pos) = remainder.find(']') {
                let inside = &remainder[1..close_pos];
                result.push_str(inside);
                i += close_pos + 1;
                continue;
            } else {
                result.push_str(&remainder[1..]);
                break;
            }
        }

        // 2. Check for tokens in descending order of length
        let mut matched = false;
        for &(token, value) in &token_replacements {
            if remainder.starts_with(token) {
                result.push_str(value);
                i += token.len();
                matched = true;
                break;
            }
        }

        // 3. Fallback to literal char
        if !matched {
            let ch = remainder.chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }

    result
}

pub fn format_date_with_pattern(dt: &DateTime<Local>, pattern: &str) -> String {
    format_moment_date(dt, pattern, None)
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
    pub locale: Option<String>,
}

pub fn resolve_daily_settings(vault_path: &Path) -> ResolvedDailySettings {
    let obs_cfg = read_obsidian_daily_notes_config(vault_path).unwrap_or_default();
    let locale = resolve_locale(vault_path);

    let folder = obs_cfg
        .folder
        .filter(|f| !f.trim().is_empty())
        .unwrap_or_default();

    let format = obs_cfg
        .format
        .filter(|f| !f.trim().is_empty())
        .unwrap_or_else(|| "YYYY-MM-DD".to_string());

    let template_rel = obs_cfg
        .template
        .filter(|t| !t.trim().is_empty());

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
        locale: Some(locale),
    }
}

pub fn ensure_daily_note_exists(
    vault_path: &Path,
    settings: &ResolvedDailySettings,
    today_dt: &DateTime<Local>,
) -> Result<PathBuf, String> {
    let formatted_date = format_moment_date(today_dt, &settings.format, settings.locale.as_deref());
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

fn is_worklog_timestamp_heading(line: &str) -> bool {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("### ") {
        let rest = rest.trim();
        let parts: Vec<&str> = rest.split(':').collect();
        if parts.len() == 2 && parts[0].len() == 2 && parts[1].len() == 2 {
            return parts[0].chars().all(|c| c.is_ascii_digit())
                && parts[1].chars().all(|c| c.is_ascii_digit());
        }
    }
    false
}

pub fn format_worklog_entry(timestamp: &str, memo: &str) -> String {
    let ts = timestamp.trim();
    let header = if ts.starts_with("### ") || ts.starts_with('#') {
        ts.to_string()
    } else {
        format!("### {}", ts)
    };
    if memo.trim().is_empty() {
        header
    } else {
        format!("{}\n{}", header, memo.trim())
    }
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
                    if level > 0 && level <= heading_level && !is_worklog_timestamp_heading(line_trim) {
                        insert_idx = i;
                        break;
                    }
                }
            }

            let has_existing_entry = lines[(idx + 1)..insert_idx]
                .iter()
                .any(|line| !line.trim().is_empty());

            let mut result = Vec::new();
            for i in 0..insert_idx {
                result.push(lines[i].to_string());
            }

            while let Some(last) = result.last() {
                if last.is_empty() {
                    result.pop();
                } else {
                    break;
                }
            }

            if !has_existing_entry {
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

pub fn insert_entry_before_anchor(content: &str, anchor: &str, entry: &str) -> Option<String> {
    let trimmed_anchor = anchor.trim();
    if trimmed_anchor.is_empty() {
        return None;
    }
    let lines: Vec<&str> = content.lines().collect();
    let mut anchor_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == trimmed_anchor {
            anchor_idx = Some(i);
            break;
        }
    }

    let idx = anchor_idx?;

    let mut section_start_idx = 0;
    for i in (0..idx).rev() {
        let line_trim = lines[i].trim();
        if line_trim.starts_with('#') && !is_worklog_timestamp_heading(line_trim) {
            section_start_idx = i;
            break;
        }
    }

    let has_existing_entry = lines[(section_start_idx + 1)..idx]
        .iter()
        .any(|line| !line.trim().is_empty());

    let mut result = Vec::new();
    for i in 0..idx {
        result.push(lines[i].to_string());
    }

    while let Some(last) = result.last() {
        if last.is_empty() {
            result.pop();
        } else {
            break;
        }
    }

    if !has_existing_entry {
        result.push(String::new());
    }
    result.push(entry.to_string());
    result.push(String::new());

    for i in idx..lines.len() {
        result.push(lines[i].to_string());
    }

    Some(result.join("\n"))
}

pub fn insert_worklog_entry(
    content: &str,
    heading: &str,
    entry: &str,
    use_anchor: bool,
    anchor_comment: &str,
) -> String {
    if use_anchor {
        if let Some(updated) = insert_entry_before_anchor(content, anchor_comment, entry) {
            return updated;
        }
    }
    insert_entry_into_content(content, heading, entry)
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
    let settings = resolve_daily_settings(vault_dir);
    let daily_note_path = ensure_daily_note_exists(vault_dir, &settings, &now)?;

    let timestamp = input
        .timestamp
        .clone()
        .unwrap_or_else(|| now.format("%H:%M").to_string());

    let entry = format_worklog_entry(&timestamp, &input.memo);

    let existing_content = fs::read_to_string(&daily_note_path)
        .map_err(|e| format!("Failed to read daily note '{:?}': {}", daily_note_path, e))?;

    let updated_content = insert_worklog_entry(
        &existing_content,
        &config.worklog_section_heading,
        &entry,
        config.worklog_use_anchor,
        &config.worklog_anchor_comment,
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
    fn test_format_moment_date_comprehensive() {
        use chrono::TimeZone;
        let dt = Local.with_ymd_and_hms(2026, 11, 8, 14, 5, 9).unwrap();

        // 1. Japanese weekday tests
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(ddd)", Some("ja")),
            "2026-11-08(日)"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(dddd)", Some("ja")),
            "2026-11-08(日曜日)"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(dd)", Some("ja")),
            "2026-11-08(日)"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(d)", Some("ja")),
            "2026-11-08(0)"
        );

        // 2. English weekday tests
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(ddd)", Some("en")),
            "2026-11-08(Sun)"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(dddd)", Some("en")),
            "2026-11-08(Sunday)"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-MM-DD(dd)", Some("en")),
            "2026-11-08(Su)"
        );

        // 3. Collision avoidance: ddd is not mangled by dd
        assert_ne!(
            format_moment_date(&dt, "YYYY-MM-DD(ddd)", Some("ja")),
            "2026-11-08(08d)"
        );

        // 4. Bracket escaping [...]
        assert_eq!(
            format_moment_date(&dt, "[Today is] YYYY-MM-DD [ddd]", Some("ja")),
            "Today is 2026-11-08 ddd"
        );
        assert_eq!(
            format_moment_date(&dt, "YYYY-[MM]-DD", Some("ja")),
            "2026-MM-08"
        );
        assert_eq!(
            format_moment_date(&dt, "[Unclosed bracket", Some("ja")),
            "Unclosed bracket"
        );

        // 5. Month and Year tokens
        assert_eq!(
            format_moment_date(&dt, "YYYY/YY/yyyy/yy", Some("en")),
            "2026/26/2026/26"
        );
        assert_eq!(
            format_moment_date(&dt, "MMMM D, YYYY", Some("en")),
            "November 8, 2026"
        );
        assert_eq!(
            format_moment_date(&dt, "MMM D, YYYY", Some("en")),
            "Nov 8, 2026"
        );
        assert_eq!(
            format_moment_date(&dt, "MMMM D日 YYYY年", Some("ja")),
            "11月 8日 2026年"
        );

        // 6. Time tokens and AM/PM
        assert_eq!(
            format_moment_date(&dt, "HH:mm:ss", Some("en")),
            "14:05:09"
        );
        assert_eq!(
            format_moment_date(&dt, "hh:m:s A", Some("en")),
            "02:5:9 PM"
        );
        assert_eq!(
            format_moment_date(&dt, "hh:mm A", Some("ja")),
            "02:05 午後"
        );

        // 7. Quarter
        assert_eq!(format_moment_date(&dt, "[Q]Q", Some("en")), "Q4");
    }

    #[test]
    fn test_format_worklog_entry() {
        let entry = format_worklog_entry("09:15", "レビュー依頼対応\n- PR #123 の確認");
        assert_eq!(entry, "### 09:15\nレビュー依頼対応\n- PR #123 の確認");

        // If timestamp already has ### prefix, don't duplicate
        let entry2 = format_worklog_entry("### 14:30", "打ち合わせ");
        assert_eq!(entry2, "### 14:30\n打ち合わせ");
    }

    #[test]
    fn test_insert_entry_into_existing_heading() {
        let content = r#"# 2026-11-08

## 作業ログ

### 10:00
朝会参加

## 振り返り
Good day!
"#;

        let entry = "### 11:30\nタスクAの実装";
        let updated = insert_entry_into_content(content, "## 作業ログ", entry);

        assert!(updated.contains("### 10:00\n朝会参加\n### 11:30\nタスクAの実装"));
        assert!(!updated.contains("朝会参加\n\n### 11:30"));
        assert!(updated.contains("## 振り返り"));

        let log_pos = updated.find("## 作業ログ").unwrap();
        let first_entry_pos = updated.find("### 10:00\n朝会参加").unwrap();
        let new_entry_pos = updated.find("### 11:30\nタスクAの実装").unwrap();
        let reflect_pos = updated.find("## 振り返り").unwrap();

        assert!(log_pos < first_entry_pos);
        assert!(first_entry_pos < new_entry_pos);
        assert!(new_entry_pos < reflect_pos);

        // Third entry append: should also be placed contiguously without blank lines
        let entry3 = "### 14:00\nコードレビュー";
        let updated2 = insert_entry_into_content(&updated, "## 作業ログ", entry3);
        assert!(updated2.contains("### 10:00\n朝会参加\n### 11:30\nタスクAの実装\n### 14:00\nコードレビュー"));
        assert!(!updated2.contains("タスクAの実装\n\n### 14:00"));
        let third_pos = updated2.find("### 14:00\nコードレビュー").unwrap();
        let reflect_pos2 = updated2.find("## 振り返り").unwrap();
        assert!(third_pos < reflect_pos2);
    }

    #[test]
    fn test_insert_entry_into_missing_heading() {
        let content = "# 2026-11-08\n\nDaily content.";
        let entry = "### 14:15\nレビュー対応";
        let updated = insert_entry_into_content(content, "## 作業ログ", entry);

        assert!(updated.contains("## 作業ログ"));
        assert!(updated.contains("### 14:15\nレビュー対応"));
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

        let resolved = resolve_daily_settings(&temp_dir);

        assert_eq!(resolved.format, "YYYY_MM_DD");
        assert_eq!(resolved.folder, "DailyNotes");
        assert!(resolved.template_path.is_some());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_resolve_daily_settings_fallback_defaults() {
        let temp_dir = env::temp_dir().join(format!("obs_fallback_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let resolved = resolve_daily_settings(&temp_dir);

        assert_eq!(resolved.format, "YYYY-MM-DD");
        assert_eq!(resolved.folder, "");
        assert_eq!(resolved.template_path, None);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_append_worklog_integration() {
        let temp_dir = env::temp_dir().join(format!("obs_worklog_integ_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        let mut app_cfg = AppConfig::default();
        app_cfg.vault_path = temp_dir.to_str().unwrap().to_string();
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
        assert!(content1.contains("### 09:30\nタスクの調査\n- 仕様書確認"));

        let input2 = WorklogInput {
            memo: "実装着手".to_string(),
            timestamp: Some("10:45".to_string()),
        };

        let res2 = append_worklog(&input2, &app_cfg).expect("Second append failed");
        assert!(res2.success);

        let content2 = fs::read_to_string(&res2.daily_note_path).unwrap();
        assert!(content2.contains("### 09:30\nタスクの調査\n- 仕様書確認\n### 10:45\n実装着手"));
        assert!(!content2.contains("仕様書確認\n\n### 10:45"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_insert_entry_before_anchor() {
        let content = r#"# 2026-11-08

## 今日の目標
- [ ] 目標1

## 作業ログ

%% worklog %%

## 振り返り
Good day.
"#;
        let entry1 = "### 10:00\n朝会参加";
        let updated1 = insert_entry_before_anchor(content, "%% worklog %%", entry1)
            .expect("Anchor insertion should succeed");

        assert!(updated1.contains("### 10:00\n朝会参加"));
        assert!(updated1.contains("%% worklog %%"));
        assert!(updated1.contains("## 振り返り"));

        let log_heading_pos = updated1.find("## 作業ログ").unwrap();
        let entry1_pos = updated1.find("### 10:00\n朝会参加").unwrap();
        let anchor_pos = updated1.find("%% worklog %%").unwrap();
        let reflect_pos = updated1.find("## 振り返り").unwrap();

        assert!(log_heading_pos < entry1_pos);
        assert!(entry1_pos < anchor_pos);
        assert!(anchor_pos < reflect_pos);

        // Append second entry before the same anchor
        let entry2 = "### 11:30\nタスクAの実装";
        let updated2 = insert_entry_before_anchor(&updated1, "%% worklog %%", entry2)
            .expect("Second anchor insertion should succeed");

        assert!(updated2.contains("### 10:00\n朝会参加\n### 11:30\nタスクAの実装"));
        assert!(!updated2.contains("朝会参加\n\n### 11:30"));
        assert!(updated2.contains("%% worklog %%"));

        let entry1_pos_2 = updated2.find("### 10:00\n朝会参加").unwrap();
        let entry2_pos_2 = updated2.find("### 11:30\nタスクAの実装").unwrap();
        let anchor_pos_2 = updated2.find("%% worklog %%").unwrap();

        assert!(entry1_pos_2 < entry2_pos_2);
        assert!(entry2_pos_2 < anchor_pos_2);
    }

    #[test]
    fn test_insert_worklog_entry_fallback_when_anchor_missing() {
        let content = r#"# 2026-11-08

## 作業ログ

### 10:00
朝会参加

## 振り返り
"#;
        let entry = "### 14:00\n設計レビュー";
        // use_anchor is true, but content lacks "%% worklog %%"
        let updated = insert_worklog_entry(
            content,
            "## 作業ログ",
            entry,
            true,
            "%% worklog %%",
        );

        assert!(updated.contains("### 14:00\n設計レビュー"));
        let heading_pos = updated.find("## 作業ログ").unwrap();
        let entry_pos = updated.find("### 14:00\n設計レビュー").unwrap();
        let reflect_pos = updated.find("## 振り返り").unwrap();

        assert!(heading_pos < entry_pos);
        assert!(entry_pos < reflect_pos);
    }

    #[test]
    fn test_insert_worklog_entry_ignores_anchor_when_disabled() {
        let content = r#"# 2026-11-08

## 作業ログ

%% worklog %%

## 振り返り
"#;
        let entry = "### 09:45\nメール確認";
        // use_anchor is false even though anchor exists
        let updated = insert_worklog_entry(
            content,
            "## 作業ログ",
            entry,
            false,
            "%% worklog %%",
        );

        assert!(updated.contains("### 09:45\nメール確認"));
        let heading_pos = updated.find("## 作業ログ").unwrap();
        let entry_pos = updated.find("### 09:45\nメール確認").unwrap();
        let reflect_pos = updated.find("## 振り返り").unwrap();

        // Under heading mode, it should be between heading and next heading
        assert!(heading_pos < entry_pos);
        assert!(entry_pos < reflect_pos);
    }

    #[test]
    fn test_append_worklog_anchor_integration() {
        let temp_dir = env::temp_dir().join(format!("obs_worklog_anchor_integ_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).unwrap();

        // Create template with %% worklog %%
        let tpl_dir = temp_dir.join("Templates");
        fs::create_dir_all(&tpl_dir).unwrap();
        let tpl_content = "# {{title}}\n\n## 作業ログ\n\n%% worklog %%\n\n## 振り返り\n";
        fs::write(tpl_dir.join("Daily.md"), tpl_content).unwrap();

        // Create .obsidian/daily-notes.json
        let obs_dir = temp_dir.join(".obsidian");
        fs::create_dir_all(&obs_dir).unwrap();
        let daily_cfg = serde_json::json!({
            "format": "YYYY-MM-DD",
            "folder": "Daily",
            "template": "Templates/Daily"
        });
        fs::write(obs_dir.join("daily-notes.json"), daily_cfg.to_string()).unwrap();

        let mut app_cfg = AppConfig::default();
        app_cfg.vault_path = temp_dir.to_str().unwrap().to_string();
        app_cfg.worklog_section_heading = "## 作業ログ".to_string();
        app_cfg.worklog_use_anchor = true;
        app_cfg.worklog_anchor_comment = "%% worklog %%".to_string();

        let input1 = WorklogInput {
            memo: "朝のタスク確認".to_string(),
            timestamp: Some("09:00".to_string()),
        };
        let res1 = append_worklog(&input1, &app_cfg).expect("First append failed");
        assert!(res1.success);

        let content1 = fs::read_to_string(&res1.daily_note_path).unwrap();
        let pos_entry1 = content1.find("### 09:00\n朝のタスク確認").unwrap();
        let pos_anchor1 = content1.find("%% worklog %%").unwrap();
        let pos_reflect1 = content1.find("## 振り返り").unwrap();

        assert!(pos_entry1 < pos_anchor1);
        assert!(pos_anchor1 < pos_reflect1);

        let input2 = WorklogInput {
            memo: "機能実装完了".to_string(),
            timestamp: Some("17:30".to_string()),
        };
        let res2 = append_worklog(&input2, &app_cfg).expect("Second append failed");
        assert!(res2.success);

        let content2 = fs::read_to_string(&res2.daily_note_path).unwrap();
        assert!(content2.contains("### 09:00\n朝のタスク確認\n### 17:30\n機能実装完了"));
        assert!(!content2.contains("朝のタスク確認\n\n### 17:30"));
        let pos_entry1_2 = content2.find("### 09:00\n朝のタスク確認").unwrap();
        let pos_entry2_2 = content2.find("### 17:30\n機能実装完了").unwrap();
        let pos_anchor_2 = content2.find("%% worklog %%").unwrap();

        assert!(pos_entry1_2 < pos_entry2_2);
        assert!(pos_entry2_2 < pos_anchor_2);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_read_obsidian_app_locale() {
        let temp_dir = env::temp_dir().join(format!("obs_locale_test_{}", std::process::id()));
        let obs_dir = temp_dir.join(".obsidian");
        fs::create_dir_all(&obs_dir).unwrap();

        // 1. When userLocale is set
        fs::write(obs_dir.join("app.json"), r#"{"userLocale": "ja"}"#).unwrap();
        assert_eq!(read_obsidian_app_locale(&temp_dir), Some("ja".to_string()));

        // 2. When language is set instead
        fs::write(obs_dir.join("app.json"), r#"{"language": "ja-JP"}"#).unwrap();
        assert_eq!(
            read_obsidian_app_locale(&temp_dir),
            Some("ja-JP".to_string())
        );

        // 3. When app.json does not exist
        let _ = fs::remove_file(obs_dir.join("app.json"));
        assert_eq!(read_obsidian_app_locale(&temp_dir), None);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ensure_daily_note_exists_with_moment_locale() {
        use chrono::TimeZone;
        let temp_dir = env::temp_dir().join(format!("obs_ensure_moment_{}", std::process::id()));
        let obs_dir = temp_dir.join(".obsidian");
        fs::create_dir_all(&obs_dir).unwrap();

        // Configure daily notes format with weekday token: YYYY-MM-DD(ddd)
        let daily_cfg = serde_json::json!({
            "format": "YYYY-MM-DD(ddd)",
            "folder": "Daily"
        });
        fs::write(obs_dir.join("daily-notes.json"), daily_cfg.to_string()).unwrap();

        // Configure app.json with Japanese locale
        fs::write(obs_dir.join("app.json"), r#"{"userLocale": "ja"}"#).unwrap();

        let resolved = resolve_daily_settings(&temp_dir);
        assert_eq!(resolved.locale, Some("ja".to_string()));
        assert_eq!(resolved.format, "YYYY-MM-DD(ddd)");

        // 2026-11-08 was Sunday (日)
        let dt = Local.with_ymd_and_hms(2026, 11, 8, 10, 0, 0).unwrap();
        let path = ensure_daily_note_exists(&temp_dir, &resolved, &dt)
            .expect("Daily note creation should succeed");

        assert!(path.exists());
        assert_eq!(
            path.file_name().unwrap().to_str().unwrap(),
            "2026-11-08(日).md"
        );

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("# 2026-11-08(日)"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_ensure_daily_note_exists_prevents_duplicate_creation() {
        use chrono::TimeZone;
        let temp_dir = env::temp_dir().join(format!("obs_duplicate_prevent_{}", std::process::id()));
        let obs_dir = temp_dir.join(".obsidian");
        let daily_dir = temp_dir.join("Daily");
        fs::create_dir_all(&obs_dir).unwrap();
        fs::create_dir_all(&daily_dir).unwrap();

        // Configure daily notes with format YYYY-MM-DD(ddd)
        let daily_cfg = serde_json::json!({
            "format": "YYYY-MM-DD(ddd)",
            "folder": "Daily"
        });
        fs::write(obs_dir.join("daily-notes.json"), daily_cfg.to_string()).unwrap();
        fs::write(obs_dir.join("app.json"), r#"{"userLocale": "ja"}"#).unwrap();

        // Pre-create the daily note with custom content
        let existing_file = daily_dir.join("2026-11-08(日).md");
        let initial_content = "# 2026-11-08(日)\n\n## 既存セクション\n既存の内容を保持\n\n## 作業ログ\n";
        fs::write(&existing_file, initial_content).unwrap();

        let resolved = resolve_daily_settings(&temp_dir);
        let dt = Local.with_ymd_and_hms(2026, 11, 8, 14, 0, 0).unwrap();

        // Call ensure_daily_note_exists
        let path = ensure_daily_note_exists(&temp_dir, &resolved, &dt)
            .expect("Should resolve existing note");

        assert_eq!(path, existing_file);

        // Verify the existing file was NOT overwritten or reset
        let content_after = fs::read_to_string(&path).unwrap();
        assert_eq!(content_after, initial_content);

        // Verify no other files were created in the Daily folder
        let files: Vec<_> = fs::read_dir(&daily_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
        assert_eq!(files.len(), 1);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
