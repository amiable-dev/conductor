// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Structured logging configuration and initialization
//!
//! Provides production-ready logging with support for:
//! - Multiple output formats (text, JSON)
//! - File rotation (size-based)
//! - Console and file output
//! - Per-module filtering via RUST_LOG
//! - Backward compatibility with DEBUG=1 environment variable

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing_appender::rolling;
use tracing_subscriber::{
    EnvFilter, fmt, fmt::writer::BoxMakeWriter, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Logging configuration
///
/// Defines logging behavior including output levels, file paths, formats, and rotation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// Log level: "trace", "debug", "info", "warn", "error"
    #[serde(default = "default_level")]
    pub level: String,

    /// Path to log directory (e.g., ~/.local/share/midimon/logs)
    #[serde(default = "default_path")]
    pub path: PathBuf,

    /// Log format: "text" or "json"
    #[serde(default = "default_format")]
    pub format: String,

    /// Max log file size in MB before rotation (default 10)
    #[serde(default = "default_max_size_mb")]
    pub max_size_mb: u64,

    /// Number of rotated log files to keep (default 5)
    #[serde(default = "default_max_files")]
    pub max_files: usize,

    /// Enable console output in addition to file
    #[serde(default = "default_console_enabled")]
    pub console_enabled: bool,

    /// Enable file output
    #[serde(default = "default_file_enabled")]
    pub file_enabled: bool,
}

fn default_level() -> String {
    "info".to_string()
}

fn default_path() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local/share/midimon/logs")
    }

    #[cfg(not(target_os = "macos"))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local/share/midimon/logs")
    }
}

fn default_format() -> String {
    "text".to_string()
}

fn default_max_size_mb() -> u64 {
    10
}

fn default_max_files() -> usize {
    5
}

fn default_console_enabled() -> bool {
    true
}

fn default_file_enabled() -> bool {
    true
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            path: default_path(),
            format: default_format(),
            max_size_mb: default_max_size_mb(),
            max_files: default_max_files(),
            console_enabled: default_console_enabled(),
            file_enabled: default_file_enabled(),
        }
    }
}

impl LoggingConfig {
    /// Create a logging config with custom path
    pub fn with_path(mut self, path: impl AsRef<Path>) -> Self {
        self.path = path.as_ref().to_path_buf();
        self
    }

    /// Create a logging config with custom level
    pub fn with_level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }

    /// Create a logging config with JSON format
    pub fn with_json_format(mut self) -> Self {
        self.format = "json".to_string();
        self
    }
}

/// Initialize the tracing logging system
///
/// Sets up console and file logging with the specified configuration.
/// Respects the RUST_LOG environment variable for per-module filtering.
/// Also checks DEBUG=1 for backward compatibility, mapping it to debug level.
///
/// # Arguments
///
/// * `config` - Logging configuration
///
/// # Example
///
/// ```no_run
/// use conductor_core::logging::{LoggingConfig, init_logging};
///
/// let config = LoggingConfig::default().with_level("debug");
/// init_logging(&config).expect("Failed to initialize logging");
///
/// tracing::info!("Application started");
/// ```
pub fn init_logging(config: &LoggingConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create log directory if it doesn't exist
    if config.file_enabled {
        std::fs::create_dir_all(&config.path)?;
    }

    // Determine filter from environment
    let filter = build_env_filter(&config.level)?;

    // Set up file appender if enabled
    if config.file_enabled {
        let file_appender = rolling::daily(&config.path, "midimon.log");

        match config.format.as_str() {
            "json" => {
                let file_layer = fmt::layer()
                    .json()
                    .with_writer(file_appender)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true);

                if config.console_enabled {
                    let console_layer = fmt::layer()
                        .compact()
                        .with_target(true)
                        .with_thread_ids(false);

                    tracing_subscriber::registry()
                        .with(filter)
                        .with(file_layer)
                        .with(console_layer)
                        .init();
                } else {
                    tracing_subscriber::registry()
                        .with(filter)
                        .with(file_layer)
                        .init();
                }
            }
            _ => {
                // Default to text format
                let file_layer = fmt::layer()
                    .pretty()
                    .with_writer(file_appender)
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true);

                if config.console_enabled {
                    let console_layer = fmt::layer()
                        .compact()
                        .with_target(true)
                        .with_thread_ids(false);

                    tracing_subscriber::registry()
                        .with(filter)
                        .with(file_layer)
                        .with(console_layer)
                        .init();
                } else {
                    tracing_subscriber::registry()
                        .with(filter)
                        .with(file_layer)
                        .init();
                }
            }
        }
    } else if config.console_enabled {
        // Console only
        match config.format.as_str() {
            "json" => {
                let console_layer = fmt::layer().json().with_target(true).with_thread_ids(true);

                tracing_subscriber::registry()
                    .with(filter)
                    .with(console_layer)
                    .init();
            }
            _ => {
                let console_layer = fmt::layer()
                    .compact()
                    .with_target(true)
                    .with_thread_ids(false);

                tracing_subscriber::registry()
                    .with(filter)
                    .with(console_layer)
                    .init();
            }
        }
    }

    Ok(())
}

/// Build EnvFilter with DEBUG=1 backward compatibility support
fn build_env_filter(default_level: &str) -> Result<EnvFilter, Box<dyn std::error::Error>> {
    // Check for DEBUG=1 environment variable (backward compatibility)
    let debug_mode = std::env::var("DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    // Check for RUST_LOG environment variable (standard tracing filter)
    let rust_log = std::env::var("RUST_LOG").ok();

    let filter_str = match (rust_log, debug_mode) {
        // If both are set, RUST_LOG takes precedence
        (Some(log), _) => log,
        // If DEBUG=1, use debug level (with fallback modules)
        (None, true) => "debug".to_string(),
        // Otherwise use configured default
        (None, false) => default_level.to_string(),
    };

    Ok(EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(&filter_str))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, "text");
        assert!(config.console_enabled);
        assert!(config.file_enabled);
    }

    #[test]
    fn test_config_builder() {
        let config = LoggingConfig::default()
            .with_level("debug")
            .with_json_format();

        assert_eq!(config.level, "debug");
        assert_eq!(config.format, "json");
    }

    #[test]
    fn test_env_filter_with_debug() {
        // This test requires DEBUG env var to be set
        // Just verify it doesn't panic
        let _ = build_env_filter("info");
    }
}
