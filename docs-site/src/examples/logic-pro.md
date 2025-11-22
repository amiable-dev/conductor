# Logic Pro Integration

This guide shows you how to set up Conductor to control Logic Pro, Apple's professional digital audio workstation. We'll cover transport controls, mixer automation, and smart control integration.

## Prerequisites

- macOS (Logic Pro is macOS-only)
- Conductor v2.1.0 or later
- Logic Pro 10.5 or later
- IAC Driver configured (see [DAW Control Guide](../guides/daw-control.md#macos-iac-driver))

## Quick Start

### 1. Enable IAC Driver

First, ensure your IAC Driver is enabled:

1. Open **Audio MIDI Setup** (Applications → Utilities)
2. Show MIDI Studio (Window → Show MIDI Studio)
3. Double-click **IAC Driver**
4. Check **"Device is online"**
5. Ensure at least one bus exists (default: "IAC Driver Bus 1")
6. Click **Apply**

### 2. Configure Logic Pro MIDI Input

In Logic Pro:

1. Open **Logic Pro → Settings → MIDI → Inputs** (or press ⌘,)
2. Locate **IAC Driver Bus 1** in the device list
3. Check the box to enable it
4. Close Settings

### 3. Create a Basic Transport Control Profile

Create or update your `~/.config/conductor/config.toml`:

```toml
[[modes]]
name = "Logic Pro"
color = "purple"

# Play/Pause (Pad 1)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 115  # Logic Pro Play/Pause CC
value = 127
channel = 0

# Stop (Pad 2)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 2

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 116  # Logic Pro Stop CC
value = 127
channel = 0

# Record (Pad 3)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 3

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 117  # Logic Pro Record CC
value = 127
channel = 0
```

### 4. Set Up Logic Pro Control Surface

In Logic Pro:

1. Open **Logic Pro → Control Surfaces → Setup**
2. Click **New → Install...** (or press ⌘N)
3. Choose **Mackie Control** or **Generic** from the list
4. In the device settings:
   - **Input**: Select **IAC Driver Bus 1**
   - **Output**: (Optional - for feedback)
5. Click **Apply** or **OK**

### 5. Test the Configuration

1. Start the Conductor daemon: `conductor`
2. Open a Logic Pro project
3. Press Pad 1 on your controller → Logic should Play/Pause
4. Press Pad 2 → Logic should Stop
5. Press Pad 3 → Logic should start Recording

## Logic Pro MIDI CC Mapping Reference

Logic Pro uses specific CC numbers for different transport and control functions:

| Function | CC Number | Value Range | Notes |
|----------|-----------|-------------|-------|
| Play/Pause | 115 | 127 | Toggle transport |
| Stop | 116 | 127 | Stop playback |
| Record | 117 | 127 | Toggle record |
| Rewind | 118 | 127 | Skip backward |
| Fast Forward | 119 | 127 | Skip forward |
| Cycle | 120 | 127 | Toggle cycle mode |
| Volume (Track 1) | 7 | 0-127 | Channel 0 |
| Pan (Track 1) | 10 | 0-127 | Channel 0 |
| Volume (Track 2) | 7 | 0-127 | Channel 1 |
| Pan (Track 2) | 10 | 0-127 | Channel 1 |

**Note**: Track-specific controls use different MIDI channels (Track 1 = Channel 0, Track 2 = Channel 1, etc.)

## Complete Transport Control Profile

Here's a full-featured transport control profile for Logic Pro:

```toml
[[modes]]
name = "Logic Pro"
color = "purple"

# ===== Transport Controls =====

# Play/Pause (Pad 1)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 115
value = 127
channel = 0

# Stop (Pad 2)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 2

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 116
value = 127
channel = 0

# Record (Pad 3)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 3

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 117
value = 127
channel = 0

# Rewind (Pad 4)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 4

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 118
value = 127
channel = 0

# Fast Forward (Pad 5)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 5

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 119
value = 127
channel = 0

# Cycle On/Off (Pad 6)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 6

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 120
value = 127
channel = 0
```

## Mixer Control with Velocity Curves

Use velocity-sensitive pads to control track volume with smooth curves:

```toml
# Track 1 Volume (Pad 9, velocity-sensitive)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 9

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 7  # Volume
channel = 0     # Track 1

[modes.mappings.action.then_action.message.velocity_curve]
type = "Curve"
input_min = 0
input_max = 127
output_min = 0
output_max = 127
curve = 1.5  # Slightly exponential for better fader control

# Track 2 Volume (Pad 10)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 10

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 7
channel = 1     # Track 2

[modes.mappings.action.then_action.message.velocity_curve]
type = "Curve"
input_min = 0
input_max = 127
output_min = 0
output_max = 127
curve = 1.5
```

## Smart Controls Integration

Logic Pro's Smart Controls can be MIDI-mapped for parameter automation:

### Step 1: Map Smart Controls in Logic Pro

1. Open a Logic Pro project with a software instrument
2. Show **Smart Controls** (press B or View → Show Smart Controls)
3. Click **Learn** button in Smart Controls header
4. Press a pad on your MIDI controller
5. Logic will assign that CC to the Smart Control knob
6. Repeat for up to 8 Smart Control parameters

### Step 2: Configure Conductor to Send Smart Control CCs

```toml
# Smart Control 1 (Pad 13, velocity controls CC value)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 13

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 71  # Smart Control 1 (default)
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"  # Direct velocity → CC value mapping

# Smart Control 2 (Pad 14)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 14

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 72  # Smart Control 2
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"
```

**Default Smart Control CC Numbers**:
- Smart Control 1: CC 71
- Smart Control 2: CC 72
- Smart Control 3: CC 73
- Smart Control 4: CC 74
- Smart Control 5: CC 75
- Smart Control 6: CC 76
- Smart Control 7: CC 77
- Smart Control 8: CC 78

## Advanced: Multi-Function Pads with Long Press

Use long press to access secondary functions on the same pad:

```toml
# Pad 1: Play (short press) / Stop (long press)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 115  # Play
value = 127
channel = 0

# Long press variant (Stop)
[[modes.mappings]]
[modes.mappings.trigger]
type = "LongPress"
note = 1
duration_ms = 800

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 116  # Stop
value = 127
channel = 0
```

## Using MIDI Learn in Logic Pro

Logic Pro has a built-in MIDI Learn feature for custom mappings:

### Step 1: Enter MIDI Learn Mode

1. In Logic Pro, go to **Logic Pro → Control Surfaces → Learn Assignment**
2. Click the parameter you want to control (e.g., a plugin knob, fader, button)
3. Press the pad on your MIDI controller
4. Logic will map that CC to the parameter
5. Press **Done** when finished

### Step 2: Configure Conductor

Once Logic has learned the CC mapping, configure Conductor to send that CC:

```toml
# Example: Plugin parameter learned as CC 20
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 15

[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.then_action.message]
type = "CC"
controller = 20  # Whatever Logic learned
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"
```

## Troubleshooting

### IAC Driver Not Appearing in Logic Pro

**Problem**: IAC Driver doesn't show up in Logic Pro's MIDI Inputs.

**Solution**:
1. Verify IAC Driver is enabled in Audio MIDI Setup
2. Restart Logic Pro after enabling IAC Driver
3. Check **Logic Pro → Settings → MIDI → Reset MIDI Drivers**

### Messages Sent But No Response

**Problem**: Conductor sends MIDI but Logic doesn't respond.

**Solution**:
1. Verify the control surface is configured in **Logic Pro → Control Surfaces → Setup**
2. Ensure the correct CC numbers for your Logic version (some changed in Logic Pro 10.5+)
3. Try using MIDI Learn to discover the correct CC numbers
4. Check MIDI channel (most Logic functions use Channel 0)

### Latency Issues

**Problem**: Noticeable delay between pad press and Logic response.

**Solution**:
1. Lower Logic Pro's I/O buffer size: **Logic Pro → Settings → Audio → I/O Buffer Size**
2. Use IAC Driver (lowest latency) instead of virtual MIDI apps
3. Reduce CPU load (freeze tracks, disable unused plugins)

### Control Surface Conflicts

**Problem**: Multiple control surfaces interfering with each other.

**Solution**:
1. In **Logic Pro → Control Surfaces → Setup**, disable unused control surfaces
2. Ensure IAC Driver is assigned to only one control surface
3. Use unique MIDI channels for different control surfaces

## Complete Example Profile

Here's a production-ready Logic Pro control profile with all features:

Download: [logic-pro-complete.toml](../downloads/logic-pro-complete.toml) (coming soon)

**Features**:
- Transport controls (Play, Stop, Record, Rewind, FF, Cycle)
- 4-track mixer (Volume + Pan per track)
- 8 Smart Control parameters
- Multi-function pads (short/long press)
- Velocity-sensitive mixer control
- AppFrontmost conditional (only works when Logic is active)

## Next Steps

- **[Ableton Live Integration](ableton-live.md)** - Control Ableton Live with Conductor
- **[MIDI Output Troubleshooting](../troubleshooting/midi-output.md)** - Advanced debugging
- **[DAW Control Guide](../guides/daw-control.md)** - General DAW control concepts

## Further Reading

- [Logic Pro User Guide - MIDI](https://support.apple.com/guide/logicpro/welcome/mac)
- [Logic Pro Control Surfaces](https://support.apple.com/guide/logicpro/control-surfaces-overview-lgcp2d83e0e9/mac)
- [IAC Driver Configuration](https://support.apple.com/guide/audio-midi-setup/transfer-midi-information-between-apps-ams1013/mac)
