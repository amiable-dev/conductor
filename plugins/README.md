# MIDIMon WASM Plugins

This directory contains WASM plugins for MIDIMon, extending its functionality with sandboxed, secure code execution.

## Available Plugins

### Example Plugins

These plugins serve as templates and examples for plugin development:

| Plugin | Size | Features | Use Case |
|--------|------|----------|----------|
| **wasm-minimal** | 300 bytes | No dependencies, static metadata | Learning, template, minimal overhead |
| **wasm-template** | 108 KB | Full-featured reference | Starting point for complex plugins |
| **wasm-spotify** | 54 KB | JSON support, 5 actions | Real-world media control example |

### Template Selection Guide

Choose the right template for your plugin:

**Use `wasm-minimal` when:**
- Learning WASM plugin development
- Creating simple plugins with hardcoded behavior
- Minimizing binary size is critical
- No dynamic data processing needed

**Use `wasm-spotify` (JSON template) when:**
- Need to parse JSON requests
- Working with dynamic data
- Require serialization/deserialization
- Building data-driven plugins

**Use `wasm-template` when:**
- Building full-featured plugins
- Need comprehensive examples
- Want best practices reference
- Starting a complex project

## Quick Start

### Creating a New Plugin

Use the scaffolding tool to create a new plugin:

```bash
# Minimal plugin (300 bytes)
./scripts/new-plugin.sh -t minimal my-plugin

# JSON-enabled plugin (54KB)
./scripts/new-plugin.sh -t json my-plugin

# Full-featured template (108KB)
./scripts/new-plugin.sh -t template my-plugin

# With custom metadata
./scripts/new-plugin.sh \
  -t json \
  -a "Your Name" \
  -d "Description of your plugin" \
  -l "MIT" \
  my-plugin
```

### Building a Plugin

```bash
cd plugins/wasm-my-plugin

# Quick build (uses included script)
./build.sh

# Manual build
cargo build --target wasm32-wasip1 --release

# Output location
ls target/wasm32-wasip1/release/*.wasm
```

### Testing a Plugin

```bash
# Unit tests (native Rust)
cargo test

# Integration tests (WASM runtime)
cd ../..
cargo test --package midimon-core \
  --test my_plugin_test \
  --features plugin-wasm
```

## Plugin Structure

### Required Files

```
my-plugin/
├── Cargo.toml          # Package manifest
├── src/
│   └── lib.rs          # Plugin implementation
├── build.sh            # Build script (optional)
└── README.md           # Plugin documentation
```

### Required Exports

Every plugin must export these 4 functions:

```rust
#[no_mangle]
pub extern "C" fn init() -> u64;  // Return metadata (ptr, len)

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32;  // Execute action

#[no_mangle]
pub extern "C" fn alloc(size: u32) -> *mut u8;  // Allocate memory

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: u32);  // Free memory
```

## Plugin Examples

### Minimal Plugin (300 bytes)

```rust
#![no_std]

static METADATA: &[u8] = br#"{"name":"my_plugin","version":"0.1.0",...}"#;

#[no_mangle]
pub extern "C" fn init() -> u64 {
    let ptr = METADATA.as_ptr() as u32;
    let len = METADATA.len() as u32;
    ((ptr as u64) << 32) | (len as u64)
}

#[no_mangle]
pub extern "C" fn execute(_ptr: u32, _len: u32) -> i32 {
    // Your logic here
    0  // Success
}

#[no_mangle]
pub extern "C" fn alloc(_size: u32) -> *mut u8 {
    core::ptr::null_mut()  // Minimal: no allocation
}

#[no_mangle]
pub extern "C" fn dealloc(_ptr: *mut u8, _size: u32) {
    // No-op
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
```

### JSON-Enabled Plugin (54KB)

```rust
#![no_std]
extern crate alloc;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ActionRequest {
    action: String,
    context: TriggerContext,
}

#[no_mangle]
pub extern "C" fn execute(ptr: u32, len: u32) -> i32 {
    let json = read_string(ptr, len)?;
    let request: ActionRequest = serde_json::from_str(&json)?;

    match request.action.as_str() {
        "my_action" => { /* ... */ 0 }
        _ => 3  // Unknown action
    }
}

// + memcmp, memcpy, memset intrinsics
```

See `wasm-spotify/src/lib.rs` for complete example.

## Binary Size Targets

| Plugin Type | Target Size | Typical Dependencies |
|-------------|-------------|---------------------|
| Minimal | 300 bytes - 1 KB | None |
| Simple | 1 - 10 KB | Basic logic, no JSON |
| JSON-enabled | 50 - 100 KB | serde + serde_json + wee_alloc |
| Full-featured | 100 - 200 KB | Multiple dependencies |

