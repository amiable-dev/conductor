// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Action types and parsing for MIDIMon core engine.
//!
//! This module defines domain-specific types (KeyCode, ModifierKey, MouseButton) that are
//! platform-independent and UI-library-agnostic. This enables midimon-core to be truly
//! UI-independent and suitable for WASM/embedded targets.
//!
//! The daemon layer (midimon-daemon/action_executor.rs) is responsible for converting
//! these domain types to platform-specific types (e.g., enigo::Key) for execution.

use crate::config::ActionConfig;
use serde::{Deserialize, Serialize};

/// Platform-independent keyboard key codes
///
/// This enum represents keyboard keys without depending on any UI library.
/// The daemon layer converts these to platform-specific key codes for execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    // Alphanumeric keys (handled via Unicode for flexibility)
    Unicode(char),

    // Special keys
    Space,
    Return,
    Tab,
    Escape,
    Backspace,
    Delete,

    // Arrow keys
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

    // Navigation keys
    Home,
    End,
    PageUp,
    PageDown,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,

    // Media keys
    VolumeUp,
    VolumeDown,
    Mute,
    PlayPause,
    Stop,
    NextTrack,
    PreviousTrack,

    // Editing keys
    Insert,
    PrintScreen,
    ScrollLock,
    Pause,
    CapsLock,
    NumLock,
}

/// Platform-independent modifier keys
///
/// Represents keyboard modifiers (Command, Control, Option/Alt, Shift).
/// These are kept separate from KeyCode for clarity and type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModifierKey {
    /// Command key (macOS) / Windows key (Windows) / Meta key (Linux)
    Command,
    /// Control key (all platforms)
    Control,
    /// Option key (macOS) / Alt key (Windows/Linux)
    Option,
    /// Shift key (all platforms)
    Shift,
}

/// Platform-independent mouse button identifiers
///
/// Represents mouse buttons without depending on any UI library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Condition for conditional action execution (v2.2)
///
/// Represents conditions that can be evaluated at runtime to determine
/// whether to execute an action. Supports time-based, app-based, mode-based,
/// and logical operators for complex conditional logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    /// Always evaluates to true (useful for testing)
    Always,

    /// Always evaluates to false (useful for disabling actions)
    Never,

    /// Time-based condition: current time falls within range
    /// Format: start and end times in 24-hour format (HH:MM)
    /// Automatically handles ranges that cross midnight
    TimeRange {
        /// Start time in 24-hour format (e.g., "09:00")
        start: String,
        /// End time in 24-hour format (e.g., "17:30")
        end: String,
    },

    /// Day of week condition
    /// Days: Monday=1, Tuesday=2, ..., Sunday=7
    DayOfWeek {
        /// Days of week when condition is true (1-7)
        days: Vec<u8>,
    },

    /// Application is currently running
    /// Checks if process with given name exists
    AppRunning {
        /// Application name (e.g., "Ableton Live")
        app_name: String,
    },

    /// Application is frontmost (has focus)
    /// Platform-specific implementation
    AppFrontmost {
        /// Application name (e.g., "Ableton Live")
        app_name: String,
    },

    /// Current mode matches
    ModeIs {
        /// Mode name to match
        mode: String,
    },

    /// Logical AND of multiple conditions
    And {
        /// Conditions that must all be true
        conditions: Vec<Condition>,
    },

    /// Logical OR of multiple conditions
    Or {
        /// At least one condition must be true
        conditions: Vec<Condition>,
    },

    /// Logical NOT (negation)
    Not {
        /// Condition to negate
        condition: Box<Condition>,
    },
}

