# Modes: Context Switching for Different Workflows

## Overview

Modes are Conductor's system for context switching - they allow you to define completely different mapping sets for different workflows, all accessible from the same MIDI controller. Think of modes as "profiles" or "layers" that transform your controller's behavior based on what you're doing.

For example:
- **Default Mode**: General productivity shortcuts (copy, paste, window switching)
- **Developer Mode**: IDE shortcuts, terminal commands, debugging tools
- **Media Mode**: Audio/video playback controls, screen capture, streaming tools

Modes enable a single 16-pad controller to provide hundreds of distinct functions without reconfiguring anything.

## Mode Architecture

### Mode Structure in config.toml

Modes are defined in your `config.toml` using the `[[modes]]` array:

```toml
[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Copy text"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Paste text"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["cmd"]

[[modes]]
name = "Developer"
color = "green"

[[modes.mappings]]
description = "Run tests"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Shell"
command = "cargo test"

[[modes.mappings]]
description = "Git commit"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Shell"
command = "git commit"
```

**Key Points**:
- Each `[[modes]]` section defines a complete mode
- `name`: Displayed in console when switching modes
- `color`: LED feedback color theme (optional but recommended)
- `[[modes.mappings]]`: Array of triggers and actions for this mode

### Mode Colors and LED Feedback

Each mode can have a distinct color theme for visual identification:

**Available Colors**:
- `blue` - Default mode (calm, general use)
- `green` - Development/productivity (focused work)
- `purple` - Media/creative (entertainment)
- `red` - Emergency/system (critical functions)
- `yellow` - Testing/debug (temporary mappings)
- `white` - Neutral (fallback)

**How Colors Work**:
```toml
[[modes]]
name = "Default"
color = "blue"  # All pads in this mode default to blue when idle
```

When using the `reactive` lighting scheme:
- Idle pads show the mode's base color at dim brightness
- Pressed pads light up with velocity-based colors (green/yellow/red)
- Released pads fade back to the mode's base color

When using static patterns (rainbow, wave, etc.):
- The mode color is used as the primary theme color
- Pattern variations are tinted with the mode color

## Global vs Mode-Specific Mappings

Conductor supports two types of mappings:

### Mode-Specific Mappings

Defined inside `[[modes.mappings]]`, these only work when that mode is active:

```toml
[[modes]]
name = "Media"
color = "purple"

[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = []
```

This Play/Pause mapping only works in Media mode.

### Global Mappings

Defined in `[[global_mappings]]` at the top level, these work in ALL modes:

```toml
[[global_mappings]]
description = "Emergency exit"
[global_mappings.trigger]
type = "LongPress"
note = 0
hold_duration_ms = 3000
[global_mappings.action]
type = "Shell"
command = "killall conductor"

[[global_mappings]]
description = "Volume up (always available)"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Use Cases for Global Mappings**:
- Emergency shutdown (long press escape)
- Volume control (encoder always controls volume)
- Mode switching (dedicated mode-change buttons)
- System-wide shortcuts (screenshots, lock screen)

**Priority Order**:
1. Mode-specific mappings are checked first
2. If no match, global mappings are checked
3. If still no match, the event is ignored

This means mode-specific mappings can "override" global ones by using the same trigger.

## Switching Between Modes

There are three ways to switch modes:

### 1. Encoder Rotation

The most common method - use your encoder as a mode selector:

```toml
[[global_mappings]]
description = "Next mode (encoder clockwise)"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = "next"

[[global_mappings]]
description = "Previous mode (encoder counter-clockwise)"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = "previous"
```

This creates a circular mode selector:
- Turn clockwise: Default → Developer → Media → Default...
- Turn counter-clockwise: Default → Media → Developer → Default...

### 2. Dedicated Mode Buttons

Assign specific pads to jump directly to modes:

```toml
[[global_mappings]]
description = "Jump to Developer mode"
[global_mappings.trigger]
type = "Note"
note = 15  # Top-right pad
[global_mappings.action]
type = "ModeChange"
mode = "Developer"

