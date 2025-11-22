// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests using the MIDI simulator
//!
//! These tests verify the complete event processing pipeline from MIDI input
//! through event processing to action execution without requiring physical hardware.

mod midi_simulator;

use midi_simulator::{EncoderDirection, Gesture, MidiSimulator, ScenarioBuilder};
use std::time::{Duration, Instant};

// Helper to skip timing-sensitive tests on macOS CI
fn should_skip_timing_test() -> bool {
    std::env::var("CI").is_ok() && cfg!(target_os = "macos")
}

// Mock the event processor types for testing

#[derive(Debug, Clone, PartialEq)]
enum VelocityLevel {
    Soft,
    Medium,
    Hard,
}

/// Helper to parse MIDI events from simulator output
struct MidiEventParser {
    events: Vec<Vec<u8>>,
}

impl MidiEventParser {
    fn new(events: Vec<Vec<u8>>) -> Self {
        Self { events }
    }

    fn count_note_on(&self) -> usize {
        self.events.iter().filter(|e| e[0] & 0xF0 == 0x90).count()
    }

    fn count_note_off(&self) -> usize {
        self.events.iter().filter(|e| e[0] & 0xF0 == 0x80).count()
    }

    fn count_control_change(&self) -> usize {
        self.events.iter().filter(|e| e[0] & 0xF0 == 0xB0).count()
    }

    fn count_aftertouch(&self) -> usize {
        self.events.iter().filter(|e| e[0] & 0xF0 == 0xD0).count()
    }

    fn count_pitch_bend(&self) -> usize {
        self.events.iter().filter(|e| e[0] & 0xF0 == 0xE0).count()
    }

    fn get_velocities(&self) -> Vec<u8> {
        self.events
            .iter()
            .filter(|e| e[0] & 0xF0 == 0x90)
            .map(|e| e[2])
            .collect()
    }

    fn get_cc_values(&self, cc: u8) -> Vec<u8> {
        self.events
            .iter()
            .filter(|e| e[0] & 0xF0 == 0xB0 && e[1] == cc)
            .map(|e| e[2])
            .collect()
    }

    fn velocity_level(velocity: u8) -> VelocityLevel {
        match velocity {
            0..=40 => VelocityLevel::Soft,
            41..=80 => VelocityLevel::Medium,
            81..=127 => VelocityLevel::Hard,
            _ => VelocityLevel::Medium,
        }
    }
}

#[test]
fn test_basic_note_events() {
    let sim = MidiSimulator::new(0);

    // Simulate a simple note press
    sim.note_on(60, 100);
    sim.note_off(60);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_note_on(), 1);
    assert_eq!(parser.count_note_off(), 1);
}

#[test]
fn test_velocity_detection() {
    let sim = MidiSimulator::new(0);

    // Test all three velocity levels
    sim.note_on(60, 30); // Soft
    sim.note_off(60);

    sim.note_on(61, 70); // Medium
    sim.note_off(61);

    sim.note_on(62, 110); // Hard
    sim.note_off(62);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);
    let velocities = parser.get_velocities();

    assert_eq!(velocities.len(), 3);
    assert_eq!(
        MidiEventParser::velocity_level(velocities[0]),
        VelocityLevel::Soft
    );
    assert_eq!(
        MidiEventParser::velocity_level(velocities[1]),
        VelocityLevel::Medium
    );
    assert_eq!(
        MidiEventParser::velocity_level(velocities[2]),
        VelocityLevel::Hard
    );
}

#[test]
fn test_long_press_simulation() {
    let sim = MidiSimulator::new(0);
    let start = Instant::now();

    sim.perform_gesture(Gesture::LongPress {
        note: 60,
        velocity: 80,
        hold_ms: 2500,
    });

    let duration = start.elapsed();
    assert!(duration >= Duration::from_millis(2500));
    assert!(duration < Duration::from_millis(2700)); // Allow 200ms tolerance

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_note_on(), 1);
    assert_eq!(parser.count_note_off(), 1);
}

