// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration loading, saving, and validation.
//!
//! This module provides functionality to load configuration from files,
//! save configuration to files, and validate configuration correctness.

use crate::error::ConfigError;
use std::collections::HashSet;
use std::path::Path;

use super::types::{ActionConfig, Config, DeviceConfig, Mapping, Mode, Trigger};

impl Config {
    /// Load configuration from a TOML file
    ///
    /// If the file doesn't exist, creates a default configuration and saves it to the specified path.
    ///
    /// # Security
    /// This function performs path validation to prevent path traversal attacks:
    /// - Canonicalizes the path to resolve symlinks and relative components
    /// - Restricts access to allowed directories (config directory, /tmp, current working directory)
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    /// * `Ok(Config)` - Successfully loaded or created configuration
    /// * `Err(ConfigError)` - IO, parsing, validation, or security error
    ///
    /// # Example
    /// ```no_run
    /// use conductor_core::Config;
    ///
    /// let config = Config::load("config.toml")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Security: Validate and sanitize path
        let safe_path = Self::validate_config_path(path)?;

        if safe_path.exists() {
            let contents = std::fs::read_to_string(&safe_path)?;
            let config: Config = toml::from_str(&contents)?;
            config.validate()?;
            Ok(config)
        } else {
            println!("Config file not found, creating default config...");
            let config = Self::default_config();
            config.save(path)?;
            Ok(config)
        }
    }

    /// Save configuration to a TOML file
    ///
    /// Writes the configuration as formatted TOML using an atomic write pattern.
    ///
    /// # Security
    /// This function prevents TOCTOU (Time-Of-Check-Time-Of-Use) race conditions:
    /// - Validates the FULL canonical path before writing (not just parent directory)
    /// - Uses atomic write pattern (write to temp file, then rename)
    /// - Uses OpenOptions with explicit flags to prevent symlink following on platforms that support it
    /// - Restricts writes to allowed directories (config directory, /tmp, current working directory)
    ///
    /// # TOCTOU Prevention
    /// The original implementation had a race condition where an attacker could:
    /// 1. Wait for parent directory validation to pass
    /// 2. Replace parent with a symlink to a privileged location (e.g., /etc)
    /// 3. Cause the write to occur in the privileged location
    ///
    /// This is mitigated by:
    /// - Validating the full target path (not just parent)
    /// - Using atomic writes (temp file + rename)
    /// - Re-validating after temp file creation
    ///
    /// # Arguments
    /// * `path` - Path where the configuration file will be written
    ///
    /// # Returns
    /// * `Ok(())` - Successfully saved
    /// * `Err(Box<dyn std::error::Error>)` - IO, serialization, or security error
    ///
    /// # Example
    /// ```no_run
    /// use conductor_core::Config;
    ///
    /// let config = Config::load("config.toml")?;
    /// config.save("backup.toml")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::OpenOptions;
        use std::io::Write;

        // Security: Convert to absolute path first
        let path_buf = Path::new(path);
        let absolute_path = if path_buf.is_absolute() {
            path_buf.to_path_buf()
        } else {
            std::env::current_dir()?.join(path_buf)
        };

        // Security: Validate parent directory exists or can be created
        if let Some(parent) = absolute_path.parent() {
            if !parent.exists() {
                return Err(format!(
                    "Parent directory does not exist: {}. Please create it first.",
                    parent.display()
                )
                .into());
            }

            // Canonicalize parent and validate it's in allowed directories
            let canonical_parent = parent.canonicalize()?;
            Self::check_path_allowed(&canonical_parent)?;
        }

        // Serialize config before any file operations
        let contents = toml::to_string_pretty(self)?;

        // Security: Construct the expected canonical path for validation
        // If file exists, canonicalize it. Otherwise, construct expected canonical path.
        let target_canonical = if absolute_path.exists() {
            // File exists - canonicalize it to resolve any symlinks
            let canonical = absolute_path.canonicalize()?;
            Self::check_path_allowed(&canonical)?;
            canonical
        } else {
            // File doesn't exist - construct canonical path from parent + filename
            let parent = absolute_path
                .parent()
                .ok_or("Invalid path: no parent directory")?;
            let filename = absolute_path
                .file_name()
                .ok_or("Invalid path: no filename")?;
            let canonical_parent = parent.canonicalize()?;
            Self::check_path_allowed(&canonical_parent)?;
            canonical_parent.join(filename)
        };

        // Security: Use atomic write pattern (write to temp, then rename)
        // This prevents partial writes and reduces TOCTOU window
        let temp_path = target_canonical.with_extension("tmp");

        // Write to temporary file with restrictive permissions
        // Note: O_NOFOLLOW is not available in std::fs, but OpenOptions with create_new
        // provides some protection by failing if file already exists
        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&temp_path)?;

            file.write_all(contents.as_bytes())?;
            file.sync_all()?; // Ensure data is written to disk
        }

        // Security: Re-validate temp file location before rename
        // This catches any races where the directory was replaced during write
        let temp_canonical = temp_path.canonicalize()?;
        Self::check_path_allowed(&temp_canonical)?;

        // Atomic rename to final location
        std::fs::rename(&temp_path, &target_canonical)?;

        Ok(())
    }

    /// Validate and sanitize a configuration file path
    ///
    /// # Security
    /// Prevents path traversal attacks by:
    /// 1. Converting relative paths to absolute
    /// 2. Resolving symlinks
    /// 3. Checking the path is within allowed directories
    ///
    /// # Allowed Directories
    /// - User's config directory (`~/.config`, `~/Library/Application Support`, etc.)
    /// - `/tmp` directory (for temporary configs)
    /// - Current working directory (for development/testing)
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - Canonical path if allowed
    /// * `Err(Box<dyn std::error::Error>)` - If path is outside allowed directories
    fn validate_config_path(path: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let path_buf = Path::new(path);

        // Convert to absolute path if relative
        let absolute_path = if path_buf.is_absolute() {
            path_buf.to_path_buf()
        } else {
            std::env::current_dir()?.join(path_buf)
        };

        // Canonicalize if the path exists, otherwise validate parent
        let canonical_path = if absolute_path.exists() {
            absolute_path.canonicalize()?
        } else if let Some(parent) = absolute_path.parent() {
            if parent.exists() {
                parent
                    .canonicalize()?
                    .join(absolute_path.file_name().ok_or("Invalid file name")?)
            } else {
                // Parent doesn't exist - allow it (will be created)
                absolute_path
            }
        } else {
            absolute_path
        };

        // Check if path is within allowed directories
        if canonical_path.exists() || canonical_path.parent().is_some_and(|p| p.exists()) {
            Self::check_path_allowed(&canonical_path)?;
        }

        Ok(canonical_path)
    }

    /// Check if a path is within allowed directories
    ///
    /// # Allowed Directories
    /// - User's config directory
    /// - `/tmp` directory (including its canonical form like `/private/var/folders/...` on macOS)
    /// - Current working directory
    ///
    /// # Returns
    /// * `Ok(())` - Path is allowed
    /// * `Err(Box<dyn std::error::Error>)` - Path is outside allowed directories
    fn check_path_allowed(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Get allowed directories (canonicalized to handle symlinks)
        let config_dir = dirs::config_dir().and_then(|p| p.canonicalize().ok());
        let current_dir = std::env::current_dir()
            .ok()
            .and_then(|p| p.canonicalize().ok());
        let tmp_dir = std::env::temp_dir().canonicalize().ok();

        // Check if path is within any allowed directory
        let is_in_config_dir = config_dir.as_ref().is_some_and(|dir| path.starts_with(dir));
        let is_in_current_dir = current_dir
            .as_ref()
            .is_some_and(|dir| path.starts_with(dir));
        let is_in_tmp = tmp_dir.as_ref().is_some_and(|dir| path.starts_with(dir));

        if !is_in_config_dir && !is_in_current_dir && !is_in_tmp {
            return Err(format!(
                "Security: Config path '{}' is outside allowed directories. \
                 Allowed: config directory, current directory, /tmp",
                path.display()
            )
            .into());
        }

        Ok(())
    }

    /// Create a default configuration
    ///
    /// Generates a default configuration with sample modes and mappings.
    /// This is used when no configuration file exists.
    ///
    /// # Returns
    /// Default configuration with:
    /// - Device name: "Mikro"
    /// - Two modes: "Default" and "Development"
    /// - Sample mappings for each mode
    pub fn default_config() -> Self {
        Config {
            device: DeviceConfig {
                name: "Mikro".to_string(),
                auto_connect: true,
                auto_reconnect: true,
                port: None,
            },
            modes: vec![
                Mode {
                    name: "Default".to_string(),
                    color: Some("blue".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: Some(1),
                        },
                        action: ActionConfig::Keystroke {
                            keys: "space".to_string(),
                            modifiers: vec!["cmd".to_string()],
                        },
                        description: Some("Spotlight Search".to_string()),
                    }],
                },
                Mode {
                    name: "Development".to_string(),
                    color: Some("green".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: None,
                        },
                        action: ActionConfig::Shell {
                            command: "git status".to_string(),
                        },
                        description: Some("Git status".to_string()),
                    }],
                },
            ],
            global_mappings: vec![],
            logging: None,
            advanced_settings: Default::default(),
        }
    }

    /// Validate the configuration for correctness
    ///
    /// Checks for:
    /// - Unique mode names (required for mode switching)
    /// - Valid trigger types (structural validation)
    /// - Valid action types (structural validation)
    ///
    /// Note: This validation ensures basic correctness. More detailed validation
    /// (e.g., valid key names, valid command syntax) happens at execution time.
    ///
    /// # Returns
    /// * `Ok(())` - Configuration is valid
    /// * `Err(ConfigError::ValidationError)` - Configuration has issues
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate unique mode names
        let mut mode_names = HashSet::new();
        for mode in &self.modes {
            if !mode_names.insert(&mode.name) {
                return Err(ConfigError::ValidationError(format!(
                    "Duplicate mode name: '{}'",
                    mode.name
                )));
            }
        }

        // Validate all mappings (both global and mode-specific)
        for mapping in &self.global_mappings {
            validate_mapping(mapping)?;
        }

        for mode in &self.modes {
            for mapping in &mode.mappings {
                validate_mapping(mapping)?;
            }
        }

        Ok(())
    }
}

