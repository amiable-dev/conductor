# WASM Plugins

**Since:** v2.5
**Status:** Production-ready

Conductor's WASM (WebAssembly) plugin system provides a secure, sandboxed environment for running third-party plugins with enterprise-grade safety guarantees.

## Overview

WASM plugins offer several advantages over native plugins:

- **Security**: Sandboxed execution with no direct system access
- **Portability**: Write once, run anywhere (same binary on macOS/Linux/Windows)
- **Safety**: Memory-safe execution, no undefined behavior
- **Isolation**: Resource limits prevent runaway plugins
- **Verification**: Cryptographic signatures ensure plugin integrity

## Architecture

```
┌─────────────────────────────────────────────────────┐
│  Conductor Core                                       │
│  ┌───────────────────────────────────────────────┐ │
│  │  WASM Runtime (wasmtime)                      │ │
│  │  ┌─────────────────────────────────────────┐ │ │
│  │  │  Plugin Instance                        │ │ │
│  │  │  - Fuel metering (CPU limits)           │ │ │
│  │  │  - Memory limits (128 MB default)       │ │ │
│  │  │  - WASI filesystem sandboxing           │ │ │
│  │  │  - Capability system (network, etc.)    │ │ │
│  │  └─────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────┘ │
│                                                     │
│  ┌───────────────────────────────────────────────┐ │
│  │  Signature Verification (v2.7)                │ │
│  │  - Ed25519 digital signatures                 │ │
│  │  - SHA-256 integrity checking                 │ │
│  │  - Trust management                           │ │
│  └───────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

## Quick Comparison

| Feature | Native Plugins (v2.3) | WASM Plugins (v2.5+) |
|---------|----------------------|---------------------|
| **Platform** | Platform-specific (.dylib/.so/.dll) | Universal (.wasm) |
| **Security** | Full system access | Sandboxed |
| **Memory Safety** | Depends on language | Guaranteed |
| **Resource Limits** | None | CPU, memory, I/O |
| **Installation** | Manual copy | Single file |
| **Verification** | SHA256 checksum | Cryptographic signatures |
| **Languages** | Rust, C, C++ | Rust, C, C++, Go, Swift, Zig |
| **Startup Time** | Fast (~1ms) | Fast (~10ms) |
| **Runtime Overhead** | None | Minimal (~5%) |

## Plugin Lifecycle

1. **Load** - WASM module loaded and validated
2. **Verify** (v2.7) - Cryptographic signature checked
3. **Initialize** - Plugin setup, capabilities granted
4. **Execute** - Plugin called for MIDI events
5. **Shutdown** - Cleanup and resource release
6. **Unload** - Module removed from memory

## Security Features

### Resource Limiting (v2.7)

**Fuel Metering:**
- CPU execution limited to prevent infinite loops
- Default: 100 million instructions (~100ms)
- Configurable per-plugin

**Memory Limits:**
- Default: 128 MB
- Prevents memory exhaustion
- Enforced by WASM runtime

**Table Growth Limits:**
- Prevents unbounded table allocation
- Maximum elements configurable

### Filesystem Sandboxing (v2.7)

**Directory Preopening:**
- WASI filesystem isolated to specific directories
- Default: `~/.local/share/conductor/plugin-data/` (Linux)
- Default: `~/Library/Application Support/conductor/plugin-data/` (macOS)
- Plugins cannot access files outside sandbox

### Cryptographic Signatures (v2.7)

**Ed25519 Digital Signatures:**
- Industry-standard cryptography
- 256-bit security level
- Signature file: `<plugin>.wasm.sig`

**Three-Tier Trust Model:**
1. **Unsigned** - Development only (optional signatures)
2. **Self-Signed** - Valid signature from any key
3. **Trusted Keys** - Signature must match trusted key list

## Capability System

Plugins request capabilities to access system resources:

| Capability | Risk | Description |
|-----------|------|-------------|
| `Network` | 🟢 Low | HTTP requests, WebSocket |
| `Filesystem` | 🟡 Medium | Read/write files (sandboxed) |
| `Subprocess` | 🔴 High | Execute shell commands |
| `SystemControl` | 🔴 High | System-level operations |

**Risk Levels:**
- 🟢 **Low**: Auto-granted
- 🟡 **Medium**: User approval required
- 🔴 **High**: Explicit approval with warning

## Example: Spotify Plugin

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SpotifyParams {
    action: String,  // "play", "pause", "next", "previous"
}

#[no_mangle]
pub extern "C" fn init() {
    // Plugin initialization
}

#[no_mangle]
pub extern "C" fn execute(params_json: *const u8, params_len: usize) -> i32 {
    // Parse parameters
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_json, params_len)
    };
    let params: SpotifyParams = serde_json::from_slice(params_bytes)
        .expect("Invalid params");

    // Control Spotify via Web API
    match params.action.as_str() {
        "play" => spotify_play(),
        "pause" => spotify_pause(),
        "next" => spotify_next(),
        "previous" => spotify_previous(),
        _ => return 1, // Error
    }

    0 // Success
}
```

