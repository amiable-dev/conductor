// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration test for chord→action mapping
//!
//! Tests that NoteChord triggers correctly map to actions through the MappingEngine.

use conductor_core::{Config, EventProcessor, MappingEngine, MidiEvent};
use std::time::Instant;

#[test]
fn test_chord_to_action_mapping() {
    // Create a config with a chord trigger
    let config_toml = r#"
        [device]
        name = "Test Device"
        auto_connect = false

        [[modes]]
        name = "Test Mode"

        [[modes.mappings]]
        description = "Emergency exit chord"
        [modes.mappings.trigger]
        type = "NoteChord"
        notes = [1, 5, 9]  # Top left, top right, bottom right corners

        [modes.mappings.action]
        type = "Shell"
        command = "echo 'Emergency exit triggered!'"
    "#;

    let config: Config = toml::from_str(config_toml).expect("Failed to parse config");

    // Load config into mapping engine
    let mut mapping_engine = MappingEngine::new();
    mapping_engine.load_from_config(&config);

    // Create event processor
    let mut event_processor = EventProcessor::new();

    // Simulate pressing all three notes within chord timeout (50ms default)
    let base_time = Instant::now();

    let note1 = MidiEvent::NoteOn {
        note: 1,
        velocity: 100,
        time: base_time,
    };

    let note2 = MidiEvent::NoteOn {
        note: 5,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(10),
    };

    let note3 = MidiEvent::NoteOn {
        note: 9,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(20),
    };

    // Process each MIDI event
    let _processed1 = event_processor.process(note1.clone());
    let _processed2 = event_processor.process(note2.clone());
    let processed3 = event_processor.process(note3.clone());

    // The third note should trigger chord detection
    let has_chord = processed3.iter().any(|e| {
        matches!(e, conductor_core::event_processor::ProcessedEvent::ChordDetected { .. })
    });

    assert!(has_chord, "ChordDetected event should be generated after third note");

    // Find the chord event
    let chord_event = processed3
        .iter()
        .find(|e| matches!(e, conductor_core::event_processor::ProcessedEvent::ChordDetected { .. }))
        .expect("ChordDetected event not found");

    // Try to get action for the chord event (mode 0)
    let action = mapping_engine.get_action_for_processed(chord_event, 0);

    assert!(action.is_some(), "Mapping engine should find an action for the chord");

    // Verify it's the correct action
    if let Some(action) = action {
        let action_str = format!("{:?}", action);
        assert!(
            action_str.contains("Shell") && action_str.contains("Emergency exit"),
            "Action should be the Shell command we configured"
        );
    }
}

#[test]
fn test_chord_with_wrong_notes_does_not_match() {
    let config_toml = r#"
        [device]
        name = "Test Device"
        auto_connect = false

        [[modes]]
        name = "Test Mode"

        [[modes.mappings]]
        [modes.mappings.trigger]
        type = "NoteChord"
        notes = [1, 5, 9]

        [modes.mappings.action]
        type = "Shell"
        command = "echo 'chord'"
    "#;

    let config: Config = toml::from_str(config_toml).expect("Failed to parse config");

    let mut mapping_engine = MappingEngine::new();
    mapping_engine.load_from_config(&config);

    let mut event_processor = EventProcessor::new();

    // Press different notes (2, 6, 10 instead of 1, 5, 9)
    let base_time = Instant::now();

    event_processor.process(MidiEvent::NoteOn {
        note: 2,
        velocity: 100,
        time: base_time,
    });

    event_processor.process(MidiEvent::NoteOn {
        note: 6,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(10),
    });

    let processed = event_processor.process(MidiEvent::NoteOn {
        note: 10,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(20),
    });

    // Should detect a chord, but it won't match our configured chord
    let chord_event = processed
        .iter()
        .find(|e| matches!(e, conductor_core::event_processor::ProcessedEvent::ChordDetected { .. }));

    if let Some(chord_event) = chord_event {
        let action = mapping_engine.get_action_for_processed(chord_event, 0);
        assert!(action.is_none(), "Wrong notes should not match the configured chord");
    }
}

#[test]
fn test_chord_exact_match_required() {
    // Test that chord matching requires exact note set (no subset/superset matching)
    let config_toml = r#"
        [device]
        name = "Test Device"
        auto_connect = false

        [[modes]]
        name = "Test Mode"

        [[modes.mappings]]
        [modes.mappings.trigger]
        type = "NoteChord"
        notes = [1, 5]  # Only 2 notes

        [modes.mappings.action]
        type = "Shell"
        command = "echo 'two note chord'"
    "#;

    let config: Config = toml::from_str(config_toml).expect("Failed to parse config");

    let mut mapping_engine = MappingEngine::new();
    mapping_engine.load_from_config(&config);

    let mut event_processor = EventProcessor::new();

    // Press three notes (1, 5, 9) - superset of configured chord (1, 5)
    let base_time = Instant::now();

    event_processor.process(MidiEvent::NoteOn {
        note: 1,
        velocity: 100,
        time: base_time,
    });

    event_processor.process(MidiEvent::NoteOn {
        note: 5,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(10),
    });

    let processed = event_processor.process(MidiEvent::NoteOn {
        note: 9,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(20),
    });

    // Chord detected should have all 3 notes
    let chord_event = processed
        .iter()
        .find(|e| matches!(e, conductor_core::event_processor::ProcessedEvent::ChordDetected { .. }))
        .expect("Should detect a 3-note chord");

    // But it shouldn't match our 2-note configured chord
    let action = mapping_engine.get_action_for_processed(chord_event, 0);
    assert!(
        action.is_none(),
        "3-note chord should not match 2-note configured chord (exact match required)"
    );
}
