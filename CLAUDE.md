# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Status

✅ **v3.0: Game Controllers (HID) Support Complete**

Conductor has completed v3.0 development, adding comprehensive support for all SDL2-compatible HID devices (gamepads, joysticks, racing wheels, flight sticks, HOTAS, and custom controllers) alongside existing MIDI controller support.

### Current Architecture (v3.0)

Conductor uses a **3-crate workspace structure** with unified input device support:

1. **conductor-core**: Pure Rust engine library (truly UI-independent)
   - Event processing, mapping engine, configuration
   - **Unified InputEvent abstraction** for MIDI and Game Controllers (HID)
   - **ID range allocation**: MIDI (0-127), HID (128-255)
   - **Action definitions only** (execution moved to daemon in Phase 2)
   - Config loading with security validation
   - Device profiles, error types
   - Public API for external integrations
   - **gamepad_events.rs**: Protocol-agnostic event mapping (gilrs v0.10)
   - **60+ tests passing** (100% pass rate)

2. **conductor-daemon**: Background service with system interaction
   - **Action Executor** (Phase 2: moved from core)
     - Keyboard/mouse simulation via enigo
     - Shell command execution
     - Application launching
     - Volume control
   - **Input Management** (v3.0: Unified MIDI + HID)
     - InputManager: Unified device management layer
     - GamepadDeviceManager: HID device lifecycle and polling
     - MidiDeviceManager: MIDI device lifecycle
     - InputMode enum: MidiOnly, GamepadOnly, Both
     - Single unified InputEvent stream
   - **Daemon infrastructure** (7 modules, ~2,000 lines)
     - IPC server (Unix domain sockets, JSON protocol)
     - Config hot-reload with file watching (500ms debounce)
     - State persistence (atomic writes, SHA256 checksums)
     - Lifecycle management (8-state machine)
     - Performance metrics with grading system
   - **CLI tools**:
     - `conductorctl` - Daemon control (status, reload, stop, validate, ping)
     - `conductor` - Main daemon binary
   - **Diagnostic tools** (6 binaries)
   - **40+ tests passing** + 1 ignored (file watching)

3. **conductor-gui**: Tauri v2 configuration interface
   - Full CRUD operations for modes, mappings, devices, settings
   - Real-time daemon synchronization
   - **MIDI Learn mode with gamepad support** (v3.0)
   - **Gamepad template selector** (v3.0)
   - LED scheme configuration
   - **26 tests passing** + 1 ignored

4. **conductor** (root): Backward compatibility layer
   - Re-exports conductor-core types for existing tests
   - Maintains v0.1.0 import paths
   - Zero breaking changes for end users

**Completed Phases**:
- ✅ **Phase 2** (v0.2.0): ActionExecutor extraction to daemon layer
- ✅ **Phase 2.5** (v2.0.1): Security hardening & architectural purity
  - Architecture score: 9.5/10 (was 8.5)
  - Security score: 9.0/10 (was 7.5)
  - 6 critical/medium vulnerabilities fixed
  - Zero UI dependencies in core achieved
  - 485+ tests passing (100% pass rate)
- ✅ **Phase 3** (v1.0.0): Daemon infrastructure & config hot-reload
  - IPC server (<1ms latency)
  - Config hot-reload (0-10ms)
  - CLI tool (conductorctl)
  - State persistence
- ✅ **Phase 4** (v2.0.0): Tauri v2 GUI & visual configuration
  - MIDI Learn mode
  - Visual config editor
  - Per-app profiles
  - Device templates
  - Live event console
- ✅ **Phase 5** (v3.0.0): Game Controllers (HID) Support
  - Unified InputEvent abstraction
  - GamepadDeviceManager with auto-reconnection
  - InputManager (MIDI + HID unified)
  - Non-overlapping ID ranges (MIDI: 0-127, HID: 128-255)
  - gilrs v0.10 integration
  - 4 new trigger types (GamepadButton, GamepadButtonChord, GamepadAnalogStick, GamepadTrigger)
  - Official gamepad templates (Xbox, PlayStation, Switch Pro)
  - MIDI Learn support for gamepads
  - GUI gamepad template selector

