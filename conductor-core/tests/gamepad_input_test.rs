// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration test for gamepad InputEvent processing
//!
//! Tests that InputEvent (from gamepads) triggers correctly process through
//! the EventProcessor, generating the same ProcessedEvents as MIDI.

use conductor_core::events::InputEvent;
use conductor_core::event_processor::{EncoderDirection, EventProcessor, ProcessedEvent, VelocityLevel};
use std::time::Instant;

#[test]
fn test_gamepad_button_press_detection() {
    let mut processor = EventProcessor::new();

    // Gamepad button press (button ID 128 = South/A/Cross/B)
    let event = InputEvent::PadPressed {
        pad: 128,
        velocity: 100,
        time: Instant::now(),
    };

    let processed = processor.process_input(event);

    // Should detect PadPressed with velocity level
    assert_eq!(processed.len(), 1);

    match &processed[0] {
        ProcessedEvent::PadPressed {
            note,
            velocity,
            velocity_level,
        } => {
            assert_eq!(*note, 128);
            assert_eq!(*velocity, 100);
            assert_eq!(*velocity_level, VelocityLevel::Hard); // 100 is in Hard range (81-127)
        }
        _ => panic!("Expected PadPressed event"),
    }
}

#[test]
fn test_gamepad_button_release_short_press() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Press button
    let press_event = InputEvent::PadPressed {
        pad: 129, // East button (B/Circle/A)
        velocity: 80,
        time: base_time,
    };

    let _ = processor.process_input(press_event);

    // Release quickly (100ms later = short press)
    let release_event = InputEvent::PadReleased {
        pad: 129,
        time: base_time + std::time::Duration::from_millis(100),
    };

    let processed = processor.process_input(release_event);

    // Should detect PadReleased + ShortPress
    assert!(processed.len() >= 2);

    // Check for ShortPress
    let has_short_press = processed
        .iter()
        .any(|e| matches!(e, ProcessedEvent::ShortPress { note: 129 }));
    assert!(has_short_press, "Should detect short press");
}

#[test]
fn test_gamepad_button_long_press() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Press button
    let press_event = InputEvent::PadPressed {
        pad: 130, // West button (X/Square/Y)
        velocity: 100,
        time: base_time,
    };

    let _ = processor.process_input(press_event);

    // Release after 1.5 seconds (long press)
    let release_event = InputEvent::PadReleased {
        pad: 130,
        time: base_time + std::time::Duration::from_millis(1500),
    };

    let processed = processor.process_input(release_event);

    // Should detect PadReleased + LongPress
    assert!(processed.len() >= 2);

    // Check for LongPress
    let has_long_press = processed.iter().any(|e| {
        matches!(e, ProcessedEvent::LongPress {
            note: 130,
            duration_ms
        } if *duration_ms >= 1000)
    });
    assert!(has_long_press, "Should detect long press");
}

#[test]
fn test_gamepad_double_tap() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // First tap
    let press1 = InputEvent::PadPressed {
        pad: 131, // North button (Y/Triangle/X)
        velocity: 100,
        time: base_time,
    };
    let _ = processor.process_input(press1);

    let release1 = InputEvent::PadReleased {
        pad: 131,
        time: base_time + std::time::Duration::from_millis(50),
    };
    let _ = processor.process_input(release1);

    // Second tap within 300ms window
    let press2 = InputEvent::PadPressed {
        pad: 131,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(200),
    };
    let processed = processor.process_input(press2);

    // Should detect DoubleTap
    let has_double_tap = processed
        .iter()
        .any(|e| matches!(e, ProcessedEvent::DoubleTap { note: 131 }));
    assert!(has_double_tap, "Should detect double tap");
}

#[test]
fn test_gamepad_chord_detection() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Press multiple buttons within chord timeout (50ms)
    let button1 = InputEvent::PadPressed {
        pad: 132, // D-Pad Up
        velocity: 100,
        time: base_time,
    };
    let _ = processor.process_input(button1);

    let button2 = InputEvent::PadPressed {
        pad: 133, // D-Pad Down
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(20),
    };
    let _ = processor.process_input(button2);

    let button3 = InputEvent::PadPressed {
        pad: 134, // D-Pad Left
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(40),
    };
    let processed = processor.process_input(button3);

    // Should detect ChordDetected
    let chord_event = processed.iter().find(|e| {
        matches!(e, ProcessedEvent::ChordDetected { notes } if notes.len() == 3)
    });

    assert!(chord_event.is_some(), "Should detect 3-button chord");
}

