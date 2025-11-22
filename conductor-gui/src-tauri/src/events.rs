// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Event types for communication between backend and frontend

use midi_msg::{ChannelVoiceMsg, MidiMsg};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Emitter;
use tokio::sync::RwLock;

/// Events emitted from backend to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[allow(dead_code)] // Part of event API, used for frontend communication
pub enum AppEvent {
    /// Daemon status changed
    DaemonStatusChanged { running: bool, connected: bool },

    /// MIDI event received (for MIDI Learn mode)
    MidiEventReceived { note: u8, velocity: u8, channel: u8 },

    /// Configuration reloaded
    ConfigReloaded {
        success: bool,
        error: Option<String>,
    },

    /// Error occurred
    Error { message: String },
}

/// MIDI event information for the live console
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // Part of live console API, used for MIDI event display
pub struct MidiEventInfo {
    pub timestamp: u64,
    pub event_type: String,
    pub channel: Option<u8>,
    pub note: Option<u8>,
    pub velocity: Option<u8>,
    pub cc_number: Option<u8>,
    pub cc_value: Option<u8>,
    pub pitch_bend: Option<i16>,
    pub aftertouch: Option<u8>,
    pub raw_bytes: Vec<u8>,
    pub description: String,
}

impl MidiEventInfo {
    /// Create from raw MIDI bytes
    #[allow(dead_code)] // Part of MIDI parsing API, used by live console
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if bytes.is_empty() {
            return Self {
                timestamp,
                event_type: "Unknown".to_string(),
                channel: None,
                note: None,
                velocity: None,
                cc_number: None,
                cc_value: None,
                pitch_bend: None,
                aftertouch: None,
                raw_bytes: bytes.to_vec(),
                description: "Empty MIDI message".to_string(),
            };
        }

