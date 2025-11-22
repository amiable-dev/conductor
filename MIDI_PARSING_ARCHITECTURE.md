# MIDI Parsing Architecture Analysis

**Date**: 2025-01-18
**Status**: ✅ **REFACTORING COMPLETE**
**Analysis**: How MIDIMon processes MIDI messages across different layers

## Executive Summary

All MIDI parsing in MIDIMon has been successfully refactored to use the `midi-msg` library (v0.8) for consistency, correctness, and maintainability. Manual byte parsing has been eliminated.

## Current Architecture

MIDIMon has a **3-layer architecture** for MIDI processing:

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Raw MIDI Bytes (from midir)                      │
│  - Receives raw bytes from MIDI hardware                    │
│  - Example: [0x90, 60, 100] (Note On, Middle C, vel 100)   │
└─────────────────────────────────────────────────────────────┘
                            ↓
                    [midi-msg library]
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 2: MidiEvent (midimon-core)                         │
│  - Protocol-specific parsed events                          │
│  - Example: MidiEvent::NoteOn { note: 60, velocity: 100 }  │
│  - Now includes PolyPressure variant                        │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  Layer 3: ProcessedEvent (midimon-core)                    │
│  - High-level semantic events with timing                   │
│  - Example: ProcessedEvent::PadPressed { velocity_level }  │
└─────────────────────────────────────────────────────────────┘
```

## Completed Refactoring

### ✅ 1. **midimon-core - MidiEvent Helper**

**File**: `midimon-core/src/event_processor.rs`

**Implementation**: Created centralized `MidiEvent::from_midi_msg()` helper

```rust
impl MidiEvent {
    pub fn from_midi_msg(msg: &[u8]) -> Result<Self, String> {
        use midi_msg::{MidiMsg, ChannelVoiceMsg, ControlChange};

        match MidiMsg::from_midi(msg) {
            Ok((MidiMsg::ChannelVoice { msg, .. }, _)) |
            Ok((MidiMsg::RunningChannelVoice { msg, .. }, _)) => {
                let time = Instant::now();
                match msg {
                    ChannelVoiceMsg::NoteOn { note, velocity } => {
                        if velocity > 0 {
                            Ok(MidiEvent::NoteOn { note, velocity, time })
                        } else {
                            Ok(MidiEvent::NoteOff { note, time })
                        }
                    }
                    ChannelVoiceMsg::NoteOff { note, .. } => {
                        Ok(MidiEvent::NoteOff { note, time })
                    }
                    ChannelVoiceMsg::PolyPressure { note, pressure } => {
                        Ok(MidiEvent::PolyPressure { note, pressure, time })
                    }
                    ChannelVoiceMsg::ControlChange { control } => {
                        if let ControlChange::CC { control, value } = control {
                            Ok(MidiEvent::ControlChange { cc: control, value, time })
                        } else {
                            Err("Unsupported control change type".to_string())
                        }
                    }
                    ChannelVoiceMsg::ProgramChange { program } => {
                        Ok(MidiEvent::ProgramChange { program, time })
                    }
                    ChannelVoiceMsg::ChannelPressure { pressure } => {
                        Ok(MidiEvent::Aftertouch { pressure, time })
                    }
                    ChannelVoiceMsg::PitchBend { value } => {
                        Ok(MidiEvent::PitchBend { value, time })
                    }
                    _ => Err(format!("Unsupported MIDI message: {:?}", msg)),
                }
            }
            _ => Err("Only channel voice messages are supported".to_string()),
        }
    }
}
```

**Status**: ✅ **COMPLETE** - All 9 test cases passing

---

### ✅ 2. **midimon-core - Polyphonic Aftertouch Support**

**Files**:
- `midimon-core/src/event_processor.rs`
- `midimon-core/src/events.rs`

**Implementation**: Added `PolyPressure` variant to both `MidiEvent` and `InputEvent` enums

```rust
// MidiEvent enum
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8, time: Instant },
    NoteOff { note: u8, time: Instant },
    ControlChange { cc: u8, value: u8, time: Instant },
    PolyPressure { note: u8, pressure: u8, time: Instant }, // NEW
    Aftertouch { pressure: u8, time: Instant },
    PitchBend { value: u16, time: Instant },
    ProgramChange { program: u8, time: Instant },
}

