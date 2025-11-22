# MIDIMon Plugins - Quick Start Guide

**Version**: v2.4 Plugin Ecosystem
**Date**: 2025-01-18
**Status**: Phase 1 Foundation Complete

## What Are MIDIMon Plugins?

MIDIMon plugins extend the core functionality by allowing you to trigger external services and applications from your MIDI controller. Plugins are dynamically loaded shared libraries that implement the `ActionPlugin` trait.

## Available Plugins

### 1. Spotify Plugin (✅ Ready)

Control Spotify playback through the Spotify Web API.

**Capabilities**: Network, Filesystem
**Actions**: 11 (play/pause, skip, volume, playlists, shuffle, repeat, like track)

**Quick Setup**:
```bash
# 1. Set environment variables
export RSPOTIFY_CLIENT_ID="your_spotify_client_id"
export RSPOTIFY_CLIENT_SECRET="your_spotify_client_secret"
export RSPOTIFY_REDIRECT_URI="http://localhost:8888/callback"

# 2. Authenticate (first time only)
midimon spotify auth

# 3. Install plugin
cp plugins/midimon-spotify-plugin/target/release/libmidimon_spotify_plugin.dylib \
   ~/.config/midimon/plugins/

# 4. Add to config.toml
[[modes.mappings]]
note = 36
action = { type = "plugin", plugin = "spotify", data = { type = "play_pause" } }
```

**Full Documentation**: See [plugins/midimon-spotify-plugin/README.md](midimon-spotify-plugin/README.md)

### 2. OBS Studio Plugin (⏳ In Progress)

Control OBS Studio through WebSocket protocol.

**Capabilities**: Network
**Actions**: 17 (scenes, recording, streaming, sources, audio, hotkeys)

**Quick Setup**:
```bash
# 1. Enable OBS WebSocket (Tools → WebSocket Server Settings)
# 2. Set environment variables
export OBS_HOST="localhost"
export OBS_PORT="4455"
export OBS_PASSWORD="your_obs_password"

# 3. Install plugin
cp plugins/midimon-obs-plugin/target/release/libmidimon_obs_plugin.dylib \
   ~/.config/midimon/plugins/

# 4. Add to config.toml
[[modes.mappings]]
note = 37
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Gaming"
} }
```

**Full Documentation**: See [plugins/midimon-obs-plugin/README.md](midimon-obs-plugin/README.md)

## Using Plugins in Config

### Basic Plugin Action

```toml
[[modes.mappings]]
note = 36  # Pad number
action = {
    type = "plugin",           # Action type
    plugin = "spotify",        # Plugin ID
    data = {                   # Plugin-specific parameters
        type = "play_pause"
    }
}
```

### With Velocity Sensitivity

```toml
# Soft press: 50% volume
[[modes.mappings]]
note = 40
velocity = { range = [1, 40] }
action = {
    type = "plugin",
    plugin = "spotify",
    data = { type = "set_volume", volume = 50 }
}

# Hard press: 100% volume
[[modes.mappings]]
note = 40
velocity = { range = [81, 127] }
action = {
    type = "plugin",
    plugin = "spotify",
    data = { type = "set_volume", volume = 100 }
}
```

### Chained Actions

```toml
# Start recording AND switch scene
[[modes.mappings]]
note = 42
action = {
    type = "sequence",
    actions = [
        { type = "plugin", plugin = "obs", data = { type = "start_recording" } },
        { type = "delay", ms = 100 },
        { type = "plugin", plugin = "obs", data = {
            type = "switch_scene",
            scene_name = "Recording"
        } }
    ]
}
```

## Installing Plugins

### Method 1: Manual Installation

```bash
# 1. Build the plugin
cd plugins/midimon-spotify-plugin
cargo build --release

# 2. Copy to plugins directory
mkdir -p ~/.config/midimon/plugins
cp target/release/libmidimon_spotify_plugin.dylib ~/.config/midimon/plugins/

# 3. Restart midimon daemon
midimonctl restart
```

### Method 2: Plugin Marketplace (Coming in v2.4)

```bash
# Install from registry
midimon plugin install spotify

# List installed plugins
midimon plugin list

# Uninstall plugin
midimon plugin uninstall spotify
```

## Plugin Discovery

MIDIMon automatically discovers plugins in:

1. `~/.config/midimon/plugins/` (user plugins)
2. `/usr/local/share/midimon/plugins/` (system plugins)
3. Path specified by `MIDIMON_PLUGIN_PATH` env var

Plugin files must match: `libmidimon_*_plugin.{dylib,so,dll}`

## Security & Capabilities

Plugins declare required capabilities that you must approve:

| Capability | Risk | Description |
|------------|------|-------------|
| Network | Medium | HTTP/HTTPS requests to external servers |
| Filesystem | High | Read/write files on your computer |
| Audio | Low | Access audio devices |
| Midi | Low | Send/receive MIDI messages |
| Subprocess | High | Execute shell commands |
| SystemControl | Medium | Control system settings |

