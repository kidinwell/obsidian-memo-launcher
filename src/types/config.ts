export interface CustomProperty {
  key: string;
  label: string;
  type: string;
  default?: string | number | boolean | null;
}

export interface AppConfig {
  vault_path: string;
  tasks_folder: string;
  default_mode: "worklog" | "task";
  worklog_section_heading: string;
  daily_notes_folder: string;
  daily_note_format: string;
  daily_note_template_path: string;
  global_shortcut: string;
  auto_hide_on_blur: boolean;
  task_custom_properties: CustomProperty[];
}

export interface VaultValidationResult {
  is_valid: boolean;
  has_obsidian_folder: boolean;
  message?: string | null;
}
