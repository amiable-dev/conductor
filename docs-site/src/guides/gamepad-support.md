# Gamepad Support Guide

**Version**: 3.0
**Status**: Stable
**Platforms**: macOS, Linux, Windows

## Overview

Conductor v3.0 introduces full support for gamepad controllers, allowing you to use Xbox, PlayStation, Nintendo Switch Pro, and other SDL-compatible gamepads as macro input devices alongside MIDI controllers.

## Quick Start

### 1. Connect Your Gamepad

1. Connect your gamepad via USB or Bluetooth
2. Ensure it's recognized by your system
3. Conductor will automatically detect compatible gamepads

**Supported Controllers**:
- Xbox 360, Xbox One, Xbox Series X|S
- PlayStation DualShock 4, DualSense (PS5)
- Nintendo Switch Pro Controller
- Any SDL2-compatible gamepad

### 2. Choose a Template

The quickest way to get started is using a pre-configured template:

**Via GUI** (Recommended):
1. Open Conductor GUI
2. Navigate to "Device Templates"
3. Filter by "Gamepad Controllers"
4. Select your controller (Xbox, PlayStation, or Switch Pro)
5. Click "Create Config"

**Via Configuration File**:
Copy one of the example configs from `config/examples/`:
- `gamepad-xbox-basic.toml` - Xbox controller template
- Or use the included device templates

### 3. Start Using

```bash
# Start Conductor daemon
conductor --foreground

# Your gamepad is now ready to trigger actions!
```

## Configuration Reference

### Basic Setup

```toml
[device]
name = "Gamepad"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50          # Multi-button chord detection window
double_tap_timeout_ms = 300    # Double-tap detection window
hold_threshold_ms = 2000       # Long press threshold
```

### Trigger Types

#### 1. GamepadButton

Simple button press trigger.

```toml
[[modes.mappings]]
description = "A button: Confirm"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A button (Xbox) / Cross (PS) / B (Switch)
[modes.mappings.action]
type = "Keystroke"
keys = "Return"
```

**Optional fields**:
- `velocity_min`: Minimum pressure (0-255)

#### 2. GamepadButtonChord

Multiple buttons pressed simultaneously.

```toml
[[modes.mappings]]
description = "A+B: Screenshot"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]  # A + B buttons
timeout_ms = 50       # Buttons must be pressed within 50ms
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]
```

#### 3. GamepadAnalogStick

Analog stick movement detection.

```toml
[[modes.mappings]]
description = "Right stick right: Forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130              # Right stick X-axis
direction = "Clockwise" # Moving right
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]
```

**Stick Axes**:
- `128`: Left stick X-axis
- `129`: Left stick Y-axis
- `130`: Right stick X-axis
- `131`: Right stick Y-axis

**Directions**:
- `"Clockwise"`: Right (X-axis) or Up (Y-axis)
- `"CounterClockwise"`: Left (X-axis) or Down (Y-axis)

**Dead Zone**: Automatic 10% dead zone prevents false triggers

#### 4. GamepadTrigger

Analog trigger threshold detection (L2/R2, LT/RT, ZL/ZR).

```toml
[[modes.mappings]]
description = "Right trigger: Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133   # Right trigger
threshold = 128 # Half-pull (0-255 range)
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Triggers**:
- `132`: Left trigger (L2, LT, ZL)
- `133`: Right trigger (R2, RT, ZR)

## Button ID Reference

### Standard Gamepad Layout

Conductor uses a unified button ID scheme across all gamepads:

#### Face Buttons (128-131)

| ID  | Xbox | PlayStation | Switch |
|-----|------|-------------|--------|
| 128 | A (South) | Cross | B |
| 129 | B (East) | Circle | A |
| 130 | X (West) | Square | Y |
| 131 | Y (North) | Triangle | X |

#### D-Pad (132-135)

| ID  | Button |
|-----|--------|
| 132 | Up |
| 133 | Down |
| 134 | Left |
| 135 | Right |

#### Shoulder Buttons (136-137)

| ID  | Xbox | PlayStation | Switch |
|-----|------|-------------|--------|
| 136 | LB (L1) | L1 | L |
| 137 | RB (R1) | R1 | R |

#### Stick Clicks (138-139)

| ID  | Button |
|-----|--------|
| 138 | Left stick click (L3) |
| 139 | Right stick click (R3) |

#### Menu Buttons (140-142)

| ID  | Xbox | PlayStation | Switch |
|-----|------|-------------|--------|
| 140 | Menu (Start) | Options | + (Plus) |
| 141 | View (Select) | Share/Create | - (Minus) |
| 142 | Xbox button | PS button | Home |

#### Trigger Buttons Digital (143-144)

| ID  | Xbox | PlayStation | Switch |
|-----|------|-------------|--------|
| 143 | LT (digital) | L2 (digital) | ZL |
| 144 | RT (digital) | R2 (digital) | ZR |

### Analog Axes

#### Stick Axes (128-131)

| ID  | Control |
|-----|---------|
| 128 | Left stick X-axis |
| 129 | Left stick Y-axis |
| 130 | Right stick X-axis |
| 131 | Right stick Y-axis |

#### Trigger Axes (132-133)

| ID  | Control |
|-----|---------|
| 132 | Left trigger analog |
| 133 | Right trigger analog |

**Value Range**: 0-255 (128 = center for sticks)

## Advanced Features

### Pattern Detection

Conductor automatically detects advanced button patterns:

#### Double-Tap
Press the same button twice within 300ms.

```toml
# Automatically detected by MIDI Learn
# Manual configuration:
[modes.mappings.trigger]
type = "DoubleTap"
note = 128  # Button ID (reuses Note type)
timeout_ms = 300
```

#### Long Press
Hold a button for 2+ seconds.

```toml
# Automatically detected by MIDI Learn
# Manual configuration:
[modes.mappings.trigger]
type = "LongPress"
note = 128  # Button ID (reuses Note type)
duration_ms = 2000
```

### MIDI Learn Mode

MIDI Learn now supports gamepad inputs:

**Via GUI**:
1. Open Conductor GUI
2. Click "Learn" next to any mapping
3. Press a button or move a stick on your gamepad
4. Conductor automatically generates the correct trigger config

**Pattern Detection**:
- Press button once → GamepadButton
- Press button twice quickly → DoubleTap
- Hold button → LongPress
- Press multiple buttons → GamepadButtonChord
- Move analog stick → GamepadAnalogStick
- Pull trigger → GamepadTrigger

### Mode Switching

Use button chords for mode switching:

```toml
[[global_mappings]]
description = "LB+RB: Switch to Media mode"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]  # LB + RB
timeout_ms = 50
[global_mappings.action]
type = "ModeChange"
mode = "Media"
```

### Hybrid MIDI + Gamepad Setup

You can use MIDI controllers and gamepads simultaneously:

```toml
[device]
name = "Maschine Mikro MK3"  # MIDI device name
auto_connect = true