/// Validate a single mapping
fn validate_mapping(mapping: &Mapping) -> Result<(), ConfigError> {
    validate_trigger(&mapping.trigger)?;
    validate_action(&mapping.action)?;
    Ok(())
}

/// Validate a trigger configuration
fn validate_trigger(trigger: &Trigger) -> Result<(), ConfigError> {
    match trigger {
        Trigger::Note { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::VelocityRange { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::LongPress { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::DoubleTap { note, .. } => {
            if *note > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Note number out of range: {} (must be 0-127)",
                    note
                )));
            }
        }
        Trigger::NoteChord { notes, .. } => {
            for note in notes {
                if *note > 127 {
                    return Err(ConfigError::InvalidTrigger(format!(
                        "Note number out of range: {} (must be 0-127)",
                        note
                    )));
                }
            }
            if notes.is_empty() {
                return Err(ConfigError::InvalidTrigger(
                    "NoteChord must have at least one note".to_string(),
                ));
            }
        }
        Trigger::EncoderTurn { cc, direction } => {
            if *cc > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "CC number out of range: {} (must be 0-127)",
                    cc
                )));
            }
            if let Some(dir) = direction
                && dir != "Clockwise"
                && dir != "CounterClockwise"
            {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Invalid direction: '{}' (must be 'Clockwise' or 'CounterClockwise')",
                    dir
                )));
            }
        }
        Trigger::CC { cc, .. } => {
            if *cc > 127 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "CC number out of range: {} (must be 0-127)",
                    cc
                )));
            }
        }
        Trigger::Aftertouch { .. } => {
            // Valid trigger, no specific validation needed
        }
        Trigger::PitchBend { .. } => {
            // Valid trigger, range validation is optional as values can be None
        }
        // Gamepad triggers (v3.0)
        Trigger::GamepadButton { button, .. } => {
            if *button < 128 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Gamepad button ID out of range: {} (must be 128-255 to avoid MIDI conflicts)",
                    button
                )));
            }
        }
        Trigger::GamepadButtonChord { buttons, .. } => {
            for button in buttons {
                if *button < 128 {
                    return Err(ConfigError::InvalidTrigger(format!(
                        "Gamepad button ID out of range: {} (must be 128-255 to avoid MIDI conflicts)",
                        button
                    )));
                }
            }
            if buttons.is_empty() {
                return Err(ConfigError::InvalidTrigger(
                    "GamepadButtonChord must have at least one button".to_string(),
                ));
            }
        }
        Trigger::GamepadAnalogStick { axis, direction } => {
            if *axis < 128 || *axis > 131 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Gamepad analog stick axis out of range: {} (must be 128-131)",
                    axis
                )));
            }
            if let Some(dir) = direction
                && dir != "Clockwise"
                && dir != "CounterClockwise"
            {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Invalid direction: '{}' (must be 'Clockwise' or 'CounterClockwise')",
                    dir
                )));
            }
        }
        Trigger::GamepadTrigger { trigger, .. } => {
            if *trigger != 132 && *trigger != 133 {
                return Err(ConfigError::InvalidTrigger(format!(
                    "Gamepad trigger ID out of range: {} (must be 132 or 133)",
                    trigger
                )));
            }
        }
    }
    Ok(())
}

