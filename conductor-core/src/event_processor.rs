// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::events::InputEvent; // Protocol-agnostic event processing (v3.0)
use midi_msg::{ChannelVoiceMsg, ControlChange, MidiMsg};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, trace};

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
    PolyPressure {
        note: u8,
        pressure: u8,
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

impl MidiEvent {
    /// Parse raw MIDI bytes into a MidiEvent using the midi-msg library.
    ///
    /// This centralizes MIDI message parsing across the codebase, ensuring
    /// consistent handling of MIDI messages and providing better error messages.
    ///
    /// # Arguments
    /// * `msg` - Raw MIDI message bytes (typically 1-3 bytes)
    ///
    /// # Returns
    /// * `Ok(MidiEvent)` - Successfully parsed MIDI event with current timestamp
    /// * `Err(String)` - Error message describing why parsing failed
    ///
    /// # Example
    /// ```rust
    /// use conductor_core::event_processor::MidiEvent;
    ///
    /// // Note On: C4 (60) with velocity 100
    /// let note_on = MidiEvent::from_midi_msg(&[0x90, 60, 100]).unwrap();
    ///
    /// // Control Change: CC 7 (volume) with value 127
    /// let cc = MidiEvent::from_midi_msg(&[0xB0, 7, 127]).unwrap();
    /// ```
    pub fn from_midi_msg(msg: &[u8]) -> Result<Self, String> {
        let now = Instant::now();

        match MidiMsg::from_midi(msg) {
            Ok((MidiMsg::ChannelVoice { msg: voice_msg, .. }, _))
            | Ok((MidiMsg::RunningChannelVoice { msg: voice_msg, .. }, _)) => {
                match voice_msg {
                    ChannelVoiceMsg::NoteOn { note, velocity } => {
                        if velocity > 0 {
                            Ok(MidiEvent::NoteOn {
                                note,
                                velocity,
                                time: now,
                            })
                        } else {
                            // Note On with velocity 0 is treated as Note Off
                            Ok(MidiEvent::NoteOff { note, time: now })
                        }
                    }

                    ChannelVoiceMsg::NoteOff { note, .. } => {
                        Ok(MidiEvent::NoteOff { note, time: now })
                    }

                    ChannelVoiceMsg::ControlChange { control } => {
                        // Extract CC number and value from ControlChange enum
                        if let ControlChange::CC { control: cc, value } = control {
                            Ok(MidiEvent::ControlChange {
                                cc,
                                value,
                                time: now,
                            })
                        } else {
                            Err(format!("Unsupported ControlChange variant: {:?}", control))
                        }
                    }

                    ChannelVoiceMsg::PolyPressure { note, pressure } => {
                        Ok(MidiEvent::PolyPressure {
                            note,
                            pressure,
                            time: now,
                        })
                    }

                    ChannelVoiceMsg::ChannelPressure { pressure } => Ok(MidiEvent::Aftertouch {
                        pressure,
                        time: now,
                    }),

                    ChannelVoiceMsg::PitchBend { bend } => Ok(MidiEvent::PitchBend {
                        value: bend,
                        time: now,
                    }),

                    ChannelVoiceMsg::ProgramChange { program } => {
                        Ok(MidiEvent::ProgramChange { program, time: now })
                    }

                    _ => Err(format!(
                        "Unsupported MIDI voice message type: {:?}",
                        voice_msg
                    )),
                }
            }

            Ok((MidiMsg::SystemCommon { .. }, _)) => {
                Err("System Common messages not supported".to_string())
            }

            Ok((MidiMsg::SystemRealTime { .. }, _)) => {
                Err("System Real-Time messages not supported".to_string())
            }

            Err(e) => Err(format!("Failed to parse MIDI message: {:?}", e)),

            _ => Err(format!("Unknown MIDI message type: {:02X?}", msg)),
        }
    }
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

            MidiEvent::PolyPressure { .. } => {
                // Polyphonic aftertouch is received but not currently processed
                // into high-level events. This is a placeholder for future support.
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

    /// Process a protocol-agnostic InputEvent (v3.0)
    ///
    /// This method mirrors `process()` but handles InputEvent instead of MidiEvent,
    /// enabling support for multiple input protocols (MIDI, HID gamepad, etc.) through
    /// a unified processing pipeline.
    ///
    /// # Arguments
    ///
    /// * `event` - Protocol-agnostic input event (from MIDI, gamepad, etc.)
    ///
    /// # Returns
    ///
    /// Vector of ProcessedEvent results (short press, long press, chord detection, etc.)
    ///
    /// # Example
    ///
    /// ```rust
    /// use conductor_core::{EventProcessor, events::InputEvent};
    /// use std::time::Instant;
    ///
    /// let mut processor = EventProcessor::new();
    ///
    /// // Gamepad button press (button ID 128 = South/A/Cross/B)
    /// let event = InputEvent::PadPressed {
    ///     pad: 128,
    ///     velocity: 100,
    ///     time: Instant::now(),
    /// };
    ///
    /// let processed = processor.process_input(event);
    /// // Will detect velocity level, double-tap, chords, etc.
    /// ```
    pub fn process_input(&mut self, event: InputEvent) -> Vec<ProcessedEvent> {
        let mut results = Vec::new();

        match event {
            InputEvent::PadPressed {
                pad,
                velocity,
                time,
            } => {
                self.note_press_times.insert(pad, time);
                self.held_notes.insert(pad, time);

                // Check for double-tap
                if let Some(&last_tap_time) = self.last_note_tap.get(&pad) {
                    if time.duration_since(last_tap_time) < self.double_tap_timeout {
                        results.push(ProcessedEvent::DoubleTap { note: pad });
                        self.last_note_tap.remove(&pad);
                    } else {
                        self.last_note_tap.insert(pad, time);
                    }
                } else {
                    self.last_note_tap.insert(pad, time);
                }

                // Detect velocity levels
                let velocity_level = match velocity {
                    0..=40 => VelocityLevel::Soft,
                    41..=80 => VelocityLevel::Medium,
                    81..=127 => VelocityLevel::Hard,
                    _ => VelocityLevel::Medium,
                };

                results.push(ProcessedEvent::PadPressed {
                    note: pad,
                    velocity,
                    velocity_level,
                });

                // Add to chord buffer
                self.chord_buffer.push((pad, time));

                // Check for chord (multiple pads pressed within chord_timeout)
                self.chord_buffer
                    .retain(|(_, t)| time.duration_since(*t) < self.chord_timeout);

                if self.chord_buffer.len() > 1 {
                    let notes: Vec<u8> = self.chord_buffer.iter().map(|(n, _)| *n).collect();
                    results.push(ProcessedEvent::ChordDetected { notes });
                }
            }

            InputEvent::PadReleased { pad, time } => {
                if let Some(press_time) = self.note_press_times.remove(&pad) {
                    let duration = time.duration_since(press_time);
                    let duration_ms = duration.as_millis();

                    results.push(ProcessedEvent::PadReleased {
                        note: pad,
                        hold_duration_ms: duration_ms,
                    });

                    if duration_ms < 200 {
                        results.push(ProcessedEvent::ShortPress { note: pad });
                    } else if duration_ms < 1000 {
                        results.push(ProcessedEvent::MediumPress {
                            note: pad,
                            duration_ms,
                        });
                    } else {
                        results.push(ProcessedEvent::LongPress {
                            note: pad,
                            duration_ms,
                        });
                    }
                }
                self.held_notes.remove(&pad);

                // Remove from chord buffer
                self.chord_buffer.retain(|(n, _)| *n != pad);
            }

            InputEvent::EncoderTurned {
                encoder,
                value,
                ..
            } => {
                // Detect encoder direction
                if let Some(&last_value) = self.last_cc_values.get(&encoder) {
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
                        cc: encoder,
                        value,
                        direction,
                        delta,
                    });
                }
                self.last_cc_values.insert(encoder, value);
            }

            InputEvent::PolyPressure { .. } => {
                // Polyphonic aftertouch/pressure is received but not currently processed
                // into high-level events. This is a placeholder for future support.
            }

            InputEvent::Aftertouch { pressure, .. } => {
                results.push(ProcessedEvent::AftertouchChanged { pressure });
            }

            InputEvent::PitchBend { value, .. } => {
                results.push(ProcessedEvent::PitchBendMoved { value });
            }

            InputEvent::ControlChange { control, value, .. } => {
                // Generic control change - treat like encoder for now
                if let Some(&last_value) = self.last_cc_values.get(&control) {
                    let direction = if value > last_value {
                        EncoderDirection::Clockwise
                    } else if value < last_value {
                        EncoderDirection::CounterClockwise
                    } else {
                        return results;
                    };

                    let delta = (value as i16 - last_value as i16).unsigned_abs() as u8;

                    results.push(ProcessedEvent::EncoderTurned {
                        cc: control,
                        value,
                        direction,
                        delta,
                    });
                }
                self.last_cc_values.insert(control, value);
            }

            InputEvent::ProgramChange { .. } => {
                // Program change events are not currently processed into high-level events
            }
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
        match event {
            ProcessedEvent::PadPressed {
                note,
                velocity,
                velocity_level,
            } => {
                let level_str = match velocity_level {
                    VelocityLevel::Soft => "SOFT",
                    VelocityLevel::Medium => "MED",
                    VelocityLevel::Hard => "HARD",
                };
                debug!(mode, note, velocity, level = level_str, "Pad pressed");
            }
            ProcessedEvent::PadReleased {
                note,
                hold_duration_ms,
            } => {
                debug!(mode, note, hold_duration_ms, "Pad released");
            }
            ProcessedEvent::ShortPress { note } => {
                debug!(mode, note, "Short tap detected");
            }
            ProcessedEvent::MediumPress { note, duration_ms } => {
                debug!(mode, note, duration_ms, "Medium press detected");
            }
            ProcessedEvent::LongPress { note, duration_ms } => {
                debug!(mode, note, duration_ms, "Long press detected");
            }
            ProcessedEvent::HoldDetected { note } => {
                debug!(mode, note, "Hold detected");
            }
            ProcessedEvent::DoubleTap { note } => {
                debug!(mode, note, "Double tap detected");
            }
            ProcessedEvent::ChordDetected { notes } => {
                debug!(mode, ?notes, "Chord detected");
            }
            ProcessedEvent::EncoderTurned {
                cc,
                value,
                direction,
                delta,
            } => {
                let direction_str = match direction {
                    EncoderDirection::Clockwise => "clockwise",
                    EncoderDirection::CounterClockwise => "counter-clockwise",
                };
                debug!(
                    mode,
                    cc,
                    value,
                    direction = direction_str,
                    delta,
                    "Encoder turned"
                );
            }
            ProcessedEvent::AftertouchChanged { pressure } => {
                trace!(mode, pressure, "Aftertouch changed");
            }
            ProcessedEvent::PitchBendMoved { value } => {
                trace!(mode, value, "Pitch bend moved");
            }
        };
    }
}
