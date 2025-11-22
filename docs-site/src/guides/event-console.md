# Event Console

The Live Event Console provides real-time visualization of MIDI events, helping you debug mappings, discover note numbers, and understand your controller's behavior.

## Overview

The Event Console displays:

- **MIDI events**: Note on/off, CC, aftertouch, pitch bend
- **Timing information**: Timestamps and event duration
- **Velocity data**: Press intensity values
- **Processed events**: Long press, double-tap, chord detection
- **Action execution**: What actions were triggered
- **Error messages**: Failed actions or validation issues

## Access the Console

### GUI Method

1. Open Conductor GUI
2. Navigate to **Settings** tab
3. Click **📊 Show Event Console**
4. View live events in real-time

### CLI Method

```bash
# Monitor all events
conductorctl events

# Filter by event type
conductorctl events --type note

# Follow mode (live tail)
conductorctl events --follow
```

## Event Types

### MIDI Events

#### Note On/Off

```
[12:34:56.789] NOTE_ON  | Note: 36 | Vel: 87 | Ch: 1
[12:34:57.120] NOTE_OFF | Note: 36 | Vel: 0  | Ch: 1
Duration: 331ms
```

#### Control Change

```
[12:34:58.456] CC | CC: 7 | Value: 64 | Ch: 1
```

#### Aftertouch

```
[12:35:00.123] AFTERTOUCH | Pressure: 95 | Ch: 1
```

#### Pitch Bend

```
[12:35:01.789] PITCH_BEND | Value: 8192 | Ch: 1
```

### Processed Events

#### Long Press Detected

```
[12:35:05.000] LONG_PRESS | Note: 36 | Duration: 2150ms
Trigger matched: "Emergency Stop" (mapping #5)
```

#### Double Tap Detected

```
[12:35:10.100] DOUBLE_TAP | Note: 37 | Interval: 245ms
Trigger matched: "Quick Launch" (mapping #8)
```

#### Chord Detected

```
[12:35:15.500] CHORD | Notes: [36, 40, 43] | Window: 87ms
Trigger matched: "Save All" (mapping #12)
```

#### Encoder Turn

```
[12:35:20.750] ENCODER_TURN | CC: 1 | Direction: Clockwise | Steps: 3
Trigger matched: "Volume Up" (mapping #2)
```

### Action Execution

```
[12:35:25.100] ACTION_START | Type: Keystroke
  Modifiers: [Cmd, Shift]
  Keys: "S"
[12:35:25.105] ACTION_COMPLETE | Duration: 5ms | Status: Success
```

### Errors

```
[12:35:30.000] ERROR | Failed to execute shell command
  Command: "/usr/bin/notexist"
  Error: "No such file or directory"
  Mapping: #15 ("Custom Script")
```

## Filtering Events

### By Event Type

```bash
# Only note events
conductorctl events --type note

# Only CC events
conductorctl events --type cc

# Only processed events
conductorctl events --type processed

# Only actions
conductorctl events --type action

# Only errors
conductorctl events --type error
```

### By MIDI Channel

```bash
conductorctl events --channel 1
```

### By Note Range

```bash
# Only pads (notes 36-51)
conductorctl events --note-min 36 --note-max 51
```

### By Time Range

```bash
# Last 5 minutes
conductorctl events --since 5m

# Last hour
conductorctl events --since 1h

# Specific time
conductorctl events --since "2025-01-15 12:00"
```

## Use Cases

### Discover Note Numbers

**Problem**: Don't know which MIDI note a pad sends

**Solution**:
1. Open Event Console
2. Press the pad
3. See `NOTE_ON | Note: 36` in console
4. Use note 36 in your mapping

```
[12:40:00.000] NOTE_ON | Note: 36 | Vel: 92 | Ch: 1
                       ↑
                   This is the note number
```

### Debug Long Press Not Triggering

**Problem**: Long press mapping not activating

**Solution**:
1. Open Event Console
2. Hold the pad
3. Check if `LONG_PRESS` event appears
4. Compare duration with your config:

```
[12:45:00.000] NOTE_ON    | Note: 36 | Vel: 87
[12:45:01.500] NOTE_OFF   | Note: 36 | Duration: 1500ms
                                      ↑
                        Too short! Config requires 2000ms
```

Fix by reducing `duration_ms` in config or holding longer.

### Verify Velocity Ranges

**Problem**: Soft/medium/hard velocity ranges not working as expected

**Solution**:
1. Press pad softly: Check velocity value
2. Press pad medium: Check velocity value
3. Press pad hard: Check velocity value
4. Adjust ranges in config

```
Soft press:   [12:50:00.000] NOTE_ON | Vel: 25  ← Within 0-40
Medium press: [12:50:05.000] NOTE_ON | Vel: 65  ← Within 41-80
Hard press:   [12:50:10.000] NOTE_ON | Vel: 110 ← Within 81-127
```

### Debug Chord Detection

**Problem**: Chord mapping not triggering

**Solution**:
1. Press chord notes
2. Check timing window in console:

```
[13:00:00.000] NOTE_ON | Note: 36
[13:00:00.150] NOTE_ON | Note: 40  ← 150ms gap, too slow!
[13:00:00.300] NOTE_ON | Note: 43

Config requires all notes within 100ms
```