/// Action to be executed when a trigger is matched
///
/// This enum uses domain-specific types (KeyCode, ModifierKey, MouseButton) instead
/// of UI library types, making the core engine truly platform-independent.
#[derive(Debug, Clone)]
pub enum Action {
    Keystroke {
        keys: Vec<KeyCode>,
        modifiers: Vec<ModifierKey>,
    },
    Text(String),
    Launch(String),
    Shell(String),
    Sequence(Vec<Action>),
    Delay(u64),
    MouseClick {
        button: MouseButton,
        x: Option<i32>,
        y: Option<i32>,
    },
    Repeat {
        action: Box<Action>,
        count: usize,
        delay_ms: Option<u64>,
    },
    Conditional {
        condition: Condition,
        then_action: Box<Action>,
        else_action: Option<Box<Action>>,
    },
    VolumeControl {
        operation: VolumeOperation,
        value: Option<u8>,
    },
    ModeChange {
        mode: String,
    },
    SendMidi {
        port: String,
        message_type: MidiMessageType,
        channel: u8,
        params: MidiMessageParams,
    },
    /// Plugin action (v2.3)
    ///
    /// Execute a custom action plugin with given parameters.
    /// The plugin must be installed and enabled for this to work.
    Plugin {
        /// Plugin identifier (must match ActionPlugin::name())
        plugin: String,
        /// Plugin-specific parameters (JSON object)
        params: serde_json::Value,
    },
}

/// MIDI message type (v2.1)
///
/// Represents the type of MIDI message to send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MidiMessageType {
    NoteOn,
    NoteOff,
    ControlChange,
    ProgramChange,
    PitchBend,
    Aftertouch,
}

/// Velocity mapping mode for SendMIDI actions (v2.2)
///
/// Defines how trigger velocity is mapped to output MIDI velocity.
/// This enables dynamic velocity control where pad dynamics can be preserved,
/// scaled, or transformed when sending MIDI notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VelocityMapping {
    /// Fixed velocity (current behavior, v2.1 compatibility)
    /// Always send the same velocity regardless of trigger velocity
    Fixed {
        velocity: u8, // 0-127
    },

    /// Pass-through mode (1:1 mapping)
    /// Output velocity = trigger velocity
    PassThrough,

    /// Linear scaling with configurable range
    /// Maps input range (0-127) to output range (min-max)
    Linear {
        min: u8, // Minimum output velocity (0-127)
        max: u8, // Maximum output velocity (0-127)
    },

    /// Curve-based transformation
    /// Applies non-linear curve to velocity values
    Curve {
        curve_type: VelocityCurve,
        intensity: f32, // 0.0-1.0, curve strength
    },
}

/// Velocity curve types for non-linear transformations
///
/// Defines the shape of the velocity mapping curve.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VelocityCurve {
    /// Exponential curve (soft hits louder)
    /// Output = input ^ (1 + intensity)
    /// Makes soft hits louder while preserving hard hits
    Exponential,

    /// Logarithmic curve (soft hits quieter)
    /// Output = log(1 + input * intensity) / log(1 + 127 * intensity) * 127
    /// Compresses dynamic range, makes soft hits quieter
    Logarithmic,

    /// S-curve (sigmoid) for smooth transitions
    /// Output = 127 / (1 + exp(-intensity * (input - 63.5)))
    /// Creates smooth acceleration in the middle range
    SCurve,
}

