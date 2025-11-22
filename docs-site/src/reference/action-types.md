# Action Types

Actions are what Conductor executes when a trigger is detected. This page documents all available action types and their configuration.

## Core Actions

### Keystroke

Simulates keyboard input with optional modifiers.

```toml
[action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]
```

**Parameters:**
- `keys` (string): Key(s) to press (e.g., "space", "return", "f1")
- `modifiers` (array): Optional modifiers: "cmd", "ctrl", "alt", "shift"

**Supported Keys:**
- Letters: `a-z`
- Numbers: `0-9`
- Special: `space`, `return`, `tab`, `escape`, `backspace`, `delete`
- Arrows: `up`, `down`, `left`, `right`
- Function: `f1` through `f12`
- Navigation: `home`, `end`, `pageup`, `pagedown`

### Text

Types arbitrary text strings with full Unicode support. Uses keyboard simulation to type character-by-character.

```toml
[action]
type = "Text"
text = "Hello, World!"
```

**Parameters:**
- `text` (string): Text to type (supports Unicode, emoji, multi-line)

**Use Cases:**
- Text snippets and templates
- Email signatures and contact information
- Code boilerplate and common patterns
- Form auto-fill
- Multi-language text input

**Configuration Examples:**

**Simple Text:**
```toml
[action]
type = "Text"
text = "user@example.com"
```

**Multi-line Code Snippet:**
```toml
[action]
type = "Text"
text = """
fn main() {
    println!(\"Hello, World!\");
}
"""
```

**Unicode and Emoji:**
```toml
[action]
type = "Text"
text = "✅ Task completed! 🎉"
```

**Special Characters:**
```toml
[action]
type = "Text"
text = "!@#$%^&*()_+-=[]{}|;':\",./<>?"
```

**Text with Delay (using Sequence):**
```toml
[action]
type = "Sequence"
actions = [
    { type = "Text", text = "username" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "@company.com" }
]
```

**TOML String Escaping:**
```toml
# Escape quotes in basic strings
text = "He said \"hello\""

# Use multiline strings for complex text
text = """
Line 1
Line 2 with "quotes"
"""

# Use literal strings to avoid escaping backslashes
text = 'C:\Users\Documents\file.txt'
```

**Platform Support:**
- macOS: Accessibility APIs
- Linux: X11/Wayland
- Windows: Windows API

**Behavior:**
- Types character-by-character (no clipboard)
- Respects active keyboard layout
- Typing speed controlled by OS
- Requires text input focus

**Troubleshooting:**
- Ensure target app has input focus
- Check Input Monitoring permissions (macOS)
- For slow UIs, use Sequence with Delay actions
- For special keys (Enter, Tab, Ctrl), use Keystroke action instead

### Launch

Opens an application by name or path using platform-specific system commands.

```toml
[action]
type = "Launch"
app = "Safari"
```

**Parameters:**
- `app` (string): Application name or full path to executable/bundle

**Platform Behavior:**

**macOS:**
- Uses `open -a` command
- Accepts app names: "Safari", "Visual Studio Code", "Logic Pro"
- Accepts full paths: "/Applications/Safari.app"
- If app already running, brings to front
- Searches standard paths: /Applications, ~/Applications, /System/Applications

**Linux:**
- Direct executable launch via `Command::spawn()`
- Requires full path or executable in $PATH
- No automatic .desktop file resolution
- Typically launches new instance even if already running

**Windows:**
- Uses `cmd /C start` command
- Accepts app names or paths
- Uses Windows file associations
- Behavior varies by app's single-instance implementation

**Examples:**

```toml
# Launch by app name (macOS)
[action]
type = "Launch"
app = "Terminal"

# Launch by full path
[action]
type = "Launch"
app = "/Applications/Utilities/Terminal.app"

# Launch script with full path
[action]
type = "Launch"
app = "/Users/username/scripts/backup.sh"

# Launch multiple apps in sequence
[action]
type = "Sequence"
actions = [
    { type = "Launch", app = "VS Code" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Terminal" }
]
```

