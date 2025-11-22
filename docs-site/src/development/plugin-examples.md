# Plugin Examples

Conductor includes three official WASM plugins that demonstrate real-world integration patterns. All plugins are signed with the official Conductor team key and include full source code.

## Official Plugins

### Spotify Web API Plugin

**File:** `conductor_wasm_spotify.wasm`
**Capabilities:** Network
**Status:** Production-ready

Control Spotify playback directly from your MIDI controller.

#### Features

- Play/Pause control
- Track navigation (next/previous)
- Volume control
- Shuffle toggle
- Repeat mode toggle
- Get current playback state

#### Setup

1. **Get Spotify API Credentials**
   - Visit [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
   - Create an app
   - Note Client ID and Client Secret
   - Add redirect URI: `http://localhost:8888/callback`

2. **Authenticate**
   ```bash
   # First-time setup (opens browser for OAuth)
   spotify-auth --client-id YOUR_ID --client-secret YOUR_SECRET
   ```

3. **Configuration**
   ```toml
   # Play/Pause
   [[modes.mappings]]
   trigger = { Note = { note = 60 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
       params = {
           "action": "play_pause"
       }
   }}

   # Next track
   [[modes.mappings]]
   trigger = { Note = { note = 61 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
       params = {
           "action": "next"
       }
   }}

   # Previous track
   [[modes.mappings]]
   trigger = { Note = { note = 59 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
       params = {
           "action": "previous"
       }
   }}

   # Volume control (velocity-sensitive)
   [[modes.mappings]]
   trigger = { Note = { note = 62 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
       params = {
           "action": "volume",
           "level": "{velocity}"  # 0-127 mapped to 0-100%
       }
   }}
   ```

#### Available Actions

| Action | Parameters | Description |
|--------|-----------|-------------|
| `play` | None | Resume playback |
| `pause` | None | Pause playback |
| `play_pause` | None | Toggle play/pause |
| `next` | None | Skip to next track |
| `previous` | None | Previous track |
| `volume` | `level: 0-127` | Set volume (maps to 0-100%) |
| `shuffle` | None | Toggle shuffle |
| `repeat` | `mode: "off"\|"track"\|"context"` | Set repeat mode |
| `get_state` | None | Get current playback state |

#### Advanced: Velocity-Sensitive Volume

```toml
[[modes.mappings]]
trigger = { VelocityRange = {
    note = 62,
    ranges = [
        { min = 0, max = 40, action_index = 0 },    # Soft: -10%
        { min = 41, max = 80, action_index = 1 },   # Medium: no change
        { min = 81, max = 127, action_index = 2 }   # Hard: +10%
    ]
}}
actions = [
    { WasmPlugin = {
        path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
        params = { "action": "volume", "delta": "-10" }
    }},
    { WasmPlugin = {
        path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
        params = { "action": "get_state" }
    }},
    { WasmPlugin = {
        path = "~/.conductor/wasm-plugins/conductor_wasm_spotify.wasm",
        params = { "action": "volume", "delta": "+10" }
    }}
]
```

---

### OBS Studio Control Plugin

**File:** `conductor_wasm_obs_control.wasm`
**Capabilities:** Network
**Status:** Production-ready

Control OBS Studio streaming/recording via WebSocket.

#### Features

- Scene switching
- Start/Stop streaming
- Start/Stop recording
- Source mute/unmute
- Filter toggle
- Transition control

#### Setup

1. **Enable OBS WebSocket**
   - OBS Studio → Tools → WebSocket Server Settings
   - Enable WebSocket server
   - Set password (optional but recommended)
   - Note port (default: 4455)

2. **Configuration**
   ```toml
   # Scene switching
   [[modes.mappings]]
   trigger = { Note = { note = 48 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_obs_control.wasm",
       params = {
           "action": "set_scene",
           "scene_name": "Gaming",
           "host": "localhost:4455",
           "password": "your_password"  # Optional
       }
   }}

   # Start streaming
   [[modes.mappings]]
   trigger = { Note = { note = 49 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_obs_control.wasm",
       params = {
           "action": "start_streaming",
           "host": "localhost:4455"
       }
   }}

   # Stop streaming
   [[modes.mappings]]
   trigger = { Note = { note = 50 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_obs_control.wasm",
       params = {
           "action": "stop_streaming",
           "host": "localhost:4455"
       }
   }}

   # Toggle mic mute
   [[modes.mappings]]
   trigger = { Note = { note = 51 } }
   action = { WasmPlugin = {
       path = "~/.conductor/wasm-plugins/conductor_wasm_obs_control.wasm",
       params = {
           "action": "toggle_mute",
           "source_name": "Microphone",
           "host": "localhost:4455"
       }
   }}
   ```

#### Available Actions

| Action | Parameters | Description |
|--------|-----------|-------------|
| `set_scene` | `scene_name` | Switch to scene |
| `get_current_scene` | None | Get active scene |
| `start_streaming` | None | Start streaming |
| `stop_streaming` | None | Stop streaming |
| `toggle_streaming` | None | Toggle streaming |
| `start_recording` | None | Start recording |
| `stop_recording` | None | Stop recording |
| `toggle_recording` | None | Toggle recording |
| `toggle_mute` | `source_name` | Mute/unmute source |
| `set_volume` | `source_name`, `volume: 0-1` | Set source volume |
| `toggle_filter` | `source_name`, `filter_name` | Toggle filter |
| `set_transition` | `transition_name`, `duration_ms` | Set scene transition |

#### Advanced: Scene Hotkeys

```toml
# Map pads to scenes
[[modes]]
name = "OBS Control"

[[modes.mappings]]
trigger = { Note = { note = 36 } }  # Pad 1
action = { WasmPlugin = { path = "...", params = { "action": "set_scene", "scene_name": "Intro" }}}

[[modes.mappings]]
trigger = { Note = { note = 37 } }  # Pad 2
action = { WasmPlugin = { path = "...", params = { "action": "set_scene", "scene_name": "Gaming" }}}

[[modes.mappings]]
trigger = { Note = { note = 38 } }  # Pad 3
action = { WasmPlugin = { path = "...", params = { "action": "set_scene", "scene_name": "Chatting" }}}

[[modes.mappings]]
trigger = { Note = { note = 39 } }  # Pad 4
action = { WasmPlugin = { path = "...", params = { "action": "set_scene", "scene_name": "BRB" }}}
```

---

### System Utilities Plugin

**File:** `conductor_wasm_system_utils.wasm`
**Capabilities:** SystemControl
**Status:** Production-ready

System-level operations like screen lock, sleep, notifications.

#### Features

- Lock screen
- Sleep/shutdown
- Brightness control
- System notifications
- Application launcher
- Volume control (system-wide)

#### Configuration

```toml
# Lock screen
[[modes.mappings]]
trigger = { LongPress = { note = 60, duration_ms = 2000 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/conductor_wasm_system_utils.wasm",
    params = {
        "action": "lock_screen"
    }
}}

# Display notification
[[modes.mappings]]
trigger = { Note = { note = 61 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/conductor_wasm_system_utils.wasm",
    params = {
        "action": "notify",
        "title": "Recording Started",
        "message": "Stream is now live!",
        "sound": true
    }
}}

# Brightness control (velocity-sensitive)
[[modes.mappings]]
trigger = { Note = { note = 62 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/conductor_wasm_system_utils.wasm",
    params = {
        "action": "brightness",
        "level": "{velocity}"  # 0-127 mapped to 0-100%
    }
}}

# Launch application
[[modes.mappings]]
trigger = { Note = { note = 63 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/conductor_wasm_system_utils.wasm",
    params = {
        "action": "launch",
        "app": "Spotify"
    }
}}
```

#### Available Actions

| Action | Parameters | Description |
|--------|-----------|-------------|
| `lock_screen` | None | Lock screen (macOS/Linux) |
| `sleep` | None | Put system to sleep |
| `shutdown` | `force: bool` | Shutdown system |
| `notify` | `title`, `message`, `sound: bool` | Show notification |
| `brightness` | `level: 0-127` | Set screen brightness |
| `launch` | `app: string` | Launch application |
| `volume` | `level: 0-127` | Set system volume |
| `volume_up` | None | Increase volume 10% |
| `volume_down` | None | Decrease volume 10% |
| `mute` | None | Toggle system mute |

#### Platform-Specific Notes

**macOS:**
- `lock_screen`: Uses `pmset` command
- `brightness`: Requires screen brightness permission
- `launch`: Uses `open -a`

**Linux:**
- `lock_screen`: Uses `loginctl` or `xdg-screensaver`
- `brightness`: Requires `/sys/class/backlight` access
- `launch`: Uses `xdg-open`

**Windows:**
- `lock_screen`: Uses `rundll32.exe`
- `brightness`: Uses WMI
- `launch`: Uses `start`

---

## Creating Your Own Plugin

### Template Repository

Start with the official template:

```bash
git clone https://github.com/amiable-dev/conductor-wasm-plugin-template
cd conductor-wasm-plugin-template
```

### Example: Simple Notification Plugin

```rust
// src/lib.rs
use serde::Deserialize;

#[derive(Deserialize)]
struct NotifyParams {
    message: String,
}

#[no_mangle]
pub extern "C" fn init() {
    eprintln!("Notification plugin initialized");
}

#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    let params_bytes = unsafe {
        std::slice::from_raw_parts(params_ptr, params_len)
    };

    let params: NotifyParams = match serde_json::from_slice(params_bytes) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Invalid params: {}", e);
            return 1;
        }
    };

    // Platform-specific notification (simplified)
    #[cfg(target_os = "macos")]
    {
        let cmd = format!(
            "osascript -e 'display notification \"{}\" with title \"Conductor\"'",
            params.message
        );
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .expect("Failed to show notification");
    }

    eprintln!("Notification sent: {}", params.message);
    0
}
```

### Build and Test

```bash
# Build
cargo build --target wasm32-wasip1 --release

# Sign
conductor-sign sign \
  target/wasm32-wasip1/release/my_notify_plugin.wasm \
  ~/.conductor/my-key \
  --name "Your Name" --email "you@example.com"

# Test
cat > test_config.toml <<EOF
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { WasmPlugin = {
    path = "target/wasm32-wasip1/release/my_notify_plugin.wasm",
    params = { "message": "Test notification" }
}}
EOF

conductor --config test_config.toml 0
```

## Best Practices

### Error Handling

```rust
#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Always validate inputs
    if params_len == 0 {
        eprintln!("ERROR: Empty parameters");
        return 1;
    }

    // Handle JSON parsing errors
    let params: MyParams = match serde_json::from_slice(params_bytes) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: Invalid JSON: {}", e);
            return 1;
        }
    };

    // Handle operation errors
    match perform_action(&params) {
        Ok(_) => 0,   // Success
        Err(e) => {
            eprintln!("ERROR: Action failed: {}", e);
            1  // Error
        }
    }
}
```

### Performance Optimization

```rust
use std::sync::OnceLock;

// Lazy initialization (runs once)
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client")
    })
}

#[no_mangle]
pub extern "C" fn execute(params_ptr: *const u8, params_len: usize) -> i32 {
    // Reuse client instead of creating new one
    let client = get_client();

    // Your code...
    0
}
```

### Resource Management

```rust
// Use Drop for cleanup
struct PluginState {
    connection: Option<Connection>,
}

impl Drop for PluginState {
    fn drop(&mut self) {
        if let Some(conn) = &mut self.connection {
            let _ = conn.close();
        }
        eprintln!("Plugin state cleaned up");
    }
}

static mut STATE: Option<PluginState> = None;

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        STATE = Some(PluginState {
            connection: None,
        });
    }
}

#[no_mangle]
pub extern "C" fn shutdown() {
    unsafe {
        STATE = None;  // Triggers Drop
    }
}
```

## Troubleshooting

### Plugin Not Executing

1. **Check logs:**
   ```bash
   DEBUG=1 conductor --config config.toml 0 2>&1 | grep WASM
   ```

2. **Verify WASM format:**
   ```bash
   file my_plugin.wasm
   # Should show: WebAssembly (wasm) binary module
   ```

3. **Check signature:**
   ```bash
   conductor-sign verify my_plugin.wasm
   ```

### Out of Fuel

```rust
// Symptoms: Plugin terminates early

// Solution 1: Optimize code
// - Move heavy work to init()
// - Reduce loop iterations
// - Use lazy initialization

// Solution 2: Increase fuel limit (config.toml)
[wasm]
max_fuel = 200_000_000
```

### Network Requests Failing

```rust
// Check capability is granted
fn capabilities() -> Vec<String> {
    vec!["Network".to_string()]
}

// Use appropriate timeout
let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(5))
    .build()?;

// Handle errors gracefully
match client.get(url).send().await {
    Ok(response) => { /* ... */ },
    Err(e) => {
        eprintln!("Network error: {}", e);
        return 1;
    }
}
```

## Further Reading

- [WASM Plugin Development Guide](wasm-plugin-development.md)
- [Plugin Security](plugin-security.md)
- [WASM Plugins Overview](wasm-plugins.md)
- [Official Plugin Source Code](https://github.com/amiable-dev/conductor/tree/main/plugins)
