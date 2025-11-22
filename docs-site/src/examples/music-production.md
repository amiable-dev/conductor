# Music Production Workflows

Complete Conductor configurations for music producers using DAWs, sample libraries, and production software.

## Overview

Conductor transforms MIDI controllers into intelligent, context-aware control surfaces for music production. Use velocity sensitivity, per-app profiles, and LED feedback to create efficient recording and mixing workflows.

**What You'll Learn**:
- Velocity-sensitive recording workflows
- Per-app profile switching for different DAWs
- LED feedback for visual mode indicators
- Hybrid MIDI+gamepad setups for hands-free control

---

## Quick Start: Velocity-Sensitive Recording

The killer feature for producers: map different recording modes to velocity ranges on a single pad.

### Configuration

```toml
[device]
name = "Music Production Setup"
auto_connect = true

[[modes]]
name = "Recording"
color = "red"

# Soft press = Loop Record
[[modes.mappings]]
description = "Pad 1 Soft: Loop Record"
[modes.mappings.trigger]
type = "VelocityRange"
note = 36
min_velocity = 0
max_velocity = 40
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]

# Medium press = Punch Record
[[modes.mappings]]
description = "Pad 1 Medium: Punch Record"
[modes.mappings.trigger]
type = "VelocityRange"
note = 36
min_velocity = 41
max_velocity = 80
[modes.mappings.action]
type = "Keystroke"
keys = "p"
modifiers = ["cmd", "shift"]

# Hard press = Toggle Record Enable
[[modes.mappings]]
description = "Pad 1 Hard: Record Enable"
[modes.mappings.trigger]
type = "VelocityRange"
note = 36
min_velocity = 81
max_velocity = 127
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["opt", "cmd"]
```

**Result**: One pad, three recording modes based on velocity. No more mode switching!

---

## Ableton Live Integration

Full control surface setup for Ableton Live with transport, clip launching, and mixer control.

### Hardware Recommendations
- **MIDI Controller**: Maschine Mikro MK3, Launchpad Mini, APC Mini
- **Optional Gamepad**: Xbox/PlayStation controller for navigation

### Complete Configuration

```toml
[device]
name = "Ableton Live Control"
auto_connect = true

[advanced_settings]
hold_threshold_ms = 1000
double_tap_timeout_ms = 300

# ========== Mode 1: Recording ==========
[[modes]]
name = "Recording"
color = "red"

# Transport Controls (Pads 1-4)
[[modes.mappings]]
description = "Pad 1: Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
description = "Pad 2: Stop"
[modes.mappings.trigger]
type = "Note"
note = 37
[modes.mappings.action]
type = "Keystroke"
keys = "Space"
modifiers = ["shift"]

[[modes.mappings]]
description = "Pad 3: Record (Velocity-Sensitive)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 38
min_velocity = 0
max_velocity = 80
[modes.mappings.action]
type = "Keystroke"
keys = "F9"

[[modes.mappings]]
description = "Pad 3 Hard: Overdub Record"
[modes.mappings.trigger]
type = "VelocityRange"
note = 38
min_velocity = 81
max_velocity = 127
[modes.mappings.action]
type = "Keystroke"
keys = "F9"
modifiers = ["shift"]

# Clip Launch (Pads 5-12)
[[modes.mappings]]
description = "Pad 5-12: Launch Clips in Scene"
[modes.mappings.trigger]
type = "Note"
note = 40
[modes.mappings.action]
type = "Keystroke"
keys = "Return"

# ========== Mode 2: Mixing ==========
[[modes]]
name = "Mixing"
color = "green"

# Volume (Encoders or Velocity-Mapped Pads)
[[modes.mappings]]
description = "Encoder 1: Master Volume"
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 1
direction = "Clockwise"
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Send Effects
[[modes.mappings]]
description = "Pad 1-4: Toggle Send A/B/C/D"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["opt", "cmd"]

# ========== Global: Mode Switching ==========
[[global_mappings]]
description = "Encoder Turn: Cycle Modes"
[global_mappings.trigger]
type = "EncoderTurn"
encoder = 0
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = "Mixing"
```

