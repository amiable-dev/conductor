// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Unified Input Management (v3.0)
//!
//! This module provides a unified interface for managing both MIDI and gamepad input devices.
//! It combines MidiDeviceManager and GamepadDeviceManager into a single manager that outputs
//! a unified stream of InputEvents.
//!
//! # Features
//!
//! - **Multi-Protocol Support**: MIDI and HID gamepad input
//! - **Unified Event Stream**: Single InputEvent channel for all inputs
//! - **Flexible Device Selection**: MIDI-only, gamepad-only, or both simultaneously
//! - **Automatic Reconnection**: Inherits reconnection behavior from device managers
//! - **Thread Safety**: Arc/Mutex patterns for safe concurrent access
//!
//! # Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │  InputManager                                    │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  Device Selection                          │ │
//! │  │  - MidiOnly / GamepadOnly / Both           │ │
//! │  └────────────────────────────────────────────┘ │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  MidiDeviceManager                         │ │
//! │  │  - Outputs: MidiEvent                      │ │
//! │  │  - Converts to: InputEvent                 │ │
//! │  └────────────────────────────────────────────┘ │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  HidDeviceManager                          │ │
//! │  │  - Outputs: InputEvent (native)            │ │
//! │  └────────────────────────────────────────────┘ │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  Unified InputEvent Stream                 │ │
//! │  │  - Merged from MIDI + Gamepad              │ │
//! │  │  - Single mpsc channel                     │ │
//! │  └────────────────────────────────────────────┘ │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use conductor_daemon::input_manager::{InputManager, InputMode};
//! use tokio::sync::mpsc;
//!
//! # async fn example() -> Result<(), String> {
//! let (event_tx, mut event_rx) = mpsc::channel(1024);
//! let (command_tx, _) = mpsc::channel(32);
//!
//! let mut manager = InputManager::new(
//!     Some("Maschine Mikro MK3".to_string()),
//!     true,  // auto_reconnect
//!     InputMode::MidiOnly
//! );
//!
//! manager.connect(event_tx, command_tx)?;
//!
//! // Process unified event stream
//! while let Some(event) = event_rx.recv().await {
//!     println!("Input: {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```

use crate::daemon::DaemonCommand;
use crate::gamepad_device::HidDeviceManager;
use crate::midi_device::MidiDeviceManager;
use conductor_core::events::InputEvent;
use conductor_core::event_processor::MidiEvent;
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Input device selection mode (v3.0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Use MIDI device only
    MidiOnly,
    /// Use gamepad device only
    GamepadOnly,
    /// Use both MIDI and gamepad simultaneously
    Both,
}

/// Unified input device manager (v3.0)
///
/// Manages connections to MIDI and/or gamepad devices, providing a unified
/// stream of InputEvents for processing by the event engine.
///
/// # Thread Safety
///
/// The manager is safe to share across threads. Device managers use Arc/Mutex
/// for internal state synchronization.
///
/// # Connection Lifecycle
///
/// 1. **Connect**: Establish connections based on InputMode
/// 2. **Active**: Process events from all active devices
/// 3. **Disconnect**: Clean up device connections
/// 4. **Reconnect**: Automatic reconnection via device managers
pub struct InputManager {
    /// MIDI device manager (optional)
    midi_manager: Option<MidiDeviceManager>,

    /// HID device manager (optional)
    gamepad_manager: Option<HidDeviceManager>,

    /// Input mode selection
    mode: InputMode,
}

impl InputManager {
    /// Create a new unified input manager
    ///
    /// # Arguments
    ///
    /// * `midi_device_name` - Name of MIDI device to connect to (None = first available)
    /// * `auto_reconnect` - Enable automatic reconnection for both MIDI and gamepad
    /// * `mode` - Input mode selection (MidiOnly, GamepadOnly, or Both)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::input_manager::{InputManager, InputMode};
    ///
    /// // MIDI + Gamepad hybrid setup
    /// let manager = InputManager::new(
    ///     Some("Maschine Mikro MK3".to_string()),
    ///     true,
    ///     InputMode::Both
    /// );
    ///
    /// // Gamepad-only setup
    /// let gamepad_only = InputManager::new(
    ///     None,
    ///     true,
    ///     InputMode::GamepadOnly
    /// );
    /// ```
    pub fn new(
        midi_device_name: Option<String>,
        auto_reconnect: bool,
        mode: InputMode,
    ) -> Self {
        let midi_manager = if mode == InputMode::MidiOnly || mode == InputMode::Both {
            Some(MidiDeviceManager::new(
                midi_device_name.unwrap_or_default(),
                auto_reconnect,
            ))
        } else {
            None
        };

        let gamepad_manager = if mode == InputMode::GamepadOnly || mode == InputMode::Both {
            Some(HidDeviceManager::new(auto_reconnect))
        } else {
            None
        };

        Self {
            midi_manager,
            gamepad_manager,
            mode,
        }
    }

