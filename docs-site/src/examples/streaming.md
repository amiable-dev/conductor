# Streaming Workflows

Complete OBS Studio and streaming platform integration using gamepads as free Stream Deck alternatives.

## Overview

Turn your Xbox or PlayStation controller into a professional streaming control surface. Save $150-300 by repurposing existing gamepad hardware instead of buying a Stream Deck.

**What You'll Learn**:
- Scene switching and source control
- Audio mixing with velocity-sensitive fading
- Multi-platform setup (OBS + Discord + Spotify)
- Emergency controls (instant mute, BRB scene)
- Advanced: Button chords for complex actions

---

## Quick Start: Basic OBS Control

Essential streaming controls on a gamepad.

```toml
[device]
name = "Basic Streaming Setup"
auto_connect = true

[[modes]]
name = "Streaming"
color = "purple"

# Face Buttons: Core Controls
[[modes.mappings]]
description = "A Button: Start/Stop Recording"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "B Button: Mute/Unmute Microphone"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "m"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "X Button: Scene 1 (Gameplay)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "Y Button: Scene 2 (Webcam)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Keystroke"
keys = "2"
modifiers = ["ctrl", "shift"]
```

**Setup Required**: Configure OBS hotkeys to match (File → Settings → Hotkeys)

---

## Professional Streaming Setup

Complete Xbox/PlayStation controller configuration for OBS + Discord + Spotify.

### Hardware
- **Xbox Controller**: Xbox One, Series X/S ($30-60)
- **PlayStation Controller**: DualShock 4, DualSense ($30-70)
- **Cost vs Stream Deck**: $30-70 vs $150-300 (save $120-230!)

### Full Configuration

```toml
[device]
name = "Pro Streaming Controller"
auto_connect = true

[advanced_settings]
hold_threshold_ms = 1000
double_tap_timeout_ms = 300

[[modes]]
name = "Live Streaming"
color = "red"

# ========== Scene Switching ==========
# Face Buttons: Primary Scenes
[[modes.mappings]]
description = "A: Gameplay Scene"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "B: Webcam Scene"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "2"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "X: Desktop Scene"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "Y: BRB Scene"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Keystroke"
keys = "4"
modifiers = ["ctrl", "shift"]

# ========== Audio Control ==========
# D-Pad: Volume & Music
[[modes.mappings]]
description = "D-Pad Up: Desktop Volume Up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

[[modes.mappings]]
description = "D-Pad Down: Desktop Volume Down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133
[modes.mappings.action]
type = "VolumeControl"
operation = "Down"

[[modes.mappings]]
description = "D-Pad Left: Previous Track (Spotify)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 134
[modes.mappings.action]
type = "Keystroke"
keys = "Previous"

[[modes.mappings]]
description = "D-Pad Right: Next Track (Spotify)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135
[modes.mappings.action]
type = "Keystroke"
keys = "Next"

# ========== Recording & Streaming ==========
# Shoulders: Record & Stream
[[modes.mappings]]
description = "LB: Toggle Recording"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "RB: Toggle Streaming"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137
[modes.mappings.action]
type = "Keystroke"
keys = "s"
modifiers = ["ctrl", "shift"]

# ========== Velocity-Sensitive Audio Fading ==========
# Triggers: Audio Mixing
[[modes.mappings]]
description = "LT: Fade Out Desktop Audio"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 64
[modes.mappings.action]
type = "Sequence"
actions = [
    { VolumeControl = { operation = "Down" } },
    { Delay = { duration_ms = 50 } },
    { VolumeControl = { operation = "Down" } },
    { Delay = { duration_ms = 50 } },
    { VolumeControl = { operation = "Down" } }
]

[[modes.mappings]]
description = "RT: Fade In Desktop Audio"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64
[modes.mappings.action]
type = "Sequence"
actions = [
    { VolumeControl = { operation = "Up" } },
    { Delay = { duration_ms = 50 } },
    { VolumeControl = { operation = "Up" } },
    { Delay = { duration_ms = 50 } },
    { VolumeControl = { operation = "Up" } }
]

# ========== Emergency Controls ==========
# Button Chords: Emergency Actions
[[modes.mappings]]
description = "LB+RB: Mute All (Mic + Desktop)"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]
timeout_ms = 50
[modes.mappings.action]
type = "Sequence"
actions = [
    { Keystroke = { keys = "m", modifiers = ["ctrl", "shift"] } },  # Mute mic
    { VolumeControl = { operation = "Mute" } }  # Mute desktop
]

[[modes.mappings]]
description = "LT+RT: Emergency BRB Scene"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [138, 139]
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "9"
modifiers = ["ctrl", "shift"]  # Scene 9 = Emergency BRB

# ========== Source Control ==========
# Menu Buttons: Source Visibility
[[modes.mappings]]
description = "Start: Toggle Webcam Visibility"
[modes.mappings.trigger]
type = "GamepadButton"
button = 140
[modes.mappings.action]
type = "Keystroke"
keys = "w"
modifiers = ["ctrl", "shift"]

[[modes.mappings]]
description = "Select: Screenshot"
[modes.mappings.trigger]
type = "GamepadButton"
button = 141
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]
```