## Building WASM Plugins

### Prerequisites

```bash
# Add WASM target
rustup target add wasm32-wasip1
```

### Project Setup

```toml
# Cargo.toml
[package]
name = "my-wasm-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Build

```bash
cargo build --target wasm32-wasip1 --release
```

Output: `target/wasm32-wasip1/release/my_wasm_plugin.wasm`

## Plugin Distribution

### Recommended Structure

```
my-plugin/
├── my_plugin.wasm           # WASM binary
├── my_plugin.wasm.sig       # Cryptographic signature (v2.7)
├── README.md                # Documentation
└── LICENSE                  # License file
```

### Signing Your Plugin (v2.7)

```bash
# Generate keypair (one-time)
conductor-sign generate-key ~/.conductor/my-plugin-key

# Sign plugin
conductor-sign sign my_plugin.wasm ~/.conductor/my-plugin-key \
  --name "Your Name" \
  --email "you@example.com"

# Verify signature
conductor-sign verify my_plugin.wasm
```

## Installation

### User Installation

```bash
# Copy plugin to Conductor directory
mkdir -p ~/.conductor/wasm-plugins/
cp my_plugin.wasm ~/.conductor/wasm-plugins/
cp my_plugin.wasm.sig ~/.conductor/wasm-plugins/  # v2.7
```

### Configuration

```toml
# config.toml
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/my_plugin.wasm",
    params = {
        "action": "play"
    }
}}
```

## Official Plugins

Conductor provides several official WASM plugins:

### Spotify Control
**File:** `conductor_wasm_spotify.wasm`
**Capabilities:** Network
**Actions:** play, pause, next, previous, volume, shuffle, repeat

### OBS Studio Control
**File:** `conductor_wasm_obs_control.wasm`
**Capabilities:** Network
**Actions:** scene switching, recording, streaming, mute/unmute

### System Utilities
**File:** `conductor_wasm_system_utils.wasm`
**Capabilities:** SystemControl
**Actions:** lock screen, sleep, notifications, brightness

## Performance

**Typical Execution Times:**
- Plugin load: ~10ms (one-time)
- First execution: ~5ms (JIT compilation)
- Subsequent executions: <1ms
- Memory overhead: ~2-5 MB per plugin

**Optimization Tips:**
- Keep plugins small (<1 MB ideal)
- Minimize allocations in hot paths
- Use `wasm-opt` for size/speed optimization
- Profile with `wasmtime::Store::fuel_consumed()`

## Troubleshooting

### Plugin Fails to Load

**Check WASM target:**
```bash
file my_plugin.wasm
# Should show: WebAssembly (wasm) binary module version 0x1
```

**Verify WASI compatibility:**
```bash
wasm-objdump -x my_plugin.wasm | grep -A5 "Import"
# Should show WASI imports like wasi_snapshot_preview1
```

### Out of Fuel Error

Increase fuel limit in configuration:

```rust
let mut config = WasmConfig::default();
config.max_fuel = 200_000_000;  // 200M instructions
```

### Memory Limit Exceeded

Increase memory limit:

```rust
config.max_memory_bytes = 256 * 1024 * 1024;  // 256 MB
```

### Signature Verification Failed

```bash
# Verify signature manually
conductor-sign verify my_plugin.wasm

# Check if key is trusted
conductor-sign trust list

# Add key to trusted list
conductor-sign trust add <public-key-hex> "Plugin Author"
```

## Next Steps

- [WASM Plugin Development Guide](wasm-plugin-development.md) - Complete development tutorial
- [Plugin Security](plugin-security.md) - Signing and verification
- [Plugin Examples](plugin-examples.md) - Real-world examples
- [Plugin API Reference](../reference/wasm-plugin-api.md) - API documentation

## Version History

- **v2.5** - Initial WASM plugin runtime
- **v2.6** - Example plugins (Spotify, OBS, System Utils)
- **v2.7** - Security hardening:
  - Resource limiting (fuel, memory, tables)
  - Directory preopening (filesystem sandboxing)
  - Plugin signing/verification (Ed25519)

## Further Reading

- [WebAssembly Security](https://webassembly.org/docs/security/)
- [WASI Documentation](https://wasi.dev/)
- [wasmtime Guide](https://docs.wasmtime.dev/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
