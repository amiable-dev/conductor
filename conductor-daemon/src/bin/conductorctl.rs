// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon daemon control CLI
//!
//! Command-line interface for controlling the MIDIMon daemon.

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use colored::Colorize;
use conductor_daemon::{IpcClient, IpcCommand, get_socket_path};
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "midimonctl")]
#[command(about = "Control the MIDIMon daemon", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// JSON output format
    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Check daemon status
    Status,

    /// Reload configuration
    Reload,

    /// Stop the daemon gracefully via IPC
    Shutdown,

    /// Validate configuration file
    Validate {
        /// Path to config file (defaults to daemon's current config)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Ping the daemon
    Ping,

    /// List available MIDI devices
    #[command(name = "list-devices")]
    ListDevices,

    /// Switch to a different MIDI device
    #[command(name = "set-device")]
    SetDevice {
        /// Port index of the device to switch to
        port: usize,
    },

    /// Get current MIDI device information
    #[command(name = "get-device")]
    GetDevice,

    // ============================================================================
    // Service Management Commands
    // ============================================================================
    /// Install MIDIMon as a system service (LaunchAgent)
    Install {
        /// Install daemon binary to /usr/local/bin
        #[arg(long)]
        install_binary: bool,

        /// Force reinstall even if already installed
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall MIDIMon service
    Uninstall {
        /// Also remove daemon binary from /usr/local/bin
        #[arg(long)]
        remove_binary: bool,

        /// Remove log files
        #[arg(long)]
        remove_logs: bool,
    },

    /// Start the daemon service
    Start {
        /// Wait for daemon to be ready (seconds)
        #[arg(short, long, default_value = "5")]
        wait: u64,
    },

    /// Stop the daemon service
    Stop {
        /// Force stop without graceful shutdown
        #[arg(short, long)]
        force: bool,
    },

    /// Restart the daemon service
    Restart {
        /// Wait for daemon to be ready (seconds)
        #[arg(short, long, default_value = "5")]
        wait: u64,
    },

    /// Enable auto-start on login
    Enable,

    /// Disable auto-start on login
    Disable,

    /// Show service installation status
    ServiceStatus,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging if verbose
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("conductor_daemon=debug")
            .init();
    }

    match execute_command(&cli).await {
        Ok(()) => Ok(()),
        Err(e) => {
            if !cli.json {
                eprintln!("{} {}", "Error:".red().bold(), e);
            } else {
                let error_json = serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                });
                println!("{}", serde_json::to_string_pretty(&error_json)?);
            }
            std::process::exit(1);
        }
    }
}

async fn execute_command(cli: &Cli) -> Result<()> {
    // Service management commands don't need IPC client
    match &cli.command {
        Commands::Install {
            install_binary,
            force,
        } => {
            return handle_install(*install_binary, *force, cli.json);
        }
        Commands::Uninstall {
            remove_binary,
            remove_logs,
        } => {
            return handle_uninstall(*remove_binary, *remove_logs, cli.json);
        }
        Commands::Start { wait } => {
            return handle_start(*wait, cli.json).await;
        }
        Commands::Stop { force } => {
            return handle_stop_service(*force, cli.json).await;
        }
        Commands::Restart { wait } => {
            return handle_restart(*wait, cli.json).await;
        }
        Commands::Enable => {
            return handle_enable(cli.json);
        }
        Commands::Disable => {
            return handle_disable(cli.json);
        }
        Commands::ServiceStatus => {
            return handle_service_status(cli.json);
        }
        _ => {} // Fall through to IPC commands
    }

    // IPC commands require connection to daemon
    let socket_path = get_socket_path()
        .context("Failed to determine IPC socket path")?
        .to_string_lossy()
        .to_string();

    let mut client = IpcClient::new(socket_path)
        .await
        .context("Failed to connect to daemon. Is the daemon running?")?;

    // Execute IPC command
    match &cli.command {
        Commands::Status => handle_status(&mut client, cli.json).await?,
        Commands::Reload => handle_reload(&mut client, cli.json).await?,
        Commands::Shutdown => handle_shutdown(&mut client, cli.json).await?,
        Commands::Validate { config } => handle_validate(&mut client, config, cli.json).await?,
        Commands::Ping => handle_ping(&mut client, cli.json).await?,
        Commands::ListDevices => handle_list_devices(&mut client, cli.json).await?,
        Commands::SetDevice { port } => handle_set_device(&mut client, *port, cli.json).await?,
        Commands::GetDevice => handle_get_device(&mut client, cli.json).await?,
        _ => unreachable!("Service commands handled above"),
    }

    Ok(())
}

