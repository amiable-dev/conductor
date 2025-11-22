# Video Editing Workflows

Unique controller configurations for DaVinci Resolve, Final Cut Pro, and Adobe Premiere using racing wheels, MIDI controllers, and gamepads.

## Overview

Conductor enables creative, ergonomic video editing workflows that traditional keyboard shortcuts can't match. Use racing wheel pedals for analog timeline control, MIDI pads for quick cuts, and gamepads for timeline navigation.

**What You'll Learn**:
- Racing wheel pedals for timeline speed & zoom (analog control!)
- MIDI pads for markers, cuts, and effects
- Gamepad navigation for hands-free editing
- Hybrid setups combining multiple controllers

---

## Racing Wheel for Video Editing

The most unique and ergonomic editing workflow: use racing wheel pedals for analog timeline control.

### Hardware
- **Racing Wheel**: Logitech G29, G920, Thrustmaster T150 ($150-300)
- **Pedals**: Gas, Brake, Clutch (included with wheel)
- **Software**: DaVinci Resolve, Final Cut Pro, Premiere Pro

### Why This Works
> "Analog pedal control for playback speed is game-changing. I can smoothly ramp from 10% to 200% playback speed, which is impossible with keyboard shortcuts. Plus, it's ergonomic—my feet are doing work my hands don't have to."

### Configuration

```toml
[device]
name = "Racing Wheel Editor"
auto_connect = true

[[modes]]
name = "Editing"
color = "orange"

# ========== Pedals (Analog Control) ==========
# Gas Pedal: Timeline Playback Speed (0-200%)
[[modes.mappings]]
description = "Gas Pedal: Timeline Speed Control"
[modes.mappings.trigger]
type = "GamepadAxis"
axis = 133  # Gas pedal (R2/RT axis)
threshold = 10
[modes.mappings.action]
type = "Keystroke"
keys = "l"  # Increase playback speed
# Note: Pressure determines speed (0-200%)

# Brake Pedal: Zoom Level (Analog)
[[modes.mappings]]
description = "Brake Pedal: Timeline Zoom"
[modes.mappings.trigger]
type = "GamepadAxis"
axis = 132  # Brake pedal (L2/LT axis)
threshold = 10
[modes.mappings.action]
type = "Keystroke"
keys = "="
modifiers = ["cmd"]  # Zoom in

# Clutch Pedal: Master Volume
[[modes.mappings]]
description = "Clutch: Volume Control"
[modes.mappings.trigger]
type = "GamepadAxis"
axis = 131  # Clutch axis
threshold = 10
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"

# ========== Wheel Rotation ==========
# Wheel Turn: Scrub Timeline
[[modes.mappings]]
description = "Wheel Left: Frame Backward"
[modes.mappings.trigger]
type = "GamepadAxis"
axis = 128  # Steering wheel X-axis
threshold = -20
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"

[[modes.mappings]]
description = "Wheel Right: Frame Forward"
[modes.mappings.trigger]
type = "GamepadAxis"
axis = 128
threshold = 20
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"

# ========== Wheel Buttons ==========
# Face Buttons: Common Edits
[[modes.mappings]]
description = "Button 1: Mark In"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "i"

[[modes.mappings]]
description = "Button 2: Mark Out"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "o"

[[modes.mappings]]
description = "Button 3: Cut"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "b"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Button 4: Ripple Delete"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Keystroke"
keys = "Delete"
modifiers = ["cmd", "shift"]

# Paddle Shifters: Previous/Next Edit Point
[[modes.mappings]]
description = "Left Paddle: Previous Edit"
[modes.mappings.trigger]
type = "GamepadButton"
button = 144  # Left paddle
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

[[modes.mappings]]
description = "Right Paddle: Next Edit"
[modes.mappings.trigger]
type = "GamepadButton"
button = 145  # Right paddle
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"

# D-Pad: Track Navigation
[[modes.mappings]]
description = "D-Pad Up: Track Up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
modifiers = ["shift"]

[[modes.mappings]]
description = "D-Pad Down: Track Down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"
modifiers = ["shift"]
```

