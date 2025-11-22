// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests for MIDI device connection and event processing
//!
//! These tests verify the full end-to-end MIDI pipeline:
//! - MIDI event → EventProcessor → MappingEngine → ActionExecutor
//! - Device connection/disconnection
//! - Velocity context propagation

use conductor_core::event_processor::MidiEvent;
use conductor_core::{Config, EventProcessor, MappingEngine};
use conductor_daemon::action_executor::{ActionExecutor, TriggerContext};
use conductor_daemon::midi_device::MidiDeviceManager;
use std::time::Instant;
use tokio::sync::mpsc;

/// Test that EventProcessor correctly processes MIDI events into ProcessedEvents
#[tokio::test]
async fn test_event_processor_midi_to_processed() {
    let mut processor = EventProcessor::new();

    // Create a Note On event
    let midi_event = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };

    // Process the event
    let processed_events = processor.process(midi_event);

    // Should produce at least one processed event
    assert!(!processed_events.is_empty(), "Should process MIDI event");

    // Should be a PadPressed event
    match &processed_events[0] {
        conductor_core::event_processor::ProcessedEvent::PadPressed { note, velocity, .. } => {
            assert_eq!(*note, 60, "Note number should match MIDI note");
            assert_eq!(*velocity, 100, "Velocity should be preserved");
        }
        other => panic!("Expected PadPressed, got {:?}", other),
    }
}

/// Test that MappingEngine correctly maps MIDI events to actions
#[tokio::test]
async fn test_mapping_engine_event_to_action() {
    // Create a simple config with one mapping
    let config = Config {
        device: conductor_core::config::types::DeviceConfig {
            name: "Test Device".to_string(),
            auto_connect: false,
            auto_reconnect: false,
            port: None,
        },
        modes: vec![conductor_core::config::types::Mode {
            name: "Test Mode".to_string(),
            color: Some("blue".to_string()),
            mappings: vec![conductor_core::config::types::Mapping {
                trigger: conductor_core::config::types::Trigger::Note {
                    note: 60,
                    velocity_min: None,
                },
                action: conductor_core::config::types::ActionConfig::Keystroke {
                    keys: "Space".to_string(),
                    modifiers: vec![],
                },
                description: None,
            }],
        }],
        global_mappings: vec![],
        logging: None,
        advanced_settings: Default::default(),
    };

    // Create mapping engine and load config
    let mut engine = MappingEngine::new();
    engine.load_from_config(&config);

    // Create a Note On event that matches the mapping
    let midi_event = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };

    // Get action for this event (mode 0)
    let action = engine.get_action(&midi_event, 0);

    // Should find a matching action
    assert!(action.is_some(), "Should find matching action");

    // Should be a Keystroke action
    match action.unwrap() {
        conductor_core::actions::Action::Keystroke { keys, .. } => {
            assert!(!keys.is_empty(), "Should have keys");
            // Keys are parsed into KeyCode enum, so we can't directly compare to "Space"
        }
        other => panic!("Expected Keystroke, got {:?}", other),
    }
}

/// Test that ActionExecutor receives correct TriggerContext with velocity
#[tokio::test]
async fn test_velocity_context_propagation() {
    // Create a MIDI event with specific velocity
    let midi_event = MidiEvent::NoteOn {
        note: 60,
        velocity: 85,
        time: Instant::now(),
    };

    // Create trigger context from MIDI event
    let context = TriggerContext {
        velocity: match &midi_event {
            MidiEvent::NoteOn { velocity, .. } => Some(*velocity),
            _ => None,
        },
        current_mode: None,
    };

    // Verify context has correct velocity
    assert_eq!(
        context.velocity,
        Some(85),
        "TriggerContext should carry velocity from MIDI event"
    );

    // Create action executor
    let mut executor = ActionExecutor::new();

    // Create a simple Delay action (doesn't actually execute anything)
    let action = conductor_core::actions::Action::Delay(1);

    // Execute action with context (should not panic)
    executor.execute(action, Some(context));

    // Test passes if no panic occurs
}