    /// Connect to input devices based on mode
    ///
    /// Creates a unified InputEvent stream by:
    /// - Converting MIDI events to InputEvents
    /// - Merging gamepad InputEvents
    /// - Sending both through the same channel
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel sender for unified InputEvent stream
    /// * `command_tx` - Channel sender for daemon commands
    ///
    /// # Returns
    ///
    /// Status message describing connected devices
    ///
    /// # Errors
    ///
    /// Returns error if no devices could be connected
    pub fn connect(
        &mut self,
        event_tx: mpsc::Sender<InputEvent>,
        command_tx: mpsc::Sender<DaemonCommand>,
    ) -> Result<String, String> {
        let mut status_messages = Vec::new();

        // Connect MIDI device (with MidiEvent → InputEvent conversion)
        if let Some(ref mut midi_mgr) = self.midi_manager {
            // Create intermediate channel for MIDI events
            let (midi_event_tx, mut midi_event_rx) = mpsc::channel::<MidiEvent>(1024);

            match midi_mgr.connect(midi_event_tx, command_tx.clone()) {
                Ok((port_idx, port_name)) => {
                    info!(
                        device_type = "MIDI",
                        port = %port_name,
                        "Connected to MIDI device (port {})",
                        port_idx
                    );
                    status_messages.push(format!("MIDI: {} (port {})", port_name, port_idx));

                    // Spawn converter task: MidiEvent → InputEvent
                    let event_tx_clone = event_tx.clone();
                    tokio::spawn(async move {
                        while let Some(midi_event) = midi_event_rx.recv().await {
                            let input_event = convert_midi_to_input(midi_event);
                            if let Err(e) = event_tx_clone.send(input_event).await {
                                warn!(error = %e, "Failed to send converted InputEvent");
                                break;
                            }
                        }
                        debug!("MIDI-to-Input converter task exited");
                    });
                }
                Err(e) => {
                    if self.mode == InputMode::MidiOnly {
                        return Err(format!("Failed to connect MIDI device: {}", e));
                    }
                    warn!(error = %e, "Failed to connect MIDI device (continuing with gamepad)");
                }
            }
        }

        // Connect gamepad device (native InputEvent)
        if let Some(ref mut gamepad_mgr) = self.gamepad_manager {
            match gamepad_mgr.connect(event_tx.clone(), command_tx.clone()) {
                Ok((gamepad_id, gamepad_name)) => {
                    info!(
                        device_type = "Gamepad",
                        name = %gamepad_name,
                        "Connected to gamepad (ID {:?})",
                        gamepad_id
                    );
                    status_messages.push(format!("Gamepad: {} (ID {:?})", gamepad_name, gamepad_id));
                }
                Err(e) => {
                    if self.mode == InputMode::GamepadOnly {
                        return Err(format!("Failed to connect gamepad: {}", e));
                    }
                    warn!(error = %e, "Failed to connect gamepad (continuing with MIDI)");
                }
            }
        }

        if status_messages.is_empty() {
            return Err("No input devices could be connected".to_string());
        }

        Ok(status_messages.join(" | "))
    }

    /// Check if any input device is connected
    pub fn is_connected(&self) -> bool {
        let midi_connected = self
            .midi_manager
            .as_ref()
            .map(|m| m.is_connected())
            .unwrap_or(false);

        let gamepad_connected = self
            .gamepad_manager
            .as_ref()
            .map(|g| g.is_connected())
            .unwrap_or(false);

        midi_connected || gamepad_connected
    }

    /// Get connection status for both devices
    pub fn get_status(&self) -> (bool, bool) {
        let midi_connected = self
            .midi_manager
            .as_ref()
            .map(|m| m.is_connected())
            .unwrap_or(false);

        let gamepad_connected = self
            .gamepad_manager
            .as_ref()
            .map(|g| g.is_connected())
            .unwrap_or(false);

        (midi_connected, gamepad_connected)
    }

    /// Disconnect all input devices
    pub fn disconnect(&mut self) {
        if let Some(ref mut midi_mgr) = self.midi_manager {
            midi_mgr.disconnect();
            info!("MIDI device disconnected");
        }

        if let Some(ref mut gamepad_mgr) = self.gamepad_manager {
            gamepad_mgr.disconnect();
            info!("Gamepad device disconnected");
        }
    }

    /// Get current input mode
    pub fn mode(&self) -> InputMode {
        self.mode
    }