---

## DaVinci Resolve Configuration

Optimized for DaVinci Resolve's edit, color, and Fairlight pages.

### Edit Page

```toml
[[modes]]
name = "Resolve Edit"
color = "red"

# Quick Tools
[[modes.mappings]]
description = "A: Select Tool"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "a"

[[modes.mappings]]
description = "B: Blade Tool"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "b"

[[modes.mappings]]
description = "X: Trim Tool"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "t"

# Markers
[[modes.mappings]]
description = "Y: Add Marker"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Keystroke"
keys = "m"
```

### Color Page

```toml
[[modes]]
name = "Resolve Color"
color = "purple"

# Color Wheels
[[modes.mappings]]
description = "Encoder 1: Lift Brightness"
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 1
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
modifiers = ["ctrl"]
```

---

## Final Cut Pro Configuration

```toml
[[modes]]
name = "Final Cut Pro"
color = "blue"

# Timeline
[[modes.mappings]]
description = "A: Play/Pause"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

# Effects
[[modes.mappings]]
description = "B: Effects Browser"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "5"
modifiers = ["cmd"]

# Export
[[modes.mappings]]
description = "LB+RB: Export"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "e"
modifiers = ["cmd"]
```

---

## Adobe Premiere Pro Configuration

```toml
[[modes]]
name = "Premiere Pro"
color = "violet"

# Essential Graphics
[[modes.mappings]]
description = "X: Essential Graphics"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "9"
modifiers = ["shift"]

# Lumetri Color
[[modes.mappings]]
description = "Y: Lumetri Color"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Keystroke"
keys = "5"
modifiers = ["shift"]
```

---

## MIDI Controller Setup (Launchpad/APC)

Use MIDI pads for quick access to effects, transitions, and markers.

```toml
# Effects Grid (8x8 Pads)
[[modes.mappings]]
description = "Pad 1: Gaussian Blur"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Sequence"
actions = [
    { Keystroke = { keys = "5", modifiers = ["cmd"] } },  # Open Effects
    { Delay = { duration_ms = 100 } },
    { Text = { text = "Gaussian Blur" } },
    { Keystroke = { keys = "Return" } }
]
```

---

## Hybrid Setup: Wheel + MIDI + Gamepad

Combine racing wheel pedals (timeline), MIDI pads (effects), and gamepad (navigation).

```toml
[device]
name = "Ultimate Editing Rig"

# Wheel Pedals (Analog)
[[modes.mappings]]
trigger = { GamepadAxis = { axis = 133, threshold = 10 } }
action = { Keystroke = { keys = "l" } }  # Playback speed

# MIDI Pads (Effects)
[[modes.mappings]]
trigger = { Note = { note = 36 } }
action = { Keystroke = { keys = "b", modifiers = ["cmd"] } }  # Blade

# Gamepad (Navigation)
[[modes.mappings]]
trigger = { GamepadButton = { button = 132 } }
action = { Keystroke = { keys = "UpArrow" } }
```

---

## Troubleshooting

### Pedal Input Not Recognized
- **Problem**: Pedals don't trigger actions
- **Solution**: Check axis IDs with `cargo run --bin midi_diagnostic`
- **Calibrate**: Ensure pedals are calibrated in OS settings

### Timeline Scrubbing Too Sensitive
- **Problem**: Wheel rotation causes too much movement
- **Solution**: Increase axis threshold (try 30-40 instead of 20)

### Effects Not Applying
- **Problem**: Effect shortcuts open panel but don't apply
- **Solution**: Add delays between keystrokes in Sequence (100-200ms)

---

## Next Steps

- **[See Success Stories](../inspiration/success-stories.md)** - Chris's racing wheel workflow
- **[Explore Gallery](../inspiration/gallery.md)** - Visual setup examples
- **[Learn Analog Axes](../configuration/triggers.md#gamepadaxis)** - Pedal control
- **[Join Video Editing Community](../resources/community.md)** - Share your creative setup