/// MIDI message parameters (v2.2 - updated for variable velocity)
///
/// Type-specific parameters for MIDI messages.
/// Note messages now support velocity mapping instead of fixed velocity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MidiMessageParams {
    Note {
        note: u8,
        velocity_mapping: VelocityMapping, // v2.2: was `velocity: u8`
    },
    CC {
        controller: u8,
        value: u8,
    },
    ProgramChange {
        program: u8,
    },
    PitchBend {
        value: i16, // -8192 to +8191
    },
    Aftertouch {
        pressure: u8,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum VolumeOperation {
    Up,
    Down,
    Mute,
    Unmute,
    Set,
}

// ActionExecutor has been moved to midimon-daemon (Phase 2 refactor)
// Only pure Action types and parsing remain in core

impl From<ActionConfig> for Action {
    fn from(config: ActionConfig) -> Self {
        match config {
            ActionConfig::Keystroke { keys, modifiers } => Action::Keystroke {
                keys: parse_keys(&keys),
                modifiers: modifiers.iter().flat_map(|m| parse_modifier(m)).collect(),
            },
            ActionConfig::Text { text } => Action::Text(text),
            ActionConfig::Launch { app } => Action::Launch(app),
            ActionConfig::Shell { command } => Action::Shell(command),
            ActionConfig::Sequence { actions } => {
                Action::Sequence(actions.into_iter().map(Into::into).collect())
            }
            ActionConfig::Delay { ms } => Action::Delay(ms),
            ActionConfig::MouseClick { button, x, y } => Action::MouseClick {
                button: parse_mouse_button(&button),
                x,
                y,
            },
            ActionConfig::VolumeControl { operation, value } => Action::VolumeControl {
                operation: parse_volume_operation(&operation),
                value,
            },
            ActionConfig::ModeChange { mode } => Action::ModeChange { mode },
            ActionConfig::Repeat {
                action,
                count,
                delay_ms,
            } => Action::Repeat {
                action: Box::new((*action).into()),
                count,
                delay_ms,
            },
            ActionConfig::Conditional {
                condition,
                then_action,
                else_action,
            } => Action::Conditional {
                condition,
                then_action: Box::new((*then_action).into()),
                else_action: else_action.map(|a| Box::new((*a).into())),
            },
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
                let msg_type = parse_midi_message_type(&message_type);
                let params = match msg_type {
                    MidiMessageType::NoteOn | MidiMessageType::NoteOff => {
                        // v2.2: Use Fixed velocity mapping for backward compatibility
                        // If velocity is specified, create Fixed mapping with that value
                        let velocity_mapping = VelocityMapping::Fixed {
                            velocity: velocity.unwrap_or(100),
                        };
                        MidiMessageParams::Note {
                            note: note.unwrap_or(60),
                            velocity_mapping,
                        }
                    }
                    MidiMessageType::ControlChange => MidiMessageParams::CC {
                        controller: controller.unwrap_or(0),
                        value: value.unwrap_or(0),
                    },
                    MidiMessageType::ProgramChange => MidiMessageParams::ProgramChange {
                        program: program.unwrap_or(0),
                    },
                    MidiMessageType::PitchBend => MidiMessageParams::PitchBend {
                        value: pitch.unwrap_or(0),
                    },
                    MidiMessageType::Aftertouch => MidiMessageParams::Aftertouch {
                        pressure: pressure.unwrap_or(0),
                    },
                };

                Action::SendMidi {
                    port,
                    message_type: msg_type,
                    channel,
                    params,
                }
            }
        }
    }
}

/// Parse a key string into a list of KeyCodes
///
/// Keys are separated by '+' (e.g., "Cmd+Shift+A")
/// This function is used to convert config strings into domain KeyCode types.
fn parse_keys(keys: &str) -> Vec<KeyCode> {
    keys.split('+')
        .filter_map(|k| parse_key(k.trim()))
        .collect()
}

