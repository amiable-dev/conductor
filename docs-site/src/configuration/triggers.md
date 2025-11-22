# Triggers Reference

Triggers define the input events that activate mappings in Conductor. This page provides comprehensive documentation for all supported trigger types across MIDI controllers and game controllers.

## Overview

Conductor supports two primary input protocols:

- **MIDI Controllers** (v1.0+): MIDI keyboards, pad controllers, encoders, and touch strips
- **Game Controllers (HID)** (v3.0+): Gamepads, joysticks, racing wheels, flight sticks, HOTAS, arcade controllers, and custom SDL2-compatible HID devices

Both protocols share the same unified configuration format and can be used simultaneously in hybrid setups.

### ID Range Allocation

To prevent conflicts between MIDI and HID devices, Conductor uses distinct ID ranges:

| Range | Protocol | Used For | Examples |
|-------|----------|----------|----------|
| **0-127** | MIDI | Notes, CC, Encoders | MIDI note C4=60, CC Mod Wheel=1 |
| **128-255** | Game Controllers | Buttons, Axes, Triggers | Gamepad A button=128, Left stick X=128 |

This non-overlapping allocation ensures seamless coexistence of MIDI and gamepad inputs without configuration conflicts.

---

## MIDI Triggers

MIDI triggers respond to events from MIDI controllers such as keyboards, pad controllers, and control surfaces.

### Note

Basic note trigger with optional velocity threshold.

**Use Case**: Trigger actions on specific note presses (e.g., pad hits, keyboard notes).

```toml
[[modes.mappings]]
description = "Pad 1: Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 36          # MIDI note number (0-127)
velocity_min = 1   # Optional: Minimum velocity to trigger

[modes.mappings.action]
type = "Keystroke"
keys = "Space"
```

**Parameters**:
- `note` (required): MIDI note number (0-127)
- `velocity_min` (optional): Minimum velocity to trigger (0-127). Omit to trigger on any velocity.

**Examples**:
```toml
# Trigger on any velocity
note = 60  # Middle C

# Trigger only on hard hits (velocity > 80)
note = 36
velocity_min = 80
```

---

### VelocityRange

Velocity-sensitive trigger that classifies note presses into soft, medium, and hard levels.

**Use Case**: Execute different actions based on how hard a pad or key is pressed.

```toml
[[modes.mappings]]
description = "Velocity-sensitive pad"
[modes.mappings.trigger]
type = "VelocityRange"
note = 36
soft_max = 40      # Optional: Max velocity for soft (default: 40)
medium_max = 80    # Optional: Max velocity for medium (default: 80)

[modes.mappings.action]
type = "Conditional"
condition = { type = "VelocityLevel", level = "Hard" }
then_action = { type = "Keystroke", keys = "F1" }
else_action = { type = "Keystroke", keys = "Space" }
```

**Velocity Classification**:
- **Soft**: 0 to `soft_max` (default: 0-40)
- **Medium**: `soft_max` + 1 to `medium_max` (default: 41-80)
- **Hard**: `medium_max` + 1 to 127 (default: 81-127)

**Parameters**:
- `note` (required): MIDI note number (0-127)
- `soft_max` (optional): Maximum velocity for soft (default: 40)
- `medium_max` (optional): Maximum velocity for medium (default: 80)

---

### LongPress

Triggers when a note is held for longer than a specified duration.

**Use Case**: Hold a pad/key for extended actions (e.g., hold for shutdown, tap for play).

```toml
[[modes.mappings]]
description = "Hold pad for 2 seconds to shutdown"
[modes.mappings.trigger]
type = "LongPress"
note = 40
duration_ms = 2000  # Optional: Hold duration in milliseconds (default: 2000)

[modes.mappings.action]
type = "Shell"
command = "shutdown -h now"
```

**Parameters**:
- `note` (required): MIDI note number (0-127)
- `duration_ms` (optional): Hold duration in milliseconds (default: 2000)

**Timing**:
- Default threshold: 2000ms (2 seconds)
- Configurable globally via `advanced_settings.hold_threshold_ms`

---

### DoubleTap