[[global_mappings]]
description = "Jump to Media mode"
[global_mappings.trigger]
type = "Note"
note = 11  # Another pad
[global_mappings.action]
type = "ModeChange"
mode = "Media"
```

### 3. Chord-Based Mode Switching

Use pad combinations for advanced mode switching:

```toml
[[global_mappings]]
description = "Secret admin mode (pads 0+1+2 together)"
[global_mappings.trigger]
type = "NoteChord"
notes = [0, 1, 2]
chord_timeout_ms = 100
[global_mappings.action]
type = "ModeChange"
mode = "Admin"
```

**ModeChange Action Parameters**:
- `mode = "next"` - Cycle to next mode
- `mode = "previous"` - Cycle to previous mode
- `mode = "Default"` - Jump to specific mode by name (case-sensitive)
- `mode = 0` - Jump to mode by index (0-based)

## Practical Mode Examples

### Example 1: Three-Mode General Setup

```toml
# Mode 0: Default (General Productivity)
[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Copy"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Paste"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Switch to next app"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Keystroke"
keys = "tab"
modifiers = ["cmd"]

# Mode 1: Developer (Coding Tools)
[[modes]]
name = "Developer"
color = "green"

[[modes.mappings]]
description = "Run tests"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Shell"
command = "cargo test"

[[modes.mappings]]
description = "Build release"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Shell"
command = "cargo build --release"

[[modes.mappings]]
description = "Open terminal"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Launch"
app = "Terminal"

# Mode 2: Media (Audio/Video Control)
[[modes]]
name = "Media"
color = "purple"

[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = []

[[modes.mappings]]
description = "Next track"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Keystroke"
keys = "right"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Screenshot"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Keystroke"
keys = "4"
modifiers = ["cmd", "shift"]

# Global: Mode switching and volume
[[global_mappings]]
description = "Next mode"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = "next"

[[global_mappings]]
description = "Previous mode"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = "previous"
```

### Example 2: Context-Aware Developer Modes

```toml
# General development
[[modes]]
name = "Dev-General"
color = "green"

[[modes.mappings]]
description = "Save all"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "s"
modifiers = ["cmd", "alt"]

# Rust-specific
[[modes]]
name = "Dev-Rust"
color = "green"

[[modes.mappings]]
description = "cargo check"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Shell"
command = "cargo check"

[[modes.mappings]]
description = "cargo fmt"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Shell"
command = "cargo fmt"

# Python-specific
[[modes]]
name = "Dev-Python"
color = "green"

[[modes.mappings]]
description = "pytest"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Shell"
command = "pytest"

[[modes.mappings]]
description = "black format"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Shell"
command = "black ."
```

### Example 3: Velocity-Sensitive Multi-Mode

Combine modes with velocity detection for even more control:

```toml
[[modes]]
name = "Advanced"
color = "yellow"

# Soft press: Small increase
[[modes.mappings]]
description = "Small volume up"
[modes.mappings.trigger]
type = "VelocityRange"
note = 12
min_velocity = 0
max_velocity = 40
[modes.mappings.action]
type = "Shell"
command = "osascript -e 'set volume output volume (output volume of (get volume settings) + 5)'"

# Hard press: Large increase
[[modes.mappings]]
description = "Large volume up"
[modes.mappings.trigger]
type = "VelocityRange"
note = 12
min_velocity = 81
max_velocity = 127
[modes.mappings.action]
type = "Shell"
command = "osascript -e 'set volume output volume (output volume of (get volume settings) + 20)'"
```

## Mode Design Best Practices

### 1. Use Descriptive Names

```toml
# Good: Clear what the mode does
[[modes]]
name = "Media-Playback"

# Bad: Ambiguous
[[modes]]
name = "Mode3"
```

### 2. Assign Consistent Color Themes

```toml
# Productivity modes: Blue family
[[modes]]
name = "Default"
color = "blue"

[[modes]]
name = "Email"
color = "blue"

# Development modes: Green family
[[modes]]
name = "Developer"
color = "green"

[[modes]]
name = "Testing"
color = "green"

# Creative modes: Purple/Magenta family
[[modes]]
name = "Media"
color = "purple"

[[modes]]
name = "Design"
color = "magenta"
```

### 3. Keep Mode Count Manageable

**Recommended**: 3-5 modes for most users
- Too few modes: You'll run out of pads for all your functions
- Too many modes: You'll forget which mode you're in

**Strategy**: Group related functions into modes rather than creating a mode for every app.

### 4. Reserve Global Mappings for Critical Functions

Only use `[[global_mappings]]` for:
- Mode switching itself
- Emergency shutdowns
- System-wide controls (volume, screen lock)
- Functions that should work regardless of mode

### 5. Use Mode Colors for Visual Feedback

If your device supports LED feedback:
- Test each mode and verify the color matches the function
- Use calming colors (blue, green) for frequent modes
- Use attention-grabbing colors (red, yellow) for special modes

### 6. Document Your Modes

Add descriptions to help remember your layout:

```toml
[[modes]]
name = "Developer"
color = "green"
# Description comments help future you remember the layout:
# Pad 0-3: Git commands (status, add, commit, push)
# Pad 4-7: Build commands (check, test, build, run)
# Pad 8-11: IDE shortcuts (format, refactor, debug, terminal)
# Pad 12-15: Project navigation (files, search, grep, docs)

[[modes.mappings]]
description = "Git status"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Shell"
command = "git status"
```

## Advanced Mode Techniques

### Mode-Specific Timing Adjustments

Override advanced settings per mode:

```toml
[advanced_settings]
chord_timeout_ms = 100
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Gaming"
color = "red"
# Gaming mode needs faster response
# (Note: Per-mode advanced settings not yet implemented,
#  but this is the planned syntax)
```

### Conditional Mode Switching

Combine with Conditional actions for smart mode switching:

```toml
[[global_mappings]]
description = "Auto-switch to Dev mode if VS Code is active"
[global_mappings.trigger]
type = "Note"
note = 15
[global_mappings.action]
type = "Conditional"
condition = "ActiveApp"
value = "Visual Studio Code"
then_action = { type = "ModeChange", mode = "Developer" }
else_action = { type = "ModeChange", mode = "Default" }
```

### Mode Sequences

Chain mode changes with other actions:

```toml
[[modes.mappings]]
description = "Open Spotify and switch to Media mode"
[modes.mappings.trigger]
type = "Note"
note = 15
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Spotify" },
    { type = "Delay", duration_ms = 1000 },
    { type = "ModeChange", mode = "Media" }
]
```

## Troubleshooting Modes

### Mode Not Switching

**Symptoms**: Encoder turns but mode doesn't change

**Checks**:
- Verify `[[global_mappings]]` contains ModeChange actions
- Check encoder is mapped correctly (use `cargo run --bin midi_diagnostic 2`)
- Ensure mode names are spelled exactly as defined
- Look for console output confirming mode changes

**Debug**:
```bash
DEBUG=1 cargo run --release 2
# Look for: "Mode changed: Default -> Developer"
```

### Mappings Not Working in Mode

**Symptoms**: Pad press does nothing in a specific mode

**Checks**:
- Verify you're in the correct mode (check console output)
- Confirm note numbers match (use `cargo run --bin pad_mapper`)
- Check `[[modes.mappings]]` is properly nested under the mode
- Ensure TOML syntax is valid

**Test**:
```bash
# Add a simple test mapping to verify mode is active
[[modes.mappings]]
description = "Test mode active"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Shell"
command = "echo 'Mode working!' | tee /dev/tty"
```

### Conflicting Mappings

**Symptoms**: Unexpected action triggers

**Checks**:
- Global mappings override mode mappings if using same trigger
- Multiple modes might have similar triggers
- Check for copy-paste errors in note numbers

**Solution**: Use unique note numbers for each function, or intentionally use the priority system:
```toml
# Global: Emergency stop (works everywhere)
[[global_mappings]]
[global_mappings.trigger]
type = "LongPress"
note = 0

# Mode: Normal pad 0 function (only when not held)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 0
```

## See Also

- [First Mapping Tutorial](first-mapping.md) - Learn basic mapping syntax
- [Configuration Overview](../configuration/overview.md) - Complete config.toml structure
- [Actions Reference](../reference/actions.md) - All available actions including ModeChange
- [LED System](../LED_SYSTEM.md) - How mode colors work with LED feedback

---

**Last Updated**: November 11, 2025
**Implementation Status**: Fully functional in current release
