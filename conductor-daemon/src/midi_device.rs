// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDI Device Management
//!
//! This module manages the lifecycle of MIDI device connections with automatic
//! reconnection support and robust error handling.
//!
//! # Features
//!
//! - **Automatic Reconnection**: Exponential backoff with configurable max attempts
//! - **Thread Safety**: Arc/Mutex patterns for safe concurrent access
//! - **Non-blocking Callbacks**: Uses try_send to prevent callback blocking
//! - **Device Discovery**: Find devices by name or use first available
//! - **Connection Lifecycle**: Clean connection/disconnection with proper cleanup
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │  MidiDeviceManager                      │
//! │  ┌───────────────────────────────────┐ │
//! │  │  Connection State                 │ │
//! │  │  - device_name                    │ │
//! │  │  - port_index                     │ │
//! │  │  - is_connected                   │ │
//! │  └───────────────────────────────────┘ │
//! │  ┌───────────────────────────────────┐ │
//! │  │  midir::MidiInputConnection       │ │
//! │  │  - Callback thread                │ │
//! │  │  - Parse with midi-msg            │ │
//! │  │  - Send MidiEvent via mpsc        │ │
//! │  └───────────────────────────────────┘ │
//! │  ┌───────────────────────────────────┐ │
//! │  │  Reconnection Logic               │ │
//! │  │  - Exponential backoff            │ │
//! │  │  - Background thread              │ │
//! │  │  - Send DaemonCommand on result   │ │
//! │  └───────────────────────────────────┘ │
//! └─────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use conductor_daemon::midi_device::MidiDeviceManager;
//! use tokio::sync::mpsc;
//! use conductor_core::event_processor::MidiEvent;
//! use conductor_daemon::DaemonCommand;
//!
//! # async fn example() -> Result<(), String> {
//! let (event_tx, mut event_rx) = mpsc::channel::<MidiEvent>(1024);
//! let (command_tx, mut command_rx) = mpsc::channel::<DaemonCommand>(32);
//!
//! let mut manager = MidiDeviceManager::new(
//!     "Maschine Mikro MK3".to_string(),
//!     true  // auto_reconnect
//! );
//!
//! // Connect to device
//! let (port_index, port_name) = manager.connect(
//!     event_tx.clone(),
//!     command_tx.clone()
//! )?;
//!
//! println!("Connected to device: {} (port {})", port_name, port_index);
//!
//! // Process events
//! while let Some(event) = event_rx.recv().await {
//!     println!("Received: {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```

use conductor_core::event_processor::MidiEvent;
use midir::{MidiInput, MidiInputConnection, MidiInputPort};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, trace, warn};

use crate::daemon::DaemonCommand;

/// Reconnection backoff schedule (in seconds)
///
/// Exponential backoff: 1s, 2s, 4s, 8s, 16s, 30s
const RECONNECT_BACKOFF: &[u64] = &[1, 2, 4, 8, 16, 30];

/// Maximum number of reconnection attempts
const MAX_RECONNECT_ATTEMPTS: usize = 6;

/// MIDI device manager with automatic reconnection
///
/// Manages the connection to a MIDI input device with automatic reconnection
/// support, exponential backoff, and thread-safe access patterns.
///
/// # Thread Safety
///
/// The manager uses `Arc<Mutex<>>` for internal state and is safe to share
/// across threads. The MIDI callback runs in a separate thread managed by midir.
///
/// # Connection Lifecycle
///
/// 1. **Connect**: Find device by name (or first available) and establish connection
/// 2. **Active**: Process MIDI events via callback, send to mpsc channel
/// 3. **Disconnect**: Clean up connection, optionally trigger reconnection
/// 4. **Reconnect**: Background thread with exponential backoff
///
/// # Example
///
/// ```no_run
/// use conductor_daemon::midi_device::MidiDeviceManager;
/// use tokio::sync::mpsc;
///
/// # async fn example() -> Result<(), String> {
/// let (event_tx, _) = mpsc::channel(1024);
/// let (command_tx, _) = mpsc::channel(32);
///
/// let mut manager = MidiDeviceManager::new("My Device".to_string(), true);
/// manager.connect(event_tx, command_tx)?;
///
/// assert!(manager.is_connected());
/// # Ok(())
/// # }
/// ```
pub struct MidiDeviceManager {
    /// Name of the device to connect to (or empty for first available)
    device_name: String,