async fn handle_status(client: &mut IpcClient, json: bool) -> Result<()> {
    let response = client
        .send_command(IpcCommand::Status, Value::Null)
        .await
        .context("Failed to get daemon status")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        // Pretty print status
        if let Some(data) = response.data {
            println!("{}", "MIDIMon Daemon Status".bold().cyan());
            println!("{}", "─".repeat(50));

            if let Some(state) = data.get("state").and_then(|v| v.as_str()) {
                let state_colored = match state {
                    "Running" => state.green(),
                    "Reloading" => state.yellow(),
                    "Degraded" => state.red(),
                    _ => state.normal(),
                };
                println!("State:           {}", state_colored);
            }

            if let Some(mode) = data.get("current_mode").and_then(|v| v.as_str()) {
                println!("Current Mode:    {}", mode.cyan());
            }

            if let Some(config_path) = data.get("config_path").and_then(|v| v.as_str()) {
                println!("Config:          {}", config_path);
            }

            if let Some(uptime) = data.get("uptime_secs").and_then(|v| v.as_u64()) {
                println!("Uptime:          {}", format_duration(uptime));
            }

            if let Some(events) = data.get("events_processed").and_then(|v| v.as_u64()) {
                println!("Events:          {}", format_number(events));
            }

            if let Some(reloads) = data.get("config_reloads").and_then(|v| v.as_u64()) {
                println!("Config Reloads:  {}", reloads);
            }

            // Reload statistics
            if let Some(reload_stats) = data.get("reload_stats") {
                println!("\n{}", "Reload Performance".bold());
                println!("{}", "─".repeat(50));

                if let Some(last) = reload_stats.get("last_reload_ms").and_then(|v| v.as_u64()) {
                    println!("Last Reload:     {} ms", last);
                }
                if let Some(avg) = reload_stats.get("avg_reload_ms").and_then(|v| v.as_u64()) {
                    let grade = if avg < 20 {
                        "A".green()
                    } else if avg < 50 {
                        "B".yellow()
                    } else {
                        "C".red()
                    };
                    println!("Average:         {} ms (grade: {})", avg, grade);
                }
                if let Some(fastest) = reload_stats
                    .get("fastest_reload_ms")
                    .and_then(|v| v.as_u64())
                {
                    println!("Fastest:         {} ms", fastest);
                }
                if let Some(slowest) = reload_stats
                    .get("slowest_reload_ms")
                    .and_then(|v| v.as_u64())
                {
                    println!("Slowest:         {} ms", slowest);
                }
            }

            println!();
        }
    }

    Ok(())
}

async fn handle_reload(client: &mut IpcClient, json: bool) -> Result<()> {
    if !json {
        println!("Reloading configuration...");
    }

    let response = client
        .send_command(IpcCommand::Reload, Value::Null)
        .await
        .context("Failed to reload configuration")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data {
        println!("{}", "✓ Configuration reloaded successfully".green());

        if let Some(duration) = data.get("reload_duration_ms").and_then(|v| v.as_u64()) {
            let grade = data
                .get("performance_grade")
                .and_then(|v| v.as_str())
                .unwrap_or("?");
            let grade_colored = match grade {
                "A" => grade.green(),
                "B" => grade.yellow(),
                _ => grade.red(),
            };
            println!("Duration:  {} ms (grade: {})", duration, grade_colored);
        }

        if let Some(modes) = data.get("modes_loaded").and_then(|v| v.as_u64()) {
            println!("Modes:     {}", modes);
        }

        if let Some(mappings) = data.get("mappings_loaded").and_then(|v| v.as_u64()) {
            println!("Mappings:  {}", mappings);
        }
    }

    Ok(())
}