// InputEvent enum
pub enum InputEvent {
    PadPressed { pad: u8, velocity: u8, time: Instant },
    PadReleased { pad: u8, time: Instant },
    EncoderTurned { encoder: u8, value: u8, time: Instant },
    PolyPressure { pad: u8, pressure: u8, time: Instant }, // NEW
    Aftertouch { pressure: u8, time: Instant },
    PitchBend { value: u16, time: Instant },
    ProgramChange { program: u8, time: Instant },
    ControlChange { control: u8, value: u8, time: Instant },
}
```

**Key Distinction**:
- `PolyPressure` (0xA0): Per-note pressure (2 data bytes: note + pressure)
- `Aftertouch` (0xD0): Channel-wide pressure (1 data byte: pressure)

**Status**: ✅ **COMPLETE** - 14 tests passing in input_event_tests.rs

---

### ✅ 3. **Diagnostic Tool - Musical Note Display**

**File**: `midimon-daemon/src/bin/midi_diagnostic.rs`

**Implementation**: Uses `midi-msg` with musical note names

```rust
fn note_to_name(note: u8) -> String {
    const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octave = (note / 12) as i32 - 1; // MIDI note 60 = C4
    let note_name = NOTE_NAMES[(note % 12) as usize];
    format!("{}{}", note_name, octave)
}
```

**Output Format**:
```
0.152s (MIDI: 123456μs) #1 | Note ON C4 ( 60) vel=100 ch= 1 ████████████████
```

**Status**: ✅ **COMPLETE** - Committed in previous commit

---

### ✅ 4. **GUI - MIDI Event Parsing**

**File**: `midimon-gui/src-tauri/src/events.rs`

**Before**:
```rust
match msg[0] & 0xF0 {
    0x90 => { /* Manual NoteOn parsing */ }
    0x80 => { /* Manual NoteOff parsing */ }
    0xB0 => { /* Manual CC parsing */ }
    // ...
}
```

**After**:
```rust
use midi_msg::{ChannelVoiceMsg, MidiMsg};

match MidiMsg::from_midi(msg) {
    Ok((MidiMsg::ChannelVoice { channel, msg }, _)) => {
        match msg {
            ChannelVoiceMsg::NoteOn { note, velocity } => { /* ... */ }
            ChannelVoiceMsg::NoteOff { note, velocity } => { /* ... */ }
            ChannelVoiceMsg::PolyPressure { note, pressure } => { /* ... */ }
            ChannelVoiceMsg::ControlChange { control } => { /* ... */ }
            ChannelVoiceMsg::ProgramChange { program } => { /* ... */ }
            ChannelVoiceMsg::ChannelPressure { pressure } => { /* ... */ }
            ChannelVoiceMsg::PitchBend { value } => { /* ... */ }
            _ => { /* ... */ }
        }
    }
    _ => { /* ... */ }
}
```

**Status**: ✅ **COMPLETE** - 27 tests passing

---

### ✅ 5. **GUI - MIDI Message Construction**

**File**: `midimon-gui/src-tauri/src/commands.rs`

**Before**:
```rust
let msg = vec![0x90 | (channel & 0x0F), note & 0x7F, velocity & 0x7F];
```

**After**:
```rust
use midi_msg::{Channel, ChannelVoiceMsg, ControlChange, MidiMsg};

let channel = Channel::from_u8(channel.saturating_sub(1))
    .map_err(|_| "Invalid MIDI channel")?;

let midi_msg = MidiMsg::ChannelVoice {
    channel,
    msg: ChannelVoiceMsg::NoteOn { note, velocity },
};

