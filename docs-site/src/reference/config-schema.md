# Configuration Schema Reference

Complete reference for Conductor's TOML configuration file format. Covers both MIDI and Game Controllers (HID) with detailed field descriptions, examples, and validation rules.

## Table of Contents

- [File Structure](#file-structure)
- [Device Configuration](#device-configuration)
- [Input Mode Selection](#input-mode-selection)
- [Modes](#modes)
- [Mappings](#mappings)
- [Trigger Types](#trigger-types)
  - [MIDI Triggers](#midi-triggers)
  - [Gamepad Triggers](#gamepad-triggers)
- [Action Types](#action-types)
- [Advanced Settings](#advanced-settings)
- [ID Range Allocation](#id-range-allocation)
- [Complete Examples](#complete-examples)

## File Structure

A Conductor configuration file consists of these top-level sections:

```toml
[device]                    # Device configuration
[logging]                   # Logging settings (optional)
[advanced_settings]         # Timing thresholds and behavior
[[modes]]                   # Mode definitions (array)
[[global_mappings]]         # Global mappings (array, optional)
```

## Device Configuration

The `[device]` section defines basic device settings.

### Fields

```toml
[device]
name = "My Controller"           # String: Device name (for display)
auto_connect = true              # Boolean: Auto-connect on startup (default: true)
```

### Field Reference

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | String | Yes | - | Human-readable device name |
| `auto_connect` | Boolean | No | `true` | Automatically connect to device on startup |

### Examples

**MIDI Controller**:
```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
```

**Gamepad**:
```toml
[device]
name = "Xbox Controller"
auto_connect = true
```

**Hybrid Setup** (MIDI + Gamepad):
```toml
[device]
name = "Studio Setup"
auto_connect = true
# Both MIDI and gamepad can coexist - no conflicts
```

## Input Mode Selection

**Note**: The `input_mode` field is not yet implemented in the configuration file format. The daemon currently determines input mode based on available devices at runtime:

- **MIDI Only**: When only MIDI devices are connected
- **Gamepad Only**: When only gamepads are connected
- **Both**: When both MIDI and gamepad devices are available

### Future Configuration (Planned)

In a future release, you'll be able to explicitly set the input mode in the `[device]` section:

```toml
[device]
name = "My Setup"
input_mode = "Both"  # Options: "MidiOnly", "GamepadOnly", "Both"
```

### Current Behavior

The system automatically handles:
- **MIDI-only configs**: Mappings use ID range 0-127
- **Gamepad-only configs**: Mappings use ID range 128-255
- **Hybrid configs**: Mappings can use both ranges simultaneously

No configuration changes are needed to support multiple input types - simply define mappings using the appropriate ID ranges.

## Modes

Modes allow you to define different sets of mappings for different contexts. Switch between modes using encoder rotation or button combinations.

### Mode Definition

```toml
[[modes]]
name = "Default"                # String: Mode name (unique)
color = "blue"                  # String: Color for LED feedback (MIDI controllers)
# mappings array follows...
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | Yes | Unique mode identifier |
| `color` | String | No | LED color: "blue", "green", "purple", "red", "yellow", "cyan", "white", "off" |

### Examples

```toml
[[modes]]
name = "Desktop"
color = "blue"

[[modes]]
name = "Media"
color = "purple"

[[modes]]
name = "Development"
color = "green"
```

## Mappings

Mappings connect triggers (input events) to actions (system commands). Two types:
- **Mode mappings**: Active only in their parent mode
- **Global mappings**: Active across all modes

### Mapping Structure

```toml
[[modes.mappings]]               # Mode-specific mapping
description = "Action description"
[modes.mappings.trigger]
type = "Note"                    # Trigger type
note = 60                        # Trigger parameters
[modes.mappings.action]
type = "Keystroke"               # Action type
keys = "Space"                   # Action parameters

[[global_mappings]]              # Global mapping
description = "Global action"
[global_mappings.trigger]
# ... trigger definition
[global_mappings.action]
# ... action definition
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | No | Human-readable description of the mapping |
| `trigger` | Table | Yes | Trigger definition (see [Trigger Types](#trigger-types)) |
| `action` | Table | Yes | Action definition (see [Action Types](#action-types)) |

## Trigger Types

Triggers define when an action should execute. Conductor supports MIDI triggers (ID range 0-127) and Gamepad triggers (ID range 128-255).

### MIDI Triggers

MIDI triggers use the standard MIDI ID range (0-127) for notes, control changes, and other MIDI messages.

#### Note

Basic MIDI note on/off detection.

```toml
[trigger]
type = "Note"
note = 60                        # Integer: MIDI note number (0-127)
velocity_min = 1                 # Integer (optional): Minimum velocity (default 1)
```

**Use Cases**: Pad presses, piano keys, basic button mapping

#### VelocityRange

Different actions based on press intensity.

```toml
[trigger]
type = "VelocityRange"
note = 60                        # Integer: MIDI note number
min_velocity = 80                # Integer: Minimum velocity (0-127)
max_velocity = 127               # Integer: Maximum velocity (0-127)
```

**Velocity Levels**:
- **Soft**: 0-40
- **Medium**: 41-80
- **Hard**: 81-127

**Use Cases**: Soft press for play, hard press for stop; velocity-sensitive shortcuts

#### LongPress

Detect when a pad is held for a duration.

```toml
[trigger]
type = "LongPress"
note = 60                        # Integer: MIDI note number
min_duration_ms = 1000           # Integer: Minimum hold duration (default 2000)
```

**Use Cases**: Hold 2s to quit app (prevent accidental quits); confirmation for destructive actions

#### DoubleTap

Detect quick double presses.

```toml
[trigger]
type = "DoubleTap"
note = 60                        # Integer: MIDI note number
max_interval_ms = 300            # Integer (optional): Max time between taps (default 300)
```

**Use Cases**: Double-tap to toggle fullscreen; gesture-based shortcuts

#### NoteChord

Multiple notes pressed simultaneously.

```toml
[trigger]
type = "NoteChord"
notes = [60, 64, 67]            # Array: List of MIDI note numbers
max_interval_ms = 100            # Integer (optional): Max time between notes (default 100)
```

**Use Cases**: Emergency exit (press 3 corners); complex shortcuts requiring multiple pads

#### CC (Control Change)

Continuous controller messages.

```toml
[trigger]
type = "CC"
cc = 1                          # Integer: Control change number (0-127)
value_min = 64                  # Integer (optional): Minimum value to trigger
```

**Use Cases**: Button presses sending CC messages; threshold-based triggers

#### EncoderTurn

Encoder rotation with direction detection.

```toml
[trigger]
type = "EncoderTurn"
cc = 2                          # Integer: Control change number
direction = "Clockwise"         # String: "Clockwise" or "CounterClockwise"
```

**Use Cases**: Volume control with encoder; mode switching; parameter adjustment

#### Aftertouch

Channel pressure sensitivity (pressure after initial press).

```toml
[trigger]
type = "Aftertouch"
note = 1                        # Integer (optional): Specific pad for polyphonic (omit for channel)
min_pressure = 64               # Integer (optional): Minimum pressure (0-127)
max_pressure = 127              # Integer (optional): Maximum pressure (0-127)
```

**Aftertouch Types**:
- **Polyphonic (0xA0)**: Per-pad pressure
- **Channel (0xD0)**: Global pressure for entire device

**Use Cases**: Apply pressure to modulate effects; pressure-sensitive volume control

#### PitchBend

Touch strip or pitch wheel control (14-bit precision).

```toml
[trigger]
type = "PitchBend"
min_value = 8192                # Integer: Minimum bend value (0-16383)
max_value = 16383               # Integer: Maximum bend value (0-16383)
center_deadzone = 100           # Integer (optional): Deadzone around center (8192)
```

**Value Range**:
- **Full Range**: 0-16383 (14-bit resolution)
- **Center**: 8192
- **Down**: 0-8191
- **Up**: 8193-16383

**Use Cases**: Touch strip for volume control; timeline scrubbing; multi-zone selection

### Gamepad Triggers

Gamepad triggers use the extended ID range (128-255) for buttons, analog sticks, and triggers. All SDL2-compatible controllers are supported (Xbox, PlayStation, Nintendo Switch Pro, joysticks, racing wheels, flight sticks, HOTAS, custom controllers).

#### GamepadButton

Simple button press trigger.

```toml
[trigger]
type = "GamepadButton"
button = 128                    # Integer: Button ID (128-255)
velocity_min = 1                # Integer (optional): Minimum pressure (default 1)
```

**Button ID Reference**:

| ID | Xbox | PlayStation | Switch | Description |
|----|------|-------------|--------|-------------|
| 128 | A | Cross (✕) | B | South button |
| 129 | B | Circle (○) | A | East button |
| 130 | X | Square (□) | Y | West button |
| 131 | Y | Triangle (△) | X | North button |
| 132 | - | - | - | D-Pad Up |
| 133 | - | - | - | D-Pad Down |
| 134 | - | - | - | D-Pad Left |
| 135 | - | - | - | D-Pad Right |
| 136 | LB | L1 | L | Left shoulder |
| 137 | RB | R1 | R | Right shoulder |
| 138 | L3 | L3 | L-Click | Left stick click |
| 139 | R3 | R3 | R-Click | Right stick click |
| 140 | Menu | Options | + | Start button |
| 141 | View | Share | - | Select/Back button |
| 142 | Xbox | PS | Home | Guide/Home button |
| 143 | LT | L2 | ZL | Left trigger (digital) |
| 144 | RT | R2 | ZR | Right trigger (digital) |

**Use Cases**: Face button for confirm/cancel; D-Pad for arrow keys; shoulder buttons for tab switching

**Examples**:

```toml
# Xbox A button (PlayStation Cross, Switch B)
[[modes.mappings]]
description = "Confirm action"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

# D-Pad Up
[[modes.mappings]]
description = "Navigate up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# Left shoulder button
[[modes.mappings]]
description = "Previous tab"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Keystroke"
keys = "Tab"
modifiers = ["cmd", "shift"]
```

#### GamepadButtonChord

Multiple buttons pressed simultaneously.

```toml
[trigger]
type = "GamepadButtonChord"
buttons = [128, 129]            # Array: List of button IDs (128-255)
timeout_ms = 50                 # Integer (optional): Max time between buttons (default 50)
```

**Use Cases**: Multi-button combos for screenshots; safety mechanisms; mode switching

**Examples**:

```toml
# A+B chord for screenshot
[[modes.mappings]]
description = "A+B: Screenshot"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]  # A + B
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

# LB+RB chord for Mission Control
[[modes.mappings]]
description = "LB+RB: Mission Control"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]  # LB + RB
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
modifiers = ["ctrl"]

# L3+R3 (both stick clicks) for mode change
[[global_mappings]]
description = "Stick clicks: Switch mode"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [138, 139]
[global_mappings.action]
type = "ModeChange"
mode = "Media"
```

#### GamepadAnalogStick

Analog stick movement detection with direction.

```toml
[trigger]
type = "GamepadAnalogStick"
axis = 130                      # Integer: Axis ID (128-133)
direction = "Clockwise"         # String: "Clockwise" or "CounterClockwise"
```

**Axis ID Reference**:

| ID | Axis | Description |
|----|------|-------------|
| 128 | Left Stick X | Horizontal movement (left/right) |
| 129 | Left Stick Y | Vertical movement (up/down) |
| 130 | Right Stick X | Horizontal movement (left/right) |
| 131 | Right Stick Y | Vertical movement (up/down) |
| 132 | Left Trigger | Analog trigger pressure (L2/LT/ZL) |
| 133 | Right Trigger | Analog trigger pressure (R2/RT/ZR) |

**Direction Mapping**:
- **X-axis**: "Clockwise" = moving right, "CounterClockwise" = moving left
- **Y-axis**: "Clockwise" = moving up, "CounterClockwise" = moving down

**Use Cases**: Right stick for browser navigation; left stick for scrolling; camera control

**Examples**:

```toml
# Right stick horizontal: Browser navigation
[[modes.mappings]]
description = "Right stick right: Forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X-axis
direction = "Clockwise"  # Moving right
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Right stick left: Back"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X-axis
direction = "CounterClockwise"  # Moving left
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"
modifiers = ["cmd"]

# Right stick vertical: Scrolling
[[modes.mappings]]
description = "Right stick up: Scroll up"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 131  # Right stick Y-axis
direction = "Clockwise"  # Moving up
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

[[modes.mappings]]
description = "Right stick down: Scroll down"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 131  # Right stick Y-axis
direction = "CounterClockwise"  # Moving down
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"
```

#### GamepadTrigger

Analog trigger pressure detection with threshold.

```toml
[trigger]
type = "GamepadTrigger"
trigger = 132                   # Integer: Trigger axis ID (132 or 133)
threshold = 64                  # Integer (optional): Pressure threshold (0-255, default 128)
```

**Trigger IDs**:
- **132**: Left trigger (L2/LT/ZL)
- **133**: Right trigger (R2/RT/ZR)

**Threshold Values**:
- **0**: Triggers immediately on any pressure
- **64**: Triggers at 25% pressure (light press)
- **128**: Triggers at 50% pressure (half-press, default)
- **192**: Triggers at 75% pressure (hard press)
- **255**: Triggers only at full press

**Use Cases**: Gradual volume control; variable speed actions; pressure-sensitive shortcuts

**Examples**:

```toml
# Light trigger press for volume control
[[modes.mappings]]
description = "Right trigger: Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger
threshold = 64  # 25% pressure
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

[[modes.mappings]]
description = "Left trigger: Volume down"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132  # Left trigger
threshold = 64
[modes.mappings.action]
type = "VolumeControl"
operation = "Down"

# Full trigger press for different action
[[modes.mappings]]
description = "Right trigger full press: Next track"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 240  # ~94% pressure
[modes.mappings.action]
type = "Keystroke"
keys = "Next"
```

### Device-Specific Controller Types

While the trigger types above work with all SDL2-compatible controllers, here are specific device type examples:

#### Joysticks (Flight Sticks)

```toml
# Joystick trigger button
[[modes.mappings]]
description = "Trigger button: Fire"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Primary trigger
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

# Hat switch (often mapped to D-Pad buttons 132-135)
[[modes.mappings]]
description = "Hat up: Look up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# Joystick pitch axis
[[modes.mappings]]
description = "Pitch forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Y-axis
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "w"
```

#### Racing Wheels

```toml
# Wheel rotation (left/right)
[[modes.mappings]]
description = "Steer left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # Wheel X-axis
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"

# Throttle pedal
[[modes.mappings]]
description = "Accelerate"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Throttle
threshold = 32  # Light pressure
[modes.mappings.action]
type = "Keystroke"
keys = "w"

# Brake pedal
[[modes.mappings]]
description = "Brake"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132  # Brake
threshold = 32
[modes.mappings.action]
type = "Keystroke"
keys = "s"
```

#### HOTAS (Hands On Throttle And Stick)

```toml
# Throttle axis
[[modes.mappings]]
description = "Throttle up"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 132  # Throttle axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "="

# Multiple hat switches (mapped to available buttons)
[[modes.mappings]]
description = "Hat 1 up: Target up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

# Pinky switch
[[modes.mappings]]
description = "Pinky switch: Modifier"
[modes.mappings.trigger]
type = "GamepadButton"
button = 143
[modes.mappings.action]
type = "ModeChange"
mode = "Combat"
```

## Action Types

Actions define what happens when a trigger fires. For complete action type documentation, see [Action Types Reference](action-types.md).

Common action types:

```toml
# Keystroke
[action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

# Launch application
[action]
type = "Launch"
app = "Terminal"

# Shell command
[action]
type = "Shell"
command = "echo Hello"

# Volume control
[action]
type = "VolumeControl"
operation = "Up"  # "Up", "Down", "Mute", "Set"

# Mode change
[action]
type = "ModeChange"
mode = "Media"
```

## Advanced Settings

The `[advanced_settings]` section configures timing thresholds and detection behavior.

### Fields

```toml
[advanced_settings]
chord_timeout_ms = 50           # Integer: Chord detection window (default 100)
double_tap_timeout_ms = 300     # Integer: Double-tap window (default 300)
hold_threshold_ms = 2000        # Integer: Long press threshold (default 2000)
```

### Field Reference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `chord_timeout_ms` | Integer | 100 | Max time between first and last note/button in chord (ms) |
| `double_tap_timeout_ms` | Integer | 300 | Max time between taps for double-tap detection (ms) |
| `hold_threshold_ms` | Integer | 2000 | Minimum hold duration for long press detection (ms) |

### Recommendations

**For gamepads** (faster input):
```toml
[advanced_settings]
chord_timeout_ms = 50           # Shorter window for button chords
double_tap_timeout_ms = 200     # Faster double-tap
hold_threshold_ms = 1000        # Shorter long press (1s)
```

**For MIDI controllers** (larger physical pads):
```toml
[advanced_settings]
chord_timeout_ms = 100          # Longer window for pad chords
double_tap_timeout_ms = 300     # Standard double-tap
hold_threshold_ms = 2000        # Standard long press (2s)
```

## ID Range Allocation

Conductor uses non-overlapping ID ranges to prevent conflicts between input protocols.

### Current Allocation

| Range | Protocol | Type | Examples |
|-------|----------|------|----------|
| **0-127** | **MIDI** | **Notes/Pads** | C0=36, C4=60, G9=127 |
| 0-127 | MIDI | CC/Encoders | Mod Wheel=1, Volume=7 |
| **128-144** | **Game Controllers** | **Buttons** | Face buttons, D-Pad, shoulders |
| 128-133 | Game Controllers | Analog Axes | Sticks, triggers |
| 145-255 | Reserved | Future | Extended gamepad, keyboard, mouse |

### Detailed Gamepad ID Mapping

**Face Buttons (128-131)**:
- 128: South (A/Cross/B)
- 129: East (B/Circle/A)
- 130: West (X/Square/Y)
- 131: North (Y/Triangle/X)

**D-Pad (132-135)**:
- 132: Up
- 133: Down
- 134: Left
- 135: Right

**Shoulders & Sticks (136-139)**:
- 136: Left shoulder (LB/L1/L)
- 137: Right shoulder (RB/R1/R)
- 138: Left stick click (L3)
- 139: Right stick click (R3)

**Menu Buttons (140-142)**:
- 140: Start (Menu/Options/+)
- 141: Select (View/Share/-)
- 142: Guide (Xbox/PS/Home)

**Trigger Buttons (143-144)**:
- 143: Left trigger digital (L2/LT/ZL)
- 144: Right trigger digital (R2/RT/ZR)

**Analog Axes (128-133)**:
- 128: Left stick X-axis
- 129: Left stick Y-axis
- 130: Right stick X-axis
- 131: Right stick Y-axis
- 132: Left trigger analog
- 133: Right trigger analog

### Usage Guidelines

1. **MIDI configs**: Use IDs 0-127 only
2. **Gamepad configs**: Use IDs 128-255 only
3. **Hybrid configs**: Mix both ranges freely - no conflicts!

## Complete Examples

### Example 1: MIDI-Only Configuration

```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 100
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Pad 1: Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "PlayPause"

[[modes.mappings]]
description = "Pads 1+2: Screenshot"
[modes.mappings.trigger]
type = "NoteChord"
notes = [36, 37]
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

[[global_mappings]]
description = "Encoder: Volume"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
```

### Example 2: Gamepad-Only Configuration

```toml
[device]
name = "Xbox Controller"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 200
hold_threshold_ms = 1000

[[modes]]
name = "Desktop"
color = "blue"

[[modes.mappings]]
description = "A button: Confirm"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

[[modes.mappings]]
description = "B button: Cancel"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "Escape"

[[modes.mappings]]
description = "Right stick right: Forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]

[[modes.mappings]]
description = "A+B chord: Screenshot"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

[[modes]]
name = "Media"
color = "purple"

[[modes.mappings]]
description = "A button: Play/Pause"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "PlayPause"

[[modes.mappings]]
description = "Right trigger: Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

[[global_mappings]]
description = "LB+RB: Switch mode"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]
[global_mappings.action]
type = "ModeChange"
mode = "Media"
```

### Example 3: Hybrid Configuration (MIDI + Gamepad)

Use both MIDI controller and gamepad simultaneously. No conflicts - they use different ID ranges.

```toml
[device]
name = "Studio Hybrid Setup"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 75
double_tap_timeout_ms = 300
hold_threshold_ms = 1500

[[modes]]
name = "Production"
color = "green"

# MIDI mappings (ID range 0-127)
[[modes.mappings]]
description = "MIDI Pad 1: Record"
[modes.mappings.trigger]
type = "Note"
note = 36  # MIDI note
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]

[[modes.mappings]]
description = "MIDI Encoder: Volume"
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 2  # MIDI CC
direction = "Clockwise"
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Gamepad mappings (ID range 128-255)
[[modes.mappings]]
description = "Gamepad A: Play/Pause"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad button
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
description = "Gamepad Right Stick: Navigate"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Gamepad axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]

# Hybrid chord: MIDI pad + Gamepad button
[[modes.mappings]]
description = "MIDI Pad 1 + Gamepad A: Save"
[modes.mappings.trigger]
type = "NoteChord"
notes = [36, 128]  # Mix MIDI and gamepad IDs
[modes.mappings.action]
type = "Keystroke"
keys = "s"
modifiers = ["cmd"]

[[global_mappings]]
description = "Gamepad Guide: Spotlight"
[global_mappings.trigger]
type = "GamepadButton"
button = 142
[global_mappings.action]
type = "Keystroke"
keys = "Space"
modifiers = ["cmd"]
```

### Example 4: Joystick/Flight Stick Configuration

```toml
[device]
name = "Logitech Flight Stick"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 1000

[[modes]]
name = "Flight"
color = "blue"

[[modes.mappings]]
description = "Trigger: Fire"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
description = "Hat up: Look up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

[[modes.mappings]]
description = "Pitch forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "w"

[[modes.mappings]]
description = "Yaw left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "a"
```

### Example 5: Racing Wheel Configuration

```toml
[device]
name = "Logitech G29"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 1000

[[modes]]
name = "Racing"
color = "red"

[[modes.mappings]]
description = "Steer left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # Wheel rotation
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"

[[modes.mappings]]
description = "Steer right"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"

[[modes.mappings]]
description = "Throttle"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 32
[modes.mappings.action]
type = "Keystroke"
keys = "w"

[[modes.mappings]]
description = "Brake"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 32
[modes.mappings.action]
type = "Keystroke"
keys = "s"

[[modes.mappings]]
description = "Shift up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137  # Right paddle
[modes.mappings.action]
type = "Keystroke"
keys = "e"

[[modes.mappings]]
description = "Shift down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136  # Left paddle
[modes.mappings.action]
type = "Keystroke"
keys = "q"
```

## Validation Rules

Conductor validates configurations at load time. Common validation errors:

### ID Range Violations

```toml
# ❌ INVALID: MIDI trigger using gamepad ID
[trigger]
type = "Note"
note = 150  # ERROR: MIDI notes must be 0-127

# ✅ VALID: Use GamepadButton for IDs 128+
[trigger]
type = "GamepadButton"
button = 150
```

### Missing Required Fields

```toml
# ❌ INVALID: Missing 'note' field
[trigger]
type = "Note"
# ERROR: 'note' is required

# ✅ VALID: All required fields present
[trigger]
type = "Note"
note = 60
```

### Invalid Values

```toml
# ❌ INVALID: Direction typo
[trigger]
type = "EncoderTurn"
cc = 2
direction = "CW"  # ERROR: Must be "Clockwise" or "CounterClockwise"

# ✅ VALID: Correct direction value
[trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
```

## See Also

- [Trigger Types Reference](trigger-types.md) - Detailed trigger documentation
- [Action Types Reference](action-types.md) - Detailed action documentation
- [Gamepad Support Guide](../guides/gamepad-support.md) - Getting started with gamepads
- [Configuration Examples](../configuration/examples.md) - Real-world configuration examples
- [CLI Commands](cli-commands.md) - Command-line tools and debugging

---

**Last Updated**: 2025-01-21
**Version**: 3.0
**Status**: Complete