**Tips for small binaries:**
- Use `opt-level = "z"` in Cargo.toml
- Enable LTO (`lto = true`)
- Use `wee_alloc` instead of default allocator (~9KB savings)
- Minimize dependencies
- Use `strip = true` to remove debug symbols

## Performance Guidelines

### Execution Time
- **Target:** <1ms per action
- **Maximum:** 5s (configurable timeout)
- **Budget:** 100M instructions (fuel limit)

### Memory Usage
- **Default limit:** 128 MB
- **Minimal overhead:** ~1KB (with wee_alloc)
- **Typical usage:** 5-50 MB

### Optimization Tips

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # Smaller panic handler
strip = true         # Strip debug symbols
```

## Security & Capabilities

### Sandboxing

All plugins run in a sandboxed environment:
- ✅ Memory isolation (WASM linear memory)
- ✅ Fuel metering (100M instruction limit)
- ✅ Execution timeout (5s default)
- ✅ No access to host memory

### Capabilities

Plugins must request capabilities:

```json
{
  "capabilities": ["Network", "Filesystem", "Process"]
}
```

| Capability | Description | Use Cases |
|------------|-------------|-----------|
| `Network` | Network access (HTTP, sockets) | API calls, webhooks |
| `Filesystem` | File I/O (preopened dirs) | Config files, logs |
| `Process` | Execute external commands | AppleScript, shell scripts |

Capabilities must be granted by the user before the plugin can use them.

## Common Use Cases

### 1. Media Control
**Example:** Spotify plugin
- Control playback (play/pause, next/previous)
- Adjust volume
- Execute AppleScript commands

### 2. System Utilities
- Take screenshots
- Manipulate clipboard
- Window management

### 3. DAW Integration
- Logic Pro / Ableton Live control
- Transport control
- Track arming
- Parameter automation

### 4. OBS Control
- Scene switching
- Start/stop recording
- Toggle sources

### 5. Home Automation
- Control smart lights (Philips Hue, etc.)
- Adjust thermostat
- Trigger scenes

### 6. Custom LED Effects
- Animated patterns
- Music-reactive effects
- Custom visualizations

## Troubleshooting

### Plugin Won't Load

**Error:** `Failed to load WASM module`

**Possible causes:**
1. Missing required exports (init, execute, alloc, dealloc)
2. Invalid WASM format
3. Target mismatch (must be wasm32-wasip1)

**Solution:**
```bash
# Verify exports
wasm-objdump -x my_plugin.wasm | grep -A 5 "Export"

# Check build target
cargo build --target wasm32-wasip1 --release
```

### Plugin Crashes Immediately

**Error:** `error while executing at wasm backtrace`

**Possible causes:**
1. Missing panic handler
2. Missing global allocator
3. Invalid metadata JSON

**Solution:**
```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[global_allocator]
static ALLOCATOR: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### JSON Parsing Fails

**Error:** `unknown import: env::memcmp`

**Cause:** serde_json requires memory intrinsics

**Solution:** Add manual intrinsic implementations (see wasm-spotify example)

### Out of Fuel

**Error:** `out of fuel`

**Cause:** Plugin exceeded 100M instruction limit

**Solution:**
- Optimize code (remove loops, reduce complexity)
- Use more efficient algorithms
- Cache computed values

## Documentation

- **Development Guide:** `docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md` (1,200+ lines)
- **Quick Reference:** `docs/WASM_PLUGIN_QUICK_REFERENCE.md` (one-page)
- **Complete Summary:** `docs/v2.5-WASM-COMPLETE-SUMMARY.md`

## Resources

### Example Code
- `wasm-minimal/` - Minimal 300-byte plugin
- `wasm-spotify/` - JSON-enabled media control
- `wasm-template/` - Full-featured reference

### Integration Tests
- `../midimon-core/tests/wasm_plugin_integration_test.rs` - Runtime tests
- `../midimon-core/tests/spotify_wasm_test.rs` - Spotify plugin tests

### Tools
- `../scripts/new-plugin.sh` - Plugin scaffolding tool

## Getting Help

### Issues & Questions
- GitHub Issues: https://github.com/amiable/midimon/issues
- Discussions: https://github.com/amiable/midimon/discussions

### Contributing
- See `CONTRIBUTING.md` (coming soon)
- Submit plugins via pull request
- Include tests and documentation

## Plugin Marketplace (Coming Soon)

We're building a plugin marketplace where you can:
- Discover community plugins
- Install with one click
- Manage updates automatically
- Rate and review plugins
- Share your own plugins

Stay tuned for v2.6!

---

**Ready to build your first plugin?**

```bash
./scripts/new-plugin.sh -t minimal my-awesome-plugin
cd plugins/wasm-my-awesome-plugin
# Edit src/lib.rs
./build.sh
```

Happy plugin development! 🚀
