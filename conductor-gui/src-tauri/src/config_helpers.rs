// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration helpers for auto-filling trigger configs from MIDI Learn

use crate::midi_learn::TriggerSuggestion;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Trigger configuration that can be serialized to TOML or JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TriggerConfig {
    Note {
        note: u8,
        velocity_min: Option<u8>,
    },
    VelocityRange {
        note: u8,
        soft_max: Option<u8>,
        medium_max: Option<u8>,
    },
    LongPress {
        note: u8,
        duration_ms: Option<u64>,
    },
    DoubleTap {
        note: u8,
        timeout_ms: Option<u64>,
    },
    NoteChord {
        notes: Vec<u8>,
        timeout_ms: Option<u64>,
    },
    EncoderTurn {
        cc: u8,
        direction: Option<String>,
    },
    CC {
        cc: u8,
        value_min: Option<u8>,
    },
    Aftertouch {
        pressure_min: Option<u8>,
    },
    PitchBend {
        value_min: Option<u16>,
        value_max: Option<u16>,
    },
    // Gamepad trigger types (v3.0)
    GamepadButton {
        button: u8,
        velocity_min: Option<u8>,
    },
    GamepadButtonChord {
        buttons: Vec<u8>,
        timeout_ms: Option<u64>,
    },
    GamepadAnalogStick {
        axis: u8,
        direction: Option<String>,
    },
    GamepadTrigger {
        trigger: u8,
        threshold: Option<u8>,
    },
}

/// Convert TriggerSuggestion to TriggerConfig suitable for config.toml
pub fn suggestion_to_config(suggestion: &TriggerSuggestion) -> TriggerConfig {
    match suggestion {
        TriggerSuggestion::Note {
            note,
            velocity_range,
        } => TriggerConfig::Note {
            note: *note,
            velocity_min: velocity_range.map(|(min, _)| min),
        },
        TriggerSuggestion::VelocityRange {
            note,
            velocity_min: _,
            velocity_max,
            level,
        } => {
            // Convert back to soft_max/medium_max format
            let (soft_max, medium_max) = match level.as_str() {
                "soft" => (Some(*velocity_max), Some(80)), // soft: 0-40, medium: 41-80
                "medium" => (Some(40), Some(*velocity_max)), // soft: 0-40, medium: 41-80
                "hard" => (Some(40), Some(80)), // soft: 0-40, medium: 41-80, hard: 81-127
                _ => (None, None),
            };

            TriggerConfig::VelocityRange {
                note: *note,
                soft_max,
                medium_max,
            }
        }
        TriggerSuggestion::LongPress { note, duration_ms } => TriggerConfig::LongPress {
            note: *note,
            duration_ms: Some(*duration_ms),
        },
        TriggerSuggestion::DoubleTap { note, timeout_ms } => TriggerConfig::DoubleTap {
            note: *note,
            timeout_ms: Some(*timeout_ms),
        },
        TriggerSuggestion::Chord { notes, window_ms } => TriggerConfig::NoteChord {
            notes: notes.clone(),
            timeout_ms: Some(*window_ms),
        },
        TriggerSuggestion::Encoder { cc, direction } => TriggerConfig::EncoderTurn {
            cc: *cc,
            direction: direction.clone(),
        },
        TriggerSuggestion::CC { cc, value_range } => TriggerConfig::CC {
            cc: *cc,
            value_min: value_range.map(|(min, _)| min),
        },
        TriggerSuggestion::Aftertouch {
            note: _,
            pressure_range,
        } => {
            // Note: current midimon-core Aftertouch doesn't have note field,
            // only pressure_min, so we ignore the note for now
            TriggerConfig::Aftertouch {
                pressure_min: Some(pressure_range.0),
            }
        }
        TriggerSuggestion::PitchBend { bend_range } => {
            // Convert i16 to u16 (0-16383 range)
            let min_u16 = (bend_range.0 + 8192).max(0).min(16383) as u16;
            let max_u16 = (bend_range.1 + 8192).max(0).min(16383) as u16;

            TriggerConfig::PitchBend {
                value_min: Some(min_u16),
                value_max: Some(max_u16),
            }
        }
        // Gamepad trigger conversions (v3.0)
        TriggerSuggestion::GamepadButton {
            button,
            velocity_range,
        } => TriggerConfig::GamepadButton {
            button: *button,
            velocity_min: velocity_range.map(|(min, _)| min),
        },
        TriggerSuggestion::GamepadButtonChord { buttons, window_ms } => {
            TriggerConfig::GamepadButtonChord {
                buttons: buttons.clone(),
                timeout_ms: Some(*window_ms),
            }
        }
        TriggerSuggestion::GamepadAnalogStick { axis, direction } => {
            TriggerConfig::GamepadAnalogStick {
                axis: *axis,
                direction: direction.clone(),
            }
        }
        TriggerSuggestion::GamepadTrigger { trigger, threshold } => {
            TriggerConfig::GamepadTrigger {
                trigger: *trigger,
                threshold: Some(*threshold),
            }
        }
    }
}

