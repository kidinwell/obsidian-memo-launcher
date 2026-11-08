# Spec Delta

## Purpose

Enables rapid capture of structured tasks into individual Obsidian markdown notes with configurable frontmatter metadata and markdown body text.

## ADDED Requirements

### Requirement: Structured task input fields
The system SHALL provide input fields for task title, requester, deadline, and task body in the Task creation form.

#### Scenario: Displaying task fields
- **WHEN** the launcher is in Task creation mode
- **THEN** the form SHALL present input controls for Title, Requester, Deadline, and Task Body with keyboard navigation between them

### Requirement: Task note generation in Obsidian Vault
The system SHALL create a new Markdown note inside the configured task folder of the specified Obsidian Vault upon submission.

#### Scenario: Successful task creation
- **WHEN** the user enters a title "新規機能の設計", requester "田中", deadline "2026-11-15", and body content, then submits via submission shortcut (e.g., Ctrl+Enter)
- **THEN** the system SHALL create `[Vault]/[TaskFolder]/新規機能の設計.md` containing frontmatter properties and the body content, and dismiss the launcher window

#### Scenario: Handling existing file with identical title
- **WHEN** the user submits a task whose title already exists as a note in the task folder
- **THEN** the system SHALL prevent accidental overwriting by appending a unique timestamp or increment suffix, or notifying the user before saving

#### Scenario: Filename sanitization
- **WHEN** the user enters a title containing characters disallowed in filenames (e.g., `\ / : * ? " < > |`)
- **THEN** the system SHALL sanitize invalid characters or alert the user so that a valid Markdown file can be safely created

### Requirement: Extensible frontmatter properties
The system SHALL format task notes with standard YAML frontmatter containing requester, deadline, completion status/date, and any additional configured properties.

#### Scenario: Default frontmatter fields
- **WHEN** a task note is generated
- **THEN** the frontmatter SHALL include `requester: <value>`, `due: <value>`, and an empty or unset `completed: null` property

#### Scenario: Custom property extension
- **WHEN** the user has configured additional properties (such as estimate or priority) in settings
- **THEN** the task note frontmatter SHALL include the configured keys with their corresponding input values

