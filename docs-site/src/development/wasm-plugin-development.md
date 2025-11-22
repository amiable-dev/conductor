# WASM Plugin Development

This guide walks you through creating a WASM plugin for Conductor from scratch.

## Prerequisites

### Install Rust WASM Target

```bash
rustup target add wasm32-wasip1
```

### Verify Installation

```bash
rustup target list | grep wasm32-wasip1
# Should show: wasm32-wasip1 (installed)
```

## Creating Your First Plugin

### 1. Project Setup

```bash
# Create new library project
cargo new --lib my-midi-plugin
cd my-midi-plugin
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-midi-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Required for WASM

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Remove debug symbols
```

### 3. Implement Plugin Logic

```rust
// src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PluginParams {
    message: String,
}

#[derive(Serialize)]
struct PluginResult {
    success: bool,
    output: String,
}

/// Initialize plugin (called once on load)
#[no_mangle]
pub extern "C" fn init() {
    eprintln!("Plugin initialized");
}

/// Execute plugin action
#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Parse input parameters
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_ptr, params_len)
    };

    let params: PluginParams = match serde_json::from_slice(params_bytes) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to parse params: {}", e);
            return 1;  // Error code
        }
    };

    // Plugin logic
    eprintln!("Received message: {}", params.message);

    // Return success
    0
}

/// Cleanup (called before unload)
#[no_mangle]
pub extern "C" fn shutdown() {
    eprintln!("Plugin shutting down");
}
```

### 4. Build the Plugin

```bash
cargo build --target wasm32-wasip1 --release
```

Output file: `target/wasm32-wasip1/release/my_midi_plugin.wasm`

### 5. Test the Plugin

Create `test_config.toml`:

```toml
[[modes]]
name = "Default"

[[modes.mappings]]
trigger = { Note = { note = 60 } }  # Middle C
action = { WasmPlugin = {
    path = "target/wasm32-wasip1/release/my_midi_plugin.wasm",
    params = {
        "message": "Hello from MIDI!"
    }
}}
```

## Advanced Examples

### Example 1: HTTP Request Plugin

```rust
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct HttpParams {
    url: String,
    method: String,
}

#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_ptr, params_len)
    };

    let params: HttpParams = serde_json::from_slice(params_bytes)
        .expect("Invalid params");

    // Note: Requires Network capability
    // This is a simplified example - real implementation would use reqwest
    eprintln!("Making {} request to {}", params.method, params.url);

    0
}
```

**Required Capability:** `Network`

### Example 2: File Logger Plugin

```rust
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Deserialize)]
struct LogParams {
    message: String,
    level: String,  // "info", "warn", "error"
}

#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_ptr, params_len)
    };

    let params: LogParams = serde_json::from_slice(params_bytes)
        .expect("Invalid params");

    // Write to sandboxed directory
    // Path: ~/Library/Application Support/conductor/plugin-data/
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/plugin.log")  // Sandboxed path
        .expect("Failed to open log file");

    let timestamp = chrono::Utc::now().to_rfc3339();
    writeln!(file, "[{}] {}: {}",
        timestamp, params.level, params.message)
        .expect("Failed to write log");

    0
}
```

**Required Capability:** `Filesystem`

### Example 3: Velocity-Responsive Plugin

```rust
#[derive(Deserialize)]
struct VelocityParams {
    action: String,
}

#[derive(Deserialize)]
struct TriggerContext {
    velocity: Option<u8>,
    mode: Option<u8>,
}

#[no_mangle]
pub extern "C" fn execute_with_context(
    params_ptr: *const u8,
    params_len: usize,
    context_ptr: *const u8,
    context_len: usize,
) -> i32 {
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_ptr, params_len)
    };
    let context_bytes = unsafe {
        std::slice::from_raw_parts(context_ptr, context_len)
    };

    let params: VelocityParams = serde_json::from_slice(params_bytes)
        .expect("Invalid params");
    let context: TriggerContext = serde_json::from_slice(context_bytes)
        .expect("Invalid context");

    let velocity = context.velocity.unwrap_or(0);

    // Adjust action based on velocity
    match velocity {
        0..=40 => eprintln!("Soft press: {}", params.action),
        41..=80 => eprintln!("Medium press: {}", params.action),
        81..=127 => eprintln!("Hard press: {}", params.action),
    }

    0
}
```

## Capability Declaration

### Declaring Capabilities

Capabilities are declared via a metadata function:

```rust
#[no_mangle]
pub extern "C" fn capabilities() -> *const u8 {
    let caps = vec!["Network", "Filesystem"];
    let json = serde_json::to_string(&caps).unwrap();
    let boxed = Box::new(json.into_bytes());
    Box::into_raw(boxed) as *const u8
}
```

### Available Capabilities

```rust
// Low risk - auto-granted
"Network"      // HTTP requests, WebSocket
"Audio"        // Audio device access
"Midi"         // MIDI device access

// Medium risk - user approval
"Filesystem"   // File read/write (sandboxed)

// High risk - explicit approval
"Subprocess"   // Shell command execution
"SystemControl" // System-level operations
```

## Resource Limits

### Fuel (CPU) Limits

Plugins are limited by "fuel" (instruction count):

```rust
// Conductor automatically limits plugins to 100M instructions
// This prevents infinite loops and excessive CPU usage

// Check remaining fuel (from plugin side):
#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Your plugin code...

    // Conductor will automatically terminate if fuel runs out
    0
}
```

**Default:** 100,000,000 instructions (~100ms execution time)

### Memory Limits

**Default:** 128 MB per plugin