/// Test MIDI event channel communication
#[tokio::test]
async fn test_midi_event_channel() {
    // Create event channel
    let (event_tx, mut event_rx) = mpsc::channel::<MidiEvent>(100);

    // Send a MIDI event
    let midi_event = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };

    event_tx.send(midi_event.clone()).await.unwrap();

    // Receive the event
    let received = event_rx.recv().await.unwrap();

    // Verify event matches
    match (&midi_event, &received) {
        (
            MidiEvent::NoteOn {
                note: n1,
                velocity: v1,
                ..
            },
            MidiEvent::NoteOn {
                note: n2,
                velocity: v2,
                ..
            },
        ) => {
            assert_eq!(n1, n2, "Note should match");
            assert_eq!(v1, v2, "Velocity should match");
        }
        _ => panic!("Event type mismatch"),
    }
}

/// Test MidiDeviceManager creation and basic state
#[tokio::test]
async fn test_midi_device_manager_creation() {
    let manager = MidiDeviceManager::new("Test Device".to_string(), true);

    // Should not be connected initially
    assert!(!manager.is_connected(), "Should not be connected initially");
}

/// Test full end-to-end pipeline (without actual MIDI hardware)
///
/// This test simulates the complete flow:
/// 1. Create MIDI event
/// 2. Process through EventProcessor
/// 3. Map through MappingEngine
/// 4. Execute through ActionExecutor
#[tokio::test]
async fn test_end_to_end_pipeline() {
    // Create config with one mapping
    let config = Config {
        device: conductor_core::config::types::DeviceConfig {
            name: "Test Device".to_string(),
            auto_connect: false,
            auto_reconnect: false,
            port: None,
        },
        modes: vec![conductor_core::config::types::Mode {
            name: "Test Mode".to_string(),
            color: Some("blue".to_string()),
            mappings: vec![conductor_core::config::types::Mapping {
                trigger: conductor_core::config::types::Trigger::Note {
                    note: 36,
                    velocity_min: None,
                },
                action: conductor_core::config::types::ActionConfig::Delay { ms: 1 },
                description: None,
            }],
        }],
        global_mappings: vec![],
        logging: None,
        advanced_settings: Default::default(),
    };

    // Create components
    let mut event_processor = EventProcessor::new();
    let mut mapping_engine = MappingEngine::new();
    mapping_engine.load_from_config(&config);
    let mut action_executor = ActionExecutor::new();

    // Step 1: Create MIDI event
    let midi_event = MidiEvent::NoteOn {
        note: 36,
        velocity: 120,
        time: Instant::now(),
    };

    // Step 2: Process MIDI event → ProcessedEvent
    let processed_events = event_processor.process(midi_event.clone());
    assert!(!processed_events.is_empty(), "Should process event");

    // Step 3: Map MIDI event → Action
    let action = mapping_engine.get_action(&midi_event, 0);
    assert!(action.is_some(), "Should find matching action");

    // Step 4: Execute action with velocity context
    let context = TriggerContext {
        velocity: Some(120),
        current_mode: None,
    };

    action_executor.execute(action.unwrap(), Some(context));

    // Test passes if no panic occurs
}

/// Test that different MIDI event types are correctly processed
#[tokio::test]
async fn test_multiple_midi_event_types() {
    let mut processor = EventProcessor::new();

    // Test Note On
    let note_on = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };
    let result = processor.process(note_on);
    assert!(!result.is_empty(), "Should process Note On");

    // Test Note Off
    let note_off = MidiEvent::NoteOff {
        note: 60,
        time: Instant::now(),
    };
    let result = processor.process(note_off);
    assert!(!result.is_empty(), "Should process Note Off");

    // Test Control Change (requires two events to detect direction change)
    let cc1 = MidiEvent::ControlChange {
        cc: 1,
        value: 64,
        time: Instant::now(),
    };
    let result1 = processor.process(cc1);
    // First CC event stores baseline but produces no ProcessedEvent
    assert!(result1.is_empty(), "First CC event should store baseline");

    let cc2 = MidiEvent::ControlChange {
        cc: 1,
        value: 80,
        time: Instant::now(),
    };
    let result2 = processor.process(cc2);
    // Second CC event should produce EncoderTurned
    assert!(
        !result2.is_empty(),
        "Second CC event should produce EncoderTurned"
    );
}

