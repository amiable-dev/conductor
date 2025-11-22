// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Menu bar service for MIDIMon daemon
//!
//! Provides a system tray icon with status display and quick actions.
//! Communicates with the midimon daemon via IPC.

use conductor_daemon::daemon::{
    IconState, IpcClient, IpcCommand, IpcRequest, MenuAction, MenuBar, MenuBarError, ResponseStatus,
};
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("midimon_menubar=info".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting MIDIMon menu bar");

    // Create menu bar
    let mut menu_bar = match MenuBar::new(IconState::Stopped) {
        Ok(mb) => mb,
        Err(MenuBarError::HeadlessEnvironment) => {
            tracing::warn!("Cannot create menu bar in headless environment");
            return;
        }
        Err(e) => {
            tracing::error!("Failed to create menu bar: {}", e);
            return;
        }
    };

    tracing::info!("Menu bar created successfully");

    // Main event loop
    let mut last_status_check = std::time::Instant::now();
    let status_interval = Duration::from_secs(3);

    loop {
        // Check for menu events
        if let Some(action) = menu_bar.poll_events() {
            handle_menu_action(action, &mut menu_bar).await;
        }

        // Update status periodically
        if last_status_check.elapsed() >= status_interval {
            update_status(&mut menu_bar).await;
            last_status_check = std::time::Instant::now();
        }

        // Sleep to avoid busy-waiting
        time::sleep(Duration::from_millis(100)).await;
    }
}

/// Handle a menu action from the user
async fn handle_menu_action(action: MenuAction, menu_bar: &mut MenuBar) {
    match action {
        MenuAction::ReloadConfig => {
            tracing::info!("Reload config requested");
            menu_bar.update_status("MIDIMon: Reloading...");

            match send_ipc_command(IpcCommand::Reload).await {
                Ok(_) => {
                    tracing::info!("Config reloaded successfully");
                    menu_bar.update_status("MIDIMon: Running");
                    // Icon will be updated on next status check
                }
                Err(e) => {
                    tracing::error!("Failed to reload config: {}", e);
                    menu_bar.update_status(&format!("MIDIMon: Error - {}", e));
                    let _ = menu_bar.set_state(IconState::Error);
                }
            }
        }
        MenuAction::Quit => {
            tracing::info!("Quit requested");
            menu_bar.update_status("MIDIMon: Stopping...");

            match send_ipc_command(IpcCommand::Stop).await {
                Ok(_) => {
                    tracing::info!("Daemon stop command sent");
                    time::sleep(Duration::from_millis(500)).await;
                    std::process::exit(0);
                }
                Err(e) => {
                    tracing::error!("Failed to stop daemon: {}", e);
                    menu_bar.update_status(&format!("Error: {}", e));
                }
            }
        }
    }
}

/// Update the menu bar status by querying the daemon
async fn update_status(menu_bar: &mut MenuBar) {
    match send_ipc_command(IpcCommand::Status).await {
        Ok(response) => {
            // Parse the status response
            if let Some(status_text) = extract_status(&response) {
                menu_bar.update_status(&status_text);

                // Update icon based on daemon state
                if status_text.contains("Running") {
                    let _ = menu_bar.set_state(IconState::Running);
                } else if status_text.contains("Error") {
                    let _ = menu_bar.set_state(IconState::Error);
                } else {
                    let _ = menu_bar.set_state(IconState::Stopped);
                }
            }
        }
        Err(_) => {
            // Daemon not responding
            menu_bar.update_status("MIDIMon: Not running");
            let _ = menu_bar.set_state(IconState::Stopped);
        }
    }
}

/// Send an IPC command to the daemon
async fn send_ipc_command(command: IpcCommand) -> Result<String, String> {
    let mut client = IpcClient::connect()
        .await
        .map_err(|e| format!("Failed to connect: {}", e))?;

    let request = IpcRequest {
        id: Uuid::new_v4().to_string(),
        command,
        args: serde_json::Value::Null,
    };

    let response = client
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    match response.status {
        ResponseStatus::Success => {
            Ok(serde_json::to_string_pretty(&response.data).unwrap_or_default())
        }
        ResponseStatus::Error => {
            let error_msg = response
                .error
                .map(|e| e.message)
                .unwrap_or_else(|| "Unknown error".to_string());
            Err(error_msg)
        }
    }
}

/// Extract status text from IPC response
fn extract_status(response: &str) -> Option<String> {
    // Parse JSON response to extract relevant status info
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(response) {
        // Try to extract daemon info
        if let Some(daemon) = value.get("daemon") {
            let state = daemon
                .get("lifecycle_state")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");

            let uptime = daemon
                .get("uptime_seconds")
                .and_then(|v| v.as_f64())
                .map(format_duration)
                .unwrap_or_else(|| "Unknown".to_string());

            return Some(format!("MIDIMon: {}\nUptime: {}", state, uptime));
        }
    }

    Some("MIDIMon: Running".to_string())
}

/// Format duration in human-readable form
fn format_duration(seconds: f64) -> String {
    let secs = seconds as u64;
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else if minutes > 0 {
        format!("{}m", minutes)
    } else {
        format!("{}s", secs)
    }
}
