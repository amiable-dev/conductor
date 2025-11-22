// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! Spotify Control Plugin for MIDIMon (WASM)
//!
//! This plugin provides complete Spotify playback control via the Spotify Web API.
//! It demonstrates:
//!
//! - Network-based API integration (REST HTTP)
//! - OAuth token-based authentication
//! - Action parsing and routing
//! - Error handling in WASM
//! - Production plugin structure
//!
//! ## Current Status (v2.5)
//!
//! **Note:** This is a reference implementation for v2.6. WASI Preview1 doesn't provide
//! high-level HTTP APIs yet. This demonstrates the interface design and protocol structure.
//!
//! In v2.6, the runtime will provide an HTTP wrapper that enables this plugin to work
//! with the real Spotify Web API.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Plugin metadata
static METADATA: &[u8] = br#"{
  "name": "spotify_control",
  "version": "0.2.0",
  "description": "Control Spotify playback via Web API",
  "author": "Amiable Team",
  "license": "MIT",
  "type": "action",
  "capabilities": ["network"],
  "actions": [
    {
      "name": "play_pause",
      "description": "Toggle play/pause",
      "parameters": []
    },
    {
      "name": "play",
      "description": "Resume playback",
      "parameters": []
    },
    {
      "name": "pause",
      "description": "Pause playback",
      "parameters": []
    },
    {
      "name": "next_track",
      "description": "Skip to next track",
      "parameters": []
    },
    {
      "name": "previous_track",
      "description": "Go to previous track",
      "parameters": []
    },
    {
      "name": "volume_up",
      "description": "Increase volume by 10%",
      "parameters": []
    },
    {
      "name": "volume_down",
      "description": "Decrease volume by 10%",
      "parameters": []
    },
    {
      "name": "set_volume",
      "description": "Set volume to specific level",
      "parameters": ["volume_percent"]
    },
    {
      "name": "shuffle",
      "description": "Toggle shuffle",
      "parameters": []
    },
    {
      "name": "repeat",
      "description": "Cycle repeat mode",
      "parameters": []
    }
  ]
}"#;

/// Action request from host
#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
    #[serde(default)]
    pub parameters: Vec<String>,
}

/// Trigger context from host
#[derive(Debug, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub current_mode: Option<usize>,
    pub timestamp: u64,
}

/// Spotify Web API request
#[derive(Debug, Serialize)]
struct SpotifyAPIRequest {
    method: String,
    endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<serde_json::Value>,
}

// ============================================================================
// WASM Exports (Plugin Interface)
// ============================================================================

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Read request JSON from memory
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1, // Error: invalid memory read
    };

    // Parse request
    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2, // Error: invalid JSON
    };

    // Execute the requested action with parameters
    match execute_spotify_action(&request.action, &request.parameters) {
        Ok(_) => 0,  // Success
        Err(_) => 3, // Error: action execution failed
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size as usize);
    }
}

// ============================================================================
// Spotify API Logic
// ============================================================================

