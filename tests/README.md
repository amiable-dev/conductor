# MIDI Device Simulator

A comprehensive MIDI device simulator for hardware-independent testing of MIDIMon.

## Overview

The MIDI simulator allows testing all MIDIMon functionality without requiring physical MIDI hardware. It supports all MIDI event types, timing-sensitive operations, and complex user gestures.

## Features

- **Complete MIDI Event Support**: Note On/Off, Control Change, Aftertouch, Pitch Bend, Program Change
- **Velocity Detection**: Three levels (Soft 0-40, Medium 41-80, Hard 81-127)
- **Timing Operations**: Long press, double-tap, chord detection
- **Complex Gestures**: Pre-built gestures for common patterns
- **Event Capture**: Inspect all generated MIDI messages
- **Interactive CLI**: Manual testing and experimentation

## Quick Start

### Using in Tests

```rust
use midi_simulator::{MidiSimulator, Gesture};

#[test]
fn test_my_feature() {
    let sim = MidiSimulator::new(0);

    sim.note_on(60, 100);
    sim.note_off(60);

    let events = sim.get_events();
    assert_eq!(events.len(), 2);
}
```

### Interactive CLI

```bash
cargo run --bin midi_simulator
```

## File Structure

```
tests/
├── README.md                    # This file
├── midi_simulator.rs            # Core simulator implementation
├── integration_tests.rs         # Integration tests using simulator
└── config_compatibility_test.rs # Config format tests
```

## Running Tests

```bash
# Run simulator unit tests
cargo test --test midi_simulator

# Run integration tests
cargo test --test integration_tests

# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Test Coverage

### Current Coverage

- ✅ 12 simulator unit tests (100% pass rate)
- ✅ 29 integration tests (100% pass rate)
- ✅ All MIDI message types
- ✅ Velocity level detection
- ✅ Timing-based operations (long press, double-tap)
- ✅ Complex gestures (chords, encoder rotation)
- ✅ Event queue operations
- ✅ Multiple channel support

### Test Results Summary

```
Test Suite                   Tests  Passed  Failed  Duration
-------------------------    -----  ------  ------  --------
midi_simulator (unit)           12      12       0    1.03s
integration_tests               29      29       0    2.51s
config_compatibility_test       15      15       0    0.05s
-------------------------    -----  ------  ------  --------
Total                           56      56       0    3.59s
```

## Supported Gestures

### Basic Events

```rust
sim.note_on(60, 100);           // Note On
sim.note_off(60);               // Note Off
sim.control_change(1, 64);      // CC message
sim.aftertouch(80);             // Aftertouch
sim.pitch_bend(8192);           // Pitch Bend (center)
```

### High-Level Gestures

```rust
// Simple tap (press and release)
sim.perform_gesture(Gesture::SimpleTap {
    note: 60,
    velocity: 80,
    duration_ms: 100,
});

// Long press (2.5 seconds)
sim.perform_gesture(Gesture::LongPress {
    note: 60,
    velocity: 80,
    hold_ms: 2500,
});

// Double-tap
sim.perform_gesture(Gesture::DoubleTap {
    note: 60,
    velocity: 80,
    tap_duration_ms: 50,
    gap_ms: 200,
});

// Chord (C major)
sim.perform_gesture(Gesture::Chord {
    notes: vec![60, 64, 67],
    velocity: 80,
    stagger_ms: 10,
    hold_ms: 500,
});

// Encoder rotation
sim.perform_gesture(Gesture::EncoderTurn {
    cc: 1,
    direction: EncoderDirection::Clockwise,
    steps: 5,
    step_delay_ms: 50,
});

// Velocity ramp
sim.perform_gesture(Gesture::VelocityRamp {
    note: 60,
    min_velocity: 20,
    max_velocity: 120,
    steps: 5,
});
```

## Scenario Builder

Build complex test scenarios:

```rust
use midi_simulator::ScenarioBuilder;

let scenario = ScenarioBuilder::new()
    .note_on(60, 100)
    .wait(100)
    .control_change(1, 64)
    .wait(100)
    .aftertouch(80)
    .note_off(60)
    .build();

sim.execute_sequence(scenario);
```

## Interactive CLI Commands

```
Basic Commands:
  help              Show help message
  quit              Exit simulator
  clear             Clear event queue
  events            Show captured events

