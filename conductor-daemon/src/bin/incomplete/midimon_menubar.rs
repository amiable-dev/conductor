// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon menu bar application
//!
//! System tray icon with quick access to daemon controls.

use anyhow::{Context, Result};
use conductor_daemon::{get_socket_path, IpcClient, IpcCommand};
use std::sync::Arc;
use tokio::sync::Mutex;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

struct MenuBarApp {
    _tray_icon: TrayIcon,
    client: Arc<Mutex<Option<IpcClient>>>,
}

impl MenuBarApp {
    async fn new() -> Result<Self> {
        // Create menu items
        let menu = Menu::new();

        let status_item = MenuItem::new("Status: Connecting...", false, None);
        let separator1 = MenuItem::new("─────────────────", false, None);
        let reload_item = MenuItem::new("Reload Configuration", true, None);
        let open_config_item = MenuItem::new("Open Config File", true, None);
        let separator2 = MenuItem::new("─────────────────", false, None);
        let quit_item = MenuItem::new("Quit Daemon", true, None);

        menu.append(&status_item)?;
        menu.append(&separator1)?;
        menu.append(&reload_item)?;
        menu.append(&open_config_item)?;
        menu.append(&separator2)?;
        menu.append(&quit_item)?;

        // Create tray icon
        let icon = Self::create_icon()?;
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("MIDIMon")
            .with_icon(icon)
            .build()?;

        // Try to connect to daemon
        let socket_path = get_socket_path()
            .context("Failed to get socket path")?
            .to_string_lossy()
            .to_string();

        let client = match IpcClient::new(socket_path).await {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Warning: Could not connect to daemon: {}", e);
                None
            }
        };

        Ok(Self {
            _tray_icon: tray_icon,
            client: Arc::new(Mutex::new(client)),
        })
    }

    fn create_icon() -> Result<Icon> {
        // Create a simple 32x32 icon with MIDIMon branding
        // For now, create a simple blue square
        let width = 32;
        let height = 32;
        let mut rgba = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            for x in 0..width {
                // Create a blue circle in the center
                let dx = (x as f32) - 16.0;
                let dy = (y as f32) - 16.0;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 14.0 {
                    // Blue circle
                    rgba.push(0x3B); // R
                    rgba.push(0x82); // G
                    rgba.push(0xF6); // B
                    rgba.push(0xFF); // A
                } else if dist < 15.0 {
                    // White border
                    rgba.push(0xFF);
                    rgba.push(0xFF);
                    rgba.push(0xFF);
                    rgba.push(0xFF);
                } else {
                    // Transparent
                    rgba.push(0x00);
                    rgba.push(0x00);
                    rgba.push(0x00);
                    rgba.push(0x00);
                }
            }
        }

        Icon::from_rgba(rgba, width as u32, height as u32)
            .context("Failed to create icon")
    }

    async fn handle_menu_event(&self, event: MenuEvent) -> Result<()> {
        let menu_id = event.id.0.as_ref();

        match menu_id {
            "Reload Configuration" => {
                println!("Reloading configuration...");
                self.reload_config().await?;
            }
            "Open Config File" => {
                println!("Opening config file...");
                self.open_config_file().await?;
            }
            "Quit Daemon" => {
                println!("Stopping daemon...");
                self.quit_daemon().await?;
                std::process::exit(0);
            }
            _ => {}
        }

        Ok(())
    }

    async fn reload_config(&self) -> Result<()> {
        let mut client_guard = self.client.lock().await;

        if let Some(client) = client_guard.as_mut() {
            match client.send_command(IpcCommand::Reload, serde_json::Value::Null).await {
                Ok(response) => {
                    if let Some(data) = response.data {
                        if let Some(duration) = data.get("reload_duration_ms") {
                            println!("✓ Config reloaded in {} ms", duration);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to reload config: {}", e);
                }
            }
        } else {
            eprintln!("Not connected to daemon");
        }

        Ok(())
    }

    async fn open_config_file(&self) -> Result<()> {
        let state_dir = conductor_daemon::get_state_dir()?;
        let config_path = state_dir.join("config.toml");

        // Open with default editor
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(&config_path)
                .spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(&config_path)
                .spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "start", config_path.to_str().unwrap()])
                .spawn()?;
        }

        Ok(())
    }

    async fn quit_daemon(&self) -> Result<()> {
        let mut client_guard = self.client.lock().await;

        if let Some(client) = client_guard.as_mut() {
            match client.send_command(IpcCommand::Stop, serde_json::Value::Null).await {
                Ok(_) => {
                    println!("✓ Daemon stopped");
                }
                Err(e) => {
                    eprintln!("Failed to stop daemon: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn update_status(&self) -> Result<()> {
        let mut client_guard = self.client.lock().await;

        if let Some(client) = client_guard.as_mut() {
            match client.send_command(IpcCommand::Status, serde_json::Value::Null).await {
                Ok(response) => {
                    if let Some(data) = response.data {
                        if let Some(state) = data.get("state").and_then(|v| v.as_str()) {
                            println!("Status: {}", state);
                        }
                    }
                }
                Err(_) => {
                    // Connection lost, try to reconnect
                    let socket_path = get_socket_path()?
                        .to_string_lossy()
                        .to_string();

                    if let Ok(new_client) = IpcClient::new(socket_path).await {
                        *client = new_client;
                    }
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("MIDIMon Menu Bar starting...");

    let app = MenuBarApp::new().await?;

    // Menu event handler
    let menu_channel = MenuEvent::receiver();
    let app = Arc::new(app);

    // Status update loop
    let app_clone = app.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            if let Err(e) = app_clone.update_status().await {
                eprintln!("Status update error: {}", e);
            }
        }
    });

    // Event loop
    println!("Menu bar ready. Press Ctrl+C to exit.");
    loop {
        if let Ok(event) = menu_channel.try_recv() {
            let app_clone = app.clone();
            tokio::spawn(async move {
                if let Err(e) = app_clone.handle_menu_event(event).await {
                    eprintln!("Menu event error: {}", e);
                }
            });
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