**Next Phase**: Phase 6 - Advanced Features & Polish

### Architecture Overview

```
┌──────────────────────────────────────────────────┐
│  conductorctl (CLI)                                │
│  - status, reload, stop, validate, ping          │
└────────────┬─────────────────────────────────────┘
             │ IPC (JSON over Unix socket)
             ▼
┌──────────────────────────────────────────────────┐
│  conductor-daemon Service                          │
│  ┌────────────────────────────────────────────┐ │
│  │  IPC Server                                │ │
│  │  - Accept connections                      │ │
│  │  - Route commands                          │ │
│  └──────────┬─────────────────────────────────┘ │
│             ▼                                    │
│  ┌────────────────────────────────────────────┐ │
│  │  Engine Manager                            │ │
│  │  - Lifecycle management                    │ │
│  │  - Atomic config swaps (Arc<RwLock<>>)     │ │
│  │  - Performance metrics                     │ │
│  └──────────┬─────────────────────────────────┘ │
│             ▼                                    │
│  ┌────────────────────────────────────────────┐ │
│  │  Input Manager (v3.0 Unified)              │ │
│  │  ┌──────────────┐    ┌──────────────┐     │ │
│  │  │ MIDI Device  │    │ Gamepad      │     │ │
│  │  │ Manager      │    │ Device Mgr   │     │ │
│  │  └──────┬───────┘    └──────┬───────┘     │ │
│  │         │                   │              │ │
│  │         └─────────┬─────────┘              │ │
│  │                   ▼                        │ │
│  │         Unified InputEvent Stream          │ │
│  └────────────────────┬───────────────────────┘ │
│                       │                          │
│  ┌────────────────────▼───────────────────────┐ │
│  │  Config Watcher                            │ │
│  │  - File system monitoring                  │ │
│  │  - 500ms debounce                          │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │  State Manager                             │ │
│  │  - Atomic persistence                      │ │
│  │  - Emergency save handler                  │ │
│  └────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────────────┐
│  conductor-core Engine (UI-Independent)            │
│  - Event processing (MIDI + Gamepad)             │
│  - Mapping execution (protocol-agnostic)         │
│  - Action dispatch (data only)                   │
└──────────────────────────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────────────┐
│  Action Executor (in daemon)                     │
│  - Keyboard/mouse simulation                     │
│  - Shell command execution                       │
│  - System volume control                         │
└──────────────────────────────────────────────────┘
```

### Game Controllers (HID) Architecture (v3.0)

Conductor v3.0 introduces comprehensive support for all SDL2-compatible HID devices through a unified input abstraction:

#### Design Principles

1. **Protocol Agnostic**: Unified `InputEvent` abstraction for MIDI and Game Controllers (HID)
2. **Non-Overlapping IDs**: MIDI (0-127), Gamepad (128-255)
3. **Zero Latency Overhead**: Direct event translation, no additional processing
4. **Backwards Compatible**: All existing MIDI configs work unchanged
5. **Extensible**: Easy to add new input protocols (keyboard, mouse, etc.)

#### ID Range Allocation

- **MIDI Controllers**: IDs 0-127
  - Notes: 0-127
  - CC: 0-127
  - Program Change: 0-127
- **Game Controllers (HID)**: IDs 128-255
  - Buttons: 128-144 (17 buttons mapped)
  - Analog Sticks: 128-131 (4 axes: Left X/Y, Right X/Y)
  - Analog Triggers: 132-133 (2 axes: Left Z, Right Z)

#### InputMode Enum

Controls which input devices are active:

```rust
pub enum InputMode {
    MidiOnly,      // Use MIDI device only
    GamepadOnly,   // Use gamepad device only
    Both,          // Use both MIDI and gamepad simultaneously
}
```

#### Button Mapping (128-144)