let msg = midi_msg.to_midi();
```

**Status**: ✅ **COMPLETE** - Release build successful (3m 54s)

---

## MIDI Parsing Locations Summary

| Component | File | Parsing Method | Status |
|-----------|------|---------------|---------|
| Core Helper | `midimon-core/src/event_processor.rs` | `MidiEvent::from_midi_msg()` | ✅ Complete |
| Diagnostic Tool | `midimon-daemon/bin/midi_diagnostic.rs` | `midi-msg` library | ✅ Complete |
| GUI Events | `midimon-gui/src-tauri/src/events.rs` | `midi-msg` library | ✅ Complete |
| GUI Commands | `midimon-gui/src-tauri/src/commands.rs` | `midi-msg` library | ✅ Complete |
| Daemon Main | N/A (Phase 3 infrastructure) | Not yet implemented | ⏸️ Pending |

**Note**: The daemon's main MIDI event loop is not yet implemented. Phase 3 completed the daemon infrastructure (IPC, config hot-reload, state persistence) but MIDI device connection is still TODO.

---

## Test Results

**midimon-core**:
- ✅ 69 library tests passing
- ✅ All InputEvent tests passing (including new PolyPressure tests)
- ✅ All MidiEvent::from_midi_msg() tests passing (9 test cases)

**midimon-gui**:
- ✅ 27 tests passing
- ✅ 1 ignored (expected)
- ✅ Release build successful

**midimon-daemon**:
- ✅ 12 tests passing
- ✅ 3 doctests passing

---

## Benefits Achieved

### 1. **Consistency**
- All MIDI parsing now uses the same library (`midi-msg`)
- Eliminates manual byte manipulation across the codebase
- Single source of truth for MIDI protocol handling

### 2. **Correctness**
- `midi-msg` handles edge cases (running status, velocity 0 → Note Off)
- Proper support for all MIDI message types
- Type-safe message construction

### 3. **Maintainability**
- Centralized helper function in `midimon-core`
- Clear separation of concerns
- Easier to add new message types

### 4. **Completeness**
- Added missing Polyphonic Aftertouch support
- Proper distinction between poly and channel pressure
- Support for high-resolution note messages (CA-031)

### 5. **Developer Experience**
- Musical note names in diagnostic tool (C4 instead of 60)
- Better error messages
- Self-documenting code with enum patterns

---

## Performance Considerations

**Status**: Not yet benchmarked (pending task)

**Expected Impact**: Minimal
- `midi-msg` parsing is lightweight (pattern matching on bytes)
- Hot path is event processing, not parsing
- Potential 0-10μs overhead per event (negligible for human interaction)

**Recommendation**: Benchmark before production deployment

---

## Future Work

### Short Term
- [ ] Benchmark `midi-msg` performance vs manual parsing
- [ ] Update documentation with new patterns
- [ ] Create migration guide for external users

### Medium Term
- [ ] Implement daemon MIDI device connection
- [ ] Use `MidiEvent::from_midi_msg()` in daemon
- [ ] Add ProcessedEvent variants for PolyPressure

### Long Term
- [ ] Consider HID message parsing abstraction
- [ ] Unified InputEvent → ProcessedEvent → Action pipeline
- [ ] MIDI 2.0 support (if `midi-msg` adds it)

---

## Migration Guide

For developers using MIDIMon's MIDI parsing:

**Old Pattern**:
```rust
match msg[0] & 0xF0 {
    0x90 => {
        let note = msg[1];
        let velocity = msg[2];
        // ...
    }
    // ...
}
```

**New Pattern**:
```rust
use midimon_core::event_processor::MidiEvent;

match MidiEvent::from_midi_msg(msg) {
    Ok(MidiEvent::NoteOn { note, velocity, time }) => {
        // ...
    }
    Ok(MidiEvent::PolyPressure { note, pressure, time }) => {
        // NEW: Polyphonic Aftertouch support
    }
    // ...
    Err(e) => eprintln!("MIDI parse error: {}", e),
}
```

---

## Conclusion

✅ **All MIDI parsing refactoring recommendations have been successfully implemented.**

The codebase now has:
- Consistent MIDI parsing using `midi-msg` library
- Centralized helper function in `midimon-core`
- Support for Polyphonic Aftertouch (previously missing)
- Musical note display in diagnostic tool
- Type-safe MIDI message construction in GUI
- 100% test pass rate (108 tests passing)

**Next Step**: Performance benchmarking to validate production readiness.