**Troubleshooting:**
- **App not found**: Use full path instead of app name
- **macOS test**: Run `open -a "App Name"` in Terminal
- **Linux test**: Run `which app-name` or use full path
- **Silent failures**: Enable debug mode with `DEBUG=1`
- **Permissions**: Ensure execute permissions on scripts (`chmod +x`)

**Notes:**
- Launch is non-blocking (returns immediately)
- No validation that app exists before launch attempt
- Errors fail silently (check debug logs)
- App names with spaces are automatically handled
- Launch time varies: 100ms (small apps) to 10s (large IDEs/DAWs)

### Shell

Executes a shell command.

```toml
[action]
type = "Shell"
command = "npm test"
```

**Parameters:**
- `command` (string): Shell command to execute

**Platform Notes:**
- Unix/macOS: Uses `sh -c`
- Windows: Uses `cmd /C`

**Security Warning:** Be cautious with user input in shell commands.

## System Actions

### VolumeControl

Controls system volume with cross-platform support for increase, decrease, mute/unmute, and absolute level setting.

```toml
[action]
type = "VolumeControl"
operation = "Up"
amount = 5  # Optional: volume increment (default 5)
```

**Parameters:**
- `operation` (string): Volume operation to perform
- `amount` (integer, optional):
  - For `Up`/`Down`: Increment amount 0-100 (default 5)
  - For `Set`: Absolute level 0-100

**Operations:**
- `Up`: Increase volume by amount
- `Down`: Decrease volume by amount
- `Mute`: Mute audio output
- `Unmute`: Unmute audio output
- `Toggle`: Toggle mute state
- `Set`: Set to specific level (requires `amount` parameter)

**Platform Support:**

| Platform | Method | Latency | Dependencies |
|----------|--------|---------|--------------|
| macOS | AppleScript | 50-100ms | None (built-in) |
| Linux (PulseAudio) | pactl | 10-30ms | pulseaudio-utils |
| Linux (Pipewire) | wpctl | 5-15ms | wireplumber |
| Linux (ALSA) | amixer | 15-40ms | alsa-utils |
| Windows | nircmd | 20-50ms | nircmd.exe |
| Windows | COM API | 5-10ms | None (built-in) |

**Configuration Examples:**

```toml
# Encoder volume control
[[global_mappings]]
description = "Volume Up"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
amount = 2  # Small increments for smooth control

[[global_mappings]]
description = "Volume Down"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "CounterClockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Down"
amount = 2

# Mute toggle
[[global_mappings]]
description = "Mute/Unmute"
[global_mappings.trigger]
type = "Note"
note = 16
[global_mappings.action]
type = "VolumeControl"
operation = "Toggle"

# Set to specific level
[[modes.mappings]]
description = "Set volume to 50%"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
amount = 50

# Velocity-based volume (multiple mappings)
[[modes.mappings]]
description = "Soft press = 25%"
[modes.mappings.trigger]
type = "VelocityRange"
note = 9
min_velocity = 0
max_velocity = 40
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
amount = 25

[[modes.mappings]]
description = "Medium press = 50%"
[modes.mappings.trigger]
type = "VelocityRange"
note = 9
min_velocity = 41
max_velocity = 80
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
amount = 50

[[modes.mappings]]
description = "Hard press = 75%"
[modes.mappings.trigger]
type = "VelocityRange"
note = 9
min_velocity = 81
max_velocity = 127
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
amount = 75
```

**Use Cases:**
- **Producer**: Encoder for smooth volume adjustment while mixing
- **Developer**: Quick mute during video calls, volume presets for different activities
- **Streamer**: Real-time volume balancing during streams, emergency mute
- **Presenter**: Volume control without touching laptop during presentations

**Troubleshooting:**

***macOS:***
- No dependencies required
- AppleScript latency ~50-100ms is normal
- Requires no special permissions

