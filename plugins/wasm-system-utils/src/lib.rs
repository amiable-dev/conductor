// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! System utilities plugin for common system interactions
//!
//! This plugin demonstrates practical system utilities that are commonly needed:
//! - Clipboard operations (copy/paste)
//! - Screenshot capture
//! - System notifications
//! - Window management
//! - Volume control
//!
//! ## Current Status (v2.5)
//!
//! **Note:** This is a reference implementation for v2.6. Some capabilities like
//! filesystem access and subprocess execution are limited in WASI Preview1.
//!
//! This demonstrates the interface design. In v2.6:
//! - Filesystem operations will use directory preopening
//! - Screenshot capture may use subprocess capability
//! - Clipboard access will use host functions

#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Plugin metadata
static METADATA: &[u8] = br#"{
  "name": "system_utils",
  "version": "0.1.0",
  "description": "System utilities: clipboard, screenshots, notifications",
  "author": "Amiable Team",
  "license": "MIT",
  "type": "action",
  "capabilities": ["filesystem", "subprocess", "systemcontrol"],
  "actions": [
    {
      "name": "copy_to_clipboard",
      "description": "Copy text to system clipboard",
      "parameters": ["text"]
    },
    {
      "name": "paste_from_clipboard",
      "description": "Paste text from system clipboard"
    },
    {
      "name": "take_screenshot",
      "description": "Capture screenshot to file",
      "parameters": ["filename"]
    },
    {
      "name": "show_notification",
      "description": "Display system notification",
      "parameters": ["title", "message"]
    },
    {
      "name": "minimize_window",
      "description": "Minimize the active window"
    },
    {
      "name": "maximize_window",
      "description": "Maximize the active window"
    },
    {
      "name": "mute_system",
      "description": "Mute system audio"
    },
    {
      "name": "unmute_system",
      "description": "Unmute system audio"
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

/// System command to execute
#[derive(Debug, Serialize)]
struct SystemCommand {
    command: String,
    args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stdin: Option<String>,
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
    match execute_system_action(&request.action, &request.parameters) {
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
// System Utilities Logic
// ============================================================================

/// Execute a system utility action
fn execute_system_action(action: &str, params: &[String]) -> Result<(), &'static str> {
    match action {
        "copy_to_clipboard" => {
            let _text = params.first().ok_or("Missing text parameter")?;
            // Validate command structure
            let cmd = SystemCommand {
                command: String::from("pbcopy"),
                args: Vec::new(),
                stdin: Some(String::from("placeholder")),
            };
            validate_command(&cmd)
        }
        "paste_from_clipboard" => {
            let cmd = SystemCommand {
                command: String::from("pbpaste"),
                args: Vec::new(),
                stdin: None,
            };
            validate_command(&cmd)
        }
        "take_screenshot" => {
            let filename = params.first().ok_or("Missing filename parameter")?;
            let cmd = SystemCommand {
                command: String::from("screencapture"),
                args: vec![String::from("-x"), filename.clone()],
                stdin: None,
            };
            validate_command(&cmd)
        }
        "show_notification" => {
            let _title = params.get(0).ok_or("Missing title parameter")?;
            let _message = params.get(1).ok_or("Missing message parameter")?;
            let cmd = SystemCommand {
                command: String::from("osascript"),
                args: vec![String::from("-e")],
                stdin: None,
            };
            validate_command(&cmd)
        }
        "minimize_window" | "maximize_window" => {
            let cmd = SystemCommand {
                command: String::from("osascript"),
                args: vec![String::from("-e")],
                stdin: None,
            };
            validate_command(&cmd)
        }
        "mute_system" | "unmute_system" => {
            let cmd = SystemCommand {
                command: String::from("osascript"),
                args: vec![String::from("-e")],
                stdin: None,
            };
            validate_command(&cmd)
        }
        _ => Err("Unknown action"),
    }
}

/// Validate command structure (placeholder for v2.6 subprocess execution)
fn validate_command(command: &SystemCommand) -> Result<(), &'static str> {
    // Validate command can be serialized
    let _json = serde_json::to_string(command)
        .map_err(|_| "Failed to serialize command")?;

    // In v2.6, this would call: subprocess_exec(&command)
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
    fn test_clipboard_actions() {
        let text = String::from("test text");
        assert!(execute_system_action("copy_to_clipboard", &[text]).is_ok());
        assert!(execute_system_action("paste_from_clipboard", &[]).is_ok());
    }

    #[test]
    fn test_screenshot_action() {
        let filename = String::from("screenshot.png");
        assert!(execute_system_action("take_screenshot", &[filename]).is_ok());
    }

    #[test]
    fn test_notification_action() {
        let title = String::from("Test");
        let message = String::from("This is a test");
        assert!(execute_system_action("show_notification", &[title, message]).is_ok());
    }

    #[test]
    fn test_window_actions() {
        assert!(execute_system_action("minimize_window", &[]).is_ok());
        assert!(execute_system_action("maximize_window", &[]).is_ok());
    }

    #[test]
    fn test_audio_actions() {
        assert!(execute_system_action("mute_system", &[]).is_ok());
        assert!(execute_system_action("unmute_system", &[]).is_ok());
    }

    #[test]
    fn test_missing_parameters() {
        // Should fail without required parameters
        assert!(execute_system_action("copy_to_clipboard", &[]).is_err());
        assert!(execute_system_action("take_screenshot", &[]).is_err());
        assert!(execute_system_action("show_notification", &[]).is_err());
    }

    #[test]
    fn test_unknown_action() {
        assert!(execute_system_action("invalid_action", &[]).is_err());
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
