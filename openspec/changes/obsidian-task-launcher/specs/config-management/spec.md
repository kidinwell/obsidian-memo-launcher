# Spec Delta

## Purpose

Provides configuration persistence and validation for Obsidian vault paths, task folder locations, section headings, global hotkey bindings, and extensible task property schemas (daily note folder and format settings are delegated entirely to Obsidian's built-in Daily Notes core plugin and excluded from launcher configuration).

## ADDED Requirements

### Requirement: Obsidian Vault path configuration
The system SHALL require and validate a path pointing to an accessible Obsidian Vault directory on the local file system.

#### Scenario: Valid vault directory
- **WHEN** the user configures an existing directory containing `.obsidian` as the Vault path
- **THEN** the system SHALL accept the path and persist it in application configuration

#### Scenario: Missing or inaccessible vault directory
- **WHEN** the launcher attempts to read or write to a Vault path that does not exist or lacks write permissions
- **THEN** the system SHALL display a clear error message prompting the user to update the Vault location

### Requirement: Obsidian core Daily Notes integration
The system SHALL automatically inspect and synchronize settings from the Vault's .obsidian/daily-notes.json configuration file.

#### Scenario: Automatic detection of daily notes settings
- **WHEN** a valid Vault path is configured and `.obsidian/daily-notes.json` exists
- **THEN** the system SHALL parse `format`, `folder`, and `template` from the file to resolve daily note creation behavior

#### Scenario: Missing daily notes config file
- **WHEN** `.obsidian/daily-notes.json` does not exist in the configured Vault
- **THEN** the system SHALL fall back to Obsidian core Daily Notes default values (date format `YYYY-MM-DD`, Vault root folder, no template) without throwing an error

### Requirement: Task folder mapping
The system SHALL support configuring a custom subfolder path within the Vault for storing task notes. Daily note locations SHALL be resolved exclusively via Obsidian's core Daily Notes plugin.

#### Scenario: Custom task subfolder path configured
- **WHEN** the user sets `task_folder: "Tasks"` in configuration
- **THEN** task notes SHALL be created in `[Vault]/Tasks/`

#### Scenario: Default task folder fallback
- **WHEN** no subfolder is configured for tasks
- **THEN** the system SHALL place task notes in `[Vault]/Tasks/` by default, or in the Vault root if empty

### Requirement: Configurable worklog section heading
The system SHALL allow users to customize the target section heading string used for locating and appending work logs in daily notes.

#### Scenario: Customizing section heading
- **WHEN** the user sets `worklog_section_heading: "## 業務ログ"` in configuration
- **THEN** the system SHALL save the setting and use `## 業務ログ` as the target section heading for daily note insertions

#### Scenario: Default section heading
- **WHEN** no custom section heading is specified in configuration
- **THEN** the system SHALL use `## 作業ログ` as the default target section heading

### Requirement: Customizable global hotkey binding
The system SHALL allow users to customize the global shortcut key combination used to activate the launcher.

#### Scenario: Registering custom hotkey
- **WHEN** the user changes the hotkey in configuration to a valid modifier and key combination (such as `Win+Shift+Q` or `Ctrl+Shift+Space`)
- **THEN** the system SHALL re-register the global hotkey with the OS without requiring an application restart

#### Scenario: Hotkey registration conflict
- **WHEN** the requested hotkey combination is already occupied by another application or the OS
- **THEN** the system SHALL alert the user that registration failed and keep the previous valid hotkey active

### Requirement: Extensible task property schema configuration
The system SHALL allow users to define additional frontmatter properties to be rendered in the task creation form and inserted into task notes.

#### Scenario: Adding custom task property definition
- **WHEN** configuration defines an extra property with key `estimate`, label `見積もり (h)`, and type `number`
- **THEN** the Task creation form SHALL render an input field for `estimate` and include `estimate: <value>` in generated task note frontmatter

### Requirement: Worklog anchor comment configuration
The system SHALL allow users to enable explicit anchor comment insertion and specify a custom anchor comment string.

#### Scenario: Default anchor settings
- **WHEN** application configuration is loaded or initialized
- **THEN** `worklog_use_anchor` SHALL default to `false` and `worklog_anchor_comment` SHALL default to `%% worklog %%`

#### Scenario: Customizing anchor settings
- **WHEN** the user sets `worklog_use_anchor: true` and `worklog_anchor_comment: "<!-- worklog -->"`
- **THEN** the system SHALL persist the settings and use them when resolving worklog insertions