/// Convert TriggerConfig to TOML string
pub fn config_to_toml(config: &TriggerConfig) -> Result<String, toml::ser::Error> {
    toml::to_string_pretty(config)
}

/// Convert TriggerConfig to JSON value
pub fn config_to_json(config: &TriggerConfig) -> serde_json::Value {
    serde_json::to_value(config).unwrap_or_else(|_| json!({}))
}

/// Generate a full mapping TOML snippet with placeholder action
pub fn generate_mapping_toml(trigger_config: &TriggerConfig, mode_name: &str) -> String {
    let trigger_toml = config_to_toml(trigger_config).unwrap_or_else(|_| String::new());

    format!(
        r#"# Add this to [[modes.mappings]] in config.toml under mode "{}"

[[modes.mappings]]
[modes.mappings.trigger]
{}

[modes.mappings.action]
type = "Keystroke"
keys = "space"  # TODO: Replace with desired action
modifiers = []
"#,
        mode_name,
        trigger_toml
            .lines()
            .map(|line| format!("{}", line))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_suggestion_to_config() {
        let suggestion = TriggerSuggestion::Note {
            note: 60,
            velocity_range: Some((1, 127)),
        };

        let config = suggestion_to_config(&suggestion);
        assert!(matches!(
            config,
            TriggerConfig::Note {
                note: 60,
                velocity_min: Some(1)
            }
        ));
    }

    #[test]
    fn test_velocity_range_soft() {
        let suggestion = TriggerSuggestion::VelocityRange {
            note: 60,
            velocity_min: 0,
            velocity_max: 40,
            level: "soft".to_string(),
        };

        let config = suggestion_to_config(&suggestion);
        if let TriggerConfig::VelocityRange {
            note,
            soft_max,
            medium_max,
        } = config
        {
            assert_eq!(note, 60);
            assert_eq!(soft_max, Some(40));
            assert_eq!(medium_max, Some(80));
        } else {
            panic!("Expected VelocityRange config");
        }
    }

    #[test]
    fn test_long_press() {
        let suggestion = TriggerSuggestion::LongPress {
            note: 60,
            duration_ms: 2500,
        };

        let config = suggestion_to_config(&suggestion);
        assert!(matches!(
            config,
            TriggerConfig::LongPress {
                note: 60,
                duration_ms: Some(2500)
            }
        ));
    }

    #[test]
    fn test_chord() {
        let suggestion = TriggerSuggestion::Chord {
            notes: vec![60, 64, 67],
            window_ms: 100,
        };

        let config = suggestion_to_config(&suggestion);
        if let TriggerConfig::NoteChord { notes, timeout_ms } = config {
            assert_eq!(notes, vec![60, 64, 67]);
            assert_eq!(timeout_ms, Some(100));
        } else {
            panic!("Expected NoteChord config");
        }
    }

    #[test]
    fn test_encoder_with_direction() {
        let suggestion = TriggerSuggestion::Encoder {
            cc: 1,
            direction: Some("clockwise".to_string()),
        };

        let config = suggestion_to_config(&suggestion);
        if let TriggerConfig::EncoderTurn { cc, direction } = config {
            assert_eq!(cc, 1);
            assert_eq!(direction, Some("clockwise".to_string()));
        } else {
            panic!("Expected EncoderTurn config");
        }
    }

    #[test]
    fn test_toml_generation() {
        let config = TriggerConfig::Note {
            note: 60,
            velocity_min: Some(1),
        };

        let toml = config_to_toml(&config).unwrap();
        assert!(toml.contains(r#"type = "Note""#));
        assert!(toml.contains("note = 60"));
        assert!(toml.contains("velocity_min = 1"));
    }

    #[test]
    fn test_full_mapping_generation() {
        let config = TriggerConfig::LongPress {
            note: 60,
            duration_ms: Some(2000),
        };

        let mapping_toml = generate_mapping_toml(&config, "Default");
        assert!(mapping_toml.contains(r#"mode "Default""#));
        assert!(mapping_toml.contains(r#"type = "LongPress""#));
        assert!(mapping_toml.contains("note = 60"));
        assert!(mapping_toml.contains("duration_ms = 2000"));
        assert!(mapping_toml.contains(r#"type = "Keystroke""#));
        assert!(mapping_toml.contains("TODO"));
    }
}