Triggers when a note is pressed and released twice within a time window.

**Use Case**: Double-tap a pad for quick actions (e.g., skip track, open app).

```toml
[[modes.mappings]]
description = "Double-tap pad to skip track"
[modes.mappings.trigger]
type = "DoubleTap"
note = 48
timeout_ms = 300  # Optional: Time window for double-tap (default: 300)

[modes.mappings.action]
type = "Keystroke"
keys = "Next"
```

**Parameters**:
- `note` (required): MIDI note number (0-127)
- `timeout_ms` (optional): Time window in milliseconds for detecting double-tap (default: 300)

**Timing**:
- Default window: 300ms
- Configurable globally via `advanced_settings.double_tap_timeout_ms`

---

### NoteChord

Triggers when multiple notes are pressed simultaneously (within a narrow time window).

**Use Case**: Combine multiple pads/keys for complex actions (e.g., emergency exit, mode switch).

```toml
[[modes.mappings]]
description = "Pads 1+2+3: Emergency exit"
[modes.mappings.trigger]
type = "NoteChord"
notes = [36, 37, 38]  # List of MIDI notes
timeout_ms = 50       # Optional: Simultaneous press window (default: 50)

[modes.mappings.action]
type = "Shell"
command = "killall conductor"
```

**Parameters**:
- `notes` (required): Array of MIDI note numbers (0-127)
- `timeout_ms` (optional): Time window in milliseconds for simultaneous presses (default: 50)

**Timing**:
- Default window: 50ms
- Configurable globally via `advanced_settings.chord_timeout_ms`

---

### EncoderTurn

Triggers on encoder/knob rotation (MIDI CC messages).

**Use Case**: Respond to encoder turns for volume, scrolling, or mode switching.

```toml
[[modes.mappings]]
description = "Encoder clockwise: Volume up"
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1                    # MIDI CC number (0-127)
direction = "Clockwise"   # Optional: "Clockwise", "CounterClockwise", or omit for both

[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Parameters**:
- `cc` (required): Control Change number (0-127)
- `direction` (optional): Filter by direction:
  - `"Clockwise"`: Encoder turned right/up (CC value increasing)
  - `"CounterClockwise"`: Encoder turned left/down (CC value decreasing)
  - Omit: Respond to both directions

**Common CC Numbers**:
- CC 1: Modulation Wheel
- CC 7: Volume
- CC 10: Pan
- CC 74: Brightness/Filter Cutoff

---

### Aftertouch

Triggers based on channel pressure (aftertouch) messages.

**Use Case**: Respond to pressure sensitivity on pads or keys.

```toml
[[modes.mappings]]
description = "Aftertouch: Vibrato"
[modes.mappings.trigger]
type = "Aftertouch"
pressure_min = 64  # Optional: Minimum pressure (0-127)

[modes.mappings.action]
type = "SendMidi"
port = "Virtual Output"
message_type = "CC"
channel = 0
controller = 1
value = 127
```

**Parameters**:
- `pressure_min` (optional): Minimum pressure value to trigger (0-127)

---

### PitchBend

Triggers based on pitch bend messages from touch strips or pitch bend wheels.

**Use Case**: Respond to touch strip movements or pitch wheel changes.

```toml
[[modes.mappings]]
description = "Pitch bend up"
[modes.mappings.trigger]
type = "PitchBend"
value_min = 8192   # Optional: Minimum value (0-16383, 8192 = center)
value_max = 16383  # Optional: Maximum value