***Linux:***
- Install dependencies:
  ```bash
  # PulseAudio (most common)
  sudo apt install pulseaudio-utils

  # Pipewire (modern, fastest)
  sudo apt install pipewire wireplumber

  # ALSA (minimal systems)
  sudo apt install alsa-utils
  ```
- Conductor auto-detects available backend
- User must be in `audio` group

***Windows:***
- Download nircmd.exe from https://www.nirsoft.net/utils/nircmd.html
- Add to PATH or place in Conductor directory
- COM API fallback requires no dependencies

**Performance Notes:**
- Volume commands are non-blocking
- State changes typically take 5-100ms depending on platform
- Multiple rapid commands may queue on slower platforms (macOS AppleScript)

### ModeChange

Switches between different mapping modes with optional LED transition effects and relative navigation.

```toml
[action]
type = "ModeChange"
mode = 1  # Switch to mode index 1
```

**Parameters:**
- `mode` (integer): Zero-based mode index or offset (if `relative = true`)
- `relative` (boolean, optional): If true, `mode` is offset from current mode (default false)
- `transition_effect` (string, optional): Visual transition effect: "Flash", "Sweep", "FadeOut", "Spiral", "None"

**Mode Indexing:**
- Modes are zero-based: Mode 0, Mode 1, Mode 2, etc.
- Mode 0 is typically the default mode on startup
- Invalid mode indices are clamped to valid range

**Absolute vs Relative Mode Changes:**

***Absolute (direct jump):***
```toml
[action]
type = "ModeChange"
mode = 2  # Jump directly to Mode 2
transition_effect = "Flash"
```

***Relative (navigation):***
```toml
# Next mode (with wrapping)
[action]
type = "ModeChange"
mode = 1  # +1 offset
relative = true
transition_effect = "Sweep"

# Previous mode (with wrapping)
[action]
type = "ModeChange"
mode = -1  # -1 offset
relative = true
```

**Transition Effects:**

| Effect | Duration | Description |
|--------|----------|-------------|
| Flash | 150ms | Quick white flash |
| Sweep | 120ms | Left-to-right wave |
| FadeOut | 200ms | Fade out old, fade in new |
| Spiral | 240ms | Center-outward spiral |
| None | 0ms | Instant switch |

**Configuration Examples:**

```toml
# Encoder for mode cycling
[[global_mappings]]
description = "Next Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1  # +1 offset
relative = true
transition_effect = "Sweep"

[[global_mappings]]
description = "Previous Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = -1  # -1 offset (wraps backward)
relative = true
transition_effect = "Sweep"

# Direct mode selection with chords
[[global_mappings]]
description = "Jump to Default Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 9]  # Top-left corners
[global_mappings.action]
type = "ModeChange"
mode = 0
transition_effect = "Flash"

[[global_mappings]]
description = "Jump to Development Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [2, 10]
[global_mappings.action]
type = "ModeChange"
mode = 1
transition_effect = "Flash"

[[global_mappings]]
description = "Jump to Media Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [3, 11]
[global_mappings.action]
type = "ModeChange"
mode = 2
transition_effect = "FadeOut"

# Conditional mode change
[[modes.mappings]]
description = "Switch to Media mode if Spotify running"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "Conditional"
conditions = [{ type = "AppRunning", bundle_id = "com.spotify.client" }]
then_action = { type = "ModeChange", mode = 2, transition_effect = "FadeOut" }
else_action = { type = "Launch", app = "Spotify" }
```

**Mode Configuration:**

Modes are defined in your config.toml with names and LED colors:

```toml
[[modes]]
name = "Default"
color = "blue"             # LED color for this mode
led_idle_brightness = 20   # Brightness when pad not pressed
led_active_brightness = 255 # Brightness when pad pressed
mappings = [...]

[[modes]]
name = "Development"
color = "green"
led_idle_brightness = 30
led_active_brightness = 255
mappings = [...]

[[modes]]
name = "Media"
color = "purple"
led_idle_brightness = 15
led_active_brightness = 200
mappings = [...]
```

