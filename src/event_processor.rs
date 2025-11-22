// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use colored::Colorize;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum MidiEvent {
    NoteOn {
        note: u8,
        velocity: u8,
        time: Instant,
    },
    NoteOff {
        note: u8,
        time: Instant,
    },
    ControlChange {
        cc: u8,
        value: u8,
        time: Instant,
    },
    Aftertouch {
        pressure: u8,
        time: Instant,
    },
    PitchBend {
        value: u16,
        time: Instant,
    },
    ProgramChange {
        program: u8,
        time: Instant,
    },
}

#[derive(Debug, Clone)]
pub enum ProcessedEvent {
    ShortPress {
        note: u8,
    },
    MediumPress {
        note: u8,
        duration_ms: u128,
    },
    LongPress {
        note: u8,
        duration_ms: u128,
    },
    HoldDetected {
        note: u8,
    },
    PadPressed {
        note: u8,
        velocity: u8,
        velocity_level: VelocityLevel,
    },
    PadReleased {
        note: u8,
        hold_duration_ms: u128,
    },
    EncoderTurned {
        cc: u8,
        value: u8,
        direction: EncoderDirection,
        delta: u8,
    },
    DoubleTap {
        note: u8,
    },
    ChordDetected {
        notes: Vec<u8>,
    },
    AftertouchChanged {
        pressure: u8,
    },
    PitchBendMoved {
        value: u16,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VelocityLevel {
    Soft,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncoderDirection {
    Clockwise,
    CounterClockwise,
}

pub struct EventProcessor {
    note_press_times: HashMap<u8, Instant>,
    held_notes: HashMap<u8, Instant>,
    last_cc_values: HashMap<u8, u8>,
    last_note_tap: HashMap<u8, Instant>,
    chord_buffer: Vec<(u8, Instant)>,
    chord_timeout: Duration,
    double_tap_timeout: Duration,
    hold_threshold: Duration,
}

impl Default for EventProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            note_press_times: HashMap::new(),
            held_notes: HashMap::new(),
            last_cc_values: HashMap::new(),
            last_note_tap: HashMap::new(),
            chord_buffer: Vec::new(),
            chord_timeout: Duration::from_millis(50),
            double_tap_timeout: Duration::from_millis(300),
            hold_threshold: Duration::from_secs(2),
        }
    }

    pub fn process(&mut self, event: MidiEvent) -> Vec<ProcessedEvent> {
        let mut results = Vec::new();

        match event {
            MidiEvent::NoteOn {
                note,
                velocity,
                time,
            } => {
                self.note_press_times.insert(note, time);
                self.held_notes.insert(note, time);

                // Check for double-tap
                if let Some(&last_tap_time) = self.last_note_tap.get(&note) {
                    if time.duration_since(last_tap_time) < self.double_tap_timeout {
                        results.push(ProcessedEvent::DoubleTap { note });
                        self.last_note_tap.remove(&note);
                    } else {
                        self.last_note_tap.insert(note, time);
                    }
                } else {
                    self.last_note_tap.insert(note, time);
                }

                // Detect velocity levels
                let velocity_level = match velocity {
                    0..=40 => VelocityLevel::Soft,
                    41..=80 => VelocityLevel::Medium,
                    81..=127 => VelocityLevel::Hard,
                    _ => VelocityLevel::Medium,
                };

                results.push(ProcessedEvent::PadPressed {
                    note,
                    velocity,
                    velocity_level,
                });

                // Add to chord buffer
                self.chord_buffer.push((note, time));

                // Check for chord (multiple notes pressed within chord_timeout)
                self.chord_buffer
                    .retain(|(_, t)| time.duration_since(*t) < self.chord_timeout);

                if self.chord_buffer.len() > 1 {
                    let notes: Vec<u8> = self.chord_buffer.iter().map(|(n, _)| *n).collect();
                    results.push(ProcessedEvent::ChordDetected { notes });
                }
            }

            MidiEvent::NoteOff { note, time } => {
                if let Some(press_time) = self.note_press_times.remove(&note) {
                    let duration = time.duration_since(press_time);
                    let duration_ms = duration.as_millis();

                    results.push(ProcessedEvent::PadReleased {
                        note,
                        hold_duration_ms: duration_ms,
                    });

                    if duration_ms < 200 {
                        results.push(ProcessedEvent::ShortPress { note });
                    } else if duration_ms < 1000 {
                        results.push(ProcessedEvent::MediumPress { note, duration_ms });
                    } else {
                        results.push(ProcessedEvent::LongPress { note, duration_ms });
                    }
                }
                self.held_notes.remove(&note);

                // Remove from chord buffer
                self.chord_buffer.retain(|(n, _)| *n != note);
            }

            MidiEvent::ControlChange { cc, value, .. } => {
                // Detect encoder direction
                if let Some(&last_value) = self.last_cc_values.get(&cc) {
                    let direction = if value > last_value {
                        EncoderDirection::Clockwise
                    } else if value < last_value {
                        EncoderDirection::CounterClockwise
                    } else {
                        // No change
                        return results;
                    };

                    let delta = (value as i16 - last_value as i16).unsigned_abs() as u8;

                    results.push(ProcessedEvent::EncoderTurned {
                        cc,
                        value,
                        direction,
                        delta,
                    });
                }
                self.last_cc_values.insert(cc, value);
            }

            MidiEvent::Aftertouch { pressure, .. } => {
                results.push(ProcessedEvent::AftertouchChanged { pressure });
            }

            MidiEvent::PitchBend { value, .. } => {
                results.push(ProcessedEvent::PitchBendMoved { value });
            }

            _ => {}
        }

        results
    }

