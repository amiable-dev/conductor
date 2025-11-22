# MIDIMon OBS Studio Plugin

Control OBS Studio through the OBS WebSocket protocol using your MIDI controller.

## Features

- **Scene Management**: Switch scenes, toggle between scenes
- **Recording Control**: Start, stop, pause, toggle recording
- **Streaming Control**: Start, stop, toggle streaming
- **Source Management**: Toggle source visibility, set visibility
- **Audio Control**: Mute/unmute sources, set volume levels
- **Replay Buffer**: Toggle and save replay buffer
- **Studio Mode**: Toggle studio mode
- **Hotkeys**: Trigger any OBS hotkey by name
- **Status Queries**: Get current scene, recording, and streaming status

## Setup

### 1. Enable OBS WebSocket

OBS Studio 28+ has WebSocket built-in:

1. Open OBS Studio
2. Go to Tools → WebSocket Server Settings
3. Check "Enable WebSocket server"
4. Note the port (default: 4455)
5. Set a password (recommended for security)
6. Click "Show Connect Info" to verify settings

### 2. Configure Environment Variables

```bash
export OBS_HOST="localhost"        # Or IP address if OBS is on another machine
export OBS_PORT="4455"             # Default OBS WebSocket v5 port
export OBS_PASSWORD="your_password_here"  # Optional but recommended
```

Add to your shell profile (~/.zshrc, ~/.bashrc) to persist.

### 3. Build Plugin

```bash
cd plugins/midimon-obs-plugin
cargo build --release
```

### 4. Install Plugin

```bash
# macOS
cp target/release/libmidimon_obs_plugin.dylib ~/.config/midimon/plugins/

# Linux
cp target/release/libmidimon_obs_plugin.so ~/.config/midimon/plugins/
```

## Usage in Config

### Scene Switching

```toml
[[modes.mappings]]
note = 36  # Pad 1
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Gaming"
} }

[[modes.mappings]]
note = 37  # Pad 2
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Webcam"
} }

[[modes.mappings]]
note = 38  # Pad 3
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Screen Share"
} }
```

### Recording Control

```toml
# Toggle recording (single pad for start/stop)
[[modes.mappings]]
note = 39  # Pad 4
action = { type = "plugin", plugin = "obs", data = { type = "toggle_recording" } }

# Separate pads for start and stop
[[modes.mappings]]
note = 40  # Pad 5
action = { type = "plugin", plugin = "obs", data = { type = "start_recording" } }

[[modes.mappings]]
note = 41  # Pad 6
action = { type = "plugin", plugin = "obs", data = { type = "stop_recording" } }

# Pause/resume recording
[[modes.mappings]]
note = 42  # Pad 7
action = { type = "plugin", plugin = "obs", data = { type = "pause_recording" } }
```

### Streaming Control

```toml
[[modes.mappings]]
note = 43  # Pad 8
action = { type = "plugin", plugin = "obs", data = { type = "toggle_streaming" } }
```

### Source Visibility

```toml
# Toggle specific source in current scene
[[modes.mappings]]
note = 44  # Pad 9
action = { type = "plugin", plugin = "obs", data = {
    type = "toggle_source",
    source_name = "Webcam"
} }

# Toggle source in specific scene
[[modes.mappings]]
note = 45  # Pad 10
action = { type = "plugin", plugin = "obs", data = {
    type = "toggle_source",
    scene_name = "Gaming",
    source_name = "Overlay"
} }

# Set source visibility explicitly
[[modes.mappings]]
note = 46  # Pad 11
action = { type = "plugin", plugin = "obs", data = {
    type = "set_source_visibility",
    source_name = "BRB Screen",
    visible = true
} }
```

### Audio Control

```toml
# Toggle mute for microphone
[[modes.mappings]]
note = 47  # Pad 12
action = { type = "plugin", plugin = "obs", data = {
    type = "toggle_mute",
    source_name = "Microphone"
} }

# Set volume level (0.0 - 1.0)
[[modes.mappings]]
note = 48  # Pad 13
velocity = { range = [81, 127] }  # Hard press
action = { type = "plugin", plugin = "obs", data = {
    type = "set_volume",
    source_name = "Desktop Audio",
    volume = 1.0
} }

[[modes.mappings]]
note = 48  # Pad 13
velocity = { range = [41, 80] }   # Medium press
action = { type = "plugin", plugin = "obs", data = {
    type = "set_volume",
    source_name = "Desktop Audio",
    volume = 0.5
} }
```

### Replay Buffer

```toml
# Save last 30 seconds (requires Replay Buffer to be enabled in OBS)
[[modes.mappings]]
note = 49  # Pad 14
action = { type = "plugin", plugin = "obs", data = { type = "save_replay_buffer" } }

# Toggle replay buffer on/off
[[modes.mappings]]
note = 50  # Pad 15
action = { type = "plugin", plugin = "obs", data = { type = "toggle_replay_buffer" } }
```

