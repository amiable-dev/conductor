// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Error types for daemon operations

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Configuration error: {0}")]
    Config(#[from] conductor_core::ConfigError),

    #[error("Engine error: {0}")]
    Engine(#[from] conductor_core::EngineError),

    #[error("IPC error: {0}")]
    Ipc(String),

    #[error("File watcher error: {0}")]
    FileWatcher(String),

    #[error("State persistence error: {0}")]
    StatePersistence(String),

    #[error("Service registration error: {0}")]
    ServiceRegistration(String),

    #[error("Menu bar error: {0}")]
    MenuBar(String),

    #[error("Invalid state transition: {from:?} → {to:?}")]
    InvalidStateTransition { from: String, to: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Channel send error")]
    ChannelSend,

    #[error("Channel receive error")]
    ChannelRecv,

    #[error("Device not connected")]
    DeviceNotConnected,

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Fatal error: {0}")]
    Fatal(String),
}

pub type Result<T> = std::result::Result<T, DaemonError>;

/// IPC error codes for structured error responses
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(u16)]
pub enum IpcErrorCode {
    // Protocol errors (1xxx)
    InvalidJson = 1001,
    MissingField = 1002,
    UnknownCommand = 1003,
    InvalidRequest = 1004,

    // Config errors (2xxx)
    ConfigNotFound = 2001,
    ConfigValidationFailed = 2002,
    ConfigParseFailed = 2003,

    // State errors (3xxx)
    InvalidState = 3001,
    DeviceNotConnected = 3002,

    // System errors (4xxx)
    InternalError = 4001,
    Timeout = 4002,
}

impl IpcErrorCode {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidJson => "Invalid JSON in request",
            Self::MissingField => "Required field missing in request",
            Self::UnknownCommand => "Unknown command",
            Self::InvalidRequest => "Invalid request (exceeds size limits or malformed)",
            Self::ConfigNotFound => "Configuration file not found",
            Self::ConfigValidationFailed => "Configuration validation failed",
            Self::ConfigParseFailed => "Failed to parse configuration",
            Self::InvalidState => "Invalid daemon state for this operation",
            Self::DeviceNotConnected => "MIDI device not connected",
            Self::InternalError => "Internal server error",
            Self::Timeout => "Operation timed out",
        }
    }
}
