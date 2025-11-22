// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDI Device Simulator for Hardware-Independent Testing
//!
//! This module provides a comprehensive MIDI device simulator that allows testing
//! MIDIMon without requiring physical hardware. It supports all MIDI event types
//! including timing-sensitive operations like long press, double-tap, and chords.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Represents a simulated MIDI event with precise timing control
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SimulatorEvent {
    /// Note On with note number, velocity, and optional delay before sending
    NoteOn {
        note: u8,
        velocity: u8,
        delay_ms: u64,
    },
    /// Note Off with note number and optional delay before sending
    NoteOff { note: u8, delay_ms: u64 },
    /// Control Change with CC number, value, and optional delay
    ControlChange { cc: u8, value: u8, delay_ms: u64 },
    /// Aftertouch (channel pressure) with pressure value and optional delay
    Aftertouch { pressure: u8, delay_ms: u64 },
    /// Pitch Bend with 14-bit value (0-16383, center=8192) and optional delay
    PitchBend { value: u16, delay_ms: u64 },
    /// Program Change with program number and optional delay
    ProgramChange { program: u8, delay_ms: u64 },
    /// Wait for a specified duration (for scripted sequences)
    Wait { duration_ms: u64 },
}

/// High-level gestures that simulate complex user interactions
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Gesture {
    /// Simple note press and release with configurable velocity and duration
    SimpleTap {
        note: u8,
        velocity: u8,
        duration_ms: u64,
    },
    /// Double-tap gesture with configurable timing between taps
    DoubleTap {
        note: u8,
        velocity: u8,
        tap_duration_ms: u64,
        gap_ms: u64,
    },
    /// Long press gesture with configurable hold duration
    LongPress {
        note: u8,
        velocity: u8,
        hold_ms: u64,
    },
    /// Chord gesture with multiple notes pressed simultaneously
    Chord {
        notes: Vec<u8>,
        velocity: u8,
        stagger_ms: u64,
        hold_ms: u64,
    },
    /// Encoder rotation simulation using CC messages
    EncoderTurn {
        cc: u8,
        direction: EncoderDirection,
        steps: u8,
        step_delay_ms: u64,
    },
    /// Velocity ramp from soft to hard
    VelocityRamp {
        note: u8,
        min_velocity: u8,
        max_velocity: u8,
        steps: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum EncoderDirection {
    Clockwise,
    CounterClockwise,
}

/// MIDI Device Simulator
///
/// Provides methods to simulate all MIDI events and complex gestures for testing
pub struct MidiSimulator {
    /// Event queue for captured events
    events: Arc<Mutex<Vec<Vec<u8>>>>,
    /// Channel number to use (0-15)
    channel: u8,
    /// Whether to print debug output
    debug: bool,
}

#[allow(dead_code)]
impl MidiSimulator {
    /// Create a new MIDI simulator on the specified channel (0-15)
    pub fn new(channel: u8) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            channel: channel & 0x0F, // Ensure channel is 0-15
            debug: false,
        }
    }

    /// Enable or disable debug output
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Get all captured events and clear the queue
    pub fn get_events(&self) -> Vec<Vec<u8>> {
        let mut events = self.events.lock().unwrap();
        let captured = events.clone();
        events.clear();
        captured
    }

    /// Get the most recent event without clearing the queue
    pub fn peek_last_event(&self) -> Option<Vec<u8>> {
        self.events.lock().unwrap().last().cloned()
    }

    /// Clear all captured events
    pub fn clear_events(&self) {
        self.events.lock().unwrap().clear();
    }

    /// Send a raw MIDI message
    fn send_raw(&self, message: Vec<u8>) {
        if self.debug {
            println!("[SIM] Sending: {:02X?}", message);
        }
        self.events.lock().unwrap().push(message);
    }

    /// Send a Note On message
    pub fn note_on(&self, note: u8, velocity: u8) {
        let status = 0x90 | self.channel;
        self.send_raw(vec![status, note & 0x7F, velocity & 0x7F]);
    }

    /// Send a Note Off message
    pub fn note_off(&self, note: u8) {
        let status = 0x80 | self.channel;
        self.send_raw(vec![status, note & 0x7F, 0x40]);
    }

    /// Send a Control Change message
    pub fn control_change(&self, cc: u8, value: u8) {
        let status = 0xB0 | self.channel;
        self.send_raw(vec![status, cc & 0x7F, value & 0x7F]);
    }

    /// Send an Aftertouch (Channel Pressure) message
    pub fn aftertouch(&self, pressure: u8) {
        let status = 0xD0 | self.channel;
        self.send_raw(vec![status, pressure & 0x7F]);
    }

    /// Send a Pitch Bend message (14-bit value: 0-16383, center=8192)
    pub fn pitch_bend(&self, value: u16) {
        let status = 0xE0 | self.channel;
        let value = value & 0x3FFF; // 14-bit value
        let lsb = (value & 0x7F) as u8;
        let msb = ((value >> 7) & 0x7F) as u8;
        self.send_raw(vec![status, lsb, msb]);
    }

    /// Send a Program Change message
    pub fn program_change(&self, program: u8) {
        let status = 0xC0 | self.channel;
        self.send_raw(vec![status, program & 0x7F]);
    }

    /// Execute a simulator event with timing
    pub fn execute(&self, event: SimulatorEvent) {
        match event {
            SimulatorEvent::NoteOn {
                note,
                velocity,
                delay_ms,
            } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.note_on(note, velocity);
            }
            SimulatorEvent::NoteOff { note, delay_ms } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.note_off(note);
            }
            SimulatorEvent::ControlChange {
                cc,
                value,
                delay_ms,
            } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.control_change(cc, value);
            }
            SimulatorEvent::Aftertouch { pressure, delay_ms } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.aftertouch(pressure);
            }
            SimulatorEvent::PitchBend { value, delay_ms } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.pitch_bend(value);
            }
            SimulatorEvent::ProgramChange { program, delay_ms } => {
                if delay_ms > 0 {
                    thread::sleep(Duration::from_millis(delay_ms));
                }
                self.program_change(program);
            }
            SimulatorEvent::Wait { duration_ms } => {
                thread::sleep(Duration::from_millis(duration_ms));
            }
        }
    }

    /// Execute a sequence of simulator events
    pub fn execute_sequence(&self, events: Vec<SimulatorEvent>) {
        for event in events {
            self.execute(event);
        }
    }

    /// Perform a high-level gesture
    pub fn perform_gesture(&self, gesture: Gesture) {
        match gesture {
            Gesture::SimpleTap {
                note,
                velocity,
                duration_ms,
            } => {
                self.note_on(note, velocity);
                thread::sleep(Duration::from_millis(duration_ms));
                self.note_off(note);
            }
            Gesture::DoubleTap {
                note,
                velocity,
                tap_duration_ms,
                gap_ms,
            } => {
                // First tap
                self.note_on(note, velocity);
                thread::sleep(Duration::from_millis(tap_duration_ms));
                self.note_off(note);

                // Gap between taps
                thread::sleep(Duration::from_millis(gap_ms));

                // Second tap
                self.note_on(note, velocity);
                thread::sleep(Duration::from_millis(tap_duration_ms));
                self.note_off(note);
            }
            Gesture::LongPress {
                note,
                velocity,
                hold_ms,
            } => {
                self.note_on(note, velocity);
                thread::sleep(Duration::from_millis(hold_ms));
                self.note_off(note);
            }
            Gesture::Chord {
                notes,
                velocity,
                stagger_ms,
                hold_ms,
            } => {
                // Press all notes with stagger
                for (i, &note) in notes.iter().enumerate() {
                    if i > 0 && stagger_ms > 0 {
                        thread::sleep(Duration::from_millis(stagger_ms));
                    }
                    self.note_on(note, velocity);
                }

                // Hold the chord
                thread::sleep(Duration::from_millis(hold_ms));

                // Release all notes in reverse order
                for &note in notes.iter().rev() {
                    self.note_off(note);
                    if stagger_ms > 0 {
                        thread::sleep(Duration::from_millis(stagger_ms));
                    }
                }
            }
            Gesture::EncoderTurn {
                cc,
                direction,
                steps,
                step_delay_ms,
            } => {
                let mut current_value: u8 = 64; // Start at center

                for _ in 0..steps {
                    current_value = match direction {
                        EncoderDirection::Clockwise => current_value.saturating_add(1).min(127),
                        EncoderDirection::CounterClockwise => current_value.saturating_sub(1),
                    };

                    self.control_change(cc, current_value);

                    if step_delay_ms > 0 {
                        thread::sleep(Duration::from_millis(step_delay_ms));
                    }
                }
            }
            Gesture::VelocityRamp {
                note,
                min_velocity,
                max_velocity,
                steps,
            } => {
                let step_size = (max_velocity - min_velocity) / steps;

                for i in 0..steps {
                    let velocity = min_velocity + (step_size * i);
                    self.note_on(note, velocity);
                    thread::sleep(Duration::from_millis(100));
                    self.note_off(note);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

/// Builder for creating test scenarios
#[allow(dead_code)]
pub struct ScenarioBuilder {
    events: Vec<SimulatorEvent>,
}

impl Default for ScenarioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ScenarioBuilder {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn note_on(mut self, note: u8, velocity: u8) -> Self {
        self.events.push(SimulatorEvent::NoteOn {
            note,
            velocity,
            delay_ms: 0,
        });
        self
    }

    pub fn note_off(mut self, note: u8) -> Self {
        self.events
            .push(SimulatorEvent::NoteOff { note, delay_ms: 0 });
        self
    }

    pub fn wait(mut self, duration_ms: u64) -> Self {
        self.events.push(SimulatorEvent::Wait { duration_ms });
        self
    }

    pub fn control_change(mut self, cc: u8, value: u8) -> Self {
        self.events.push(SimulatorEvent::ControlChange {
            cc,
            value,
            delay_ms: 0,
        });
        self
    }

    pub fn aftertouch(mut self, pressure: u8) -> Self {
        self.events.push(SimulatorEvent::Aftertouch {
            pressure,
            delay_ms: 0,
        });
        self
    }

    pub fn pitch_bend(mut self, value: u16) -> Self {
        self.events
            .push(SimulatorEvent::PitchBend { value, delay_ms: 0 });
        self
    }

    pub fn build(self) -> Vec<SimulatorEvent> {
        self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_on_off() {
        let sim = MidiSimulator::new(0);
        sim.note_on(60, 100);
        sim.note_off(60);

        let events = sim.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], vec![0x90, 60, 100]); // Note On
        assert_eq!(events[1], vec![0x80, 60, 0x40]); // Note Off
    }

    #[test]
    fn test_velocity_levels() {
        let sim = MidiSimulator::new(0);

        // Soft
        sim.note_on(60, 30);
        // Medium
        sim.note_on(61, 70);
        // Hard
        sim.note_on(62, 110);

        let events = sim.get_events();
        assert_eq!(events[0][2], 30); // Soft velocity
        assert_eq!(events[1][2], 70); // Medium velocity
        assert_eq!(events[2][2], 110); // Hard velocity
    }

    #[test]
    fn test_control_change() {
        let sim = MidiSimulator::new(0);
        sim.control_change(1, 64);

        let events = sim.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], vec![0xB0, 1, 64]);
    }

    #[test]
    fn test_aftertouch() {
        let sim = MidiSimulator::new(0);
        sim.aftertouch(80);

        let events = sim.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], vec![0xD0, 80]);
    }

    #[test]
    fn test_pitch_bend() {
        let sim = MidiSimulator::new(0);
        sim.pitch_bend(8192); // Center position

        let events = sim.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], vec![0xE0, 0x00, 0x40]); // LSB=0, MSB=64
    }

    #[test]
    fn test_program_change() {
        let sim = MidiSimulator::new(0);
        sim.program_change(5);

        let events = sim.get_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], vec![0xC0, 5]);
    }

    #[test]
    fn test_simple_tap_gesture() {
        let sim = MidiSimulator::new(0);
        sim.perform_gesture(Gesture::SimpleTap {
            note: 60,
            velocity: 100,
            duration_ms: 50,
        });

        let events = sim.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0][0], 0x90); // Note On
        assert_eq!(events[1][0], 0x80); // Note Off
    }

    #[test]
    fn test_scenario_builder() {
        let sim = MidiSimulator::new(0);
        let scenario = ScenarioBuilder::new()
            .note_on(60, 100)
            .wait(100)
            .note_off(60)
            .build();

        sim.execute_sequence(scenario);

        let events = sim.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0][1], 60); // Note number
    }

    #[test]
    fn test_encoder_simulation() {
        let sim = MidiSimulator::new(0);
        sim.perform_gesture(Gesture::EncoderTurn {
            cc: 1,
            direction: EncoderDirection::Clockwise,
            steps: 5,
            step_delay_ms: 0,
        });

        let events = sim.get_events();
        assert_eq!(events.len(), 5);

        // Verify increasing values
        for i in 1..events.len() {
            assert!(events[i][2] > events[i - 1][2]);
        }
    }

    #[test]
    fn test_chord_gesture() {
        let sim = MidiSimulator::new(0);
        sim.perform_gesture(Gesture::Chord {
            notes: vec![60, 64, 67], // C major chord
            velocity: 80,
            stagger_ms: 10,
            hold_ms: 100,
        });

        let events = sim.get_events();
        assert_eq!(events.len(), 6); // 3 note ons + 3 note offs

        // Verify all notes are pressed
        assert_eq!(events[0][1], 60);
        assert_eq!(events[1][1], 64);
        assert_eq!(events[2][1], 67);
    }

    #[test]
    fn test_velocity_ramp_gesture() {
        let sim = MidiSimulator::new(0);
        sim.perform_gesture(Gesture::VelocityRamp {
            note: 60,
            min_velocity: 20,
            max_velocity: 120,
            steps: 5,
        });

        let events = sim.get_events();
        assert_eq!(events.len(), 10); // 5 note ons + 5 note offs

        // Verify increasing velocities
        assert!(events[2][2] > events[0][2]); // Second press harder than first
    }

    #[test]
    fn test_channel_masking() {
        let sim = MidiSimulator::new(15); // Max channel
        sim.note_on(60, 100);

        let events = sim.get_events();
        assert_eq!(events[0][0], 0x9F); // Status byte with channel 15
    }
}
