// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! HID Device Management (Game Controllers)
//!
//! This module manages the lifecycle of HID input device connections (gamepads, joysticks,
//! racing wheels, flight sticks, HOTAS controllers) with automatic reconnection support
//! and robust error handling.
//!
//! # Features
//!
//! - **Automatic Reconnection**: Detects disconnections and attempts reconnection
//! - **Thread Safety**: Arc/Mutex patterns for safe concurrent access
//! - **Event Polling**: Continuous polling loop (1ms intervals)
//! - **Device Discovery**: List all connected game controllers
//! - **Connection Lifecycle**: Clean connection/disconnection with proper cleanup
//! - **Multi-Device Support**: Gamepads, joysticks, racing wheels, flight sticks, HOTAS
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  HidDeviceManager                       │
//! │  ┌───────────────────────────────────┐ │
//! │  │  Connection State                 │ │
//! │  │  - gamepad_id                     │ │
//! │  │  - gamepad_name                   │ │
//! │  │  - is_connected                   │ │
//! │  └───────────────────────────────────┘ │
//! │  ┌───────────────────────────────────┐ │
//! │  │  gilrs::Gilrs                     │ │
//! │  │  - Event polling thread           │ │
//! │  │  - Convert to InputEvent          │ │
//! │  │  - Send via mpsc channel          │ │
//! │  └───────────────────────────────────┘ │
//! │  ┌───────────────────────────────────┐ │
//! │  │  Reconnection Logic               │ │
//! │  │  - Detect disconnects             │ │
//! │  │  - Background polling             │ │
//! │  │  - Send DaemonCommand on result   │ │
//! │  └───────────────────────────────────┘ │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use conductor_daemon::gamepad_device::HidDeviceManager;
//! use tokio::sync::mpsc;
//! use conductor_core::events::InputEvent;
//! use conductor_daemon::DaemonCommand;
//!
//! # async fn example() -> Result<(), String> {
//! let (event_tx, mut event_rx) = mpsc::channel::<InputEvent>(1024);
//! let (command_tx, mut command_rx) = mpsc::channel::<DaemonCommand>(32);
//!
//! let mut manager = HidDeviceManager::new(true); // auto_reconnect
//!
//! // Connect to first available game controller
//! let (gamepad_id, gamepad_name) = manager.connect(
//!     event_tx.clone(),
//!     command_tx.clone()
//! )?;
//!
//! println!("Connected to game controller: {} (ID {})", gamepad_name, gamepad_id);
//!
//! // Process events
//! while let Some(event) = event_rx.recv().await {
//!     println!("Received: {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```

use conductor_core::events::InputEvent;
use conductor_core::gamepad_events::{
    axis_changed_to_input, button_pressed_to_input, button_released_to_input,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace, warn};

use crate::daemon::DaemonCommand;

/// Polling interval for gamepad events (1ms)
const POLL_INTERVAL_MS: u64 = 1;

/// Reconnection backoff schedule (in seconds)
///
/// Exponential backoff: 1s, 2s, 4s, 8s, 16s, 30s
const RECONNECT_BACKOFF: &[u64] = &[1, 2, 4, 8, 16, 30];

/// Maximum number of reconnection attempts
const MAX_RECONNECT_ATTEMPTS: usize = 6;

/// HID device manager with automatic reconnection
///
/// Manages the connection to HID input devices (gamepads, joysticks, racing wheels,
/// flight sticks, HOTAS controllers) with automatic reconnection support, continuous
/// event polling, and thread-safe access patterns.
///
/// # Thread Safety
///
/// The manager uses `Arc<Mutex<>>` and `Arc<AtomicBool>` for internal state and
/// is safe to share across threads. The polling loop runs in a separate background thread.
///
/// # Connection Lifecycle
///
/// 1. **Connect**: Find first available game controller and start polling thread
/// 2. **Active**: Poll events at 1ms intervals, convert to InputEvent, send to channel
/// 3. **Disconnect**: Detect disconnection, stop polling, optionally trigger reconnection
/// 4. **Reconnect**: Background thread with exponential backoff
///
/// # Example
///
/// ```no_run
/// use conductor_daemon::gamepad_device::HidDeviceManager;
/// use tokio::sync::mpsc;
///
/// # async fn example() -> Result<(), String> {
/// let (event_tx, _) = mpsc::channel(1024);
/// let (command_tx, _) = mpsc::channel(32);
///
/// let mut manager = HidDeviceManager::new(true);
/// manager.connect(event_tx, command_tx)?;
///
/// assert!(manager.is_connected());
/// # Ok(())
/// # }
/// ```
pub struct HidDeviceManager {
    /// Whether to automatically reconnect on disconnect
    auto_reconnect: bool,