**LED Feedback Integration:**
- Mode colors automatically update LEDs on mode change
- Transition effects provide visual feedback
- Optional mode indicator pads (e.g., bottom row shows active mode)
- Idle/active brightness levels per mode

**Use Cases:**

***Developer (Sam) - Context Switching:***
- **Mode 0**: General productivity shortcuts
- **Mode 1**: IDE shortcuts (build, test, debug)
- **Mode 2**: Media controls
- Encoder rotation for quick mode cycling

***Producer (Alex) - Production Workflow:***
- **Mode 0**: Recording (transport, record arm)
- **Mode 1**: Mixing (volume, mute/solo, effects)
- **Mode 2**: Mastering (compressor, EQ, limiter)
- Chord combinations for instant mode jumps

***Streamer (Jordan) - Live Streaming:***
- **Mode 0**: Pre-Stream (setup checks, app launching)
- **Mode 1**: Live (scene switching, alerts)
- **Mode 2**: BRB (limited controls, auto-mute)
- **Mode 3**: Post-Stream (save, shutdown sequence)

***Designer (Taylor) - Creative Workflows:***
- **Mode 0**: Sketch (drawing tools, layers)
- **Mode 1**: Edit (selection, transform, filters)
- **Mode 2**: Export (save presets, formats)
- Visual LED feedback shows current mode

**Circular Mode Navigation:**

With `relative = true`, modes wrap around:
- Mode 0 → +1 → Mode 1
- Mode 1 → +1 → Mode 2
- Mode 2 → +1 → Mode 0 (wraps to beginning)
- Mode 0 → -1 → Mode 2 (wraps to end)

**Troubleshooting:**

***Mode Not Switching:***
- Verify mode index is within range (0 to num_modes-1)
- Check that modes are defined in config.toml
- Enable debug logging: `DEBUG=1 cargo run`
- Ensure mode change mapping is in `global_mappings` to work from all modes

***LEDs Not Updating:***
- Verify `color` is set for each mode
- Check device supports RGB LED feedback
- Ensure LED feedback is enabled
- Test with different transition effects

***Transition Effects Not Working:***
- Only works with devices supporting RGB LEDs (Maschine Mikro MK3, etc.)
- MIDI-only devices show instant switches
- Check transition timing isn't too fast to notice

**Performance Notes:**
- Mode switches are near-instant (<1ms)
- Transition effects add 0-240ms visual delay
- LED updates are non-blocking
- Rapid mode changes are queued and handled gracefully

## Advanced Actions

### Sequence

Executes multiple actions in order.

```toml
[action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "n", modifiers = ["cmd"] },
    { type = "Delay", ms = 500 },
    { type = "Text", text = "New Document" }
]
```

**Parameters:**
- `actions` (array): Array of action configurations

**Notes:**
- Actions execute sequentially
- Default 50ms delay between actions
- Use `Delay` action for longer pauses

### Delay

Pauses execution.

```toml
[action]
type = "Delay"
ms = 1000  # Wait 1 second
```

**Parameters:**
- `ms` (integer): Milliseconds to wait

**Use Cases:**
- Wait for app to launch
- Timing between keystrokes
- Synchronization in sequences

### Repeat

Repeats an action multiple times.

```toml
[action]
type = "Repeat"
count = 10
delay_between_ms = 100
action = { type = "Keystroke", keys = "right" }
```

**Parameters:**
- `action` (object): Action to repeat
- `count` (integer): Number of repetitions
- `delay_between_ms` (integer, optional): Delay between repetitions in milliseconds

**Use Cases:**
- Navigate through items
- Rapid-fire actions
- Automation tasks

### MouseClick

Simulates mouse clicks.

```toml
[action]
type = "MouseClick"
button = "left"
x = 100
y = 200
```

**Parameters:**
- `button` (string): "left", "right", or "middle"
- `x` (integer, optional): X coordinate (absolute)
- `y` (integer, optional): Y coordinate (absolute)