    /// Whether to automatically reconnect on disconnect
    auto_reconnect: bool,

    /// Active MIDI input connection (None if disconnected)
    connection: Arc<Mutex<Option<MidiInputConnection<()>>>>,

    /// Currently connected port index
    port_index: Arc<Mutex<Option<usize>>>,

    /// Currently connected port name
    port_name: Arc<Mutex<Option<String>>>,

    /// Whether currently connected
    is_connected: Arc<Mutex<bool>>,
}

impl MidiDeviceManager {
    /// Create a new MIDI device manager
    ///
    /// # Arguments
    ///
    /// * `device_name` - Name of device to connect to (empty = first available)
    /// * `auto_reconnect` - Whether to automatically reconnect on disconnect
    ///
    /// # Example
    ///
    /// ```
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    ///
    /// // Connect to specific device with auto-reconnect
    /// let manager = MidiDeviceManager::new("Maschine Mikro MK3".to_string(), true);
    ///
    /// // Connect to first available device without auto-reconnect
    /// let manager = MidiDeviceManager::new(String::new(), false);
    /// ```
    pub fn new(device_name: String, auto_reconnect: bool) -> Self {
        Self {
            device_name,
            auto_reconnect,
            connection: Arc::new(Mutex::new(None)),
            port_index: Arc::new(Mutex::new(None)),
            port_name: Arc::new(Mutex::new(None)),
            is_connected: Arc::new(Mutex::new(false)),
        }
    }

    /// Connect to MIDI device and start event processing
    ///
    /// Attempts to find the configured device by name, or uses the first available
    /// device if no name is set. Creates a midir callback that parses MIDI messages
    /// and sends them to the event channel.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel for sending parsed MIDI events
    /// * `command_tx` - Channel for sending reconnection commands
    ///
    /// # Returns
    ///
    /// * `Ok((port_index, port_name))` - Successfully connected
    /// * `Err(String)` - Connection failed with error message
    ///
    /// # Errors
    ///
    /// - No MIDI input ports available
    /// - Named device not found (falls back to first port with warning)
    /// - Failed to create MIDI input
    /// - Failed to establish connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new("My Device".to_string(), true);
    /// let (port_idx, port_name) = manager.connect(event_tx, command_tx)?;
    ///
    /// println!("Connected to port {}: {}", port_idx, port_name);
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect(
        &mut self,
        event_tx: mpsc::Sender<MidiEvent>,
        _command_tx: mpsc::Sender<DaemonCommand>,
    ) -> Result<(usize, String), String> {
        info!(
            "Connecting to MIDI device: {}",
            if self.device_name.is_empty() {
                "[first available]"
            } else {
                &self.device_name
            }
        );

        // Create MIDI input
        let midi_in = MidiInput::new("MIDIMon Daemon")
            .map_err(|e| format!("Failed to create MIDI input: {}", e))?;

        // Get available ports
        let ports = midi_in.ports();
        if ports.is_empty() {
            return Err("No MIDI input ports available".to_string());
        }

        // Find target port
        let (port, port_index) = self.find_port(&midi_in, &ports)?;
        let port_name = midi_in
            .port_name(&port)
            .unwrap_or_else(|_| format!("Port {}", port_index));

        debug!("Opening MIDI port {} (index {})", port_name, port_index);

        // Create callback that parses and sends events
        let callback = move |_timestamp: u64, message: &[u8], _: &mut ()| {
            trace!(
                "MIDI callback received {} bytes: {:02X?}",
                message.len(),
                message
            );

            // Parse MIDI message using midi-msg library
            match MidiEvent::from_midi_msg(message) {
                Ok(event) => {
                    // Non-blocking send to prevent callback blocking
                    if let Err(e) = event_tx.try_send(event.clone()) {
                        match e {
                            mpsc::error::TrySendError::Full(_) => {
                                warn!("Event channel full, dropping event: {:?}", event);
                            }
                            mpsc::error::TrySendError::Closed(_) => {
                                error!("Event channel closed, cannot send event");
                            }
                        }
                    } else {
                        trace!("Successfully sent event: {:?}", event);
                    }
                }
                Err(e) => {
                    // Log parsing errors at debug level (many MIDI messages are unsupported)
                    debug!(
                        "Failed to parse MIDI message: {} (bytes: {:02X?})",
                        e, message
                    );
                }
            }
        };

        // Establish connection
        let connection = midi_in
            .connect(&port, &format!("midimon-{}", port_index), callback, ())
            .map_err(|e| format!("Failed to connect to port: {}", e))?;

        // Update state
        *self.connection.lock().unwrap() = Some(connection);
        *self.port_index.lock().unwrap() = Some(port_index);
        *self.port_name.lock().unwrap() = Some(port_name.clone());
        *self.is_connected.lock().unwrap() = true;

        info!(
            "Successfully connected to MIDI device: {} (port {})",
            port_name, port_index
        );

        Ok((port_index, port_name))
    }