    /// Currently connected gamepad ID
    gamepad_id: Arc<Mutex<Option<gilrs::GamepadId>>>,

    /// Currently connected gamepad name
    gamepad_name: Arc<Mutex<Option<String>>>,

    /// Whether currently connected
    is_connected: Arc<AtomicBool>,

    /// Flag to signal polling thread to stop
    stop_polling: Arc<AtomicBool>,

    /// Handle to polling thread (if active)
    polling_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

/// Type alias for backward compatibility (v3.0)
///
/// Use `HidDeviceManager` for new code. This alias ensures existing code
/// using `GamepadDeviceManager` continues to work without modification.
pub type GamepadDeviceManager = HidDeviceManager;

impl HidDeviceManager {
    /// Create a new HID device manager
    ///
    /// # Arguments
    ///
    /// * `auto_reconnect` - Whether to automatically reconnect on disconnect
    ///
    /// # Returns
    ///
    /// A new HidDeviceManager instance
    ///
    /// # Example
    ///
    /// ```
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    ///
    /// let manager = HidDeviceManager::new(true);
    /// assert!(!manager.is_connected());
    /// ```
    pub fn new(auto_reconnect: bool) -> Self {
        Self {
            auto_reconnect,
            gamepad_id: Arc::new(Mutex::new(None)),
            gamepad_name: Arc::new(Mutex::new(None)),
            is_connected: Arc::new(AtomicBool::new(false)),
            stop_polling: Arc::new(AtomicBool::new(false)),
            polling_thread: Arc::new(Mutex::new(None)),
        }
    }

    /// List all connected game controllers
    ///
    /// Returns a list of (ID, Name, UUID) tuples for all connected HID game controllers.
    ///
    /// # Returns
    ///
    /// Vector of (GamepadId, Name, UUID) for each connected game controller
    ///
    /// # Errors
    ///
    /// Returns error string if gilrs initialization fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    ///
    /// # fn example() -> Result<(), String> {
    /// let gamepads = HidDeviceManager::list_gamepads()?;
    /// for (id, name, uuid) in gamepads {
    ///     println!("Game controller: {} (ID: {:?}, UUID: {})", name, id, uuid);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_gamepads() -> Result<Vec<(gilrs::GamepadId, String, String)>, String> {
        let gilrs = gilrs::Gilrs::new().map_err(|e| format!("Failed to initialize gilrs: {}", e))?;

        let mut gamepads = Vec::new();
        for (id, gamepad) in gilrs.gamepads() {
            let name = gamepad.name().to_string();
            let uuid = format!("{:?}", gamepad.uuid());
            gamepads.push((id, name, uuid));
        }

        Ok(gamepads)
    }