### Hotkeys

```toml
# Trigger any OBS hotkey
[[modes.mappings]]
note = 51  # Pad 16
action = { type = "plugin", plugin = "obs", data = {
    type = "trigger_hotkey",
    hotkey_name = "OBSBasic.Screenshot"
} }
```

## Available Actions

| Action | Parameters | Description |
|--------|------------|-------------|
| `switch_scene` | `scene_name: String` | Switch to named scene |
| `toggle_scene` | `scene_name: String` | Toggle to/from named scene |
| `toggle_recording` | None | Start/stop recording |
| `start_recording` | None | Start recording |
| `stop_recording` | None | Stop recording |
| `pause_recording` | None | Pause/resume recording |
| `toggle_streaming` | None | Start/stop streaming |
| `start_streaming` | None | Start streaming |
| `stop_streaming` | None | Stop streaming |
| `toggle_source` | `source_name: String`<br/>`scene_name?: String` | Toggle source visibility |
| `set_source_visibility` | `source_name: String`<br/>`scene_name?: String`<br/>`visible: bool` | Set source visibility |
| `toggle_mute` | `source_name: String` | Toggle audio source mute |
| `set_volume` | `source_name: String`<br/>`volume: 0.0-1.0` | Set audio source volume |
| `trigger_hotkey` | `hotkey_name: String` | Trigger OBS hotkey |
| `toggle_replay_buffer` | None | Toggle replay buffer |
| `save_replay_buffer` | None | Save replay buffer |
| `toggle_studio_mode` | None | Toggle studio mode |
| `get_current_scene` | None | Get current scene name |
| `get_recording_status` | None | Get recording status |
| `get_streaming_status` | None | Get streaming status |

## Finding Scene and Source Names

### Scene Names
1. Open OBS Studio
2. Look at the Scenes panel (bottom-left)
3. Use exact names as shown (case-sensitive)

### Source Names
1. Select a scene in OBS
2. Look at the Sources panel (middle)
3. Use exact names as shown (case-sensitive)

### Hotkey Names
1. OBS → Settings → Hotkeys
2. Set a hotkey for the action you want
3. Use the internal name (e.g., `OBSBasic.Screenshot`)
4. Or check OBS logs when triggering hotkeys manually

## Troubleshooting

### Connection refused

1. Verify OBS Studio is running
2. Check WebSocket server is enabled (Tools → WebSocket Server Settings)
3. Verify host and port settings match OBS
4. Check firewall if connecting remotely

### Authentication failed

Ensure `OBS_PASSWORD` environment variable matches the password set in OBS WebSocket settings.

### Scene/Source not found

- Names are case-sensitive
- Check for extra spaces in names
- Verify the scene/source exists in OBS

### Actions not working

1. Check OBS logs (Help → Log Files → View Current Log)
2. Enable debug logging: `RUST_LOG=midimon_obs_plugin=debug midimon`
3. Verify WebSocket protocol version (OBS 28+ uses v5)

## Remote OBS Control

To control OBS on another machine:

```bash
export OBS_HOST="192.168.1.100"  # IP of machine running OBS
export OBS_PORT="4455"
export OBS_PASSWORD="your_password"
```

Security recommendations:
- Always use a password for remote connections
- Use VPN or SSH tunnel for connections over internet
- Limit access with firewall rules

## Advanced Usage

### Conditional Scene Switching

```toml
# Switch to different scenes based on velocity
[[modes.mappings]]
note = 52
velocity = { range = [1, 40] }    # Soft press
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Quiet Scene"
} }

[[modes.mappings]]
note = 52
velocity = { range = [81, 127] }  # Hard press
action = { type = "plugin", plugin = "obs", data = {
    type = "switch_scene",
    scene_name = "Action Scene"
} }
```

### Chained Actions

```toml
# Start recording AND switch to recording scene
[[modes.mappings]]
note = 53
action = { type = "sequence", actions = [
    { type = "plugin", plugin = "obs", data = { type = "start_recording" } },
    { type = "delay", ms = 100 },
    { type = "plugin", plugin = "obs", data = {
        type = "switch_scene",
        scene_name = "Recording"
    } }
] }
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Debugging

Enable debug logging:
```bash
RUST_LOG=midimon_obs_plugin=debug midimon
```

## References

- [OBS WebSocket Protocol](https://github.com/obsproject/obs-websocket/blob/master/docs/generated/protocol.md)
- [obws Rust Client](https://github.com/dnaka91/obws)

## License

MIT License - see LICENSE file for details