[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Parameters**:
- `value_min` (optional): Minimum pitch bend value (0-16383)
- `value_max` (optional): Maximum pitch bend value (0-16383)

**Value Range**:
- 0-8191: Bend down
- 8192: Center (no bend)
- 8193-16383: Bend up

---

### CC (Control Change)

Generic trigger for any MIDI Control Change message.

**Use Case**: Respond to sliders, knobs, buttons, or pedals that send CC messages.

```toml
[[modes.mappings]]
description = "Sustain pedal"
[modes.mappings.trigger]
type = "CC"
cc = 64           # CC number (0-127)
value_min = 64    # Optional: Minimum value to trigger (0-127)

[modes.mappings.action]
type = "SendMidi"
port = "Virtual Output"
message_type = "CC"
channel = 0
controller = 64
value = 127
```

**Parameters**:
- `cc` (required): Control Change number (0-127)
- `value_min` (optional): Minimum value to trigger (0-127)

**Common CC Numbers**:
- CC 1: Modulation Wheel
- CC 7: Volume
- CC 10: Pan
- CC 64: Sustain Pedal
- CC 74: Brightness/Filter Cutoff

---

## Game Controllers (HID) Triggers (v3.0+)

Game Controllers (HID) triggers respond to events from gamepad controllers, joysticks, racing wheels, flight sticks, HOTAS systems, arcade controllers, and any SDL2-compatible HID device.

**Supported Device Types**:
- **Gamepads**: Xbox 360/One/Series X|S, PlayStation DualShock 4/DualSense, Nintendo Switch Pro Controller
- **Joysticks**: Flight sticks, arcade sticks with analog axes and buttons
- **Racing Wheels**: Logitech, Thrustmaster, Fanatec wheels with pedals
- **Flight Sticks**: Thrustmaster T.Flight, Logitech Extreme 3D Pro
- **HOTAS**: Hands On Throttle And Stick systems for flight simulation
- **Arcade Controllers**: Fight sticks, arcade pads
- **Custom Controllers**: Any SDL2-compatible HID device

All gamepad triggers use ID range **128-255** to avoid conflicts with MIDI (0-127).

---

### GamepadButton

Standard button press trigger for face buttons, D-pad, shoulders, triggers, and menu buttons.

**Use Case**: Trigger actions on gamepad button presses (e.g., A button to confirm, D-pad for navigation).

```toml
[[modes.mappings]]
description = "A button: Confirm"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128       # Gamepad button ID (128-255)
velocity_min = 1   # Optional: Minimum pressure (0-127)

[modes.mappings.action]
type = "Keystroke"
keys = "Return"
```

**Parameters**:
- `button` (required): Gamepad button ID (128-255)
- `velocity_min` (optional): Minimum pressure to trigger (0-127). Useful for analog buttons on some controllers.

**Standard Button IDs**:

| ID Range | Button Type | Specific Buttons |
|----------|-------------|------------------|
| **128-131** | Face Buttons | 128=South (A/Cross/B), 129=East (B/Circle/A), 130=West (X/Square/Y), 131=North (Y/Triangle/X) |
| **132-135** | D-Pad | 132=Up, 133=Down, 134=Left, 135=Right |
| **136-137** | Shoulders | 136=L1/LB/L, 137=R1/RB/R |
| **138-139** | Stick Clicks | 138=L3 (left stick click), 139=R3 (right stick click) |
| **140-142** | Menu Buttons | 140=Start, 141=Select/Back/View, 142=Guide/Home/PS |
| **143-144** | Digital Triggers | 143=L2/LT/ZL, 144=R2/RT/ZR (digital press, not analog) |

**Cross-Platform Button Mapping**:

| Button | Xbox | PlayStation | Nintendo Switch |
|--------|------|-------------|-----------------|
| **South (128)** | A | Cross (×) | B |
| **East (129)** | B | Circle (○) | A |
| **West (130)** | X | Square (□) | Y |
| **North (131)** | Y | Triangle (△) | X |
| **L1 (136)** | LB | L1 | L |
| **R1 (137)** | RB | R1 | R |
| **L2 (143)** | LT | L2 | ZL |
| **R2 (144)** | RT | R2 | ZR |

**Examples**:

```toml
# Xbox A button / PlayStation Cross / Switch B
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

# D-Pad Up for navigation
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# Left shoulder (LB/L1/L) for tab switching
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Keystroke"
keys = "Tab"
modifiers = ["cmd", "shift"]

# Xbox Guide button for Spotlight
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 142
[modes.mappings.action]
type = "Keystroke"
keys = "Space"
modifiers = ["cmd"]
```

**Device-Specific Notes**:

- **Racing Wheels**: Buttons on the wheel hub (paddle shifters, D-pad, rotary encoders) map to standard button IDs
- **Flight Sticks**: Hat switches map to D-pad IDs (132-135), trigger button to face button ID
- **HOTAS**: Throttle buttons, base buttons, and hat switches use extended button IDs (145+)
- **Arcade Controllers**: All buttons (typically 6-8 action buttons) map to face button and shoulder IDs

---

### GamepadButtonChord

Multiple gamepad buttons pressed simultaneously (chord detection).

**Use Case**: Combine multiple buttons for complex actions (e.g., LB+RB for mode switch, A+B for screenshot).

```toml
[[modes.mappings]]
description = "LB+RB: Switch mode"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]  # Array of button IDs (128-255)
timeout_ms = 50       # Optional: Simultaneous press window (default: 50)

[modes.mappings.action]
type = "ModeChange"
mode = "Media"
```

**Parameters**:
- `buttons` (required): Array of gamepad button IDs (128-255). All buttons must be pressed within the timeout window.
- `timeout_ms` (optional): Time window in milliseconds for simultaneous presses (default: 50)

**Timing**:
- Default window: 50ms
- Configurable globally via `advanced_settings.chord_timeout_ms`

**Examples**:

```toml
# Two-button chord: A+B for screenshot
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

# Three-button chord: LB+RB+Start for emergency exit
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137, 140]
timeout_ms = 100
[modes.mappings.action]
type = "Shell"
command = "killall conductor"

# Racing wheel: Both paddle shifters for mode change
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]  # Left + Right paddles
[modes.mappings.action]
type = "ModeChange"
mode = "Racing"
```

---

### GamepadAnalogStick

Analog stick movement detection with directional filtering.

**Use Case**: Respond to analog stick movements for scrolling, navigation, or camera control.

```toml
[[modes.mappings]]
description = "Right stick right: Forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130              # Stick axis ID (128-131)
direction = "Clockwise" # Optional: "Clockwise" (right/up) or "CounterClockwise" (left/down)

[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]
```

**Parameters**:
- `axis` (required): Analog stick axis ID (128-131)
- `direction` (optional): Filter by direction:
  - `"Clockwise"`: Right (X-axis) or Up (Y-axis)
  - `"CounterClockwise"`: Left (X-axis) or Down (Y-axis)
  - Omit: Respond to both directions

**Analog Stick Axis IDs**:

| ID | Axis | Description |
|----|------|-------------|
| **128** | Left Stick X | Left stick horizontal (left = CounterClockwise, right = Clockwise) |
| **129** | Left Stick Y | Left stick vertical (down = CounterClockwise, up = Clockwise) |
| **130** | Right Stick X | Right stick horizontal (left = CounterClockwise, right = Clockwise) |
| **131** | Right Stick Y | Right stick vertical (down = CounterClockwise, up = Clockwise) |

**Dead Zone**:
- Automatic 10% dead zone prevents false triggers from stick drift
- Axis values below threshold are ignored

**Value Normalization**:
- Input range: -1.0 to +1.0 (raw analog values)
- Output range: 0-255 (normalized to MIDI-like range)
- Formula: `((value + 1.0) * 127.5) as u8`

**Examples**:

```toml
# Left stick right for next item
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"

# Right stick up for volume up
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 131
direction = "Clockwise"
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Left stick (any direction) for scrolling
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Left stick Y
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
```

**Device-Specific Notes**:

- **Flight Sticks**: Primary stick axes (pitch/roll) typically map to right stick IDs (130-131)
- **HOTAS**: Throttle axis may map to left stick Y (129), rudder to left stick X (128)
- **Racing Wheels**: Steering wheel rotation maps to left stick X (128)
- **Arcade Sticks**: Analog joystick (if present) maps to left stick (128-129)

---

### GamepadTrigger

Analog trigger pull detection with threshold (L2/R2, LT/RT, ZL/ZR).

**Use Case**: Respond to analog trigger pulls for fine-grained control (e.g., volume, acceleration, braking).

```toml
[[modes.mappings]]
description = "Right trigger: Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133   # Trigger ID (132-133)
threshold = 64  # Optional: Minimum pull value (0-127)

[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Parameters**:
- `trigger` (required): Analog trigger ID (132-133)
- `threshold` (optional): Minimum pull value to trigger (0-127). Omit to trigger on any pull.

**Analog Trigger IDs**:

| ID | Trigger | Description |
|----|---------|-------------|
| **132** | Left Trigger | L2 (PlayStation), LT (Xbox), ZL (Switch) |
| **133** | Right Trigger | R2 (PlayStation), RT (Xbox), ZR (Switch) |

**Value Range**:
- 0: Trigger fully released
- 127: Trigger fully pulled
- Typical threshold: 64 (half-pull)

**Examples**:

```toml
# Right trigger half-pull for volume up
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Left trigger full-pull for screenshot
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 100
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

# Right trigger (any pull) for next track
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
[modes.mappings.action]
type = "Keystroke"
keys = "Next"
```

**Device-Specific Notes**:

- **Racing Wheels**: Throttle pedal maps to right trigger (133), brake pedal to left trigger (132)
- **Flight Sticks**: Some flight sticks have analog trigger buttons that map to these IDs
- **HOTAS**: Throttle position may map to right trigger (133)
- **DualSense (PS5)**: Supports adaptive trigger resistance (future feature)

**Threshold Guidelines**:

| Threshold | Use Case | Sensitivity |
|-----------|----------|-------------|
| **0-20** | Feather touch | Very sensitive |
| **40-60** | Light pull | Medium sensitivity |
| **64** | Half-pull | Balanced (recommended) |
| **80-100** | Firm pull | Low sensitivity |
| **110-127** | Full pull | Very low sensitivity |

---

## Hybrid MIDI + Game Controller Mappings

Conductor v3.0 supports using MIDI and gamepad controllers simultaneously. The non-overlapping ID ranges (MIDI: 0-127, Gamepad: 128-255) ensure seamless coexistence.

### Use Cases

- **Live Performance**: MIDI controller for music, gamepad for lighting/effects
- **Production**: MIDI keyboard for notes, gamepad for transport controls
- **Gaming + Music**: Gamepad for game macros, MIDI pads for audio triggers
- **Accessibility**: Use whichever input device is most comfortable

### Example: Hybrid Configuration

```toml
[device]
name = "Hybrid"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Hybrid"
color = "cyan"

# MIDI pad for play/pause
[[modes.mappings]]
description = "MIDI Pad 1: Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 36  # MIDI note (0-127)
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

# Gamepad A button for confirm
[[modes.mappings]]
description = "Gamepad A: Confirm"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad button (128-255)
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

# MIDI encoder for volume
[[modes.mappings]]
description = "MIDI Encoder: Volume"
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Gamepad right stick for scrolling
[[modes.mappings]]
description = "Gamepad Right Stick: Scroll"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 131  # Right stick Y
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# MIDI + Gamepad chord for emergency exit
[[global_mappings]]
description = "MIDI Pad 1 + Gamepad A: Emergency Exit"
[global_mappings.trigger]
type = "NoteChord"
notes = [36, 128]  # Mix MIDI and gamepad IDs in chord
timeout_ms = 100
[global_mappings.action]
type = "Shell"
command = "killall conductor"
```

### Hybrid Configuration Tips

1. **ID Separation**: Always use 0-127 for MIDI, 128-255 for gamepad
2. **Chord Detection**: You can mix MIDI and gamepad IDs in `NoteChord` and `GamepadButtonChord` triggers
3. **Mode Switching**: Use either MIDI or gamepad inputs to switch modes
4. **Global Mappings**: Define device-agnostic global mappings that work across both protocols
5. **Input Mode**: Set `input_mode = "Both"` in `[device]` to enable hybrid mode (automatic in v3.0)

---

## Advanced Trigger Patterns

### Long Press with Gamepad

```toml
[[modes.mappings]]
description = "Hold A button for 2 seconds"
[modes.mappings.trigger]
type = "LongPress"
note = 128  # Gamepad button ID
duration_ms = 2000
[modes.mappings.action]
type = "Shell"
command = "shutdown -h now"
```

### Double-Tap with Gamepad

```toml
[[modes.mappings]]
description = "Double-tap A button"
[modes.mappings.trigger]
type = "DoubleTap"
note = 128  # Gamepad button ID
timeout_ms = 300
[modes.mappings.action]
type = "Keystroke"
keys = "Next"
```

### Velocity-Sensitive Gamepad Button

Some gamepads (e.g., DualShock 4, DualSense) have pressure-sensitive buttons:

```toml
[[modes.mappings]]
description = "Pressure-sensitive X button"
[modes.mappings.trigger]
type = "VelocityRange"
note = 130  # Gamepad X button
soft_max = 40
medium_max = 80
[modes.mappings.action]
type = "Conditional"
condition = { type = "VelocityLevel", level = "Hard" }
then_action = { type = "Keystroke", keys = "F1" }
else_action = { type = "Keystroke", keys = "Space" }
```

---

## ID Reference Tables

### MIDI Note Numbers (0-127)

| Octave | C | C# | D | D# | E | F | F# | G | G# | A | A# | B |
|--------|---|-------|---|-------|---|---|-------|---|-------|---|-------|---|
| **-2** | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
| **-1** | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |
| **0** | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 |
| **1** | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |
| **2** | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 |
| **3** | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 |
| **4** | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 |
| **5** | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 |
| **6** | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 |
| **7** | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 |
| **8** | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | - | - | - | - |

**Common MIDI Note Examples**:
- **36 (C1)**: Typical bass drum / first pad on controllers
- **60 (C3)**: Middle C
- **127 (G8)**: Highest MIDI note

### Gamepad Button IDs (128-255)

| ID | Button | Xbox | PlayStation | Switch | Device Type |
|----|--------|------|-------------|--------|-------------|
| **128** | South | A | Cross (×) | B | Gamepad |
| **129** | East | B | Circle (○) | A | Gamepad |
| **130** | West | X | Square (□) | Y | Gamepad |
| **131** | North | Y | Triangle (△) | X | Gamepad |
| **132** | D-Pad Up | D-Up | D-Up | D-Up | Gamepad |
| **133** | D-Pad Down | D-Down | D-Down | D-Down | Gamepad |
| **134** | D-Pad Left | D-Left | D-Left | D-Left | Gamepad |
| **135** | D-Pad Right | D-Right | D-Right | D-Right | Gamepad |
| **136** | Left Shoulder | LB | L1 | L | Gamepad |
| **137** | Right Shoulder | RB | R1 | R | Gamepad |
| **138** | Left Stick Click | L3 | L3 | L-Stick | Gamepad |
| **139** | Right Stick Click | R3 | R3 | R-Stick | Gamepad |
| **140** | Start | Menu | Options | + | Gamepad |
| **141** | Select | View | Share | - | Gamepad |
| **142** | Guide | Xbox | PS | Home | Gamepad |
| **143** | Left Trigger | LT | L2 | ZL | Gamepad (digital) |
| **144** | Right Trigger | RT | R2 | ZR | Gamepad (digital) |

### Gamepad Axis IDs (128-133)

| ID | Axis | Description | Device Type |
|----|------|-------------|-------------|
| **128** | Left Stick X | Horizontal (left/right) | Gamepad, Racing Wheel |
| **129** | Left Stick Y | Vertical (up/down) | Gamepad, HOTAS Throttle |
| **130** | Right Stick X | Horizontal (left/right) | Gamepad, Flight Stick |
| **131** | Right Stick Y | Vertical (up/down) | Gamepad, Flight Stick |
| **132** | Left Trigger | Analog (0-127) | Gamepad, Racing Wheel (brake) |
| **133** | Right Trigger | Analog (0-127) | Gamepad, Racing Wheel (throttle) |

### Device-Specific Mappings

**Racing Wheels**:
- **128 (Left Stick X)**: Steering wheel rotation (left = CounterClockwise, right = Clockwise)
- **132 (Left Trigger)**: Brake pedal (0-127)
- **133 (Right Trigger)**: Throttle/gas pedal (0-127)
- **136-137**: Paddle shifters (left/right)
- **128-144**: Wheel buttons (D-pad, face buttons on wheel hub)

**Flight Sticks**:
- **130 (Right Stick X)**: Stick roll (left/right)
- **131 (Right Stick Y)**: Stick pitch (forward/back)
- **132-135**: Hat switch (maps to D-pad IDs)
- **128-131**: Primary action buttons (trigger, thumb buttons)

**HOTAS (Hands On Throttle And Stick)**:
- **129 (Left Stick Y)**: Throttle axis
- **128 (Left Stick X)**: Rudder/twist axis
- **130-131**: Stick axes (pitch/roll)
- **132-135**: Hat switches
- **136-144**: Base buttons, throttle buttons

**Arcade Sticks**:
- **128-131**: 4-way/8-way joystick (if analog)
- **132-135**: D-pad (if digital joystick)
- **128-137**: Action buttons (6-8 button layout)

---

## Configuration Tips

### 1. Finding Button IDs

Use the MIDI Learn feature (v3.0) to automatically detect button IDs:

1. Open Conductor GUI
2. Navigate to a mapping
3. Click "MIDI Learn"
4. Press the desired button on your gamepad
5. The button ID will be auto-filled

### 2. Testing Triggers

Use the live event console in Conductor GUI to see real-time trigger events:

1. Open Conductor GUI
2. Navigate to "Event Console"
3. Press buttons/move sticks on your gamepad
4. Observe the event type and ID
5. Use this information to configure triggers

### 3. Device Templates

Conductor includes pre-configured templates for popular devices:

- **Xbox Controller**: `xbox-controller.toml`
- **PlayStation Controller**: `playstation-controller.toml`
- **Switch Pro Controller**: `switch-pro-controller.toml`

Load a template via GUI:
1. Open Conductor GUI
2. Navigate to "Device Templates"
3. Filter by device type
4. Select template and click "Create Config"

### 4. Global vs Mode Mappings

- **Global Mappings**: Work across all modes (e.g., emergency exit, mode switching)
- **Mode Mappings**: Active only in specific modes (e.g., media controls in "Media" mode)

**Best Practice**: Use global mappings for critical actions (exit, mode switch) and mode mappings for context-specific actions.

### 5. Timing Configuration

Fine-tune timing thresholds in `[advanced_settings]`:

```toml
[advanced_settings]
chord_timeout_ms = 50       # Multi-button chord detection
double_tap_timeout_ms = 300 # Double-tap detection
hold_threshold_ms = 2000    # Long press threshold
```

**Recommendations**:
- **Chord**: 50ms (default) works well for most controllers
- **Double-Tap**: 300ms (default) balances speed and accuracy
- **Long Press**: 2000ms (default) prevents accidental triggers

---

## Troubleshooting

### Gamepad Not Detected

1. Verify USB/Bluetooth connection
2. Check if gamepad is recognized by OS
3. Ensure Conductor has Input Monitoring permissions (macOS)
4. Try reconnecting the gamepad

### Button IDs Not Working

1. Use MIDI Learn to verify correct button ID
2. Check ID range (128-255 for gamepad)
3. Ensure gamepad is SDL2-compatible
4. Consult device-specific documentation

### Analog Stick False Triggers

1. Increase dead zone threshold (default: 10%)
2. Clean analog stick (dust/debris can cause drift)
3. Use directional filtering (`direction` parameter)

### Analog Trigger Sensitivity

1. Adjust `threshold` parameter (64 = half-pull)
2. Lower threshold for more sensitivity
3. Higher threshold for less sensitivity
4. Test with event console to find optimal value

---

## Related Documentation

- **[Actions Reference](./actions.md)**: Learn about available actions to execute
- **[Gamepad Support Guide](../guides/gamepad-support.md)**: Comprehensive guide to using gamepads with Conductor
- **[Configuration Guide](../guides/configuration.md)**: Complete configuration file structure
- **[MIDI Learn](../guides/midi-learn.md)**: Auto-detect button and note IDs
- **[Device Templates](../guides/device-templates.md)**: Pre-configured templates for popular devices

---

**Last Updated**: 2025-01-21
**Version**: 3.0
**Maintainer**: Amiable Team