**First-time use**: MIDIMon will prompt you to approve plugin capabilities.

## Troubleshooting

### Plugin Not Loading

```bash
# Check plugin directory
ls -la ~/.config/midimon/plugins/

# Check midimon logs
tail -f ~/.config/midimon/logs/midimon.log

# Verify plugin format
file ~/.config/midimon/plugins/libmidimon_spotify_plugin.dylib
```

### Plugin Action Not Working

```bash
# Enable debug logging
DEBUG=1 midimon

# Check plugin capabilities are approved
midimon plugin capabilities spotify

# Verify plugin parameters
midimon plugin test spotify '{"type":"play_pause"}'
```

### Authentication Issues (Spotify)

```bash
# Re-authenticate
rm ~/.config/midimon/spotify_token_cache
midimon spotify auth
```

### Connection Issues (OBS)

```bash
# Verify OBS WebSocket is running
# OBS → Tools → WebSocket Server Settings → Show Connect Info

# Test connection
curl -X POST http://localhost:4455
```

## Building Your Own Plugin

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone plugin template
git clone https://github.com/amiable-dev/midimon-plugin-template
cd midimon-plugin-template
```

### Plugin Structure

```rust
use midimon_core::plugin::{ActionPlugin, Capability, TriggerContext};
use serde_json::Value;
use std::error::Error;

pub struct MyPlugin;

impl ActionPlugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"  // Unique ID
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn description(&self) -> &str {
        "My custom plugin"
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Network]  // Required permissions
    }

    fn execute(&mut self, params: Value, context: TriggerContext)
        -> Result<(), Box<dyn Error>>
    {
        // Your plugin logic here
        println!("Action triggered! Params: {:?}", params);
        println!("Velocity: {:?}", context.velocity);
        Ok(())
    }
}

// Export plugin constructor
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn ActionPlugin {
    Box::into_raw(Box::new(MyPlugin))
}
```

### Build & Test

```bash
# Build plugin
cargo build --release

# Test locally
cp target/release/libmy_plugin.dylib ~/.config/midimon/plugins/

# Add to config
cat >> ~/.config/midimon/config.toml << EOF
[[modes.mappings]]
note = 50
action = { type = "plugin", plugin = "my_plugin", data = {} }
EOF

# Test it
midimon
```

## Publishing Plugins

### Submit to Registry

1. **Create GitHub repository** for your plugin
2. **Build release binaries** for all platforms (macOS, Linux, Windows)
3. **Create GitHub release** with binaries
4. **Submit PR** to [midimon-plugins registry](https://github.com/amiable-dev/midimon-plugins)

### Registry Entry

```json
{
  "id": "my_plugin",
  "name": "My Plugin",
  "description": "Description of what it does",
  "author": "Your Name",
  "version": "0.1.0",
  "homepage": "https://github.com/yourname/midimon-my-plugin",
  "repository": "https://github.com/yourname/midimon-my-plugin",
  "license": "MIT",
  "categories": ["productivity"],
  "tags": ["automation", "custom"],
  "capabilities": ["network"],
  "platforms": ["macos", "linux", "windows"],
  "min_midimon_version": "2.4.0",
  "downloads": {
    "macos-aarch64": "https://github.com/.../my_plugin-aarch64-apple-darwin.dylib",
    "linux-x86_64": "https://github.com/.../my_plugin-x86_64-unknown-linux-gnu.so",
    "windows-x86_64": "https://github.com/.../my_plugin-x86_64-pc-windows-msvc.dll"
  },
  "checksums": {
    "macos-aarch64": "sha256:...",
    "linux-x86_64": "sha256:...",
    "windows-x86_64": "sha256:..."
  }
}
```

## Resources

- **Plugin Development Guide**: [docs/development/plugin-development.md](../docs-site/src/development/plugin-development.md)
- **Plugin Template**: https://github.com/amiable-dev/midimon-plugin-template
- **Plugin Registry**: https://github.com/amiable-dev/midimon-plugins
- **API Documentation**: https://docs.rs/midimon-core

## Support

- **Issues**: https://github.com/amiable-dev/midimon/issues
- **Discussions**: https://github.com/amiable-dev/midimon/discussions
- **Discord**: https://discord.gg/midimon (coming soon)

## Examples in the Wild

- **Spotify Plugin**: Full Spotify Web API integration
- **OBS Plugin**: Complete OBS Studio control
- **HTTP Request Plugin**: Generic HTTP client
- **Webhook Plugin**: Trigger webhooks with MIDI

Browse all plugins at: https://plugins.midimon.dev (coming in v2.4)

---

**Last Updated**: 2025-01-18
**Version**: v2.4 Plugin Ecosystem
**Status**: Phase 1 Complete (90%)