async fn handle_shutdown(client: &mut IpcClient, json: bool) -> Result<()> {
    if !json {
        println!("Stopping daemon...");
    }

    let response = client
        .send_command(IpcCommand::Stop, Value::Null)
        .await
        .context("Failed to stop daemon")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!("{}", "✓ Daemon stopped successfully".green());
    }

    Ok(())
}

async fn handle_validate(
    client: &mut IpcClient,
    config_path: &Option<PathBuf>,
    json: bool,
) -> Result<()> {
    let args = if let Some(path) = config_path {
        serde_json::json!({ "path": path })
    } else {
        Value::Null
    };

    let response = client
        .send_command(IpcCommand::ValidateConfig, args)
        .await
        .context("Failed to validate configuration")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data
        && let Some(valid) = data.get("valid").and_then(|v| v.as_bool())
    {
        if valid {
            println!("{}", "✓ Configuration is valid".green());

            if let Some(modes) = data.get("modes").and_then(|v| v.as_u64()) {
                println!("Modes:    {}", modes);
            }

            if let Some(mappings) = data.get("mappings").and_then(|v| v.as_u64()) {
                println!("Mappings: {}", mappings);
            }
        } else {
            println!("{}", "✗ Configuration is invalid".red());
        }
    }

    Ok(())
}

async fn handle_ping(client: &mut IpcClient, json: bool) -> Result<()> {
    let start = std::time::Instant::now();
    let response = client
        .send_command(IpcCommand::Ping, Value::Null)
        .await
        .context("Failed to ping daemon")?;

    let latency = start.elapsed();

    if json {
        let mut resp_json = serde_json::to_value(&response)?;
        if let Some(obj) = resp_json.as_object_mut() {
            obj.insert(
                "latency_ms".to_string(),
                serde_json::json!(latency.as_millis()),
            );
        }
        println!("{}", serde_json::to_string_pretty(&resp_json)?);
    } else {
        println!(
            "{} ({:.2} ms)",
            "✓ Daemon is responding".green(),
            latency.as_secs_f64() * 1000.0
        );
    }

    Ok(())
}

async fn handle_list_devices(client: &mut IpcClient, json: bool) -> Result<()> {
    let response = client
        .send_command(IpcCommand::ListDevices, Value::Null)
        .await
        .context("Failed to list MIDI devices")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data {
        if let Some(devices) = data.get("devices").and_then(|v| v.as_array()) {
            println!("{}", "Available MIDI Devices".bold().cyan());
            println!("{}", "─".repeat(50));

            if devices.is_empty() {
                println!("No MIDI devices found");
            } else {
                for device in devices {
                    let port = device
                        .get("port_index")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    let name = device
                        .get("port_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let connected = device
                        .get("connected")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    let status = if connected {
                        " (connected)".green()
                    } else {
                        "".normal()
                    };

                    println!("  [{}] {}{}", port, name, status);
                }
            }
            println!();
        }
    }

    Ok(())
}

async fn handle_set_device(client: &mut IpcClient, port: usize, json: bool) -> Result<()> {
    let args = serde_json::json!({ "port": port });

    let response = client
        .send_command(IpcCommand::SetDevice, args)
        .await
        .context("Failed to set MIDI device")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data {
        if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
            println!("{}", message.green());
        } else {
            println!(
                "{}",
                format!("✓ Switched to device at port {}", port).green()
            );
        }
    }

    Ok(())
}