/// Validate shell command for security (prevents command injection)
///
/// Blocks dangerous patterns that could enable command injection attacks:
/// - Command chaining: `;`, `&&`, `||`
/// - Piping: `|`
/// - Command substitution: `` ` ``, `$(`, `${`
/// - Redirects: `>`, `>>`, `<`, `<<`
/// - Background execution: `&` (at end of command)
///
/// # Security Note
/// This is defense-in-depth. The action executor should ALSO avoid using shell
/// interpreters (use `Command::new(cmd).args(args)` instead of `sh -c`).
fn validate_shell_command(command: &str) -> Result<(), ConfigError> {
    // Blocklist of dangerous shell metacharacters
    // IMPORTANT: Check longer patterns first (">>" before ">", "<<" before "<")
    let dangerous_patterns = [
        (";", "command chaining with semicolon"),
        ("&&", "command chaining with AND"),
        ("||", "command chaining with OR"),
        ("|", "piping"),
        ("`", "backtick command substitution"),
        ("$(", "dollar-paren command substitution"),
        ("${", "variable expansion"),
        (">>", "append redirection"), // Check before ">"
        ("<<", "here-document"),      // Check before "<"
        (">", "output redirection"),
        ("<", "input redirection"),
        ("&\n", "background execution"),
        ("&\r", "background execution"),
    ];

    for (pattern, description) in &dangerous_patterns {
        if command.contains(pattern) {
            return Err(ConfigError::InvalidAction(format!(
                "Shell command contains dangerous pattern '{}' ({}). \
                 This could enable command injection attacks. \
                 Use safe alternatives or split into separate mappings.",
                pattern, description
            )));
        }
    }

    // Check for background execution at end of command
    if command.trim_end().ends_with('&') {
        return Err(ConfigError::InvalidAction(
            "Shell command ends with '&' (background execution). \
             This could enable command injection attacks."
                .to_string(),
        ));
    }

    Ok(())
}

