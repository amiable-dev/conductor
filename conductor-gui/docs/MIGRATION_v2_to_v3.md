# Migration Guide: v2.x to v3.0

**MIDIMon v3.0: Game Controller (HID) Support**

This guide helps you upgrade from MIDIMon v2.x to v3.0, which introduces comprehensive game controller support through a unified input management system.

## Table of Contents

1. [Breaking Changes](#breaking-changes)
2. [New Features](#new-features)
3. [Upgrade Steps](#upgrade-steps)
4. [Configuration Changes](#configuration-changes)
5. [Adopting Game Controller Features](#adopting-game-controller-features)
6. [ID Range Allocation](#id-range-allocation)
7. [Performance Notes](#performance-notes)
8. [Troubleshooting](#troubleshooting)
9. [Rollback Instructions](#rollback-instructions)

---

## Breaking Changes

**None!** MIDIMon v3.0 is **100% backward compatible** with v2.x configurations.

### What This Means

- ✅ All v2.x `config.toml` files work without modification
- ✅ Existing MIDI mappings continue to function identically
- ✅ All v2.x features remain fully supported
- ✅ Performance characteristics are maintained (sub-1ms latency)
- ✅ No API changes to existing functionality

Your existing setup will continue working exactly as before after upgrading to v3.0.

---

## New Features

MIDIMon v3.0 adds the following capabilities while preserving all v2.x functionality:

### 1. Game Controller (HID) Support

Use Xbox, PlayStation, Nintendo Switch Pro, and any SDL2-compatible gamepads as macro input devices.

**Supported Controllers**:
- Xbox 360, Xbox One, Xbox Series X|S
- PlayStation DualShock 4, DualSense (PS5)
- Nintendo Switch Pro Controller
- Generic USB/Bluetooth gamepads (100+ via SDL2 mappings)

**Key Features**:
- Button press detection
- Analog stick movement tracking
- Analog trigger threshold detection
- Button chord combinations
- Double-tap and long-press patterns
- 1000Hz polling (1ms latency)

### 2. Unified Input Manager

A new architectural component that seamlessly integrates MIDI and gamepad inputs:

- **Single Event Stream**: MIDI and gamepad events processed through unified pipeline
- **ID Range Separation**: MIDI (0-127), Gamepads (128-255) - zero conflicts
- **Concurrent Support**: Use MIDI controllers and gamepads simultaneously
- **Hot-Plugging**: Automatic device reconnection with exponential backoff
- **Thread-Safe**: Lock-free event channels with Arc/Mutex patterns

### 3. Extended GUI

The Tauri GUI now includes:

- **Gamepad Template Selector**: 6 official controller templates
  - Xbox Controller
  - PlayStation DualShock/DualSense
  - Nintendo Switch Pro
  - Generic Gamepad
- **Enhanced MIDI Learn**: Auto-detects gamepad button patterns
- **Visual ID Display**: Clear separation of MIDI vs gamepad mappings
- **Device Status**: Real-time connection status for all devices

---

## Upgrade Steps

### Option 1: Package Manager (Recommended)

#### macOS (Homebrew)

```bash
# Update tap
brew update

# Upgrade MIDIMon
brew upgrade midimon

# Restart daemon
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist
launchctl load ~/Library/LaunchAgents/com.amiable.midimon.plist

# Verify version
midimonctl --version
# Expected: midimon v3.0.0
```

#### Linux (APT)

```bash
# Update package list
sudo apt update

# Upgrade MIDIMon
sudo apt install --only-upgrade midimon

# Restart daemon
systemctl --user restart midimon

# Verify version
midimonctl --version
```

### Option 2: Build from Source

```bash
# Navigate to MIDIMon directory
cd ~/projects/midimon

# Pull latest changes
git fetch origin
git checkout v3.0.0

# Build workspace
cargo build --release --workspace

# Install binaries
sudo install -m 755 target/release/midimon /usr/local/bin/
sudo install -m 755 target/release/midimonctl /usr/local/bin/

# Install Tauri GUI (optional)
cd midimon-gui
npm install
npm run tauri build
# Binary: src-tauri/target/release/bundle/

# Restart daemon
# macOS:
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist
launchctl load ~/Library/LaunchAgents/com.amiable.midimon.plist

# Linux:
systemctl --user restart midimon
```

### Option 3: Pre-Built Binary

```bash
# Download latest release
wget https://github.com/amiable/midimon/releases/download/v3.0.0/midimon-v3.0.0-$(uname -s)-$(uname -m).tar.gz

# Extract
tar -xzf midimon-v3.0.0-*.tar.gz

# Install
sudo install -m 755 midimon /usr/local/bin/
sudo install -m 755 midimonctl /usr/local/bin/

# Restart daemon (see Option 2 commands above)
```

### Post-Upgrade Verification

```bash
# Check version
midimonctl --version
# Expected: midimon v3.0.0

# Check daemon status
midimonctl status
# Should show:
# - Version: v3.0.0
# - Unified Input Manager: Enabled
# - MIDI device: [your MIDI device]
# - Gamepad device: [connected gamepad or "None"]

# Verify existing mappings work
# Press a MIDI pad/button you have mapped
# Should execute configured action without issues
```

---

## Configuration Changes

### New Optional Field: `input_mode`

MIDIMon v3.0 introduces an optional `input_mode` field to explicitly control which input sources are active.

#### Default Behavior (Backward Compatible)

If you **do not** specify `input_mode`, v3.0 uses intelligent auto-detection:

```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
# input_mode not specified

# Auto-detection logic:
# 1. Connects to MIDI device if name matches
# 2. Also scans for gamepad if present
# 3. Result: Hybrid mode (both MIDI + gamepad if available)
```

This ensures your v2.x configs work without modification.

#### Explicit Configuration (v3.0 New)

For explicit control, add the `input_mode` field:

```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
input_mode = "midi"  # Options: "midi", "gamepad", "hybrid"
```

**Options**:
- `"midi"`: MIDI-only (v2.x behavior)
- `"gamepad"`: Gamepad-only (new in v3.0)
- `"hybrid"`: Both MIDI + gamepad simultaneously (v3.0 default)

**Example 1: Pure Gamepad Config**

```toml
[device]
name = "Gamepad"
auto_connect = true
input_mode = "gamepad"  # Only gamepad, no MIDI scanning
```

**Example 2: Hybrid Config**

```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
input_mode = "hybrid"  # Both MIDI + gamepad (explicit)
```

### New Trigger Types

v3.0 adds 4 new trigger types for gamepad inputs:

#### 1. GamepadButton

Simple button press trigger.

```toml
[[modes.mappings]]
description = "A button: Enter"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # South face button (A/Cross/B)
[modes.mappings.action]
type = "Keystroke"
keys = "Return"
```

#### 2. GamepadButtonChord

Multiple buttons pressed simultaneously.

```toml
[[modes.mappings]]
description = "A+B: Screenshot"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]  # A + B buttons
timeout_ms = 50       # Press within 50ms
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]
```

#### 3. GamepadAnalogStick

Analog stick movement detection with dead zone.

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

**Available Axes**:
- `128`: Left stick X-axis
- `129`: Left stick Y-axis
- `130`: Right stick X-axis
- `131`: Right stick Y-axis

**Directions**:
- `"Clockwise"`: Right (X-axis) or Up (Y-axis)
- `"CounterClockwise"`: Left (X-axis) or Down (Y-axis)

#### 4. GamepadTrigger

Analog trigger threshold detection.

```toml
[[modes.mappings]]
description = "Right trigger: Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133   # Right trigger (R2/RT/ZR)
threshold = 128 # Half-pull (0-255 range)
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

**Available Triggers**:
- `132`: Left trigger (L2/LT/ZL)
- `133`: Right trigger (R2/RT/ZR)

### Existing Trigger Types Work for Gamepads

The following v2.x trigger types **automatically work** with gamepad button IDs (128-255):

```toml
# Double-tap detection
[modes.mappings.trigger]
type = "DoubleTap"
note = 128  # Gamepad button ID
timeout_ms = 300

# Long press detection
[modes.mappings.trigger]
type = "LongPress"
note = 129  # Gamepad button ID
duration_ms = 2000

# Velocity-sensitive (uses button pressure if available)
[modes.mappings.trigger]
type = "VelocityRange"
note = 128
min_velocity = 80
max_velocity = 127
```

---

## Adopting Game Controller Features

### For Gamepads: Template-Based Quick Setup

The fastest way to get started with gamepads is using official templates.

#### Via GUI (Recommended)

1. **Open MIDIMon GUI**
   ```bash
   # macOS:
   open /Applications/MIDIMon.app

   # Linux:
   midimon-gui
   ```

2. **Navigate to Device Templates**
   - Click "Templates" in sidebar
   - Filter by "Gamepad Controllers"

3. **Select Your Controller**
   - Xbox Controller (Xbox 360/One/Series)
   - PlayStation Controller (DualShock 4/DualSense)
   - Nintendo Switch Pro Controller
   - Generic Gamepad

4. **Generate Config**
   - Click "Create Config from Template"
   - Review mappings in visual editor
   - Save configuration

5. **Reload Daemon**
   ```bash
   midimonctl reload
   ```

#### Via Configuration File

Copy an example template:

```bash
# Copy Xbox template
cp config/examples/gamepad-xbox-basic.toml ~/.config/midimon/config.toml

# Edit as needed
nano ~/.config/midimon/config.toml

# Reload daemon
midimonctl reload
```

**Available Templates**:
- `config/examples/gamepad-xbox-basic.toml` - Xbox controller
- See GUI for PlayStation and Switch templates

### For Joysticks, Wheels, Flight Sticks: Manual Configuration

For specialized HID devices, manually configure the button/axis mappings.

#### Example 1: Flight Stick Setup

```toml
[device]
name = "Logitech Extreme 3D Pro"
auto_connect = true
input_mode = "gamepad"  # Treat as gamepad

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

# Mode 1: Flight Controls
[[modes]]
name = "Flight"
color = "blue"

# Trigger button: Fire weapon
[[modes.mappings]]
description = "Trigger: Fire"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Trigger button
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

# Thumb button: Lock target
[[modes.mappings]]
description = "Thumb button: Lock target"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "t"

# Stick X-axis: Roll left/right
[[modes.mappings]]
description = "Stick right: Roll right"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # X-axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "d"

[[modes.mappings]]
description = "Stick left: Roll left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # X-axis
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "a"

# Stick Y-axis: Pitch up/down
[[modes.mappings]]
description = "Stick forward: Pitch down"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Y-axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "s"

[[modes.mappings]]
description = "Stick back: Pitch up"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Y-axis
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "w"

# Throttle control (twist axis)
[[modes.mappings]]
description = "Throttle up"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Throttle/twist axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "equal"  # Increase throttle

[[modes.mappings]]
description = "Throttle down"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "minus"  # Decrease throttle
```

#### Example 2: Racing Wheel Setup

```toml
[device]
name = "Logitech G29"
auto_connect = true
input_mode = "gamepad"

# Mode 1: Racing
[[modes]]
name = "Racing"
color = "red"

# Wheel rotation (X-axis)
[[modes.mappings]]
description = "Wheel right: Steer right"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # Steering wheel X-axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"

[[modes.mappings]]
description = "Wheel left: Steer left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"

# Gas pedal (right trigger)
[[modes.mappings]]
description = "Gas pedal: Accelerate"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger (gas)
threshold = 32  # 12.5% depression
[modes.mappings.action]
type = "Keystroke"
keys = "w"

# Brake pedal (left trigger)
[[modes.mappings]]
description = "Brake pedal: Decelerate"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132  # Left trigger (brake)
threshold = 32
[modes.mappings.action]
type = "Keystroke"
keys = "s"

# Gear shift up (button or paddle)
[[modes.mappings]]
description = "Shift up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137  # Right paddle/button
[modes.mappings.action]
type = "Keystroke"
keys = "e"

# Gear shift down
[[modes.mappings]]
description = "Shift down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136  # Left paddle/button
[modes.mappings.action]
type = "Keystroke"
keys = "q"

# Handbrake (button chord)
[[modes.mappings]]
description = "Handbrake"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [128, 129]  # Two face buttons
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "Space"
```

#### Example 3: HOTAS (Throttle + Stick) Setup

```toml
[device]
name = "Thrustmaster HOTAS"
auto_connect = true
input_mode = "gamepad"

# Mode 1: Space Combat
[[modes]]
name = "SpaceCombat"
color = "cyan"

# Stick controls (axes 128-129)
[[modes.mappings]]
description = "Stick pitch forward"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Stick Y-axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "s"

[[modes.mappings]]
description = "Stick pitch back"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "w"

[[modes.mappings]]
description = "Stick roll right"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # Stick X-axis
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "d"

[[modes.mappings]]
description = "Stick roll left"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "a"

# Throttle controls (axis 132)
[[modes.mappings]]
description = "Throttle increase"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132  # Throttle axis
threshold = 180  # ~70% forward
[modes.mappings.action]
type = "Keystroke"
keys = "equal"

[[modes.mappings]]
description = "Throttle decrease"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 75  # ~30% back
[modes.mappings.action]
type = "Keystroke"
keys = "minus"

# Weapon controls
[[modes.mappings]]
description = "Primary weapon"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Trigger button
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
description = "Secondary weapon"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129  # Secondary trigger
[modes.mappings.action]
type = "Keystroke"
keys = "LeftAlt"

# HAT switch (D-Pad 132-135)
[[modes.mappings]]
description = "Look up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132  # HAT up
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"

[[modes.mappings]]
description = "Look down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133  # HAT down
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"

[[modes.mappings]]
description = "Look left"
[modes.mappings.trigger]
type = "GamepadButton"
button = 134  # HAT left
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"

[[modes.mappings]]
description = "Look right"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135  # HAT right
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
```

### Using MIDI Learn for Gamepads

MIDI Learn mode (introduced in v2.0) now supports gamepad inputs:

```bash
# Start GUI and enable MIDI Learn
# Click "Learn" button next to any mapping

# Press buttons or move sticks on your gamepad
# MIDIMon automatically detects:
# - Button press → GamepadButton
# - Multiple buttons → GamepadButtonChord
# - Hold button → LongPress
# - Double tap → DoubleTap
# - Analog stick → GamepadAnalogStick
# - Trigger pull → GamepadTrigger

# Config is auto-generated with correct IDs and types
```

---

## ID Range Allocation

MIDIMon v3.0 uses a split ID range to prevent conflicts between MIDI and gamepad inputs.

### MIDI: IDs 0-127 (Unchanged)

MIDI devices use the standard MIDI protocol ID range:

- **Notes**: 0-127 (C-1 to G9)
- **CC**: 0-127 (Control Change)
- **Example**:
  ```toml
  [modes.mappings.trigger]
  type = "Note"
  note = 36  # MIDI note 36 (kick drum in GM)
  ```

### Gamepad: IDs 128-255 (New in v3.0)

Gamepad devices use the upper ID range:

#### Buttons: 128-144

| ID Range | Control Type |
|----------|-------------|
| 128-131  | Face buttons (A/B/X/Y or Cross/Circle/Square/Triangle) |
| 132-135  | D-Pad (Up/Down/Left/Right) |
| 136-137  | Shoulder buttons (L1/LB, R1/RB) |
| 138-139  | Stick clicks (L3, R3) |
| 140-142  | Menu buttons (Start, Select, Home) |
| 143-144  | Trigger buttons digital (L2/LT, R2/RT) |

**Example**:
```toml
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad button 128 (A/Cross/B)
```

#### Axes: 128-133

| ID  | Axis Type |
|-----|-----------|
| 128 | Left stick X-axis |
| 129 | Left stick Y-axis |
| 130 | Right stick X-axis |
| 131 | Right stick Y-axis |
| 132 | Left trigger analog |
| 133 | Right trigger analog |

**Example**:
```toml
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X-axis
direction = "Clockwise"
```

### Why This Split?

1. **Zero Conflicts**: MIDI and gamepad IDs never overlap
2. **Hybrid Mode**: Use both simultaneously without collisions
3. **Clear Separation**: Easy to identify MIDI vs gamepad mappings
4. **Standards Compliance**: MIDI uses 0-127 per spec
5. **Future Expansion**: Room for additional input types (256+)

### Hybrid Configuration Example

```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
input_mode = "hybrid"  # Both MIDI + gamepad

# MIDI mapping (ID 0-127)
[[modes.mappings]]
description = "MIDI Pad 1: Spotlight"
[modes.mappings.trigger]
type = "Note"
note = 36  # MIDI note
[modes.mappings.action]
type = "Keystroke"
keys = "Space"
modifiers = ["cmd"]

# Gamepad mapping (ID 128-255)
[[modes.mappings]]
description = "Gamepad A: Enter"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad button
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

# Both work simultaneously, no conflicts!
```

---

## Performance Notes

MIDIMon v3.0 maintains v2.x performance characteristics while adding gamepad support.

### Latency

| Metric | v2.x | v3.0 | Notes |
|--------|------|------|-------|
| MIDI Event Latency | <1ms | <1ms | Unchanged |
| Gamepad Event Latency | N/A | <1ms | 1000Hz polling |
| Config Reload | 0-10ms | 0-10ms | Unchanged |
| IPC Round-Trip | <1ms | <1ms | Unchanged |

**Total Input-to-Action Latency**: ~2-5ms (both MIDI and gamepad)

### Resource Usage

| Metric | v2.x | v3.0 | Notes |
|--------|------|------|-------|
| CPU Idle | <1% | <1% | Unchanged |
| CPU Active | <5% | <5% | No measurable increase |
| Memory | 5-10MB | 5-10MB | Gamepad adds ~500KB |
| Binary Size | 3-5MB | 3-6MB | +1MB for gilrs library |

### Throughput

- **MIDI**: 1000+ events/second (unchanged)
- **Gamepad**: 1000+ events/second (new)
- **Combined**: 2000+ events/second (hybrid mode)

### Polling Intervals

- **MIDI**: Hardware-driven (native USB MIDI)
- **Gamepad**: 1ms polling interval (1000Hz)
- **Config Reload**: 500ms debounce (unchanged)

### Benchmark Results (Apple M1)

```bash
# Run benchmarks
cargo bench --package midimon-daemon

# Config reload performance (unchanged)
# 2 modes, 10 mappings:  0-2ms   (Grade A)
# 5 modes, 50 mappings:  2-5ms   (Grade A)
# 10 modes, 100 mappings: 5-8ms   (Grade A)

# Gamepad event processing (new)
# Button press:           <1ms
# Chord detection:        <2ms
# Analog stick:           <1ms
# Trigger threshold:      <1ms
```

### Optimization Tips

1. **Disable Unused Inputs**: Use `input_mode` to disable MIDI or gamepad if not needed
   ```toml
   input_mode = "midi"  # Gamepad scanning disabled
   ```

2. **Adjust Polling**: Lower polling rate for battery-powered gamepads (not yet configurable)

3. **Profile Configs**: Use per-app profiles to reduce active mapping count

---

## Troubleshooting

### Gamepad Not Detected

**Symptoms**:
- `midimonctl status` shows "Gamepad: None"
- Gamepad button presses do nothing

**Solutions**:

1. **Verify system recognition**:
   ```bash
   # macOS:
   # System Settings > Game Controllers

   # Linux:
   ls /dev/input/js*

   # Windows:
   # Devices and Printers
   ```

2. **Check USB/Bluetooth connection**:
   - Try reconnecting
   - Try USB instead of Bluetooth (or vice versa)
   - Check for driver updates (Windows)

3. **Verify SDL2 compatibility**:
   ```bash
   # Check gilrs supported devices
   midimonctl status --verbose
   # Look for "Gamepad SDL mapping: [controller name]"
   ```

4. **Enable debug logging**:
   ```toml
   [logging]
   level = "debug"
   ```
   ```bash
   midimonctl reload
   # Check logs: ~/Library/Logs/midimon.log (macOS)
   # Check logs: ~/.local/state/midimon/daemon.log (Linux)
   ```

### Gamepad Buttons Not Working

**Symptoms**:
- Gamepad detected but button presses don't trigger actions

**Solutions**:

1. **Verify button IDs with MIDI Learn**:
   - Open GUI
   - Click "Learn" button
   - Press gamepad button
   - Check generated button ID

2. **Check ID range**:
   ```toml
   # Incorrect (MIDI range):
   button = 36  # ❌ Too low

   # Correct (gamepad range):
   button = 128  # ✅ Gamepad button
   ```

3. **Verify mapping isn't overridden**:
   - Global mappings take precedence
   - Check for conflicting mappings

### Analog Stick Too Sensitive / Not Responding

**Symptoms**:
- Stick triggers actions without moving
- Stick movement doesn't trigger actions

**Solutions**:

1. **Dead zone is fixed at 10%**: Not configurable in v3.0

2. **Use button triggers instead**:
   ```toml
   # Instead of analog stick
   [modes.mappings.trigger]
   type = "GamepadButton"
   button = 134  # D-Pad left
   ```

3. **Adjust threshold for triggers**:
   ```toml
   [modes.mappings.trigger]
   type = "GamepadTrigger"
   trigger = 133
   threshold = 64  # Try 32, 64, 128, 192
   ```

### Button Chords Not Detected

**Symptoms**:
- Pressing multiple buttons doesn't trigger chord mapping

**Solutions**:

1. **Increase chord timeout**:
   ```toml
   [advanced_settings]
   chord_timeout_ms = 100  # Increase from 50ms
   ```

2. **Press buttons more simultaneously**:
   - Practice timing
   - Try different button combinations

3. **Use MIDI Learn to test**:
   - Learn mode shows detected chord events
   - Helps verify timing

### MIDI Mappings Stopped Working

**Symptoms**:
- MIDI device worked in v2.x, doesn't work in v3.0

**Solutions**:

1. **Check `input_mode` not set to `"gamepad"`**:
   ```toml
   [device]
   input_mode = "gamepad"  # ❌ MIDI disabled

   # Fix: Remove field or use "hybrid"
   input_mode = "hybrid"   # ✅ Both enabled
   ```

2. **Verify MIDI device connection**:
   ```bash
   midimonctl status
   # Check "MIDI device: [name]" is present
   ```

3. **Reload configuration**:
   ```bash
   midimonctl reload
   ```

### Performance Degradation

**Symptoms**:
- Increased latency after v3.0 upgrade
- High CPU usage

**Solutions**:

1. **Check for excessive logging**:
   ```toml
   [logging]
   level = "info"  # Not "debug" or "trace"
   ```

2. **Disable unused input mode**:
   ```toml
   input_mode = "midi"  # If not using gamepad
   ```

3. **Verify daemon is running**:
   ```bash
   ps aux | grep midimon
   ```

4. **Check resource usage**:
   ```bash
   midimonctl status
   # Look for performance metrics
   ```

---

## Rollback Instructions

If you encounter issues with v3.0, you can rollback to v2.7.0.

### Step 1: Stop v3.0 Daemon

```bash
# macOS:
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist

# Linux:
systemctl --user stop midimon
```

### Step 2: Restore v2.7.0 Binaries

#### Option A: Package Manager

```bash
# macOS (Homebrew):
brew uninstall midimon
brew install midimon@2.7.0

# Linux (APT):
sudo apt install midimon=2.7.0
```

#### Option B: From Source

```bash
cd ~/projects/midimon
git checkout v2.7.0
cargo build --release --workspace
sudo install -m 755 target/release/midimon /usr/local/bin/
sudo install -m 755 target/release/midimonctl /usr/local/bin/
```

#### Option C: Pre-Built Binary

```bash
wget https://github.com/amiable/midimon/releases/download/v2.7.0/midimon-v2.7.0-$(uname -s)-$(uname -m).tar.gz
tar -xzf midimon-v2.7.0-*.tar.gz
sudo install -m 755 midimon /usr/local/bin/
sudo install -m 755 midimonctl /usr/local/bin/
```

### Step 3: Restore v2.7.0 Configuration

Your v2.7.0 config will work as-is (v3.0 is backward compatible).

If you made v3.0-specific changes, remove them:

```toml
# Remove v3.0-specific fields
[device]
# input_mode = "hybrid"  # Remove this line

# Remove v3.0-specific trigger types
# [[modes.mappings]]
# [modes.mappings.trigger]
# type = "GamepadButton"  # Remove gamepad mappings
# button = 128
```

### Step 4: Restart v2.7.0 Daemon

```bash
# macOS:
launchctl load ~/Library/LaunchAgents/com.amiable.midimon.plist

# Linux:
systemctl --user start midimon
```

### Step 5: Verify Rollback

```bash
# Check version
midimonctl --version
# Expected: midimon v2.7.0

# Check status
midimonctl status

# Test MIDI mappings
# Press a MIDI pad/button
```

### Step 6: Report Issue (Optional)

If you rolled back due to a bug, please report it:

```bash
# Create GitHub issue
# https://github.com/amiable/midimon/issues/new

# Include:
# - v3.0 version number
# - Rollback reason
# - Config snippet (if relevant)
# - Logs: ~/Library/Logs/midimon.log
```

---

## Additional Resources

- **[Gamepad Support Guide](../docs-site/src/guides/gamepad-support.md)**: Complete gamepad documentation
- **[Configuration Examples](../config/examples/)**: Example configs for various devices
- **[CHANGELOG.md](../CHANGELOG.md)**: Full v3.0 release notes
- **[GitHub Issues](https://github.com/amiable/midimon/issues)**: Bug reports and feature requests
- **[GitHub Discussions](https://github.com/amiable/midimon/discussions)**: Community support

---

## Summary

MIDIMon v3.0 is a **non-breaking** upgrade that adds comprehensive game controller support while maintaining 100% backward compatibility with v2.x configurations.

**Key Points**:
- ✅ All v2.x configs work without modification
- ✅ MIDI performance unchanged (sub-1ms latency)
- ✅ New gamepad support (128-255 ID range)
- ✅ Unified input manager (hybrid mode)
- ✅ Extended GUI (gamepad templates)
- ✅ Easy rollback if needed

**Next Steps**:
1. Upgrade to v3.0 (see [Upgrade Steps](#upgrade-steps))
2. Verify existing MIDI mappings work
3. (Optional) Connect a gamepad and explore templates
4. (Optional) Use MIDI Learn to auto-configure gamepad buttons
5. Enjoy enhanced input capabilities!

**Need Help?**
- GitHub Issues: https://github.com/amiable/midimon/issues
- Documentation: https://midimon.dev/docs
- Examples: `config/examples/`

---

**Version**: 3.0.0
**Date**: 2025-11-21
**Compatibility**: v2.x → v3.0
**Breaking Changes**: None
