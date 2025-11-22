# SendMIDI Action Guide

**Version**: 2.1.0
**Status**: Backend Complete, GUI Pending
**Last Updated**: 2025-01-17

## Overview

The `SendMIDI` action allows MIDIMon to send MIDI messages to virtual or physical MIDI output ports. This enables powerful use cases like:

- **DAW Control**: Trigger instruments, effects, and automation in Ableton, Logic, FL Studio, etc.
- **Hardware Synth Control**: Send notes, CC messages, and program changes to external synthesizers
- **MIDI Routing**: Route MIDI from your controller to multiple destinations
- **Creative Workflows**: Build complex MIDI sequences, arpeggios, and generative patterns

---

## Quick Start

###1. Setup a Virtual MIDI Port (macOS)

```bash
# macOS: Use IAC Driver (built-in)
# 1. Open Audio MIDI Setup app
# 2. Window → Show MIDI Studio
# 3. Double-click "IAC Driver"
# 4. Check "Device is online"
# 5. Add a port (e.g., "IAC Driver Bus 1")
```

### 2. Basic Configuration

```toml
# config.toml
[[modes.mappings]]
description = "Send MIDI Note to DAW"

[modes.mappings.trigger]
type = "Note"
note = 36  # Pad 1

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60        # Middle C
velocity = 100   # Medium-hard hit
```

### 3. Test It

```bash
# Start the daemon
cargo run --release

# In your DAW:
# - Add a virtual instrument on MIDI channel 1
# - Set input to "IAC Driver Bus 1"
# - Press pad 1 on your controller → hear the note!
```

---

## Message Types

The SendMIDI action supports all standard MIDI message types:

| Message Type | Description | Use Case |
|--------------|-------------|----------|
| `NoteOn` | Trigger a note | Play instruments, trigger samples |
| `NoteOff` | Release a note | Stop sustained notes |
| `CC` (ControlChange) | Send continuous controller | Control volume, pan, effects |
| `ProgramChange` | Change instrument preset | Switch patches on synths |
| `PitchBend` | Pitch wheel movement | Pitch slides, vibrato |
| `Aftertouch` | Channel pressure | Expression, filter sweeps |

---

## Configuration Reference

### SendMIDI Action Parameters

```toml
[action]
type = "SendMidi"
port = "PORT_NAME"              # Required: MIDI output port name
message_type = "MESSAGE_TYPE"   # Required: See message types below
channel = 0                     # Required: MIDI channel (0-15)

# Optional parameters (depending on message type):
note = 60          # Note number (0-127) for NoteOn/NoteOff
velocity = 100     # Note velocity (0-127) for NoteOn/NoteOff
controller = 7     # Controller number (0-127) for CC messages
value = 64         # Controller value (0-127) for CC messages
program = 42       # Program number (0-127) for ProgramChange
pitch = 0          # Pitch bend value (-8192 to +8191)
pressure = 80      # Aftertouch pressure (0-127)
```

### Message Type Aliases

The `message_type` field accepts various string formats:

| Standard | Aliases |
|----------|---------|
| `NoteOn` | `noteon`, `note_on`, `note-on` |
| `NoteOff` | `noteoff`, `note_off`, `note-off` |
| `ControlChange` | `CC`, `cc`, `control_change`, `control-change` |
| `ProgramChange` | `PC`, `pc`, `program_change`, `program-change` |
| `PitchBend` | `PB`, `pb`, `pitch_bend`, `pitch-bend` |
| `Aftertouch` | `AT`, `at` |

---

## Examples

### 1. Trigger a Drum Sample

```toml
[[modes.mappings]]
description = "Trigger kick drum in Ableton"

[modes.mappings.trigger]
type = "Note"
note = 36  # Pad 1

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 9  # MIDI drums are usually on channel 10 (0-indexed = 9)
note = 36    # GM Kick Drum
velocity = 127
```

### 2. Control Volume with Encoder

```toml
[[modes.mappings]]
description = "Control track volume with encoder"

[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "CC"
channel = 0
controller = 7   # CC 7 = Volume
value = 127      # Max volume
```

### 3. Change Synth Preset

```toml
[[modes.mappings]]
description = "Switch to preset 42"

[modes.mappings.trigger]
type = "Note"
note = 40

[modes.mappings.action]
type = "SendMidi"
port = "Virtual Synth"
message_type = "ProgramChange"
channel = 0
program = 42
```

### 4. Pitch Bend Sweep

```toml
[[modes.mappings]]
description = "Pitch bend up"

[modes.mappings.trigger]
type = "Note"
note = 41

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "PitchBend"
channel = 0
pitch = 8191  # Maximum pitch bend up
```