    /// Connect to a specific MIDI port by index
    ///
    /// Similar to `connect()` but bypasses device name search and connects
    /// directly to the specified port index.
    ///
    /// # Arguments
    ///
    /// * `port_index` - Explicit port index to connect to
    /// * `event_tx` - Channel for sending parsed MIDI events
    /// * `command_tx` - Channel for sending reconnection commands
    ///
    /// # Returns
    ///
    /// * `Ok((port_index, port_name))` - Successfully connected
    /// * `Err(String)` - Connection failed with error message
    ///
    /// # Errors
    ///
    /// - Port index out of range
    /// - Failed to create MIDI input
    /// - Failed to establish connection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new(String::new(), true);
    /// let (port_idx, port_name) = manager.connect_to_port(2, event_tx, command_tx)?;
    ///
    /// println!("Connected to port {}: {}", port_idx, port_name);
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect_to_port(
        &mut self,
        port_index: usize,
        event_tx: mpsc::Sender<MidiEvent>,
        _command_tx: mpsc::Sender<DaemonCommand>,
    ) -> Result<(usize, String), String> {
        info!("Connecting to MIDI port index {}", port_index);

        // Create MIDI input
        let midi_in = MidiInput::new("MIDIMon Daemon")
            .map_err(|e| format!("Failed to create MIDI input: {}", e))?;

        // Get available ports
        let ports = midi_in.ports();
        if ports.is_empty() {
            return Err("No MIDI input ports available".to_string());
        }

        // Validate port index
        if port_index >= ports.len() {
            return Err(format!(
                "Port index {} out of range (0-{})",
                port_index,
                ports.len() - 1
            ));
        }

        // Get target port
        let port = &ports[port_index];
        let port_name = midi_in
            .port_name(port)
            .unwrap_or_else(|_| format!("Port {}", port_index));

        debug!("Opening MIDI port {} (index {})", port_name, port_index);

        // Create callback that parses and sends events
        let callback = move |_timestamp: u64, message: &[u8], _: &mut ()| {
            trace!(
                "MIDI callback received {} bytes: {:02X?}",
                message.len(),
                message
            );

            // Parse MIDI message using midi-msg library
            match MidiEvent::from_midi_msg(message) {
                Ok(event) => {
                    // Non-blocking send to prevent callback blocking
                    if let Err(e) = event_tx.try_send(event.clone()) {
                        match e {
                            mpsc::error::TrySendError::Full(_) => {
                                warn!("Event channel full, dropping event: {:?}", event);
                            }
                            mpsc::error::TrySendError::Closed(_) => {
                                error!("Event channel closed, cannot send event");
                            }
                        }
                    } else {
                        trace!("Successfully sent event: {:?}", event);
                    }
                }
                Err(e) => {
                    // Log parsing errors at debug level (many MIDI messages are unsupported)
                    debug!(
                        "Failed to parse MIDI message: {} (bytes: {:02X?})",
                        e, message
                    );
                }
            }
        };

        // Establish connection
        let connection = midi_in
            .connect(port, &format!("midimon-{}", port_index), callback, ())
            .map_err(|e| format!("Failed to connect to port: {}", e))?;

        // Update state
        *self.connection.lock().unwrap() = Some(connection);
        *self.port_index.lock().unwrap() = Some(port_index);
        *self.port_name.lock().unwrap() = Some(port_name.clone());
        *self.is_connected.lock().unwrap() = true;

        info!(
            "Successfully connected to MIDI device: {} (port {})",
            port_name, port_index
        );

        Ok((port_index, port_name))
    }

