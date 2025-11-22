# Trigger Types

Triggers define when an action should execute. Conductor supports a wide range of trigger types from simple note presses to complex patterns like chords and long presses.

## Core Triggers

### Note

Basic MIDI note on/off detection.

```toml
[trigger]
type = "Note"
note = 60  # Middle C
velocity_min = 1  # Optional: minimum velocity (default 1)
```

**Parameters:**
- `note` (integer): MIDI note number (0-127)
- `velocity_min` (integer, optional): Minimum velocity to trigger (default 1)

**Use Cases:**
- Simple pad presses
- Piano key detection
- Basic button mapping

### VelocityRange

Different actions based on how hard you press.

```toml
[trigger]
type = "VelocityRange"
note = 60
min_velocity = 80
max_velocity = 127
```

**Parameters:**
- `note` (integer): MIDI note number
- `min_velocity` (integer): Minimum velocity (0-127)
- `max_velocity` (integer): Maximum velocity (0-127)

**Velocity Levels:**
- **Soft**: 0-40
- **Medium**: 41-80
- **Hard**: 81-127

**Use Cases:**
- Soft press for play, hard press for stop
- Different shortcuts based on press intensity
- Expressive control mappings

### LongPress

Detect when a pad is held for a duration.

```toml
[trigger]
type = "LongPress"
note = 60
min_duration_ms = 1000  # Hold for 1 second
```

**Parameters:**
- `note` (integer): MIDI note number
- `min_duration_ms` (integer): Minimum hold duration in milliseconds (default 2000)

**Use Cases:**
- Hold for 2s to quit app (prevent accidental quits)
- Short tap for screenshot, long press for screen recording
- Confirmation for destructive actions

### DoubleTap

Detect quick double presses.

```toml
[trigger]
type = "DoubleTap"
note = 60
max_interval_ms = 300  # Optional: max time between taps
```

**Parameters:**
- `note` (integer): MIDI note number
- `max_interval_ms` (integer, optional): Maximum interval between taps (default 300ms)

**Use Cases:**
- Double-tap to toggle fullscreen
- Quick double-tap for emergency actions
- Gesture-based shortcuts

### NoteChord

Multiple notes pressed simultaneously.

```toml
[trigger]
type = "NoteChord"
notes = [60, 64, 67]  # C major chord
max_interval_ms = 100  # Optional
```

**Parameters:**
- `notes` (array): List of MIDI note numbers
- `max_interval_ms` (integer, optional): Maximum time between first and last note (default 100ms)

**Use Cases:**
- Emergency exit (press 3 corners simultaneously)
- Complex shortcuts requiring multiple pads
- Safety mechanisms for important actions

### CC (Control Change)

Continuous controller messages.

```toml
[trigger]
type = "CC"
cc = 1  # Modulation wheel
value_min = 64  # Optional: trigger only when value >= 64
```

**Parameters:**
- `cc` (integer): Control change number (0-127)
- `value_min` (integer, optional): Minimum value to trigger

**Use Cases:**
- Button presses sending CC messages
- Threshold-based triggers
- Binary on/off switches

### EncoderTurn

Encoder rotation with direction detection.

```toml
[trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"  # or "CounterClockwise"
```

**Parameters:**
- `cc` (integer): Control change number
- `direction` (string): "Clockwise" or "CounterClockwise"

**Use Cases:**
- Volume control with encoder
- Mode switching (clockwise = next, counter-clockwise = previous)
- Parameter adjustment

## Advanced Triggers

### Aftertouch

Channel pressure sensitivity (pressure after initial press).

```toml
[trigger]
type = "Aftertouch"
note = 1  # Optional: specific pad (omit for channel aftertouch)
min_pressure = 64  # Optional: minimum pressure
max_pressure = 127  # Optional: maximum pressure
```

**Parameters:**
- `note` (integer, optional): Specific pad for polyphonic aftertouch (omit for channel aftertouch)
- `min_pressure` (integer, optional): Minimum pressure value (0-127)
- `max_pressure` (integer, optional): Maximum pressure value (0-127)

**Aftertouch Types:**
- **Polyphonic (0xA0)**: Per-pad pressure (Maschine Mikro MK3, Launchpad Pro)
- **Channel (0xD0)**: Global pressure for entire device (most MIDI keyboards)

**Use Cases:**
- Apply pressure to modulate effects
- Pressure-sensitive volume control
- Expression control without additional hardware

**Device Compatibility:**
| Device | Support | Type | Notes |
|--------|---------|------|-------|
| Maschine Mikro MK3 | ✅ | Polyphonic | Excellent sensitivity |
| Akai MPD Series | ✅ | Polyphonic | Requires firmware 1.5+ |
| Novation Launchpad Pro | ✅ | Polyphonic | Excellent sensitivity |
| Launchpad Mini | ❌ | None | No aftertouch |
| Generic MIDI Keyboard | ⚠️ | Channel | Global only |

**Configuration Examples:**