### 5. Send Note Sequence (Arpeggio)

```toml
[[modes.mappings]]
description = "Play C major arpeggio"

[modes.mappings.trigger]
type = "Note"
note = 42

[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60, velocity = 100 },
    { type = "Delay", ms = 150 },
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 64, velocity = 100 },
    { type = "Delay", ms = 150 },
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 67, velocity = 100 },
    { type = "Delay", ms = 150 },
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 72, velocity = 100 },
]
```

### 6. Velocity-Sensitive Triggering

```toml
[[modes.mappings]]
description = "Send MIDI with controller velocity"

[modes.mappings.trigger]
type = "VelocityRange"
note = 36
soft_max = 40
medium_max = 80

[modes.mappings.action]
type = "Sequence"
actions = [
    # Soft hit → send quiet note
    { type = "Conditional", condition = "Always", then_action = { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60, velocity = 30 } },
]
```

### 7. Repeated CC Messages

```toml
[[modes.mappings]]
description = "Gradually increase filter cutoff"

[modes.mappings.trigger]
type = "Note"
note = 43

[modes.mappings.action]
type = "Repeat"
count = 10
delay_ms = 50
action = { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "CC", channel = 0, controller = 74, value = 127 }
```

---

## Platform-Specific Setup

### macOS

**Virtual MIDI Ports** (recommended):
1. Use the built-in IAC Driver (see Quick Start above)
2. Ports are created via Audio MIDI Setup app
3. No additional software needed

**Physical MIDI Ports**:
- USB MIDI devices appear automatically
- Use `list_output_ports()` to see available devices

### Linux

**Virtual MIDI Ports**:
```bash
# Option 1: ALSA (built-in)
sudo modprobe snd-virmidi

# Option 2: JACK
jack_control start
a2jmidid -e &
```

**Physical MIDI Ports**:
- USB MIDI devices work via ALSA
- May need to add user to `audio` group: `sudo usermod -a -G audio $USER`

### Windows

**Virtual MIDI Ports**:
Windows does not have built-in virtual MIDI support. Use third-party drivers:

- **loopMIDI** (recommended, free): https://www.tobias-erichsen.de/software/loopmidi.html
- **MIDI Yoke**: Alternative virtual MIDI driver

**Physical MIDI Ports**:
- USB MIDI devices work automatically with Windows drivers

---

## Common Use Cases

### DAW Control

#### Ableton Live
```toml
# Map pads to clip launch
[[modes.mappings]]
description = "Launch clip in slot 1"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 0  # Ableton clip grid starts at note 0
velocity = 127
```

#### Logic Pro
```toml
# Trigger software instruments
[[modes.mappings]]
description = "Play Logic instrument"
[modes.mappings.trigger]
type = "Note"
note = 37
[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60
velocity = 100
```

#### FL Studio
```toml
# Control mixer volume
[[modes.mappings]]
description = "Mixer track 1 volume"
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1
[modes.mappings.action]
type = "SendMidi"
port = "loopMIDI Port"  # Windows virtual port
message_type = "CC"
channel = 0
controller = 7
value = 100
```

### Hardware Synth Control

```toml
# Send notes to external synth
[[modes.mappings]]
description = "Play note on Moog"
[modes.mappings.trigger]
type = "Note"
note = 48
[modes.mappings.action]
type = "SendMidi"
port = "Moog Subsequent 37"  # USB MIDI device name
message_type = "NoteOn"
channel = 0
note = 48
velocity = 100
```

### MIDI Routing

```toml
# Route one controller to multiple destinations
[[modes.mappings]]
description = "Multi-output routing"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60, velocity = 100 },
    { type = "SendMidi", port = "Virtual Synth A", message_type = "NoteOn", channel = 1, note = 64, velocity = 100 },
    { type = "SendMidi", port = "Virtual Synth B", message_type = "NoteOn", channel = 2, note = 67, velocity = 100 },
]
```

---

## Troubleshooting

### MIDI Messages Not Received

**Check port connection**:
```bash
# macOS: List available ports
# Run MIDIMon with debug logging
DEBUG=1 cargo run --release
```

**Verify DAW MIDI settings**:
- Check that the DAW is listening to the correct virtual port
- Ensure MIDI channel matches (0-15 in config = 1-16 in DAW UI)
- Enable MIDI input monitoring in the DAW

**Test with MIDI Monitor**:
- macOS: Download "MIDI Monitor" app from Snoize
- Linux: Use `aseqdump` or `amidi -l`
- Windows: Use "MIDI-OX"

### Port Not Found Error