```
128: South (A/Cross/B)     | 136: LeftTrigger (L1/LB/L)
129: East (B/Circle/A)     | 137: RightTrigger (R1/RB/R)
130: West (X/Square/Y)     | 138: LeftThumb (L3)
131: North (Y/Triangle/X)  | 139: RightThumb (R3)
132: DPadUp                | 140: Start
133: DPadDown              | 141: Select
134: DPadLeft              | 142: Mode (Guide/Home)
135: DPadRight             | 143: LeftTrigger2 (L2/LT/ZL)
                           | 144: RightTrigger2 (R2/RT/ZR)
```

#### Axis Mapping

**Analog Sticks (128-131)**:
- 128: LeftStickX
- 129: LeftStickY
- 130: RightStickX
- 131: RightStickY

**Analog Triggers (132-133)**:
- 132: LeftZ (Left trigger analog)
- 133: RightZ (Right trigger analog)

**Value Normalization**:
- Input: -1.0 to 1.0 (gilrs float)
- Output: 0 to 255 (u8)
- Formula: `((value + 1.0) * 127.5) as u8`
- Dead zone: 0.1 (10% threshold)

#### gilrs Library Integration

Conductor uses **gilrs v0.10** for cross-platform gamepad support:

- **Platform Support**: Windows, macOS, Linux, BSD
- **Device Types**: All SDL2-compatible controllers (gamepads, joysticks, racing wheels, flight sticks, HOTAS)
- **Polling Frequency**: 1000Hz (1ms intervals)
- **Latency**: <1ms event propagation
- **Auto-Reconnection**: Exponential backoff (1s → 30s)

### Performance Characteristics

- **Config Reload**: 0-8ms (production configs: <3ms)
- **IPC Round-Trip**: <1ms
- **Gamepad Polling**: 1ms (1000Hz)
- **Build Time**: 26s clean, 4s incremental
- **Binary Size**: 3-5MB (release)
- **Memory Usage**: 5-10MB resident
- **Test Suite**: 0.24s execution time

## Project Overview

Conductor is a Rust-based input device mapping system that transforms MIDI controllers and Game Controllers (HID) into advanced macro pads with velocity sensitivity, long press detection, double-tap, chord detection, and full RGB LED feedback.

**Supported Devices**:
- **MIDI Controllers**: Native Instruments Maschine Mikro MK3, Launchpad, KORG nanoKONTROL, etc.
- **Game Controllers (HID)**: Xbox controllers, PlayStation controllers, Nintendo Switch Pro, racing wheels, flight sticks, HOTAS, joysticks, and all SDL2-compatible devices

**Current Architecture**: Cargo workspace with 3 packages
- `conductor-core`: Pure engine library (UI-independent)
- `conductor-daemon`: CLI binaries and daemon
- `conductor`: Compatibility layer

## Build, Run & Development Commands

### Building
```bash
# Build entire workspace (all 3 packages)
cargo build --workspace

# Release build (optimized binaries)
cargo build --release --workspace

# Build specific package
cargo build --package conductor-core
cargo build --package conductor-daemon

# Build times (release mode, M1 Mac)
# - Clean build: ~26s
# - Incremental: <4s
```

### Running the Main Application

#### MIDI Device Mode
```bash
# List available MIDI ports
cargo run --release

# Connect to a specific port (e.g., port 2)
cargo run --release 2

# With LED lighting scheme
cargo run --release 2 --led reactive
cargo run --release 2 --led rainbow

# With device profile support
cargo run --release 2 --profile ~/Downloads/base-template-ni-mikro-mk3.ncmm3
cargo run --release 2 --profile mikro.ncmm3 --pad-page H

# Enable debug logging
DEBUG=1 cargo run --release 2
```

#### Game Controller (HID) Mode (v3.0)
```bash
# List available gamepads
cargo run --release -- --list-gamepads

# Connect to first available gamepad
cargo run --release -- --gamepad

# Connect to specific gamepad by name
cargo run --release -- --gamepad "Xbox Controller"

# Hybrid mode: MIDI + Gamepad simultaneously
cargo run --release 2 --gamepad

# Use gamepad template
cargo run --release -- --gamepad --template xbox-elite-series-2.toml

# Debug gamepad events
DEBUG=1 cargo run --release -- --gamepad
```

