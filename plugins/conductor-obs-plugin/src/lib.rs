// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! OBS Studio integration plugin for MIDIMon
//!
//! This plugin provides actions for controlling OBS Studio through the OBS WebSocket protocol.
//! Supports scene switching, source visibility, recording, streaming, and more.

use midimon_core::plugin::{ActionPlugin, Capability, TriggerContext};
use obws::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

/// OBS action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ObsAction {
    /// Switch to a specific scene
    SwitchScene { scene_name: String },
    /// Start/Stop recording
    ToggleRecording,
    /// Start recording
    StartRecording,
    /// Stop recording
    StopRecording,
    /// Pause/Resume recording
    PauseRecording,
    /// Start/Stop streaming
    ToggleStreaming,
    /// Start streaming
    StartStreaming,
    /// Stop streaming
    StopStreaming,
    /// Toggle mute for audio source
    ToggleMute { source_name: String },
    /// Set audio source volume (0.0 - 1.0)
    SetVolume { source_name: String, volume: f32 },
    /// Start/Stop replay buffer
    ToggleReplayBuffer,
    /// Save replay buffer
    SaveReplayBuffer,
    /// Toggle Studio Mode
    ToggleStudioMode,
}

/// OBS plugin state
struct ObsPluginState {
    client: Option<Client>,
    host: String,
    port: u16,
    password: Option<String>,
}

/// OBS integration plugin
pub struct ObsPlugin {
    state: Arc<Mutex<ObsPluginState>>,
    runtime: Runtime,
}

impl ObsPlugin {
    /// Create a new OBS plugin instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Create tokio runtime for async operations
        let runtime = Runtime::new()?;

        let host = std::env::var("OBS_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("OBS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(4455); // OBS WebSocket v5 default port
        let password = std::env::var("OBS_PASSWORD").ok();

        Ok(Self {
            state: Arc::new(Mutex::new(ObsPluginState {
                client: None,
                host,
                port,
                password,
            })),
            runtime,
        })
    }

    /// Ensure connection to OBS WebSocket
    fn ensure_connected(&self) -> Result<(), Box<dyn Error>> {
        let mut state = self.state.lock().unwrap();

        if state.client.is_some() {
            // TODO: Check if connection is still alive
            return Ok(());
        }

        // Connect to OBS WebSocket using runtime
        let host = state.host.clone();
        let port = state.port;
        let password = state.password.clone();

        let client = self.runtime.block_on(async {
            Client::connect(&host, port, password)
                .await
                .map_err(|e| format!("Failed to connect to OBS at {}:{}: {}", host, port, e))
        })?;

        state.client = Some(client);
        Ok(())
    }

    /// Execute an OBS action (sync wrapper around async operations)
    fn execute_action(&self, action: &ObsAction) -> Result<(), Box<dyn Error>> {
        self.ensure_connected()?;

        let state = self.state.lock().unwrap();
        let client = state
            .client
            .as_ref()
            .ok_or("Not connected to OBS")?;

        // Use the runtime to execute async operations synchronously
        self.runtime.block_on(async {
            match action {
                ObsAction::SwitchScene { scene_name } => {
                    client
                        .scenes()
                        .set_current_program_scene(scene_name)
                        .await
                        .map_err(|e| format!("Failed to switch scene: {}", e))?;
                }

                ObsAction::ToggleRecording => {
                    client
                        .recording()
                        .toggle()
                        .await
                        .map_err(|e| format!("Failed to toggle recording: {}", e))?;
                }

                ObsAction::StartRecording => {
                    client
                        .recording()
                        .start()
                        .await
                        .map_err(|e| format!("Failed to start recording: {}", e))?;
                }

                ObsAction::StopRecording => {
                    client
                        .recording()
                        .stop()
                        .await
                        .map_err(|e| format!("Failed to stop recording: {}", e))?;
                }

                ObsAction::PauseRecording => {
                    client
                        .recording()
                        .toggle_pause()
                        .await
                        .map_err(|e| format!("Failed to pause/resume recording: {}", e))?;
                }

                ObsAction::ToggleStreaming => {
                    client
                        .streaming()
                        .toggle()
                        .await
                        .map_err(|e| format!("Failed to toggle streaming: {}", e))?;
                }

                ObsAction::StartStreaming => {
                    client
                        .streaming()
                        .start()
                        .await
                        .map_err(|e| format!("Failed to start streaming: {}", e))?;
                }

                ObsAction::StopStreaming => {
                    client
                        .streaming()
                        .stop()
                        .await
                        .map_err(|e| format!("Failed to stop streaming: {}", e))?;
                }

                ObsAction::ToggleMute { source_name } => {
                    client
                        .inputs()
                        .toggle_mute(source_name)
                        .await
                        .map_err(|e| format!("Failed to toggle mute: {}", e))?;
                }

                ObsAction::SetVolume {
                    source_name,
                    volume,
                } => {
                    use obws::requests::inputs::Volume;
                    let volume_db = 20.0 * volume.log10();
                    client
                        .inputs()
                        .set_volume(source_name, Volume::Db(volume_db))
                        .await
                        .map_err(|e| format!("Failed to set volume: {}", e))?;
                }

                ObsAction::ToggleReplayBuffer => {
                    client
                        .replay_buffer()
                        .toggle()
                        .await
                        .map_err(|e| format!("Failed to toggle replay buffer: {}", e))?;
                }

                ObsAction::SaveReplayBuffer => {
                    client
                        .replay_buffer()
                        .save()
                        .await
                        .map_err(|e| format!("Failed to save replay buffer: {}", e))?;
                }

                ObsAction::ToggleStudioMode => {
                    let enabled = client
                        .ui()
                        .studio_mode_enabled()
                        .await
                        .map_err(|e| format!("Failed to get studio mode state: {}", e))?;

                    client
                        .ui()
                        .set_studio_mode_enabled(!enabled)
                        .await
                        .map_err(|e| format!("Failed to toggle studio mode: {}", e))?;
                }
            }

            Ok(())
        })
    }
}

impl Default for ObsPlugin {
    fn default() -> Self {
        Self::new().expect("Failed to create OBS plugin")
    }
}

impl ActionPlugin for ObsPlugin {
    fn name(&self) -> &str {
        "obs"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "Control OBS Studio through WebSocket protocol"
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Network]
    }

    fn execute(&mut self, params: Value, _context: TriggerContext) -> Result<(), Box<dyn Error>> {
        // Parse action from JSON
        let action: ObsAction = serde_json::from_value(params)
            .map_err(|e| format!("Invalid OBS action format: {}", e))?;

        // Execute the action
        self.execute_action(&action)
    }

    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        // Try to connect on initialization
        let _ = self.ensure_connected();
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        let mut state = self.state.lock().unwrap();
        if let Some(client) = state.client.take() {
            drop(client);
        }
        Ok(())
    }
}

// Export the plugin constructor
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn ActionPlugin {
    let plugin = Box::new(ObsPlugin::new().expect("Failed to create OBS plugin"));
    Box::into_raw(plugin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let plugin = ObsPlugin::new().unwrap();
        assert_eq!(plugin.name(), "obs");
        assert_eq!(plugin.version(), "0.1.0");
        assert!(plugin.capabilities().contains(&Capability::Network));
    }

    #[test]
    fn test_action_serialization() {
        let action = ObsAction::StartRecording;
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "start_recording");

        let action = ObsAction::SwitchScene {
            scene_name: "Scene 1".to_string(),
        };
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "switch_scene");
        assert_eq!(json["scene_name"], "Scene 1");
    }
}
