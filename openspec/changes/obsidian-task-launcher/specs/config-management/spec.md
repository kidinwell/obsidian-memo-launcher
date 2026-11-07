# Spec Delta

## Purpose

Provides configuration persistence and validation for Obsidian vault paths, Obsidian daily notes integration, folder locations, section headings, global hotkey bindings, and extensible task property schemas.

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
- **THEN** the system SHALL fall back to application-level daily note settings without throwing an error

### Requirement: Task and Daily Note folder mapping
The system SHALL support configuring custom subfolder paths within the Vault for storing task notes and daily notes.

#### Scenario: Custom subfolder paths configured
- **WHEN** the user sets `task_folder: "Tasks"` and `daily_notes_folder: "Daily"` in configuration
- **THEN** task notes SHALL be created in `[Vault]/Tasks/` and daily notes in `[Vault]/Daily/`

#### Scenario: Default folder fallback
- **WHEN** no subfolder is configured for tasks or daily notes
- **THEN** the system SHALL safely place the files directly in the Vault root or designated default folders

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