### Diagnostic Tools
```bash
# MIDI diagnostic tool (visualize all MIDI events)
cargo run --bin midi_diagnostic 2

# Gamepad diagnostic tool (v3.0 - visualize gamepad events)
cargo run --bin gamepad_diagnostic

# LED diagnostic tool
cargo run --bin led_diagnostic

# LED tester
cargo run --bin led_tester

# Pad mapper utility
cargo run --bin pad_mapper

# Test MIDI ports
cargo run --bin test_midi
```

### Testing
```bash
# Run all tests across workspace (500+ tests)
cargo test --workspace

# Run tests for specific package
cargo test --package conductor-core
cargo test --package conductor-daemon
cargo test --package conductor

# Run with verbose output
cargo test --workspace -- --nocapture

# Test execution time: ~29s (parallel across 3 packages)
```

## Architecture & Key Concepts

### Workspace Structure

```
conductor/                          # Root workspace
├── Cargo.toml                    # Workspace manifest
├── conductor-core/                 # Pure engine library
│   ├── src/
│   │   ├── lib.rs               # Public API exports
│   │   ├── config.rs            # Config structures & parsing
│   │   ├── events.rs            # Unified InputEvent (v3.0)
│   │   ├── gamepad_events.rs    # Gamepad event mapping (v3.0)
│   │   ├── event_processor.rs   # Event → ProcessedEvent
│   │   ├── mapping.rs           # Mapping engine
│   │   ├── actions.rs           # Action types & execution
│   │   ├── feedback.rs          # LED feedback traits
│   │   ├── device.rs            # Device profile support
│   │   ├── error.rs             # Error types (thiserror)
│   │   ├── mikro_leds.rs        # HID LED implementation
│   │   └── midi_feedback.rs     # MIDI LED fallback
│   └── tests/                   # Integration tests
├── conductor-daemon/              # CLI daemon + tools
│   ├── src/
│   │   ├── main.rs              # Main daemon binary
│   │   ├── input_manager.rs     # Unified input manager (v3.0)
│   │   ├── gamepad_device.rs    # Gamepad device manager (v3.0)
│   │   ├── midi_device.rs       # MIDI device manager
│   │   └── bin/                 # 6 diagnostic tools
│   └── tests/
└── src/                         # Compatibility layer
    └── lib.rs                   # Re-exports for old tests

```

### Event Processing Pipeline

The system follows a unified architecture supporting both MIDI and Game Controllers (HID):

1. **Input Collection** (conductor-daemon)
   - **MidiDeviceManager**: Raw MIDI bytes → `MidiEvent` → `InputEvent`
   - **GamepadDeviceManager**: gilrs events → `InputEvent`
   - **InputManager**: Unified stream management
2. **Event Processing** (conductor-core/src/event_processor.rs)
   - `InputEvent` → `ProcessedEvent`
   - Detects velocity levels, long press, double-tap, chords, encoder/stick direction
3. **Mapping & Execution** (conductor-core/src/mapping.rs, actions.rs)
   - `ProcessedEvent` → `Action` → execution
   - Protocol-agnostic: same mapping logic for MIDI and HID

### Core Components (conductor-core)

- **config.rs**: Configuration structures for TOML parsing (Trigger, Action, Mode)
- **events.rs**: Unified InputEvent abstraction (v3.0) - protocol-agnostic
- **gamepad_events.rs**: Gamepad event mapping (v3.0) - gilrs to InputEvent conversion
- **event_processor.rs**: Transforms InputEvents into ProcessedEvents with timing
- **mapping.rs**: Mapping engine (renamed from mappings.rs) matches events to actions
- **actions.rs**: Action execution using `enigo` for keyboard/mouse simulation
- **feedback.rs**: Unified LED feedback trait and device factory
- **device.rs**: Parser for NI Controller Editor profiles (.ncmm3 XML)
- **error.rs**: Structured error types (EngineError, ConfigError, ActionError, etc.)
- **mikro_leds.rs**: HID-based RGB LED control (Maschine Mikro MK3)
- **midi_feedback.rs**: Standard MIDI LED feedback fallback

### Core Components (conductor-daemon)