```
Error: Failed to send MIDI message to 'IAC Driver Bus 1': Port not found
```

**Solutions**:
1. Check port name spelling (case-sensitive!)
2. Verify port exists in Audio MIDI Setup (macOS) or `aconnect -o` (Linux)
3. Restart MIDIMon daemon after creating new virtual ports

### Wrong Notes Being Triggered

**Check MIDI channel**:
- Config uses 0-indexed channels (0-15)
- DAW UI uses 1-indexed channels (1-16)
- Channel 0 in config = Channel 1 in DAW

**Check note numbers**:
- MIDI notes range from 0-127
- Middle C (C4) = 60
- Use a MIDI monitor to verify outgoing notes

### Latency Issues

**Reduce message queue latency**:
- Use `send_message()` directly instead of `queue_message()`
- Minimize `Delay` values in sequences
- Check DAW buffer settings

---

## Advanced Techniques

### Dynamic Velocity

```toml
# Map pad velocity to MIDI velocity
[[modes.mappings]]
description = "Velocity-sensitive note"
[modes.mappings.trigger]
type = "VelocityRange"
note = 36
[modes.mappings.action]
type = "Conditional"
condition = "Always"
then_action = {
    type = "SendMidi",
    port = "IAC Driver Bus 1",
    message_type = "NoteOn",
    channel = 0,
    note = 60,
    velocity = 100  # TODO: v2.2 will support variable velocity
}
```

### Chord Stacking

```toml
# Play full chord on single pad press
[[modes.mappings]]
description = "C major chord"
[modes.mappings.trigger]
type = "Note"
note = 48
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60, velocity = 100 },  # C
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 64, velocity = 100 },  # E
    { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 67, velocity = 100 },  # G
]
```

### CC Ramping

```toml
# Gradually increase parameter value
[[modes.mappings]]
description = "Fade in filter cutoff"
[modes.mappings.trigger]
type = "Note"
note = 50
[modes.mappings.action]
type = "Repeat"
count = 20
delay_ms = 25
action = {
    type = "SendMidi",
    port = "IAC Driver Bus 1",
    message_type = "CC",
    channel = 0,
    controller = 74,  # Filter cutoff
    value = 127  # TODO: v2.2 will support incremental values
}
```

---

## MIDI Reference

### Common CC Numbers

| CC# | Parameter | Description |
|-----|-----------|-------------|
| 1 | Modulation | Vibrato depth |
| 7 | Volume | Track/channel volume |
| 10 | Pan | Stereo position |
| 11 | Expression | Secondary volume |
| 64 | Sustain Pedal | On/off (0-63 = off, 64-127 = on) |
| 71 | Resonance | Filter resonance |
| 74 | Cutoff | Filter cutoff frequency |
| 91 | Reverb | Reverb send level |
| 93 | Chorus | Chorus send level |

### General MIDI Drum Map (Channel 10)

| Note | Drum Sound |
|------|------------|
| 35 | Acoustic Bass Drum |
| 36 | Bass Drum 1 |
| 38 | Acoustic Snare |
| 42 | Closed Hi-Hat |
| 44 | Pedal Hi-Hat |
| 46 | Open Hi-Hat |
| 49 | Crash Cymbal 1 |
| 51 | Ride Cymbal 1 |

### Note Number Reference

| Note | Number | Octave |
|------|--------|--------|
| C-2 | 0 | Lowest MIDI note |
| C-1 | 12 | |
| C0 | 24 | |
| C1 | 36 | |
| C2 | 48 | |
| C3 | 60 | Middle C (C4 in some DAWs) |
| C4 | 72 | |
| C5 | 84 | |
| C6 | 96 | |
| C7 | 108 | |
| G8 | 127 | Highest MIDI note |

---

## Future Enhancements (v2.2+)

Planned improvements for SendMIDI:

- **Variable Velocity**: Map trigger velocity to MIDI velocity dynamically
- **CC Value Mapping**: Map encoder rotation to incremental CC values
- **Note Duration**: Auto-send NoteOff after specified duration
- **MIDI Learn**: Click-and-press to auto-configure mappings
- **SysEx Support**: Send system-exclusive messages
- **MIDI Clock**: Send timing/sync messages
- **Channel Aftertouch**: Polyphonic aftertouch support

---

## See Also

- [Configuration Guide](configuration/README.md)
- [Action Types Reference](configuration/actions.md)
- [Trigger Types Reference](configuration/triggers.md)
- [v2.1 Virtual MIDI Output Design](v2.1-virtual-midi-output-design.md)

---

## License

Copyright 2025 Amiable
SPDX-License-Identifier: MIT
