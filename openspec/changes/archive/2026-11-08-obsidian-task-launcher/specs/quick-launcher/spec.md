# Spec Delta

## Purpose

Provides a system-wide global hotkey launcher and a compact modal window allowing users to quickly capture notes without leaving their current application.

## ADDED Requirements

### Requirement: Global hotkey activation
The system SHALL register an OS-level global hotkey and bring the launcher window to the front with focus when triggered from any active application.

#### Scenario: Pressing configured global hotkey
- **WHEN** the user presses the configured global shortcut (e.g. Win+Shift+Q) while another application is active
- **THEN** the launcher window SHALL immediately appear in the foreground centered or anchored on screen and automatically focus the primary input field of the active mode (memo textarea in Worklog mode or title input in Task mode), accepting keyboard input immediately

#### Scenario: Pressing hotkey when launcher is already visible
- **WHEN** the user presses the global shortcut while the launcher window is currently active
- **THEN** the launcher window SHALL close or hide without saving uncommitted changes

### Requirement: Dismiss launcher on demand
The system SHALL allow the user to immediately dismiss the launcher window using a standard keyboard shortcut or when focus is lost.

#### Scenario: Pressing Escape key
- **WHEN** the user presses the Escape key inside the launcher window
- **THEN** the launcher window SHALL close and focus SHALL return to the previously active application

#### Scenario: Clicking outside launcher window
- **WHEN** the launcher loses focus because the user clicks outside its boundary
- **THEN** the launcher window SHALL automatically hide or close if configured to auto-dismiss on blur

### Requirement: Mode switching between Task and Worklog
The system SHALL provide a fast keyboard-friendly mechanism to switch between Task creation mode and Worklog entry mode within the launcher window.

#### Scenario: Switching modes via shortcut
- **WHEN** the user presses the mode toggle shortcut (such as Tab on empty input or Ctrl+Tab / Alt+1 / Alt+2)
- **THEN** the launcher view SHALL immediately switch between the Task creation form and the Worklog input form, preserve existing inputs in each mode, and automatically focus the primary input field of the target mode

#### Scenario: Default initial mode
- **WHEN** the launcher window is opened
- **THEN** it SHALL display the Worklog input mode by default with the memo textarea immediately focused

#### Scenario: Restoring focus on window activation and modal dismiss
- **WHEN** the launcher window is reopened from a hidden state or the settings modal is dismissed
- **THEN** the primary input field of the current active mode SHALL automatically regain focus

