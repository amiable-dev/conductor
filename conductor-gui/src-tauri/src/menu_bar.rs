// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! System tray / menu bar integration for MIDIMon
//!
//! Provides a persistent menu bar icon with quick actions and status display.
//! Platform-specific implementations:
//! - macOS: NSStatusBar
//! - Linux: AppIndicator
//! - Windows: System Tray

use tauri::{
    AppHandle, Emitter, Manager,
    menu::{Menu, MenuItem, MenuItemKind, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_shell::ShellExt;

/// Menu item IDs
const MENU_STATUS: &str = "status";
const MENU_RELOAD: &str = "reload";
const MENU_PAUSE: &str = "pause";
const MENU_RESUME: &str = "resume";
const MENU_MODE_PREFIX: &str = "mode_";
const MENU_LOGS: &str = "logs";
const MENU_CONFIG: &str = "config";
const MENU_QUIT: &str = "quit";
const MENU_SHOW: &str = "show";

/// Icon states for visual feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Part of menu bar API for icon state management
pub enum TrayIconState {
    Running,
    Stopped,
    Error,
    Paused,
}

/// Build the system tray menu structure
pub fn build_tray_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let status = MenuItem::with_id(app, MENU_STATUS, "Status: Checking...", false, None::<&str>)?;
    status.set_enabled(false)?;

    let show = MenuItem::with_id(app, MENU_SHOW, "Show MIDIMon", true, None::<&str>)?;
    let reload = MenuItem::with_id(app, MENU_RELOAD, "Reload Configuration", true, None::<&str>)?;
    let pause = MenuItem::with_id(app, MENU_PAUSE, "Pause Processing", true, None::<&str>)?;
    let resume = MenuItem::with_id(app, MENU_RESUME, "Resume Processing", true, None::<&str>)?;
    resume.set_enabled(false)?;

    // Mode switching submenu
    let mode_default = MenuItem::with_id(
        app,
        &format!("{}{}", MENU_MODE_PREFIX, 0),
        "Default",
        true,
        None::<&str>,
    )?;
    let mode_dev = MenuItem::with_id(
        app,
        &format!("{}{}", MENU_MODE_PREFIX, 1),
        "Development",
        true,
        None::<&str>,
    )?;
    let mode_media = MenuItem::with_id(
        app,
        &format!("{}{}", MENU_MODE_PREFIX, 2),
        "Media",
        true,
        None::<&str>,
    )?;
    let modes_menu = Submenu::with_items(
        app,
        "Switch Mode",
        true,
        &[&mode_default, &mode_dev, &mode_media],
    )?;

    let logs = MenuItem::with_id(app, MENU_LOGS, "View Logs", true, None::<&str>)?;
    let config = MenuItem::with_id(app, MENU_CONFIG, "Open Config File", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, MENU_QUIT, "Quit MIDIMon", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &status,
            &PredefinedMenuItem::separator(app)?,
            &show,
            &reload,
            &pause,
            &resume,
            &PredefinedMenuItem::separator(app)?,
            &modes_menu,
            &PredefinedMenuItem::separator(app)?,
            &logs,
            &config,
            &PredefinedMenuItem::separator(app)?,
            &quit,
        ],
    )?;

    Ok(menu)
}

/// Initialize the system tray
pub fn setup_tray(app: &AppHandle) -> Result<(), tauri::Error> {
    let menu = build_tray_menu(app)?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| handle_menu_click(app, event.id.as_ref()))
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                // Show main window on left click
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