```rust
// Conductor enforces memory limits automatically
// Allocations beyond the limit will fail

#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Be mindful of memory allocations
    let large_buffer = vec![0u8; 1024 * 1024];  // 1 MB - OK
    // let huge_buffer = vec![0u8; 200 * 1024 * 1024];  // 200 MB - Would fail

    0
}
```

### Filesystem Sandbox

Plugins with `Filesystem` capability can only access:

**macOS:** `~/Library/Application Support/conductor/plugin-data/`
**Linux:** `~/.local/share/conductor/plugin-data/`
**Windows:** `%APPDATA%\conductor\plugin-data\`

```rust
// These paths are relative to the sandbox root:
let ok_path = "/my-data.json";          // OK - sandboxed
let ok_path2 = "/subdir/file.txt";      // OK - sandboxed
// let bad_path = "/etc/passwd";         // BLOCKED - outside sandbox
// let bad_path2 = "../../../etc/passwd"; // BLOCKED - path traversal
```

## Optimization

### Size Optimization

```toml
# Cargo.toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true
codegen-units = 1
strip = true
panic = "abort"     # Smaller panic handling
```

### Post-Build Optimization

```bash
# Install wasm-opt
cargo install wasm-opt

# Optimize WASM binary
wasm-opt -Oz \
  target/wasm32-wasip1/release/my_plugin.wasm \
  -o target/wasm32-wasip1/release/my_plugin_opt.wasm

# Check size reduction
ls -lh target/wasm32-wasip1/release/*.wasm
```

### Performance Tips

1. **Minimize allocations in hot paths**
   ```rust
   // Bad: allocates on every call
   fn process(data: &str) -> String {
       format!("Processed: {}", data)
   }

   // Good: reuse buffer
   fn process(data: &str, buffer: &mut String) {
       buffer.clear();
       buffer.push_str("Processed: ");
       buffer.push_str(data);
   }
   ```

2. **Use static data when possible**
   ```rust
   // Bad: allocates vec on every call
   fn get_options() -> Vec<String> {
       vec!["option1".to_string(), "option2".to_string()]
   }

   // Good: static slice
   const OPTIONS: &[&str] = &["option1", "option2"];
   ```

3. **Lazy initialization**
   ```rust
   use std::sync::OnceLock;

   static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

   fn get_client() -> &'static reqwest::Client {
       HTTP_CLIENT.get_or_init(|| reqwest::Client::new())
   }
   ```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_params() {
        let params = r#"{"message": "test"}"#;
        let parsed: PluginParams = serde_json::from_str(params).unwrap();
        assert_eq!(parsed.message, "test");
    }
}
```

### Integration Testing

```bash
# Build plugin
cargo build --target wasm32-wasip1 --release

# Test with wasmtime
wasmtime target/wasm32-wasip1/release/my_plugin.wasm

# Or use Conductor directly
conductor --config test_config.toml 0
```

## Debugging

### Enable Debug Logging

```rust
#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Use eprintln! for debug output
    eprintln!("DEBUG: Received {} bytes", params_len);
    eprintln!("DEBUG: Params: {:?}", params);

    // This appears in Conductor's stderr
    0
}
```

### Run with Debug Output

```bash
# See plugin debug output
DEBUG=1 conductor --config test_config.toml 0 2>&1 | grep "DEBUG:"
```

### Common Issues

**Plugin not loading:**
```bash
# Verify WASM format
file target/wasm32-wasip1/release/my_plugin.wasm
# Should show: WebAssembly (wasm) binary module

# Check for WASI imports
wasm-objdump -x target/wasm32-wasip1/release/my_plugin.wasm | grep wasi
```

**Out of fuel error:**
- Reduce computation in execute()
- Move heavy work to init()
- Use lazy initialization

**Memory limit exceeded:**
- Reduce buffer sizes
- Use streaming instead of loading entire data
- Profile with `cargo-bloat`

## Signing Your Plugin

**See:** [Plugin Security Guide](plugin-security.md) for complete signing instructions.

**Quick reference:**

```bash
# Generate keypair (one-time)
conductor-sign generate-key ~/.conductor/my-key

# Sign plugin
conductor-sign sign \
  target/wasm32-wasip1/release/my_plugin.wasm \
  ~/.conductor/my-key \
  --name "Your Name" \
  --email "you@example.com"

# Creates: my_plugin.wasm.sig
```

## Distribution

### Package Structure

```
my-midi-plugin/
├── my_plugin.wasm           # Binary
├── my_plugin.wasm.sig       # Signature
├── README.md                # Documentation
├── LICENSE                  # License
└── examples/
    └── config.toml          # Example configuration
```

### README Template

```markdown
# My MIDI Plugin

Brief description of what your plugin does.

## Installation

1. Download `my_plugin.wasm` and `my_plugin.wasm.sig`
2. Copy to `~/.conductor/wasm-plugins/`
3. Add to configuration (see example)

## Configuration

\```toml
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/my_plugin.wasm",
    params = {
        "param1": "value1"
    }
}}
\```

## Parameters

- `param1`: Description
- `param2`: Description

## Capabilities

- Network: For HTTP requests
- Filesystem: For data persistence

## License

MIT
```

## Next Steps

- [Plugin Security](plugin-security.md) - Signing and verification
- [Plugin Examples](plugin-examples.md) - Real-world examples
- [WASM Plugins Overview](wasm-plugins.md) - Architecture and concepts

## Resources

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [WASI Documentation](https://wasi.dev/)
- [wasmtime Guide](https://docs.wasmtime.dev/)
