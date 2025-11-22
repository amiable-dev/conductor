# Configuration Overview

## Introduction

Conductor uses a single `config.toml` file to define all device settings, modes, mappings, and advanced behavior. This file uses TOML (Tom's Obvious, Minimal Language) - a human-friendly configuration format that's easy to read and edit.

This guide provides a comprehensive overview of the configuration structure, required sections, validation, and best practices.

## Configuration File Location

By default, Conductor looks for `config.toml` in the current working directory:

```bash
# Current directory
./config.toml

# Or specify a custom location
conductor --config /path/to/my-config.toml
```

**Tip**: Keep your `config.toml` in the project root directory for simplicity.

## TOML Syntax Basics

If you're new to TOML, here are the essentials:

```toml
# Comments start with #

# Simple key-value pairs
name = "My Device"
port = 2

# Booleans
auto_connect = true
debug = false

# Numbers
velocity = 127
timeout_ms = 300

# Strings (use quotes)
description = "This is a description"

# Arrays
notes = [12, 13, 14, 15]
modifiers = ["cmd", "shift"]

# Tables (sections)
[section_name]
key = "value"

# Array of tables (repeated sections)
[[modes]]
name = "Mode 1"

[[modes]]
name = "Mode 2"
```

**Key Points**:
- Use `=` for assignment
- Use `[]` for sections (tables)
- Use `[[]]` for repeating sections (arrays of tables)
- Strings must be in quotes
- Comments use `#`

## Configuration Structure Diagram

```
config.toml
├── [device]                    # Device identification (optional)
│   ├── name
│   └── auto_connect
│
├── [advanced_settings]         # Timing thresholds (optional)
│   ├── chord_timeout_ms
│   ├── double_tap_timeout_ms
│   └── hold_threshold_ms
│
├── [[modes]]                   # Mode definitions (required, 1+)
│   ├── name
│   ├── color
│   └── [[modes.mappings]]      # Mode-specific mappings
│       ├── description
│       ├── [trigger]
│       └── [action]
│
└── [[global_mappings]]         # Global mappings (optional)
    ├── description
    ├── [trigger]
    └── [action]
```

## Required vs Optional Sections

### Required Sections

**Minimum viable config**:

```toml
# At least one mode is REQUIRED
[[modes]]
name = "Default"

# At least one mapping (either in modes or global) is REQUIRED
[[modes.mappings]]
description = "Example mapping"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []
```

Without these, Conductor will start but won't do anything useful.

### Optional Sections

These enhance functionality but aren't required:

```toml
# Optional: Device identification
[device]
name = "Mikro"
auto_connect = true

# Optional: Timing customization
[advanced_settings]
chord_timeout_ms = 100
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

# Optional: Global mappings (work in all modes)
[[global_mappings]]
description = "Volume control"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
```

## Section Breakdown

### [device] Section

Optional section for device identification:

```toml
[device]
name = "Mikro"           # Friendly name (used in logs)
auto_connect = true      # Automatically connect to device on startup
```

**Parameters**:
- `name` (string): Device name for logging/display
- `auto_connect` (boolean): If true, auto-connect to first available MIDI device

**Default behavior** (if omitted): No auto-connect, manual port selection required.

### [advanced_settings] Section

Optional timing customization:

```toml
[advanced_settings]
chord_timeout_ms = 100          # Max time between notes in a chord
double_tap_timeout_ms = 300     # Max time between taps for double-tap
hold_threshold_ms = 2000        # Min hold time for long press
```

**Parameters**:
- `chord_timeout_ms` (u64): Milliseconds to wait for additional notes in a chord (default: 100)
- `double_tap_timeout_ms` (u64): Maximum time between two taps to count as double-tap (default: 300)
- `hold_threshold_ms` (u64): Minimum hold duration for long press trigger (default: 2000)

**Defaults** (if omitted):
```toml
chord_timeout_ms = 100
double_tap_timeout_ms = 300
hold_threshold_ms = 2000
```

**Use cases**:
- Increase `chord_timeout_ms` for slower chord playing (e.g., 200-300ms)
- Decrease `double_tap_timeout_ms` for faster double-tap detection (e.g., 200ms)
- Adjust `hold_threshold_ms` for longer/shorter long-press (e.g., 1000-3000ms)

### [[modes]] Section

Defines a mode with its mappings. **At least one mode is required.**

```toml
[[modes]]
name = "Default"              # Mode name (required, unique)
color = "blue"                # LED color theme (optional)

[[modes.mappings]]            # Mode-specific mappings (array)
description = "Copy"          # Human-readable description (optional)
[modes.mappings.trigger]      # Trigger definition (required)
type = "Note"
note = 12
[modes.mappings.action]       # Action definition (required)
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]
```

**Mode Parameters**:
- `name` (string, required): Unique mode identifier
- `color` (string, optional): LED theme color (blue, green, purple, red, yellow, white)

**Mapping Parameters**:
- `description` (string, optional): Human-readable description (useful for documentation)
- `trigger` (table, required): Defines what event triggers this mapping
- `action` (table, required): Defines what happens when triggered

**Multiple modes**:
```toml
[[modes]]
name = "Default"
# ... mappings ...

[[modes]]
name = "Developer"
# ... mappings ...

[[modes]]
name = "Media"
# ... mappings ...
```

### [[global_mappings]] Section

Optional mappings that work in ALL modes:

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
description = "Volume up"
[global_mappings.trigger]
type = "EncoderTurn"
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Use cases**:
- Emergency shutdown
- Mode switching
- Volume control
- System-wide shortcuts (screenshot, lock screen)

**Priority**: Global mappings are checked after mode-specific mappings. If a mode-specific mapping matches the same trigger, it takes precedence.

## Triggers and Actions

Every mapping consists of a **trigger** (what event to detect) and an **action** (what to do when triggered).

### Common Trigger Types

```toml
# Basic note on/off
[trigger]
type = "Note"
note = 12

# Velocity-sensitive
[trigger]
type = "VelocityRange"
note = 12
min_velocity = 0
max_velocity = 40

# Long press
[trigger]
type = "LongPress"
note = 12
hold_duration_ms = 2000

# Double-tap
[trigger]
type = "DoubleTap"
note = 12
double_tap_timeout_ms = 300

# Chord (multiple notes together)
[trigger]
type = "NoteChord"
notes = [12, 13, 14]
chord_timeout_ms = 100

# Encoder rotation
[trigger]
type = "EncoderTurn"
direction = "Clockwise"  # or "CounterClockwise"

# Control change
[trigger]
type = "CC"
cc_number = 1
```

See [Triggers Reference](../reference/triggers.md) for complete list.

### Common Action Types

```toml
# Keyboard shortcut
[action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

# Type text
[action]
type = "Text"
text = "Hello, world!"

# Open application
[action]
type = "Launch"
app = "Terminal"

# Run shell command
[action]
type = "Shell"
command = "cargo test"

# Volume control
[action]
type = "VolumeControl"
operation = "Up"  # Up, Down, Mute, Set

# Change mode
[action]
type = "ModeChange"
mode = "next"  # next, previous, or mode name

# Sequence of actions
[action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Spotify" },
    { type = "Delay", duration_ms = 1000 },
    { type = "Keystroke", keys = "space", modifiers = [] }
]
```

See [Actions Reference](../reference/actions.md) for complete list.

## Minimal Configuration Example

The simplest possible working config:

```toml
[[modes]]
name = "Default"

[[modes.mappings]]
description = "Test"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []
```

**What this does**:
- Creates one mode called "Default"
- Pressing pad 12 types "A"

## Full Configuration Example

A complete, production-ready config:

```toml
# Device configuration
[device]
name = "Mikro"
auto_connect = true

# Advanced timing settings
[advanced_settings]
chord_timeout_ms = 100
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

# Mode 1: General productivity
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
description = "Switch app"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Keystroke"
keys = "tab"
modifiers = ["cmd"]

# Mode 2: Development
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

# Mode 3: Media control
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

# Global mappings (work in all modes)
[[global_mappings]]
description = "Emergency exit (hold 3 seconds)"
[global_mappings.trigger]
type = "LongPress"
note = 0
hold_duration_ms = 3000
[global_mappings.action]
type = "Shell"
command = "killall conductor"

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

[[global_mappings]]
description = "Volume up"
[global_mappings.trigger]
type = "Note"
note = 15
[global_mappings.action]
type = "VolumeControl"
operation = "Up"

[[global_mappings]]
description = "Volume down"
[global_mappings.trigger]
type = "Note"
note = 11
[global_mappings.action]
type = "VolumeControl"
operation = "Down"
```

## Configuration Validation

### Loading Process

When Conductor starts, it:

1. **Locates** `config.toml`
2. **Parses** TOML syntax
3. **Validates** structure and values
4. **Compiles** triggers and actions
5. **Initializes** mapping engine

### Common Validation Errors

#### Syntax Error

**Error**:
```
TOML parse error: expected `=`, but found `:`
```

**Cause**: Invalid TOML syntax (e.g., using `:` instead of `=`)

**Fix**: Check TOML syntax rules, ensure proper formatting

---

#### Missing Required Field

**Error**:
```
Missing field `type` in trigger definition
```

**Cause**: Trigger or action missing required `type` field

**Fix**: Add required fields:
```toml
[trigger]
type = "Note"  # Required
note = 12      # Required for Note trigger
```

---

#### Invalid Field Value

**Error**:
```
Invalid trigger type: 'NotePress' (expected: Note, VelocityRange, LongPress, ...)
```

**Cause**: Typo or unsupported value

**Fix**: Check spelling, refer to [Triggers Reference](../reference/triggers.md)

---

#### Mode Name Conflict

**Error**:
```
Duplicate mode name: 'Default'
```

**Cause**: Two modes with the same name

**Fix**: Ensure each mode has a unique name

---

#### No Mappings Defined

**Warning** (not an error, but Conductor won't do anything):
```
Warning: No mappings defined. Controller won't trigger any actions.
```

**Fix**: Add at least one mapping in a mode or global_mappings

### Validation Checklist

Before running Conductor, verify:

- [ ] TOML syntax is valid (use `cargo check` or online TOML validator)
- [ ] At least one `[[modes]]` section exists
- [ ] Each mode has a unique `name`
- [ ] At least one mapping exists (in modes or global)
- [ ] Every mapping has both `trigger` and `action`
- [ ] All trigger/action types are spelled correctly
- [ ] Note numbers match your device (use `pad_mapper` to verify)
- [ ] File is saved as UTF-8 (not UTF-16 or other encoding)

## Configuration Tips and Best Practices

### 1. Use Comments Liberally

```toml
# ===================================
# MODE 1: General Productivity
# ===================================
[[modes]]
name = "Default"
color = "blue"

# Pad layout:
# 12=Copy  13=Paste  14=Undo  15=Redo
# 8=Save   9=Open    10=Find  11=App Switch
# 4=Cut    5=Select  6=Delete 7=Screenshot
# 0=Escape 1=Enter   2=Tab    3=Space

[[modes.mappings]]
description = "Copy (Pad 12)"
# ... trigger and action ...
```

### 2. Group Related Mappings

```toml
# Clipboard operations
[[modes.mappings]]
description = "Copy"
# ... copy mapping ...

[[modes.mappings]]
description = "Cut"
# ... cut mapping ...

[[modes.mappings]]
description = "Paste"
# ... paste mapping ...

# Navigation
[[modes.mappings]]
description = "Next tab"
# ... next tab ...
```

### 3. Use Descriptive Names

```toml
# Good: Clear and specific
[[modes]]
name = "Development-Rust"

# Bad: Ambiguous
[[modes]]
name = "Mode2"
```

### 4. Test Incrementally

When building a complex config:
1. Start with one mode and one mapping
2. Test it works
3. Add more mappings one at a time
4. Test after each addition

### 5. Keep a Backup

```bash
# Before major changes
cp config.toml config.toml.backup

# Or use version control
git add config.toml
git commit -m "Add media mode mappings"
```

### 6. Use Variables (for consistency)

While TOML doesn't have variables, you can use comments to document repeated values:

```toml
# Standard velocity ranges (document once, use everywhere):
# Soft: 0-40
# Medium: 41-80
# Hard: 81-127

[[modes.mappings]]
description = "Soft press"
[modes.mappings.trigger]
type = "VelocityRange"
note = 12
min_velocity = 0    # Soft range
max_velocity = 40
# ... action ...

[[modes.mappings]]
description = "Hard press"
[modes.mappings.trigger]
type = "VelocityRange"
note = 12
min_velocity = 81   # Hard range
max_velocity = 127
# ... action ...
```

## Configuration Hot-Reloading (Future Feature)

**Note**: Config hot-reloading is not yet implemented in the current version. To reload config changes:

```bash
# Current workaround: Restart Conductor
killall conductor
./target/release/conductor 2
```

**Planned feature** (Phase 2):
- Watch `config.toml` for changes
- Automatically reload without restarting
- Validate before applying (no downtime on errors)

## See Also

- [Modes Guide](../getting-started/modes.md) - Deep dive into modes
- [Triggers Reference](../reference/triggers.md) - All trigger types
- [Actions Reference](../reference/actions.md) - All action types
- [First Mapping Tutorial](../getting-started/first-mapping.md) - Step-by-step guide
- [TOML Specification](https://toml.io/) - Official TOML documentation

## Troubleshooting

### Config Not Loading

**Problem**: Conductor says "Config file not found"

**Solutions**:
```bash
# Check file exists
ls -la config.toml

# Check current directory
pwd

# Specify full path
./conductor --config /full/path/to/config.toml
```

---

### Syntax Errors

**Problem**: "TOML parse error"

**Solutions**:
1. Validate TOML syntax online: https://www.toml-lint.com/
2. Check quotes, brackets, and indentation
3. Look for typos in section names

---

### Mappings Not Working

**Problem**: Config loads but pads don't trigger actions

**Debug steps**:
```bash
# 1. Verify note numbers
cargo run --bin pad_mapper

# 2. Enable debug logging
DEBUG=1 cargo run --release 2

# 3. Check MIDI events
cargo run --bin midi_diagnostic 2
```

See [Common Issues](../troubleshooting/common-issues.md) for more troubleshooting.

---

**Last Updated**: November 11, 2025
**Config Version**: 0.1 (current implementation)
