// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Core types for daemon operations

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;

/// Daemon lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    /// Initial state, loading configuration and connecting to devices
    Init,

    /// Starting up, initializing all components
    Starting,

    /// Running normally, processing events
    Running,

    /// Reloading configuration
    Reloading,

    /// Device disconnected, attempting to reconnect
    Degraded,

    /// Attempting to reconnect to device
    Reconnecting,

    /// Shutting down gracefully
    Stopping,

    /// Stopped, daemon has exited
    Stopped,
}

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init => write!(f, "Init"),
            Self::Starting => write!(f, "Starting"),
            Self::Running => write!(f, "Running"),
            Self::Reloading => write!(f, "Reloading"),
            Self::Degraded => write!(f, "Degraded"),
            Self::Reconnecting => write!(f, "Reconnecting"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Stopped => write!(f, "Stopped"),
        }
    }
}

impl LifecycleState {
    /// Check if a state transition is valid
    pub fn can_transition_to(&self, new_state: Self) -> bool {
        matches!(
            (self, new_state),
            (Self::Init, Self::Starting)
                | (Self::Starting, Self::Running)
                | (Self::Running, Self::Reloading)
                | (Self::Running, Self::Degraded)
                | (Self::Running, Self::Stopping)
                | (Self::Reloading, Self::Running)
                | (Self::Reloading, Self::Degraded)
                | (Self::Degraded, Self::Reconnecting)
                | (Self::Degraded, Self::Stopping)
                | (Self::Reconnecting, Self::Running)
                | (Self::Reconnecting, Self::Degraded)
                | (Self::Stopping, Self::Stopped)
        )
    }
}

/// Commands that can be sent to the daemon
#[derive(Debug)]
pub enum DaemonCommand {
    /// Config file changed, trigger reload
    ConfigFileChanged(PathBuf),

    /// IPC request from client
    IpcRequest {
        request: IpcRequest,
        response_tx: oneshot::Sender<IpcResponse>,
    },

    /// Menu bar action
    MenuBarAction(MenuBarAction),

    /// Device disconnected
    DeviceDisconnected,

    /// Device reconnected
    DeviceReconnected,

    /// Gamepad reconnected (v3.0)
    ReconnectGamepad,

    /// Device reconnection failed after max attempts
    DeviceReconnectionFailed,

    /// Fatal error occurred
    FatalError(String),

    /// Graceful shutdown requested
    Shutdown,
}

/// IPC request from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest {
    pub id: String,
    pub command: IpcCommand,
    #[serde(default)]
    pub args: serde_json::Value,
}

/// IPC commands
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IpcCommand {
    Ping,
    Status,
    Reload,
    Stop,
    ValidateConfig,

    // Device management
    ListDevices,
    SetDevice,
    GetDevice,
}

/// IPC response to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub id: String,
    pub status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Menu bar actions
#[derive(Debug, Clone)]
pub enum MenuBarAction {
    ReloadConfig,
    OpenConfigFile,
    ViewStatus,
    Quit,
}

/// Device status information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub connected: bool,
    pub name: Option<String>,
    pub port: Option<usize>,
    pub last_event_at: Option<u64>, // Unix timestamp in seconds
}

/// MIDI device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiDeviceInfo {
    pub port_index: usize,
    pub port_name: String,
    pub manufacturer: Option<String>,
    pub connected: bool,
}

/// Daemon statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DaemonStatistics {
    pub events_processed: u64,
    pub actions_executed: u64,
    pub errors_since_start: u64,
    pub config_reloads: u64,
    pub uptime_secs: u64,
    pub last_reload_duration_ms: Option<u64>,
    pub fastest_reload_ms: Option<u64>,
    pub slowest_reload_ms: Option<u64>,
    pub avg_reload_ms: Option<u64>,
}

impl DaemonStatistics {
    /// Update reload statistics with new metrics
    pub fn update_reload_metrics(&mut self, metrics: &ReloadMetrics) {
        self.config_reloads += 1;
        self.last_reload_duration_ms = Some(metrics.duration_ms);

        // Update fastest
        self.fastest_reload_ms = Some(match self.fastest_reload_ms {
            None => metrics.duration_ms,
            Some(fastest) => fastest.min(metrics.duration_ms),
        });

        // Update slowest
        self.slowest_reload_ms = Some(match self.slowest_reload_ms {
            None => metrics.duration_ms,
            Some(slowest) => slowest.max(metrics.duration_ms),
        });

        // Update average
        self.avg_reload_ms = Some(match self.avg_reload_ms {
            None => metrics.duration_ms,
            Some(avg) => {
                // Running average: new_avg = old_avg + (new_value - old_avg) / count
                avg + (metrics.duration_ms.saturating_sub(avg)) / self.config_reloads
            }
        });
    }
}

/// Error entry for error log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub timestamp: u64, // Unix timestamp in seconds
    pub kind: String,
    pub message: String,
}

impl ErrorEntry {
    pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            kind: kind.into(),
            message: message.into(),
        }
    }
}

/// Config reload metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReloadMetrics {
    pub duration_ms: u64,
    pub modes_loaded: usize,
    pub mappings_loaded: usize,
    pub config_load_ms: u64,
    pub mapping_compile_ms: u64,
    pub swap_ms: u64,
}

impl ReloadMetrics {
    /// Check if reload met performance targets
    pub fn met_target(&self) -> bool {
        self.duration_ms < 50 // Target: <50ms
    }

    /// Get performance grade (A/B/C/D/F)
    pub fn performance_grade(&self) -> char {
        match self.duration_ms {
            0..=20 => 'A',    // Excellent
            21..=50 => 'B',   // Good (target)
            51..=100 => 'C',  // Acceptable
            101..=200 => 'D', // Poor
            _ => 'F',         // Unacceptable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_state_transitions() {
        assert!(LifecycleState::Init.can_transition_to(LifecycleState::Starting));
        assert!(LifecycleState::Starting.can_transition_to(LifecycleState::Running));
        assert!(LifecycleState::Running.can_transition_to(LifecycleState::Reloading));
        assert!(LifecycleState::Running.can_transition_to(LifecycleState::Degraded));
        assert!(LifecycleState::Running.can_transition_to(LifecycleState::Stopping));

        // Invalid transitions
        assert!(!LifecycleState::Init.can_transition_to(LifecycleState::Running));
        assert!(!LifecycleState::Running.can_transition_to(LifecycleState::Starting));
        assert!(!LifecycleState::Stopped.can_transition_to(LifecycleState::Running));
    }

    #[test]
    fn test_ipc_request_serialization() {
        let request = IpcRequest {
            id: "test-123".to_string(),
            command: IpcCommand::Ping,
            args: serde_json::json!({}),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"command\":\"PING\""));
        assert!(json.contains("\"id\":\"test-123\""));
    }

    #[test]
    fn test_ipc_response_serialization() {
        let response = IpcResponse {
            id: "test-456".to_string(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"message": "pong"})),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"success\""));
        assert!(!json.contains("error")); // Should be skipped when None
    }

    #[test]
    fn test_error_entry_creation() {
        let entry = ErrorEntry::new("DeviceDisconnected", "MIDI device unplugged");
        assert_eq!(entry.kind, "DeviceDisconnected");
        assert_eq!(entry.message, "MIDI device unplugged");
        assert!(entry.timestamp > 0);
    }
}