/// Handle menu item clicks
fn handle_menu_click(app: &AppHandle, menu_id: &str) {
    match menu_id {
        MENU_SHOW => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        MENU_RELOAD => {
            // Call the reload command via IPC
            tauri::async_runtime::spawn(async move {
                // Use the existing reload_config command
                // This will be called from the frontend via the menu
            });
        }
        MENU_PAUSE => {
            // Pause daemon processing
            update_pause_resume_state(app, true);
        }
        MENU_RESUME => {
            // Resume daemon processing
            update_pause_resume_state(app, false);
        }
        MENU_LOGS => {
            // Open logs viewer or terminal
            #[cfg(target_os = "macos")]
            {
                let shell = app.shell();
                let _ = shell.command("open").args(["-a", "Console"]).spawn();
            }
            #[cfg(target_os = "linux")]
            {
                let shell = app.shell();
                let _ = shell.command("xdg-open").args(["/var/log/midimon"]).spawn();
            }
            #[cfg(target_os = "windows")]
            {
                let shell = app.shell();
                let _ = shell
                    .command("explorer")
                    .args(["%APPDATA%\\midimon\\logs"])
                    .spawn();
            }
        }
        MENU_CONFIG => {
            // Open config file in default editor
            #[cfg(target_os = "macos")]
            {
                let config_path = dirs::config_dir()
                    .map(|p| p.join("midimon/config.toml"))
                    .and_then(|p| p.to_str().map(String::from));

                if let Some(path) = config_path {
                    let shell = app.shell();
                    let _ = shell.command("open").args(["-t", &path]).spawn();
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                // Generic fallback
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("open-config", ());
                }
            }
        }
        MENU_QUIT => {
            // Stop daemon and quit
            app.exit(0);
        }
        id if id.starts_with(MENU_MODE_PREFIX) => {
            // Mode switch
            if let Some(mode_str) = id.strip_prefix(MENU_MODE_PREFIX) {
                if let Ok(mode) = mode_str.parse::<usize>() {
                    switch_mode(app, mode);
                }
            }
        }
        _ => {}
    }
}

/// Update pause/resume menu state
fn update_pause_resume_state(app: &AppHandle, paused: bool) {
    if let Some(menu) = app.menu() {
        if let Some(pause_item) = menu.get(MENU_PAUSE) {
            if let MenuItemKind::MenuItem(item) = pause_item {
                let _ = item.set_enabled(!paused);
            }
        }
        if let Some(resume_item) = menu.get(MENU_RESUME) {
            if let MenuItemKind::MenuItem(item) = resume_item {
                let _ = item.set_enabled(paused);
            }
        }
    }
}

/// Switch to a different mode
fn switch_mode(app: &AppHandle, mode: usize) {
    // Emit event to frontend to handle mode switch
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("switch-mode", mode);
    }
}

/// Update status display in menu bar
#[allow(dead_code)] // Part of menu bar API, used for status updates
pub fn update_status(app: &AppHandle, status_text: &str) {
    if let Some(menu) = app.menu() {
        if let Some(status_item) = menu.get(MENU_STATUS) {
            if let MenuItemKind::MenuItem(item) = status_item {
                let _ = item.set_text(status_text);
            }
        }
    }
}

/// Update icon based on daemon state
#[allow(dead_code)] // Part of menu bar API, used for icon updates
pub fn update_icon(_app: &AppHandle, state: TrayIconState) {
    // Icon updates would use different icon files based on state
    // For now, this is a placeholder for the icon switching logic
    let _icon_name = match state {
        TrayIconState::Running => "icon_running",
        TrayIconState::Stopped => "icon_stopped",
        TrayIconState::Error => "icon_error",
        TrayIconState::Paused => "icon_paused",
    };

    // In a full implementation, load the appropriate icon file:
    // if let Some(tray) = app.tray_by_id("main") {
    //     let icon = tauri::image::Image::from_path(format!("icons/{}.png", icon_name))?;
    //     let _ = tray.set_icon(Some(icon));
    // }
}

/// Start background status polling
pub fn start_status_polling(app: AppHandle) {
    use std::time::Duration;

    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            interval.tick().await;

            // Poll daemon status
            if let Some(window) = app.get_webview_window("main") {
                // Trigger status update via IPC
                let _ = window.emit("poll-status", ());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_states() {
        let states = vec![
            TrayIconState::Running,
            TrayIconState::Stopped,
            TrayIconState::Error,
            TrayIconState::Paused,
        ];

        for state in states {
            // Just ensure all states are valid
            let _ = format!("{:?}", state);
        }
    }
}