### Workflow Tips
- **Use velocity for recording modes**: Soft = loop, hard = punch
- **LED feedback**: Configure LED schemes to show current mode (red = recording, green = mixing)
- **Per-track control**: Map pads to track-specific record enable buttons

---

## Logic Pro X Integration

Optimized for Logic Pro's workflow with Smart Controls, Drummer, and arrangement features.

[See detailed Logic Pro examples in the Logic Pro Integration guide]

---

## FL Studio Integration

### Quick Setup

```toml
[[modes]]
name = "FL Studio"
color = "orange"

# Piano Roll
[[modes.mappings]]
description = "Pad 1: Open Piano Roll"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "F7"

# Mixer
[[modes.mappings]]
description = "Pad 2: Open Mixer"
[modes.mappings.trigger]
type = "Note"
note = 37
[modes.mappings.action]
type = "Keystroke"
keys = "F9"

# Pattern/Song Mode Toggle
[[modes.mappings]]
description = "Pad 3: Toggle Pattern/Song"
[modes.mappings.trigger]
type = "Note"
note = 38
[modes.mappings.action]
type = "Keystroke"
keys = "F8"
```

---

## Hybrid MIDI + Gamepad Workflow

Combine MIDI controller for recording with gamepad for DAW navigation.

### Setup
- **MIDI**: Maschine, Launchpad, or APC for pads and encoders
- **Gamepad**: Xbox/PlayStation controller for transport and navigation

```toml
[device]
name = "Hybrid Production Setup"

# MIDI Mappings (Note range 0-127)
[[modes.mappings]]
description = "MIDI Pad 1: Record"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]

# Gamepad Mappings (Button range 128-255)
[[modes.mappings]]
description = "Xbox A: Play/Pause"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
description = "Xbox D-Pad: Track Navigation"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132  # D-Pad Up
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
```

**Why This Works**: Keep hands on MIDI pads for recording, use thumbs on gamepad for transport.

---

## Sample Libraries & Virtual Instruments

### Kontakt / Native Instruments

```toml
[[modes.mappings]]
description = "Pad Hold: Articulation Switching"
[modes.mappings.trigger]
type = "LongPress"
note = 36
duration_ms = 1000
[modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["ctrl"]
```

### Splice Integration

Quick sample browser navigation:

```toml
[[modes.mappings]]
description = "Encoder: Browse Samples"
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 1
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "DownArrow"
```

---

## Hardware Recommendations

### Budget Setup ($0-50)
- **Reuse existing**: Maschine, Launchpad, Xbox controller
- **Software**: Any DAW (Ableton, Logic, FL Studio)
- **Cost**: $0 (repurpose hardware)

### Mid-Range Setup ($50-200)
- **MIDI Controller**: Arturia BeatStep ($99)
- **Gamepad**: Xbox Elite Controller ($180) or Standard ($60)
- **Hybrid**: Best of both worlds

### Pro Setup ($200+)
- **MIDI**: Maschine MK3 ($599)
- **Optional**: Stream Deck for visual feedback ($150)
- **Result**: Professional control surface

---

## Troubleshooting

### MIDI Latency Issues
- **Problem**: Delayed response from MIDI pads
- **Solution**: Reduce buffer size in DAW preferences (128 samples or lower)
- **Conductor Impact**: <1ms latency on Conductor side, latency is DAW-side

### Velocity Not Working
- **Problem**: All presses trigger same action regardless of velocity
- **Solution**: Check MIDI controller supports velocity sensitivity
- **Test**: Use `cargo run --bin midi_diagnostic` to verify velocity values

### Per-App Profiles Not Switching
- **Problem**: Same mappings work in all apps
- **Solution**: Enable per-app profiles in Conductor config
- **Check**: Ensure app names match exactly (case-sensitive)

---

## Next Steps

- **[Explore Velocity Curves](../guides/velocity-curves.md)** - Fine-tune velocity response
- **[Set Up LED Feedback](../guides/led-system.md)** - Visual mode indicators
- **[Join Community](../resources/community.md)** - Share your setup and get help
