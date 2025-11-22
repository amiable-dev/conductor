// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! OBS Studio control via WebSocket protocol
//!
//! This plugin demonstrates network-based control as an alternative to process execution.
//! It provides a complete API surface that will work once v2.6's HTTP/WebSocket wrapper is implemented.
//!
//! ## OBS WebSocket Protocol
//!
//! OBS Studio provides a WebSocket API (obs-websocket v5.x) for remote control.
//! Default endpoint: ws://localhost:4455
//!
//! ## Supported Actions
//!
//! - `switch_scene` - Switch to a different scene
//! - `start_recording` - Start recording
//! - `stop_recording` - Stop recording
//! - `start_streaming` - Start streaming
//! - `stop_streaming` - Stop streaming
//! - `toggle_mute` - Toggle microphone mute
//! - `toggle_source` - Show/hide a source
//!
//! ## Setup Requirements
//!
//! 1. Install OBS Studio (https://obsproject.com/)
//! 2. Install obs-websocket plugin (usually included in OBS 28+)
//! 3. Enable WebSocket server in OBS:
//!    Tools → WebSocket Server Settings → Enable WebSocket server
//! 4. Note the port (default: 4455) and password
//!
//! ## Current Status (v2.5)
//!
//! **Note:** This plugin is a reference implementation for v2.6. WASI Preview1 doesn't
//! provide high-level HTTP/WebSocket APIs yet. This demonstrates the interface design.
//!
//! In v2.6, the runtime will provide a WebSocket wrapper that enables this plugin to work.

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Plugin metadata
static METADATA: &[u8] = br#"{
  "name": "obs_control",
  "version": "0.1.0",
  "description": "OBS Studio control via WebSocket protocol",
  "author": "Amiable Team",
  "license": "MIT",
  "type": "action",
  "capabilities": ["network"],
  "actions": [
    {
      "name": "switch_scene",
      "description": "Switch to a different OBS scene",
      "parameters": ["scene_name"]
    },
    {
      "name": "start_recording",
      "description": "Start OBS recording"
    },
    {
      "name": "stop_recording",
      "description": "Stop OBS recording"
    },
    {
      "name": "start_streaming",
      "description": "Start OBS streaming"
    },
    {
      "name": "stop_streaming",
      "description": "Stop OBS streaming"
    },
    {
      "name": "toggle_mute",
      "description": "Toggle microphone mute",
      "parameters": ["source_name"]
    },
    {
      "name": "toggle_source",
      "description": "Show/hide a source",
      "parameters": ["source_name"]
    }
  ],
  "obs_websocket_version": "5.x",
  "default_endpoint": "ws://localhost:4455"
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

/// OBS WebSocket request message (v5.x protocol)
#[derive(Debug, Serialize)]
struct OBSRequest {
    #[serde(rename = "op")]
    opcode: u8,  // 6 = Request
    d: OBSRequestData,
}

#[derive(Debug, Serialize)]
struct OBSRequestData {
    #[serde(rename = "requestType")]
    request_type: String,
    #[serde(rename = "requestId")]
    request_id: String,
    #[serde(rename = "requestData", skip_serializing_if = "Option::is_none")]
    request_data: Option<serde_json::Value>,
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

    // Execute the requested action
    match execute_obs_action(&request.action, &request.parameters) {
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
// OBS Control Logic
// ============================================================================

/// Execute an OBS action
///
/// Note: In v2.5, this demonstrates the interface. In v2.6, the runtime will provide
/// WebSocket functionality to make these calls work.
fn execute_obs_action(action: &str, params: &[String]) -> Result<(), &'static str> {
    match action {
        "switch_scene" => {
            let scene_name = params.first().ok_or("Missing scene_name parameter")?;
            send_obs_request("SetCurrentProgramScene", Some(scene_name))
        }
        "start_recording" => {
            send_obs_request("StartRecord", None)
        }
        "stop_recording" => {
            send_obs_request("StopRecord", None)
        }
        "start_streaming" => {
            send_obs_request("StartStream", None)
        }
        "stop_streaming" => {
            send_obs_request("StopStream", None)
        }
        "toggle_mute" => {
            let source_name = params.first().ok_or("Missing source_name parameter")?;
            send_obs_request("ToggleInputMute", Some(source_name))
        }
        "toggle_source" => {
            let source_name = params.first().ok_or("Missing source_name parameter")?;
            send_obs_request("ToggleSceneItemEnabled", Some(source_name))
        }
        _ => Err("Unknown action"),
    }
}

/// Send a request to OBS WebSocket
///
/// In v2.6, this will use the runtime's WebSocket wrapper.
/// For now, it demonstrates the protocol structure.
fn send_obs_request(request_type: &str, param: Option<&String>) -> Result<(), &'static str> {
    // Build request data
    let request_data = if let Some(p) = param {
        // Create JSON object with parameter
        // This is simplified - real implementation would use proper parameter names
        Some(serde_json::json!({"name": p}))
    } else {
        None
    };

    let request = OBSRequest {
        opcode: 6,  // Request opcode
        d: OBSRequestData {
            request_type: String::from(request_type),
            request_id: String::from("midimon-request"),
            request_data,
        },
    };

    // In v2.6, this would call: websocket_send("ws://localhost:4455", &request)
    // For now, we just validate the request can be serialized
    let _json = serde_json::to_string(&request)
        .map_err(|_| "Failed to serialize OBS request")?;

    // Placeholder: In v2.6, this will actually send the WebSocket message
    // For v2.5, we document the protocol and return success for testing
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_routing() {
        // Test that all actions route correctly
        assert!(execute_obs_action("start_recording", &[]).is_ok());
        assert!(execute_obs_action("stop_recording", &[]).is_ok());
        assert!(execute_obs_action("start_streaming", &[]).is_ok());
        assert!(execute_obs_action("stop_streaming", &[]).is_ok());
    }

    #[test]
    fn test_parametrized_actions() {
        let scene = String::from("Gaming");
        assert!(execute_obs_action("switch_scene", &[scene.clone()]).is_ok());

        let source = String::from("Microphone");
        assert!(execute_obs_action("toggle_mute", &[source]).is_ok());
    }

    #[test]
    fn test_missing_parameters() {
        // Should fail without required parameters
        assert!(execute_obs_action("switch_scene", &[]).is_err());
        assert!(execute_obs_action("toggle_mute", &[]).is_err());
    }

    #[test]
    fn test_unknown_action() {
        assert!(execute_obs_action("invalid_action", &[]).is_err());
    }
}
