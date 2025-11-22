// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon WASM Plugin Template
//!
//! This template provides the foundation for building sandboxed MIDIMon plugins
//! compiled to WebAssembly. WASM plugins run in an isolated environment with:
//!
//! - Memory isolation (separate linear memory)
//! - Resource limits (configurable max memory, execution time)
//! - Capability-based permissions (filesystem, network via WASI)
//! - Platform independence (same .wasm runs on macOS/Linux/Windows)
//!
//! ## Plugin Interface
//!
//! Your plugin must export two functions:
//!
//! 1. `init() -> (ptr: u32, len: u32)` - Returns JSON metadata
//! 2. `execute(ptr: u32, len: u32) -> i32` - Executes an action
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize)]
//! struct PluginMetadata {
//!     name: String,
//!     version: String,
//!     description: String,
//!     actions: Vec<String>,
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn init() -> (u32, u32) {
//!     let metadata = PluginMetadata {
//!         name: "My Plugin".to_string(),
//!         version: "1.0.0".to_string(),
//!         description: "Example WASM plugin".to_string(),
//!         actions: vec!["my_action".to_string()],
//!     };
//!
//!     let json = serde_json::to_string(&metadata).unwrap();
//!     let (ptr, len) = allocate_string(&json);
//!     (ptr as u32, len as u32)
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Optional: Use wee_alloc for smaller binary size
#[cfg(feature = "wee_alloc_support")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Plugin Metadata
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub license: String,
    #[serde(rename = "type")]
    pub plugin_type: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

// ============================================================================
// Action Request/Response
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub note: Option<u8>,
    pub cc: Option<u8>,
    pub value: Option<u8>,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}

// ============================================================================
// WASM Exports (Required Interface)
// ============================================================================

/// Pre-computed metadata JSON (static to avoid allocation issues)
static METADATA_JSON: &[u8] = br#"{"name":"example_wasm_plugin","version":"0.1.0","description":"Example WASM plugin for MIDIMon","author":"Amiable","license":"MIT","type":"action","capabilities":[]}"#;

/// Initialize the plugin and return metadata
///
/// Returns: (ptr, len) pointer and length of JSON metadata string (packed in u64)
#[no_mangle]
pub extern "C" fn init() -> u64 {
    // Return pointer to static data in WASM linear memory
    let ptr = METADATA_JSON.as_ptr() as u32;
    let len = METADATA_JSON.len() as u32;

    // Pack ptr and len into single u64 (ptr in high 32 bits, len in low 32 bits)
    ((ptr as u64) << 32) | (len as u64)
}

/// Execute a plugin action
///
/// Arguments:
/// - ptr: Pointer to JSON request string in WASM memory
/// - len: Length of JSON request string
///
/// Returns: 0 on success, non-zero error code on failure
#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Read request from WASM memory
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1, // Error: invalid memory read
    };

    // Parse request
    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2, // Error: invalid JSON
    };

    // Execute action
    match execute_action(&request.action, &request.context) {
        Ok(_) => 0,  // Success
        Err(_) => 3, // Error: action execution failed
    }
}

/// Allocate memory in WASM linear memory
///
/// This function is called by the host to allocate space for passing strings
#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf); // Prevent deallocation
    ptr
}

/// Deallocate memory in WASM linear memory
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size as usize);
        // Drop will deallocate
    }
}

// ============================================================================
// Plugin Action Implementation
// ============================================================================

fn execute_action(action: &str, context: &TriggerContext) -> Result<(), String> {
    match action {
        "hello" => {
            println!("Hello from WASM plugin!");
            println!("Velocity: {:?}", context.velocity);
            Ok(())
        }
        "greet" => {
            let name = context
                .metadata
                .get("name")
                .map(|s| s.as_str())
                .unwrap_or("World");
            println!("Hello, {}!", name);
            Ok(())
        }
        "goodbye" => {
            println!("Goodbye from WASM plugin!");
            Ok(())
        }
        _ => Err(format!("Unknown action: {}", action)),
    }
}

// ============================================================================
// Memory Helpers
// ============================================================================

/// Allocate a string in WASM memory and return (ptr, len)
fn allocate_string(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let ptr = alloc(len as u32);

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, len);
    }

    (ptr as usize, len)
}

/// Read a string from WASM memory
fn read_string(ptr: usize, len: usize) -> Result<String, std::string::FromUtf8Error> {
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_serialization() {
        let metadata = PluginMetadata {
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            author: "Test Author".to_string(),
            actions: vec!["test_action".to_string()],
            capabilities: vec![],
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: PluginMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Test Plugin");
        assert_eq!(deserialized.version, "1.0.0");
    }

    #[test]
    fn test_request_serialization() {
        let context = TriggerContext {
            velocity: Some(100),
            note: Some(60),
            cc: None,
            value: None,
            timestamp: 12345,
            metadata: HashMap::new(),
        };

        let request = ActionRequest {
            action: "test".to_string(),
            context,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ActionRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.action, "test");
        assert_eq!(deserialized.context.velocity, Some(100));
    }

    #[test]
    fn test_execute_action_logic() {
        let context = TriggerContext {
            velocity: Some(100),
            note: Some(60),
            cc: None,
            value: None,
            timestamp: 12345,
            metadata: HashMap::new(),
        };

        // Test valid actions
        assert!(execute_action("hello", &context).is_ok());
        assert!(execute_action("greet", &context).is_ok());
        assert!(execute_action("goodbye", &context).is_ok());

        // Test invalid action
        assert!(execute_action("unknown", &context).is_err());
    }
}
