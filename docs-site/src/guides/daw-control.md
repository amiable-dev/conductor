# DAW Control with Conductor

Control your Digital Audio Workstation (DAW) directly from your MIDI controller using Conductor's SendMIDI action. Transform your Maschine Mikro MK3 or other MIDI controller into a custom control surface for transport control, mixer automation, and parameter manipulation.

---

## Overview

Conductor can send MIDI messages to your DAW, allowing you to:

- **Transport Control**: Play, Stop, Record, Rewind, Fast Forward
- **Mixer Control**: Volume faders (CC 7), Pan controls (CC 10), Mute/Solo
- **Parameter Automation**: Plugin parameters, effect sends, EQ controls
- **MIDI Learn**: Map any pad to any DAW parameter using MIDI Learn
- **Scene/Clip Triggering**: Launch clips, scenes, or patterns in Live/Bitwig

This enables you to create custom control surfaces tailored to your workflow, without needing hardware-specific DAW integration.

---

## How It Works

```
[Your MIDI Controller]
        ↓ (MIDI Input to Conductor)
  [Conductor Daemon]
        ↓ (SendMIDI Action)
  [Virtual MIDI Port] (IAC Driver, loopMIDI, ALSA)
        ↓ (MIDI from Conductor)
    [Your DAW] (Logic Pro, Ableton Live, etc.)
```

**Key Components**:

1. **Input**: Conductor receives MIDI from your controller (pads, encoders, buttons)
2. **Mapping**: Conductor maps input events to SendMIDI actions
3. **Output**: SendMIDI sends MIDI messages to a virtual MIDI port
4. **DAW**: Your DAW receives MIDI from the virtual port and responds

---

## Platform Setup

Before using SendMIDI, you need a virtual MIDI port that Conductor can send to and your DAW can receive from.

### macOS: IAC Driver

macOS includes a built-in virtual MIDI driver called IAC (Inter-Application Communication).

**Setup Steps**:

1. Open **Audio MIDI Setup** application
   - Found in `/Applications/Utilities/Audio MIDI Setup.app`
   - Or press `Cmd+Space` and search "Audio MIDI Setup"

2. Show MIDI Studio
   - Go to `Window` → `Show MIDI Studio` (or press `Cmd+2`)

3. Double-click **IAC Driver** icon
   - If you don't see it, create it: `Window` → `Show MIDI Studio` → click the globe icon

4. Enable the driver
   - Check **"Device is online"** checkbox
   - You should see at least one port named "IAC Driver Bus 1"

5. (Optional) Add more ports
   - Click the **+** button under "Ports" to create additional buses
   - Rename buses to something meaningful (e.g., "Conductor → Logic Pro")

6. Click **Apply**

**Verify IAC Driver is Working**:

```bash
# List MIDI output ports (Conductor should see IAC Driver)
./target/release/conductor-daemon --list-midi-outputs

# You should see:
# MIDI Output Ports:
# 0: IAC Driver Bus 1
```

**Troubleshooting**:
- If IAC Driver doesn't appear: Restart Audio MIDI Setup or reboot macOS
- If port disappears after reboot: Make sure "Device is online" was checked and saved

---

### Windows: loopMIDI

Windows doesn't include built-in virtual MIDI ports, so we use **loopMIDI** (free, open-source).

**Download & Install**:

1. Visit [Tobias Erichsen's loopMIDI page](https://www.tobias-erichsen.de/software/loopmidi.html)
2. Download the installer (free, no registration required)
3. Run the installer (requires Administrator privileges)
4. Launch **loopMIDI** from Start Menu

**Create Virtual Port**:

1. In loopMIDI window, enter a port name (e.g., "Conductor Virtual Out")
2. Click **+** (Plus button) to create the port
3. The port should appear in the list with status "Opened by 0 applications"

**Keep loopMIDI Running**:
- loopMIDI must be running for the virtual port to exist
- To auto-start with Windows: Right-click loopMIDI in system tray → "Start minimized with Windows"

**Verify loopMIDI Port**:

```powershell
# List MIDI output ports
.\target\release\conductor-daemon.exe --list-midi-outputs

# You should see:
# MIDI Output Ports:
# 0: Conductor Virtual Out
```

**Troubleshooting**:
- Port not found: Make sure loopMIDI is running and port is created
- Port opens but no MIDI received in DAW: Check DAW MIDI input preferences
- loopMIDI crashes: Try creating port with simpler name (no special characters)

---

### Linux: ALSA Virtual Port

Linux supports virtual MIDI ports through **ALSA** (Advanced Linux Sound Architecture).

**Create Virtual Port with `aconnect`**:

```bash
# Method 1: Using Conductor's built-in virtual port creation (recommended)
# Conductor can create virtual ports automatically on Linux via midir

# Method 2: Manual ALSA virtual port (if needed)
# Install ALSA utilities
sudo apt-get install alsa-utils

# Create a virtual port named "Conductor Output"
# This command creates a port that stays active
aseqdump -p "Conductor Output" &

# List ALSA MIDI ports
aconnect -l

# You should see something like:
# client 128: 'Conductor Output' [type=user]
#     0 'Conductor Output'
```

**Using JACK for MIDI (Alternative)**:

If you use JACK for audio:

```bash
# Install JACK MIDI tools
sudo apt-get install qjackctl

# Start JACK
qjackctl &

# In QjackCtl:
# - Go to "Graph" or "Patchbay"
# - Create MIDI connections between Conductor and your DAW
```

**Verify ALSA Port**:

```bash
# List MIDI output ports
./target/release/conductor-daemon --list-midi-outputs

# You should see your created virtual port
```

**Troubleshooting**:
- No ALSA ports: `sudo modprobe snd-seq` (load ALSA sequencer module)
- Permission denied: Add user to `audio` group: `sudo usermod -a -G audio $USER` (logout/login after)
- JACK vs ALSA confusion: Choose one system (ALSA is simpler for most users)

---

## Conductor Configuration

### Basic SendMIDI Example

Here's a minimal configuration that sends a MIDI Note On message when you press pad 1:

```toml
# config.toml

[[modes]]
name = "Default"

# Send MIDI Note On (C4 / note 60) when pad 1 is pressed
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1  # Pad 1 on Maschine Mikro MK3

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"  # macOS (use your port name)
# port = "Conductor Virtual Out"  # Windows (loopMIDI)
# port = "Conductor Output"  # Linux (ALSA)

[modes.mappings.action.message]
type = "NoteOn"
note = 60  # MIDI note 60 = C4 (Middle C)
velocity = 100
channel = 0  # MIDI channel 1 (0-indexed)
```

**Test This Configuration**:

1. Save the config above to your `config.toml`
2. Restart Conductor daemon: `conductorctl reload`
3. Open your DAW and create a software instrument track
4. Set the track's MIDI input to your virtual port (IAC Driver/loopMIDI)
5. Enable MIDI input recording on the track
6. Press pad 1 on your controller
7. You should hear the instrument play note C4!

---

## MIDI Message Types

Conductor supports all common MIDI message types. Here's a reference for each:

### 1. Note On / Note Off

Trigger notes in software instruments.

```toml
[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

# Note On: Start playing a note
[modes.mappings.action.message]
type = "NoteOn"
note = 60        # MIDI note number (0-127, 60 = C4)
velocity = 100   # How hard the note is played (0-127)
channel = 0      # MIDI channel (0-15, displays as 1-16 in DAWs)
```

```toml
# Note Off: Stop playing a note
[modes.mappings.action.message]
type = "NoteOff"
note = 60        # Same note number as the Note On
velocity = 64    # Release velocity (usually 0 or 64)
channel = 0
```

**Common Note Numbers**:
- C3 (48), C4 (60), C5 (72) - Middle octaves
- A0 (21) - Lowest note on 88-key piano
- C8 (108) - Highest note on 88-key piano

**Use Cases**:
- Trigger drum samples in drum racks
- Play melodies on virtual instruments
- Trigger clips in Ableton Live's Session View

---

### 2. Control Change (CC)

Control parameters like volume, pan, filters, and effects.

```toml
[modes.mappings.action.message]
type = "CC"
controller = 7   # Controller number (0-127)
value = 100      # Controller value (0-127)
channel = 0
```

**Common CC Numbers**:

| CC# | Name | Purpose | Range |
|-----|------|---------|-------|
| 1 | Modulation Wheel | Vibrato, tremolo | 0-127 |
| 7 | Volume | Track/channel volume | 0-127 (100=max) |
| 10 | Pan | Left/right panning | 0=left, 64=center, 127=right |
| 11 | Expression | Volume changes within a note | 0-127 |
| 64 | Sustain Pedal | Hold notes | 0-63=off, 64-127=on |
| 71 | Filter Resonance | Synth filter resonance | 0-127 |
| 74 | Filter Cutoff | Synth filter cutoff freq | 0-127 |

**Example: Volume Control**

```toml
# Soft velocity = low volume (CC 7 = 40)
[[modes.mappings]]
[modes.mappings.trigger]
type = "VelocityRange"
note = 1
min_velocity = 0
max_velocity = 40

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 7
value = 40
channel = 0
```

---

### 3. Program Change

Switch between presets/patches in instruments or effects.

```toml
[modes.mappings.action.message]
type = "ProgramChange"
program = 42     # Program number (0-127)
channel = 0
```

**Use Cases**:
- Switch guitar amp presets
- Change synthesizer patches
- Load different drum kits

**Note**: Program Change numbers are 0-indexed in MIDI (0-127) but often displayed as 1-128 in DAWs.

---

### 4. Pitch Bend

Bend notes up or down (like a pitch wheel).

```toml
[modes.mappings.action.message]
type = "PitchBend"
value = 0        # Pitch bend value (-8192 to +8191)
                 # 0 = center (no bend)
                 # +8191 = max up
                 # -8192 = max down
channel = 0
```

**Examples**:

```toml
# Pitch up (full bend up)
value = 8191

# Center position (no bend)
value = 0

# Pitch down (full bend down)
value = -8192
```

**Use Cases**:
- Guitar bends in virtual guitars
- Synth lead pitch slides
- Trombone-style glissando effects

---

### 5. Aftertouch (Channel Pressure)

Apply pressure-based modulation to all notes on a channel.

```toml
[modes.mappings.action.message]
type = "Aftertouch"
pressure = 64    # Pressure amount (0-127)
channel = 0
```

**Use Cases**:
- Vibrato after note starts
- Filter modulation with sustained notes
- Expressive synth control

**Note**: This is *Channel Aftertouch* (affects all notes). MIDI also supports *Polyphonic Aftertouch* (per-note), but it's rarely used and not currently supported by SendMIDI.

---

## Common Use Cases

### Transport Control

Control DAW playback using MIDI CC messages. Most DAWs support MMC (MIDI Machine Control) or specific CC numbers for transport.

**Logic Pro Transport** (using CC messages):

```toml
# Play/Pause (CC 115)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 115  # Play/Continue
value = 127
channel = 0

# Stop (CC 116)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 2

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 116  # Stop
value = 127
channel = 0

# Record (CC 117)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 3

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 117  # Record
value = 127
channel = 0
```

**Ableton Live Transport** (using Note messages):

Ableton responds to specific MIDI notes for transport in some modes:

```toml
# Play (Note 91 on Channel 1)
[modes.mappings.action.message]
type = "NoteOn"
note = 91
velocity = 127
channel = 0

# Stop (Note 93)
[modes.mappings.action.message]
type = "NoteOn"
note = 93
velocity = 127
channel = 0
```

*Note: Actual values depend on your DAW's MIDI remote script. Check your DAW's documentation or use MIDI Learn.*

---

### Mixer Control (Volume & Pan)

Control track volume and panning using CC 7 (Volume) and CC 10 (Pan).

**Volume Fader with Velocity**:

```toml
# Map pad velocity to track volume
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 7    # Volume
value = 100       # Will be overridden by velocity mapping
channel = 0       # Track 1

# Use velocity mapping to scale velocity to volume
[modes.mappings.action.velocity_mapping]
type = "Linear"
min = 0           # Soft hit = volume 0
max = 127         # Hard hit = volume 127
```

**Pan Control**:

```toml
# Encoder left/right = pan left/right
[[modes.mappings]]
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 1
direction = "Clockwise"

[modes.mappings.action]
type = "SendMIDI"
port = "IAC Driver Bus 1"

[modes.mappings.action.message]
type = "CC"
controller = 10   # Pan
value = 127       # Full right
channel = 0
```

---

### MIDI Learn in Your DAW

Most DAWs support **MIDI Learn** - a feature that lets you map any incoming MIDI message to any parameter by clicking the parameter and moving your controller.

**General MIDI Learn Workflow**:

1. **Configure Conductor** to send a MIDI message when you press a pad:
   ```toml
   [[modes.mappings]]
   [modes.mappings.trigger]
   type = "Note"
   note = 1

   [modes.mappings.action]
   type = "SendMIDI"
   port = "IAC Driver Bus 1"

   [modes.mappings.action.message]
   type = "CC"
   controller = 20  # Arbitrary CC number
   value = 127
   channel = 0
   ```

2. **In your DAW**:
   - Enable MIDI Learn mode (varies by DAW)
   - Click the parameter you want to control (e.g., filter cutoff)
   - Press your mapped pad
   - DAW learns the association (CC 20 → filter cutoff)
   - Exit MIDI Learn mode

3. **Test**: Press the pad again - the parameter should move!

**DAW-Specific MIDI Learn**:

- **Logic Pro**: `Cmd+L` → Click parameter → Move controller → `Cmd+L` to exit
- **Ableton Live**: Click "MIDI" button in top right → Click parameter → Move controller → Click "MIDI" again
- **Reaper**: Right-click parameter → "Learn" → Move controller
- **FL Studio**: Right-click parameter → "Link to controller" → Move controller

---

## Combining with Conditionals

Use Conditional actions to send different MIDI messages based on context (time, active app, mode).

**Example: Different Transport Controls for Different DAWs**

```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.action]
type = "Conditional"

# If Logic Pro is frontmost, send CC 115 (Play)
[[modes.mappings.action.conditions]]
type = "AppFrontmost"
bundle_id = "com.apple.logic10"

[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"
message = { type = "CC", controller = 115, value = 127, channel = 0 }

# Otherwise (Ableton), send Note 91
[modes.mappings.action.else_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"
message = { type = "NoteOn", note = 91, velocity = 127, channel = 0 }
```

---

## Troubleshooting

### MIDI Messages Not Received in DAW

**Check Conductor is sending**:

```bash
# Enable debug logging
DEBUG=1 conductorctl reload

# Press your mapped pad
# You should see: "Sending MIDI: [0x90, 0x3C, 0x64] to port 'IAC Driver Bus 1'"
```

**Check DAW MIDI input settings**:

1. Open DAW preferences/settings
2. Go to MIDI Input settings
3. Ensure your virtual port is enabled:
   - Logic Pro: `Preferences` → `MIDI` → `Inputs` → Enable "IAC Driver Bus 1"
   - Ableton Live: `Preferences` → `Link/Tempo/MIDI` → `MIDI Ports` → Enable "Track" and "Remote" for IAC Driver
   - Reaper: `Preferences` → `MIDI Devices` → Enable input for virtual port

**Check track MIDI input**:

- Make sure the track is set to receive MIDI from your virtual port
- Enable "Input Monitoring" or "Record Enable" on the track

---

### High Latency / Delayed Response

**Symptoms**: MIDI messages arrive 50-500ms late

**Causes**:
1. **DAW Buffer Size**: Larger audio buffers = more latency
   - Solution: Reduce buffer size in DAW audio preferences (e.g., 128 or 256 samples)

2. **MIDI Port Polling**: Some virtual MIDI implementations poll slowly
   - Solution: Restart DAW and Conductor daemon

3. **System Load**: High CPU usage delays MIDI processing
   - Solution: Close unnecessary applications

**Test Latency**:

```bash
# Send test Note On and measure response time in DAW
./target/release/conductorctl send-test-midi
```

---

### Port Not Found

**Error**: `Port 'IAC Driver Bus 1' is not connected`

**Solutions**:

1. **List available ports**:
   ```bash
   conductorctl list-midi-outputs
   ```

2. **Check port name matches exactly** (case-sensitive):
   ```toml
   # Wrong:
   port = "iac driver bus 1"

   # Correct:
   port = "IAC Driver Bus 1"
   ```

3. **macOS**: Verify IAC Driver is online in Audio MIDI Setup

4. **Windows**: Ensure loopMIDI is running and port is created

5. **Linux**: Check ALSA port exists with `aconnect -l`

---

### Wrong MIDI Channel

**Symptom**: MIDI messages sent but DAW doesn't respond, or wrong track responds

**Solution**: Check MIDI channel configuration

```toml
# MIDI channels are 0-indexed in Conductor config (0-15)
# But displayed as 1-16 in DAWs

# Channel 0 in Conductor = Channel 1 in DAW
[modes.mappings.action.message]
channel = 0  # This is MIDI channel 1

# Channel 15 in Conductor = Channel 16 in DAW
channel = 15  # This is MIDI channel 16
```

**Check DAW track MIDI input channel**:
- Set track to receive from "All Channels" or specific channel that matches your config

---

## Best Practices

### 1. Use Descriptive Port Names

```toml
# Instead of generic names:
port = "IAC Driver Bus 1"

# Use descriptive names (rename in Audio MIDI Setup):
port = "Conductor → Logic Pro"
port = "Conductor → Ableton"
```

### 2. Organize Mappings by Function

Group related controls together in your config:

```toml
# Transport Controls
[[modes.mappings]]
# ... Play mapping ...

[[modes.mappings]]
# ... Stop mapping ...

# Mixer Controls
[[modes.mappings]]
# ... Volume mapping ...

[[modes.mappings]]
# ... Pan mapping ...
```

### 3. Document Your CC Assignments

```toml
# CC 20-29: Filter controls
[[modes.mappings]]
[modes.mappings.action.message]
type = "CC"
controller = 20  # Filter Cutoff
value = 64
channel = 0

# CC 30-39: Effect sends
[[modes.mappings]]
[modes.mappings.action.message]
type = "CC"
controller = 30  # Reverb Send
value = 50
channel = 0
```

### 4. Use Velocity Mapping for Expressive Control

```toml
# Map pad velocity to filter cutoff for expressive control
[modes.mappings.action.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.5  # Gentle curve for nuanced control
```

### 5. Test Incrementally

- Add one mapping at a time
- Test each mapping before adding more
- Use `DEBUG=1` to verify MIDI messages are sent correctly

---

## Next Steps

- **Platform-Specific Guides**: See detailed examples for [Logic Pro](../examples/logic-pro.md) and [Ableton Live](../examples/ableton-live.md)
- **Advanced Patterns**: Learn about [context-aware mappings](context-aware.md) with conditionals
- **Velocity Control**: Explore [velocity curves](velocity-curves.md) for expressive MIDI control
- **Troubleshooting**: Check [MIDI Output Troubleshooting](../troubleshooting/midi-output.md) for common issues

---

**Ready to control your DAW?** Start with the basic transport control example above, then customize to your workflow!
