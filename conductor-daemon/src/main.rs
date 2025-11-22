// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Daemon - Background MIDI controller mapping service
//!
//! This is the main entry point for the MIDIMon daemon service.
//! It parses command-line arguments and launches the daemon infrastructure
//! with IPC control, config hot-reload, and state persistence.

use clap::Parser;
use conductor_daemon::{get_socket_path, run_daemon_with_config};
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

/// MIDIMon Daemon - MIDI controller mapping service
///
/// Transform MIDI devices into advanced macro pads with velocity sensitivity,
/// long press detection, double-tap, chord detection, and RGB LED feedback.
///
/// The daemon runs as a background service with:
/// - Config hot-reload (zero downtime)
/// - IPC control via Unix domain socket
/// - State persistence across restarts
/// - Performance metrics and health monitoring
///
/// Control the daemon using `midimonctl`:
///   midimonctl status   - Check daemon state
///   midimonctl reload   - Hot-reload configuration
///   midimonctl validate - Validate config without reloading
///   midimonctl ping     - Health check
///   midimonctl stop     - Graceful shutdown
#[derive(Parser, Debug)]
#[command(name = "midimon")]
#[command(version)]
#[command(about = "MIDIMon Daemon - MIDI Macro Pad Service", long_about = None)]
struct Args {
    /// Path to configuration file
    ///
    /// Defaults to ~/Library/Application Support/midimon/config.toml on macOS
    /// or ~/.config/midimon/config.toml on Linux
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Enable verbose logging (debug level)
    ///
    /// Sets logging level to DEBUG for all MIDIMon modules.
    /// Can also be controlled via RUST_LOG environment variable.
    #[arg(short, long)]
    verbose: bool,

    /// Enable trace-level logging
    ///
    /// Sets logging level to TRACE (very verbose, includes all events).
    /// Useful for diagnosing event processing issues.
    #[arg(short = 'T', long)]
    trace: bool,

    /// Run in foreground mode (don't detach)
    ///
    /// By default the daemon runs in foreground mode.
    /// Use systemd/launchd for proper background service management.
    #[arg(short, long)]
    foreground: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    setup_logging(args.verbose, args.trace);

    info!("MIDIMon daemon starting");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Determine config path
    let config_path = match args.config {
        Some(path) => path,
        None => {
            // Use default path based on platform
            let default_path = get_default_config_path()?;
            info!("Using default config path: {}", default_path.display());
            default_path
        }
    };

    // Verify config file exists
    if !config_path.exists() {
        error!("Config file not found: {}", config_path.display());
        eprintln!(
            "Error: Configuration file not found: {}",
            config_path.display()
        );
        eprintln!();
        eprintln!(
            "Please create a config file at this location or specify a different path with --config"
        );
        eprintln!();
        eprintln!("Example config.toml:");
        eprintln!("{}", get_example_config());
        std::process::exit(1);
    }

    info!("Config file: {}", config_path.display());

    // Show socket path for IPC
    let socket_path = get_socket_path()?;
    info!("IPC socket: {}", socket_path.display());

    // Run in foreground mode (tokio runtime required for async daemon)
    let rt = tokio::runtime::Runtime::new()?;

    info!(
        "Starting daemon service (foreground mode: {})",
        args.foreground
    );

    let result = rt.block_on(async { run_daemon_with_config(config_path).await });

    match result {
        Ok(()) => {
            info!("Daemon stopped successfully");
            Ok(())
        }
        Err(e) => {
            error!("Daemon error: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Setup logging with tracing-subscriber
fn setup_logging(verbose: bool, trace: bool) {
    let log_level = if trace {
        "trace"
    } else if verbose {
        "debug"
    } else {
        "info"
    };

    // Build filter: MIDIMon modules at specified level, others at WARN
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "midimon={},conductor_core={},conductor_daemon={},warn",
            log_level, log_level, log_level
        ))
    });

    // Use human-readable format with timestamps
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_line_number(true),
        )
        .init();
}

/// Get default config path based on platform
fn get_default_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        Ok(PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("midimon")
            .join("config.toml"))
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        let config_home =
            std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
        Ok(PathBuf::from(config_home)
            .join("midimon")
            .join("config.toml"))
    }

    #[cfg(target_os = "windows")]
    {
        let appdata =
            std::env::var("APPDATA").map_err(|_| "APPDATA environment variable not set")?;
        Ok(PathBuf::from(appdata).join("midimon").join("config.toml"))
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Err("Unsupported platform".into())
    }
}

/// Get example config for error messages
fn get_example_config() -> &'static str {
    r#"[device]
name = "Mikro"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Pad 1 triggers Cmd+Space (Spotlight)"
[modes.mappings.trigger]
type = "Note"
note = 60

[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

[[global_mappings]]
description = "Emergency exit on Pad 16 (Note 75)"
[global_mappings.trigger]
type = "Note"
note = 75

[global_mappings.action]
type = "Shell"
command = "pkill midimon"
"#
}
