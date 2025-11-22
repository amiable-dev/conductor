# Testing Guide

This guide covers testing strategies for Conductor, including hardware-independent testing using the MIDI device simulator.

## Table of Contents

- [MIDI Device Simulator](#midi-device-simulator)
- [Running Tests](#running-tests)
- [End-to-End Test Suite](#end-to-end-test-suite)
- [Code Coverage](#code-coverage)
- [Test Reporting](#test-reporting)
- [Writing Tests](#writing-tests)
- [Interactive CLI Tool](#interactive-cli-tool)
- [Test Scenarios](#test-scenarios)
- [Game Controllers (HID) Testing (v3.0+)](#game-controllers-hid-testing-v30)

## MIDI Device Simulator

The MIDI device simulator allows comprehensive testing of Conductor without requiring physical hardware. It simulates all MIDI events and complex user interactions with precise timing control.

### Features

The simulator supports:

- **Basic MIDI Events**: Note On/Off, Control Change, Aftertouch, Pitch Bend, Program Change
- **Velocity Levels**: Soft (0-40), Medium (41-80), Hard (81-127)
- **Timing-Based Triggers**: Long press, double-tap detection
- **Complex Gestures**: Chords, encoder rotation, velocity ramps
- **Precise Timing**: Configurable delays and durations
- **Event Capture**: Inspect all generated MIDI messages

### Quick Start

```rust
use midi_simulator::{MidiSimulator, Gesture, EncoderDirection};

// Create a simulator on MIDI channel 0
let sim = MidiSimulator::new(0);

// Simulate a simple note press
sim.note_on(60, 100);
sim.note_off(60);

// Get captured events
let events = sim.get_events();
assert_eq!(events.len(), 2); // Note on + Note off
```

## Running Tests

### Unit Tests

Run the simulator's built-in unit tests:

```bash
cargo test --test midi_simulator
```

Expected output:
```
running 12 tests
test tests::test_note_on_off ... ok
test tests::test_velocity_levels ... ok
test tests::test_control_change ... ok
test tests::test_aftertouch ... ok
test tests::test_pitch_bend ... ok
test tests::test_program_change ... ok
test tests::test_simple_tap_gesture ... ok
test tests::test_chord_gesture ... ok
test tests::test_encoder_simulation ... ok
test tests::test_velocity_ramp_gesture ... ok
test tests::test_scenario_builder ... ok
test tests::test_channel_masking ... ok

test result: ok. 12 passed; 0 failed
```

### Integration Tests

Run integration tests that verify the complete event processing pipeline:

```bash
cargo test --test integration_tests
```

These tests cover:
- Basic note event handling
- Velocity level detection
- Long press simulation with timing validation
- Double-tap detection
- Chord detection with multiple notes
- Encoder direction detection (CW/CCW)
- Aftertouch and pitch bend
- Complex multi-event scenarios

### All Tests

Run all tests including unit and integration:

```bash
cargo test
```

### Using Nextest (Recommended)

For improved test output and parallel execution, use cargo-nextest:

```bash
# Install nextest
cargo install cargo-nextest

# Run tests with nextest
cargo nextest run --all-features

# Or use the convenience script
./scripts/test-nextest.sh
```

Nextest provides:
- Faster test execution through parallelization
- Better output formatting with progress indicators
- More detailed failure reporting
- Per-test timing information

## End-to-End Test Suite

The E2E test suite (`tests/e2e_tests.rs`) provides comprehensive validation of the complete Conductor pipeline from MIDI input through event processing, mapping, and action execution. See [Integration Test Suites](#integration-test-suites) section below for full documentation of all E2E workflows, test architecture, and writing E2E tests.

### Quick Start

```bash
# Run all E2E tests (20+ workflow tests)
cargo test --test e2e_tests

# Expected: 37 tests passed covering all critical workflows
```

## Code Coverage

Conductor uses `cargo-llvm-cov` for code coverage tracking. The project maintains a minimum coverage threshold of 0.35% (baseline) with a Phase 1 target of 85%.

### Installing Coverage Tools

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov
```

### Generating Coverage Reports

#### Terminal Summary (Default)

Generate a coverage summary in the terminal:

```bash
# Using cargo-llvm-cov directly
cargo llvm-cov --all-features --workspace

# Or use the convenience script
./scripts/coverage.sh

# Or use just command
just coverage
```

Output example:
```
Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed       Lines      Missed Lines     Cover
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
actions.rs                        212               212     0.00%          12                12     0.00%         134               134     0.00%
config.rs                          52                42    19.23%           3                 2    33.33%          52                47     9.62%
main.rs                           278               278     0.00%          13                13     0.00%         151               151     0.00%
mappings.rs                        96                96     0.00%           8                 8     0.00%          71                71     0.00%
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
TOTAL                            2263              2253     0.44%          66                65     1.52%        1413              1408     0.35%
```

#### HTML Report

Generate an interactive HTML coverage report:

```bash
# Generate HTML report
./scripts/coverage.sh --html

# Or use just command
just coverage-html

# Report saved to: target/llvm-cov/html/index.html
```

#### HTML Report with Auto-Open

Generate and automatically open the coverage report in your browser:

```bash
# Generate and open HTML report
./scripts/coverage.sh --open

# Or use just command
just coverage-open
```

#### LCOV Format (for CI)

Generate coverage in LCOV format for Codecov or other CI tools:

```bash
# Generate lcov.info
./scripts/coverage.sh --lcov

# Or use just command
just coverage-lcov

# Output: lcov.info
```

### Coverage Configuration

Coverage settings are configured in `.llvm-cov.toml`:

```toml
[report]
# Fail if coverage is below this percentage
fail-under-lines = 0.35

[filter]
# Exclude test files and binaries from coverage
exclude-filename-regex = [
    ".*/tests/.*",
    ".*/bin/.*"
]
```

### Coverage Targets

- **Phase 0 Baseline**: 0.35% (established)
- **Phase 1 Target**: 85% line coverage
- **Minimum Threshold**: 80% (enforced in CI)

### Coverage in CI/CD

Coverage is automatically tracked on every pull request:

1. **GitHub Actions** runs coverage on all PRs
2. **Codecov** receives coverage reports and provides:
   - Coverage percentage badge
   - Line-by-line coverage visualization
   - Coverage diffs on PRs
   - Historical coverage trends
3. **PR Comments** show coverage delta (increase/decrease)
4. **Status Checks** fail if coverage drops below threshold

View current coverage: [![codecov](https://codecov.io/gh/amiable-dev/conductor/branch/main/graph/badge.svg)](https://codecov.io/gh/amiable-dev/conductor)

### Local Coverage Workflow

Recommended workflow for maintaining coverage:

```bash
# 1. Write tests for new code
vim tests/my_feature_test.rs

# 2. Run tests to verify they pass
cargo nextest run

# 3. Generate coverage report
just coverage-open

# 4. Identify uncovered lines (shown in red in HTML report)

# 5. Add tests for uncovered code paths

# 6. Verify coverage improved
just coverage
```

## Test Reporting

### GitHub Actions Integration

All tests run automatically on:
- **Push to main/develop branches**
- **Pull requests**
- **Manual workflow dispatch**

Test results are displayed in the GitHub Actions UI with:
- Test count (passed/failed/skipped)
- Execution time
- Detailed failure logs
- Coverage percentage

### Nextest Reports

Nextest provides enhanced test reporting:

```bash
# Run with detailed output
cargo nextest run --verbose

# Generate JUnit XML report
cargo nextest run --junit junit.xml

# Show only failed tests
cargo nextest run --failure-output immediate
```

### Coverage Reports in PRs

When you create a pull request, Codecov automatically:
1. Analyzes coverage for changed files
2. Posts a comment with coverage diff
3. Updates status checks
4. Shows coverage on changed lines

Example PR comment:
```
Coverage: 85.2% (+2.1%) vs main

Files changed coverage:
  - src/new_feature.rs: 92.3% ✓
  - src/existing.rs: 78.1% ⚠️ (below target)
```

### Local Test Scripts

Use convenience scripts for common testing tasks:

```bash
# Run all tests
./scripts/test.sh

# Run with nextest
./scripts/test-nextest.sh

# Generate coverage
./scripts/coverage.sh [--html|--open|--lcov]
```

### Just Commands

The `justfile` provides convenient shortcuts:

```bash
# View all available commands
just

# Run tests
just test              # Standard cargo test
just test-nextest      # With nextest
just test-watch        # Watch mode (requires cargo-watch)

# Coverage
just coverage          # Terminal summary
just coverage-html     # HTML report
just coverage-open     # HTML report + open in browser
just coverage-lcov     # LCOV format for CI

# Linting and formatting
just lint              # Run clippy
just fmt               # Format code
just fmt-check         # Check formatting

# Complete CI check locally
just ci                # Run all CI checks (fmt, lint, test, coverage)
```

## Writing Tests

### Basic Event Tests

Test simple MIDI event generation:

```rust
#[test]
fn test_my_feature() {
    let sim = MidiSimulator::new(0);

    // Simulate user action
    sim.note_on(60, 100);
    sim.note_off(60);

    // Verify events
    let events = sim.get_events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0], vec![0x90, 60, 100]); // Note On
    assert_eq!(events[1], vec![0x80, 60, 0x40]); // Note Off
}
```

### Velocity Level Tests

Test velocity-sensitive actions:

```rust
#[test]
fn test_velocity_levels() {
    let sim = MidiSimulator::new(0);

    // Test soft, medium, hard presses
    sim.note_on(60, 30);   // Soft (0-40)
    sim.note_on(60, 70);   // Medium (41-80)
    sim.note_on(60, 110);  // Hard (81-127)

    let events = sim.get_events();
    assert_eq!(events.len(), 3);
}
```

### Timing-Based Tests

Test long press and timing detection:

```rust
#[test]
fn test_long_press() {
    let sim = MidiSimulator::new(0);
    let start = Instant::now();

    sim.perform_gesture(Gesture::LongPress {
        note: 60,
        velocity: 80,
        hold_ms: 2500,
    });

    let duration = start.elapsed();
    assert!(duration >= Duration::from_millis(2500));
}
```

### Double-Tap Tests

Test double-tap detection with gap timing:

```rust
#[test]
fn test_double_tap() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::DoubleTap {
        note: 60,
        velocity: 80,
        tap_duration_ms: 50,
        gap_ms: 200,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 4); // 2 note ons + 2 note offs
}
```

### Chord Tests

Test chord detection with multiple simultaneous notes:

```rust
#[test]
fn test_chord() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::Chord {
        notes: vec![60, 64, 67], // C major chord
        velocity: 80,
        stagger_ms: 10,
        hold_ms: 500,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 6); // 3 note ons + 3 note offs
}
```

### Encoder Tests

Test encoder rotation with direction detection:

```rust
#[test]
fn test_encoder() {
    let sim = MidiSimulator::new(0);

    sim.perform_gesture(Gesture::EncoderTurn {
        cc: 1,
        direction: EncoderDirection::Clockwise,
        steps: 5,
        step_delay_ms: 0,
    });

    let events = sim.get_events();
    assert_eq!(events.len(), 5);

    // Verify values are increasing
    for i in 1..events.len() {
        assert!(events[i][2] > events[i-1][2]);
    }
}
```

### Scenario Builder

Create complex test scenarios with the builder pattern:

```rust
use midi_simulator::ScenarioBuilder;

#[test]
fn test_complex_scenario() {
    let sim = MidiSimulator::new(0);

    let scenario = ScenarioBuilder::new()
        .note_on(60, 100)
        .wait(100)
        .control_change(1, 64)
        .wait(100)
        .aftertouch(80)
        .wait(100)
        .note_off(60)
        .build();

    sim.execute_sequence(scenario);

    let events = sim.get_events();
    assert_eq!(events.len(), 4);
}
```

## Interactive CLI Tool

The simulator includes an interactive command-line interface for manual testing and experimentation.

### Starting the CLI

```bash
cargo run --bin midi_simulator
```

### Available Commands

```
╭─────────────────────────────────────────────────────────────╮
│ COMMANDS                                                    │
├─────────────────────────────────────────────────────────────┤
│ Basic:                                                      │
│   help, h, ?              Show help message                 │
│   quit, exit, q           Exit the simulator                │
│   clear, c                Clear event queue                 │
│   events, e               Show captured events              │
├─────────────────────────────────────────────────────────────┤
│ MIDI Events:                                                │
│   note <num> <vel>        Send note on/off                  │
│   velocity <note>         Test velocity levels              │
│   long <note> [ms]        Simulate long press               │
│   double <note> [gap_ms]  Simulate double-tap               │
│   chord <n1> <n2> ...     Simulate chord                    │
│   encoder <cc> <cw|ccw>   Simulate encoder rotation         │
│   aftertouch <pressure>   Send aftertouch                   │
│   pitch <value>           Send pitch bend (0-16383)         │
│   cc <num> <val>          Send control change               │
├─────────────────────────────────────────────────────────────┤
│ Scenarios:                                                  │
│   demo                    Run full demonstration            │
│   scenario [name]         Run specific test scenario        │
╰─────────────────────────────────────────────────────────────╯
```

### Example Session

```bash
# Start the CLI
cargo run --bin midi_simulator

# Test velocity levels
> velocity 60
Simulating velocity levels (soft, medium, hard)...
✓ Velocity test complete

# Test long press
> long 60 2500
Simulating long press for 2500ms...
✓ Long press complete

# Test double-tap
> double 60 200
Simulating double-tap with 200ms gap...
✓ Double-tap complete

# Test chord (C major)
> chord 60 64 67
Simulating chord: [60, 64, 67]
✓ Chord complete

# Test encoder rotation
> encoder 1 cw 5
Simulating encoder CC1 Clockwise 5 steps...
✓ Encoder simulation complete

# Show captured events
> events
Captured events:
  1: [90, 60, 64, ...]
  2: [80, 60, 40, ...]
  ...

# Run full demo
> demo
Running demonstration scenarios...
1. Testing velocity levels...
2. Testing long press...
3. Testing double-tap...
4. Testing chord...
5. Testing encoder...
✓ Demo complete

# Exit
> quit
Goodbye!
```

## Test Scenarios

The simulator includes pre-built test scenarios for common testing needs.

### Velocity Scenario

Tests all three velocity levels:

```bash
> scenario velocity
Testing velocity levels: Soft (30), Medium (70), Hard (110)
✓ Velocity scenario complete
```

### Timing Scenario

Tests short, medium, and long press durations:

```bash
> scenario timing
Testing press durations: Short (100ms), Medium (500ms), Long (2500ms)
✓ Timing scenario complete
```

### Double-Tap Scenario

Tests double-tap detection:

```bash
> scenario doubletap
Testing double-tap with 200ms gap
✓ Double-tap scenario complete
```

### Chord Scenario

Tests chord detection with C major:

```bash
> scenario chord
Testing chord detection: C major (60, 64, 67)
✓ Chord scenario complete
```

### Encoder Scenario

Tests encoder rotation in both directions:

```bash
> scenario encoder
Testing encoder: 5 steps CW, then 5 steps CCW
✓ Encoder scenario complete
```

### Complex Scenario

Tests mixed events and complex interactions:

```bash
> scenario complex
Running complex scenario: mixed events...
✓ Complex scenario complete
```

## Advanced Gestures

### Velocity Ramp

Simulate a velocity ramp from soft to hard:

```rust
sim.perform_gesture(Gesture::VelocityRamp {
    note: 60,
    min_velocity: 20,
    max_velocity: 120,
    steps: 5,
});
```

### Simple Tap

Simulate a quick tap with precise duration:

```rust
sim.perform_gesture(Gesture::SimpleTap {
    note: 60,
    velocity: 80,
    duration_ms: 100,
});
```

## Event Inspection

### Getting Events

```rust
// Get all events and clear the queue
let events = sim.get_events();

// Peek at last event without clearing
let last = sim.peek_last_event();

// Clear the queue
sim.clear_events();
```

### Parsing Events

```rust
for event in events {
    let status = event[0];
    let message_type = status & 0xF0;
    let channel = status & 0x0F;

    match message_type {
        0x90 => println!("Note On: {} vel {}", event[1], event[2]),
        0x80 => println!("Note Off: {}", event[1]),
        0xB0 => println!("CC{}: {}", event[1], event[2]),
        0xD0 => println!("Aftertouch: {}", event[1]),
        0xE0 => println!("Pitch Bend: {} {}", event[1], event[2]),
        _ => println!("Unknown message type"),
    }
}
```

## Debug Output

Enable debug output to see all MIDI messages:

```rust
let mut sim = MidiSimulator::new(0);
sim.set_debug(true);

sim.note_on(60, 100);
// Output: [SIM] Sending: [90, 3C, 64]
```

## Best Practices

1. **Clear events between tests**: Always clear the event queue between tests to avoid interference
2. **Use gestures for complex interactions**: Prefer high-level gestures over manual event sequences
3. **Verify timing**: Use `Instant::now()` to verify timing-sensitive operations
4. **Test edge cases**: Test velocity boundaries (0, 40, 41, 80, 81, 127)
5. **Test multiple channels**: Verify channel masking works correctly
6. **Use scenario builder**: Build complex scenarios declaratively with the builder pattern

## Continuous Integration

The simulator is designed to work in CI environments without hardware:

```yaml
# .github/workflows/test.yml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Run tests
        run: cargo test --all-features
```

## Troubleshooting

### Tests Timeout

If timing-based tests timeout, increase tolerance:

```rust
assert!(duration >= Duration::from_millis(2500));
assert!(duration < Duration::from_millis(2700)); // 200ms tolerance
```

### Event Count Mismatch

Remember that `get_events()` clears the queue:

```rust
let events1 = sim.get_events(); // Gets and clears
let events2 = sim.get_events(); // Empty, events were already consumed
```

### Velocity Detection

Ensure velocities match expected ranges:

```rust
// Soft: 0-40
// Medium: 41-80
// Hard: 81-127
```

## Integration Test Suites

Conductor includes comprehensive integration test suites that verify complete feature sets without requiring physical hardware.

### Event Processing Tests (AMI-117)

Location: `tests/event_processing_tests.rs`

Tests for aftertouch and pitch bend event processing:

**Aftertouch Tests (26 tests)**:
- Full pressure range validation (0-127)
- Continuous pressure variation
- Boundary value testing (min/max)
- Aftertouch with note press scenarios
- Multi-channel aftertouch support

**Pitch Bend Tests (26 tests)**:
- Center position verification (8192)
- Full 14-bit range testing (0-16383)
- Positive and negative bend ranges
- Smooth sweep simulations
- Pitch bend with note combinations
- Multi-channel pitch bend support

```bash
# Run event processing tests
cargo test --test event_processing_tests
```

### Action Tests (AMI-118)

Location: `tests/action_tests.rs`

Tests for application launch and volume control actions:

**Launch Application Tests (14 tests)**:
- Valid application path handling
- Invalid path error handling
- Paths with spaces
- Process spawning verification
- Permission denied scenarios
- Concurrent process spawning
- Platform-specific behavior detection

**Volume Control Tests (14 tests)**:
- Command detection (macOS, Linux, Windows)
- Volume up/down command structure
- Mute toggle command structure
- Volume set command structure
- Mock volume control execution
- Shell command escaping

```bash
# Run action tests
cargo test --test action_tests
```

### Action Orchestration Tests (AMI-119)

Location: `tests/action_orchestration_tests.rs`

Tests for complex action orchestration (38 tests):

**Sequence Actions (F16)**:
- Action ordering verification
- Empty sequence handling
- Single action sequences
- Sequences with delays
- Error propagation in sequences

**Delay Actions (F17)**:
- Timing accuracy (50ms, 100ms, 500ms)
- Zero delay handling
- Multiple sequential delays
- Timing precision validation (±10ms tolerance)

**MouseClick Actions (F18)**:
- Click simulation structure
- Coordinate validation
- Button type validation (left, right, middle)
- Click sequences with delays

**Repeat Actions (F19)**:
- Repeat count verification
- Repeat with delays
- Zero and single repetitions
- High-volume repeat handling (100+ iterations)

**Conditional Actions (F20)**:
- Application-based conditions
- Time-based conditions (hour ranges)
- Modifier key conditions
- Mode-based conditions
- Multiple condition combinations (AND/OR logic)
- Complex conditional expressions

```bash
# Run action orchestration tests
cargo test --test action_orchestration_tests
```

### End-to-End Tests (AMI-120)

Location: `tests/e2e_tests.rs`

Comprehensive E2E testing of the complete Conductor pipeline (MIDI Input → Event Processing → Mapping → Action Execution):

**Critical Workflows (20 tests)**:
- Simple pad press → keystroke
- Velocity-sensitive mapping (soft/medium/hard)
- Long press detection (≥1000ms threshold)
- Double-tap recognition (<300ms window)
- Chord detection (<50ms window)
- Mode switching via encoder
- Mode-specific vs global mappings
- Action sequences with delays
- Conditional actions (app/time/mode)
- Volume control via encoder

**Performance & Edge Cases (5 tests)**:
- Timing latency verification (<1ms)
- Rapid note events (20+ events)
- Invalid note range handling (0, 1, 126, 127)
- Throughput testing (200 events <10ms)
- Memory stability (1000 events)

```bash
# Run all E2E tests
cargo test --test e2e_tests

# Expected: 37 tests passed
```

### Test Coverage Summary

Total test count: **183 tests**

Breakdown by suite:
- `integration_tests.rs`: 29 tests (basic event processing)
- `event_processing_tests.rs`: 26 tests (aftertouch & pitch bend)
- `action_tests.rs`: 14 tests (launch & volume control)
- `action_orchestration_tests.rs`: 38 tests (sequences & conditionals)
- `e2e_tests.rs`: 37 tests (end-to-end critical workflows) ← **NEW**
- `config_compatibility_test.rs`: 15 tests (config validation)
- `midi_simulator.rs`: 12 tests (simulator validation)
- Additional unit tests: 12 tests (various modules)

### Running All Integration Tests

```bash
# Run all integration tests
cargo test --test integration_tests \
           --test event_processing_tests \
           --test action_tests \
           --test action_orchestration_tests

# Run all tests with coverage
cargo test --all-features
```

### Writing New Integration Tests

When adding new integration tests:

1. **Use the MIDI simulator** for all MIDI event generation
2. **Test edge cases** (boundary values, timing variations)
3. **Include negative tests** (error conditions, invalid inputs)
4. **Verify timing** with tolerance (±10-35ms for CI stability)
5. **Document test purpose** with clear comments
6. **Group related tests** into logical test modules

Example template:

```rust
#[test]
fn test_feature_name() {
    let sim = MidiSimulator::new(0);

    // Setup: Generate test events
    sim.note_on(60, 80);

    // Execute: Perform action
    let events = sim.get_events();

    // Verify: Check results
    assert_eq!(events.len(), 1);
    assert_eq!(events[0][0] & 0xF0, 0x90); // Note On
}
```

### CI/CD Integration

All integration tests run automatically in GitHub Actions:

- **No hardware required**: Uses MIDI simulator
- **Fast execution**: <5 seconds total for all tests
- **Timing tolerance**: Increased for CI environments (±35ms)
- **Platform coverage**: Tests run on macOS, Linux, Windows

## Game Controllers (HID) Testing (v3.0+)

Conductor v3.0 added support for all SDL2-compatible HID devices (gamepads, joysticks, racing wheels, flight sticks, HOTAS, and custom controllers). This section covers comprehensive testing strategies for gamepad functionality.

### Overview

Game controller testing in Conductor covers three main areas:

1. **Unit Tests**: Component-level testing of InputManager, GamepadDeviceManager, and event conversion
2. **Integration Tests**: Multi-component testing of hybrid mode, event streams, and device lifecycle
3. **Manual Testing**: Physical hardware testing with real game controllers

### Unit Tests

#### InputManager Creation Tests

Test InputManager initialization with different input modes:

```bash
# Run InputManager tests
cargo test input_manager
```

**Test Coverage**:
- `InputMode::MidiOnly` - MIDI device only (gamepad_manager = None)
- `InputMode::GamepadOnly` - Gamepad device only (midi_manager = None)
- `InputMode::Both` - Hybrid mode (both managers initialized)
- Auto-reconnection configuration propagation
- Device name configuration

Example test:

```rust
#[test]
fn test_input_manager_gamepad_only_mode() {
    use conductor_daemon::input_manager::{InputManager, InputMode};

    let manager = InputManager::new(
        None,  // No MIDI device
        true,  // auto_reconnect
        InputMode::GamepadOnly
    );

    // Verify only gamepad manager is initialized
    assert!(manager.has_gamepad_manager());
    assert!(!manager.has_midi_manager());
}
```

#### GamepadDeviceManager Lifecycle Tests

Test gamepad connection, disconnection, and state management:

```bash
# Run gamepad-specific unit tests
cargo test gamepad
```

**Test Coverage**:
- Device detection and enumeration
- Connection lifecycle (connect → active → disconnect)
- Connection state tracking (is_connected flag)
- Device ID assignment (0-based indexing)
- Device name retrieval
- Thread safety (Arc/Mutex patterns)

Example test:

```rust
#[test]
fn test_gamepad_connection_lifecycle() {
    use conductor_daemon::gamepad_device::GamepadDeviceManager;
    use tokio::sync::mpsc;

    let (event_tx, _event_rx) = mpsc::channel(1024);
    let (command_tx, _command_rx) = mpsc::channel(32);

    let mut manager = GamepadDeviceManager::new(true);

    // Test connection (requires physical gamepad)
    match manager.connect(event_tx, command_tx) {
        Ok((id, name)) => {
            println!("Connected: {} (ID {})", name, id);
            assert!(manager.is_connected());
        }
        Err(_) => {
            // Expected when no gamepad is connected
            assert!(!manager.is_connected());
        }
    }
}
```

#### Event Conversion Tests (HID → InputEvent)

Test conversion of gamepad events to InputEvent format:

**Test Coverage**:
- Button press → `InputEvent::PadPressed` (IDs 128-255)
- Button release → `InputEvent::PadReleased` (IDs 128-255)
- Analog stick movement → `InputEvent::EncoderTurned` (X/Y axes)
- Trigger pull → `InputEvent::EncoderTurned` (analog triggers)
- D-pad press → `InputEvent::PadPressed` (direction buttons)

Example test (from `conductor-core/tests/gamepad_input_test.rs`):

```rust
#[test]
fn test_gamepad_button_press_detection() {
    let mut processor = EventProcessor::new();

    // Gamepad button press (button ID 128 = South/A/Cross/B)
    let event = InputEvent::PadPressed {
        pad: 128,
        velocity: 100,
        time: Instant::now(),
    };

    let processed = processor.process_input(event);

    // Should detect PadPressed with velocity level
    assert_eq!(processed.len(), 1);

    match &processed[0] {
        ProcessedEvent::PadPressed {
            note,
            velocity,
            velocity_level,
        } => {
            assert_eq!(*note, 128);
            assert_eq!(*velocity, 100);
            assert_eq!(*velocity_level, VelocityLevel::Hard); // 100 is in Hard range (81-127)
        }
        _ => panic!("Expected PadPressed event"),
    }
}
```

#### ID Range Validation Tests

Verify gamepad IDs are correctly mapped to 128-255 range:

**Test Coverage**:
- Button IDs: 128-143 (Face buttons: 128-131, D-pad: 132-135, Shoulder buttons: 136-139, etc.)
- Analog stick IDs: 128-131 (Left X: 128, Left Y: 129, Right X: 130, Right Y: 131)
- Trigger IDs: 132-133 (Left trigger: 132, Right trigger: 133)
- No collision with MIDI note range (0-127)

Example test:

```rust
#[test]
fn test_gamepad_id_range_no_midi_collision() {
    use conductor_core::events::InputEvent;
    use std::time::Instant;

    let time = Instant::now();

    // Test button IDs are >= 128
    let button_event = InputEvent::PadPressed {
        pad: 128,  // South button
        velocity: 100,
        time,
    };

    match button_event {
        InputEvent::PadPressed { pad, .. } => {
            assert!(pad >= 128, "Gamepad button ID must be >= 128");
            assert!(pad <= 255, "Gamepad button ID must be <= 255");
        }
        _ => panic!("Expected PadPressed"),
    }

    // Test analog stick IDs are >= 128
    let stick_event = InputEvent::EncoderTurned {
        encoder: 128,  // Left stick X
        value: 64,
        time,
    };

    match stick_event {
        InputEvent::EncoderTurned { encoder, .. } => {
            assert!(encoder >= 128, "Gamepad analog ID must be >= 128");
            assert!(encoder <= 255, "Gamepad analog ID must be <= 255");
        }
        _ => panic!("Expected EncoderTurned"),
    }
}
```

#### Button/Axis Mapping Correctness Tests

Verify correct mapping of standard gamepad layout:

**Test Coverage**:
- Face buttons (South/East/West/North)
- D-pad (Up/Down/Left/Right)
- Shoulder buttons (L1/R1/L2/R2)
- Stick buttons (L3/R3)
- Start/Select/Guide buttons
- Analog sticks (Left/Right X/Y)
- Analog triggers (L2/R2)

Example test:

```rust
#[test]
fn test_standard_gamepad_button_mapping() {
    // Standard SDL2 gamepad button mappings
    const BUTTON_SOUTH: u8 = 128;    // A/Cross/B
    const BUTTON_EAST: u8 = 129;     // B/Circle/A
    const BUTTON_WEST: u8 = 130;     // X/Square/Y
    const BUTTON_NORTH: u8 = 131;    // Y/Triangle/X
    const DPAD_UP: u8 = 132;
    const DPAD_DOWN: u8 = 133;
    const DPAD_LEFT: u8 = 134;
    const DPAD_RIGHT: u8 = 135;

    // Verify no ID collisions
    let button_ids = vec![
        BUTTON_SOUTH, BUTTON_EAST, BUTTON_WEST, BUTTON_NORTH,
        DPAD_UP, DPAD_DOWN, DPAD_LEFT, DPAD_RIGHT
    ];

    let mut seen = std::collections::HashSet::new();
    for id in button_ids {
        assert!(seen.insert(id), "Duplicate button ID: {}", id);
        assert!(id >= 128, "Button ID must be >= 128");
    }
}
```

### Integration Tests

Integration tests verify multi-component interactions and complex workflows.

#### Hybrid Mode Event Stream Tests

Test simultaneous MIDI + gamepad event processing:

```bash
# Run integration tests
cargo test --test integration
```

**Test Coverage**:
- Simultaneous MIDI and gamepad events
- Event stream merging (single InputEvent channel)
- No event loss or corruption
- Correct event ordering
- Thread synchronization

Example integration test:

```rust
#[test]
async fn test_hybrid_mode_event_stream() {
    use conductor_daemon::input_manager::{InputManager, InputMode};
    use tokio::sync::mpsc;

    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _command_rx) = mpsc::channel(32);

    let mut manager = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::Both  // Hybrid mode
    );

    // Connect both devices
    manager.connect(event_tx, command_tx).unwrap();

    // Simulate MIDI event
    // ... (MIDI event simulation)

    // Simulate gamepad event
    // ... (gamepad event simulation)

    // Verify both events arrive in order
    let event1 = event_rx.recv().await.unwrap();
    let event2 = event_rx.recv().await.unwrap();

    // Verify event types and ordering
    // ...
}
```

#### MIDI + Gamepad Event Ordering Tests

Verify events maintain temporal ordering:

**Test Coverage**:
- Timestamp-based ordering
- No race conditions
- Event interleaving
- Microsecond-level timing precision

Example test:

```rust
#[test]
fn test_event_ordering_with_timestamps() {
    use conductor_core::events::InputEvent;
    use std::time::{Duration, Instant};

    let base_time = Instant::now();

    // Create events with precise timestamps
    let midi_event = InputEvent::PadPressed {
        pad: 60,  // MIDI note
        velocity: 100,
        time: base_time,
    };

    let gamepad_event = InputEvent::PadPressed {
        pad: 128,  // Gamepad button
        velocity: 100,
        time: base_time + Duration::from_millis(10),
    };

    // Verify timestamps for ordering
    match (midi_event, gamepad_event) {
        (InputEvent::PadPressed { time: t1, .. },
         InputEvent::PadPressed { time: t2, .. }) => {
            assert!(t2 > t1, "Gamepad event should have later timestamp");
        }
        _ => panic!("Expected PadPressed events"),
    }
}
```

#### Device Disconnection/Reconnection Tests

Test automatic reconnection behavior:

**Test Coverage**:
- Detect device disconnection
- Automatic reconnection attempts
- Exponential backoff (1s, 2s, 4s, 8s, 16s, 30s)
- Maximum retry limit (6 attempts)
- State restoration after reconnection
- Event stream recovery

Example test:

```rust
#[test]
async fn test_gamepad_reconnection_logic() {
    use conductor_daemon::gamepad_device::GamepadDeviceManager;
    use tokio::sync::mpsc;
    use std::time::Duration;

    let (event_tx, _event_rx) = mpsc::channel(1024);
    let (command_tx, mut command_rx) = mpsc::channel(32);

    let mut manager = GamepadDeviceManager::new(true);  // auto_reconnect = true

    // Simulate disconnection by connecting then disconnecting
    if let Ok(_) = manager.connect(event_tx.clone(), command_tx.clone()) {
        manager.disconnect();

        // Verify disconnection detected
        assert!(!manager.is_connected());

        // Wait for reconnection attempt (background thread)
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Check for reconnection command
        if let Ok(cmd) = tokio::time::timeout(
            Duration::from_secs(1),
            command_rx.recv()
        ).await {
            // Verify reconnection command received
            // ...
        }
    }
}
```

#### Auto-Reconnection Logic Tests

Verify reconnection backoff and retry behavior:

**Test Coverage**:
- Backoff schedule: 1s, 2s, 4s, 8s, 16s, 30s
- Maximum 6 attempts
- DaemonCommand::DeviceReconnectionResult sent on completion
- No resource leaks during retries
- Thread cleanup on failure

#### Mode Switching with Gamepad Tests

Test switching between input modes during runtime:

**Test Coverage**:
- Switch from MidiOnly to Both
- Switch from GamepadOnly to Both
- Switch from Both to MidiOnly
- Switch from Both to GamepadOnly
- Clean device disconnection during mode change
- No event loss during transition

Example test:

```rust
#[test]
fn test_mode_switching_midi_to_hybrid() {
    use conductor_daemon::input_manager::{InputManager, InputMode};

    // Start with MIDI only
    let mut manager = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::MidiOnly
    );

    assert!(manager.has_midi_manager());
    assert!(!manager.has_gamepad_manager());

    // Switch to hybrid mode (would require runtime mode switching API)
    // Note: Current implementation requires manager recreation
    // Future enhancement: dynamic mode switching

    // Create new manager with Both mode
    let manager_hybrid = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::Both
    );

    assert!(manager_hybrid.has_midi_manager());
    assert!(manager_hybrid.has_gamepad_manager());
}
```

### Manual Testing

Manual testing with physical game controllers is essential for validating real-world behavior.

#### Physical Gamepad Connection

**Test Procedure**:
1. Connect physical gamepad via USB or Bluetooth
2. Launch Conductor daemon with gamepad support
3. Verify gamepad detected and connected
4. Check logs for connection confirmation

```bash
# Start daemon with gamepad-only mode
cargo run --release -- --input-mode gamepad

# Or hybrid mode (MIDI + gamepad)
cargo run --release -- --input-mode both

# Check logs for connection status
tail -f ~/.conductor/daemon.log
```

**Expected Output**:
```
[INFO] Gamepad connected: Xbox Series Controller (ID 0)
[INFO] Gamepad events: 15 buttons, 6 axes
[INFO] Polling thread started
```

#### Button Mapping Verification

**Test Procedure**:
1. Press each button on the gamepad
2. Verify correct button ID assigned (128-255)
3. Check Event Console for button events
4. Verify no duplicate IDs

**Manual Test Checklist**:
- [ ] South button (A/Cross/B) → ID 128
- [ ] East button (B/Circle/A) → ID 129
- [ ] West button (X/Square/Y) → ID 130
- [ ] North button (Y/Triangle/X) → ID 131
- [ ] D-Pad Up → ID 132
- [ ] D-Pad Down → ID 133
- [ ] D-Pad Left → ID 134
- [ ] D-Pad Right → ID 135
- [ ] Left Shoulder (L1/LB) → ID 136
- [ ] Right Shoulder (R1/RB) → ID 137
- [ ] Left Trigger Button (L2/LT) → ID 138 (if digital)
- [ ] Right Trigger Button (R2/RT) → ID 139 (if digital)
- [ ] Left Stick Button (L3) → ID 140
- [ ] Right Stick Button (R3) → ID 141
- [ ] Start/Options → ID 142
- [ ] Select/Share → ID 143

**Verification Command**:
```bash
# Open Event Console in GUI
# Press each button and verify ID appears correctly
```

#### Analog Stick Dead Zone Testing

Test dead zone behavior for analog sticks:

**Test Procedure**:
1. Leave analog sticks at center (neutral) position
2. Verify no events generated (dead zone active)
3. Move stick slightly (within dead zone)
4. Verify no events still (dead zone threshold)
5. Move stick beyond dead zone
6. Verify `EncoderTurned` events generated
7. Return stick to center
8. Verify events stop (dead zone reactivated)

**Dead Zone Configuration** (default: 0.1 or 10%):
```rust
// Dead zone prevents drift from neutral position
const ANALOG_DEAD_ZONE: f32 = 0.1;  // 10% of full range
```

**Manual Test Checklist**:
- [ ] Left stick neutral → no events
- [ ] Left stick small movement → no events (within dead zone)
- [ ] Left stick large movement → events generated
- [ ] Left stick return to center → events stop
- [ ] Right stick neutral → no events
- [ ] Right stick small movement → no events
- [ ] Right stick large movement → events generated
- [ ] Right stick return to center → events stop

#### Trigger Threshold Testing

Test analog trigger activation thresholds:

**Test Procedure**:
1. Leave triggers released (0.0 position)
2. Verify no events generated
3. Pull trigger slightly (below threshold)
4. Verify no events (threshold not met)
5. Pull trigger beyond threshold
6. Verify `EncoderTurned` events generated
7. Release trigger
8. Verify events stop

**Trigger Configuration** (default threshold: 0.1 or 10%):
```rust
// Threshold for analog trigger activation
const TRIGGER_THRESHOLD: f32 = 0.1;  // 10% of full pull
```

**Manual Test Checklist**:
- [ ] Left trigger released → no events
- [ ] Left trigger slight pull → no events (below threshold)
- [ ] Left trigger half pull → events generated
- [ ] Left trigger full pull → events generated (max value)
- [ ] Left trigger release → events stop
- [ ] Right trigger released → no events
- [ ] Right trigger slight pull → no events
- [ ] Right trigger half pull → events generated
- [ ] Right trigger full pull → events generated
- [ ] Right trigger release → events stop

#### Template Loading Verification

Test gamepad template loading:

**Test Procedure**:
1. Create gamepad template file (TOML)
2. Place in `~/.conductor/templates/` directory
3. Select template in GUI
4. Verify mappings loaded correctly
5. Test button mappings from template
6. Verify actions execute correctly

**Example Template** (Xbox controller):
```toml
# ~/.conductor/templates/xbox-series-controller.toml

[device]
name = "Xbox Series Controller"
type = "gamepad"

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
trigger = { PadPressed = { pad = 128, velocity_range = [0, 127] } }  # A button
action = { Keystroke = { key = "Space", modifiers = [] } }

[[modes.mappings]]
trigger = { PadPressed = { pad = 129 } }  # B button
action = { Keystroke = { key = "Escape", modifiers = [] } }
```

**Verification Commands**:
```bash
# List available templates
conductorctl templates list

# Load template
conductorctl templates load xbox-series-controller

# Verify template active
conductorctl status
```

**Manual Test Checklist**:
- [ ] Template file exists and is valid TOML
- [ ] Template appears in GUI template selector
- [ ] Template loads without errors
- [ ] Mappings appear in mapping list
- [ ] Button presses trigger correct actions
- [ ] LED feedback works (if supported)

#### MIDI Learn with Gamepad

Test MIDI Learn mode with gamepad buttons:

**Test Procedure**:
1. Open GUI configuration
2. Create new mapping
3. Click "MIDI Learn" button
4. Press gamepad button
5. Verify button ID captured (128-255)
6. Assign action to mapping
7. Test mapping works

**Manual Test Checklist**:
- [ ] MIDI Learn mode activates
- [ ] Gamepad button press detected
- [ ] Correct button ID captured
- [ ] Button ID displayed in UI
- [ ] Mapping saved successfully
- [ ] Mapping triggers action correctly
- [ ] Multiple gamepad buttons can be learned
- [ ] Chord detection works (multiple buttons)
- [ ] Long press detection works
- [ ] Double-tap detection works

#### Device Disconnection/Reconnection

Test device hot-plugging:

**Test Procedure**:
1. Connect gamepad and verify active
2. Physically disconnect gamepad (unplug USB or disable Bluetooth)
3. Verify daemon detects disconnection
4. Wait for reconnection attempts (check logs)
5. Reconnect gamepad
6. Verify daemon reconnects automatically
7. Test button presses work after reconnection

**Manual Test Checklist**:
- [ ] Daemon detects disconnection immediately
- [ ] Logs show "Gamepad disconnected" message
- [ ] Reconnection attempts start (1s, 2s, 4s, 8s, 16s, 30s backoff)
- [ ] Logs show reconnection attempts
- [ ] Gamepad reconnects when plugged back in
- [ ] Logs show "Gamepad reconnected" message
- [ ] Button presses work immediately after reconnection
- [ ] No event loss after reconnection
- [ ] State restored (mode, mappings, etc.)

#### Cross-Platform Verification

Test gamepad support across different operating systems:

**Platform-Specific Testing**:

**macOS**:
- [ ] USB gamepad detection
- [ ] Bluetooth gamepad detection
- [ ] Xbox controller support
- [ ] PlayStation controller support
- [ ] Nintendo Switch Pro controller support
- [ ] Generic HID gamepad support
- [ ] Input Monitoring permissions granted

**Linux**:
- [ ] USB gamepad detection via evdev
- [ ] Bluetooth gamepad detection
- [ ] Xbox controller support (xpad kernel module)
- [ ] PlayStation controller support (hid-sony kernel module)
- [ ] udev rules configured correctly
- [ ] Permissions for `/dev/input/event*`

**Windows**:
- [ ] USB gamepad detection
- [ ] Bluetooth gamepad detection
- [ ] Xbox controller support (native)
- [ ] PlayStation controller support (DS4Windows)
- [ ] DirectInput gamepad support
- [ ] XInput gamepad support

### Platform-Specific Testing Notes

#### macOS

**Hardware Requirements**:
- Real hardware required (no emulation available)
- Native SDL2 support via macOS HID APIs

**Permissions**:
- Input Monitoring permissions required (System Settings → Privacy & Security)
- Grant permissions to Terminal or Conductor daemon

**Testing Approach**:
- Use physical controllers only
- Test native Apple controllers (PS5, Xbox Series)
- Test third-party controllers (8BitDo, Logitech)

```bash
# Check Input Monitoring permissions
tccutil reset SystemPolicyInputMonitoring

# Grant permissions to Terminal
sudo sqlite3 ~/Library/Application\ Support/com.apple.TCC/TCC.db \
  "INSERT INTO access VALUES('kTCCServiceAccessibility','com.apple.Terminal',0,1,1,NULL,NULL,NULL,NULL,NULL,NULL,NULL);"
```

#### Linux

**Hardware Requirements**:
- Real hardware preferred
- evdev emulation possible with `evemu-device`

**Permissions**:
- User must be in `input` group
- udev rules required for device access

**Testing Approach**:
- Test with real controllers via USB/Bluetooth
- Use evdev emulation for CI/CD testing
- Test with various kernel modules (xpad, hid-sony)

```bash
# Add user to input group
sudo usermod -a -G input $USER

# Create udev rule for gamepad access
echo 'KERNEL=="event*", SUBSYSTEM=="input", MODE="0666"' | \
  sudo tee /etc/udev/rules.d/99-input.rules

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# List connected gamepads
ls -l /dev/input/event*

# Test with evtest
sudo evtest /dev/input/event0
```

**evdev Emulation for Testing**:
```bash
# Install evemu tools
sudo apt-get install evemu-tools

# Record gamepad events to file
sudo evemu-record /dev/input/event0 > gamepad.events

# Replay events for testing
sudo evemu-play /dev/input/event0 < gamepad.events
```

#### Windows

**Hardware Requirements**:
- Real hardware preferred
- Virtual gamepad possible with vJoy

**Testing Approach**:
- Test with real Xbox/PlayStation controllers
- Use vJoy for virtual gamepad testing
- Test both DirectInput and XInput modes

**vJoy for Virtual Gamepads**:
```powershell
# Install vJoy
# Download from: https://sourceforge.net/projects/vjoystick/

# Configure virtual gamepad
vJoyConf.exe

# Test with gamepad tester
# Download from: https://gamepad-tester.com/
```

### Test Coverage Requirements

Conductor maintains high test coverage standards for gamepad functionality:

#### Core Functionality Coverage (Target: 90%+)

- **InputManager**: 95% line coverage
  - Mode selection (MidiOnly, GamepadOnly, Both)
  - Device initialization
  - Connection lifecycle
  - Event stream merging

- **GamepadDeviceManager**: 90% line coverage
  - Device detection
  - Connection/disconnection
  - Event polling loop
  - Reconnection logic
  - State management

- **Event Conversion**: 95% line coverage
  - Button press/release conversion
  - Analog stick conversion
  - Trigger conversion
  - ID range validation

#### Edge Cases Coverage (Target: 85%+)

- **Device Not Found**:
  - No gamepad connected
  - Invalid device ID
  - Device disconnected during operation
  - Rapid connect/disconnect cycles

- **SDL2 Unavailable**:
  - SDL2 library not installed
  - SDL2 initialization failure
  - gilrs library unavailable

- **Error Handling Paths**:
  - Connection timeout
  - Thread spawn failure
  - Channel send/receive errors
  - Reconnection limit exceeded

Example edge case test:

```rust
#[test]
fn test_no_gamepad_connected_error() {
    use conductor_daemon::gamepad_device::GamepadDeviceManager;
    use tokio::sync::mpsc;

    let (event_tx, _event_rx) = mpsc::channel(1024);
    let (command_tx, _command_rx) = mpsc::channel(32);

    let mut manager = GamepadDeviceManager::new(false);  // auto_reconnect = false

    // Attempt connection with no gamepad present
    let result = manager.connect(event_tx, command_tx);

    // Should return error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("No gamepad detected"));

    // Manager should remain disconnected
    assert!(!manager.is_connected());
}
```

### Test Commands Reference

```bash
# Run all gamepad-specific unit tests
cargo test gamepad

# Run InputManager tests
cargo test input_manager

# Run integration tests (requires physical gamepad)
cargo test --test integration

# Run all tests with verbose output
cargo test gamepad -- --nocapture

# Run tests and show timing
cargo nextest run gamepad

# Run tests with coverage
cargo llvm-cov --test gamepad --html

# Run specific test
cargo test test_gamepad_button_press_detection
```

### Test Fixtures and Mocking

#### Mock Gamepad Devices for CI/CD

For CI/CD environments without physical hardware:

**Linux (evdev emulation)**:
```rust
#[cfg(test)]
mod mock_gamepad {
    use std::process::Command;

    pub fn create_virtual_gamepad() -> Result<String, String> {
        // Create virtual gamepad using evemu
        let output = Command::new("evemu-device")
            .arg("/path/to/gamepad.desc")
            .output()
            .map_err(|e| format!("Failed to create virtual gamepad: {}", e))?;

        if output.status.success() {
            let device = String::from_utf8_lossy(&output.stdout);
            Ok(device.trim().to_string())
        } else {
            Err("Failed to create virtual gamepad".to_string())
        }
    }
}
```

**Windows (vJoy)**:
```rust
#[cfg(target_os = "windows")]
#[cfg(test)]
mod mock_gamepad {
    use winapi::um::winuser::*;

    pub fn create_virtual_gamepad() -> Result<(), String> {
        // Initialize vJoy device
        // ...
        Ok(())
    }
}
```

#### Simulated HID Events

For unit tests without physical devices:

```rust
#[cfg(test)]
mod simulated_events {
    use conductor_core::events::InputEvent;
    use std::time::Instant;

    pub fn simulate_button_press(button: u8) -> InputEvent {
        InputEvent::PadPressed {
            pad: button,
            velocity: 100,
            time: Instant::now(),
        }
    }

    pub fn simulate_button_release(button: u8) -> InputEvent {
        InputEvent::PadReleased {
            pad: button,
            time: Instant::now(),
        }
    }

    pub fn simulate_analog_stick_movement(axis: u8, value: u8) -> InputEvent {
        InputEvent::EncoderTurned {
            encoder: axis,
            value,
            time: Instant::now(),
        }
    }
}
```

#### Test Data for Button/Axis Mapping

Standard test data for gamepad button/axis mapping:

```rust
#[cfg(test)]
mod test_data {
    // Standard gamepad button IDs (Xbox layout)
    pub const BUTTON_SOUTH: u8 = 128;     // A/Cross/B
    pub const BUTTON_EAST: u8 = 129;      // B/Circle/A
    pub const BUTTON_WEST: u8 = 130;      // X/Square/Y
    pub const BUTTON_NORTH: u8 = 131;     // Y/Triangle/X
    pub const DPAD_UP: u8 = 132;
    pub const DPAD_DOWN: u8 = 133;
    pub const DPAD_LEFT: u8 = 134;
    pub const DPAD_RIGHT: u8 = 135;
    pub const LEFT_SHOULDER: u8 = 136;    // L1/LB
    pub const RIGHT_SHOULDER: u8 = 137;   // R1/RB
    pub const LEFT_TRIGGER_BTN: u8 = 138; // L2/LT (digital)
    pub const RIGHT_TRIGGER_BTN: u8 = 139;// R2/RT (digital)
    pub const LEFT_STICK: u8 = 140;       // L3
    pub const RIGHT_STICK: u8 = 141;      // R3
    pub const START: u8 = 142;
    pub const SELECT: u8 = 143;

    // Analog axis IDs
    pub const LEFT_STICK_X: u8 = 128;
    pub const LEFT_STICK_Y: u8 = 129;
    pub const RIGHT_STICK_X: u8 = 130;
    pub const RIGHT_STICK_Y: u8 = 131;
    pub const LEFT_TRIGGER: u8 = 132;     // L2/LT (analog)
    pub const RIGHT_TRIGGER: u8 = 133;    // R2/RT (analog)

    // Test velocity values
    pub const VELOCITY_SOFT: u8 = 30;     // 0-40
    pub const VELOCITY_MEDIUM: u8 = 60;   // 41-80
    pub const VELOCITY_HARD: u8 = 100;    // 81-127

    // Test analog values (0-127 normalized)
    pub const ANALOG_CENTER: u8 = 64;
    pub const ANALOG_MIN: u8 = 0;
    pub const ANALOG_MAX: u8 = 127;
}
```

### Debugging Tips

#### Event Console Usage

The GUI Event Console is invaluable for debugging gamepad events:

1. Open Conductor GUI
2. Navigate to "Event Console" tab
3. Press gamepad buttons/move sticks
4. Observe live event stream with IDs and values

**Event Console Output Example**:
```
[14:23:45.123] PadPressed { pad: 128, velocity: 100 }  // South button
[14:23:45.234] PadReleased { pad: 128 }
[14:23:46.001] EncoderTurned { encoder: 128, value: 95, direction: CW, delta: 31 }  // Left stick X
[14:23:46.112] PadPressed { pad: 132, velocity: 100 }  // D-Pad Up
```

#### Log Inspection

Enable debug logging for gamepad module:

```bash
# Set RUST_LOG environment variable
export RUST_LOG=conductor_daemon::gamepad_device=debug

# Or for all Conductor modules
export RUST_LOG=conductor=debug

# Run daemon
cargo run --release
```

**Log Output Example**:
```
[DEBUG conductor_daemon::gamepad_device] Gamepad 0 connected: Xbox Series Controller
[DEBUG conductor_daemon::gamepad_device] Polling thread started for gamepad 0
[DEBUG conductor_daemon::gamepad_device] Button pressed: South (128) velocity 100
[DEBUG conductor_daemon::gamepad_device] Axis movement: LeftX (128) value 95 delta 31
[DEBUG conductor_daemon::gamepad_device] Button released: South (128)
```

#### gilrs Event Debugging

For low-level HID event debugging:

```rust
#[cfg(test)]
fn debug_gilrs_events() {
    use gilrs::{Gilrs, Event};

    let mut gilrs = Gilrs::new().unwrap();

    println!("Detected gamepads:");
    for (_id, gamepad) in gilrs.gamepads() {
        println!("  {} (ID: {})", gamepad.name(), gamepad.id());
        println!("    Buttons: {}", gamepad.buttons().count());
        println!("    Axes: {}", gamepad.axes().count());
    }

    println!("\nPress buttons to see raw gilrs events (Ctrl+C to exit):");

    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("[{:?}] Gamepad {}: {:?}", time, id, event);
        }
    }
}
```

**Run Debug Tool**:
```bash
cargo test debug_gilrs_events -- --nocapture --ignored
```

### Continuous Integration (CI/CD)

Gamepad tests in CI environments require special considerations:

#### GitHub Actions Configuration

```yaml
name: Gamepad Tests