- **input_manager.rs**: Unified input device manager (v3.0) - MIDI + HID
- **gamepad_device.rs**: Gamepad device lifecycle management (v3.0)
- **midi_device.rs**: MIDI device lifecycle management
- **daemon/engine_manager.rs**: Event processing coordination
- **daemon/ipc_server.rs**: Unix socket IPC server
- **daemon/state_manager.rs**: Atomic state persistence

### Key Design Patterns

**Unified Input Abstraction (v3.0)**: The `InputEvent` enum abstracts MIDI and gamepad inputs into a common protocol-agnostic format, enabling unified mapping and action execution logic.

**Mode System**: Multiple modes (Default, Development, Media) allow different mapping sets. Mode changes are triggered by encoder rotation, specific pad/button combinations, or gamepad inputs.

**Global vs Mode Mappings**: Global mappings work across all modes (e.g., emergency exit, encoder volume control), while mode-specific mappings are scoped to their mode.

**Hybrid MIDI + Gamepad Workflow (v3.0)**: Use InputMode::Both to combine MIDI pads for velocity-sensitive triggers with gamepad analog sticks for directional control.

**Profile-Based Note Mapping**: Supports loading Native Instruments Controller Editor profiles (.ncmm3) to map physical pad positions to MIDI notes. Can auto-detect the active pad page or use a specified page (A-H).

**Velocity Levels**: Three velocity levels (Soft: 0-40, Medium: 41-80, Hard: 81-127) enable velocity-sensitive actions on both MIDI and gamepad triggers.

**LED Feedback**: Reactive LED system provides visual feedback for pad presses, mode changes, and actions. Supports multiple schemes (reactive, rainbow, pulse, breathing, static, etc.).

**HID Shared Device Access**: Uses `hidapi` with `macos-shared-device` feature to allow simultaneous access with Native Instruments Controller Editor.

### Trigger Types

#### MIDI Triggers
- **Note**: Basic note on/off with optional velocity range
- **VelocityRange**: Different actions for soft/medium/hard presses on same note
- **LongPress**: Hold detection with configurable duration (default 2000ms)
- **DoubleTap**: Quick double-tap detection (default 300ms window)
- **NoteChord**: Multiple notes pressed simultaneously (default 100ms window)
- **EncoderTurn**: Encoder rotation with direction (Clockwise/CounterClockwise)
- **Aftertouch**: Pressure sensitivity
- **PitchBend**: Touch strip control
- **CC**: Control change messages

#### Game Controller (HID) Triggers (v3.0)
- **GamepadButton**: Basic button press with velocity (IDs 128-144)
- **GamepadButtonChord**: Multiple buttons pressed simultaneously
- **GamepadAnalogStick**: Directional stick movement (IDs 128-131, normalized to 0-255)
- **GamepadTrigger**: Analog trigger pull (IDs 132-133, normalized to 0-255)

### Action Types

- **Keystroke**: Keyboard shortcuts with modifiers
- **Text**: Type text strings
- **Launch**: Open applications
- **Shell**: Execute shell commands
- **VolumeControl**: System volume (Up/Down/Mute/Set)
- **ModeChange**: Switch between mapping modes
- **Sequence**: Chain multiple actions
- **Delay**: Timing control
- **MouseClick**: Mouse simulation
- **Repeat**: Repeat an action N times
- **Conditional**: Conditional execution based on app state, time, etc.

## Configuration (config.toml)

All mappings are defined in TOML format. The config structure supports both MIDI and Game Controllers (HID):

```toml
[device]
name = "Mikro"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Default"
color = "blue"

# MIDI mapping example
[[modes.mappings]]
trigger = { type = "Note", note = 36, velocity_range = [80, 127] }
action = { type = "Keystroke", keys = ["cmd", "space"] }

# Gamepad mapping example (v3.0)
[[modes.mappings]]
trigger = { type = "GamepadButton", button = 128 }  # South button (A/Cross)
action = { type = "Keystroke", keys = ["cmd", "c"] }

# Gamepad analog stick example (v3.0)
[[modes.mappings]]
trigger = { type = "GamepadAnalogStick", axis = 128, direction = "Right", threshold = 200 }
action = { type = "VolumeControl", action = "Up" }

[[global_mappings]]
# Global mappings (work in all modes)...
```

