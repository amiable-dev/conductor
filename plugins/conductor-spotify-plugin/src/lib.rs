// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Spotify integration plugin for MIDIMon
//!
//! This plugin provides actions for controlling Spotify playback through the Spotify Web API.
//! Supports play/pause, next/previous track, volume control, and playlist management.

use midimon_core::plugin::{ActionPlugin, Capability, TriggerContext};
use rspotify::{
    model::{PlayableItem, RepeatState},
    prelude::*,
    AuthCodeSpotify, Credentials, OAuth, Token,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

/// Spotify action types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpotifyAction {
    Play,
    Pause,
    PlayPause,
    NextTrack,
    PreviousTrack,
    SetVolume { volume: u8 },
    AdjustVolume { delta: i8 },
    ToggleShuffle,
    CycleRepeat,
    PlayPlaylist { uri: String },
    LikeCurrentTrack,
}

/// Spotify plugin state
struct SpotifyPluginState {
    client: AuthCodeSpotify,
    is_authenticated: bool,
}

/// Spotify integration plugin
pub struct SpotifyPlugin {
    state: Arc<Mutex<SpotifyPluginState>>,
    runtime: Runtime,
}

impl SpotifyPlugin {
    /// Create a new Spotify plugin instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Create tokio runtime for async operations
        let runtime = Runtime::new()?;

        // Initialize Spotify client
        let creds = Credentials::from_env().unwrap_or_else(|| {
            eprintln!("Warning: RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET not found");
            Credentials::default()
        });

        let oauth = OAuth::from_env(rspotify::scopes!(
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "user-library-modify",
            "user-library-read",
            "playlist-read-private"
        ))
        .unwrap_or_default();

        let client = AuthCodeSpotify::new(creds, oauth);

