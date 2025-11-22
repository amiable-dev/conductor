// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Application state management

use crate::app_detection::{AppDetector, AppInfo};
use crate::events::EventStreamManager;
use crate::midi_learn::MidiLearnSession;
use crate::profile_manager::ProfileManager;
use conductor_core::midi_output::MidiOutputManager;
use conductor_daemon::plugin_manager::PluginManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Global application state
pub struct AppState {
    /// Inner state behind Arc<RwLock<>> for thread-safe access
    inner: Arc<RwLock<AppStateInner>>,
}

struct AppStateInner {
    /// Whether the daemon is currently connected
    daemon_connected: bool,
    /// Current MIDI Learn session (if active)
    learn_session: Option<MidiLearnSession>,
    /// App detector for frontmost app monitoring
    app_detector: Arc<AppDetector>,
    /// Profile manager for per-app configs
    profile_manager: Arc<ProfileManager>,
    /// Event stream manager for live console
    event_stream_manager: Arc<EventStreamManager>,
    /// MIDI output manager for virtual MIDI output
    midi_output_manager: Arc<RwLock<MidiOutputManager>>,
    /// Plugin manager for plugin management
    plugin_manager: Arc<RwLock<PluginManager>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        let profile_manager = ProfileManager::new().expect("Failed to create profile manager");

        // Initialize plugin manager with default plugins directory
        let plugins_dir = dirs::home_dir()
            .map(|home| home.join(".midimon/plugins"))
            .unwrap_or_else(|| PathBuf::from(".midimon/plugins"));
        let plugin_manager = PluginManager::new(plugins_dir);

        Self {
            inner: Arc::new(RwLock::new(AppStateInner {
                daemon_connected: false,
                learn_session: None,
                app_detector: Arc::new(AppDetector::new()),
                profile_manager: Arc::new(profile_manager),
                event_stream_manager: Arc::new(EventStreamManager::new(1000)),
                midi_output_manager: Arc::new(RwLock::new(MidiOutputManager::new())),
                plugin_manager: Arc::new(RwLock::new(plugin_manager)),
            })),
        }
    }

    /// Check if the daemon is connected
    #[allow(dead_code)] // Part of state API for daemon status checks
    pub async fn is_daemon_connected(&self) -> bool {
        self.inner.read().await.daemon_connected
    }

    /// Set the daemon connection status
    pub async fn set_daemon_connected(&self, connected: bool) {
        self.inner.write().await.daemon_connected = connected;
    }

    /// Set the current MIDI Learn session
    pub async fn set_learn_session(&self, session: MidiLearnSession) {
        self.inner.write().await.learn_session = Some(session);
    }

    /// Get the current MIDI Learn session
    pub async fn get_learn_session(&self) -> Option<MidiLearnSession> {
        self.inner.read().await.learn_session.clone()
    }

    /// Clear the MIDI Learn session
    #[allow(dead_code)] // Part of MIDI Learn API for session cleanup
    pub async fn clear_learn_session(&self) {
        self.inner.write().await.learn_session = None;
    }

    /// Get the current frontmost app
    pub async fn get_current_app(&self) -> Option<AppInfo> {
        let inner = self.inner.read().await;
        inner.app_detector.get_current_app().await
    }

    /// Start app detection with change notifications
    pub async fn start_app_detection(&self) {
        let inner = self.inner.read().await;
        let detector = Arc::clone(&inner.app_detector);

        detector
            .start_detection(move |_app_info| {
                // App change callback
                // Could emit Tauri events here for frontend notification
                // For now, just store in detector state
            })
            .await;
    }

    /// Stop app detection
    pub async fn stop_app_detection(&self) {
        let inner = self.inner.read().await;
        inner.app_detector.stop_detection().await;
    }

    /// Get profile manager
    pub async fn get_profile_manager(&self) -> Arc<ProfileManager> {
        let inner = self.inner.read().await;
        Arc::clone(&inner.profile_manager)
    }

    /// Start event monitoring for live console
    pub async fn start_event_monitoring(&self) {
        let inner = self.inner.read().await;
        inner.event_stream_manager.start().await;
    }

    /// Stop event monitoring
    pub async fn stop_event_monitoring(&self) {
        let inner = self.inner.read().await;
        inner.event_stream_manager.stop().await;
    }

    /// Check if event monitoring is active
    pub async fn is_event_monitoring_active(&self) -> bool {
        let inner = self.inner.read().await;
        inner.event_stream_manager.is_active().await
    }

    /// Get event stream manager
    #[allow(dead_code)] // Part of event monitoring API
    pub async fn get_event_stream_manager(&self) -> Arc<EventStreamManager> {
        let inner = self.inner.read().await;
        Arc::clone(&inner.event_stream_manager)
    }

    /// Get MIDI output manager
    pub async fn get_midi_output_manager(&self) -> Arc<RwLock<MidiOutputManager>> {
        let inner = self.inner.read().await;
        Arc::clone(&inner.midi_output_manager)
    }

    /// Get plugin manager
    pub async fn get_plugin_manager(&self) -> Arc<RwLock<PluginManager>> {
        let inner = self.inner.read().await;
        Arc::clone(&inner.plugin_manager)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