MIDI Events:
  note <n> <vel>    Send note on/off
  velocity <note>   Test velocity levels
  long <note> [ms]  Simulate long press
  double <note> [gap] Simulate double-tap
  chord <n1> <n2>...  Simulate chord
  encoder <cc> <dir>  Simulate encoder
  aftertouch <val>  Send aftertouch
  pitch <val>       Send pitch bend
  cc <num> <val>    Send CC message

Scenarios:
  demo              Run full demonstration
  scenario velocity Test velocity levels
  scenario timing   Test timing detection
  scenario doubletap Test double-tap
  scenario chord    Test chord detection
  scenario encoder  Test encoder rotation
  scenario complex  Test complex sequence
```

## Example Test Session

```bash
$ cargo run --bin midi_simulator

> velocity 60
Simulating velocity levels (soft, medium, hard)...
✓ Velocity test complete

> long 60 2500
Simulating long press for 2500ms...
✓ Long press complete

> chord 60 64 67
Simulating chord: [60, 64, 67]
✓ Chord complete

> events
Captured events:
  1: [90, 3C, 1E]  # Note On (60, vel 30)
  2: [80, 3C, 40]  # Note Off (60)
  ...

> demo
Running demonstration scenarios...
1. Testing velocity levels...
2. Testing long press...
3. Testing double-tap...
4. Testing chord...
5. Testing encoder...
✓ Demo complete

> quit
Goodbye!
```

## Event Inspection

```rust
// Get all events and clear queue
let events = sim.get_events();

// Peek without clearing
let last = sim.peek_last_event();

// Clear queue
sim.clear_events();

// Parse events
for event in events {
    let status = event[0];
    let message_type = status & 0xF0;
    match message_type {
        0x90 => println!("Note On: {}", event[1]),
        0x80 => println!("Note Off: {}", event[1]),
        0xB0 => println!("CC: {}", event[1]),
        _ => {}
    }
}
```

## Debug Output

Enable debug logging to see all MIDI messages:

```rust
let mut sim = MidiSimulator::new(0);
sim.set_debug(true);

sim.note_on(60, 100);
// Output: [SIM] Sending: [90, 3C, 64]
```

## Integration with CI/CD

The simulator works in CI environments without hardware:

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: cargo test --all-features
```

## Best Practices

1. **Clear events between tests** to avoid interference
2. **Use gestures** for complex interactions instead of manual sequences
3. **Verify timing** with `Instant::now()` for timing-sensitive tests
4. **Test edge cases** at velocity boundaries (0, 40, 41, 80, 81, 127)
5. **Test multiple channels** to verify channel masking

## Architecture

The simulator uses an in-memory event queue with precise timing control:

```
User Test Code
      ↓
MidiSimulator API
      ↓
Event Generation (with timing)
      ↓
Event Queue (Vec<Vec<u8>>)
      ↓
Event Inspection/Verification
```

## Limitations

- **Virtual MIDI ports**: Does not create actual MIDI ports (uses in-memory queue)
- **No DAW integration**: Cannot send to external applications
- **Synchronous execution**: Gestures block until complete
- **Single channel per simulator**: Create multiple instances for multi-channel testing

## Future Enhancements

- [ ] Virtual MIDI port creation for DAW integration
- [ ] Async gesture execution
- [ ] MIDI file playback
- [ ] Real-time event scheduling
- [ ] Performance profiling integration

## Documentation

Complete documentation available at:
- [Testing Guide](../docs-site/src/development/testing.md)
- [CLAUDE.md](../CLAUDE.md) - Development guide
- [README.md](../README.md) - Project overview

## Support

For issues or questions:
1. Check integration tests for examples
2. Use interactive CLI to experiment
3. Review source code in `midi_simulator.rs`
4. Open GitHub issue with test failure details

## Related Issues

- AMI-121: Build device simulator for hardware-independent testing ✅ Complete
- AMI-117: Core event processor integration tests (blocked on AMI-121)
- AMI-118: Mapping engine integration tests (blocked on AMI-121)
- AMI-119: Action executor integration tests (blocked on AMI-121)
- AMI-120: Full pipeline end-to-end tests (blocked on AMI-121)
