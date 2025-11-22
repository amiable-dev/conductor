# Ableton Live Integration

This guide shows you how to set up Conductor to control Ableton Live, one of the most popular DAWs for electronic music production and live performance. We'll cover transport controls, clip launching, device parameter mapping, and session view integration.

## Prerequisites

- macOS, Windows, or Linux
- Conductor v2.1.0 or later
- Ableton Live 11 or later (Standard or Suite)
- Virtual MIDI port configured:
  - **macOS**: IAC Driver (see [DAW Control Guide](../guides/daw-control.md#macos-iac-driver))
  - **Windows**: loopMIDI (see [DAW Control Guide](../guides/daw-control.md#windows-loopmidi))
  - **Linux**: ALSA virtual port (see [DAW Control Guide](../guides/daw-control.md#linux-alsa))

## Quick Start

### 1. Configure Virtual MIDI Port

#### macOS (IAC Driver)

1. Open **Audio MIDI Setup** (Applications → Utilities)
2. Show MIDI Studio (Window → Show MIDI Studio)
3. Double-click **IAC Driver**
4. Check **"Device is online"**
5. Click **Apply**

#### Windows (loopMIDI)

1. Download and install [loopMIDI](https://www.tobias-erichsen.de/software/loopmidi.html)
2. Launch loopMIDI
3. Create a new port (e.g., "Conductor Virtual")
4. Leave loopMIDI running in the background

#### Linux (ALSA)

```bash
# Create virtual MIDI port
sudo modprobe snd-virmidi

# Verify port created
aconnect -l
```

### 2. Configure Ableton Live MIDI Input

In Ableton Live:

1. Open **Live → Preferences** (macOS) or **Options → Preferences** (Windows/Linux)
2. Go to the **Link, Tempo & MIDI** tab
3. In the **MIDI Ports** section, locate your virtual MIDI port:
   - macOS: **IAC Driver (IAC Bus 1)**
   - Windows: **loopMIDI Port**
   - Linux: **Virtual Raw MIDI 1-0**
4. Enable **Track** and **Remote** for the input port
5. Close Preferences

### 3. Create a Basic Transport Control Profile

Create or update your `~/.config/conductor/config.toml`:

```toml
[[modes]]
name = "Ableton Live"
color = "orange"

# Play/Stop (Pad 1)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"  # macOS
# windows_title_regex = ".*Ableton Live.*"  # Windows/Linux alternative

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"  # Or "loopMIDI Port" on Windows

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 0  # Ableton Scene Launch (Scene 1)
velocity = 127
channel = 0

# Stop All Clips (Pad 2)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 2

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 1  # Custom mapped to "Stop All Clips"
value = 127
channel = 0
```

### 4. Test the Configuration

1. Start the Conductor daemon: `conductor`
2. Open Ableton Live with a project containing clips
3. Press Pad 1 on your controller → Scene 1 should launch
4. Press Pad 2 → All clips should stop (if mapped)

## Ableton Live MIDI Mapping

Ableton Live uses a flexible MIDI mapping system. Unlike Logic Pro's fixed CC assignments, you map each control manually using MIDI Learn.

### Using MIDI Map Mode

1. In Ableton Live, click **MIDI** in the top-right corner (or press ⌘M / Ctrl+M)
2. The interface will highlight in purple
3. Click the parameter you want to control (e.g., a volume fader, play button, device knob)
4. Press the pad on your MIDI controller
5. Ableton will map that MIDI message to the parameter
6. Click **MIDI** again to exit MIDI Map Mode

**What Ableton Learns**:
- **Note messages** → Note number, channel
- **CC messages** → CC number, channel
- **Velocity** → Mapped to 0-127 parameter range

## Clip Launching

Ableton's Session View uses a grid of clips. Each clip can be triggered via MIDI notes.

### Default Clip Launch Mapping

By default, Ableton maps:
- **Track 1**: Notes 0-127 (clip slots 1-128)
- **Track 2**: Same notes on MIDI Channel 1
- **Track 3**: Same notes on MIDI Channel 2
- etc.

**OR** you can use MIDI Map Mode to assign specific notes to specific clips.

### Example: Launch Clips in Track 1

```toml
# Launch Clip 1 in Track 1 (Pad 9)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 9

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 0  # Clip slot 1
velocity = 127
channel = 0  # Track 1

# Launch Clip 2 in Track 1 (Pad 10)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 10

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 1  # Clip slot 2
velocity = 127
channel = 0
```

### Scene Launching

Scenes launch all clips in a row simultaneously:

```toml
# Launch Scene 1 (Pad 1)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 0  # Scene 1 (use MIDI Learn to discover exact note)
velocity = 127
channel = 0
```

**Note**: Scene launch note numbers vary by Ableton version and configuration. Use MIDI Learn to discover the correct note numbers for your setup.

## Device Parameter Control

Ableton's devices (instruments, effects) have mappable parameters.

### Step 1: Map Device Parameters in Ableton

1. Open a device (e.g., Filter, Reverb, Wavetable synth)
2. Enter **MIDI Map Mode** (click MIDI or press ⌘M)
3. Click a device knob or parameter
4. Press a pad on your MIDI controller
5. Ableton maps that control

**Pro Tip**: Ableton learns CC messages from your controller. To use velocity instead, you'll need to map in Ableton using MIDI notes, not CCs.

### Step 2: Configure Conductor for Velocity-Controlled Parameters

Use velocity curves to control device parameters smoothly:

```toml
# Control Filter Cutoff with Velocity (Pad 13)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 13

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 20  # Learned in Ableton MIDI Map Mode
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "Curve"
input_min = 0
input_max = 127
output_min = 0
output_max = 127
curve = 1.2  # Slightly exponential for better control

# Control Reverb Decay (Pad 14)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 14

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 21  # Learned in Ableton
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "Linear"
input_min = 0
input_max = 127
output_min = 20
output_max = 100  # Limited range (20-100% decay)
```

## Complete Transport & Mixer Control

Here's a comprehensive Ableton Live control profile:

```toml
[[modes]]
name = "Ableton Live"
color = "orange"

# ===== Transport Controls =====

# Play/Stop (Pad 1)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 10  # Mapped via MIDI Learn
value = 127
channel = 0

# Record (Pad 2)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 2

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 11  # Mapped via MIDI Learn
value = 127
channel = 0

# ===== Mixer Controls =====

# Track 1 Volume (Pad 9, velocity-sensitive)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 9

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 20  # Mapped via MIDI Learn to Track 1 Volume
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "Curve"
input_min = 0
input_max = 127
output_min = 0
output_max = 127
curve = 1.5  # Exponential for fader feel

# Track 2 Volume (Pad 10)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 10

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 21  # Mapped to Track 2 Volume
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "Curve"
input_min = 0
input_max = 127
output_min = 0
output_max = 127
curve = 1.5

# ===== Clip Launching =====

# Launch Clip Slot 1 (Pad 13)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 13

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 0  # Clip slot 1 in Track 1
velocity = 127
channel = 0

# Launch Clip Slot 2 (Pad 14)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 14

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 1  # Clip slot 2
velocity = 127
channel = 0
```

## Advanced: Drum Rack Control

Ableton's Drum Rack is a powerful sampler for drum sounds. Each pad in a Drum Rack can be triggered via MIDI notes.

### Default Drum Rack Mapping

Drum Racks use notes 36-99 (C1-D#7) for the 64 pads:
- **C1 (36)**: Kick drum (top-left pad)
- **C#1 (37)**: Next pad
- **D1 (38)**: Snare drum (often)
- **F#1 (42)**: Closed hi-hat (often)
- **A#1 (46)**: Open hi-hat (often)

### Example: Trigger Drum Rack Sounds

```toml
# Kick Drum (Pad 9)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 9

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 36  # C1 - Kick
channel = 0  # Track with Drum Rack

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"  # Use controller velocity

# Snare Drum (Pad 10)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 10

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 38  # D1 - Snare
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"

# Closed Hi-Hat (Pad 11)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 11

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 42  # F#1 - Closed Hi-Hat
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"
```

## Push-Style Session Navigation (Advanced)

Ableton Push controllers use a grid layout for clips. You can replicate this with Conductor:

```toml
# 4x4 Clip Grid (Pads 9-12, 13-16, 17-20, 21-24)
# Track 1, Clip Slots 1-4
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 9

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.ableton.live"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 0  # Track 1, Clip 1
velocity = 127
channel = 0

# Continue for all 16 pads...
# (Pad 10 → Track 1 Clip 2, Pad 13 → Track 2 Clip 1, etc.)
```

**Mapping Strategy**:
- Row 1 (Pads 9-12): Track 1, Clips 1-4
- Row 2 (Pads 13-16): Track 2, Clips 1-4
- Row 3 (Pads 17-20): Track 3, Clips 1-4
- Row 4 (Pads 21-24): Track 4, Clips 1-4

## Troubleshooting

### MIDI Port Not Appearing in Ableton

**Problem**: Virtual MIDI port doesn't show up in Ableton's Preferences.

**Solution**:
- **macOS**: Verify IAC Driver is enabled in Audio MIDI Setup
- **Windows**: Ensure loopMIDI is running in the background
- **Linux**: Check `aconnect -l` to verify virtual port exists
- Restart Ableton Live after creating the virtual port

### Messages Sent But No Response

**Problem**: Conductor sends MIDI but Ableton doesn't respond.

**Solution**:
1. Verify **Track** and **Remote** are enabled for the MIDI port in Preferences
2. Ensure the correct MIDI channel is used (usually Channel 0)
3. Use **MIDI Map Mode** in Ableton to verify what MIDI messages are being received
4. Check Conductor logs: `conductor --verbose`

### Clip Launch Notes Don't Match

**Problem**: Sending Note 0 doesn't launch the expected clip.

**Solution**:
1. Enter **MIDI Map Mode** in Ableton
2. Click the clip you want to trigger
3. Press the pad on your controller to learn the note
4. Update your Conductor config with the correct note number

### Windows: Port Name Issues

**Problem**: Port name "loopMIDI Port" not found.

**Solution**:
1. Check the exact port name in loopMIDI application
2. Update Conductor config to match the exact name (case-sensitive)
3. Ensure loopMIDI is running before starting Conductor daemon

### Latency Issues

**Problem**: Noticeable delay between pad press and Ableton response.

**Solution**:
1. Lower Ableton's audio buffer size: **Preferences → Audio → Buffer Size**
2. Use native virtual MIDI drivers (IAC on macOS, ALSA on Linux) instead of third-party apps
3. Close unnecessary applications to reduce CPU load
4. On Windows, ensure loopMIDI is up to date

## Complete Example Profile

Here's a production-ready Ableton Live control profile:

Download: [ableton-live-complete.toml](../downloads/ableton-live-complete.toml) (coming soon)

**Features**:
- Transport controls (Play, Stop, Record)
- 8 clip launchers (2 tracks × 4 clips)
- 4 device parameter controls (velocity-sensitive)
- Drum Rack triggering (4 sounds with velocity)
- AppFrontmost conditional (only works when Ableton is active)
- Velocity curves for expressive control

## Next Steps

- **[Logic Pro Integration](logic-pro.md)** - Control Logic Pro with Conductor
- **[MIDI Output Troubleshooting](../troubleshooting/midi-output.md)** - Advanced debugging
- **[DAW Control Guide](../guides/daw-control.md)** - General DAW control concepts

## Further Reading

- [Ableton Live Manual - MIDI](https://www.ableton.com/en/manual/midi-and-key-remote-control/)
- [Ableton Live MIDI Mapping](https://www.ableton.com/en/manual/midi-and-key-remote-control/#midi-mapping)
- [Creating Custom Control Surfaces](https://help.ableton.com/hc/en-us/articles/209072409-Creating-custom-control-surfaces)