async fn handle_get_device(client: &mut IpcClient, json: bool) -> Result<()> {
    let response = client
        .send_command(IpcCommand::GetDevice, Value::Null)
        .await
        .context("Failed to get current MIDI device")?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else if let Some(data) = response.data {
        if let Some(device) = data.get("device") {
            println!("{}", "Current MIDI Device".bold().cyan());
            println!("{}", "─".repeat(50));

            let connected = device
                .get("connected")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let status = if connected {
                "Connected".green()
            } else {
                "Disconnected".red()
            };

            println!("Status:     {}", status);

            if let Some(name) = device.get("name").and_then(|v| v.as_str()) {
                println!("Name:       {}", name);
            }

            if let Some(port) = device.get("port").and_then(|v| v.as_u64()) {
                println!("Port:       {}", port);
            }

            if let Some(last_event) = device.get("last_event_at").and_then(|v| v.as_u64()) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let secs_ago = now.saturating_sub(last_event);
                println!("Last Event: {} ago", format_duration(secs_ago));
            }

            println!();
        }
    }

    Ok(())
}

/// Format duration in seconds to human-readable string
fn format_duration(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if days > 0 {
        format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Format large numbers with comma separators
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count == 3 {
            result.push(',');
            count = 0;
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}

// ============================================================================
// Service Management Implementation
// ============================================================================

/// Service configuration constants
mod service {
    use std::path::PathBuf;

    pub const SERVICE_LABEL: &str = "com.amiable.midimon";
    pub const DAEMON_BINARY_NAME: &str = "midimon";

    pub fn get_plist_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join("Library/LaunchAgents")
            .join(format!("{}.plist", SERVICE_LABEL))
    }

    pub fn get_binary_install_path() -> PathBuf {
        PathBuf::from("/usr/local/bin").join(DAEMON_BINARY_NAME)
    }

    pub fn get_log_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join("Library/Logs")
    }

    pub fn get_template_plist_path() -> Option<PathBuf> {
        // Try to find the template plist in the source tree
        let cargo_manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok()?;
        let template_path = PathBuf::from(cargo_manifest_dir)
            .join("launchd")
            .join(format!("{}.plist", SERVICE_LABEL));

        if template_path.exists() {
            Some(template_path)
        } else {
            None
        }
    }
}

/// Check if running on macOS
fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

/// Check if service is installed
fn is_service_installed() -> bool {
    service::get_plist_path().exists()
}

