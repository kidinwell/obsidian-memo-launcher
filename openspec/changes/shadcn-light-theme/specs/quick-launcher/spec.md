# Spec Delta

## ADDED Requirements

### Requirement: shadcn UI component rendering and theme compliance
The system SHALL render launcher controls, forms, and dialogs using shadcn-based UI components that automatically adhere to the active color theme.

#### Scenario: Launcher view in light theme
- **WHEN** the launcher window is displayed while the light theme is active
- **THEN** the window container, header tabs, input fields, textareas, and submit buttons SHALL render using semantic tokens with light backgrounds, readable foreground text, and subtle borders

#### Scenario: Settings modal appearance in light theme
- **WHEN** the user opens the settings modal while the light theme is active
- **THEN** the modal backdrop, dialog container, section headings, input boxes, and buttons SHALL render with light theme surface styles and clear visual hierarchy

#### Scenario: Toast notification appearance in light theme
- **WHEN** a toast notification is shown while the light theme is active
- **THEN** the notification badge SHALL display high-contrast text and recognizable status borders against the light background surface
