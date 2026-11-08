# Spec Delta

## Purpose

Provides system color scheme detection and theme synchronization across the launcher UI, ensuring seamless support for light and dark modes according to OS settings.

## ADDED Requirements

### Requirement: System color scheme synchronization
The system SHALL detect the operating system's preferred color scheme (`prefers-color-scheme`) and apply the corresponding theme to the application root.

#### Scenario: Operating system is in light mode
- **WHEN** the OS color scheme preference is set to light or `prefers-color-scheme: light` matches
- **THEN** the system SHALL apply light theme styling by removing the `.dark` class from the root document element and rendering components with light semantic tokens

#### Scenario: Operating system is in dark mode
- **WHEN** the OS color scheme preference is set to dark or `prefers-color-scheme: dark` matches
- **THEN** the system SHALL apply dark theme styling by attaching the `.dark` class to the root document element

#### Scenario: Operating system color scheme changes at runtime
- **WHEN** the user changes their OS theme preference while the application is running
- **THEN** the application SHALL automatically detect the change and update the root theme class without requiring a restart or manual reload

### Requirement: Semantic theme token mapping
The system SHALL expose shadcn-compatible CSS variables for backgrounds, foregrounds, borders, inputs, and accents across themes.

#### Scenario: Light theme token rendering
- **WHEN** the light theme is active
- **THEN** all UI elements SHALL resolve colors through CSS variables ensuring proper contrast, readable text, and defined borders without relying on hardcoded dark utility classes