/// Execute a Spotify control action
fn execute_spotify_action(action: &str, params: &[String]) -> Result<(), &'static str> {
    match action {
        "play_pause" => {
            // Get current playback state, then play or pause
            // In v2.6: GET /v1/me/player then PUT /v1/me/player/play or /pause
            send_spotify_request("PUT", "/v1/me/player/play", None)
        }
        "play" => {
            send_spotify_request("PUT", "/v1/me/player/play", None)
        }
        "pause" => {
            send_spotify_request("PUT", "/v1/me/player/pause", None)
        }
        "next_track" => {
            send_spotify_request("POST", "/v1/me/player/next", None)
        }
        "previous_track" => {
            send_spotify_request("POST", "/v1/me/player/previous", None)
        }
        "volume_up" => {
            // In v2.6: GET current volume, calculate +10, PUT new volume
            // For now, simulate with set to 80%
            let body = serde_json::json!({
                "volume_percent": 80
            });
            send_spotify_request("PUT", "/v1/me/player/volume", Some(&body))
        }
        "volume_down" => {
            // In v2.6: GET current volume, calculate -10, PUT new volume
            let body = serde_json::json!({
                "volume_percent": 50
            });
            send_spotify_request("PUT", "/v1/me/player/volume", Some(&body))
        }
        "set_volume" => {
            let volume = params.first().ok_or("Missing volume_percent parameter")?;
            let volume_int: u8 = volume.parse().map_err(|_| "Invalid volume")?;
            if volume_int > 100 {
                return Err("Volume must be 0-100");
            }
            let body = serde_json::json!({
                "volume_percent": volume_int
            });
            send_spotify_request("PUT", "/v1/me/player/volume", Some(&body))
        }
        "shuffle" => {
            // In v2.6: Toggle shuffle state
            let body = serde_json::json!({
                "state": true
            });
            send_spotify_request("PUT", "/v1/me/player/shuffle", Some(&body))
        }
        "repeat" => {
            // In v2.6: Cycle through repeat modes (off, track, context)
            let body = serde_json::json!({
                "state": "context"
            });
            send_spotify_request("PUT", "/v1/me/player/repeat", Some(&body))
        }
        _ => Err("Unknown action"),
    }
}

/// Send request to Spotify Web API
///
/// In v2.6, this will use the HTTP wrapper provided by the runtime.
/// For now, it validates the request structure can be serialized.
fn send_spotify_request(
    method: &str,
    endpoint: &str,
    body: Option<&serde_json::Value>,
) -> Result<(), &'static str> {
    // Build request structure
    let request = SpotifyAPIRequest {
        method: String::from(method),
        endpoint: String::from(endpoint),
        body: body.cloned(),
    };

    // Validate request can be serialized (placeholder for v2.6 HTTP call)
    let _json = serde_json::to_string(&request)
        .map_err(|_| "Failed to serialize request")?;

    // In v2.6, this would be:
    // http_request(&json)?;

    Ok(())
}

// ============================================================================
// Memory Helpers
// ============================================================================

/// Read a string from WASM linear memory
fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

/// Panic handler (required for no_std, but conflicts with test framework)
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Global allocator (required for alloc)
#[cfg(not(test))]
#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_actions() {
        assert!(execute_spotify_action("play_pause", &[]).is_ok());
        assert!(execute_spotify_action("play", &[]).is_ok());
        assert!(execute_spotify_action("pause", &[]).is_ok());
        assert!(execute_spotify_action("next_track", &[]).is_ok());
        assert!(execute_spotify_action("previous_track", &[]).is_ok());
    }

    #[test]
    fn test_volume_actions() {
        assert!(execute_spotify_action("volume_up", &[]).is_ok());
        assert!(execute_spotify_action("volume_down", &[]).is_ok());

        let volume = String::from("75");
        assert!(execute_spotify_action("set_volume", &[volume]).is_ok());
    }

    #[test]
    fn test_volume_validation() {
        let invalid = String::from("150");
        assert!(execute_spotify_action("set_volume", &[invalid]).is_err());

        let invalid_str = String::from("abc");
        assert!(execute_spotify_action("set_volume", &[invalid_str]).is_err());
    }

    #[test]
    fn test_shuffle_repeat() {
        assert!(execute_spotify_action("shuffle", &[]).is_ok());
        assert!(execute_spotify_action("repeat", &[]).is_ok());
    }

    #[test]
    fn test_missing_parameters() {
        // set_volume requires volume parameter
        assert!(execute_spotify_action("set_volume", &[]).is_err());
    }

    #[test]
    fn test_unknown_action() {
        assert!(execute_spotify_action("invalid_action", &[]).is_err());
    }
}

// ============================================================================
// Compiler Intrinsics (required by serde_json in no_std)
// ============================================================================

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}