        // Parse MIDI message using midi-msg library
        match MidiMsg::from_midi(bytes) {
            Ok((msg, _len)) => match msg {
                MidiMsg::ChannelVoice { channel, msg } => {
                    let channel_num = channel as u8;

                    match msg {
                        ChannelVoiceMsg::NoteOff { note, velocity } => Self {
                            timestamp,
                            event_type: "NoteOff".to_string(),
                            channel: Some(channel_num),
                            note: Some(note.into()),
                            velocity: Some(velocity.into()),
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: None,
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Note Off: Note {} Velocity {} Channel {}",
                                u8::from(note),
                                u8::from(velocity),
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::NoteOn { note, velocity } => Self {
                            timestamp,
                            event_type: "NoteOn".to_string(),
                            channel: Some(channel_num),
                            note: Some(note.into()),
                            velocity: Some(velocity.into()),
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: None,
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Note On: Note {} Velocity {} Channel {}",
                                u8::from(note),
                                u8::from(velocity),
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::PolyPressure { note, pressure } => Self {
                            timestamp,
                            event_type: "PolyAftertouch".to_string(),
                            channel: Some(channel_num),
                            note: Some(note.into()),
                            velocity: None,
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: Some(pressure.into()),
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Poly Aftertouch: Note {} Pressure {} Channel {}",
                                u8::from(note),
                                u8::from(pressure),
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::ControlChange { control } => {
                            // Extract CC number and value from ControlChange enum
                            use midi_msg::ControlChange;
                            if let ControlChange::CC { control: cc, value } = control {
                                Self {
                                    timestamp,
                                    event_type: "ControlChange".to_string(),
                                    channel: Some(channel_num),
                                    note: None,
                                    velocity: None,
                                    cc_number: Some(cc),
                                    cc_value: Some(value),
                                    pitch_bend: None,
                                    aftertouch: None,
                                    raw_bytes: bytes.to_vec(),
                                    description: format!(
                                        "CC: #{} Value {} Channel {}",
                                        cc, value, channel_num
                                    ),
                                }
                            } else {
                                // For other ControlChange variants (e.g., Bank Select MSB/LSB, etc.)
                                Self {
                                    timestamp,
                                    event_type: "ControlChange".to_string(),
                                    channel: Some(channel_num),
                                    note: None,
                                    velocity: None,
                                    cc_number: None,
                                    cc_value: None,
                                    pitch_bend: None,
                                    aftertouch: None,
                                    raw_bytes: bytes.to_vec(),
                                    description: format!("CC (special): {:02X?}", bytes),
                                }
                            }
                        }
                        ChannelVoiceMsg::ProgramChange { program } => Self {
                            timestamp,
                            event_type: "ProgramChange".to_string(),
                            channel: Some(channel_num),
                            note: None,
                            velocity: None,
                            cc_number: None,
                            cc_value: Some(program.into()),
                            pitch_bend: None,
                            aftertouch: None,
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Program Change: {} Channel {}",
                                u8::from(program),
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::ChannelPressure { pressure } => Self {
                            timestamp,
                            event_type: "ChannelAftertouch".to_string(),
                            channel: Some(channel_num),
                            note: None,
                            velocity: None,
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: Some(pressure.into()),
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Channel Aftertouch: Pressure {} Channel {}",
                                u8::from(pressure),
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::PitchBend { bend } => {
                            // midi-msg uses u16 for pitch bend (0-16383, center=8192)
                            // Store as i16 for compatibility with existing code
                            let bend_value = bend as i16;
                            Self {
                                timestamp,
                                event_type: "PitchBend".to_string(),
                                channel: Some(channel_num),
                                note: None,
                                velocity: None,
                                cc_number: None,
                                cc_value: None,
                                pitch_bend: Some(bend_value),
                                aftertouch: None,
                                raw_bytes: bytes.to_vec(),
                                description: format!(
                                    "Pitch Bend: {} Channel {}",
                                    bend_value, channel_num
                                ),
                            }
                        }
                        // High-resolution variants (CA-031)
                        ChannelVoiceMsg::HighResNoteOn { note, velocity } => Self {
                            timestamp,
                            event_type: "NoteOn".to_string(),
                            channel: Some(channel_num),
                            note: Some(note),
                            velocity: Some((velocity >> 9) as u8), // Convert 16-bit to 7-bit
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: None,
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Note On (High-Res): Note {} Velocity {} Channel {}",
                                note,
                                velocity >> 9,
                                channel_num
                            ),
                        },
                        ChannelVoiceMsg::HighResNoteOff { note, velocity } => Self {
                            timestamp,
                            event_type: "NoteOff".to_string(),
                            channel: Some(channel_num),
                            note: Some(note),
                            velocity: Some((velocity >> 9) as u8), // Convert 16-bit to 7-bit
                            cc_number: None,
                            cc_value: None,
                            pitch_bend: None,
                            aftertouch: None,
                            raw_bytes: bytes.to_vec(),
                            description: format!(
                                "Note Off (High-Res): Note {} Velocity {} Channel {}",
                                note,
                                velocity >> 9,
                                channel_num
                            ),
                        },
                    }
                }
                _ => {
                    // Handle non-channel-voice messages (System Common, System Real-Time, etc.)
                    Self {
                        timestamp,
                        event_type: "Unknown".to_string(),
                        channel: None,
                        note: None,
                        velocity: None,
                        cc_number: None,
                        cc_value: None,
                        pitch_bend: None,
                        aftertouch: None,
                        raw_bytes: bytes.to_vec(),
                        description: format!("Non-channel-voice MIDI message: {:02X?}", bytes),
                    }
                }
            },
            Err(_) => {
                // Failed to parse MIDI message
                Self {
                    timestamp,
                    event_type: "Unknown".to_string(),
                    channel: None,
                    note: None,
                    velocity: None,
                    cc_number: None,
                    cc_value: None,
                    pitch_bend: None,
                    aftertouch: None,
                    raw_bytes: bytes.to_vec(),
                    description: format!("Invalid MIDI message: {:02X?}", bytes),
                }
            }
        }
    }

    /// Format as human-readable note name
    #[allow(dead_code)] // Part of MIDI event API, used for note display
    pub fn note_name(&self) -> Option<String> {
        self.note.map(|n| {
            let note_names = [
                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
            ];
            let octave = (n / 12) as i32 - 1;
            let note_index = (n % 12) as usize;
            format!("{}{}", note_names[note_index], octave)
        })
    }
}

/// Event stream manager for MIDI event monitoring
pub struct EventStreamManager {
    #[allow(dead_code)] // Stored for future use in event emission
    app_handle: Option<tauri::AppHandle>,
    active: Arc<RwLock<bool>>,
    #[allow(dead_code)]
    max_events: usize,
}

impl EventStreamManager {
    pub fn new(max_events: usize) -> Self {
        Self {
            app_handle: None,
            active: Arc::new(RwLock::new(false)),
            max_events,
        }
    }

    #[allow(dead_code)] // Part of event stream API, used for setting up event emission
    pub fn set_app_handle(&mut self, handle: tauri::AppHandle) {
        self.app_handle = Some(handle);
    }

    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }

    pub async fn start(&self) {
        *self.active.write().await = true;
    }

    pub async fn stop(&self) {
        *self.active.write().await = false;
    }

    /// Emit a MIDI event to the frontend
    #[allow(dead_code)] // Part of live console API for emitting events
    pub async fn emit_event(&self, event: MidiEventInfo) {
        if let Some(handle) = &self.app_handle {
            if *self.active.read().await {
                // Emit to frontend via Tauri event system
                if let Err(e) = handle.emit("midi-event", event) {
                    tracing::error!("Failed to emit MIDI event: {}", e);
                }
            }
        }
    }

    /// Emit multiple events (batch)
    #[allow(dead_code)] // Part of live console API for batch event emission
    pub async fn emit_events(&self, events: Vec<MidiEventInfo>) {
        if let Some(handle) = &self.app_handle {
            if *self.active.read().await {
                if let Err(e) = handle.emit("midi-events", events) {
                    tracing::error!("Failed to emit MIDI events: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_on_parsing() {
        let bytes = vec![0x90, 60, 100]; // Note On, Middle C, velocity 100
        let event = MidiEventInfo::from_bytes(&bytes);

        assert_eq!(event.event_type, "NoteOn");
        assert_eq!(event.note, Some(60));
        assert_eq!(event.velocity, Some(100));
        assert_eq!(event.channel, Some(0));
        assert_eq!(event.note_name(), Some("C4".to_string()));
    }

    #[test]
    fn test_note_off_parsing() {
        let bytes = vec![0x80, 60, 0]; // Note Off, Middle C
        let event = MidiEventInfo::from_bytes(&bytes);

        assert_eq!(event.event_type, "NoteOff");
        assert_eq!(event.note, Some(60));
        assert_eq!(event.velocity, Some(0));
    }

    #[test]
    fn test_cc_parsing() {
        let bytes = vec![0xB0, 1, 64]; // CC #1 (Mod Wheel), value 64
        let event = MidiEventInfo::from_bytes(&bytes);

        assert_eq!(event.event_type, "ControlChange");
        assert_eq!(event.cc_number, Some(1));
        assert_eq!(event.cc_value, Some(64));
    }

    #[test]
    fn test_pitch_bend_parsing() {
        let bytes = vec![0xE0, 0x00, 0x40]; // Pitch bend center
        let event = MidiEventInfo::from_bytes(&bytes);

        assert_eq!(event.event_type, "PitchBend");
        assert_eq!(event.pitch_bend, Some(8192)); // 0x40 << 7 = 8192
    }

    #[test]
    fn test_note_names() {
        assert_eq!(
            MidiEventInfo::from_bytes(&[0x90, 0, 100]).note_name(),
            Some("C-1".to_string())
        );
        assert_eq!(
            MidiEventInfo::from_bytes(&[0x90, 60, 100]).note_name(),
            Some("C4".to_string())
        );
        assert_eq!(
            MidiEventInfo::from_bytes(&[0x90, 127, 100]).note_name(),
            Some("G9".to_string())
        );
    }
}
