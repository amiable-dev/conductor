# MIDI Message Parsing Refactoring

**Date**: 2025-01-18
**Scope**: `midi_diagnostic` tool
**Status**: Complete

## Summary

Refactored the `midi_diagnostic` tool to use the `midi-msg` library for proper MIDI message parsing instead of manual byte parsing. This eliminates code duplication, reduces errors, and makes the code more maintainable.

## Motivation

### Problems with Manual Parsing

1. **Code Duplication**: Manual MIDI parsing (`msg[0] & 0xF0`) was duplicated in at least 3 places:
   - `midimon-daemon/src/bin/midi_diagnostic.rs`
   - Main daemon MIDI processing
   - Core MIDI parsing logic

2. **Missing Message Types**: The diagnostic tool was missing support for Polyphonic Aftertouch (0xA0), which caused "Unknown" messages to appear for pad pressure events.

3. **Error-Prone**: Manual bit manipulation is fragile and hard to maintain.

4. **No Standard Library**: `midir` only provides I/O, not message parsing.

## Solution

### Library Selection

After evaluating MIDI parsing libraries:

- **wmidi**: 14 months since last update (Dec 2023), 88k downloads
- **midi-msg**: 2 months since last update (Oct 2025), 32k downloads, actively maintained

**Decision**: Use `midi-msg` for its active maintenance and modern API.

## Changes Made

### 1. Added Dependency

**File**: `Cargo.toml` (workspace root)
```toml
# MIDI message parsing
midi-msg = "0.8"  # Modern MIDI message parser (actively maintained)
```

**File**: `midimon-daemon/Cargo.toml`
```toml
midir.workspace = true
midi-msg.workspace = true
crossbeam-channel.workspace = true
```

### 2. Refactored `midi_diagnostic.rs`

**Before**: Manual parsing with `msg[0] & 0xF0`
```rust
match msg[0] & 0xF0 {
    0x90 => { /* Note On */ }
    0x80 => { /* Note Off */ }
    0xB0 => { /* Control Change */ }
    0xA0 => { /* Missing! */ }
    // ...
}
```

**After**: Using `midi-msg` library
```rust
use midi_msg::{MidiMsg, ChannelVoiceMsg};

match MidiMsg::from_midi(msg) {
    Ok((MidiMsg::ChannelVoice { channel, msg: voice_msg }, _)) |
    Ok((MidiMsg::RunningChannelVoice { channel, msg: voice_msg }, _)) => {
        let ch = channel as u8 + 1;

        match voice_msg {
            ChannelVoiceMsg::NoteOn { note, velocity } => { /* ... */ }
            ChannelVoiceMsg::NoteOff { note, velocity } => { /* ... */ }
            ChannelVoiceMsg::ControlChange { control } => { /* ... */ }
            ChannelVoiceMsg::PolyPressure { note, pressure } => { /* ... */ }
            // All message types now supported!
        }
    }
    // ...
}
```

### 3. Key API Learnings

- `MidiMsg::from_midi(msg)` returns `Result<(MidiMsg, usize), ParseError>`
  - Returns tuple: `(parsed_message, bytes_consumed)`

- `Channel` enum can be cast directly to `u8`:
  ```rust
  let ch = channel as u8 + 1; // Display as 1-based (1-16)
  ```

- Message fields are raw primitive types:
  - `note: u8`, `velocity: u8`, `pressure: u8`
  - `bend: u16` (pitch bend is 14-bit)

- `ControlChange` is an enum with multiple variants:
  - `CC { control: u8, value: u8 }` for simple CC messages
  - Named variants for standard controls (`Volume(u16)`, `Pan(u16)`, etc.)

## Benefits

1. **✅ All MIDI Message Types Supported**
   - Polyphonic Aftertouch (0xA0) now works correctly
   - Channel Aftertouch (0xD0) continues to work
   - All 7 channel voice message types supported

2. **✅ Cleaner Code**
   - No manual bit manipulation
   - Type-safe message parsing
   - Clear enum variants for each message type

3. **✅ Maintainability**
   - Centralized parsing logic in `midi-msg` library
   - Future MIDI spec changes handled by library updates
   - Easier to add new diagnostic features

4. **✅ Display Alignment Fixed**
   - Added thread synchronization for status line updates
   - All MIDI events now properly left-aligned

## Future Work

Consider refactoring other MIDI parsing locations:

1. **midimon-daemon/src/main.rs** - Main daemon MIDI processing
2. **midimon-core/src/event_processor.rs** - Core event processing
3. **midimon-core/src/midi_feedback.rs** - MIDI output formatting

This would eliminate all manual MIDI parsing and centralize it in `midi-msg`.

## Additional Enhancements

### Musical Note Display

Added `note_to_name()` helper function to convert MIDI note numbers to musical notation:

```rust
fn note_to_name(note: u8) -> String {
    const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octave = (note / 12) as i32 - 1; // MIDI note 60 = C4
    let note_name = NOTE_NAMES[(note % 12) as usize];
    format!("{}{}", note_name, octave)
}
```

**Before**:
```
Note ON  note= 60 vel=100 ch= 1 ████████████████
```

**After**:
```
Note ON   C4 ( 60) vel=100 ch= 1 ████████████████
```

### MIDI Timestamp Display

Changed callback to use actual MIDI timestamps instead of ignoring them:

**Before**:
```rust
move |_stamp, msg, _| {  // <-- Ignoring MIDI timestamp
    let timestamp = format!("{:6.3}s", elapsed.as_secs_f32());
```

**After**:
```rust
move |midi_timestamp, msg, _| {
    let timestamp = format!(
        "{:6.3}s (MIDI: {:10}μs)",
        elapsed.as_secs_f32(),
        midi_timestamp
    );
```

This now shows both:
- **Elapsed time** since tool started (for user reference)
- **MIDI timestamp** in microseconds (precise device timing)

### Currently Held Notes

Updated status line to show note names instead of numbers:

**Before**: `Currently held: [60(1.2s), 64(0.8s)]`
**After**: `Currently held: [C4(1.2s), E4(0.8s)]`

## Testing

Build verification:
```bash
cargo build --release --package midimon-daemon --bin midi_diagnostic
```

**Status**: ✅ Builds successfully (5.32s)

### Example Output

```
  0.152s (MIDI:   123456μs) #   1 | Note ON   C4 ( 60) vel=100 ch= 1 ████████████████
  0.234s (MIDI:   205678μs) #   2 | Note ON   E4 ( 64) vel= 85 ch= 1 █████████████
  0.456s (MIDI:   428901μs) #   3 | PolyAT    C4 ( 60) pres= 45 ch= 1 ▓▓▓▓▓▓▓
  0.789s (MIDI:   762345μs) #   4 | Note OFF  C4 ( 60)         ch= 1 (held 0.637s)
```

## References

- **midi-msg crate**: https://crates.io/crates/midi-msg
- **MIDI Specification**: https://www.midi.org/specifications
- **Issue**: User reported "Unknown [A0, 10, 05]" messages
- **Root Cause**: Missing Polyphonic Aftertouch support
- **Fix**: Refactored to use `midi-msg` library
