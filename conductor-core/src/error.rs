// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use thiserror::Error;

/// Engine-level errors
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("MIDI connection failed: {0}")]
    MidiConnectionFailed(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Already running")]
    AlreadyRunning,

    #[error("Not running")]
    NotRunning,

    #[error("Invalid mode: {0}")]
    InvalidMode(u8),

    #[error("MIDI initialization failed: {0}")]
    MidiInit(String),

    #[error("MIDI output error: {0}")]
    MidiOutput(String),

    #[error("Plugin load failed: {0}")]
    PluginLoadFailed(String),

    #[error("Plugin execution failed: {0}")]
    PluginExecutionFailed(String),
}

/// Configuration parsing errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid trigger: {0}")]
    InvalidTrigger(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
}

/// Action execution errors
#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Application not found: {0}")]
    AppNotFound(String),
}

/// LED feedback errors
#[derive(Debug, Error)]
pub enum FeedbackError {
    #[error("Device not connected")]
    NotConnected,

    #[error("HID error: {0}")]
    HidError(String),

    #[error("MIDI error: {0}")]
    MidiError(String),
}

/// Device profile errors
#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("XML parse error: {0}")]
    XmlError(String),

    #[error("Invalid profile: {0}")]
    InvalidProfile(String),
}
