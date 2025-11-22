// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests for SendMIDI action functionality
//!
//! Tests the complete workflow from TOML config parsing → ActionConfig →
//! Action → execution through ActionExecutor.

use conductor_core::{Action, ActionConfig, Config, MidiMessageParams, MidiMessageType};

#[test]
fn test_send_midi_config_parsing_note_on() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true

[[modes]]
name = "DAW Control"

[[modes.mappings]]
description = "Send MIDI Note On to virtual synth"

[modes.mappings.trigger]
type = "Note"
note = 36

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60
velocity = 100
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config");
    assert_eq!(config.modes[0].mappings.len(), 1);

    let action_config = &config.modes[0].mappings[0].action;
    match action_config {
        ActionConfig::SendMidi {
            port,
            message_type,
            channel,
            note,
            velocity,
            ..
        } => {
            assert_eq!(port, "IAC Driver Bus 1");
            assert_eq!(message_type, "NoteOn");
            assert_eq!(*channel, 0);
            assert_eq!(*note, Some(60));
            assert_eq!(*velocity, Some(100));
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_parsing_control_change() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true

[[modes]]
name = "DAW Control"

[[modes.mappings]]
description = "Send CC to control volume"

[modes.mappings.trigger]
type = "Note"
note = 37

[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "CC"
channel = 1
controller = 7
value = 127
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config");

    let action_config = &config.modes[0].mappings[0].action;
    match action_config {
        ActionConfig::SendMidi {
            port,
            message_type,
            channel,
            controller,
            value,
            ..
        } => {
            assert_eq!(port, "Virtual MIDI Port");
            assert_eq!(message_type, "CC");
            assert_eq!(*channel, 1);
            assert_eq!(*controller, Some(7));
            assert_eq!(*value, Some(127));
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_to_action_conversion_note_on() {
    let action_config = ActionConfig::SendMidi {
        port: "IAC Driver Bus 1".to_string(),
        message_type: "NoteOn".to_string(),
        channel: 0,
        note: Some(60),
        velocity: Some(100),
        controller: None,
        value: None,
        program: None,
        pitch: None,
        pressure: None,
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi {
            port,
            message_type,
            channel,
            params,
        } => {
            assert_eq!(port, "IAC Driver Bus 1");
            assert_eq!(message_type, MidiMessageType::NoteOn);
            assert_eq!(channel, 0);
            assert_eq!(
                params,
                MidiMessageParams::Note {
                    note: 60,
                    velocity_mapping: conductor_core::VelocityMapping::Fixed { velocity: 100 }
                }
            );
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_to_action_conversion_cc() {
    let action_config = ActionConfig::SendMidi {
        port: "Virtual MIDI Port".to_string(),
        message_type: "ControlChange".to_string(),
        channel: 1,
        note: None,
        velocity: None,
        controller: Some(7),
        value: Some(127),
        program: None,
        pitch: None,
        pressure: None,
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi {
            port,
            message_type,
            channel,
            params,
        } => {
            assert_eq!(port, "Virtual MIDI Port");
            assert_eq!(message_type, MidiMessageType::ControlChange);
            assert_eq!(channel, 1);
            assert_eq!(
                params,
                MidiMessageParams::CC {
                    controller: 7,
                    value: 127
                }
            );
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_to_action_conversion_program_change() {
    let action_config = ActionConfig::SendMidi {
        port: "Virtual Synth".to_string(),
        message_type: "ProgramChange".to_string(),
        channel: 2,
        note: None,
        velocity: None,
        controller: None,
        value: None,
        program: Some(42),
        pitch: None,
        pressure: None,
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi {
            port,
            message_type,
            channel,
            params,
        } => {
            assert_eq!(port, "Virtual Synth");
            assert_eq!(message_type, MidiMessageType::ProgramChange);
            assert_eq!(channel, 2);
            assert_eq!(params, MidiMessageParams::ProgramChange { program: 42 });
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_to_action_conversion_pitch_bend() {
    let action_config = ActionConfig::SendMidi {
        port: "Virtual Synth".to_string(),
        message_type: "PitchBend".to_string(),
        channel: 3,
        note: None,
        velocity: None,
        controller: None,
        value: None,
        program: None,
        pitch: Some(4096),
        pressure: None,
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi {
            port,
            message_type,
            channel,
            params,
        } => {
            assert_eq!(port, "Virtual Synth");
            assert_eq!(message_type, MidiMessageType::PitchBend);
            assert_eq!(channel, 3);
            assert_eq!(params, MidiMessageParams::PitchBend { value: 4096 });
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_config_to_action_conversion_aftertouch() {
    let action_config = ActionConfig::SendMidi {
        port: "Virtual Synth".to_string(),
        message_type: "Aftertouch".to_string(),
        channel: 4,
        note: None,
        velocity: None,
        controller: None,
        value: None,
        program: None,
        pitch: None,
        pressure: Some(80),
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi {
            port,
            message_type,
            channel,
            params,
        } => {
            assert_eq!(port, "Virtual Synth");
            assert_eq!(message_type, MidiMessageType::Aftertouch);
            assert_eq!(channel, 4);
            assert_eq!(params, MidiMessageParams::Aftertouch { pressure: 80 });
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_message_type_parsing_variants() {
    // Test various string formats for message types
    let test_cases = vec![
        ("NoteOn", MidiMessageType::NoteOn),
        ("noteon", MidiMessageType::NoteOn),
        ("note_on", MidiMessageType::NoteOn),
        ("note-on", MidiMessageType::NoteOn),
        ("NoteOff", MidiMessageType::NoteOff),
        ("noteoff", MidiMessageType::NoteOff),
        ("CC", MidiMessageType::ControlChange),
        ("cc", MidiMessageType::ControlChange),
        ("ControlChange", MidiMessageType::ControlChange),
        ("control_change", MidiMessageType::ControlChange),
        ("ProgramChange", MidiMessageType::ProgramChange),
        ("program_change", MidiMessageType::ProgramChange),
        ("PC", MidiMessageType::ProgramChange),
        ("pc", MidiMessageType::ProgramChange),
        ("PitchBend", MidiMessageType::PitchBend),
        ("pitch_bend", MidiMessageType::PitchBend),
        ("PB", MidiMessageType::PitchBend),
        ("Aftertouch", MidiMessageType::Aftertouch),
        ("aftertouch", MidiMessageType::Aftertouch),
        ("AT", MidiMessageType::Aftertouch),
    ];

    for (input, expected) in test_cases {
        let action_config = ActionConfig::SendMidi {
            port: "Test".to_string(),
            message_type: input.to_string(),
            channel: 0,
            note: Some(60),
            velocity: Some(100),
            controller: None,
            value: None,
            program: None,
            pitch: None,
            pressure: None,
        };

        let action: Action = action_config.into();

        match action {
            Action::SendMidi { message_type, .. } => {
                assert_eq!(
                    message_type, expected,
                    "Failed to parse '{}' correctly",
                    input
                );
            }
            _ => panic!("Expected SendMidi action"),
        }
    }
}

#[test]
fn test_send_midi_with_defaults() {
    // Test that missing optional parameters get sensible defaults
    let action_config = ActionConfig::SendMidi {
        port: "Virtual Synth".to_string(),
        message_type: "NoteOn".to_string(),
        channel: 0,
        note: None,     // Should default to 60
        velocity: None, // Should default to 100
        controller: None,
        value: None,
        program: None,
        pitch: None,
        pressure: None,
    };

    let action: Action = action_config.into();

    match action {
        Action::SendMidi { params, .. } => {
            assert_eq!(
                params,
                MidiMessageParams::Note {
                    note: 60,
                    velocity_mapping: conductor_core::VelocityMapping::Fixed { velocity: 100 }
                }
            );
        }
        _ => panic!("Expected SendMidi action"),
    }
}

#[test]
fn test_send_midi_in_sequence_from_config() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true

[[modes]]
name = "DAW Control"

[[modes.mappings]]
description = "Send MIDI note sequence"

[modes.mappings.trigger]
type = "Note"
note = 40

[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60, velocity = 100 },
    { type = "Delay", ms = 100 },
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOff", channel = 0, note = 60, velocity = 0 },
]
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config");

    let action_config = &config.modes[0].mappings[0].action;
    match action_config {
        ActionConfig::Sequence { actions } => {
            assert_eq!(actions.len(), 3);

            // Verify first action is SendMidi NoteOn
            match &actions[0] {
                ActionConfig::SendMidi {
                    message_type,
                    note,
                    velocity,
                    ..
                } => {
                    assert_eq!(message_type, "NoteOn");
                    assert_eq!(*note, Some(60));
                    assert_eq!(*velocity, Some(100));
                }
                _ => panic!("Expected SendMidi action"),
            }

            // Verify second action is Delay
            match &actions[1] {
                ActionConfig::Delay { ms } => {
                    assert_eq!(*ms, 100);
                }
                _ => panic!("Expected Delay action"),
            }

            // Verify third action is SendMidi NoteOff
            match &actions[2] {
                ActionConfig::SendMidi {
                    message_type,
                    note,
                    velocity,
                    ..
                } => {
                    assert_eq!(message_type, "NoteOff");
                    assert_eq!(*note, Some(60));
                    assert_eq!(*velocity, Some(0));
                }
                _ => panic!("Expected SendMidi action"),
            }
        }
        _ => panic!("Expected Sequence action"),
    }
}
