// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests for event processing (AMI-117)
//!
//! Tests for Aftertouch (F7) and PitchBend (F8) event processing

mod midi_simulator;

use midi_simulator::MidiSimulator;
use std::time::Duration;

/// Test aftertouch pressure message generation and detection
#[test]
fn test_aftertouch_pressure_range() {
    let sim = MidiSimulator::new(0);

    // Test full range of aftertouch pressure values (0-127)
    let test_pressures = vec![0, 32, 64, 96, 127];

    for &pressure in &test_pressures {
        sim.aftertouch(pressure);
    }

    let events = sim.get_events();

    // Verify all events are aftertouch messages
    for (i, event) in events.iter().enumerate() {
        assert_eq!(event.len(), 2, "Aftertouch messages should have 2 bytes");
        assert_eq!(
            event[0] & 0xF0,
            0xD0,
            "First byte should be aftertouch status (0xD0)"
        );
        assert_eq!(event[1], test_pressures[i], "Pressure value should match");
    }
}

#[test]
fn test_aftertouch_continuous_variation() {
    let sim = MidiSimulator::new(0);

    // Simulate continuous pressure increase (like gradually pressing harder)
    for pressure in (10..=100).step_by(10) {
        sim.aftertouch(pressure);
    }

    let events = sim.get_events();
    assert_eq!(events.len(), 10, "Should have 10 aftertouch events");

    // Extract pressure values
    let pressures: Vec<u8> = events.iter().map(|e| e[1]).collect();

    // Verify increasing pressure
    for i in 1..pressures.len() {
        assert!(
            pressures[i] > pressures[i - 1],
            "Pressure should increase: {} > {}",
            pressures[i],
            pressures[i - 1]
        );
    }
}

#[test]
fn test_aftertouch_boundary_values() {
    let sim = MidiSimulator::new(0);

    // Test boundary values
    sim.aftertouch(0); // Minimum
    sim.aftertouch(127); // Maximum

    let events = sim.get_events();
    assert_eq!(events.len(), 2);

    assert_eq!(events[0][1], 0, "Minimum pressure should be 0");
    assert_eq!(events[1][1], 127, "Maximum pressure should be 127");
}

#[test]
fn test_aftertouch_with_note_press() {
    let sim = MidiSimulator::new(0);

    // Simulate a realistic scenario: press note, apply pressure, release
    sim.note_on(60, 80);
    std::thread::sleep(Duration::from_millis(10));

    sim.aftertouch(50);
    std::thread::sleep(Duration::from_millis(10));

    sim.aftertouch(80);
    std::thread::sleep(Duration::from_millis(10));

    sim.aftertouch(100);
    std::thread::sleep(Duration::from_millis(10));

    sim.note_off(60);

    let events = sim.get_events();

    // Should have: 1 note on, 3 aftertouch, 1 note off
    let note_on_count = events.iter().filter(|e| e[0] & 0xF0 == 0x90).count();
    let aftertouch_count = events.iter().filter(|e| e[0] & 0xF0 == 0xD0).count();
    let note_off_count = events.iter().filter(|e| e[0] & 0xF0 == 0x80).count();

    assert_eq!(note_on_count, 1);
    assert_eq!(aftertouch_count, 3);
    assert_eq!(note_off_count, 1);
}

#[test]
fn test_pitch_bend_center_position() {
    let sim = MidiSimulator::new(0);

    // Center position (no bend) is 8192 (0x2000)
    sim.pitch_bend(8192);

    let events = sim.get_events();
    assert_eq!(events.len(), 1);

    let event = &events[0];
    assert_eq!(event.len(), 3, "Pitch bend messages should have 3 bytes");
    assert_eq!(event[0] & 0xF0, 0xE0, "Should be pitch bend status");

    // Verify 14-bit value encoding (LSB, MSB)
    let lsb = event[1];
    let msb = event[2];
    let reconstructed_value = ((msb as u16) << 7) | (lsb as u16);

    assert_eq!(reconstructed_value, 8192, "Center position should be 8192");
}

#[test]
fn test_pitch_bend_full_range() {
    let sim = MidiSimulator::new(0);

    // Test full 14-bit range: 0 (min down) to 16383 (max up)
    let test_values = vec![
        0,     // Maximum bend down
        4096,  // Quarter way up
        8192,  // Center (no bend)
        12288, // Quarter way up
        16383, // Maximum bend up
    ];

    for &value in &test_values {
        sim.pitch_bend(value);
    }

    let events = sim.get_events();
    assert_eq!(events.len(), 5);

    for (i, event) in events.iter().enumerate() {
        assert_eq!(event[0] & 0xF0, 0xE0, "Should be pitch bend status");

        // Reconstruct 14-bit value
        let lsb = event[1];
        let msb = event[2];
        let reconstructed_value = ((msb as u16) << 7) | (lsb as u16);

        assert_eq!(
            reconstructed_value, test_values[i],
            "Pitch bend value should match"
        );
    }
}

#[test]
fn test_pitch_bend_positive_range() {
    let sim = MidiSimulator::new(0);

    // Test positive bend (above center)
    for value in (8192..=16383).step_by(2048) {
        sim.pitch_bend(value);
    }

    let events = sim.get_events();

    // Verify all values are >= center position
    for event in &events {
        let lsb = event[1];
        let msb = event[2];
        let value = ((msb as u16) << 7) | (lsb as u16);
        assert!(value >= 8192, "Positive bend should be >= 8192");
    }
}