```toml
# Basic aftertouch trigger
[[modes.mappings]]
description = "Effect intensity via pressure"
[modes.mappings.trigger]
type = "Aftertouch"
note = 1
min_pressure = 64
[modes.mappings.action]
type = "Shell"
command = "osascript -e 'set volume 50'"

# Pressure ranges for different actions
[[modes.mappings]]
description = "Soft pressure"
[modes.mappings.trigger]
type = "Aftertouch"
note = 2
min_pressure = 1
max_pressure = 64
[modes.mappings.action]
type = "Keystroke"
keys = "1"

[[modes.mappings]]
description = "Hard pressure"
[modes.mappings.trigger]
type = "Aftertouch"
note = 2
min_pressure = 65
max_pressure = 127
[modes.mappings.action]
type = "Keystroke"
keys = "2"
```

**Throttling Options:**

Aftertouch generates continuous messages. Use throttling to control message rate:

```toml
[advanced_settings.aftertouch]
throttle_ms = 50        # Max 20 messages/sec
delta_threshold = 5     # Only trigger on ±5 pressure change
hysteresis_gap = 10     # Prevent zone oscillation
```

### PitchBend

Touch strip or pitch wheel control (14-bit precision).

```toml
[trigger]
type = "PitchBend"
min_value = 8192   # Center
max_value = 16383  # Max up
center_deadzone = 100  # Optional: ignore small movements near center
```

**Parameters:**
- `min_value` (integer): Minimum bend value (0-16383)
- `max_value` (integer): Maximum bend value (0-16383)
- `center_deadzone` (integer, optional): Deadzone size around center (8192)

**Value Range:**
- **Full Range**: 0-16383 (14-bit resolution)
- **Center**: 8192
- **Down**: 0-8191
- **Up**: 8193-16383
- **Normalized**: -1.0 to +1.0

**Use Cases:**
- Touch strip for volume control
- Timeline scrubbing in DAWs
- Smooth parameter sweeps
- Multi-zone selection (divide strip into regions)

**Configuration Examples:**

```toml
# Volume control with touch strip
[[global_mappings]]
description = "Volume up via touch strip"
[global_mappings.trigger]
type = "PitchBend"
min_value = 8192  # Center
max_value = 16383  # Max up
center_deadzone = 100
[global_mappings.action]
type = "VolumeControl"
operation = "Up"

# Multi-zone mapping (divide strip into 5 zones)
[[modes.mappings]]
description = "Zone 1: Far Left"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 0
max_value = 3276  # 20% of range
[modes.mappings.action]
type = "Keystroke"
keys = "1"

[[modes.mappings]]
description = "Zone 2: Left"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 3277
max_value = 6553  # 40% of range
[modes.mappings.action]
type = "Keystroke"
keys = "2"

# Center detection
[[modes.mappings]]
description = "Reset on center touch"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 8092  # Center - 100
max_value = 8292  # Center + 100
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]
```

**Throttling Options:**

Pitch bend generates 100-1000+ messages/sec. Use throttling:

```toml
[advanced_settings.pitch_bend]
throttle_ms = 50         # Max 20 messages/sec
delta_threshold = 100    # Trigger only on ±100 change
use_zones = true         # Zone-based triggering
num_zones = 16           # 16 discrete zones
```

**Spring-Back Behavior:**

Many controllers (Maschine Mikro MK3, MIDI keyboards) auto-return to center:
- Returns to 8192 when released
- Not suitable for persistent state/toggles
- Use for momentary effects only
- Spring-back generates message flood on release

**Platform Notes:**
- **macOS**: Full 14-bit support, no special handling
- **Linux**: Some ALSA drivers may reverse MSB/LSB byte order
- **Windows**: Supported, but some USB MIDI drivers have 10-50ms latency

## Trigger Composition

Triggers can work together for complex behaviors:

**Example: Conditional Trigger**
```toml
# Only trigger during specific time range
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Conditional"
conditions = [{ type = "TimeRange", start = "09:00", end = "17:00" }]
then_action = { type = "Shell", command = "work-task" }
else_action = { type = "Shell", command = "personal-task" }
```

## Performance Notes

- **Note/CC**: <0.1ms detection latency
- **LongPress**: Checked every 50ms
- **DoubleTap**: 300ms window (configurable)
- **Chord**: 100ms window (configurable)
- **Aftertouch**: Continuous messages, use throttling
- **PitchBend**: Continuous messages, 14-bit precision

## Troubleshooting

### Trigger Not Firing

- Use `midi_diagnostic` tool to verify MIDI messages:
  ```bash
  cargo run --bin midi_diagnostic 2
  ```
- Check note numbers match MIDI output
- Verify velocity thresholds
- Enable debug logging: `DEBUG=1 cargo run`

### Aftertouch Not Working

- Check device compatibility (not all controllers support aftertouch)
- Verify polyphonic vs channel aftertouch
- Test with `midi_diagnostic` tool
- Adjust pressure thresholds

### PitchBend Jittery

- Increase `center_deadzone` value
- Enable throttling in `advanced_settings`
- Use `delta_threshold` for change-based triggering
- Consider zone-based triggering instead of continuous

### Chord Not Detected

- Increase `max_interval_ms` (default 100ms)
- Press pads more simultaneously
- Check with `midi_diagnostic` for timing
- Verify all note numbers correct

## See Also

- [Action Types](action-types.md)
- [Configuration Examples](../configuration/examples.md)
- [Modes and Mappings](../configuration/modes.md)
- [Advanced Settings](../configuration/overview.md)