**Important**: When adding new trigger or action types, you must update:
1. The enum in `config.rs`
2. The processing logic in `event_processor.rs` (for triggers)
3. The matching logic in `mapping.rs`
4. The execution logic in `actions.rs` (for actions)

## LED Feedback System

The LED system uses a trait-based abstraction (`PadFeedback`) to support:
- **HID devices** (Maschine Mikro MK3): Full RGB control via hidapi
- **MIDI devices**: Basic on/off via MIDI Note messages

Available lighting schemes: `off`, `static`, `breathing`, `pulse`, `rainbow`, `wave`, `sparkle`, `reactive`, `vumeter`, `spiral`

**Reactive Mode**: LEDs respond to velocity (green=soft, yellow=medium, red=hard) and fade out 1 second after release.

**Mode Colors**: Each mode has distinct color themes (Mode 0=Blue, Mode 1=Green, Mode 2=Purple).

See LED_FEEDBACK.md for complete documentation.

## Device Profile Support

The system can load Native Instruments Controller Editor profiles (.ncmm3 XML) to:
- Map physical pad positions to MIDI notes (handles different pad pages A-H)
- Auto-detect the active pad page from incoming MIDI events
- Support custom controller configurations

Profile files are parsed in `device_profile.rs` using `quick-xml`.

## Gamepad Template System (v3.0)

Official gamepad templates provide pre-configured mappings for common controllers:

- **xbox-elite-series-2.toml**: Xbox Elite Series 2 with paddle macros
- **playstation-dualsense.toml**: PlayStation 5 DualSense
- **nintendo-switch-pro.toml**: Nintendo Switch Pro Controller
- **generic-gamepad.toml**: Universal template for SDL2-compatible controllers

Templates are stored in `config/templates/gamepads/` and can be selected via GUI or CLI.

## Adding New Features

### Adding a New MIDI Trigger Type

1. Add variant to `Trigger` enum in `config.rs`
2. Add variant to `ProcessedEvent` enum in `event_processor.rs`
3. Add detection logic in `EventProcessor::process()` or `EventProcessor::process_input()` (event_processor.rs)
4. Add matching case in `MappingEngine::trigger_matches_processed()` (mapping.rs)

### Adding a New HID Trigger Type (v3.0)

1. Add variant to `Trigger` enum in `config.rs`
2. Add variant to `ProcessedEvent` enum in `event_processor.rs`
3. Add detection logic in `EventProcessor::process_input()` (event_processor.rs)
4. Add matching case in `MappingEngine::trigger_matches_processed()` (mapping.rs)
5. Update gamepad event mapping in `gamepad_events.rs` if needed
6. Add example to gamepad template files

### Adding a New Action Type

1. Add variant to `ActionConfig` enum in `config.rs`
2. Add variant to `Action` enum in `actions.rs`
3. Add execution logic in `ActionExecutor::execute()` (actions.rs)
4. Update `compile_action()` in `mapping.rs` to compile the new config

### Adding a New LED Scheme

1. Add variant to `LightingScheme` enum in `feedback.rs`
2. Implement in `MikroMK3LEDs::run_scheme()` (mikro_leds.rs)
3. Add fallback in `MidiFeedback::run_scheme()` (midi_feedback.rs)
4. Update `LightingScheme::from_str()` and `list_all()`
5. Update help text in `print_usage()` (main.rs)

## Platform-Specific Notes

### macOS
- HID access requires Input Monitoring permissions (System Settings → Privacy & Security)
- Native Instruments Controller Editor can run simultaneously (shared device mode)
- Uses `enigo` for keyboard/mouse simulation
- Volume control uses AppleScript via shell commands
- Gamepad support via gilrs with IOKit backend

### Linux
- May require udev rules for HID device access
- Consider using `xdotool` for input simulation
- Gamepad support via gilrs with evdev backend

### Windows
- USB driver installation may be required
- Check device permissions
- Gamepad support via gilrs with XInput/DInput backends

## Dependencies