/// Check if daemon is running via IPC ping
async fn is_daemon_running() -> bool {
    match get_socket_path() {
        Ok(socket_path) => {
            if let Ok(mut client) = IpcClient::new(socket_path.to_string_lossy().to_string()).await
            {
                client
                    .send_command(IpcCommand::Ping, Value::Null)
                    .await
                    .is_ok()
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Install MIDIMon as a LaunchAgent service
fn handle_install(install_binary: bool, force: bool, json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service installation is currently only supported on macOS");
    }

    let plist_path = service::get_plist_path();

    // Check if already installed
    if is_service_installed() && !force {
        if json {
            let result = serde_json::json!({
                "status": "error",
                "error": "Service already installed. Use --force to reinstall."
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            eprintln!(
                "{}",
                "Service already installed. Use --force to reinstall.".yellow()
            );
        }
        return Ok(());
    }

    if !json {
        println!("{}", "Installing MIDIMon service...".bold());
    }

    // Step 1: Install binary if requested
    let binary_path = if install_binary {
        install_daemon_binary(json)?
    } else {
        // Try to find the binary in common locations
        find_daemon_binary()?
    };

    // Step 2: Create LaunchAgents directory
    let launch_agents_dir = plist_path
        .parent()
        .ok_or_else(|| anyhow!("Invalid plist path"))?;

    if !launch_agents_dir.exists() {
        std::fs::create_dir_all(launch_agents_dir)
            .context("Failed to create LaunchAgents directory")?;

        if !json {
            println!("  {} Created LaunchAgents directory", "✓".green());
        }
    }

    // Step 3: Create log directory
    let log_dir = service::get_log_dir();
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).context("Failed to create log directory")?;
    }

    if !json {
        println!(
            "  {} Created log directory: {}",
            "✓".green(),
            log_dir.display()
        );
    }

    // Step 4: Generate plist from template
    let plist_content = generate_plist(&binary_path)?;

    // Step 5: Write plist file
    std::fs::write(&plist_path, plist_content).context("Failed to write plist file")?;

    if !json {
        println!(
            "  {} Installed service plist: {}",
            "✓".green(),
            plist_path.display()
        );
    }

    // Step 6: Load the service (this enables it)
    let output = std::process::Command::new("launchctl")
        .args(["load", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to load service: {}", stderr);
    }

    if json {
        let result = serde_json::json!({
            "status": "success",
            "plist_path": plist_path,
            "binary_path": binary_path,
            "enabled": true
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "\n{}",
            "✓ MIDIMon service installed successfully".green().bold()
        );
        println!("  Binary:  {}", binary_path.display());
        println!("  Plist:   {}", plist_path.display());
        println!("  Status:  {}", "Enabled (will start on login)".green());
        println!("\nUse 'midimonctl start' to start the service now.");
    }

    Ok(())
}

/// Install daemon binary to /usr/local/bin
fn install_daemon_binary(json: bool) -> Result<PathBuf> {
    // Find the source binary (prefer release, fallback to debug)
    let cargo_target_dir =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());

    let source_binary = [
        PathBuf::from(&cargo_target_dir).join("release").join(service::DAEMON_BINARY_NAME),
        PathBuf::from(&cargo_target_dir).join("debug").join(service::DAEMON_BINARY_NAME),
    ]
    .into_iter()
    .find(|p| p.exists())
    .ok_or_else(|| anyhow!(
        "Could not find daemon binary. Build it first with 'cargo build --release --bin midimon'"
    ))?;

    let dest_binary = service::get_binary_install_path();

    // Copy binary
    std::fs::copy(&source_binary, &dest_binary)
        .context("Failed to copy binary. You may need sudo permissions.")?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        std::fs::set_permissions(&dest_binary, perms)
            .context("Failed to set binary permissions")?;
    }

    if !json {
        println!(
            "  {} Installed binary: {}",
            "✓".green(),
            dest_binary.display()
        );
    }

    Ok(dest_binary)
}

/// Find daemon binary in common locations
fn find_daemon_binary() -> Result<PathBuf> {
    let candidates = vec![
        service::get_binary_install_path(),
        PathBuf::from("target/release").join(service::DAEMON_BINARY_NAME),
        PathBuf::from("target/debug").join(service::DAEMON_BINARY_NAME),
    ];

    candidates.into_iter()
        .find(|p| p.exists())
        .ok_or_else(|| anyhow!(
            "Could not find daemon binary. Use --install-binary to install it, or build it with 'cargo build --release --bin midimon'"
        ))
}

/// Generate plist content from template
fn generate_plist(binary_path: &PathBuf) -> Result<String> {
    let username = std::env::var("USER").context("Could not determine current username")?;

    // Load template or use embedded default
    let template = if let Some(template_path) = service::get_template_plist_path() {
        std::fs::read_to_string(&template_path).context("Failed to read plist template")?
    } else {
        // Embedded template
        include_str!("../../../conductor-daemon/launchd/com.amiable.conductor.plist").to_string()
    };

    // Replace placeholders
    let plist = template
        .replace("/usr/local/bin/conductor", &binary_path.to_string_lossy())
        .replace("/Users/USERNAME", &format!("/Users/{}", username));

    Ok(plist)
}

/// Uninstall MIDIMon service
fn handle_uninstall(remove_binary: bool, remove_logs: bool, json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    if !is_service_installed() {
        if json {
            let result = serde_json::json!({
                "status": "error",
                "error": "Service not installed"
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            eprintln!("{}", "Service not installed".yellow());
        }
        return Ok(());
    }

    if !json {
        println!("{}", "Uninstalling MIDIMon service...".bold());
    }

    let plist_path = service::get_plist_path();

    // Step 1: Unload service
    let output = std::process::Command::new("launchctl")
        .args(["unload", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Don't fail if already unloaded
        if !stderr.contains("Could not find specified service") {
            eprintln!("Warning: {}", stderr);
        }
    }

    if !json {
        println!("  {} Stopped service", "✓".green());
    }

    // Step 2: Remove plist
    std::fs::remove_file(&plist_path).context("Failed to remove plist file")?;

    if !json {
        println!("  {} Removed plist: {}", "✓".green(), plist_path.display());
    }

    // Step 3: Remove binary if requested
    if remove_binary {
        let binary_path = service::get_binary_install_path();
        if binary_path.exists() {
            std::fs::remove_file(&binary_path)
                .context("Failed to remove binary. You may need sudo permissions.")?;

            if !json {
                println!(
                    "  {} Removed binary: {}",
                    "✓".green(),
                    binary_path.display()
                );
            }
        }
    }

    // Step 4: Remove logs if requested
    if remove_logs {
        let log_dir = service::get_log_dir();
        for log_file in ["midimon.log", "midimon.error.log"] {
            let log_path = log_dir.join(log_file);
            if log_path.exists() {
                std::fs::remove_file(&log_path).context("Failed to remove log file")?;
            }
        }

        if !json {
            println!("  {} Removed log files", "✓".green());
        }
    }

    if json {
        let result = serde_json::json!({
            "status": "success",
            "removed_plist": true,
            "removed_binary": remove_binary,
            "removed_logs": remove_logs
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "\n{}",
            "✓ MIDIMon service uninstalled successfully".green().bold()
        );
    }

    Ok(())
}

/// Start the daemon service
async fn handle_start(wait_secs: u64, json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    if !is_service_installed() {
        bail!("Service not installed. Run 'midimonctl install' first.");
    }

    // Check if already running
    if is_daemon_running().await {
        if json {
            let result = serde_json::json!({
                "status": "already_running"
            });
            println!("{}", serde_json::to_string_pretty(&result)?);
        } else {
            println!("{}", "Daemon is already running".yellow());
        }
        return Ok(());
    }

    if !json {
        println!("Starting MIDIMon service...");
    }

    let plist_path = service::get_plist_path();

    // Load the service
    let output = std::process::Command::new("launchctl")
        .args(["load", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "already loaded" errors
        if !stderr.contains("Already loaded") {
            bail!("Failed to start service: {}", stderr);
        }
    }

    // Wait for daemon to be ready
    if wait_secs > 0 {
        if !json {
            print!("Waiting for daemon to be ready");
            std::io::Write::flush(&mut std::io::stdout())?;
        }

        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(wait_secs);

        while start.elapsed() < timeout {
            tokio::time::sleep(Duration::from_millis(500)).await;

            if is_daemon_running().await {
                if !json {
                    println!(" {}", "✓".green());
                }
                break;
            }

            if !json {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
        }

        if !is_daemon_running().await {
            if !json {
                println!(" {}", "✗".red());
            }
            bail!(
                "Daemon did not start within {} seconds. Check logs for errors.",
                wait_secs
            );
        }
    }

    if json {
        let result = serde_json::json!({
            "status": "started",
            "ready": is_daemon_running().await
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", "✓ Service started successfully".green().bold());
    }

    Ok(())
}

/// Stop the daemon service
async fn handle_stop_service(force: bool, json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    if !is_service_installed() {
        bail!("Service not installed");
    }

    if !json {
        println!("Stopping MIDIMon service...");
    }

    // Try graceful shutdown first unless force is specified
    if !force && is_daemon_running().await {
        if !json {
            println!("  Attempting graceful shutdown via IPC...");
        }

        if let Ok(socket_path) = get_socket_path() {
            if let Ok(mut client) = IpcClient::new(socket_path.to_string_lossy().to_string()).await
            {
                let _ = client.send_command(IpcCommand::Stop, Value::Null).await;

                // Wait briefly for graceful shutdown
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Unload the service
    let plist_path = service::get_plist_path();
    let output = std::process::Command::new("launchctl")
        .args(["unload", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "not loaded" errors
        if !stderr.contains("Could not find specified service") {
            eprintln!("Warning: {}", stderr);
        }
    }

    // Verify stopped
    if is_daemon_running().await {
        bail!("Service may still be running. Check 'ps aux | grep midimon'");
    }

    if json {
        let result = serde_json::json!({
            "status": "stopped"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", "✓ Service stopped successfully".green().bold());
    }

    Ok(())
}

/// Restart the daemon service
async fn handle_restart(wait_secs: u64, json: bool) -> Result<()> {
    if !json {
        println!("{}", "Restarting MIDIMon service...".bold());
    }

    // Stop
    handle_stop_service(false, json).await?;

    // Wait a moment
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Start
    handle_start(wait_secs, json).await?;

    Ok(())
}

/// Enable auto-start on login
fn handle_enable(json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    if !is_service_installed() {
        bail!("Service not installed. Run 'midimonctl install' first.");
    }

    let plist_path = service::get_plist_path();

    // Load with -w flag enables auto-start
    let output = std::process::Command::new("launchctl")
        .args(["load", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "already loaded" errors
        if !stderr.contains("Already loaded") {
            bail!("Failed to enable service: {}", stderr);
        }
    }

    if json {
        let result = serde_json::json!({
            "status": "enabled"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "{}",
            "✓ Service enabled (will start on login)".green().bold()
        );
    }

    Ok(())
}

/// Disable auto-start on login
fn handle_disable(json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    if !is_service_installed() {
        bail!("Service not installed");
    }

    let plist_path = service::get_plist_path();

    // Unload with -w flag disables auto-start
    let output = std::process::Command::new("launchctl")
        .args(["unload", "-w", &plist_path.to_string_lossy()])
        .output()
        .context("Failed to execute launchctl")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignore "not loaded" errors
        if !stderr.contains("Could not find specified service") {
            eprintln!("Warning: {}", stderr);
        }
    }

    if json {
        let result = serde_json::json!({
            "status": "disabled"
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "{}",
            "✓ Service disabled (will not start on login)"
                .green()
                .bold()
        );
    }

    Ok(())
}

/// Show service installation and running status
fn handle_service_status(json: bool) -> Result<()> {
    if !is_macos() {
        bail!("Service management is currently only supported on macOS");
    }

    let installed = is_service_installed();
    let plist_path = service::get_plist_path();
    let binary_path = service::get_binary_install_path();
    let binary_exists = binary_path.exists();

    // Check if service is loaded with launchctl
    let list_output = std::process::Command::new("launchctl")
        .args(["list", service::SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl")?;

    let loaded = list_output.status.success();

    if json {
        let result = serde_json::json!({
            "installed": installed,
            "plist_path": plist_path,
            "binary_exists": binary_exists,
            "binary_path": binary_path,
            "loaded": loaded,
            "service_label": service::SERVICE_LABEL
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", "MIDIMon Service Status".bold().cyan());
        println!("{}", "─".repeat(50));

        let status = if installed && loaded {
            "Installed and Loaded".green()
        } else if installed {
            "Installed but Not Loaded".yellow()
        } else {
            "Not Installed".red()
        };

        println!("Status:          {}", status);
        println!("Service Label:   {}", service::SERVICE_LABEL);
        println!(
            "Plist:           {} {}",
            plist_path.display(),
            if installed {
                "✓".green()
            } else {
                "✗".red()
            }
        );
        println!(
            "Binary:          {} {}",
            binary_path.display(),
            if binary_exists {
                "✓".green()
            } else {
                "✗".red()
            }
        );

        if loaded {
            println!("\n{}", "Service is loaded (enabled)".green());
        } else if installed {
            println!(
                "\n{}",
                "Service is not loaded. Use 'midimonctl enable' or 'midimonctl start'.".yellow()
            );
        } else {
            println!(
                "\n{}",
                "Service is not installed. Use 'midimonctl install'.".yellow()
            );
        }
    }

    Ok(())
}