**Notes:**
- If `x` and `y` are omitted, clicks at current cursor position
- Coordinates are screen-absolute

### Conditional

Executes actions based on conditions.

```toml
[action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.apple.Logic" }
]
operator = "And"
then_action = { type = "Keystroke", keys = "space" }
else_action = { type = "Launch", app = "Logic Pro" }
```

**Parameters:**
- `conditions` (array): Array of condition objects
- `operator` (string, optional): "And" or "Or" (default: "And")
- `then_action` (object): Action when conditions are true
- `else_action` (object, optional): Action when conditions are false

**Condition Types:**

#### AppRunning
Check if an application is running.
```toml
{ type = "AppRunning", bundle_id = "com.spotify.client" }
```

#### AppNotRunning
Check if an application is not running.
```toml
{ type = "AppNotRunning", bundle_id = "com.spotify.client" }
```

#### TimeRange
Check if current time is within a range.
```toml
{ type = "TimeRange", start = "09:00", end = "17:00" }
```

**Format:** HH:MM in 24-hour format
**Note:** Uses local system time

#### DayOfWeek
Check if today matches specified days.
```toml
{ type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
```

**Valid Days:** "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"

#### ModifierPressed
Check if a modifier key is held.
```toml
{ type = "ModifierPressed", modifier = "Shift" }
```

**Valid Modifiers:** "Shift", "Ctrl", "Cmd", "Alt", "Option"

#### ModeActive
Check if a specific mode is active.
```toml
{ type = "ModeActive", mode = 1 }
```

**Parameter:** `mode` (integer): Zero-based mode index

**Operators:**
- `And`: All conditions must be true
- `Or`: At least one condition must be true

**Nested Conditionals:**
You can nest conditionals for complex decision trees:
```toml
[action]
type = "Conditional"
conditions = [{ type = "TimeRange", start = "09:00", end = "17:00" }]
then_action = {
    type = "Conditional",
    conditions = [{ type = "AppRunning", bundle_id = "com.microsoft.VSCode" }],
    then_action = { type = "Keystroke", keys = "b", modifiers = ["cmd", "shift"] },
    else_action = { type = "Launch", app = "Visual Studio Code" }
}
else_action = { type = "Launch", app = "Spotify" }
```

## Action Composition

Actions can be combined in powerful ways:

**Example: Complex Workflow**
```toml
[action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Terminal" },
    { type = "Delay", ms = 1000 },
    { type = "Text", text = "cd ~/projects && npm test" },
    { type = "Keystroke", keys = "return" }
]
```

**Example: Conditional with Repeat**
```toml
[action]
type = "Conditional"
conditions = [{ type = "ModifierPressed", modifier = "Shift" }]
then_action = {
    type = "Repeat",
    count = 5,
    delay_between_ms = 200,
    action = { type = "Keystroke", keys = "right" }
}
else_action = { type = "Keystroke", keys = "right" }
```

## Performance Notes

- **Keystroke**: <1ms execution time
- **Shell**: Asynchronous, non-blocking
- **Launch**: Platform-dependent, typically 100-500ms
- **Conditional**: 10-50ms per condition evaluation
- **Sequence**: Sum of individual action times + delays

## Troubleshooting

### Keystroke Not Working
- Verify key name spelling
- Check modifier syntax
- Ensure app has input focus

### Shell Command Fails Silently
- Test command in terminal first
- Check environment variables
- Enable debug logging: `DEBUG=1 cargo run`

### Conditional Not Triggering
- Verify bundle ID format (macOS): `osascript -e 'id of app "AppName"'`
- Check time format (24-hour: "09:00", not "9:00 AM")
- Verify day names match exactly

### Launch App Not Found
- Use exact app name as it appears in Applications folder
- On Linux, use full executable path if not in PATH
- Check permissions for app execution

## See Also

- [Configuration Examples](../configuration/examples.md)
- [Trigger Types](trigger-types.md)
- [Modes and Mappings](../configuration/modes.md)