    /// Find the target MIDI port by name or use first available
    ///
    /// # Arguments
    ///
    /// * `midi_in` - MIDI input instance for querying port names
    /// * `ports` - List of available MIDI ports
    ///
    /// # Returns
    ///
    /// * `Ok((port, index))` - Found port and its index
    /// * `Err(String)` - No ports available (should never happen, checked in connect)
    ///
    /// # Behavior
    ///
    /// - If device_name is empty: Use first port
    /// - If device_name is set: Search by exact match
    /// - If named device not found: Warn and fall back to first port
    fn find_port(
        &self,
        midi_in: &MidiInput,
        ports: &[MidiInputPort],
    ) -> Result<(MidiInputPort, usize), String> {
        if self.device_name.is_empty() {
            // Use first available port
            debug!("No device name specified, using first available port");
            return Ok((ports[0].clone(), 0));
        }

        // Search for named device
        for (i, port) in ports.iter().enumerate() {
            if let Ok(name) = midi_in.port_name(port)
                && name.contains(&self.device_name)
            {
                debug!("Found matching port: {} (index {})", name, i);
                return Ok((port.clone(), i));
            }
        }

        // Device not found, fall back to first port with warning
        warn!(
            "Device '{}' not found, falling back to first port",
            self.device_name
        );

        if let Ok(fallback_name) = midi_in.port_name(&ports[0]) {
            warn!("Using fallback device: {}", fallback_name);
        }

        Ok((ports[0].clone(), 0))
    }

    /// Disconnect from MIDI device
    ///
    /// Cleanly closes the MIDI connection and updates internal state.
    /// Does not trigger reconnection logic.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new("My Device".to_string(), false);
    /// manager.connect(event_tx, command_tx)?;
    ///
    /// // Later...
    /// manager.disconnect();
    /// assert!(!manager.is_connected());
    /// # Ok(())
    /// # }
    /// ```
    pub fn disconnect(&mut self) {
        info!("Disconnecting from MIDI device");

        // Close connection
        if let Some(conn) = self.connection.lock().unwrap().take() {
            // Connection is dropped here, which closes it
            drop(conn);
            debug!("MIDI connection closed");
        }

        // Clear state
        *self.port_index.lock().unwrap() = None;
        *self.port_name.lock().unwrap() = None;
        *self.is_connected.lock().unwrap() = false;

        info!("MIDI device disconnected");
    }