# Gamepad is automatically detected
# No additional configuration needed!

# MIDI mappings (button IDs 0-127)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 36  # MIDI note

# Gamepad mappings (button IDs 128-255)
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad button
```

**Key Points**:
- MIDI uses IDs 0-127
- Gamepad uses IDs 128-255
- No conflicts, works seamlessly together

## Common Use Cases

### Desktop Navigation

```toml
[[modes]]
name = "Desktop"
color = "blue"

# Arrow keys on D-Pad
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 132  # D-Pad Up
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# Copy/Paste on face buttons
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 130  # X button
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]
```

### Media Control

```toml
[[modes]]
name = "Media"
color = "purple"

# Play/Pause
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A button
[modes.mappings.action]
type = "Keystroke"
keys = "PlayPause"

# Volume control with triggers
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger
threshold = 64
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

### Browser Control

```toml
# Navigate with right stick
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]  # Forward in browser

# Scroll with right stick Y
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 131  # Right stick Y
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"  # Scroll down
```

## Troubleshooting

### Gamepad Not Detected

**Check connection**:
```bash
# List connected gamepads
conductorctl status

# Check system recognition
# macOS: System Settings > Game Controllers
# Linux: ls /dev/input/js*
# Windows: Devices and Printers
```

**Solutions**:
1. Reconnect the gamepad
2. Try USB instead of Bluetooth (or vice versa)
3. Ensure drivers are installed (Windows)
4. Check SDL2 compatibility

### Buttons Not Working

**Verify button mapping**:
1. Use MIDI Learn to discover the actual button ID
2. Check that button IDs are in the 128-255 range
3. Ensure no conflicting mappings exist

**Common mistakes**:
- Using MIDI note IDs (0-127) instead of gamepad IDs (128-255)
- Incorrect axis ID for analog sticks
- Dead zone preventing stick triggers

### Analog Stick Too Sensitive

Adjust the dead zone threshold by using button triggers instead:

```toml
# Instead of analog stick trigger
# Use button-based threshold
[modes.mappings.trigger]
type = "GamepadButton"
button = 134  # D-Pad left instead of stick
```

### Trigger Not Firing

**Check threshold**:
- Threshold too high: Lower the value (try 32, 64, 128)
- Threshold too low: Increase to prevent false triggers

```toml
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64  # Start with 25% pull
```

### Button Chords Not Detected

**Adjust chord timeout**:
```toml
[advanced_settings]
chord_timeout_ms = 100  # Increase from 50ms
```

**Tips**:
- Press buttons as simultaneously as possible
- Practice the timing
- Use MIDI Learn to test detection

## Performance Notes

### Latency
- **Input latency**: <1ms (1000Hz polling)
- **Event processing**: <1ms
- **Total latency**: ~2-5ms (comparable to MIDI)

### Resource Usage
- **CPU**: <1% (1ms polling intervals)
- **Memory**: ~5-10MB additional
- **Battery**: Minimal impact on wireless controllers

### Compatibility
- **gilrs v0.10**: Industry-standard gamepad library
- **SDL2 mappings**: Supports 100+ controller types
- **Auto-detection**: Works with most modern gamepads

## Best Practices

1. **Start with templates**: Use official Xbox/PS/Switch templates
2. **Use MIDI Learn**: Let pattern detection configure triggers
3. **Test incrementally**: Add mappings one at a time
4. **Document custom configs**: Add descriptions to mappings
5. **Use global mappings**: Mode switches work everywhere
6. **Backup configs**: Save working configurations

## Examples

Complete examples available in:
- `config/examples/gamepad-xbox-basic.toml`
- Device templates in Conductor GUI
- See [Configuration Examples](../configuration/examples.md)

## Further Reading

- [Configuration Overview](../configuration/overview.md)
- [Trigger Types](../configuration/triggers.md)
- [Action Types](../configuration/actions.md)
- [MIDI Learn Guide](./midi-learn.md)
- [Troubleshooting](../troubleshooting/common-issues.md)

---

**Need Help?**

- GitHub Issues: https://github.com/amiable/conductor/issues
- Documentation: https://conductor.dev/docs
- Examples: `config/examples/`