    /// Get connected gamepad devices
    ///
    /// Returns information about currently connected gamepads managed by this InputManager.
    /// Returns an empty vector if no gamepad manager is active.
    pub fn get_connected_gamepads(&self) -> Vec<(String, String)> {
        if let Some(ref _gamepad_mgr) = self.gamepad_manager {
            // Query the gamepad manager for connected devices
            if let Ok(gamepads) = Self::list_gamepads() {
                gamepads
                    .into_iter()
                    .map(|(id, name, _uuid)| (format!("{:?}", id), name))
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    /// List available gamepad devices
    ///
    /// Returns a list of (GamepadId, name, UUID) tuples for all detected gamepads.
    pub fn list_gamepads() -> Result<Vec<(gilrs::GamepadId, String, String)>, String> {
        HidDeviceManager::list_gamepads()
    }
}

/// Convert MidiEvent to InputEvent (v3.0 helper)
///
/// Maps MIDI protocol events to protocol-agnostic InputEvents for unified processing.
/// This enables the EventProcessor to handle both MIDI and gamepad inputs identically.
///
/// # Mapping Rules
///
/// - `MidiEvent::NoteOn` → `InputEvent::PadPressed` (velocity, timestamp)
/// - `MidiEvent::NoteOff` → `InputEvent::PadReleased` (timestamp)
/// - `MidiEvent::ControlChange` → `InputEvent::EncoderTurned` (value, timestamp)
/// - `MidiEvent::Aftertouch` → `InputEvent::Aftertouch` (pressure, timestamp)
/// - `MidiEvent::PitchBend` → `InputEvent::PitchBend` (value, timestamp)
///
/// Note: MIDI uses note numbers 0-127, gamepad uses IDs 128-255.
/// This conversion preserves the MIDI range, allowing both to coexist.
fn convert_midi_to_input(midi_event: MidiEvent) -> InputEvent {
    let now = Instant::now();

    match midi_event {
        MidiEvent::NoteOn {
            note, velocity, ..
        } => InputEvent::PadPressed {
            pad: note,
            velocity,
            time: now,
        },
        MidiEvent::NoteOff { note, .. } => InputEvent::PadReleased {
            pad: note,
            time: now,
        },
        MidiEvent::ControlChange { cc, value, .. } => InputEvent::EncoderTurned {
            encoder: cc,
            value,
            time: now,
        },
        MidiEvent::Aftertouch { pressure, .. } => InputEvent::Aftertouch {
            pressure,
            time: now,
        },
        MidiEvent::PitchBend { value, .. } => InputEvent::PitchBend {
            value,
            time: now,
        },
        MidiEvent::PolyPressure { pressure, .. } => InputEvent::Aftertouch {
            pressure,
            time: now,
        },
        MidiEvent::ProgramChange { .. } => {
            // Program Change doesn't have a direct InputEvent mapping
            // For now, ignore it or create a generic InputEvent variant in the future
            InputEvent::PadPressed {
                pad: 0,
                velocity: 0,
                time: now,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_manager_creation_midi_only() {
        let manager = InputManager::new(
            Some("Test Device".to_string()),
            true,
            InputMode::MidiOnly,
        );
        assert!(manager.midi_manager.is_some());
        assert!(manager.gamepad_manager.is_none());
        assert_eq!(manager.mode(), InputMode::MidiOnly);
    }

    #[test]
    fn test_input_manager_creation_gamepad_only() {
        let manager = InputManager::new(None, true, InputMode::GamepadOnly);
        assert!(manager.midi_manager.is_none());
        assert!(manager.gamepad_manager.is_some());
        assert_eq!(manager.mode(), InputMode::GamepadOnly);
    }

    #[test]
    fn test_input_manager_creation_both() {
        let manager = InputManager::new(
            Some("Test Device".to_string()),
            true,
            InputMode::Both,
        );
        assert!(manager.midi_manager.is_some());
        assert!(manager.gamepad_manager.is_some());
        assert_eq!(manager.mode(), InputMode::Both);
    }

    #[test]
    fn test_convert_midi_note_on() {
        let midi_event = MidiEvent::NoteOn {
            note: 60,
            velocity: 100,
            time: Instant::now(),
        };
        let input_event = convert_midi_to_input(midi_event);

        match input_event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                assert_eq!(pad, 60);
                assert_eq!(velocity, 100);
            }
            _ => panic!("Expected PadPressed"),
        }
    }

    #[test]
    fn test_convert_midi_note_off() {
        let midi_event = MidiEvent::NoteOff {
            note: 60,
            time: Instant::now(),
        };
        let input_event = convert_midi_to_input(midi_event);

        match input_event {
            InputEvent::PadReleased { pad, .. } => {
                assert_eq!(pad, 60);
            }
            _ => panic!("Expected PadReleased"),
        }
    }

    #[test]
    fn test_convert_midi_cc() {
        let midi_event = MidiEvent::ControlChange {
            cc: 7,
            value: 64,
            time: Instant::now(),
        };
        let input_event = convert_midi_to_input(midi_event);

        match input_event {
            InputEvent::EncoderTurned { encoder, value, .. } => {
                assert_eq!(encoder, 7);
                assert_eq!(value, 64);
            }
            _ => panic!("Expected EncoderTurned"),
        }
    }
}