    /// Spawn background reconnection thread with exponential backoff
    ///
    /// Creates a background thread that attempts to reconnect to the MIDI device
    /// with exponential backoff. Sends `DaemonCommand::DeviceReconnected` on success
    /// or `DaemonCommand::DeviceReconnectionFailed` after max attempts.
    ///
    /// # Arguments
    ///
    /// * `event_tx` - Channel for sending MIDI events after reconnection
    /// * `command_tx` - Channel for sending reconnection status commands
    ///
    /// # Backoff Schedule
    ///
    /// Attempts reconnection with increasing delays:
    /// - Attempt 1: 1 second
    /// - Attempt 2: 2 seconds
    /// - Attempt 3: 4 seconds
    /// - Attempt 4: 8 seconds
    /// - Attempt 5: 16 seconds
    /// - Attempt 6: 30 seconds
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, mut command_rx) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new("My Device".to_string(), true);
    ///
    /// // Simulate disconnect, spawn reconnection
    /// manager.spawn_reconnection_thread(event_tx, command_tx);
    ///
    /// // Wait for reconnection result
    /// while let Some(cmd) = command_rx.recv().await {
    ///     match cmd {
    ///         conductor_daemon::DaemonCommand::DeviceReconnected => {
    ///             println!("Device reconnected!");
    ///             break;
    ///         }
    ///         conductor_daemon::DaemonCommand::DeviceReconnectionFailed => {
    ///             println!("Reconnection failed after max attempts");
    ///             break;
    ///         }
    ///         _ => {}
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn spawn_reconnection_thread(
        &self,
        event_tx: mpsc::Sender<MidiEvent>,
        command_tx: mpsc::Sender<DaemonCommand>,
    ) {
        if !self.auto_reconnect {
            debug!("Auto-reconnect disabled, not spawning reconnection thread");
            return;
        }

        info!("Spawning reconnection thread with exponential backoff");

        let device_name = self.device_name.clone();
        let connection = Arc::clone(&self.connection);
        let port_index = Arc::clone(&self.port_index);
        let port_name = Arc::clone(&self.port_name);
        let is_connected = Arc::clone(&self.is_connected);

        thread::spawn(move || {
            for (attempt, &delay_secs) in RECONNECT_BACKOFF.iter().enumerate() {
                let attempt_num = attempt + 1;
                info!(
                    "Reconnection attempt {}/{} (waiting {}s)...",
                    attempt_num, MAX_RECONNECT_ATTEMPTS, delay_secs
                );

                thread::sleep(Duration::from_secs(delay_secs));

                // Attempt reconnection
                match Self::try_reconnect(&device_name, event_tx.clone()) {
                    Ok((new_port_idx, new_port_name, new_connection)) => {
                        // Update state
                        *connection.lock().unwrap() = Some(new_connection);
                        *port_index.lock().unwrap() = Some(new_port_idx);
                        *port_name.lock().unwrap() = Some(new_port_name.clone());
                        *is_connected.lock().unwrap() = true;

                        info!(
                            "Successfully reconnected to device: {} (port {})",
                            new_port_name, new_port_idx
                        );

                        // Notify daemon of successful reconnection
                        if let Err(e) = command_tx.blocking_send(DaemonCommand::DeviceReconnected) {
                            error!("Failed to send DeviceReconnected command: {}", e);
                        }
                        return;
                    }
                    Err(e) => {
                        warn!(
                            "Reconnection attempt {}/{} failed: {}",
                            attempt_num, MAX_RECONNECT_ATTEMPTS, e
                        );
                    }
                }
            }

            // All attempts exhausted
            error!(
                "Failed to reconnect after {} attempts",
                MAX_RECONNECT_ATTEMPTS
            );

            // Notify daemon of reconnection failure
            if let Err(e) = command_tx.blocking_send(DaemonCommand::DeviceReconnectionFailed) {
                error!("Failed to send DeviceReconnectionFailed command: {}", e);
            }
        });
    }

    /// Attempt a single reconnection to the MIDI device
    ///
    /// Internal helper for reconnection thread. Creates a new MIDI input,
    /// finds the target device, and establishes a connection.
    ///
    /// # Arguments
    ///
    /// * `device_name` - Name of device to connect to (empty = first available)
    /// * `event_tx` - Channel for sending MIDI events
    ///
    /// # Returns
    ///
    /// * `Ok((port_index, port_name, connection))` - Successful reconnection
    /// * `Err(String)` - Connection failed
    fn try_reconnect(
        device_name: &str,
        event_tx: mpsc::Sender<MidiEvent>,
    ) -> Result<(usize, String, MidiInputConnection<()>), String> {
        debug!("Attempting to reconnect to MIDI device: {}", device_name);

        // Create MIDI input
        let midi_in = MidiInput::new("MIDIMon Daemon Reconnect")
            .map_err(|e| format!("Failed to create MIDI input: {}", e))?;

        // Get available ports
        let ports = midi_in.ports();
        if ports.is_empty() {
            return Err("No MIDI input ports available".to_string());
        }

        // Find target port
        let (port, port_index) = if device_name.is_empty() {
            (ports[0].clone(), 0)
        } else {
            // Search for named device
            let mut found = None;
            for (i, p) in ports.iter().enumerate() {
                if let Ok(name) = midi_in.port_name(p)
                    && name.contains(device_name)
                {
                    found = Some((p.clone(), i));
                    break;
                }
            }

            // Fall back to first port if not found
            found.unwrap_or_else(|| {
                debug!(
                    "Device '{}' not found during reconnect, using first port",
                    device_name
                );
                (ports[0].clone(), 0)
            })
        };

        let port_name = midi_in
            .port_name(&port)
            .unwrap_or_else(|_| format!("Port {}", port_index));

        // Create callback
        let callback = move |_timestamp: u64, message: &[u8], _: &mut ()| {
            trace!(
                "MIDI callback (reconnected) received {} bytes: {:02X?}",
                message.len(),
                message
            );

            match MidiEvent::from_midi_msg(message) {
                Ok(event) => {
                    if let Err(e) = event_tx.try_send(event.clone()) {
                        match e {
                            mpsc::error::TrySendError::Full(_) => {
                                warn!("Event channel full, dropping event: {:?}", event);
                            }
                            mpsc::error::TrySendError::Closed(_) => {
                                error!("Event channel closed, cannot send event");
                            }
                        }
                    } else {
                        trace!("Successfully sent event: {:?}", event);
                    }
                }
                Err(e) => {
                    debug!(
                        "Failed to parse MIDI message: {} (bytes: {:02X?})",
                        e, message
                    );
                }
            }
        };

        // Establish connection
        let connection = midi_in
            .connect(&port, &format!("midimon-{}", port_index), callback, ())
            .map_err(|e| format!("Failed to connect to port: {}", e))?;

        Ok((port_index, port_name, connection))
    }

    /// Check if currently connected to a MIDI device
    ///
    /// # Returns
    ///
    /// `true` if connected, `false` otherwise
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    ///
    /// let manager = MidiDeviceManager::new("My Device".to_string(), false);
    /// assert!(!manager.is_connected());
    /// ```
    pub fn is_connected(&self) -> bool {
        *self.is_connected.lock().unwrap()
    }

    /// Get current device information
    ///
    /// Returns the port index and name of the currently connected device,
    /// or `None` if not connected.
    ///
    /// # Returns
    ///
    /// * `Some((port_index, port_name))` - Currently connected device
    /// * `None` - Not connected
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new("My Device".to_string(), false);
    /// manager.connect(event_tx, command_tx)?;
    ///
    /// if let Some((idx, name)) = manager.device_info() {
    ///     println!("Connected to port {}: {}", idx, name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn device_info(&self) -> Option<(usize, String)> {
        let port_index = self.port_index.lock().unwrap();
        let port_name = self.port_name.lock().unwrap();

        match (*port_index, port_name.as_ref()) {
            (Some(idx), Some(name)) => Some((idx, name.clone())),
            _ => None,
        }
    }

    /// Graceful shutdown
    ///
    /// Disconnects from the device and cleans up all resources.
    /// This is the recommended way to stop the device manager.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use conductor_daemon::midi_device::MidiDeviceManager;
    /// use tokio::sync::mpsc;
    ///
    /// # async fn example() -> Result<(), String> {
    /// let (event_tx, _) = mpsc::channel(1024);
    /// let (command_tx, _) = mpsc::channel(32);
    ///
    /// let mut manager = MidiDeviceManager::new("My Device".to_string(), false);
    /// manager.connect(event_tx, command_tx)?;
    ///
    /// // Later, during shutdown...
    /// manager.shutdown();
    /// # Ok(())
    /// # }
    /// ```
    pub fn shutdown(&mut self) {
        info!("Shutting down MIDI device manager");
        self.disconnect();
        info!("MIDI device manager shutdown complete");
    }
}