on: [push, pull_request]

jobs:
  test-gamepad-unit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      # Unit tests don't require physical hardware
      - name: Run gamepad unit tests
        run: cargo test gamepad --lib

      - name: Run InputManager tests
        run: cargo test input_manager

  test-gamepad-integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1

      # Install evemu for virtual gamepad
      - name: Install evemu
        run: sudo apt-get install -y evemu-tools

      # Create virtual gamepad
      - name: Setup virtual gamepad
        run: |
          sudo evemu-device ./tests/fixtures/virtual_gamepad.desc &
          sleep 2

      # Run integration tests with virtual gamepad
      - name: Run gamepad integration tests
        run: cargo test --test integration -- gamepad
```

#### Coverage in CI

```yaml
- name: Generate gamepad test coverage
  run: |
    cargo install cargo-llvm-cov
    cargo llvm-cov --test gamepad --lcov --output-path lcov-gamepad.info

- name: Upload coverage to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: lcov-gamepad.info
    flags: gamepad
```

### Manual Test Checklist

Complete manual testing checklist for game controller support:

#### Device Detection
- [ ] Gamepad detected via USB
- [ ] Gamepad detected via Bluetooth
- [ ] Multiple gamepads detected simultaneously
- [ ] Gamepad ID assigned correctly (0-based)
- [ ] Gamepad name retrieved correctly

#### Button ID Mapping (128-255)
- [ ] Face buttons (South/East/West/North): 128-131
- [ ] D-Pad (Up/Down/Left/Right): 132-135
- [ ] Shoulder buttons (L1/R1): 136-137
- [ ] Trigger buttons (L2/R2 digital): 138-139
- [ ] Stick buttons (L3/R3): 140-141
- [ ] Start/Select buttons: 142-143
- [ ] No ID collision with MIDI notes (0-127)

#### Analog Stick Movement
- [ ] Left stick X axis detected (ID 128)
- [ ] Left stick Y axis detected (ID 129)
- [ ] Right stick X axis detected (ID 130)
- [ ] Right stick Y axis detected (ID 131)
- [ ] Dead zone prevents drift (<10% movement)
- [ ] Full range movement (0-127 values)
- [ ] Direction detection (Clockwise/CounterClockwise)

#### Trigger Pull Detection
- [ ] Left trigger analog detected (ID 132)
- [ ] Right trigger analog detected (ID 133)
- [ ] Trigger threshold prevents noise (<10% pull)
- [ ] Full pull range (0-127 values)
- [ ] Smooth value transitions

#### Hybrid MIDI + Gamepad
- [ ] Both MIDI and gamepad devices connected
- [ ] Events from both devices processed
- [ ] No event loss or corruption
- [ ] Correct event ordering maintained
- [ ] Mode switching works with both inputs

#### Template Loading
- [ ] Gamepad template loads successfully
- [ ] Mappings appear in mapping list
- [ ] Button presses trigger correct actions
- [ ] Template selector shows gamepad templates
- [ ] Template validation passes

#### MIDI Learn with Gamepad
- [ ] MIDI Learn mode captures gamepad buttons
- [ ] Button IDs 128-255 displayed correctly
- [ ] Long press detection works in MIDI Learn
- [ ] Double-tap detection works in MIDI Learn
- [ ] Chord detection works (multiple buttons)

#### Device Disconnection/Reconnection
- [ ] Disconnection detected immediately
- [ ] Reconnection attempts start (exponential backoff)
- [ ] Gamepad reconnects when available
- [ ] Event stream resumes after reconnection
- [ ] State restored (mappings, mode, etc.)
- [ ] Maximum retry limit enforced (6 attempts)

#### Cross-Platform Verification
- [ ] macOS: USB gamepad detection
- [ ] macOS: Bluetooth gamepad detection
- [ ] macOS: Input Monitoring permissions granted
- [ ] Linux: USB gamepad detection (evdev)
- [ ] Linux: Bluetooth gamepad detection
- [ ] Linux: udev rules configured
- [ ] Windows: USB gamepad detection
- [ ] Windows: Bluetooth gamepad detection
- [ ] Windows: XInput/DirectInput support

#### Event Console
- [ ] Gamepad events appear in Event Console
- [ ] Button IDs displayed correctly (128-255)
- [ ] Velocity values displayed correctly
- [ ] Analog values displayed correctly
- [ ] Timestamps accurate
- [ ] Event filtering works

#### Performance
- [ ] Event latency <5ms
- [ ] No event drops at 1000Hz polling
- [ ] CPU usage <5% during active use
- [ ] Memory usage stable (<10MB increase)
- [ ] Thread cleanup on disconnection

## Related Documentation

- [Event Processing Architecture](../architecture/event-processing.md)
- [MIDI Event Types](../reference/midi-events.md)
- [Action System](../reference/actions.md)
- [Game Controller Support](../guides/game-controllers.md)
- [Contributing Guide](../contributing.md)

## Examples

See the integration tests in `tests/integration_tests.rs` for complete examples of:
- Velocity detection tests
- Long press simulation
- Double-tap detection
- Chord detection
- Encoder simulation
- Complex multi-event scenarios

Additional examples in specialized test suites:
- `tests/event_processing_tests.rs`: Aftertouch and pitch bend
- `tests/action_tests.rs`: Application launch and volume control
- `tests/action_orchestration_tests.rs`: Action sequences and conditionals
- `conductor-core/tests/gamepad_input_test.rs`: Game controller event processing

## Support

For questions or issues with testing:
- Check existing integration tests for examples
- Use the interactive CLI tool to experiment
- Review the simulator source code in `tests/midi_simulator.rs`
- Check gamepad tests in `conductor-core/tests/gamepad_input_test.rs`
- Open an issue on GitHub with test failure details