/// Parse a single key string into a KeyCode
///
/// Supports common key names (case-insensitive):
/// - Special keys: "space", "return", "enter", "tab", "escape", etc.
/// - Arrow keys: "up", "down", "left", "right"
/// - Function keys: "f1" through "f20"
/// - Media keys: "volumeup", "volumedown", "mute", "playpause", etc.
/// - Single characters: Any single character (a-z, 0-9, punctuation)
fn parse_key(key: &str) -> Option<KeyCode> {
    match key.to_lowercase().as_str() {
        // Special keys
        "space" => Some(KeyCode::Space),
        "return" | "enter" => Some(KeyCode::Return),
        "tab" => Some(KeyCode::Tab),
        "escape" | "esc" => Some(KeyCode::Escape),
        "backspace" => Some(KeyCode::Backspace),
        "delete" | "del" => Some(KeyCode::Delete),

        // Arrow keys
        "up" | "uparrow" => Some(KeyCode::UpArrow),
        "down" | "downarrow" => Some(KeyCode::DownArrow),
        "left" | "leftarrow" => Some(KeyCode::LeftArrow),
        "right" | "rightarrow" => Some(KeyCode::RightArrow),

        // Navigation keys
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        "pageup" | "pgup" => Some(KeyCode::PageUp),
        "pagedown" | "pgdn" => Some(KeyCode::PageDown),

        // Function keys
        "f1" => Some(KeyCode::F1),
        "f2" => Some(KeyCode::F2),
        "f3" => Some(KeyCode::F3),
        "f4" => Some(KeyCode::F4),
        "f5" => Some(KeyCode::F5),
        "f6" => Some(KeyCode::F6),
        "f7" => Some(KeyCode::F7),
        "f8" => Some(KeyCode::F8),
        "f9" => Some(KeyCode::F9),
        "f10" => Some(KeyCode::F10),
        "f11" => Some(KeyCode::F11),
        "f12" => Some(KeyCode::F12),
        "f13" => Some(KeyCode::F13),
        "f14" => Some(KeyCode::F14),
        "f15" => Some(KeyCode::F15),
        "f16" => Some(KeyCode::F16),
        "f17" => Some(KeyCode::F17),
        "f18" => Some(KeyCode::F18),
        "f19" => Some(KeyCode::F19),
        "f20" => Some(KeyCode::F20),

        // Media keys
        "volumeup" | "volup" => Some(KeyCode::VolumeUp),
        "volumedown" | "voldown" => Some(KeyCode::VolumeDown),
        "mute" => Some(KeyCode::Mute),
        "playpause" | "play" => Some(KeyCode::PlayPause),
        "stop" => Some(KeyCode::Stop),
        "nexttrack" | "next" => Some(KeyCode::NextTrack),
        "previoustrack" | "previous" | "prev" => Some(KeyCode::PreviousTrack),

        // Editing keys
        "insert" | "ins" => Some(KeyCode::Insert),
        "printscreen" | "prtsc" => Some(KeyCode::PrintScreen),
        "scrolllock" | "scrlk" => Some(KeyCode::ScrollLock),
        "pause" => Some(KeyCode::Pause),
        "capslock" | "caps" => Some(KeyCode::CapsLock),
        "numlock" | "numlk" => Some(KeyCode::NumLock),

        // Single character keys (alphanumeric and punctuation)
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Some(KeyCode::Unicode(c))
        }

        _ => None,
    }
}

/// Parse a modifier key string into a ModifierKey
///
/// Supports common modifier aliases (case-insensitive):
/// - Command: "cmd", "command", "meta"
/// - Control: "ctrl", "control"
/// - Option: "alt", "option"
/// - Shift: "shift"
fn parse_modifier(modifier: &str) -> Option<ModifierKey> {
    match modifier.to_lowercase().as_str() {
        "cmd" | "command" | "meta" => Some(ModifierKey::Command),
        "ctrl" | "control" => Some(ModifierKey::Control),
        "alt" | "option" => Some(ModifierKey::Option),
        "shift" => Some(ModifierKey::Shift),
        _ => None,
    }
}

/// Parse a mouse button string into a MouseButton
///
/// Supports: "left" (default), "right", "middle"
fn parse_mouse_button(button: &str) -> MouseButton {
    match button.to_lowercase().as_str() {
        "right" => MouseButton::Right,
        "middle" => MouseButton::Middle,
        _ => MouseButton::Left,
    }
}