impl Drop for MidiDeviceManager {
    /// Ensure clean shutdown on drop
    ///
    /// If the manager is still connected when dropped, this will
    /// disconnect cleanly to prevent resource leaks.
    fn drop(&mut self) {
        if self.is_connected() {
            warn!("MidiDeviceManager dropped while still connected, forcing disconnect");
            self.disconnect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager() {
        let manager = MidiDeviceManager::new("Test Device".to_string(), true);
        assert!(!manager.is_connected());
        assert_eq!(manager.device_info(), None);
    }

    #[test]
    fn test_new_manager_empty_name() {
        let manager = MidiDeviceManager::new(String::new(), false);
        assert!(!manager.is_connected());
    }

    #[test]
    fn test_reconnect_backoff_schedule() {
        // Verify backoff schedule is correct
        assert_eq!(RECONNECT_BACKOFF, &[1, 2, 4, 8, 16, 30]);
        assert_eq!(MAX_RECONNECT_ATTEMPTS, 6);
    }

    #[tokio::test]
    async fn test_disconnect_when_not_connected() {
        let mut manager = MidiDeviceManager::new("Test".to_string(), false);
        // Should not panic
        manager.disconnect();
        assert!(!manager.is_connected());
    }

    #[tokio::test]
    async fn test_shutdown_when_not_connected() {
        let mut manager = MidiDeviceManager::new("Test".to_string(), false);
        // Should not panic
        manager.shutdown();
        assert!(!manager.is_connected());
    }
}