#[test]
fn test_gamepad_analog_stick_movement() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Initial position (center = 64)
    let initial = InputEvent::EncoderTurned {
        encoder: 128, // Left stick X
        value: 64,
        time: base_time,
    };
    let _ = processor.process_input(initial);

    // Move right (value increases)
    let moved_right = InputEvent::EncoderTurned {
        encoder: 128,
        value: 95, // Normalized value for 0.5 on stick
        time: base_time + std::time::Duration::from_millis(10),
    };
    let processed = processor.process_input(moved_right);

    // Should detect EncoderTurned with direction
    assert_eq!(processed.len(), 1);

    match &processed[0] {
        ProcessedEvent::EncoderTurned {
            cc,
            value,
            direction,
            delta,
        } => {
            assert_eq!(*cc, 128);
            assert_eq!(*value, 95);
            assert_eq!(*direction, EncoderDirection::Clockwise); // Right = Clockwise
            assert_eq!(*delta, 31); // 95 - 64
        }
        _ => panic!("Expected EncoderTurned event"),
    }
}

#[test]
fn test_gamepad_trigger_analog() {
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Initial position (released = 0)
    let initial = InputEvent::EncoderTurned {
        encoder: 132, // Left trigger (L2/LT)
        value: 0,
        time: base_time,
    };
    let _ = processor.process_input(initial);

    // Pull trigger halfway (value 64)
    let pulled = InputEvent::EncoderTurned {
        encoder: 132,
        value: 64,
        time: base_time + std::time::Duration::from_millis(10),
    };
    let processed = processor.process_input(pulled);

    // Should detect EncoderTurned (trigger pull = clockwise)
    assert_eq!(processed.len(), 1);

    match &processed[0] {
        ProcessedEvent::EncoderTurned {
            cc,
            value,
            direction,
            delta,
        } => {
            assert_eq!(*cc, 132);
            assert_eq!(*value, 64);
            assert_eq!(*direction, EncoderDirection::Clockwise);
            assert_eq!(*delta, 64);
        }
        _ => panic!("Expected EncoderTurned for trigger pull"),
    }
}

#[test]
fn test_gamepad_velocity_levels() {
    let mut processor = EventProcessor::new();
    let time = Instant::now();

    // Test soft press (0-40)
    let soft = InputEvent::PadPressed {
        pad: 128,
        velocity: 30,
        time,
    };
    let processed = processor.process_input(soft);

    match &processed[0] {
        ProcessedEvent::PadPressed { velocity_level, .. } => {
            assert_eq!(*velocity_level, VelocityLevel::Soft);
        }
        _ => panic!("Expected PadPressed"),
    }

    // Test medium press (41-80)
    let medium = InputEvent::PadPressed {
        pad: 129,
        velocity: 60,
        time,
    };
    let processed = processor.process_input(medium);

    match &processed[0] {
        ProcessedEvent::PadPressed { velocity_level, .. } => {
            assert_eq!(*velocity_level, VelocityLevel::Medium);
        }
        _ => panic!("Expected PadPressed"),
    }

    // Test hard press (81-127)
    let hard = InputEvent::PadPressed {
        pad: 130,
        velocity: 120,
        time,
    };
    let processed = processor.process_input(hard);

    match &processed[0] {
        ProcessedEvent::PadPressed { velocity_level, .. } => {
            assert_eq!(*velocity_level, VelocityLevel::Hard);
        }
        _ => panic!("Expected PadPressed"),
    }
}

#[test]
fn test_gamepad_exact_chord_matching() {
    // Verify that chord matching requires exact note set
    let mut processor = EventProcessor::new();

    let base_time = Instant::now();

    // Press two buttons
    let button1 = InputEvent::PadPressed {
        pad: 128,
        velocity: 100,
        time: base_time,
    };
    let _ = processor.process_input(button1);

    let button2 = InputEvent::PadPressed {
        pad: 129,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(20),
    };
    let processed = processor.process_input(button2);

    // Should detect 2-button chord
    let chord = processed.iter().find_map(|e| {
        if let ProcessedEvent::ChordDetected { notes } = e {
            Some(notes)
        } else {
            None
        }
    });

    assert!(chord.is_some());
    assert_eq!(chord.unwrap().len(), 2);

    // Add third button - should create a NEW 3-button chord
    let button3 = InputEvent::PadPressed {
        pad: 130,
        velocity: 100,
        time: base_time + std::time::Duration::from_millis(40),
    };
    let processed = processor.process_input(button3);

    let chord = processed.iter().find_map(|e| {
        if let ProcessedEvent::ChordDetected { notes } = e {
            Some(notes)
        } else {
            None
        }
    });

    assert!(chord.is_some());
    assert_eq!(chord.unwrap().len(), 3);
}