fn parse_volume_operation(operation: &str) -> VolumeOperation {
    match operation.to_lowercase().as_str() {
        "up" => VolumeOperation::Up,
        "down" => VolumeOperation::Down,
        "mute" => VolumeOperation::Mute,
        "unmute" => VolumeOperation::Unmute,
        "set" => VolumeOperation::Set,
        _ => {
            eprintln!("Unknown volume operation '{}', defaulting to Up", operation);
            VolumeOperation::Up
        }
    }
}

/// Parse MIDI message type string into enum (v2.1)
///
/// Converts configuration string to MidiMessageType enum variant.
fn parse_midi_message_type(message_type: &str) -> MidiMessageType {
    match message_type.to_lowercase().as_str() {
        "noteon" | "note_on" | "note-on" => MidiMessageType::NoteOn,
        "noteoff" | "note_off" | "note-off" => MidiMessageType::NoteOff,
        "cc" | "controlchange" | "control_change" | "control-change" => {
            MidiMessageType::ControlChange
        }
        "programchange" | "program_change" | "program-change" | "pc" => {
            MidiMessageType::ProgramChange
        }
        "pitchbend" | "pitch_bend" | "pitch-bend" | "pb" => MidiMessageType::PitchBend,
        "aftertouch" | "at" => MidiMessageType::Aftertouch,
        _ => {
            eprintln!(
                "Unknown MIDI message type '{}', defaulting to NoteOn",
                message_type
            );
            MidiMessageType::NoteOn
        }
    }
}

// Condition evaluation and volume control moved to midimon-daemon/action_executor.rs

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Action execution tests have been moved to midimon-daemon/action_executor.rs
    // These tests now only cover parsing and conversion from ActionConfig to Action

    #[test]
    fn test_action_config_repeat_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::Repeat {
            action: Box::new(ActionConfig::Text {
                text: "test".to_string(),
            }),
            count: 5,
            delay_ms: Some(100),
        };

        let action: Action = config.into();

        match action {
            Action::Repeat {
                count, delay_ms, ..
            } => {
                assert_eq!(count, 5);
                assert_eq!(delay_ms, Some(100));
            }
            _ => panic!("Expected Repeat action"),
        }
    }

    #[test]
    fn test_action_config_conditional_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::Conditional {
            condition: Condition::Always,
            then_action: Box::new(ActionConfig::Text {
                text: "then".to_string(),
            }),
            else_action: Some(Box::new(ActionConfig::Text {
                text: "else".to_string(),
            })),
        };

        let action: Action = config.into();

        match action {
            Action::Conditional { condition, .. } => {
                assert_eq!(condition, Condition::Always);
            }
            _ => panic!("Expected Conditional action"),
        }
    }

    #[test]
    fn test_parse_volume_operation() {
        assert_eq!(parse_volume_operation("Up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("up"), VolumeOperation::Up);
        assert_eq!(parse_volume_operation("Down"), VolumeOperation::Down);
        assert_eq!(parse_volume_operation("Mute"), VolumeOperation::Mute);
        assert_eq!(parse_volume_operation("Unmute"), VolumeOperation::Unmute);
        assert_eq!(parse_volume_operation("Set"), VolumeOperation::Set);
        // Unknown operations default to Up
        assert_eq!(parse_volume_operation("invalid"), VolumeOperation::Up);
    }

    #[test]
    fn test_volume_control_action_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::VolumeControl {
            operation: "Up".to_string(),
            value: None,
        };
        let action: Action = config.into();

        match action {
            Action::VolumeControl { operation, value } => {
                assert_eq!(operation, VolumeOperation::Up);
                assert_eq!(value, None);
            }
            _ => panic!("Expected VolumeControl action"),
        }
    }

    #[test]
    fn test_mode_change_action_conversion() {
        use crate::config::ActionConfig;

        let config = ActionConfig::ModeChange {
            mode: "Development".to_string(),
        };
        let action: Action = config.into();

        match action {
            Action::ModeChange { mode } => {
                assert_eq!(mode, "Development");
            }
            _ => panic!("Expected ModeChange action"),
        }
    }
}