    /// Connect to the first available game controller
    ///
    /// Finds the first connected HID game controller and starts the polling loop.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel sender for InputEvent messages
    /// * `command_tx` - Channel sender for DaemonCommand messages (reconnection, etc.)
    ///
    /// # Returns
    ///
    /// Tuple of (GamepadId, Name) for the connected game controller
    ///
    /// # Errors
    ///
    /// Returns error string if:
    /// - gilrs initialization fails
    /// - No game controllers are connected
    /// - Already connected to a game controller
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = HidDeviceManager::new(true);
    /// let (id, name) = manager.connect(event_tx, command_tx)?;
    /// println!("Connected to: {}", name);
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect(
        &mut self,
        event_tx: mpsc::Sender<InputEvent>,
        command_tx: mpsc::Sender<DaemonCommand>,
    ) -> Result<(gilrs::GamepadId, String), String> {
        if self.is_connected.load(Ordering::Relaxed) {
            return Err("Already connected to a game controller".to_string());
        }

        // Initialize gilrs
        let gilrs =
            gilrs::Gilrs::new().map_err(|e| format!("Failed to initialize gilrs: {}", e))?;

        // Find first connected game controller
        let (gamepad_id, gamepad_name) = gilrs
            .gamepads()
            .next()
            .map(|(id, gamepad)| (id, gamepad.name().to_string()))
            .ok_or_else(|| "No game controllers connected".to_string())?;

        info!("Connecting to game controller: {} (ID: {:?})", gamepad_name, gamepad_id);

        // Store connection info
        *self.gamepad_id.lock().unwrap() = Some(gamepad_id);
        *self.gamepad_name.lock().unwrap() = Some(gamepad_name.clone());
        self.is_connected.store(true, Ordering::Relaxed);
        self.stop_polling.store(false, Ordering::Relaxed);

        // Start polling thread
        let stop_polling = Arc::clone(&self.stop_polling);
        let is_connected = Arc::clone(&self.is_connected);
        let gamepad_id_clone = Arc::clone(&self.gamepad_id);
        let auto_reconnect = self.auto_reconnect;

        let polling_handle = thread::spawn(move || {
            Self::polling_loop(
                gilrs,
                gamepad_id,
                event_tx,
                command_tx,
                stop_polling,
                is_connected,
                gamepad_id_clone,
                auto_reconnect,
            );
        });

        *self.polling_thread.lock().unwrap() = Some(polling_handle);

        info!("Game controller polling thread started");

        Ok((gamepad_id, gamepad_name))
    }