    pub fn check_holds(&mut self) -> Vec<ProcessedEvent> {
        let mut results = Vec::new();
        let now = Instant::now();

        for (&note, &press_time) in &self.held_notes {
            if now.duration_since(press_time) >= self.hold_threshold {
                results.push(ProcessedEvent::HoldDetected { note });
                // Note: We might want to track which holds we've already reported
                // to avoid repeated triggers
            }
        }

        results
    }

    pub fn log_processed_event(event: &ProcessedEvent, mode: u8) {
        let timestamp = chrono::Local::now().format("%H:%M:%S%.3f");
        let mode_str = format!("[Mode {}]", mode).cyan();

        let event_str = match event {
            ProcessedEvent::PadPressed {
                note,
                velocity,
                velocity_level,
            } => {
                let level_str = match velocity_level {
                    VelocityLevel::Soft => "SOFT".blue(),
                    VelocityLevel::Medium => "MED".yellow(),
                    VelocityLevel::Hard => "HARD".red(),
                };
                format!("Pad {:3} ON  vel {:3} {}", note, velocity, level_str)
            }
            ProcessedEvent::PadReleased {
                note,
                hold_duration_ms,
            } => format!("Pad {:3} OFF (held {}ms)", note, hold_duration_ms)
                .yellow()
                .to_string(),
            ProcessedEvent::ShortPress { note } => {
                format!("SHORT TAP   {:3}", note).green().to_string()
            }
            ProcessedEvent::MediumPress { note, duration_ms } => {
                format!("MEDIUM PRESS {:3} ({}ms)", note, duration_ms)
                    .blue()
                    .to_string()
            }
            ProcessedEvent::LongPress { note, duration_ms } => {
                format!("LONG PRESS  {:3} ({}ms)", note, duration_ms)
                    .magenta()
                    .to_string()
            }
            ProcessedEvent::HoldDetected { note } => {
                format!("HOLD ACTIVE {:3}", note).red().bold().to_string()
            }
            ProcessedEvent::DoubleTap { note } => {
                format!("DOUBLE TAP  {:3}", note).cyan().bold().to_string()
            }
            ProcessedEvent::ChordDetected { notes } => format!("CHORD       {:?}", notes)
                .purple()
                .bold()
                .to_string(),
            ProcessedEvent::EncoderTurned {
                cc,
                value,
                direction,
                delta,
            } => {
                let dir_str = match direction {
                    EncoderDirection::Clockwise => "→",
                    EncoderDirection::CounterClockwise => "←",
                };
                format!("Encoder {:3} {} val {:3} (Δ{})", cc, dir_str, value, delta)
                    .cyan()
                    .to_string()
            }
            ProcessedEvent::AftertouchChanged { pressure } => {
                format!("Aftertouch  {:3}", pressure).purple().to_string()
            }
            ProcessedEvent::PitchBendMoved { value } => {
                format!("Pitch Bend  {:5}", value).purple().to_string()
            }
        };

        println!(
            "{} {} {}",
            timestamp.to_string().dimmed(),
            mode_str,
            event_str
        );
    }
}