#[test]
fn test_double_tap_timing() {
    if should_skip_timing_test() {
        eprintln!("Skipping test_double_tap_timing on macOS CI due to runner timing variance");
        return;
    }

    let sim = MidiSimulator::new(0);
    let start = Instant::now();

    sim.perform_gesture(Gesture::DoubleTap {
        note: 60,
        velocity: 80,
        tap_duration_ms: 50,
        gap_ms: 200,
    });

    let duration = start.elapsed();

    // Total time should be: tap1 (50ms) + gap (200ms) + tap2 (50ms) = 300ms
    assert!(duration >= Duration::from_millis(300));
    assert!(duration < Duration::from_millis(400));

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    // Should have 2 note ons and 2 note offs
    assert_eq!(parser.count_note_on(), 2);
    assert_eq!(parser.count_note_off(), 2);
}

#[test]
fn test_chord_detection() {
    let sim = MidiSimulator::new(0);

    // Simulate a C major chord
    sim.perform_gesture(Gesture::Chord {
        notes: vec![60, 64, 67],
        velocity: 80,
        stagger_ms: 10,
        hold_ms: 500,
    });

    let events = sim.get_events();
    let parser = MidiEventParser::new(events.clone());

    // Should have 3 note ons and 3 note offs
    assert_eq!(parser.count_note_on(), 3);
    assert_eq!(parser.count_note_off(), 3);

    // Verify all notes are present
    let note_numbers: Vec<u8> = events
        .iter()
        .filter(|e| e[0] & 0xF0 == 0x90)
        .map(|e| e[1])
        .collect();

    assert!(note_numbers.contains(&60));
    assert!(note_numbers.contains(&64));
    assert!(note_numbers.contains(&67));
}

#[test]
fn test_encoder_direction_clockwise() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::EncoderTurn {
        cc: 1,
        direction: EncoderDirection::Clockwise,
        steps: 5,
        step_delay_ms: 0,
    });

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);
    let values = parser.get_cc_values(1);

    assert_eq!(values.len(), 5);

    // Values should be increasing
    for i in 1..values.len() {
        assert!(values[i] > values[i - 1], "CW values should increase");
    }
}

#[test]
fn test_encoder_direction_counterclockwise() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::EncoderTurn {
        cc: 1,
        direction: EncoderDirection::CounterClockwise,
        steps: 5,
        step_delay_ms: 0,
    });

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);
    let values = parser.get_cc_values(1);

    assert_eq!(values.len(), 5);

    // Values should be decreasing
    for i in 1..values.len() {
        assert!(values[i] < values[i - 1], "CCW values should decrease");
    }
}

#[test]
fn test_aftertouch_simulation() {
    let sim = MidiSimulator::new(0);

    sim.aftertouch(50);
    sim.aftertouch(75);
    sim.aftertouch(100);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events.clone());

    assert_eq!(parser.count_aftertouch(), 3);

    // Verify pressure values
    let pressures: Vec<u8> = events
        .iter()
        .filter(|e| e[0] & 0xF0 == 0xD0)
        .map(|e| e[1])
        .collect();

    assert_eq!(pressures, vec![50, 75, 100]);
}

#[test]
fn test_pitch_bend_simulation() {
    let sim = MidiSimulator::new(0);

    // Test center position
    sim.pitch_bend(8192);

    // Test max up
    sim.pitch_bend(16383);

    // Test min down
    sim.pitch_bend(0);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_pitch_bend(), 3);
}

#[test]
fn test_scenario_builder() {
    let sim = MidiSimulator::new(0);

    let scenario = ScenarioBuilder::new()
        .note_on(60, 100)
        .wait(100)
        .control_change(1, 64)
        .wait(100)
        .note_off(60)
        .build();

    let start = Instant::now();
    sim.execute_sequence(scenario);
    let duration = start.elapsed();

    // Should take at least 200ms (two waits)
    assert!(duration >= Duration::from_millis(200));

    let events = sim.get_events();
    assert_eq!(events.len(), 3); // Note on, CC, Note off
}

#[test]
fn test_velocity_ramp() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::VelocityRamp {
        note: 60,
        min_velocity: 20,
        max_velocity: 120,
        steps: 5,
    });

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);
    let velocities = parser.get_velocities();

    assert_eq!(velocities.len(), 5);

    // Verify velocities are increasing
    for i in 1..velocities.len() {
        assert!(
            velocities[i] >= velocities[i - 1],
            "Velocities should increase in ramp"
        );
    }
}

