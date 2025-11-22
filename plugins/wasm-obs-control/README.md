# OBS Control Plugin

Control OBS Studio via WebSocket protocol using your MIDI controller.

## Description

This plugin demonstrates network-based control as an alternative to process execution. It provides a complete API surface for OBS Studio control that will work once v2.6's HTTP/WebSocket wrapper is implemented.

## Current Status (v2.5)

**Note:** This is a reference implementation for v2.6. WASI Preview1 doesn't provide high-level HTTP/WebSocket APIs yet. This demonstrates the interface design and protocol structure.

In v2.6, the runtime will provide a WebSocket wrapper that enables this plugin to work with real OBS Studio instances.

## Supported Actions

| Action | Description | Parameters |
|--------|-------------|------------|
| `start_recording` | Start OBS recording | None |
| `stop_recording` | Stop OBS recording | None |
| `start_streaming` | Start OBS streaming | None |
| `stop_streaming` | Stop OBS streaming | None |
| `switch_scene` | Switch to a different scene | `scene_name` (v2.6+) |
| `toggle_mute` | Toggle microphone mute | `source_name` (v2.6+) |
| `toggle_source` | Show/hide a source | `source_name` (v2.6+) |

## Setup Requirements

1. **Install OBS Studio** (https://obsproject.com/)
2. **Install obs-websocket plugin** (usually included in OBS 28+)
3. **Enable WebSocket server in OBS:**
   - Tools → WebSocket Server Settings → Enable WebSocket server
   - Note the port (default: 4455)

## Building

```bash
cargo build --target wasm32-wasip1 --release
```

The compiled plugin will be at:
`target/wasm32-wasip1/release/midimon_wasm_obs_control.wasm`

## Testing

```bash
# Unit tests (native Rust)
cargo test

# Integration tests (WASM)
cd ../..
cargo test --package midimon-core --test obs_wasm_test --features plugin-wasm
```

**Test Results:** 9/9 passing (100%)

## Plugin Info

- **Name:** obs_control
- **Version:** 0.1.0
- **Author:** Amiable Team
- **License:** MIT
- **Binary Size:** 65 KB
- **Capabilities:** network
- **OBS WebSocket:** v5.x protocol

## Use Cases

- Start/stop recording with MIDI pads
- Switch scenes during live streams
- Quick mute/unmute microphone
- Control OBS while using other applications

## Development

See the [WASM Plugin Development Guide](../../docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md) for detailed information on developing plugins.

For the complete OBS WebSocket v5 protocol, see: https://github.com/obsproject/obs-websocket