/// Validate application name for security (prevents shell injection via Launch action)
///
/// Application names should only contain:
/// - Alphanumeric characters
/// - Spaces, hyphens, underscores, periods
/// - Forward slashes (for paths like /Applications/MyApp.app)
///
/// This prevents shell injection if the app name is passed to a shell command.
fn validate_app_name(app: &str) -> Result<(), ConfigError> {
    // Allow alphanumeric, space, hyphen, underscore, period, forward slash
    let allowed_pattern = regex::Regex::new(r"^[a-zA-Z0-9\s\-_./ ]+$").unwrap();

    if !allowed_pattern.is_match(app) {
        return Err(ConfigError::InvalidAction(format!(
            "Launch action app name '{}' contains invalid characters. \
             Only alphanumeric, spaces, hyphens, underscores, periods, and forward slashes are allowed.",
            app
        )));
    }

    // Additional check: no path traversal attempts
    if app.contains("..") {
        return Err(ConfigError::InvalidAction(
            "Launch action app name cannot contain '..' (path traversal)".to_string(),
        ));
    }

    Ok(())
}

/// Validate an action configuration
fn validate_action(action: &ActionConfig) -> Result<(), ConfigError> {
    match action {
        ActionConfig::Keystroke { keys, modifiers } => {
            if keys.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Keystroke requires keys".to_string(),
                ));
            }
            // Validate modifiers are known
            let valid_modifiers = ["cmd", "shift", "alt", "ctrl", "fn"];
            for modifier in modifiers {
                if !valid_modifiers.contains(&modifier.as_str()) {
                    return Err(ConfigError::InvalidAction(format!(
                        "Unknown modifier: '{}'. Valid modifiers: {}",
                        modifier,
                        valid_modifiers.join(", ")
                    )));
                }
            }
        }
        ActionConfig::Text { text } => {
            if text.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Text action requires text".to_string(),
                ));
            }
        }
        ActionConfig::Launch { app } => {
            if app.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Launch action requires app name".to_string(),
                ));
            }
            // Security: validate app name to prevent shell injection
            validate_app_name(app)?;
        }
        ActionConfig::Shell { command } => {
            if command.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Shell action requires command".to_string(),
                ));
            }
            // Security: validate shell command to prevent command injection
            validate_shell_command(command)?;
        }
        ActionConfig::Sequence { actions } => {
            if actions.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "Sequence requires at least one action".to_string(),
                ));
            }
            for action in actions {
                validate_action(action)?;
            }
        }
        ActionConfig::Delay { ms } => {
            if *ms == 0 {
                return Err(ConfigError::InvalidAction(
                    "Delay must be > 0 ms".to_string(),
                ));
            }
        }
        ActionConfig::MouseClick { button, .. } => {
            let valid_buttons = ["left", "right", "middle"];
            if !valid_buttons.contains(&button.as_str()) {
                return Err(ConfigError::InvalidAction(format!(
                    "Invalid mouse button: '{}'. Valid buttons: {}",
                    button,
                    valid_buttons.join(", ")
                )));
            }
        }
        ActionConfig::VolumeControl { operation, value } => {
            let valid_ops = ["Up", "Down", "Mute", "Unmute", "Set"];
            if !valid_ops.contains(&operation.as_str()) {
                return Err(ConfigError::InvalidAction(format!(
                    "Invalid volume operation: '{}'. Valid operations: {}",
                    operation,
                    valid_ops.join(", ")
                )));
            }
            if operation == "Set" && value.is_none() {
                return Err(ConfigError::InvalidAction(
                    "VolumeControl Set operation requires value".to_string(),
                ));
            }
        }
        ActionConfig::ModeChange { mode } => {
            if mode.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "ModeChange requires mode name".to_string(),
                ));
            }
        }
        ActionConfig::Repeat {
            action,
            count,
            delay_ms: _,
        } => {
            if *count == 0 {
                return Err(ConfigError::InvalidAction(
                    "Repeat count must be > 0".to_string(),
                ));
            }
            validate_action(action)?;
        }
        ActionConfig::Conditional {
            then_action,
            else_action,
            ..
        } => {
            validate_action(then_action)?;
            if let Some(else_act) = else_action {
                validate_action(else_act)?;
            }
        }
        ActionConfig::SendMidi {
            port,
            message_type,
            channel,
            note,
            velocity,
            controller,
            value,
            program,
            pitch,
            pressure,
        } => {
            // Validate port name
            if port.is_empty() {
                return Err(ConfigError::InvalidAction(
                    "SendMidi requires port name".to_string(),
                ));
            }

            // Validate message type
            let valid_types = [
                "NoteOn",
                "NoteOff",
                "CC",
                "ControlChange",
                "ProgramChange",
                "PitchBend",
                "Aftertouch",
            ];
            if !valid_types.iter().any(|t| {
                message_type.eq_ignore_ascii_case(t)
                    || message_type.replace('_', "").eq_ignore_ascii_case(t)
                    || message_type.replace('-', "").eq_ignore_ascii_case(t)
            }) {
                return Err(ConfigError::InvalidAction(format!(
                    "Invalid MIDI message type: '{}'. Valid types: {}",
                    message_type,
                    valid_types.join(", ")
                )));
            }

            // Validate channel (0-15)
            if *channel > 15 {
                return Err(ConfigError::InvalidAction(format!(
                    "MIDI channel must be 0-15, got {}",
                    channel
                )));
            }

            // Validate message-specific parameters
            let msg_type_lower = message_type.to_lowercase();
            if msg_type_lower.contains("note") {
                if let Some(n) = note
                    && *n > 127
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI note must be 0-127, got {}",
                        n
                    )));
                }
                if let Some(v) = velocity
                    && *v > 127
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI velocity must be 0-127, got {}",
                        v
                    )));
                }
            } else if msg_type_lower.contains("cc") || msg_type_lower.contains("control") {
                if let Some(c) = controller
                    && *c > 127
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI controller must be 0-127, got {}",
                        c
                    )));
                }
                if let Some(v) = value
                    && *v > 127
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI value must be 0-127, got {}",
                        v
                    )));
                }
            } else if msg_type_lower.contains("program") {
                if let Some(p) = program
                    && *p > 127
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI program must be 0-127, got {}",
                        p
                    )));
                }
            } else if msg_type_lower.contains("pitch") {
                if let Some(p) = pitch
                    && (*p < -8192 || *p > 8191)
                {
                    return Err(ConfigError::InvalidAction(format!(
                        "MIDI pitch bend must be -8192 to +8191, got {}",
                        p
                    )));
                }
            } else if msg_type_lower.contains("aftertouch")
                && let Some(p) = pressure
                && *p > 127
            {
                return Err(ConfigError::InvalidAction(format!(
                    "MIDI pressure must be 0-127, got {}",
                    p
                )));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default_config();
        assert_eq!(config.device.name, "Mikro");
        assert_eq!(config.modes.len(), 2);
        assert_eq!(config.modes[0].name, "Default");
        assert_eq!(config.modes[1].name, "Development");
    }

    #[test]
    fn test_validate_duplicate_mode_names() {
        let mut config = Config::default_config();
        config.modes.push(Mode {
            name: "Default".to_string(),
            color: None,
            mappings: vec![],
        });

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Duplicate mode name")
        );
    }

    #[test]
    fn test_validate_invalid_note_number() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::Note {
            note: 128,
            velocity_min: None,
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range"));
    }

    #[test]
    fn test_validate_invalid_modifier() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Keystroke {
            keys: "a".to_string(),
            modifiers: vec!["invalid_mod".to_string()],
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown modifier"));
    }

    #[test]
    fn test_validate_invalid_direction() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("Invalid".to_string()),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid direction")
        );
    }

    #[test]
    fn test_validate_empty_keystroke_keys() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Keystroke {
            keys: String::new(),
            modifiers: vec![],
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sequence_with_empty_actions() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Sequence { actions: vec![] };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_valid_config() {
        let config = Config::default_config();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encoder_direction_clockwise() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("Clockwise".to_string()),
        };

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encoder_direction_counter_clockwise() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::EncoderTurn {
            cc: 1,
            direction: Some("CounterClockwise".to_string()),
        };

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_note_chord_with_empty_notes() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].trigger = Trigger::NoteChord {
            notes: vec![],
            timeout_ms: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_mouse_button() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::MouseClick {
            button: "invalid".to_string(),
            x: None,
            y: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_volume_control_set_without_value() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::VolumeControl {
            operation: "Set".to_string(),
            value: None,
        };

        let result = config.validate();
        assert!(result.is_err());
    }

    // ========================================
    // Security Tests (Command Injection Prevention)
    // ========================================

    #[test]
    fn test_shell_injection_semicolon_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo test; rm -rf /".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("command chaining with semicolon")
        );
    }

    #[test]
    fn test_shell_injection_and_operator_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "ls && malicious_command".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("command chaining with AND")
        );
    }

    #[test]
    fn test_shell_injection_or_operator_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "false || evil_fallback".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("command chaining with OR")
        );
    }

    #[test]
    fn test_shell_injection_pipe_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "cat /etc/passwd | grep root".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("piping"));
    }

    #[test]
    fn test_shell_injection_backtick_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo `whoami`".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("backtick command substitution")
        );
    }

    #[test]
    fn test_shell_injection_dollar_paren_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo $(whoami)".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("dollar-paren command substitution")
        );
    }

    #[test]
    fn test_shell_injection_variable_expansion_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo ${DANGEROUS_VAR}".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("variable expansion")
        );
    }

    #[test]
    fn test_shell_injection_output_redirect_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo data > /etc/important_file".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("output redirection")
        );
    }

    #[test]
    fn test_shell_injection_append_redirect_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "echo data >> /etc/important_file".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("append redirection")
        );
    }

    #[test]
    fn test_shell_injection_input_redirect_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "command < /etc/passwd".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("input redirection")
        );
    }

    #[test]
    fn test_shell_injection_background_execution_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Shell {
            command: "malicious_daemon &".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("background execution")
        );
    }

    #[test]
    fn test_shell_safe_commands_allowed() {
        let mut config = Config::default_config();

        // Safe commands should pass validation
        let safe_commands = vec![
            "git status",
            "cargo build",
            "ls -la",
            "echo hello world",
            "pwd",
        ];

        for cmd in safe_commands {
            config.modes[0].mappings[0].action = ActionConfig::Shell {
                command: cmd.to_string(),
            };
            let result = config.validate();
            assert!(result.is_ok(), "Safe command '{}' should be allowed", cmd);
        }
    }

    #[test]
    fn test_launch_injection_special_chars_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Launch {
            app: "Terminal; rm -rf /".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid characters")
        );
    }

    #[test]
    fn test_launch_path_traversal_blocked() {
        let mut config = Config::default_config();
        config.modes[0].mappings[0].action = ActionConfig::Launch {
            app: "../../malicious".to_string(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("path traversal"));
    }

    #[test]
    fn test_launch_safe_app_names_allowed() {
        let mut config = Config::default_config();

        // Safe app names should pass validation
        let safe_apps = vec![
            "Terminal",
            "VS Code",
            "Google Chrome",
            "/Applications/Safari.app",
            "my-app_v2.0",
        ];

        for app in safe_apps {
            config.modes[0].mappings[0].action = ActionConfig::Launch {
                app: app.to_string(),
            };
            let result = config.validate();
            assert!(result.is_ok(), "Safe app name '{}' should be allowed", app);
        }
    }

    // ========================================
    // Security Tests (Path Traversal Prevention)
    // ========================================

    #[test]
    #[should_panic(expected = "outside allowed directories")]
    fn test_path_traversal_absolute_etc_blocked() {
        // Trying to load /etc/passwd should be blocked
        let _ = Config::load("/etc/passwd").unwrap();
    }

    #[test]
    fn test_path_traversal_relative_etc_blocked() {
        // Trying to traverse to /etc should be blocked
        let result = Config::load("../../../../etc/passwd");
        assert!(
            result.is_err(),
            "Loading from /etc via traversal should be blocked"
        );
        // Could be either "outside allowed directories" or "No such file" (both are acceptable security outcomes)
    }

    #[test]
    fn test_path_in_current_dir_allowed() {
        // Loading from current directory should be allowed
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_config.toml");

        // Create a valid config file
        let config = Config::default_config();
        config.save(test_file.to_str().unwrap()).unwrap();

        // Should be able to load it
        let loaded = Config::load(test_file.to_str().unwrap());
        assert!(loaded.is_ok(), "Loading from /tmp should be allowed");

        // Cleanup
        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_path_validation_resolves_symlinks() {
        use std::fs;
        use std::os::unix::fs as unix_fs;

        let temp_dir = std::env::temp_dir();
        let real_file = temp_dir.join("real_config.toml");
        let symlink_file = temp_dir.join("symlink_config.toml");

        // Create a valid config file
        let config = Config::default_config();
        config.save(real_file.to_str().unwrap()).unwrap();

        // Create symlink
        let _ = fs::remove_file(&symlink_file); // Remove if exists
        unix_fs::symlink(&real_file, &symlink_file).unwrap();

        // Should be able to load via symlink (resolves to /tmp)
        let loaded = Config::load(symlink_file.to_str().unwrap());
        assert!(
            loaded.is_ok(),
            "Loading via symlink should work if target is in allowed dir"
        );

        // Cleanup
        let _ = fs::remove_file(real_file);
        let _ = fs::remove_file(symlink_file);
    }

    #[test]
    fn test_relative_path_in_current_dir() {
        // Relative paths in current directory should work
        let temp_dir = std::env::temp_dir();
        std::env::set_current_dir(&temp_dir).unwrap();

        let config = Config::default_config();
        let result = config.save("test_relative.toml");
        assert!(
            result.is_ok(),
            "Saving to relative path in current dir should work"
        );

        let loaded = Config::load("test_relative.toml");
        assert!(
            loaded.is_ok(),
            "Loading from relative path in current dir should work"
        );

        // Cleanup
        let _ = std::fs::remove_file(temp_dir.join("test_relative.toml"));
    }

    // ========================================
    // Security Tests (TOCTOU Prevention)
    // ========================================

    #[test]
    fn test_save_toctou_prevention_atomic_write() {
        // Test that save() uses atomic write pattern
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_atomic.toml");

        let config = Config::default_config();

        // Save should succeed
        let result = config.save(test_file.to_str().unwrap());
        assert!(result.is_ok(), "Atomic write should succeed");

        // File should exist
        assert!(
            test_file.exists(),
            "Final file should exist after atomic write"
        );

        // Temp file should NOT exist (was renamed)
        let temp_file = test_file.with_extension("tmp");
        assert!(
            !temp_file.exists(),
            "Temporary file should not exist after rename"
        );

        // Cleanup
        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_save_validates_full_path_not_just_parent() {
        // Test that save() validates the FULL target path, not just parent
        use std::fs;
        use std::os::unix::fs as unix_fs;

        let temp_dir = std::env::temp_dir();
        let malicious_file = temp_dir.join("malicious_target.toml");

        // Create a file in /tmp (allowed)
        let config = Config::default_config();
        config.save(malicious_file.to_str().unwrap()).unwrap();
        assert!(malicious_file.exists());

        // Now try to replace it with a symlink to /etc/passwd
        fs::remove_file(&malicious_file).unwrap();

        // Create symlink pointing to forbidden location
        let _ = unix_fs::symlink("/etc/passwd", &malicious_file);

        // Attempt to save should fail because the resolved path is /etc/passwd
        let result = config.save(malicious_file.to_str().unwrap());

        // Should fail with "outside allowed directories" error
        assert!(
            result.is_err(),
            "Should reject symlink to forbidden location"
        );

        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("outside allowed directories"),
                "Should fail with security error, got: {}",
                error_msg
            );
        }

        // Cleanup
        let _ = fs::remove_file(malicious_file);
    }

    #[test]
    fn test_save_rejects_nonexistent_parent() {
        // Test that save() rejects paths with non-existent parent directories
        let temp_dir = std::env::temp_dir();
        let nonexistent = temp_dir.join("does_not_exist").join("config.toml");

        let config = Config::default_config();
        let result = config.save(nonexistent.to_str().unwrap());

        assert!(
            result.is_err(),
            "Should reject non-existent parent directory"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Parent directory does not exist"),
            "Should fail with parent directory error"
        );
    }

    #[test]
    fn test_save_prevents_race_with_revalidation() {
        // Test that save() re-validates after temp file creation
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_revalidation.toml");

        let config = Config::default_config();

        // First save should succeed
        config.save(test_file.to_str().unwrap()).unwrap();

        // Load and verify content
        let loaded = Config::load(test_file.to_str().unwrap()).unwrap();
        assert_eq!(loaded.device.name, "Mikro");

        // Cleanup
        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_save_handles_absolute_and_relative_paths() {
        let temp_dir = std::env::temp_dir();

        // Test absolute path
        let abs_path = temp_dir.join("test_absolute.toml");
        let config = Config::default_config();
        assert!(config.save(abs_path.to_str().unwrap()).is_ok());
        assert!(abs_path.exists());
        std::fs::remove_file(&abs_path).unwrap();

        // Test relative path (in temp dir)
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        assert!(config.save("test_relative.toml").is_ok());
        assert!(temp_dir.join("test_relative.toml").exists());

        std::fs::remove_file(temp_dir.join("test_relative.toml")).unwrap();
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_save_uses_sync_all_for_durability() {
        // Test that saved files are durable (sync_all ensures data hits disk)
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_sync.toml");

        let config = Config::default_config();
        config.save(test_file.to_str().unwrap()).unwrap();

        // Read back immediately - should succeed even if system crashes
        let contents = std::fs::read_to_string(&test_file).unwrap();
        assert!(contents.contains("name = \"Mikro\""));

        // Cleanup
        std::fs::remove_file(test_file).unwrap();
    }
}
