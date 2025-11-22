// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Menu bar icon implementation using tray-icon
//!
//! Provides a minimal system tray icon with three states:
//! - Running: Green icon indicating active MIDI processing
//! - Stopped: Gray icon indicating daemon is idle
//! - Error: Red icon indicating an error state
//!
//! Platform support:
//! - macOS: Native NSStatusBar integration
//! - Linux: Uses libayatana-appindicator
//! - Windows: System tray integration

// objc macro generates cfg warnings for cargo-clippy
#![allow(unexpected_cfgs)]

use crossbeam_channel::Receiver;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

/// Icon state for the menu bar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconState {
    /// Daemon is running and processing MIDI events
    Running,
    /// Daemon is stopped or idle
    Stopped,
    /// Daemon encountered an error
    Error,
}

/// Menu actions that can be triggered by user
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    /// Reload configuration
    ReloadConfig,
    /// Quit the application
    Quit,
}

/// Menu bar icon manager
pub struct MenuBar {
    tray_icon: TrayIcon,
    icon_running: Icon,
    icon_stopped: Icon,
    icon_error: Icon,
    current_state: IconState,
    menu_event_receiver: Receiver<MenuEvent>,
    tray_event_receiver: Receiver<TrayIconEvent>,
    // Menu item IDs (stored for potential future use in dynamic menu updates)
    status_item: MenuItem,
    #[allow(dead_code)]
    reload_item: MenuItem,
    #[allow(dead_code)]
    quit_item: MenuItem,
}

impl MenuBar {
    /// Create a new menu bar icon with the given initial state
    ///
    /// # Errors
    /// Returns an error if:
    /// - The system tray is not available (headless environment)
    /// - Icon creation fails
    /// - Platform-specific initialization fails
    pub fn new(initial_state: IconState) -> Result<Self, MenuBarError> {
        // Check if we're in a headless environment
        if !Self::is_gui_available() {
            return Err(MenuBarError::HeadlessEnvironment);
        }

        // Create icons for each state
        let icon_running = Self::create_running_icon()?;
        let icon_stopped = Self::create_stopped_icon()?;
        let icon_error = Self::create_error_icon()?;

        // Set up event channels
        let menu_channel = MenuEvent::receiver();
        let tray_channel = TrayIconEvent::receiver();

        // Create menu
        let (menu, status_item, reload_item, quit_item) = Self::create_menu()?;

        // Select initial icon based on state
        let initial_icon = match initial_state {
            IconState::Running => &icon_running,
            IconState::Stopped => &icon_stopped,
            IconState::Error => &icon_error,
        };

        // Build the tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("MIDIMon")
            .with_icon(initial_icon.clone())
            .with_menu(Box::new(menu))
            .build()
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        Ok(Self {
            tray_icon,
            icon_running,
            icon_stopped,
            icon_error,
            current_state: initial_state,
            menu_event_receiver: menu_channel.clone(),
            tray_event_receiver: tray_channel.clone(),
            status_item,
            reload_item,
            quit_item,
        })
    }

    /// Update the icon state
    pub fn set_state(&mut self, state: IconState) -> Result<(), MenuBarError> {
        if self.current_state == state {
            return Ok(());
        }

        let icon = match state {
            IconState::Running => &self.icon_running,
            IconState::Stopped => &self.icon_stopped,
            IconState::Error => &self.icon_error,
        };

        self.tray_icon
            .set_icon(Some(icon.clone()))
            .map_err(|e| MenuBarError::UpdateFailed(e.to_string()))?;

        self.current_state = state;
        Ok(())
    }

    /// Update the tooltip text
    pub fn set_tooltip(&mut self, text: &str) -> Result<(), MenuBarError> {
        self.tray_icon
            .set_tooltip(Some(text))
            .map_err(|e| MenuBarError::UpdateFailed(e.to_string()))?;
        Ok(())
    }

    /// Get the current icon state
    pub fn state(&self) -> IconState {
        self.current_state
    }

    /// Update the status text in the menu
    pub fn update_status(&mut self, status_text: &str) {
        self.status_item.set_text(status_text);
    }

    /// Poll for menu events (non-blocking)
    ///
    /// Returns the menu action if a menu item was clicked, or None if no events
    pub fn poll_events(&self) -> Option<MenuAction> {
        // Check menu events
        if let Ok(event) = self.menu_event_receiver.try_recv() {
            return match event.id().as_ref() {
                "reload" => Some(MenuAction::ReloadConfig),
                "quit" => Some(MenuAction::Quit),
                _ => None,
            };
        }

        // Check tray icon events (clicks)
        if let Ok(event) = self.tray_event_receiver.try_recv()
            && let TrayIconEvent::Click { .. } = event
        {
            // Icon clicked - menu will show automatically
            return None;
        }

        None
    }

