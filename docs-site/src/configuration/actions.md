# Actions Reference

Actions define what happens when a trigger condition is met. Conductor supports a rich set of action types that can be composed in powerful ways.

## Quick Reference

| Action Type | Description | Complexity |
|-------------|-------------|------------|
| [Keystroke](#keystroke) | Send keyboard shortcuts | Simple |
| [Text](#text) | Type text strings | Simple |
| [Launch](#launch) | Open applications | Simple |
| [Shell](#shell) | Execute shell commands | Simple |
| [Sequence](#sequence) | Chain multiple actions | Moderate |
| [Delay](#delay) | Add timing control | Simple |
| [MouseClick](#mouseclick) | Simulate mouse clicks | Simple |
| [Repeat](#repeat) | Repeat actions N times | Moderate |
| [VolumeControl](#volumecontrol) | System volume control | Simple |
| [ModeChange](#modechange) | Switch mapping modes | Simple |
| [SendMidi](#sendmidi) | Send MIDI messages | Moderate |
| [Conditional](#conditional) | Context-aware execution | Advanced |

## Simple Actions

### Keystroke

Send keyboard shortcuts with modifiers. The most common action type for productivity workflows.

```toml
# Single key
[modes.mappings.action]
type = "Keystroke"
keys = "space"

# With modifiers
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]  # Cmd+C

# Multiple modifiers
[modes.mappings.action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd", "shift"]  # Cmd+Shift+T
```

**Available Modifiers**:
- `cmd` / `command` / `meta` - Command key (macOS) / Windows key
- `ctrl` / `control` - Control key
- `alt` / `option` - Alt/Option key
- `shift` - Shift key

**Special Keys**:
- Navigation: `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`
- Editing: `backspace`, `delete`, `tab`, `return`, `enter`, `escape`, `esc`
- Function: `f1` through `f12`
- Other: `space`

### Text

Type arbitrary text strings with full Unicode support. Uses keyboard simulation to type character-by-character, making it reliable for Unicode and complex text.

```toml
[modes.mappings.action]
type = "Text"
text = "Hello, World!"
```

**Use Cases**:
- Email signatures and contact information
- Code snippets and boilerplate
- Commonly used phrases and templates
- Form auto-fill with saved data
- Multi-language text input
- Text expansion (abbreviation → full text)

**Advanced Examples**:

**Multi-line Code Snippet**:
```toml
[modes.mappings.action]
type = "Text"
text = """
fn main() {
    println!(\"Hello, World!\");
}
"""
```

**Email Signature**:
```toml
[modes.mappings.action]
type = "Text"
text = """
Best regards,
John Doe
Senior Developer
john@example.com
"""
```

**Unicode and Emoji**:
```toml
[modes.mappings.action]
type = "Text"
text = "✅ Task completed! 🎉"
```

**Special Characters**:
```toml
[modes.mappings.action]
type = "Text"
text = "!@#$%^&*()_+-=[]{}|;':\",./<>?"
```

**Slow Typing for Laggy UIs**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Text", text = "user" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "name" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "123" }
]
```

**TOML String Escaping Tips**:
- Escape quotes: `"He said \"hello\""`
- Multiline strings: `"""Line 1\nLine 2"""`
- Literal strings (no escaping): `'C:\Users\file.txt'`
- Backslashes: Double them `\\` or use literal strings

**Platform Behavior**:
- Does NOT use clipboard (leaves clipboard unchanged)
- Types character-by-character via keyboard simulation
- Respects active keyboard layout
- Requires text input focus in target app

**Troubleshooting**:
- If text doesn't type: Ensure app has input focus
- If Unicode fails: Check app supports UTF-8
- If too fast: Use Sequence with Delay actions
- If wrong characters: Check keyboard layout or escape TOML strings

### Launch

Open applications by name or path. Simple, cross-platform application launcher.

**Basic Usage**:
```toml
[modes.mappings.action]
type = "Launch"
app = "Terminal"  # macOS application name
```

**Full Path**:
```toml
[modes.mappings.action]
type = "Launch"
app = "/Applications/Utilities/Terminal.app"
```

**Script Execution**:
```toml
[modes.mappings.action]
type = "Launch"
app = "/Users/username/scripts/backup.sh"  # Must have execute permissions (chmod +x)
```

**Multiple Apps (Streaming Setup)**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "OBS" },
    { type = "Delay", ms = 2000 },  # Wait for OBS to load
    { type = "Launch", app = "Discord" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Spotify" }
]
```

**Development Environment**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Visual Studio Code" },
    { type = "Delay", ms = 1500 },
    { type = "Launch", app = "Terminal" },
    { type = "Delay", ms = 500 },
    { type = "Launch", app = "Safari" }
]
```

**Platform Behavior**:
- **macOS**: Uses `open -a` command
  - App names: "Safari", "Visual Studio Code", "Logic Pro"
  - Full paths: "/Applications/Safari.app"
  - If running, brings to front (doesn't launch duplicate)
- **Linux**: Direct executable launch
  - Requires full path or executable in $PATH
  - Typically launches new instance even if running
- **Windows**: Uses `cmd /C start` command
  - Accepts app names or paths
  - Uses file associations

**Troubleshooting**:
- Test manually: `open -a "App Name"` (macOS) or `which app-name` (Linux)
- Enable debug logging: `DEBUG=1 cargo run --release 2`
- Use full paths if app names don't work
- Ensure scripts have execute permissions: `chmod +x script.sh`

### Shell

Execute system commands directly without shell interpreter (secure execution).

```toml
[modes.mappings.action]
type = "Shell"
command = "git status"
```

**Security Design**: Commands are executed directly via `Command::new(program).args(args)` without using shell interpreters (`sh`, `bash`, `cmd`). This prevents command injection attacks while supporting common use cases.

**Supported Examples**:
```toml
# Simple commands
command = "git status"
command = "ls -la /tmp"

# File operations
command = "open ~/Downloads"

# System info
command = "system_profiler SPUSBDataType"

# AppleScript (macOS)
command = "osascript -e 'set volume 50'"
command = "osascript -e 'display notification \"MIDI triggered!\"'"
```

**Shell Features NOT Supported** (for security):
- Command chaining: `&&`, `||`, `;`
- Piping: `|`
- Redirection: `>`, `<`, `>>`
- Command substitution: `$(...)`, `` `...` ``
- Variable expansion: `$HOME`, `${VAR}`

**Alternative**: Use `Sequence` action to chain multiple commands:
```toml
# Instead of: "git add . && git commit -m 'save'"
type = "Sequence"
actions = [
    { type = "Shell", command = "git add ." },
    { type = "Shell", command = "git commit -m 'save'" }
]
```

## Timing & Flow Control

### Delay

Add pauses between actions in sequences.

```toml
[modes.mappings.action]
type = "Delay"
ms = 500  # 500 milliseconds
```

**Typical Uses**:
- Wait for UI to load
- Slow down rapid automation
- Add deliberate pacing to sequences

### Sequence

Execute multiple actions in order. Automatic 50ms delay between actions.

```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },  # Spotlight
    { type = "Delay", ms = 200 },  # Wait for Spotlight
    { type = "Text", text = "Terminal" },
    { type = "Keystroke", keys = "return" }  # Launch
]
```

**Design Patterns**:

**1. Application Launcher**:
```toml
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "VS Code" },
    { type = "Keystroke", keys = "return" }
]
```

**2. File Workflow**:
```toml
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "s", modifiers = ["cmd"] },  # Save
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close
    { type = "Delay", ms = 50 },
    { type = "Keystroke", keys = "down" },  # Next file
    { type = "Keystroke", keys = "return" }  # Open
]
```

**3. Git Workflow**:
```toml
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git push" }
]
```

### Repeat

Repeat an action (or sequence) multiple times.

```toml
# Simple repeat: scroll down 10 times
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "down" }
```

**With Delay Between Iterations**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 5
delay_ms = 200  # 200ms between each press
action = { type = "Keystroke", keys = "down" }
```

**Repeat a Sequence**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 3
delay_ms = 1000
action = {
    type = "Sequence",
    actions = [
        { type = "Keystroke", keys = "down" },  # Select next item
        { type = "Delay", ms = 100 },
        { type = "Keystroke", keys = "return" },  # Open
        { type = "Delay", ms = 500 },
        { type = "Keystroke", keys = "w", modifiers = ["cmd"] }  # Close
    ]
}
```

**Error Handling**:
```toml
[modes.mappings.action]
type = "Repeat"
count = 3
delay_ms = 2000
action = { type = "Launch", app = "Xcode" }
```

**Parameters**:
- `count` (required): Number of repetitions (0 = no-op, 1 = run once)
- `action` (required): Action to repeat
- `delay_ms` (optional): Delay in milliseconds between iterations (not applied after last iteration)

**Edge Cases**:
- `count = 0` is valid (no-op)
- `count = 1` executes once with no delay
- Nested repeats multiply: 10 outer × 5 inner = 50 total
- Large counts (>1000) may appear as hang
- Blocking operation - cannot interrupt

**Use Cases**:
1. **Pagination**: Scroll through long lists
2. **Batch Processing**: Repeat workflow on multiple items
3. **Velocity Mapping**: Different repeat counts for soft/medium/hard presses
4. **Retry Logic**: Try launching app multiple times

## Mouse Actions

### MouseClick

Simulate mouse clicks with optional positioning.

```toml
# Click at current cursor position
[modes.mappings.action]
type = "MouseClick"
button = "Left"

# Click at specific coordinates
[modes.mappings.action]
type = "MouseClick"
button = "Right"
x = 500
y = 300
```

**Button Options**: `Left`, `Right`, `Middle`

**Use Cases**:
- Click UI elements at fixed positions
- Context menu automation
- Drag-and-drop workflows (with sequences)

## System Actions

### VolumeControl

Control system volume with platform-specific implementations.

**Platform Support**:
- **macOS**: AppleScript via `osascript` (full support)
- **Linux/Windows**: Not yet implemented

```toml
# Volume up (increment by 5)
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Volume down (decrement by 5)
[modes.mappings.action]
type = "VolumeControl"
operation = "Down"

# Mute
[modes.mappings.action]
type = "VolumeControl"
operation = "Mute"

# Unmute
[modes.mappings.action]
type = "VolumeControl"
operation = "Unmute"

# Set specific volume (0-100)
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
value = 50
```

**Operations**:
- `Up` - Increase volume by 5%
- `Down` - Decrease volume by 5%
- `Mute` - Mute audio output
- `Unmute` - Unmute audio output
- `Set` - Set volume to specific level (0-100), requires `value` parameter

**Encoder Volume Control Example**:
```toml
[[global_mappings]]
description = "Encoder for volume control"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"

[[global_mappings]]
description = "Encoder for volume control"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "CounterClockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Down"
```

**Performance**: <100ms response time on macOS

### ModeChange

Switch between mapping modes programmatically.

```toml
# Switch to specific mode by name
[modes.mappings.action]
type = "ModeChange"
mode = "Media"
```

**Use Cases**:
- **Context Switching**: Switch to different mapping sets
- **Workflow Modes**: Development, Media, Gaming modes
- **Scene Control**: Different modes for streaming scenes

**Example - Mode Switcher Pad**:
```toml
[[modes.mappings]]
description = "Switch to Media mode"
[modes.mappings.trigger]
type = "Note"
note = 15
[modes.mappings.action]
type = "ModeChange"
mode = "Media"
```

**Note**: Mode changes trigger LED color updates and mapping context switches. The mode name must match a defined mode in your configuration.

## MIDI Output Actions

### SendMidi

Send MIDI messages to physical or virtual MIDI output ports. Enables DAW control, external synth control, and MIDI routing.

**Platform Support**: macOS, Linux, Windows (all platforms with MIDI output)

```toml
# Send MIDI Note On
[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"  # Virtual MIDI port name
message_type = "NoteOn"
channel = 0        # MIDI channel 0-15 (maps to channel 1-16)
note = 60          # Middle C (0-127)
velocity = 100     # Note velocity (0-127)
```

**Message Types**:

**NoteOn** - Note on with velocity:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "NoteOn"
channel = 0    # MIDI channel 0-15
note = 60      # Note number (0-127)
velocity = 100 # Velocity (0-127)
```

**NoteOff** - Note off:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "NoteOff"
channel = 0
note = 60
velocity = 0  # Release velocity (usually 0)
```

**CC (Control Change)** - MIDI control change:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "CC"
channel = 0
controller = 7  # Controller number (0-127, e.g., 7=Volume, 1=Modulation)
value = 100     # Controller value (0-127)
```

**ProgramChange** - Change program/preset:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "ProgramChange"
channel = 0
program = 9  # Program number (0-127)
```

**PitchBend** - Pitch bend wheel:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "PitchBend"
channel = 0
value = 0  # Pitch bend value (-8192 to +8191, 0 = center)
```

**Aftertouch** - Channel pressure:
```toml
[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "Aftertouch"
channel = 0
pressure = 64  # Pressure value (0-127)
```

**Use Cases**:
- **DAW Control**: Send notes/CC to control Logic Pro, Ableton Live, FL Studio
- **Virtual Instruments**: Trigger software synths and samplers
- **MIDI Routing**: Route controller input to different destinations
- **Hardware Synths**: Control external synthesizers and drum machines
- **Lighting Control**: Send MIDI to DMX/lighting systems

**Example - Velocity-Sensitive MIDI**:
```toml
[[modes.mappings]]
description = "Expressive MIDI note with velocity curve"

[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7  # Boost soft hits

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60
# Velocity is derived from velocity_mapping transformation
```

**Example - Control DAW Volume**:
```toml
[[modes.mappings]]
description = "Control track volume with CC"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "CC"
channel = 0
controller = 7   # Volume CC
value = 100
```

**Example - Note On/Off Sequence**:
```toml
[modes.mappings.action]
type = "Sequence"
actions = [
  { type = "SendMidi", port = "IAC", message_type = "NoteOn", channel = 0, note = 60, velocity = 100 },
  { type = "Delay", ms = 500 },
  { type = "SendMidi", port = "IAC", message_type = "NoteOff", channel = 0, note = 60, velocity = 0 }
]
```

**Virtual MIDI Ports**:
- **macOS**: Use IAC Driver (Audio MIDI Setup → Window → Show MIDI Studio → IAC Driver)
- **Windows**: Use loopMIDI or similar virtual MIDI port software
- **Linux**: Use ALSA virtual ports

**Note**: Virtual MIDI port creation is not yet automated. Use system tools to create virtual ports, then reference them by name in the `port` parameter.

## Advanced Actions

### Conditional

Execute different actions based on runtime conditions such as time, active app, current mode, or day of week. Enables context-aware, adaptive behavior.

**Complete Documentation**: See [Configuration: Conditionals](conditionals.md) for comprehensive reference.

**Basic Structure**:
```toml
[modes.mappings.action]
type = "Conditional"
condition = { /* condition definition */ }
then_action = { /* action if true */ }
else_action = { /* optional action if false */ }
```

**Quick Examples**:

**Time-Based Launcher**:
```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "TimeRange"
start = "09:00"
end = "17:00"

[modes.mappings.action.then_action]
type = "Launch"
app = "Slack"  # Work hours

[modes.mappings.action.else_action]
type = "Launch"
app = "Discord"  # Off hours
```

**App-Aware Control**:
```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "AppRunning"
app_name = "Logic Pro"

[modes.mappings.action.then_action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60
velocity = 100

[modes.mappings.action.else_action]
type = "Launch"
app = "Logic Pro"
```

**Multiple Conditions (AND)**:
```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] },
  { type = "AppRunning", app_name = "Slack" }
]

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "s"
modifiers = ["cmd", "shift"]
```

**Multiple Conditions (OR)**:
```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "Or"
conditions = [
  { type = "AppFrontmost", app_name = "Safari" },
  { type = "AppFrontmost", app_name = "Chrome" },
  { type = "AppFrontmost", app_name = "Firefox" }
]

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]  # New tab in any browser
```

**Available Condition Types**:
- `Always` - Always true
- `Never` - Always false
- `TimeRange` - Time window (HH:MM format)
- `DayOfWeek` - Specific days (1=Monday through 7=Sunday)
- `AppRunning` - Process detection (macOS, Linux)
- `AppFrontmost` - Active window detection (macOS only)
- `ModeIs` - Current mode matching
- `And` - Logical AND of multiple conditions
- `Or` - Logical OR of multiple conditions
- `Not` - Logical negation

**See Also**:
- [Configuration: Conditionals](conditionals.md) - Complete TOML reference
- [Guide: Context-Aware Mappings](../guides/context-aware.md) - User guide with examples
- [Tutorial: Dynamic Workflows](../tutorials/dynamic-workflows.md) - Step-by-step tutorial

## Action Composition Patterns

### 1. Velocity-Based Repeat Count

Different repeat counts based on how hard you hit the pad:

```toml
[modes.mappings.trigger]
type = "VelocityRange"
note = 10
ranges = [
    # Soft: repeat 3 times
    { min = 0, max = 40, action = {
        type = "Repeat",
        count = 3,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Medium: repeat 5 times
    { min = 41, max = 80, action = {
        type = "Repeat",
        count = 5,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Hard: repeat 10 times
    { min = 81, max = 127, action = {
        type = "Repeat",
        count = 10,
        action = { type = "Keystroke", keys = "down" }
    }}
]
```

### 2. Nested Repeats

Multiply effect (use with caution!):

```toml
[modes.mappings.action]
type = "Repeat"
count = 3  # Outer loop
action = {
    type = "Repeat",
    count = 5,  # Inner loop → 3 × 5 = 15 total
    action = { type = "Keystroke", keys = "down" }
}
```

### 3. Complex Automation Workflow

Combine sequences, repeats, and delays:

```toml
[modes.mappings.action]
type = "Sequence"
actions = [
    # Open file browser
    { type = "Keystroke", keys = "o", modifiers = ["cmd"] },
    { type = "Delay", ms = 500 },

    # Navigate to folder
    { type = "Keystroke", keys = "g", modifiers = ["cmd", "shift"] },
    { type = "Delay", ms = 200 },
    { type = "Text", text = "~/Downloads" },
    { type = "Keystroke", keys = "return" },
    { type = "Delay", ms = 500 },

    # Process first 3 files
    { type = "Repeat", count = 3, delay_between_ms = 1000, action = {
        type = "Sequence",
        actions = [
            { type = "Keystroke", keys = "return" },  # Open file
            { type = "Delay", ms = 800 },
            { type = "Keystroke", keys = "e", modifiers = ["cmd"] },  # Export
            { type = "Delay", ms = 500 },
            { type = "Keystroke", keys = "return" },  # Confirm
            { type = "Delay", ms = 1000 },
            { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close
            { type = "Delay", ms = 200 },
            { type = "Keystroke", keys = "down" }  # Next file
        ]
    }}
]
```

## Performance & Best Practices

### Timing Guidelines

- **Keystroke**: Instant (<1ms)
- **Text**: ~10ms per character
- **Launch**: 100-2000ms (app dependent)
- **Shell**: Variable (command dependent)
- **Sequence**: 50ms auto-delay between actions
- **Delay**: As specified
- **MouseClick**: <5ms
- **Repeat**: count × (action time + delay_between_ms)

### Optimization Tips

1. **Minimize Delays**: Only add delays where needed (UI loading, animations)
2. **Batch Operations**: Use Sequence instead of multiple mappings
3. **Avoid High-Frequency Repeats**: Add `delay_between_ms` for rapid repeats
4. **Test Incrementally**: Start with count=1, then increase
5. **Consider User Experience**: Long-running actions should have feedback

### Common Pitfalls

1. **Blocking Operations**: Repeats and sequences block - cannot interrupt
2. **Timing Fragility**: UI timing varies by system load
3. **Nested Explosions**: 10 × 10 nested repeat = 100 executions
4. **Error Swallowing**: `stop_on_error = false` continues silently

### Debugging Actions

Enable debug mode to see action execution:

```bash
DEBUG=1 cargo run --release 2
```

Watch for:
- Action start/completion logs
- Timing between actions
- Error messages
- Repeat iteration counts

## See Also

- [Action Types Reference](../reference/action-types.md) - Complete technical specifications
- [Configuration Examples](examples.md) - Real-world configuration patterns
- [Trigger Types](../reference/trigger-types.md) - Trigger configuration reference
