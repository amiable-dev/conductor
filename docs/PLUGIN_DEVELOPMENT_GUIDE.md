# MIDIMon Plugin Development Guide

**Version**: 2.3.0
**Last Updated**: 2025-01-18

This guide covers everything you need to know to create custom plugins for MIDIMon v2.3+.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Plugin Architecture](#plugin-architecture)
3. [Getting Started](#getting-started)
4. [Plugin Lifecycle](#plugin-lifecycle)
5. [Capabilities & Security](#capabilities--security)
6. [Parameter Handling](#parameter-handling)
7. [Testing Plugins](#testing-plugins)
8. [Distribution](#distribution)
9. [Best Practices](#best-practices)
10. [Troubleshooting](#troubleshooting)

---

## Introduction

MIDIMon's plugin system allows third-party developers to extend functionality without modifying the core codebase. Plugins are dynamically loaded shared libraries (.so/.dylib/.dll) that implement the `ActionPlugin` trait.

### Why Plugins?

- **Extensibility**: Add custom actions without forking MIDIMon
- **Security**: Capability-based permission system
- **Isolation**: Plugins run in the same process but with controlled permissions
- **Distribution**: Easy to share and install

### Plugin Types (v2.3)

- **Action Plugins**: Execute custom actions when MIDI events occur (available now)
- **Trigger Plugins**: Generate custom events (future feature)

---

## Plugin Architecture

### Core Components

```
┌─────────────────────────────────────────────────┐
│  MIDIMon Daemon                                 │
│  ┌───────────────────────────────────────────┐ │
│  │  PluginManager                            │ │
│  │  - Discovery                              │ │
│  │  - Loading                                │ │
│  │  - Lifecycle Management                   │ │
│  │  - Permission Checks                      │ │
│  └───────────┬───────────────────────────────┘ │
│              │                                   │
│              ▼                                   │
│  ┌───────────────────────────────────────────┐ │
│  │  Your Plugin (.dylib/.so/.dll)            │ │
│  │  implements ActionPlugin                  │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

### The ActionPlugin Trait

```rust
pub trait ActionPlugin: Send + Sync {
    /// Plugin identifier (must match manifest)
    fn name(&self) -> &str;

    /// Semantic version (e.g., "1.0.0")
    fn version(&self) -> &str;

    /// Human-readable description
    fn description(&self) -> &str;

    /// Execute the plugin action
    fn execute(&mut self, params: Value, context: TriggerContext)
        -> Result<(), Box<dyn Error>>;

    /// Required capabilities (optional, defaults to empty)
    fn capabilities(&self) -> Vec<Capability> { vec![] }

    /// Called once when plugin loads (optional)
    fn initialize(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }

    /// Called when plugin unloads (optional)
    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }
}
```

---

## Getting Started

### Step 1: Create a New Crate

```bash
cargo new --lib my_plugin
cd my_plugin
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "midimon-my-plugin"
version = "1.0.0"
edition = "2021"

# Standalone workspace (not part of main MIDIMon workspace)
[workspace]

[lib]
name = "midimon_my_plugin"
crate-type = ["cdylib"]  # Important: shared library

[dependencies]
midimon-core = { path = "../path/to/midimon/midimon-core" }
serde_json = "1.0"
# Add your dependencies here
```

### Step 3: Implement the Plugin

```rust
use midimon_core::plugin::{ActionPlugin, Capability, TriggerContext};
use serde_json::Value;
use std::error::Error;

pub struct MyPlugin {
    // Plugin state (if needed)
    call_count: usize,
}

impl ActionPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"  // Must match plugin.toml
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "My awesome plugin that does something cool"
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Network]  // Declare what you need
    }

    fn execute(&mut self, params: Value, context: TriggerContext)
        -> Result<(), Box<dyn Error>>
    {
        // Extract parameters
        let message = params["message"]
            .as_str()
            .unwrap_or("default message");

        // Use velocity from context if needed
        if let Some(vel) = context.velocity {
            println!("Velocity: {}", vel);
        }

        // Do your thing
        self.call_count += 1;
        println!("{} (call #{})", message, self.call_count);

        Ok(())
    }

    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        println!("[my_plugin] Initialized");
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        println!("[my_plugin] Shutdown (called {} times)", self.call_count);
        Ok(())
    }
}

/// Export the plugin creation function
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn ActionPlugin {
    let plugin = MyPlugin { call_count: 0 };
    Box::into_raw(Box::new(plugin))
}
```

### Step 4: Create plugin.toml Manifest

```toml
[plugin]
name = "my_plugin"
version = "1.0.0"
description = "My awesome plugin that does something cool"
author = "Your Name"
homepage = "https://github.com/yourusername/my-plugin"
license = "MIT"
type = "action"
binary = "libmidimon_my_plugin.dylib"  # .so on Linux, .dll on Windows

[plugin.capabilities]
network = true       # Set to true for capabilities you need
filesystem = false
audio = false
midi = false
subprocess = false
system_control = false
```

### Step 5: Build

```bash
cargo build --release
```

### Step 6: Install

```bash
# Create plugin directory
mkdir -p ~/.midimon/plugins/my_plugin

# Copy binary and manifest
cp target/release/libmidimon_my_plugin.dylib ~/.midimon/plugins/my_plugin/
cp plugin.toml ~/.midimon/plugins/my_plugin/
```

### Step 7: Use in Config

```toml
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { Plugin = {
    plugin = "my_plugin",
    params = {
        message = "Hello from MIDIMon!"
    }
}}
```

---

## Plugin Lifecycle

### 1. Discovery Phase

When MIDIMon starts or reloads:
1. Scans `~/.midimon/plugins/` for subdirectories
2. Reads `plugin.toml` manifests
3. Builds registry of available plugins
4. **Does not load binaries yet** (lazy loading)

### 2. Loading Phase

When a plugin is first referenced in config:
1. Verifies checksum (if present in metadata)
2. Loads .dylib/.so/.dll using `dlopen`/`LoadLibrary`
3. Resolves `_create_plugin` symbol
4. Calls creation function to instantiate plugin
5. Calls `initialize()` method
6. Grants safe capabilities (Network, Audio, MIDI auto-granted)

### 3. Execution Phase

When MIDI event triggers plugin action:
1. Checks if plugin is enabled
2. Verifies all required capabilities are granted
3. Calls `execute(params, context)`
4. Tracks execution statistics (count, failures, latency)

### 4. Unloading Phase

When plugin is unloaded or MIDIMon shuts down:
1. Calls `shutdown()` method
2. Drops plugin instance
3. Unloads shared library

---

## Capabilities & Security

### Capability Types

| Capability | Risk | Auto-Grant | Description |
|------------|------|------------|-------------|
| **Network** | Low | ✅ Yes | Make HTTP/HTTPS requests |
| **Audio** | Low | ✅ Yes | Access audio devices |
| **Midi** | Low | ✅ Yes | Send/receive MIDI |
| **Filesystem** | High | ❌ No | Read/write files |
| **Subprocess** | High | ❌ No | Execute shell commands |
| **SystemControl** | Medium | ❌ No | Control system (volume, brightness) |

### Security Model

1. **Declare Capabilities**: List all capabilities in `capabilities()` method
2. **User Approval**: High/Medium risk capabilities require explicit user grant
3. **Runtime Checks**: Every `execute()` call checks granted permissions
4. **Revocation**: Users can revoke capabilities at any time

### Example: File Plugin (Requires Approval)

```rust
impl ActionPlugin for FilePlugin {
    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Filesystem]  // High risk
    }

    fn execute(&mut self, params: Value, _context: TriggerContext)
        -> Result<(), Box<dyn Error>>
    {
        // User must approve Filesystem capability in UI
        let path = params["path"].as_str().unwrap();
        std::fs::write(path, "data")?;
        Ok(())
    }
}
```

### Checksum Verification (Optional)

Add SHA256 checksum to metadata for security:

```toml
[plugin]
checksum = "a1b2c3d4..."  # SHA256 of binary
```

Generate checksum:
```bash
shasum -a 256 target/release/libmidimon_my_plugin.dylib
```

---

## Parameter Handling

### JSON Parameters

Plugins receive parameters as `serde_json::Value`:

```rust
fn execute(&mut self, params: Value, _context: TriggerContext)
    -> Result<(), Box<dyn Error>>
{
    // Extract string
    let url = params["url"]
        .as_str()
        .ok_or("Missing 'url' parameter")?;

    // Extract number with default
    let timeout = params["timeout"]
        .as_u64()
        .unwrap_or(30);

    // Extract object
    let headers = params.get("headers")
        .and_then(|h| h.as_object());

    // Extract array
    let items: Vec<String> = params["items"]
        .as_array()
        .ok_or("Missing 'items'")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(())
}
```

### TriggerContext

The context provides event metadata:

```rust
pub struct TriggerContext {
    /// MIDI velocity (0-127) if applicable
    pub velocity: Option<u8>,

    /// Current mode index
    pub current_mode: Option<usize>,

    /// Event timestamp
    pub timestamp: Instant,
}
```

**Usage**:

```rust
fn execute(&mut self, params: Value, context: TriggerContext)
    -> Result<(), Box<dyn Error>>
{
    // Velocity-sensitive behavior
    if let Some(vel) = context.velocity {
        if vel > 100 {
            println!("Hard press!");
        } else {
            println!("Soft press");
        }
    }

    Ok(())
}
```

---

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_plugin_metadata() {
        let plugin = MyPlugin { call_count: 0 };
        assert_eq!(plugin.name(), "my_plugin");
        assert_eq!(plugin.version(), "1.0.0");
    }

    #[test]
    fn test_execute() {
        let mut plugin = MyPlugin { call_count: 0 };
        let params = serde_json::json!({ "message": "test" });
        let context = TriggerContext {
            velocity: Some(100),
            current_mode: None,
            timestamp: Instant::now(),
        };

        let result = plugin.execute(params, context);
        assert!(result.is_ok());
        assert_eq!(plugin.call_count, 1);
    }

    #[test]
    fn test_capabilities() {
        let plugin = MyPlugin { call_count: 0 };
        let caps = plugin.capabilities();
        assert!(caps.contains(&Capability::Network));
    }
}
```

### Integration Testing

Test the compiled plugin with MIDIMon:

```bash
# 1. Build plugin
cargo build --release

# 2. Install to test directory
mkdir -p ~/.midimon/plugins/my_plugin
cp target/release/libmidimon_my_plugin.dylib ~/.midimon/plugins/my_plugin/
cp plugin.toml ~/.midimon/plugins/my_plugin/

# 3. Test discovery
midimonctl reload

# 4. Trigger via MIDI or config
```

---

## Distribution

### Packaging

Create a distributable package:

```
my_plugin_v1.0.0/
├── libmidimon_my_plugin.dylib  (macOS)
├── libmidimon_my_plugin.so     (Linux)
├── midimon_my_plugin.dll       (Windows)
├── plugin.toml
└── README.md
```

### GitHub Release

```bash
# Tag release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# Create release with binaries attached
gh release create v1.0.0 \
    --title "My Plugin v1.0.0" \
    --notes "Release notes here" \
    libmidimon_my_plugin.dylib \
    libmidimon_my_plugin.so \
    midimon_my_plugin.dll \
    plugin.toml \
    README.md
```

### Installation Script

```bash
#!/bin/bash
# install.sh

PLUGIN_NAME="my_plugin"
PLUGIN_DIR="$HOME/.midimon/plugins/$PLUGIN_NAME"

echo "Installing $PLUGIN_NAME..."

# Create directory
mkdir -p "$PLUGIN_DIR"

# Detect platform
case "$(uname -s)" in
    Darwin) EXT="dylib" ;;
    Linux)  EXT="so" ;;
    *)      echo "Unsupported platform"; exit 1 ;;
esac

# Copy files
cp "libmidimon_${PLUGIN_NAME}.${EXT}" "$PLUGIN_DIR/"
cp plugin.toml "$PLUGIN_DIR/"

echo "Installed to $PLUGIN_DIR"
echo "Run 'midimonctl reload' to activate"
```

---

## Best Practices

### 1. Error Handling

Always provide helpful error messages:

```rust
fn execute(&mut self, params: Value, _context: TriggerContext)
    -> Result<(), Box<dyn Error>>
{
    let url = params["url"]
        .as_str()
        .ok_or("Missing required parameter 'url'. Example: {\"url\": \"https://...\"}")?;

    // Do work...

    Ok(())
}
```

### 2. Logging

Use stderr for debug output (goes to daemon logs):

```rust
eprintln!("[my_plugin] Processing request to {}", url);
```

### 3. Thread Safety

Plugins must be `Send + Sync`. Use:
- `Arc<Mutex<T>>` for shared mutable state
- `Arc<RwLock<T>>` for read-heavy workloads

```rust
use std::sync::{Arc, Mutex};

pub struct MyPlugin {
    state: Arc<Mutex<PluginState>>,
}
```

### 4. Resource Cleanup

Always clean up in `shutdown()`:

```rust
fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
    // Close connections
    self.client.close()?;

    // Flush buffers
    self.buffer.flush()?;

    // Save state
    self.save_state()?;

    Ok(())
}
```

### 5. Performance

- Avoid blocking operations in `execute()`
- Cache expensive computations
- Consider using async (requires tokio runtime)

### 6. Documentation

Include in your README:
- **Installation instructions**
- **Configuration examples**
- **Parameter reference**
- **Capability requirements**
- **Error codes**
- **Troubleshooting**

---

## Troubleshooting

### Plugin Not Discovered

**Symptoms**: Plugin doesn't appear in `midimonctl list-plugins`

**Fixes**:
1. Check directory structure:
   ```
   ~/.midimon/plugins/my_plugin/
   ├── libmidimon_my_plugin.dylib
   └── plugin.toml
   ```
2. Verify plugin.toml is valid TOML
3. Run `midimonctl reload`
4. Check daemon logs for errors

### Plugin Fails to Load

**Symptoms**: Error when loading plugin

**Fixes**:
1. Verify `_create_plugin` is exported:
   ```bash
   nm -gU target/release/libmidimon_my_plugin.dylib | grep _create_plugin
   ```
2. Check for missing dependencies:
   ```bash
   otool -L target/release/libmidimon_my_plugin.dylib  # macOS
   ldd target/release/libmidimon_my_plugin.so          # Linux
   ```
3. Ensure plugin was built with same Rust version as MIDIMon

### Permission Denied

**Symptoms**: Error about missing capability

**Fixes**:
1. Add capability to `capabilities()` method
2. Grant capability in Plugin Manager UI (for high-risk capabilities)
3. Check `~/.midimon/state.json` for granted permissions

### Checksum Mismatch

**Symptoms**: "Checksum verification failed"

**Fixes**:
1. Recalculate checksum after rebuilding
2. Update plugin.toml with new checksum
3. Remove checksum field if not needed

### Crashes

**Symptoms**: MIDIMon crashes when plugin runs

**Fixes**:
1. Check for null pointer dereferences
2. Ensure all `unsafe` blocks are correct
3. Test with debug builds: `cargo build --debug`
4. Use `valgrind` or `ASAN` for memory errors

---

## Example Plugins

See `examples/` directory for complete working examples:

- **http-plugin**: Make HTTP requests ([README](../../examples/http-plugin/README.md))
- More coming soon!

---

## API Reference

Full API documentation: [docs.rs/midimon-core](https://docs.rs/midimon-core)

Key modules:
- `midimon_core::plugin::ActionPlugin` - Main trait
- `midimon_core::plugin::Capability` - Permission types
- `midimon_core::plugin::TriggerContext` - Event metadata

---

## Support

- **Documentation**: https://code.claude.com/docs
- **Issues**: https://github.com/amiable-dev/midimon/issues
- **Discord**: https://discord.gg/midimon (coming soon)

---

**Happy Plugin Development! 🎹🔌**