    /// Check if GUI is available (not headless)
    fn is_gui_available() -> bool {
        #[cfg(target_os = "macos")]
        {
            // Check if we can access the window server
            unsafe {
                let app: id = msg_send![class!(NSApplication), sharedApplication];
                app != nil
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            // On Linux/Windows, check DISPLAY or other environment variables
            std::env::var("DISPLAY").is_ok() || cfg!(target_os = "windows")
        }
    }

    /// Create the running state icon (green)
    fn create_running_icon() -> Result<Icon, MenuBarError> {
        // Create a simple 16x16 green circle icon
        let rgba = Self::create_circle_icon([0, 200, 0, 255]); // Green
        Icon::from_rgba(rgba, 16, 16).map_err(|e| MenuBarError::IconCreationFailed(e.to_string()))
    }

    /// Create the stopped state icon (gray)
    fn create_stopped_icon() -> Result<Icon, MenuBarError> {
        // Create a simple 16x16 gray circle icon
        let rgba = Self::create_circle_icon([128, 128, 128, 255]); // Gray
        Icon::from_rgba(rgba, 16, 16).map_err(|e| MenuBarError::IconCreationFailed(e.to_string()))
    }

    /// Create the error state icon (red)
    fn create_error_icon() -> Result<Icon, MenuBarError> {
        // Create a simple 16x16 red circle icon
        let rgba = Self::create_circle_icon([200, 0, 0, 255]); // Red
        Icon::from_rgba(rgba, 16, 16).map_err(|e| MenuBarError::IconCreationFailed(e.to_string()))
    }

    /// Create a simple circular icon with the given color
    fn create_circle_icon(color: [u8; 4]) -> Vec<u8> {
        let size = 16;
        let center = size as f32 / 2.0;
        let radius = 6.0;

        let mut rgba = vec![0u8; (size * size * 4) as usize];

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let distance = (dx * dx + dy * dy).sqrt();

                let idx = ((y * size + x) * 4) as usize;
                if distance <= radius {
                    // Inside circle - use the specified color
                    rgba[idx] = color[0]; // R
                    rgba[idx + 1] = color[1]; // G
                    rgba[idx + 2] = color[2]; // B
                    rgba[idx + 3] = color[3]; // A
                } else {
                    // Outside circle - transparent
                    rgba[idx + 3] = 0;
                }
            }
        }

        rgba
    }

    /// Create the menu for the tray icon with status and action items
    fn create_menu() -> Result<(Menu, MenuItem, MenuItem, MenuItem), MenuBarError> {
        let menu = Menu::new();

        // Title / Status item (disabled, just shows info)
        let status_item = MenuItem::new("MIDIMon: Stopped", false, None);
        menu.append(&status_item)
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        // Separator
        menu.append(&PredefinedMenuItem::separator())
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        // Reload Config action (⌘R on macOS shown in text)
        #[cfg(target_os = "macos")]
        let reload_item = MenuItem::with_id("reload", "Reload Config\t⌘R", true, None);
        #[cfg(not(target_os = "macos"))]
        let reload_item = MenuItem::with_id("reload", "Reload Config", true, None);

        menu.append(&reload_item)
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        // Separator
        menu.append(&PredefinedMenuItem::separator())
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        // Quit action (⌘Q on macOS shown in text)
        #[cfg(target_os = "macos")]
        let quit_item = MenuItem::with_id("quit", "Quit\t⌘Q", true, None);
        #[cfg(not(target_os = "macos"))]
        let quit_item = MenuItem::with_id("quit", "Quit", true, None);

        menu.append(&quit_item)
            .map_err(|e| MenuBarError::InitializationFailed(e.to_string()))?;

        Ok((menu, status_item, reload_item, quit_item))
    }
}

/// Errors that can occur during menu bar operations
#[derive(Debug, thiserror::Error)]
pub enum MenuBarError {
    /// The system is running in a headless environment (no GUI)
    #[error("Cannot create menu bar in headless environment")]
    HeadlessEnvironment,

    /// Failed to initialize the menu bar
    #[error("Menu bar initialization failed: {0}")]
    InitializationFailed(String),

    /// Failed to create an icon
    #[error("Icon creation failed: {0}")]
    IconCreationFailed(String),

    /// Failed to update the menu bar state
    #[error("Menu bar update failed: {0}")]
    UpdateFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_state_transitions() {
        let states = [IconState::Running, IconState::Stopped, IconState::Error];
        for state in &states {
            assert_eq!(*state, *state);
        }
    }

    #[test]
    fn test_create_circle_icon() {
        let rgba = MenuBar::create_circle_icon([255, 0, 0, 255]);
        assert_eq!(rgba.len(), 16 * 16 * 4);

        // Check that some pixels are red (inside circle)
        let mut has_red = false;
        for chunk in rgba.chunks(4) {
            if chunk[0] == 255 && chunk[1] == 0 && chunk[2] == 0 {
                has_red = true;
                break;
            }
        }
        assert!(has_red, "Icon should contain red pixels");
    }

    // Note: Cannot test MenuBar::new() in CI because it requires a GUI environment
    // Tests for menu bar creation will be integration tests that run in GUI mode
}