Key external crates:
- **midir**: Cross-platform MIDI I/O
- **gilrs**: Cross-platform gamepad/HID input (v0.10) - v3.0
- **enigo**: Keyboard/mouse input simulation
- **hidapi**: HID device access (with `macos-shared-device` feature)
- **serde/toml**: Configuration parsing
- **quick-xml**: XML profile parsing
- **crossbeam-channel**: Lock-free event channels
- **colored**: Terminal output formatting
- **ctrlc**: Graceful shutdown handling
- **tokio**: Async runtime for daemon
- **tracing**: Structured logging

## Troubleshooting

### MIDI Device Not Found
```bash
# Check USB connection
system_profiler SPUSBDataType | grep -i mikro

# List MIDI ports
cargo run --bin test_midi

# Check Audio MIDI Setup
open -a "Audio MIDI Setup"
```

### Gamepad Not Found (v3.0)
```bash
# List connected gamepads
cargo run --release -- --list-gamepads

# Check USB connection (macOS)
system_profiler SPUSBDataType | grep -i controller

# Check gamepad driver (Linux)
ls /dev/input/js*

# Enable debug logging
DEBUG=1 cargo run --release -- --gamepad
```

### Gamepad Events Not Working (v3.0)
- Ensure gamepad is recognized by OS (test in System Settings / Game Controllers panel)
- Check that gamepad is SDL2-compatible (most modern controllers are)
- Verify ID ranges in config (buttons: 128-144, axes: 128-133)
- Run `cargo run --bin gamepad_diagnostic` to verify events
- Check DEBUG=1 output for polling errors
- Confirm no other application is exclusively using the gamepad

### LEDs Not Working
- Ensure Native Instruments drivers are installed
- Grant Input Monitoring permissions on macOS
- Try `--led reactive` or `--led rainbow` to test
- Check DEBUG=1 output for HID connection errors

### Events Not Triggering
- Run `cargo run --bin midi_diagnostic 2` to verify MIDI events
- Run `cargo run --bin gamepad_diagnostic` to verify gamepad events (v3.0)
- Check note/button numbers match config.toml
- Verify velocity/duration thresholds
- Check mode is correct (encoder to switch modes)
- For gamepads: ensure ID ranges are 128-255, not 0-127

### Profile Detection Issues
- Ensure .ncmm3 file is valid XML from Controller Editor
- Use `--pad-page` to force a specific page instead of auto-detect
- Check DEBUG=1 output for profile parsing errors

### Hybrid MIDI + Gamepad Issues (v3.0)
- Verify InputMode is set to `Both` in daemon configuration
- Check that both devices are connected and recognized
- Use `conductorctl status` to verify both device managers are active
- Ensure no ID conflicts (MIDI: 0-127, Gamepad: 128-255)

## Performance Characteristics

- Response latency: <1ms typical
- Gamepad polling: 1ms (1000Hz) - v3.0
- Memory usage: 5-10MB
- CPU usage: <1% idle, <5% active
- Binary size: ~3-5MB (release with LTO)

## Release Build Optimizations

The release profile in Cargo.toml uses:
- `opt-level = 3`: Maximum optimization
- `lto = true`: Link-time optimization
- `codegen-units = 1`: Single codegen unit for better optimization
- `strip = true`: Strip debug symbols for smaller binary

---

## Future Workspace Structure (Achieved in v3.0)

The target monorepo structure has been achieved:

```
conductor/
├── Cargo.toml                      # Workspace root
├── conductor-core/                   # Pure Rust engine crate (UI-free)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── events.rs               # Unified InputEvent abstraction ✅
│       ├── gamepad_events.rs       # Gamepad event mapping ✅ v3.0
│       ├── event_processor.rs      # Event normalization & detection
│       ├── mapping.rs              # Mapping engine
│       ├── actions.rs              # Action executor
│       └── config.rs               # Config loading & watching
├── conductor-daemon/                 # Background service (headless)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── input_manager.rs        # Unified input manager ✅ v3.0
│       ├── gamepad_device.rs       # Gamepad device manager ✅ v3.0
│       ├── midi_device.rs          # MIDI device manager ✅
│       └── daemon/                 # Daemon infrastructure ✅
├── conductor-gui/                    # Tauri v2 UI for configuration ✅
│   ├── Cargo.toml
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── ui/                         # Web UI (Svelte)
├── config/
│   ├── default.toml
│   └── templates/
│       ├── midi/                   # MIDI device templates
│       │   ├── maschine_mikro_mk3.toml
│       │   ├── launchpad_mini.toml
│       │   └── korg_nanokontrol.toml
│       └── gamepads/               # Gamepad templates ✅ v3.0
│           ├── xbox-elite-series-2.toml
│           ├── playstation-dualsense.toml
│           └── nintendo-switch-pro.toml
└── docs/                           # Comprehensive documentation ✅
    ├── v3.0-gamepad-technical-reference.md
    ├── v3.0-gamepad-engine-integration.md
    └── guides/
        └── gamepad-support.md
```

### Key Architectural Principles (Achieved)

1. ✅ **Engine Independence**: `conductor-core` has zero UI dependencies - pure event processing, mapping, and actions
2. ✅ **Plugin Architecture**: Device profiles as external TOML templates, easily shareable
3. ✅ **Hot Reload**: Config file watching with `notify` crate for zero-downtime updates
4. ✅ **Menu Bar UX**: Tauri tray icon with quick actions (Pause, Reload, Open Config)
5. ✅ **Auto-Start**: Tauri autostart plugin for macOS LaunchAgent integration
6. ✅ **Unified Events**: Normalize MIDI/HID into common `InputEvent` type for consistent handling (v3.0)
7. ✅ **State Machine**: Per-element timers for short/long press, double-tap, chord detection
8. ✅ **Profile Switching**: Frontmost app detection for context-aware mappings

### Migration Status

**Phase 1: Preserve & Document** ✅ COMPLETE
- ✅ Tagged v0.1.0-monolithic
- ✅ Documented baseline implementation
- ✅ All 314 tests passing

**Phase 2: Extract Core Engine** ✅ COMPLETE (v0.2.0)
- ✅ Created `conductor-core` crate (pure Rust library, zero UI dependencies)
- ✅ Created `conductor-daemon` with 7 binaries
- ✅ Created backward compatibility layer
- ✅ 339 tests passing (100% pass rate)
- ✅ Zero breaking changes
- ✅ All 26 features validated
- ✅ Performance improved (12s clean build, was 15-20s)

**Phase 3: Add Daemon & UI** ✅ COMPLETE (v1.0.0-v2.0.0)
- ✅ Created Tauri-based menu bar UI
- ✅ Added Tauri-based `conductor-gui` for visual configuration
- ✅ Implemented config hot-reloading
- ✅ Added frontmost app detection for per-app profiles

**Phase 4: Enhanced Features** ✅ COMPLETE (v2.0.0)
- ✅ MIDI Learn mode (click binding → press device element → auto-fill)
- ✅ Virtual MIDI output for DAW integration
- ✅ Profile sharing/export
- ✅ Live event console for debugging
- ✅ Velocity curves and advanced mapping conditions

**Phase 5: Game Controllers (HID) Support** ✅ COMPLETE (v3.0.0)
- ✅ Unified InputEvent abstraction
- ✅ GamepadDeviceManager with auto-reconnection
- ✅ InputManager (MIDI + HID unified)
- ✅ Non-overlapping ID ranges (MIDI: 0-127, HID: 128-255)
- ✅ gilrs v0.10 integration
- ✅ 4 new trigger types (GamepadButton, GamepadButtonChord, GamepadAnalogStick, GamepadTrigger)
- ✅ Official gamepad templates (Xbox, PlayStation, Switch Pro)
- ✅ MIDI Learn support for gamepads
- ✅ GUI gamepad template selector
- ✅ 500+ tests passing (100% pass rate)

### References

See `docs/v3.0-gamepad-technical-reference.md` for detailed gamepad architecture, `docs/v3.0-gamepad-engine-integration.md` for integration details, and `docs/guides/gamepad-support.md` for user-facing documentation.