#[test]
fn test_pitch_bend_negative_range() {
    let sim = MidiSimulator::new(0);

    // Test negative bend (below center)
    for value in (0..=8192).step_by(2048) {
        sim.pitch_bend(value);
    }

    let events = sim.get_events();

    // Verify all values are <= center position
    for event in &events {
        let lsb = event[1];
        let msb = event[2];
        let value = ((msb as u16) << 7) | (lsb as u16);
        assert!(value <= 8192, "Negative bend should be <= 8192");
    }
}

#[test]
fn test_pitch_bend_smooth_sweep() {
    let sim = MidiSimulator::new(0);

    // Simulate smooth pitch bend sweep from min to max
    let start_value = 4096_u16;
    let end_value = 12288_u16;
    let steps = 10;
    let step_size = (end_value - start_value) / steps;

    for i in 0..=steps {
        let value = start_value + (i * step_size);
        sim.pitch_bend(value);
    }

    let events = sim.get_events();
    assert_eq!(events.len(), (steps + 1) as usize);

    // Verify smooth progression
    let mut prev_value = 0_u16;
    for event in &events {
        let lsb = event[1];
        let msb = event[2];
        let value = ((msb as u16) << 7) | (lsb as u16);

        if prev_value > 0 {
            assert!(value > prev_value, "Pitch bend should increase smoothly");
        }
        prev_value = value;
    }
}

#[test]
fn test_pitch_bend_with_notes() {
    let sim = MidiSimulator::new(0);

    // Realistic scenario: note on, bend up, bend down, note off
    sim.note_on(60, 80);
    sim.pitch_bend(8192); // Center
    sim.pitch_bend(10240); // Bend up
    sim.pitch_bend(12288); // More up
    sim.pitch_bend(8192); // Back to center
    sim.pitch_bend(6144); // Bend down
    sim.pitch_bend(8192); // Back to center
    sim.note_off(60);

    let events = sim.get_events();

    let note_on_count = events.iter().filter(|e| e[0] & 0xF0 == 0x90).count();
    let pitch_bend_count = events.iter().filter(|e| e[0] & 0xF0 == 0xE0).count();
    let note_off_count = events.iter().filter(|e| e[0] & 0xF0 == 0x80).count();

    assert_eq!(note_on_count, 1);
    assert_eq!(pitch_bend_count, 6);
    assert_eq!(note_off_count, 1);
}

#[test]
fn test_continuous_message_throttling_simulation() {
    let sim = MidiSimulator::new(0);

    // Simulate high-frequency continuous controller data
    // (like a real touch strip or pressure sensor)
    for i in 0..100 {
        let value = 8192 + (i * 50); // Gradual increase
        sim.pitch_bend(value.min(16383));

        // Small delay to simulate real-time streaming
        if i % 10 == 0 {
            std::thread::sleep(Duration::from_micros(100));
        }
    }

    let events = sim.get_events();
    assert_eq!(events.len(), 100, "Should capture all continuous messages");

    // Verify message format consistency
    for event in &events {
        assert_eq!(event[0] & 0xF0, 0xE0);
        assert!(event[1] <= 127, "LSB should be 7-bit");
        assert!(event[2] <= 127, "MSB should be 7-bit");
    }
}

#[test]
fn test_aftertouch_and_pitch_bend_interleaved() {
    let sim = MidiSimulator::new(0);

    // Test interleaved aftertouch and pitch bend
    sim.note_on(60, 80);

    sim.aftertouch(50);
    sim.pitch_bend(9000);

    sim.aftertouch(70);
    sim.pitch_bend(10000);

    sim.aftertouch(90);
    sim.pitch_bend(11000);

    sim.note_off(60);

    let events = sim.get_events();

    // Count each message type
    let aftertouch_count = events.iter().filter(|e| e[0] & 0xF0 == 0xD0).count();
    let pitch_bend_count = events.iter().filter(|e| e[0] & 0xF0 == 0xE0).count();

    assert_eq!(aftertouch_count, 3, "Should have 3 aftertouch messages");
    assert_eq!(pitch_bend_count, 3, "Should have 3 pitch bend messages");

    // Verify interleaving order
    let types: Vec<u8> = events.iter().map(|e| e[0] & 0xF0).collect();

    // Pattern should be: Note On (0x90), then alternating Aftertouch (0xD0) and Pitch Bend (0xE0)
    assert_eq!(types[0], 0x90, "First should be note on");
    assert_eq!(types[types.len() - 1], 0x80, "Last should be note off");
}

#[test]
fn test_multiple_channels_aftertouch() {
    let sim_ch0 = MidiSimulator::new(0);
    let sim_ch1 = MidiSimulator::new(1);

    sim_ch0.aftertouch(64);
    sim_ch1.aftertouch(64);

    let events_ch0 = sim_ch0.get_events();
    let events_ch1 = sim_ch1.get_events();

    // Verify channel encoding in status byte
    assert_eq!(events_ch0[0][0], 0xD0, "Channel 0 aftertouch");
    assert_eq!(events_ch1[0][0], 0xD1, "Channel 1 aftertouch");
}

#[test]
fn test_multiple_channels_pitch_bend() {
    let sim_ch0 = MidiSimulator::new(0);
    let sim_ch1 = MidiSimulator::new(1);

    sim_ch0.pitch_bend(8192);
    sim_ch1.pitch_bend(8192);

    let events_ch0 = sim_ch0.get_events();
    let events_ch1 = sim_ch1.get_events();

    // Verify channel encoding in status byte
    assert_eq!(events_ch0[0][0], 0xE0, "Channel 0 pitch bend");
    assert_eq!(events_ch1[0][0], 0xE1, "Channel 1 pitch bend");
}