---

## OBS Hotkey Setup

Configure these hotkeys in OBS (File → Settings → Hotkeys):

| Action | Hotkey | Controller Button |
|--------|--------|-------------------|
| Scene 1 | Ctrl+Shift+1 | A |
| Scene 2 | Ctrl+Shift+2 | B |
| Scene 3 | Ctrl+Shift+3 | X |
| Scene 4 | Ctrl+Shift+4 | Y |
| Start Recording | Ctrl+Shift+R | LB |
| Start Streaming | Ctrl+Shift+S | RB |
| Mute Microphone | Ctrl+Shift+M | B (double-tap) |
| Toggle Webcam | Ctrl+Shift+W | Start |

---

## Multi-Platform Integration

### Discord Integration

```toml
# Push-to-Talk
[[modes.mappings]]
description = "LT (Hold): Push-to-Talk Discord"
[modes.mappings.trigger]
type = "GamepadTriggerHold"
trigger = 132
threshold = 64
duration_ms = 100
[modes.mappings.action]
type = "Keystroke"
keys = "grave"  # Backtick key
modifiers = ["ctrl"]

# Mute/Deafen
[[modes.mappings]]
description = "Select (Double-Tap): Toggle Deafen"
[modes.mappings.trigger]
type = "GamepadButtonDoubleTap"
button = 141
timeout_ms = 300
[modes.mappings.action]
type = "Keystroke"
keys = "d"
modifiers = ["ctrl", "shift"]
```

### Spotify Control

```toml
# Music Playback
[[modes.mappings]]
description = "D-Pad Center: Play/Pause Spotify"
[modes.mappings.trigger]
type = "GamepadButton"
button = 142  # Guide/Home button
[modes.mappings.action]
type = "Keystroke"
keys = "PlayPause"
```

---

## Advanced: Instant Replay & Highlights

```toml
# Save Last 30 Seconds (Instant Replay)
[[modes.mappings]]
description = "RT (Hold 2s): Save Instant Replay"
[modes.mappings.trigger]
type = "GamepadTriggerHold"
trigger = 133
threshold = 64
duration_ms = 2000
[modes.mappings.action]
type = "Keystroke"
keys = "i"
modifiers = ["ctrl", "shift"]

# Add Stream Marker
[[modes.mappings]]
description = "Y (Hold): Add Stream Marker"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 131
duration_ms = 1000
[modes.mappings.action]
type = "Keystroke"
keys = "k"
modifiers = ["ctrl", "shift"]
```

---

## Troubleshooting

### OBS Hotkeys Not Working
- **Problem**: Button presses don't trigger OBS actions
- **Solution**: Ensure OBS hotkeys match exactly (case-sensitive)
- **Check**: Run OBS as administrator (Windows) or grant permissions (macOS)

### Audio Fading Too Fast/Slow
- **Problem**: Volume changes too abruptly
- **Solution**: Adjust delay duration in Sequence actions (increase from 50ms to 100ms+)

### Button Chords Not Detecting
- **Problem**: Pressing LB+RB doesn't trigger chord action
- **Solution**: Reduce chord timeout_ms (try 30ms instead of 50ms)

---

## Cost Comparison

| Setup | Hardware | Cost | Savings |
|-------|----------|------|---------|
| **Conductor** | Xbox Controller (owned) | $0 | $150-300 |
| **Conductor** | New Xbox Controller | $30-60 | $90-270 |
| **Conductor** | PlayStation DualSense | $60-70 | $80-240 |
| **Stream Deck** | Stream Deck Mini | $79.99 | - |
| **Stream Deck** | Stream Deck MK.2 | $149.99 | - |
| **Stream Deck** | Stream Deck XL | $249.99 | - |

**Bottom Line**: Reusing an existing gamepad = $150-300 saved, same functionality.

---

## Next Steps

- **[See Success Stories](../inspiration/success-stories.md)** - Real streamer testimonials
- **[Explore Automation Examples](automation.md)** - Advanced workflows
- **[Learn Button Chords](../configuration/triggers.md#gamepadbuttonchord)** - Complex actions
- **[Join Streaming Community](../resources/community.md)** - Share your setup
