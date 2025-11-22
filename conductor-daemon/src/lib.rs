// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Daemon - Background service for MIDI controller mapping
//!
//! This crate provides the daemon infrastructure for running MIDIMon as a background service
//! with config hot-reload, IPC control, state persistence, and lifecycle management.
//!
//! # Architecture
//!
//! The daemon follows a modular architecture with clear separation of concerns:
//!
//! ```text
//! ┌──────────────────────────────────────────────────┐
//! │  midimonctl (CLI)                                │
//! │  - status, reload, stop, validate, ping          │
//! └────────────┬─────────────────────────────────────┘
//!              │ IPC (JSON over Unix socket)
//!              ▼
//! ┌──────────────────────────────────────────────────┐
//! │  midimon-daemon Service                          │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  IPC Server                                │ │
//! │  │  - Accept connections                      │ │
//! │  │  - Route commands                          │ │
//! │  └──────────┬─────────────────────────────────┘ │
//! │             ▼                                    │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  Engine Manager                            │ │
//! │  │  - Lifecycle management                    │ │
//! │  │  - Atomic config swaps (Arc<RwLock<>>)     │ │
//! │  │  - Performance metrics                     │ │
//! │  └──────────┬─────────────────────────────────┘ │
//! │             ▼                                    │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  Config Watcher                            │ │
//! │  │  - File system monitoring                  │ │
//! │  │  - 500ms debounce                          │ │
//! │  └────────────────────────────────────────────┘ │
//! │                                                  │
//! │  ┌────────────────────────────────────────────┐ │
//! │  │  State Manager                             │ │
//! │  │  - Atomic persistence                      │ │
//! │  │  - Emergency save handler                  │ │
//! │  └────────────────────────────────────────────┘ │
//! └──────────────────────────────────────────────────┘
//!              │
//!              ▼
//! ┌──────────────────────────────────────────────────┐
//! │  midimon-core Engine                             │
//! │  - Event processing                              │
//! │  - Mapping execution                             │
//! │  - Action dispatch                               │
//! └──────────────────────────────────────────────────┘
//! ```
//!
//! # Key Features
//!
//! ## Config Hot-Reload
//!
//! - **Zero Downtime**: Reload configuration without restarting
//! - **Fast**: 0-8ms reload latency (production configs: <3ms)
//! - **Atomic**: All-or-nothing config swaps via Arc<RwLock<>>
//! - **Validated**: Configuration checked before applying
//!
//! ## IPC Control
//!
//! - **Unix Domain Sockets**: Low-latency inter-process communication
//! - **JSON Protocol**: Structured request/response format
//! - **Commands**: status, reload, validate, ping, stop
//! - **Round-Trip Latency**: <1ms
//!
//! ## State Persistence
//!
//! - **Atomic Writes**: Uses tempfile + rename for crash safety
//! - **Checksums**: SHA256 validation for integrity
//! - **Emergency Saves**: Panic handler for graceful failures
//! - **Recovery**: Automatic state restoration on startup
//!
//! ## Lifecycle Management
//!
//! - **8-State Machine**: Init, Starting, Running, Reloading, Degraded, Reconnecting, Stopping, Stopped
//! - **Graceful Shutdown**: Proper resource cleanup
//! - **Health Monitoring**: Device connection tracking
//! - **Performance Metrics**: Reload latency, uptime, event counts
//!
//! # Performance Characteristics
//!
//! - **Config Reload**: 0-8ms (production configs: <3ms)
//! - **IPC Round-Trip**: <1ms
//! - **Build Time**: 26s clean, 4s incremental
//! - **Binary Size**: 3-5MB (release)
//! - **Memory Usage**: 5-10MB resident
//! - **Test Suite**: 0.24s execution time
//!
//! # Usage
//!
//! ## Starting the Daemon
//!
//! ```bash
//! # Foreground mode (development)
//! cargo run --release --bin midimon 2
//!
//! # Background service (production)
//! systemctl --user start midimon  # Linux
//! launchctl load ~/Library/LaunchAgents/com.amiable.midimon.plist  # macOS
//! ```
//!
//! ## Controlling the Daemon
//!
//! ```bash
//! # Check status
//! midimonctl status
//!
//! # Hot-reload configuration
//! midimonctl reload
//!
//! # Validate config without reloading
//! midimonctl validate
//!
//! # Health check
//! midimonctl ping
//!
//! # Graceful shutdown
//! midimonctl stop
//! ```
//!
//! ## Programmatic Usage
//!
//! ```no_run
//! use conductor_daemon::{run_daemon, get_socket_path, IpcClient};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Start daemon in another task
//! tokio::spawn(async {
//!     run_daemon().await.expect("Daemon failed");
//! });
//!
//! // Wait for daemon to start
//! tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//!
//! // Connect and send commands
//! let socket_path = get_socket_path().expect("Failed to get socket path");
//! let mut client = IpcClient::new(socket_path.to_string_lossy().to_string()).await?;
//!
//! // Get status
//! let response = client.status().await?;
//! println!("Daemon state: {:?}", response);
//!
//! // Reload config
//! let response = client.reload().await?;
//! println!("Reload result: {:?}", response);
//! # Ok(())
//! # }
//! ```
//!
//! # Modules
//!
//! - [`daemon::ipc`] - IPC server and client implementation
//! - [`daemon::config_watcher`] - File system monitoring for config hot-reload
//! - [`daemon::state`] - State persistence and recovery
//! - [`daemon::engine_manager`] - Core engine lifecycle management
//! - [`daemon::service`] - Main daemon service coordination
//! - [`daemon::types`] - Shared types and data structures
//! - [`daemon::error`] - Error types and handling

pub mod action_executor;
pub mod conditions;
pub mod daemon;
pub mod gamepad_device; // HID device management - Game Controllers (v3.0)
pub mod input_manager; // Unified MIDI + Gamepad input (v3.0)
pub mod midi_device;
pub mod plugin_manager;

// Re-export core types for convenience
pub use daemon::{
    ConfigWatcher, DaemonCommand, DaemonError, DaemonInfo, DaemonService, DaemonStatistics,
    DeviceStatus, EngineInfo, EngineManager, ErrorDetails, ErrorEntry, IpcClient, IpcCommand,
    IpcErrorCode, IpcRequest, IpcResponse, IpcServer, LifecycleState, MenuBarAction,
    MidiDeviceInfo, PersistedState, ReloadMetrics, ResponseStatus, Result, StateManager,
    calculate_checksum, create_success_response, get_socket_path, get_state_dir, run_daemon,
    run_daemon_with_config,
};

// Re-export ActionExecutor, TriggerContext, and helpers for daemon use
pub use action_executor::{ActionExecutor, TriggerContext, parse_command_line};

// Re-export condition evaluation for daemon use
pub use conditions::{ConditionContext, evaluate_condition};

// Re-export device managers for daemon use
pub use gamepad_device::HidDeviceManager; // HID device manager (v3.0)
pub use gamepad_device::GamepadDeviceManager; // Alias for backward compat (v3.0)
pub use input_manager::{InputManager, InputMode}; // Unified input (v3.0)
pub use midi_device::MidiDeviceManager;

// Re-export PluginManager for daemon use (v2.3)
pub use plugin_manager::{PluginManager, PluginManagerError, PluginManagerResult};