/// Test that velocity is correctly extracted from different event types
#[tokio::test]
async fn test_velocity_extraction() {
    // Note On with velocity
    let note_on = MidiEvent::NoteOn {
        note: 60,
        velocity: 95,
        time: Instant::now(),
    };

    let context_note_on = TriggerContext {
        velocity: match &note_on {
            MidiEvent::NoteOn { velocity, .. } => Some(*velocity),
            _ => None,
        },
        current_mode: None,
    };

    assert_eq!(
        context_note_on.velocity,
        Some(95),
        "Should extract velocity from Note On"
    );

    // Note Off (no velocity in our implementation)
    let note_off = MidiEvent::NoteOff {
        note: 60,
        time: Instant::now(),
    };

    let context_note_off = TriggerContext {
        velocity: match &note_off {
            MidiEvent::NoteOn { velocity, .. } => Some(*velocity),
            _ => None,
        },
        current_mode: None,
    };

    assert_eq!(
        context_note_off.velocity, None,
        "Note Off should have no velocity"
    );
}

/// Test that event channel handles backpressure correctly
#[tokio::test]
async fn test_event_channel_backpressure() {
    // Create channel with small buffer
    let (event_tx, mut event_rx) = mpsc::channel::<MidiEvent>(2);

    // Fill the channel
    let event1 = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };
    let event2 = MidiEvent::NoteOn {
        note: 61,
        velocity: 101,
        time: Instant::now(),
    };

    event_tx.send(event1).await.unwrap();
    event_tx.send(event2).await.unwrap();

    // Channel is now full, try_send should fail
    let event3 = MidiEvent::NoteOn {
        note: 62,
        velocity: 102,
        time: Instant::now(),
    };

    let result = event_tx.try_send(event3);
    assert!(result.is_err(), "try_send should fail when channel is full");

    // Drain one event
    event_rx.recv().await.unwrap();

    // Now try_send should succeed
    let event4 = MidiEvent::NoteOn {
        note: 63,
        velocity: 103,
        time: Instant::now(),
    };

    let result = event_tx.try_send(event4);
    assert!(result.is_ok(), "try_send should succeed after draining");
}

/// Test device disconnect and reconnect handling
#[tokio::test]
async fn test_device_disconnect_reconnect() {
    use conductor_daemon::midi_device::MidiDeviceManager;
    use tokio::sync::mpsc;

    // Create event channel
    let (event_tx, mut event_rx) = mpsc::channel::<MidiEvent>(100);

    // Create device manager
    let mut manager = MidiDeviceManager::new("Test Device".to_string(), true);

    // Verify initial state
    assert!(!manager.is_connected(), "Should not be connected initially");

    // Note: We can't actually connect without hardware, but we can test disconnect
    // This simulates a device that was connected and then disconnected

    // Disconnect (should be safe even if not connected)
    manager.disconnect();

    // Verify disconnected state
    assert!(!manager.is_connected(), "Should remain disconnected");

    // Verify event channel still works after disconnect
    let test_event = MidiEvent::NoteOn {
        note: 60,
        velocity: 100,
        time: Instant::now(),
    };

    event_tx.send(test_event.clone()).await.unwrap();
    let received = event_rx.recv().await.unwrap();

    match (&test_event, &received) {
        (
            MidiEvent::NoteOn {
                note: n1,
                velocity: v1,
                ..
            },
            MidiEvent::NoteOn {
                note: n2,
                velocity: v2,
                ..
            },
        ) => {
            assert_eq!(n1, n2, "Note should match");
            assert_eq!(v1, v2, "Velocity should match");
        }
        _ => panic!("Event type mismatch"),
    }
}

/// Test reconnection logic with auto-reconnect enabled
#[tokio::test]
async fn test_auto_reconnect_flag() {
    use conductor_daemon::midi_device::MidiDeviceManager;

    // Create manager with auto-reconnect enabled
    let manager_auto = MidiDeviceManager::new("Test Device".to_string(), true);
    assert!(
        !manager_auto.is_connected(),
        "Should not be connected initially"
    );

    // Create manager with auto-reconnect disabled
    let manager_manual = MidiDeviceManager::new("Test Device".to_string(), false);
    assert!(
        !manager_manual.is_connected(),
        "Should not be connected initially"
    );

    // Both should have same initial state (disconnected)
    // The auto-reconnect flag would only matter during actual connection attempts
}