#[test]
fn test_multiple_channels() {
    let sim0 = MidiSimulator::new(0);
    let sim15 = MidiSimulator::new(15);

    sim0.note_on(60, 100);
    sim15.note_on(60, 100);

    let events0 = sim0.get_events();
    let events15 = sim15.get_events();

    // Verify channel in status byte
    assert_eq!(events0[0][0], 0x90); // Channel 0
    assert_eq!(events15[0][0], 0x9F); // Channel 15
}

#[test]
fn test_complex_sequence() {
    let sim = MidiSimulator::new(0);

    // Simulate a complex user interaction
    let scenario = ScenarioBuilder::new()
        // Press chord
        .note_on(60, 80)
        .note_on(64, 80)
        .note_on(67, 80)
        .wait(100)
        // Add aftertouch
        .aftertouch(100)
        .wait(100)
        // Encoder turn while holding chord
        .control_change(1, 65)
        .control_change(1, 66)
        .control_change(1, 67)
        .wait(100)
        // Release chord
        .note_off(60)
        .note_off(64)
        .note_off(67)
        .build();

    sim.execute_sequence(scenario);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_note_on(), 3);
    assert_eq!(parser.count_note_off(), 3);
    assert_eq!(parser.count_aftertouch(), 1);
    assert_eq!(parser.count_control_change(), 3);
}

#[test]
fn test_rapid_note_events() {
    let sim = MidiSimulator::new(0);

    // Simulate rapid pad hits (common in drum patterns)
    for i in 0..10 {
        sim.note_on(60 + i, 100);
        sim.note_off(60 + i);
    }

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_note_on(), 10);
    assert_eq!(parser.count_note_off(), 10);
}

#[test]
fn test_sustained_notes() {
    let sim = MidiSimulator::new(0);

    // Press multiple notes and hold them
    sim.note_on(60, 80);
    sim.note_on(64, 80);
    sim.note_on(67, 80);

    // Verify all notes are held (no note offs yet)
    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    assert_eq!(parser.count_note_on(), 3);
    assert_eq!(parser.count_note_off(), 0);

    // Now release
    sim.note_off(60);
    sim.note_off(64);
    sim.note_off(67);

    let events = sim.get_events();
    let parser = MidiEventParser::new(events);

    // After getting new events, we only have the note offs
    assert_eq!(parser.count_note_on(), 0);
    assert_eq!(parser.count_note_off(), 3);
}

#[test]
fn test_event_queue_operations() {
    let sim = MidiSimulator::new(0);

    // Add some events
    sim.note_on(60, 100);
    sim.note_off(60);

    // Peek shouldn't clear
    let peeked = sim.peek_last_event();
    assert!(peeked.is_some());

    let events = sim.get_events();
    assert_eq!(events.len(), 2);

    // Queue should be empty after get_events
    let events2 = sim.get_events();
    assert_eq!(events2.len(), 0);

    // Add more and clear
    sim.note_on(61, 100);
    sim.clear_events();
    let events3 = sim.get_events();
    assert_eq!(events3.len(), 0);
}

#[test]
fn test_midi_message_format() {
    let sim = MidiSimulator::new(0);

    // Note On
    sim.note_on(60, 100);
    let events = sim.get_events();
    assert_eq!(events[0].len(), 3);
    assert_eq!(events[0][0] & 0xF0, 0x90); // Status
    assert_eq!(events[0][1], 60); // Note
    assert_eq!(events[0][2], 100); // Velocity

    sim.clear_events();

    // Control Change
    sim.control_change(1, 64);
    let events = sim.get_events();
    assert_eq!(events[0].len(), 3);
    assert_eq!(events[0][0] & 0xF0, 0xB0); // Status
    assert_eq!(events[0][1], 1); // CC number
    assert_eq!(events[0][2], 64); // Value

    sim.clear_events();

    // Aftertouch (2 bytes)
    sim.aftertouch(80);
    let events = sim.get_events();
    assert_eq!(events[0].len(), 2);
    assert_eq!(events[0][0] & 0xF0, 0xD0); // Status
    assert_eq!(events[0][1], 80); // Pressure

    sim.clear_events();

    // Pitch Bend (3 bytes with 14-bit value)
    sim.pitch_bend(8192);
    let events = sim.get_events();
    assert_eq!(events[0].len(), 3);
    assert_eq!(events[0][0] & 0xF0, 0xE0); // Status
}
