# Spec Delta

## Purpose

Enables instantaneous capture of timestamped work logs into Obsidian daily notes, adhering to Obsidian Daily Notes core plugin settings and configurable section headings.

## ADDED Requirements

### Requirement: Simple work log text entry
The system SHALL provide a multi-line text input field in Worklog mode that accepts freeform text containing multiple lines and blank lines.

#### Scenario: Inputting multiline log entry
- **WHEN** the launcher is in Worklog mode and the user inputs multiple lines of text including paragraph breaks
- **THEN** the input control SHALL preserve all line breaks and indentation without truncation

### Requirement: Daily note initialization from template
The system SHALL ensure the target daily note for the current date exists before appending log entries, initializing it using the template, folder, and date format specified in the Vault's Obsidian core plugin settings (.obsidian/daily-notes.json).

#### Scenario: Initializing note using Obsidian daily notes settings
- **WHEN** today's daily note does not exist and `.obsidian/daily-notes.json` specifies folder, format, and template
- **THEN** the system SHALL create the daily note at `[Vault]/[folder]/[formatted_date].md` initialized with content loaded from the template note in the Vault

#### Scenario: Fallback when Obsidian daily notes config is absent
- **WHEN** today's daily note does not exist and `.obsidian/daily-notes.json` is missing or does not specify a template
- **THEN** the system SHALL create the daily note using application default naming and template rules

#### Scenario: Daily note already exists
- **WHEN** the daily note for today already exists in the Vault
- **THEN** the system SHALL read the existing note and preserve all existing sections and contents

### Requirement: Chronological insertion under work log section
The system SHALL append new work log entries under the configured section heading (default: ## 作業ログ) in chronological order without modifying other sections.

#### Scenario: Appending new log entry to configured section
- **WHEN** the user submits a work log entry at local time 14:30 and configuration specifies section heading `## 作業ログ`
- **THEN** the system SHALL locate `## 作業ログ` and append the entry before the next heading or at the end of the section

#### Scenario: Handling custom configured section heading
- **WHEN** configuration specifies a custom section heading (e.g. `## 業務日報`)
- **THEN** the system SHALL locate `## 業務日報` in the daily note and append the entry under that heading

#### Scenario: Handling missing section heading
- **WHEN** an existing daily note lacks the configured section heading
- **THEN** the system SHALL safely create the configured section heading in the file and append the entry under it

### Requirement: Machine-readable WebVTT-like log formatting
The system SHALL format each work log entry with a two-digit hour and minute timestamp (`hh:mm`) followed immediately by the memo text and a blank line separator.

#### Scenario: Formatting submitted log text
- **WHEN** the user submits a memo at 09:15 with content "レビュー依頼対応\n- PR #123 の確認"
- **THEN** the appended block in Markdown SHALL match the pattern:
  ```
  09:15
  レビュー依頼対応
  - PR #123 の確認
  ```
  followed by an empty line delimiter

### Requirement: Optional explicit anchor comment insertion
The system SHALL support an optional explicit anchor comment insertion mode that appends work log entries directly before a designated anchor comment in the daily note.

#### Scenario: Appending with anchor mode enabled and anchor present
- **WHEN** configuration has `worklog_use_anchor: true` and the daily note contains the configured anchor line (default: `%% worklog %%`)
- **THEN** the system SHALL insert the worklog entry immediately before the anchor line, preserving the anchor comment for future entries

#### Scenario: Fallback when anchor comment is missing
- **WHEN** configuration has `worklog_use_anchor: true` but the daily note lacks the designated anchor comment
- **THEN** the system SHALL safely fall back to the default heading-based insertion logic under `worklog_section_heading`

#### Scenario: Default behavior when anchor mode is disabled
- **WHEN** configuration has `worklog_use_anchor: false` (default)
- **THEN** the system SHALL append worklog entries under `worklog_section_heading` using the standard heading scanning logic, ignoring any anchor comments