    /// Polling loop for HID game controller events (runs in background thread)
    ///
    /// Continuously polls gilrs for events, converts them to InputEvent,
    /// and sends them to the event channel. Handles disconnection and
    /// triggers reconnection if enabled.
    fn polling_loop(
        mut gilrs: gilrs::Gilrs,
        initial_gamepad_id: gilrs::GamepadId,
        event_tx: mpsc::Sender<InputEvent>,
        command_tx: mpsc::Sender<DaemonCommand>,
        stop_polling: Arc<AtomicBool>,
        is_connected: Arc<AtomicBool>,
        gamepad_id: Arc<Mutex<Option<gilrs::GamepadId>>>,
        auto_reconnect: bool,
    ) {
        let current_gamepad_id = initial_gamepad_id;

        loop {
            // Check stop signal
            if stop_polling.load(Ordering::Relaxed) {
                debug!("HID device polling loop stopped");
                break;
            }

            // Poll for events
            while let Some(event) = gilrs.next_event() {
                trace!("HID device event: {:?}", event);

                // Check if this event is from our connected game controller
                if event.id != current_gamepad_id {
                    continue;
                }

                match event.event {
                    gilrs::EventType::ButtonPressed(button, _code) => {
                        let input_event = button_pressed_to_input(button);
                        if let Err(e) = event_tx.try_send(input_event) {
                            warn!("Failed to send button press event: {}", e);
                        }
                    }
                    gilrs::EventType::ButtonReleased(button, _code) => {
                        let input_event = button_released_to_input(button);
                        if let Err(e) = event_tx.try_send(input_event) {
                            warn!("Failed to send button release event: {}", e);
                        }
                    }
                    gilrs::EventType::AxisChanged(axis, value, _code) => {
                        let input_event = axis_changed_to_input(axis, value);
                        if let Err(e) = event_tx.try_send(input_event) {
                            warn!("Failed to send axis change event: {}", e);
                        }
                    }
                    gilrs::EventType::Disconnected => {
                        warn!("Game controller disconnected: {:?}", current_gamepad_id);
                        is_connected.store(false, Ordering::Relaxed);
                        *gamepad_id.lock().unwrap() = None;

                        if auto_reconnect {
                            info!("Triggering reconnection...");
                            // Spawn reconnection thread
                            let command_tx_clone = command_tx.clone();
                            thread::spawn(move || {
                                Self::reconnect_loop(command_tx_clone);
                            });
                        }

                        // Exit polling loop
                        return;
                    }
                    _ => {
                        // Ignore other events (Connected, etc.)
                    }
                }
            }

            // Sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        }
    }

    /// Reconnection loop with exponential backoff
    ///
    /// Attempts to reconnect to a game controller with exponential backoff.
    /// Sends a DaemonCommand::ReconnectGamepad when a device becomes available.
    fn reconnect_loop(command_tx: mpsc::Sender<DaemonCommand>) {
        for (attempt, &delay) in RECONNECT_BACKOFF.iter().enumerate() {
            info!("Reconnection attempt {} in {}s...", attempt + 1, delay);
            thread::sleep(Duration::from_secs(delay));

            // Try to list game controllers
            match Self::list_gamepads() {
                Ok(gamepads) if !gamepads.is_empty() => {
                    info!("Game controller detected, sending reconnect command");
                    let (id, name, _uuid) = &gamepads[0];
                    debug!("Found game controller: {} (ID: {:?})", name, id);

                    // Send reconnect command
                    if let Err(e) = command_tx.try_send(DaemonCommand::ReconnectGamepad) {
                        error!("Failed to send reconnect command: {}", e);
                    }
                    return;
                }
                Ok(_) => {
                    debug!("No game controllers found, continuing backoff...");
                }
                Err(e) => {
                    error!("Error checking for game controllers: {}", e);
                }
            }
        }

        error!(
            "Failed to reconnect after {} attempts",
            MAX_RECONNECT_ATTEMPTS
        );
    }

    /// Disconnect from the current game controller
    ///
    /// Stops the polling thread and cleans up the connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = HidDeviceManager::new(true);
    /// manager.connect(event_tx, command_tx)?;
    /// manager.disconnect();
    ///
    /// assert!(!manager.is_connected());
    /// # Ok(())
    /// # }
    /// ```
    pub fn disconnect(&mut self) {
        if !self.is_connected.load(Ordering::Relaxed) {
            return;
        }

        info!("Disconnecting from game controller");

        // Signal polling thread to stop
        self.stop_polling.store(true, Ordering::Relaxed);

        // Wait for polling thread to finish
        if let Some(handle) = self.polling_thread.lock().unwrap().take() {
            let _ = handle.join();
        }

        // Clear connection state
        *self.gamepad_id.lock().unwrap() = None;
        *self.gamepad_name.lock().unwrap() = None;
        self.is_connected.store(false, Ordering::Relaxed);

        info!("Disconnected from game controller");
    }

    /// Check if currently connected to a game controller
    ///
    /// # Returns
    ///
    /// true if connected, false otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    ///
    /// let manager = HidDeviceManager::new(true);
    /// assert!(!manager.is_connected());
    /// ```
    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::Relaxed)
    }

    /// Get the current game controller name
    ///
    /// # Returns
    ///
    /// Some(name) if connected, None otherwise
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::gamepad_device::HidDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = HidDeviceManager::new(true);
    /// manager.connect(event_tx, command_tx)?;
    ///
    /// if let Some(name) = manager.get_gamepad_name() {
    ///     println!("Connected to: {}", name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_gamepad_name(&self) -> Option<String> {
        self.gamepad_name.lock().unwrap().clone()
    }
}

impl Drop for HidDeviceManager {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager() {
        let manager = GamepadDeviceManager::new(true);
        assert!(!manager.is_connected());
        assert!(manager.get_gamepad_name().is_none());
    }

    #[test]
    fn test_list_gamepads() {
        // This may pass or fail depending on whether game controllers are connected
        let result = HidDeviceManager::list_gamepads();
        assert!(result.is_ok(), "list_gamepads should not error");

        if let Ok(gamepads) = result {
            println!("Found {} game controllers", gamepads.len());
            for (id, name, uuid) in gamepads {
                println!("  - {} (ID: {:?}, UUID: {})", name, id, uuid);
            }
        }
    }

    #[test]
    fn test_manager_drop() {
        let manager = GamepadDeviceManager::new(false);
        drop(manager); // Should not panic
    }
}