        Ok(Self {
            state: Arc::new(Mutex::new(SpotifyPluginState {
                client,
                is_authenticated: false,
            })),
            runtime,
        })
    }

    /// Ensure the client is authenticated
    fn ensure_authenticated(&self) -> Result<(), Box<dyn Error>> {
        let mut state = self.state.lock().unwrap();

        if state.is_authenticated {
            return Ok(());
        }

        // Try to load cached token
        if let Ok(token) = Token::from_cache("spotify_token_cache") {
            // Use tokio runtime to set the token
            self.runtime.block_on(async {
                *state.client.token.lock().await.unwrap() = Some(token);
            });
            state.is_authenticated = true;
            return Ok(());
        }

        Err("Spotify not authenticated. Please run 'midimon spotify auth' first".into())
    }

    /// Execute a Spotify action (sync wrapper around async operations)
    fn execute_action(&self, action: &SpotifyAction) -> Result<(), Box<dyn Error>> {
        self.ensure_authenticated()?;

        let state = self.state.lock().unwrap();
        let client = &state.client;

        // Use the runtime to execute async operations synchronously
        self.runtime.block_on(async {
            match action {
                SpotifyAction::Play => {
                    client
                        .resume_playback(None, None)
                        .await
                        .map_err(|e| format!("Failed to play: {}", e))?;
                }

                SpotifyAction::Pause => {
                    client
                        .pause_playback(None)
                        .await
                        .map_err(|e| format!("Failed to pause: {}", e))?;
                }

                SpotifyAction::PlayPause => {
                    if let Some(playback) = client.current_playback(None, None::<Vec<_>>).await.ok().flatten() {
                        if playback.is_playing {
                            client.pause_playback(None).await.map_err(|e| format!("Failed to pause: {}", e))?;
                        } else {
                            client.resume_playback(None, None).await.map_err(|e| format!("Failed to play: {}", e))?;
                        }
                    } else {
                        return Err("No active playback device".into());
                    }
                }

                SpotifyAction::NextTrack => {
                    client
                        .next_track(None)
                        .await
                        .map_err(|e| format!("Failed to skip: {}", e))?;
                }

                SpotifyAction::PreviousTrack => {
                    client
                        .previous_track(None)
                        .await
                        .map_err(|e| format!("Failed to go back: {}", e))?;
                }

                SpotifyAction::SetVolume { volume } => {
                    let volume_percent = (*volume).min(100);
                    client
                        .volume(volume_percent, None)
                        .await
                        .map_err(|e| format!("Failed to set volume: {}", e))?;
                }

                SpotifyAction::AdjustVolume { delta } => {
                    if let Some(playback) = client.current_playback(None, None::<Vec<_>>).await.ok().flatten() {
                        if let Some(device_volume) = playback.device.volume_percent {
                            let new_volume = (device_volume as i16 + *delta as i16).clamp(0, 100) as u8;
                            client.volume(new_volume, None).await.map_err(|e| format!("Failed to adjust volume: {}", e))?;
                        } else {
                            return Err("Device volume not available".into());
                        }
                    } else {
                        return Err("No active playback".into());
                    }
                }

                SpotifyAction::ToggleShuffle => {
                    if let Some(playback) = client.current_playback(None, None::<Vec<_>>).await.ok().flatten() {
                        let new_state = !playback.shuffle_state;
                        client.shuffle(new_state, None).await.map_err(|e| format!("Failed to toggle shuffle: {}", e))?;
                    } else {
                        return Err("No active playback".into());
                    }
                }

                SpotifyAction::CycleRepeat => {
                    if let Some(playback) = client.current_playback(None, None::<Vec<_>>).await.ok().flatten() {
                        let new_state = match playback.repeat_state {
                            RepeatState::Off => RepeatState::Context,
                            RepeatState::Context => RepeatState::Track,
                            RepeatState::Track => RepeatState::Off,
                        };
                        client.repeat(new_state, None).await.map_err(|e| format!("Failed to cycle repeat: {}", e))?;
                    } else {
                        return Err("No active playback".into());
                    }
                }

                SpotifyAction::PlayPlaylist { uri } => {
                    // Parse playlist URI and wrap in PlayContextId
                    use rspotify::model::{PlaylistId, PlayContextId};
                    let playlist_id = PlaylistId::from_uri(uri).map_err(|e| format!("Invalid playlist URI: {}", e))?;
                    let context = PlayContextId::Playlist(playlist_id);
                    client
                        .start_context_playback(context, None, None, None)
                        .await
                        .map_err(|e| format!("Failed to play playlist: {}", e))?;
                }

                SpotifyAction::LikeCurrentTrack => {
                    if let Some(playing) = client.current_playing(None, None::<Vec<_>>).await.ok().flatten() {
                        if let Some(PlayableItem::Track(track)) = playing.item {
                            if let Some(track_id) = track.id {
                                client
                                    .current_user_saved_tracks_add(vec![track_id])
                                    .await
                                    .map_err(|e| format!("Failed to like track: {}", e))?;
                            } else {
                                return Err("Track has no ID".into());
                            }
                        } else {
                            return Err("No track currently playing".into());
                        }
                    } else {
                        return Err("Nothing currently playing".into());
                    }
                }
            }

            Ok(())
        })
    }
}

impl Default for SpotifyPlugin {
    fn default() -> Self {
        Self::new().expect("Failed to create Spotify plugin")
    }
}

impl ActionPlugin for SpotifyPlugin {
    fn name(&self) -> &str {
        "spotify"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "Control Spotify playback through the Spotify Web API"
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Network, Capability::Filesystem]
    }

    fn execute(&mut self, params: Value, _context: TriggerContext) -> Result<(), Box<dyn Error>> {
        // Parse action from JSON
        let action: SpotifyAction = serde_json::from_value(params)
            .map_err(|e| format!("Invalid Spotify action format: {}", e))?;

        // Execute the action
        self.execute_action(&action)
    }

    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        // Try to load cached authentication
        let _ = self.ensure_authenticated();
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

// Export the plugin constructor
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn ActionPlugin {
    let plugin = Box::new(SpotifyPlugin::new().expect("Failed to create Spotify plugin"));
    Box::into_raw(plugin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let plugin = SpotifyPlugin::new().unwrap();
        assert_eq!(plugin.name(), "spotify");
        assert_eq!(plugin.version(), "0.1.0");
        assert!(plugin.capabilities().contains(&Capability::Network));
    }

    #[test]
    fn test_action_serialization() {
        let action = SpotifyAction::Play;
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "play");

        let action = SpotifyAction::SetVolume { volume: 75 };
        let json = serde_json::to_value(&action).unwrap();
        assert_eq!(json["type"], "set_volume");
        assert_eq!(json["volume"], 75);
    }
}