Fix: Press notes faster OR increase `chord_timeout_ms` in config.

### Test Action Execution

**Problem**: Action not executing as expected

**Solution**:
1. Trigger the mapping
2. Watch action execution in console
3. Check for errors

```
[13:05:00.000] ACTION_START | Type: Shell
  Command: "open -a 'Visual Studio Code'"
[13:05:00.250] ACTION_COMPLETE | Duration: 250ms | Status: Success

vs.

[13:05:05.000] ACTION_START | Type: Shell
  Command: "open -a 'NotExist'"
[13:05:05.100] ERROR | Application not found
                ↑
            This tells you what went wrong
```

### Monitor System Performance

Watch event processing latency:

```
[13:10:00.000] NOTE_ON      | Note: 36
[13:10:00.001] PROCESSED    | Duration: 1ms  ← Very fast!
[13:10:00.002] ACTION_START
[13:10:00.007] ACTION_COMPLETE | Duration: 5ms

Total: 7ms from button press to action complete
```

If latency >50ms, investigate performance issues.

## GUI Console Features

### Visual Event Timeline

The GUI console includes a visual timeline showing:

- Event type (color-coded)
- Timestamp
- Event details
- Expandable for full info

### Event Highlighting

- **Green**: Successful events
- **Yellow**: Warnings (e.g., near timeout)
- **Red**: Errors
- **Blue**: Info (e.g., mode changes)

### Auto-Scroll

Console auto-scrolls to latest events. Click **Pause** to freeze for inspection.

### Event Export

1. Select events
2. Click **Export**
3. Save as JSON/CSV for analysis

### Event Statistics

View live statistics:
- Events per second
- Average velocity
- Most-pressed pad
- Error rate

## Advanced Features

### Pattern Recording

Record event sequences for analysis:

```bash
conductorctl record-events --duration 30s --output session.json
```

Playback recorded events:

```bash
conductorctl playback-events session.json
```

### Event Filtering Rules

Create custom filters in config:

```toml
[event_console]
[[event_console.filters]]
name = "Only My Pads"
include_notes = [36, 37, 38, 39]
include_types = ["note", "processed"]

[[event_console.filters]]
name = "Errors Only"
include_types = ["error"]
min_severity = "warning"
```

Activate filter in GUI or CLI:

```bash
conductorctl events --filter "Only My Pads"
```

### Event Triggers

Trigger actions based on events:

```toml
[event_console.triggers]
[[event_console.triggers.rules]]
condition = "error_rate > 5 per_minute"
action = { type = "Notification", message = "High error rate detected!" }

[[event_console.triggers.rules]]
condition = "note_36 pressed > 10 times in 5s"
action = { type = "Notification", message = "Stop mashing that button!" }
```

### Performance Profiling

Enable detailed performance metrics:

```toml
[event_console]
enable_profiling = true
track_latency = true
track_memory = true
```

View profiling data:

```bash
conductorctl events --profiling
```

Output includes:
- Event processing time
- Memory allocation
- CPU usage per event
- Bottleneck identification

## Troubleshooting

### Console Not Updating

1. Check daemon is running: `conductorctl status`
2. Verify events are being generated (press pads)
3. Restart daemon: `conductorctl restart`
4. Check event monitoring is active: `conductorctl is-event-monitoring-active`

### Too Many Events

Filter aggressively:

```bash
# Only errors and warnings
conductorctl events --type error --type warning

# Debounce rapid events
conductorctl events --debounce 100ms
```

Or reduce update rate in config:

```toml
[event_console]
max_events_per_second = 30
```

### Missing Events

1. Check buffer size isn't full:
   ```toml
   [event_console]
   buffer_size = 10000  # Increase if needed
   ```

2. Verify event types are enabled:
   ```toml
   [event_console]
   capture_midi = true
   capture_processed = true
   capture_actions = true
   ```

3. Check log level:
   ```bash
   DEBUG=1 conductorctl events
   ```

## Best Practices

1. **Start with full view**: See all event types initially
2. **Filter progressively**: Add filters as you narrow down issues
3. **Use follow mode**: `-f` flag for live monitoring
4. **Export for analysis**: Save interesting sessions
5. **Watch timing**: Event timing reveals latency issues
6. **Compare configs**: Record events, change config, compare results
7. **Document discoveries**: Note note numbers and velocities for future reference

## Integration with Other Tools

### Export to CSV for Analysis

```bash
conductorctl events --output events.csv --duration 60s
```

Analyze in Excel/Google Sheets:
- Event frequency
- Velocity distribution
- Timing patterns

### Send to External Monitoring

```bash
# Stream events to external tool
conductorctl events --format json | jq '.' | your-monitoring-tool
```

### Log to File

```bash
# Continuous logging
conductorctl events -f >> conductor-events.log
```

## Next Steps

- Use console to build mappings with [MIDI Learn](../getting-started/midi-learn.md)
- Debug complex triggers in [mapping configuration](./gui.md#mapping-configuration)
- Monitor [LED feedback](./led-system.md) in real-time
- Analyze [per-app profile](./per-app-profiles.md) switching behavior
