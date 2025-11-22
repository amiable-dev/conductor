// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Event type definitions
//!
//! This module defines the event types used throughout the MIDIMon engine:
//!
//! - [`MidiEvent`]: Raw MIDI protocol events (defined in event_processor.rs)
//! - [`InputEvent`]: Protocol-agnostic input events for abstraction layer
//! - [`ProcessedEvent`]: High-level processed events with timing/state detection
//!
//! The [`InputEvent`] enum provides a protocol-agnostic abstraction over MIDI events,
//! using domain terminology (pad, encoder, pressure) instead of MIDI-specific terms
//! (note, cc). This enables future support for HID and other input protocols.

use std::time::Instant;

pub use crate::event_processor::{EncoderDirection, MidiEvent, ProcessedEvent, VelocityLevel};

/// Protocol-agnostic input event abstraction
///
/// InputEvent provides a protocol-independent representation of controller inputs,
/// abstracting away MIDI-specific terminology. This enables the system to support
/// multiple input protocols (MIDI, HID, etc.) with a unified interface.
///
/// # Design Principles
///
/// - **Protocol-agnostic naming**: Uses domain terms (pad, encoder) not protocol terms (note, cc)
/// - **Timestamp preservation**: All events carry their original timestamp
/// - **Direct mapping**: One-to-one correspondence with MIDI events for now
/// - **Future extensibility**: Designed to support HID and other protocols later
///
/// # Examples
///
/// ```rust
/// use conductor_core::events::{InputEvent, MidiEvent};
/// use std::time::Instant;
///
/// let time = Instant::now();
/// let midi = MidiEvent::NoteOn { note: 36, velocity: 100, time };
/// let input: InputEvent = midi.into();
///
/// match input {
///     InputEvent::PadPressed { pad, velocity, .. } => {
///         println!("Pad {} pressed with velocity {}", pad, velocity);
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Pad pressed (physical button/pad on controller)
    PadPressed {
        pad: u8,
        velocity: u8,
        time: Instant,
    },
    /// Pad released (button/pad released)
    PadReleased { pad: u8, time: Instant },
    /// Encoder turned (rotary knob or encoder)
    EncoderTurned {
        encoder: u8,
        value: u8,
        time: Instant,
    },
    /// Polyphonic aftertouch/pressure applied to specific pad
    PolyPressure {
        pad: u8,
        pressure: u8,
        time: Instant,
    },
    /// Aftertouch/pressure applied (channel-wide)
    Aftertouch { pressure: u8, time: Instant },
    /// Pitch bend/touch strip moved
    PitchBend { value: u16, time: Instant },
    /// Program change
    ProgramChange { program: u8, time: Instant },
    /// Generic control change (for unmapped controls)
    ControlChange {
        control: u8,
        value: u8,
        time: Instant,
    },
}

impl InputEvent {
    /// Returns the timestamp of this event
    ///
    /// # Examples
    ///
    /// ```rust
    /// use conductor_core::events::InputEvent;
    /// use std::time::Instant;
    ///
    /// let time = Instant::now();
    /// let event = InputEvent::PadPressed { pad: 1, velocity: 100, time };
    /// assert_eq!(event.timestamp(), time);
    /// ```
    pub fn timestamp(&self) -> Instant {
        match self {
            InputEvent::PadPressed { time, .. }
            | InputEvent::PadReleased { time, .. }
            | InputEvent::EncoderTurned { time, .. }
            | InputEvent::PolyPressure { time, .. }
            | InputEvent::Aftertouch { time, .. }
            | InputEvent::PitchBend { time, .. }
            | InputEvent::ProgramChange { time, .. }
            | InputEvent::ControlChange { time, .. } => *time,
        }
    }

    /// Returns the event type as a string
    ///
    /// Useful for debugging, logging, and display purposes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use conductor_core::events::InputEvent;
    /// use std::time::Instant;
    ///
    /// let event = InputEvent::PadPressed {
    ///     pad: 1,
    ///     velocity: 100,
    ///     time: Instant::now()
    /// };
    /// assert_eq!(event.event_type(), "PadPressed");
    /// ```
    pub fn event_type(&self) -> &'static str {
        match self {
            InputEvent::PadPressed { .. } => "PadPressed",
            InputEvent::PadReleased { .. } => "PadReleased",
            InputEvent::EncoderTurned { .. } => "EncoderTurned",
            InputEvent::PolyPressure { .. } => "PolyPressure",
            InputEvent::Aftertouch { .. } => "Aftertouch",
            InputEvent::PitchBend { .. } => "PitchBend",
            InputEvent::ProgramChange { .. } => "ProgramChange",
            InputEvent::ControlChange { .. } => "ControlChange",
        }
    }
}

/// Convert MIDI events to protocol-agnostic input events
///
/// This conversion preserves all data while abstracting protocol-specific terminology.
/// MIDI concepts are mapped as follows:
///
/// - NoteOn → PadPressed (note → pad)
/// - NoteOff → PadReleased (note → pad)
/// - ControlChange → EncoderTurned (cc → encoder)
/// - PolyPressure → PolyPressure (note → pad, pressure preserved)
/// - Aftertouch → Aftertouch (pressure preserved)
/// - PitchBend → PitchBend (value preserved)
/// - ProgramChange → ProgramChange (program preserved)
impl From<MidiEvent> for InputEvent {
    fn from(midi: MidiEvent) -> Self {
        match midi {
            MidiEvent::NoteOn {
                note,
                velocity,
                time,
            } => InputEvent::PadPressed {
                pad: note,
                velocity,
                time,
            },
            MidiEvent::NoteOff { note, time } => InputEvent::PadReleased { pad: note, time },
            MidiEvent::ControlChange { cc, value, time } => InputEvent::EncoderTurned {
                encoder: cc,
                value,
                time,
            },
            MidiEvent::PolyPressure {
                note,
                pressure,
                time,
            } => InputEvent::PolyPressure {
                pad: note,
                pressure,
                time,
            },
            MidiEvent::Aftertouch { pressure, time } => InputEvent::Aftertouch { pressure, time },
            MidiEvent::PitchBend { value, time } => InputEvent::PitchBend { value, time },
            MidiEvent::ProgramChange { program, time } => {
                InputEvent::ProgramChange { program, time }
            }
        }
    }
}
