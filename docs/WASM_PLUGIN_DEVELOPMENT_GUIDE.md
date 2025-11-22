# WASM Plugin Development Guide

**Version:** 2.5
**Last Updated:** 2025-01-18

---

## Table of Contents

1. [Introduction](#introduction)
2. [Quick Start](#quick-start)
3. [Plugin Architecture](#plugin-architecture)
4. [Development Setup](#development-setup)
5. [Creating Your First Plugin](#creating-your-first-plugin)
6. [Plugin Interface Specification](#plugin-interface-specification)
7. [Working with JSON in no_std](#working-with-json-in-nostd)
8. [Memory Management](#memory-management)
9. [Testing Your Plugin](#testing-your-plugin)
10. [Best Practices](#best-practices)
11. [Common Issues & Solutions](#common-issues--solutions)
12. [Performance Optimization](#performance-optimization)
13. [Example Plugins](#example-plugins)

---

## Introduction

MIDIMon's WASM plugin system allows you to extend MIDIMon's functionality with sandboxed, secure plugins written in Rust (or any language that compiles to WASM). Plugins run in an isolated environment with:

- **Memory isolation**: Plugins can't access host memory
- **Fuel metering**: CPU usage limits prevent infinite loops
- **Capability-based security**: Fine-grained permission system
- **Cross-platform**: WASM runs identically on macOS, Linux, and Windows

### Why WASM?

- **Security**: Sandboxed execution prevents malicious code
- **Performance**: Near-native execution speed
- **Size**: Tiny binaries (300 bytes to ~100KB)
- **Portability**: Write once, run everywhere
- **Language flexibility**: Rust, C, C++, AssemblyScript, etc.

---

## Quick Start

### Prerequisites

1. **Rust toolchain** (stable 1.88+)
2. **WASM target** installed:
   ```bash
   rustup target add wasm32-wasip1
   ```

### 30-Second Plugin

```bash
# Clone the minimal template
cp -r plugins/wasm-minimal my-plugin
cd my-plugin

# Edit src/lib.rs (add your logic)

# Build
cargo build --target wasm32-wasip1 --release

# Your plugin is ready!
ls target/wasm32-wasip1/release/*.wasm
```

---

## Plugin Architecture

### Plugin Lifecycle

```
┌─────────────────────────────────────────────┐
│  1. Host loads WASM module                  │
│     - Validates exports                     │
│     - Sets up WASI context                  │
│     - Allocates fuel (100M instructions)    │
└──────────────────┬──────────────────────────┘
                   ▼
┌─────────────────────────────────────────────┐
│  2. Host calls init()                       │
│     - Plugin returns metadata (ptr, len)    │
│     - Host reads metadata from memory       │
│     - Validates plugin info                 │
└──────────────────┬──────────────────────────┘
                   ▼
┌─────────────────────────────────────────────┐
│  3. Host calls execute(action, context)     │
│     - Plugin receives JSON request          │
│     - Plugin processes action               │
│     - Returns success/error code            │
└─────────────────────────────────────────────┘
```

### Required Exports

Every plugin MUST export these 4 functions:

1. **`init() -> u64`**: Return plugin metadata
2. **`execute(ptr: u32, len: u32) -> i32`**: Execute an action
3. **`alloc(size: u32) -> *mut u8`**: Allocate memory
4. **`dealloc(ptr: *mut u8, size: u32)`**: Free memory

---

## Development Setup

### Project Structure

```
my-plugin/
├── Cargo.toml          # Plugin manifest
├── src/
│   └── lib.rs          # Plugin implementation
└── target/
    └── wasm32-wasip1/
        └── release/
            └── my_plugin.wasm  # Compiled plugin
```

### Cargo.toml Template

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[workspace]
# Empty workspace to exclude from parent

[lib]
crate-type = ["cdylib"]

[dependencies]
# No dependencies for minimal plugin
# For JSON support, add:
# serde = { version = "1.0", features = ["derive"], default-features = false }
# serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
# wee_alloc = "0.4"

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable link-time optimization
codegen-units = 1   # Reduce number of codegen units
panic = "abort"     # Abort on panic
strip = true        # Strip symbols
```

### Build Script

Create `build.sh`:

```bash
#!/bin/bash
set -e

echo "Building WASM plugin..."
cargo build --target wasm32-wasip1 --release

WASM_FILE="target/wasm32-wasip1/release/my_plugin.wasm"
SIZE=$(ls -lh "$WASM_FILE" | awk '{print $5}')

echo "✅ Plugin built successfully!"
echo "   Size: $SIZE"
echo "   Path: $WASM_FILE"
```

---

## Creating Your First Plugin

### Minimal Plugin (300 bytes)

This is the smallest possible plugin - no dependencies, no allocator:

```rust
// src/lib.rs
#![no_std]

// Plugin metadata (embedded in binary)
static METADATA: &[u8] = br#"{
  "name": "my_plugin",
  "version": "0.1.0",
  "description": "My first WASM plugin",
  "author": "Your Name",
  "license": "MIT",
  "type": "action",
  "capabilities": []
}"#;

/// Initialize plugin and return metadata
///
/// Returns: u64 with ptr in high 32 bits, len in low 32 bits
#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

/// Execute plugin action
///
/// Arguments:
/// - ptr: Pointer to JSON request in WASM memory
/// - len: Length of JSON request
///
/// Returns: 0 on success, error code on failure
#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    // Your plugin logic here
    0 // Success
}

/// Allocate memory (called by host)
#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    use core::alloc::{alloc, Layout};
    unsafe {
        let layout = Layout::from_size_align_unchecked(size as usize, 1);
        alloc(layout)
    }
}

/// Deallocate memory (called by host)
#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    use core::alloc::{dealloc, Layout};
    unsafe {
        let layout = Layout::from_size_align_unchecked(size as usize, 1);
        dealloc(ptr, layout);
    }
}

/// Panic handler (required for no_std)
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Global allocator (required for alloc)
#[global_allocator]
static ALLOCATOR: core::alloc::System = core::alloc::System;
```

**Build it:**
```bash
cargo build --target wasm32-wasip1 --release
```

**Result:** ~300 byte WASM file!

---

## Plugin Interface Specification

### Metadata Format

The `init()` function must return a pointer to a JSON object with this structure:

```json
{
  "name": "plugin_name",           // Required: Plugin identifier
  "version": "0.1.0",               // Required: Semantic version
  "description": "What it does",    // Required: Short description
  "author": "Your Name",            // Required: Author name
  "homepage": "https://...",        // Optional: Plugin homepage
  "license": "MIT",                 // Required: License (MIT, Apache-2.0, etc.)
  "type": "action",                 // Required: Plugin type (always "action")
  "capabilities": []                // Required: Requested capabilities array
}
```

### Capabilities

Plugins can request specific capabilities:

```rust
"capabilities": ["Network", "Filesystem"]
```

Available capabilities:
- `Network`: Network access (HTTP requests, etc.)
- `Filesystem`: File system access (via preopened directories)
- `Process`: Execute external processes

**Important:** Capabilities must be granted by the user before plugin can use them.

### Execute Request Format

When `execute()` is called, the host passes a JSON request:

```json
{
  "action": "action_name",
  "context": {
    "velocity": 100,
    "current_mode": 0,
    "timestamp": 1705612800000
  }
}
```

**Fields:**
- `action` (string): Action to execute (e.g., "play_pause")
- `context.velocity` (u8, optional): MIDI velocity (0-127)
- `context.current_mode` (usize, optional): Current MIDIMon mode
- `context.timestamp` (u64): Unix epoch milliseconds

### Return Codes

The `execute()` function returns an i32 status code:

- `0`: Success
- `1`: Invalid memory read
- `2`: Invalid JSON
- `3`: Action execution failed
- `4+`: Custom error codes

---

## Working with JSON in no_std

### Setup

Add to `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"], default-features = false }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
wee_alloc = "0.4"
```

### Memory Intrinsics

**CRITICAL:** serde_json requires memory intrinsics. Add these to your plugin:

```rust
extern crate alloc;

/// Global allocator (required for alloc)
#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ============================================================================
// Compiler Intrinsics (required by serde_json in no_std)
// ============================================================================

/// Memory comparison intrinsic
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

/// Memory copy intrinsic
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}

/// Memory set intrinsic
#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.add(i) = c as u8;
        i += 1;
    }
    s
}
```

### Parsing JSON

```rust
use serde::{Deserialize, Serialize};
use alloc::string::String;

#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

#[derive(Debug, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub current_mode: Option<usize>,
    pub timestamp: u64,
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Read request JSON from memory
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1, // Invalid memory read
    };

    // Parse request
    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2, // Invalid JSON
    };

    // Execute action
    match request.action.as_str() {
        "my_action" => {
            // Do something
            0 // Success
        }
        _ => 3, // Unknown action
    }
}

/// Read a string from WASM linear memory
fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}
```

---

## Memory Management

### Allocation Strategy

The host calls your `alloc()` function to allocate memory for passing data to the plugin.

**Simple implementation (minimal plugin):**
```rust
#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    use core::alloc::{alloc, Layout};
    unsafe {
        let layout = Layout::from_size_align_unchecked(size as usize, 1);
        alloc(layout)
    }
}
```

**With allocator (JSON-enabled plugin):**
```rust
use alloc::vec::Vec;

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
```

### Memory Limits

Plugins have a 128MB memory limit by default (configurable by host).

**Tips:**
- Use `wee_alloc` to minimize allocator overhead (~1KB vs ~10KB)
- Avoid large allocations in hot paths
- Reuse buffers when possible

---

## Testing Your Plugin

### Unit Tests (Native)

Add tests to your plugin:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_action() {
        // Test your action logic
        assert_eq!(execute_my_action(), Ok(()));
    }
}
```

Run with:
```bash
cargo test
```

### Integration Tests (WASM)

Create a test in the host project:

```rust
// midimon-core/tests/my_plugin_test.rs
#![cfg(all(test, feature = "plugin-wasm"))]

use midimon_core::plugin::{
    wasm_runtime::{WasmConfig, WasmPlugin},
    TriggerContext,
};
use std::path::PathBuf;

fn get_plugin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../plugins/my-plugin/target/wasm32-wasip1/release/my_plugin.wasm")
}

#[tokio::test]
async fn test_load_plugin() {
    let wasm_path = get_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping: Plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_plugin_metadata() {
    let wasm_path = get_plugin_path();
    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config).await
        .expect("Failed to load plugin");

    let metadata = plugin.init().await
        .expect("Failed to initialize plugin");

    assert_eq!(metadata.name, "my_plugin");
    assert_eq!(metadata.version, "0.1.0");
}

#[tokio::test]
async fn test_execute_action() {
    let wasm_path = get_plugin_path();
    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config).await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize");

    let context = TriggerContext::with_velocity(100);
    let result = plugin.execute("my_action", &context).await;

    assert!(result.is_ok());
}
```

Run with:
```bash
cargo test --package midimon-core --test my_plugin_test --features plugin-wasm
```

---

## Best Practices

### 1. Keep It Small

- Use `wee_alloc` for smaller binaries
- Minimize dependencies
- Use `opt-level = "z"` for size optimization

### 2. Error Handling

Always validate inputs and return appropriate error codes:

```rust
#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Validate pointer
    if ptr == 0 {
        return 1; // Invalid pointer
    }

    // Parse with error handling
    let request = match parse_request(ptr, len) {
        Ok(r) => r,
        Err(_) => return 2, // Parse error
    };

    // Execute with error handling
    match execute_action(&request.action) {
        Ok(_) => 0,
        Err(_) => 3, // Execution failed
    }
}
```

### 3. Performance

- Minimize allocations in hot paths
- Cache computed values
- Use static data when possible
- Avoid string allocations if not needed

### 4. Security

- Never trust input data
- Validate all external inputs
- Use safe Rust (avoid `unsafe` unless necessary)
- Request minimal capabilities

### 5. Documentation

Add documentation comments:

```rust
/// Execute a Spotify control action
///
/// # Supported Actions
/// - `play_pause`: Toggle playback
/// - `next_track`: Skip to next track
/// - `previous_track`: Go to previous track
///
/// # Returns
/// - `0`: Success
/// - `3`: Unknown action
fn execute_spotify_action(action: &str) -> i32 {
    // ...
}
```

---

## Common Issues & Solutions

### Issue 1: "unknown import: `env::memcmp`"

**Cause:** serde_json requires memory intrinsics

**Solution:** Add manual intrinsic implementations (see [Working with JSON](#working-with-json-in-nostd))

### Issue 2: Plugin crashes immediately

**Possible causes:**
1. Missing panic handler
2. Missing global allocator
3. Invalid metadata JSON

**Solution:**
```rust
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Issue 3: "Failed to instantiate: out of fuel"

**Cause:** Plugin execution exceeded 100M instructions

**Solution:**
- Optimize your code
- Remove infinite loops
- Reduce computational complexity

### Issue 4: Large binary size

**Cause:** Debug symbols, unoptimized code, or large dependencies

**Solution:**
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Remove debug symbols
```

### Issue 5: Memory allocation fails

**Cause:** Exceeded 128MB memory limit

**Solution:**
- Use streaming instead of loading everything into memory
- Release memory as soon as possible
- Use smaller data structures

---

## Performance Optimization

### Binary Size

**Target sizes:**
- Minimal plugin (no deps): 300 bytes - 1KB
- With JSON support: 50-100KB
- Full-featured: 100-200KB

**Optimization tips:**

1. **Use `wee_alloc`:**
   ```toml
   wee_alloc = "0.4"
   ```
   Saves ~9KB vs default allocator

2. **Optimize profile:**
   ```toml
   [profile.release]
   opt-level = "z"
   lto = true
   codegen-units = 1
   strip = true
   ```

3. **Minimize dependencies:**
   - Avoid large crates
   - Use `default-features = false`
   - Consider vendoring small utilities

4. **Use `wasm-opt`:**
   ```bash
   wasm-opt -Oz input.wasm -o output.wasm
   ```

### Runtime Performance

**Execution time budget:**
- Target: <1ms per action
- Maximum: 5s (configurable timeout)

**Tips:**

1. **Avoid allocations in hot paths:**
   ```rust
   // Bad: Allocates on every call
   fn process(data: &str) -> String {
       data.to_uppercase()
   }

   // Good: No allocation if possible
   fn process(data: &[u8], output: &mut [u8]) {
       // Process in-place
   }
   ```

2. **Cache computed values:**
   ```rust
   static mut CACHED_VALUE: Option<u32> = None;

   fn get_value() -> u32 {
       unsafe {
           if let Some(v) = CACHED_VALUE {
               return v;
           }
           let v = expensive_computation();
           CACHED_VALUE = Some(v);
           v
       }
   }
   ```

3. **Use static data:**
   ```rust
   static LOOKUP_TABLE: &[u8] = &[/* ... */];
   ```

---

## Example Plugins

### 1. Minimal Plugin (300 bytes)

See [Creating Your First Plugin](#creating-your-first-plugin)

**Use case:** Template, learning, testing

### 2. Spotify Control (54KB)

**Location:** `plugins/wasm-spotify/`

**Features:**
- JSON serialization
- 5 Spotify control actions
- Manual intrinsics

**Use case:** Media control, AppleScript integration

### 3. Template Plugin (108KB)

**Location:** `plugins/wasm-template/`

**Features:**
- Full metadata support
- Action routing
- Error handling examples

**Use case:** Starting point for custom plugins

---

## Publishing Your Plugin

### Plugin Manifest

Create `plugin.toml`:

```toml
[plugin]
name = "my-plugin"
version = "0.1.0"
description = "What my plugin does"
author = "Your Name <you@example.com>"
license = "MIT"
homepage = "https://github.com/you/my-plugin"
repository = "https://github.com/you/my-plugin"

[binary]
path = "target/wasm32-wasip1/release/my_plugin.wasm"
checksum = "sha256:..."

[capabilities]
required = ["Network"]
optional = ["Filesystem"]
```

### Distribution

1. **GitHub Releases:**
   - Tag your release
   - Upload `.wasm` file
   - Include `plugin.toml`

2. **MIDIMon Plugin Registry** (coming soon):
   - Register your plugin
   - Automatic updates
   - User ratings & reviews

---

## Getting Help

### Resources

- **Documentation:** `docs/v2.5-wasm-*.md`
- **Example Plugins:** `plugins/wasm-*/`
- **Integration Tests:** `midimon-core/tests/*_wasm_test.rs`

### Community

- **GitHub Issues:** https://github.com/amiable/midimon/issues
- **Discussions:** https://github.com/amiable/midimon/discussions

### Debugging

Enable debug logging in the host:

```bash
DEBUG=1 midimon
```

This shows:
- Plugin loading details
- Fuel consumption
- Memory allocations
- Execution traces

---

## Appendix: Complete Example

Here's a complete, working plugin with JSON support:

```rust
// src/lib.rs
#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Plugin metadata
static METADATA: &[u8] = br#"{
  "name": "example_plugin",
  "version": "0.1.0",
  "description": "Example WASM plugin",
  "author": "Your Name",
  "license": "MIT",
  "type": "action",
  "capabilities": []
}"#;

#[derive(Debug, Deserialize)]
pub struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

#[derive(Debug, Deserialize)]
pub struct TriggerContext {
    pub velocity: Option<u8>,
    pub current_mode: Option<usize>,
    pub timestamp: u64,
}

// ============================================================================
// WASM Exports
// ============================================================================

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    let request_json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1,
    };

    let request: ActionRequest = match serde_json::from_str(&request_json) {
        Ok(r) => r,
        Err(_) => return 2,
    };

    match execute_action(&request.action) {
        Ok(_) => 0,
        Err(_) => 3,
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
// Plugin Logic
// ============================================================================

fn execute_action(action: &str) -> Result<(), &'static str> {
    match action {
        "hello" => {
            // Do something
            Ok(())
        }
        _ => Err("Unknown action"),
    }
}

fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

// ============================================================================
// Required Boilerplate
// ============================================================================

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Memory intrinsics (required for serde_json)
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
```

**Build:**
```bash
cargo build --target wasm32-wasip1 --release
```

**Result:** ~54KB plugin with full JSON support!

---

**Happy plugin development!** 🎉
