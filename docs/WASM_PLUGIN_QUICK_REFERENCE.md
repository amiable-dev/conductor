# WASM Plugin Quick Reference

**One-page reference for MIDIMon WASM plugin development**

---

## Setup

```bash
# Install WASM target
rustup target add wasm32-wasip1

# Clone template
cp -r plugins/wasm-minimal my-plugin
cd my-plugin

# Build
cargo build --target wasm32-wasip1 --release
```

---

## Cargo.toml Template

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[workspace]

[lib]
crate-type = ["cdylib"]

[dependencies]
# For JSON support:
serde = { version = "1.0", features = ["derive"], default-features = false }
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
wee_alloc = "0.4"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## Required Exports

```rust
#[no_mangle]
pub extern "C" fn init() -> u64 {
    // Return (ptr << 32) | len
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    // Return 0 for success, error code otherwise
}

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    // Allocate size bytes
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    // Free allocated memory
}
```

---

## Metadata Format

```json
{
  "name": "plugin_name",
  "version": "0.1.0",
  "description": "What it does",
  "author": "Your Name",
  "license": "MIT",
  "type": "action",
  "capabilities": []
}
```

---

## Execute Request Format

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

---

## Return Codes

- `0` = Success
- `1` = Invalid memory read
- `2` = Invalid JSON
- `3` = Action execution failed
- `4+` = Custom errors

---

## Memory Intrinsics (Required for JSON)

```rust
#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b { return a as i32 - b as i32; }
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

---

## Minimal Plugin Template

```rust
#![no_std]

static METADATA: &[u8] = br#"{"name":"my_plugin","version":"0.1.0","description":"My plugin","author":"Me","license":"MIT","type":"action","capabilities":[]}"#;

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    0 // Success
}

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8 {
    use core::alloc::{alloc, Layout};
    unsafe {
        alloc(Layout::from_size_align_unchecked(size as usize, 1))
    }
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32) {
    use core::alloc::{dealloc, Layout};
    unsafe {
        dealloc(ptr, Layout::from_size_align_unchecked(size as usize, 1));
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[global_allocator]
static ALLOCATOR: core::alloc::System = core::alloc::System;
```

**Size: ~300 bytes**

---

## JSON Plugin Template

```rust
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

static METADATA: &[u8] = br#"{"name":"my_plugin","version":"0.1.0","description":"My plugin","author":"Me","license":"MIT","type":"action","capabilities":[]}"#;

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
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    let json = match read_string(ptr as usize, len as usize) {
        Ok(s) => s,
        Err(_) => return 1,
    };

    let request: ActionRequest = match serde_json::from_str(&json) {
        Ok(r) => r,
        Err(_) => return 2,
    };

    match request.action.as_str() {
        "my_action" => 0,
        _ => 3,
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
    unsafe { let _ = Vec::from_raw_parts(ptr, 0, size as usize); }
}

fn read_string(ptr: usize, len: usize) -> Result<String, alloc::string::FromUtf8Error> {
    let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    String::from_utf8(slice.to_vec())
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Add memcmp, memcpy, memset from above
```

**Size: ~54KB**

---

## Testing

```rust
#[tokio::test]
async fn test_plugin() {
    let path = PathBuf::from("target/wasm32-wasip1/release/my_plugin.wasm");
    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&path, config).await.unwrap();

    let metadata = plugin.init().await.unwrap();
    assert_eq!(metadata.name, "my_plugin");

    let context = TriggerContext::with_velocity(100);
    let result = plugin.execute("my_action", &context).await;
    assert!(result.is_ok());
}
```

---

## Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `unknown import: env::memcmp` | Missing intrinsics | Add memcmp/memcpy/memset |
| `out of fuel` | Too many instructions | Optimize code, remove loops |
| Immediate crash | Missing panic handler | Add `#[panic_handler]` |
| Parse error | Invalid JSON metadata | Validate JSON syntax |

---

## Size Targets

- **Minimal (no deps):** 300 bytes - 1KB
- **With JSON:** 50-100KB
- **Full-featured:** 100-200KB

---

## Performance Targets

- **Execution time:** <1ms typical
- **Timeout:** 5s maximum
- **Memory limit:** 128MB default
- **Fuel limit:** 100M instructions

---

## Build Commands

```bash
# Build
cargo build --target wasm32-wasip1 --release

# Test (native)
cargo test

# Test (WASM)
cargo test --package midimon-core --test my_plugin_test --features plugin-wasm

# Check size
ls -lh target/wasm32-wasip1/release/*.wasm

# Optimize (optional)
wasm-opt -Oz input.wasm -o output.wasm
```

---

## Capabilities

```rust
"capabilities": ["Network", "Filesystem", "Process"]
```

- **Network**: HTTP requests, sockets
- **Filesystem**: File I/O (preopened dirs)
- **Process**: Execute external commands

---

## Example Plugins

- **Minimal** (`plugins/wasm-minimal/`): 300 bytes, no deps
- **Template** (`plugins/wasm-template/`): 108KB, full features
- **Spotify** (`plugins/wasm-spotify/`): 54KB, JSON + actions

---

## Resources

- **Full Guide:** `docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md`
- **Examples:** `plugins/wasm-*/`
- **Tests:** `midimon-core/tests/*_wasm_test.rs`

---

**Quick Start:**
```bash
cp -r plugins/wasm-minimal my-plugin
cd my-plugin
# Edit src/lib.rs
cargo build --target wasm32-wasip1 --release
```
