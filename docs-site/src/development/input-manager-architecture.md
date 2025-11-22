# InputManager Architecture

**Version**: 3.0
**Status**: Stable
**Module**: `conductor-daemon/src/input_manager.rs`

## Overview

The **InputManager** is Conductor's unified input handling system introduced in v3.0. It provides a single, cohesive interface for managing both MIDI and HID (game controller) input devices, producing a unified stream of protocol-agnostic `InputEvent` instances for processing by the mapping engine.

### Key Features

- **Multi-Protocol Support**: Seamlessly integrates MIDI and HID game controller inputs
- **Unified Event Stream**: Single `InputEvent` channel for all input types
- **Flexible Device Selection**: Choose MIDI-only, gamepad-only, or hybrid (both) modes
- **ID Range Separation**: Non-overlapping ID ranges prevent conflicts (MIDI: 0-127, HID: 128-255)
- **Automatic Reconnection**: Inherits robust reconnection logic from device managers
- **Thread Safety**: Arc/Mutex patterns for safe concurrent access

## Architecture Diagram

```
┌────────────────────────────────────────────────────────────────────┐
│  InputManager (Unified Input Layer)                               │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  InputMode Selection                                         │ │
│  │  - MidiOnly: MIDI device only                                │ │
│  │  - GamepadOnly: Game controller only                         │ │
│  │  - Both: MIDI + Gamepad simultaneously (hybrid mode)         │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                                                                    │
│  ┌───────────────────────┐        ┌──────────────────────────┐   │
│  │  MidiDeviceManager    │        │  GamepadDeviceManager    │   │
│  │  (midir)              │        │  (gilrs v0.10)           │   │
│  │                       │        │                          │   │
│  │  - MIDI I/O           │        │  - HID event polling     │   │
│  │  - Port management    │        │  - SDL2 mappings         │   │
│  │  - Auto-reconnect     │        │  - Device enumeration    │   │
│  └──────────┬────────────┘        └────────────┬─────────────┘   │
│             │                                   │                 │
│             │ MidiEvent                         │ gilrs::Event    │
│             │                                   │                 │
│             ▼                                   ▼                 │
│  ┌──────────────────────┐          ┌──────────────────────────┐  │
│  │  MIDI → InputEvent   │          │  HID → InputEvent        │  │
│  │  Converter           │          │  Converter               │  │
│  │  (convert_midi)      │          │  (gamepad_events)        │  │
│  └──────────┬───────────┘          └────────────┬─────────────┘  │
│             │                                   │                 │
│             └───────────────┬───────────────────┘                 │
│                             ▼                                     │
│                  ┌───────────────────────┐                        │
│                  │  Unified InputEvent   │                        │
│                  │  Stream (mpsc)        │                        │
│                  └───────────┬───────────┘                        │
└──────────────────────────────┼────────────────────────────────────┘
                               │
                               ▼
                    ┌────────────────────────┐
                    │  EventProcessor        │
                    │  (conductor-core)        │
                    │                        │
                    │  - Velocity detection  │
                    │  - Long press          │
                    │  - Double-tap          │
                    │  - Chord detection     │
                    └────────┬───────────────┘
                             │
                             ▼
                    ┌────────────────────────┐
                    │  MappingEngine         │
                    │  (conductor-core)        │
                    │                        │
                    │  - Trigger matching    │
                    │  - Action execution    │
                    └────────────────────────┘
```

## InputMode Enum

The `InputMode` enum controls which input devices are active:

```rust
pub enum InputMode {
    /// Use MIDI device only
    MidiOnly,

    /// Use gamepad device only
    GamepadOnly,

    /// Use both MIDI and gamepad simultaneously (hybrid mode)
    Both,
}
```

### Mode Selection Strategy

| Mode | MIDI Manager | Gamepad Manager | Use Case |
|------|-------------|-----------------|----------|
| `MidiOnly` | ✅ Active | ❌ Disabled | Traditional MIDI controller workflows |
| `GamepadOnly` | ❌ Disabled | ✅ Active | Game controller macro setups |
| `Both` | ✅ Active | ✅ Active | Hybrid workflows (MIDI + gamepad) |

## ID Range Separation

To prevent conflicts between MIDI and HID inputs, Conductor uses non-overlapping ID ranges:

### MIDI ID Range (0-127)

MIDI protocol uses 7-bit addressing for notes and control changes:

- **Notes**: 0-127 (C-2 to G8)
- **Control Changes**: 0-127 (CC0 to CC127)
- **Velocity**: 0-127 (off to maximum)

**Example**: MIDI note 60 (Middle C) → `InputEvent::PadPressed { pad: 60, ... }`

### HID ID Range (128-255)

Game controller buttons and axes use IDs starting at 128:

#### Button IDs (128-144)

```
Face Buttons:
  128 = South (A/Cross/B)
  129 = East (B/Circle/A)
  130 = West (X/Square/Y)
  131 = North (Y/Triangle/X)

D-Pad:
  132 = Up
  133 = Down
  134 = Left
  135 = Right

Shoulder Buttons:
  136 = Left Shoulder (L1/LB)
  137 = Right Shoulder (R1/RB)

Stick Clicks:
  138 = Left Thumb (L3)
  139 = Right Thumb (R3)

Menu Buttons:
  140 = Start (Options/+)
  141 = Select (Share/-)
  142 = Guide (Xbox/PS/Home)

Trigger Digital:
  143 = Left Trigger (L2/LT)
  144 = Right Trigger (R2/RT)
```

#### Encoder IDs (128-133)

Analog stick axes and triggers use encoder IDs:

```
Analog Sticks:
  128 = Left Stick X
  129 = Left Stick Y
  130 = Right Stick X
  131 = Right Stick Y

Trigger Analog:
  132 = Left Trigger (L2/LT)
  133 = Right Trigger (R2/RT)
```

### Why Non-Overlapping Ranges?

1. **Conflict Prevention**: MIDI note 60 and gamepad button never collide
2. **Unified Processing**: EventProcessor handles both identically
3. **Simple Disambiguation**: Check ID range to determine source protocol
4. **Future Expansion**: Room for additional input types (256-65535)

## Device Management

### MidiDeviceManager

**Location**: `conductor-daemon/src/midi_device.rs`

Responsibilities:
- Connect to MIDI input ports via `midir`
- Emit `MidiEvent` instances (NoteOn, NoteOff, ControlChange, etc.)
- Handle MIDI device disconnections and reconnections
- Enumerate available MIDI ports

**Event Flow**:
```
MIDI Device → midir callback → MidiEvent → mpsc channel
```

### GamepadDeviceManager

**Location**: `conductor-daemon/src/gamepad_device.rs`

Responsibilities:
- Poll HID game controllers via `gilrs` (v0.10)
- Use SDL2-compatible controller mappings
- Emit gilrs events (ButtonPressed, AxisChanged, etc.)
- Handle gamepad disconnections and reconnections
- Enumerate connected gamepads

**Event Flow**:
```
Gamepad → gilrs::Gilrs::next_event() → gilrs::Event → gamepad_events → InputEvent → mpsc channel
```

### gilrs Integration

Conductor v3.0 uses **gilrs v0.10** for HID game controller support:

- **SDL2 Compatibility**: Supports SDL_GameController mapping database
- **Cross-Platform**: Works on macOS, Linux, Windows
- **Controller Support**: Xbox, PlayStation, Nintendo Switch Pro, generic gamepads
- **Polling Architecture**: 1ms polling interval for low latency
- **Event Types**: ButtonPressed, ButtonReleased, AxisChanged, Connected, Disconnected

## Event Normalization

### MIDI → InputEvent Conversion

The `convert_midi_to_input()` function maps MIDI protocol events to `InputEvent`:

```rust
fn convert_midi_to_input(midi_event: MidiEvent) -> InputEvent {
    match midi_event {
        MidiEvent::NoteOn { note, velocity, .. } =>
            InputEvent::PadPressed { pad: note, velocity, time: now },

        MidiEvent::NoteOff { note, .. } =>
            InputEvent::PadReleased { pad: note, time: now },

        MidiEvent::ControlChange { cc, value, .. } =>
            InputEvent::EncoderTurned { encoder: cc, value, time: now },

        MidiEvent::Aftertouch { pressure, .. } =>
            InputEvent::Aftertouch { pressure, time: now },

        MidiEvent::PitchBend { value, .. } =>
            InputEvent::PitchBend { value, time: now },

        // ... other mappings
    }
}
```

**Key Insight**: This conversion happens in a spawned tokio task, allowing the MIDI device manager to remain protocol-agnostic while the InputManager handles unification.

### HID → InputEvent Conversion

The `gamepad_events` module provides three converter functions:

```rust
// Button press: gilrs::Event → InputEvent::PadPressed
pub fn button_pressed_to_input(
    button: gilrs::Button,
    gamepad_id: gilrs::GamepadId
) -> InputEvent {
    InputEvent::PadPressed {
        pad: button_to_id(button), // Maps to 128-144 range
        velocity: 100, // Default velocity for digital buttons
        time: Instant::now(),
    }
}

// Button release: gilrs::Event → InputEvent::PadReleased
pub fn button_released_to_input(
    button: gilrs::Button,
    gamepad_id: gilrs::GamepadId
) -> InputEvent {
    InputEvent::PadReleased {
        pad: button_to_id(button),
        time: Instant::now(),
    }
}

// Analog axis: gilrs::Event → InputEvent::EncoderTurned
pub fn axis_changed_to_input(
    axis: gilrs::Axis,
    value: f32, // -1.0 to 1.0
    gamepad_id: gilrs::GamepadId
) -> InputEvent {
    InputEvent::EncoderTurned {
        encoder: axis_to_encoder_id(axis), // Maps to 128-133 range
        value: normalize_axis_value(value), // Convert to 0-127
        time: Instant::now(),
    }
}
```

## Key APIs

### Creating an InputManager

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};

// MIDI-only mode
let manager = InputManager::new(
    Some("Maschine Mikro MK3".to_string()),
    true, // auto_reconnect
    InputMode::MidiOnly
);

// Gamepad-only mode
let gamepad_manager = InputManager::new(
    None,
    true,
    InputMode::GamepadOnly
);

// Hybrid mode (both MIDI and gamepad)
let hybrid_manager = InputManager::new(
    Some("Maschine Mikro MK3".to_string()),
    true,
    InputMode::Both
);
```

### Connecting to Devices

```rust
use tokio::sync::mpsc;
use conductor_core::events::InputEvent;
use conductor_daemon::daemon::DaemonCommand;

let (event_tx, mut event_rx) = mpsc::channel::<InputEvent>(1024);
let (command_tx, mut command_rx) = mpsc::channel::<DaemonCommand>(32);

// Connect returns status message or error
match manager.connect(event_tx, command_tx) {
    Ok(status) => println!("Connected: {}", status),
    Err(e) => eprintln!("Connection failed: {}", e),
}

// Example output:
// "MIDI: Maschine Mikro MK3 (port 2) | Gamepad: Xbox 360 Controller (ID GamepadId(0))"
```

### Polling Events

```rust
// Process unified event stream
while let Some(event) = event_rx.recv().await {
    match event {
        InputEvent::PadPressed { pad, velocity, .. } => {
            if pad < 128 {
                println!("MIDI pad {} pressed (vel: {})", pad, velocity);
            } else {
                println!("Gamepad button {} pressed", pad);
            }
        }
        InputEvent::EncoderTurned { encoder, value, .. } => {
            if encoder < 128 {
                println!("MIDI CC {} = {}", encoder, value);
            } else {
                println!("Gamepad axis {} = {}", encoder, value);
            }
        }
        _ => {}
    }
}
```

### Enumerating Gamepads

```rust
// List all connected gamepads
match InputManager::list_gamepads() {
    Ok(gamepads) => {
        for (id, name, uuid) in gamepads {
            println!("Gamepad: {:?} - {} (UUID: {})", id, name, uuid);
        }
    }
    Err(e) => eprintln!("Failed to list gamepads: {}", e),
}

// Get gamepads managed by this InputManager
let connected = manager.get_connected_gamepads();
for (id, name) in connected {
    println!("Active: {} ({})", name, id);
}
```

### Checking Connection Status

```rust
// Check if any device is connected
if manager.is_connected() {
    println!("At least one device connected");
}

// Get detailed status
let (midi_connected, gamepad_connected) = manager.get_status();
println!("MIDI: {}, Gamepad: {}", midi_connected, gamepad_connected);
```

### Disconnecting

```rust
// Graceful shutdown of all devices
manager.disconnect();
```

## Hybrid Mode Architecture

When using `InputMode::Both`, the InputManager creates both MIDI and gamepad device managers and merges their event streams:

```
┌─────────────────────────────────────────────┐
│  InputManager::connect()                    │
│                                             │
│  1. Connect MIDI device                     │
│     - Create midi_event_rx channel          │
│     - Spawn converter task:                 │
│       while let Some(midi_evt) = rx.recv() {│
│         send(convert_midi_to_input(midi))   │
│       }                                     │
│                                             │
│  2. Connect Gamepad device                  │
│     - Directly sends InputEvent             │
│     - No conversion needed                  │
│                                             │
│  3. Both tasks send to same event_tx        │
│     - Unified mpsc::Sender<InputEvent>      │
│     - EventProcessor receives single stream │
└─────────────────────────────────────────────┘
```

### Hybrid Mode Event Flow Example

```rust
// Time T0: MIDI note 60 pressed
InputEvent::PadPressed { pad: 60, velocity: 100, time: T0 }

// Time T1: Gamepad A button pressed (ID 128)
InputEvent::PadPressed { pad: 128, velocity: 100, time: T1 }

// Time T2: MIDI CC 7 changed
InputEvent::EncoderTurned { encoder: 7, value: 64, time: T2 }

// Time T3: Gamepad left stick X moved
InputEvent::EncoderTurned { encoder: 128, value: 90, time: T3 }
```

All events flow through the same channel, preserving temporal ordering and enabling hybrid workflows like:
- MIDI pads for velocity-sensitive drumming
- Gamepad sticks for smooth parameter sweeps
- Gamepad buttons for mode switching
- MIDI encoder for fine-grained control

## Thread Safety

The InputManager and its device managers use Rust's ownership system and Arc/Mutex patterns for safe concurrent access:

```rust
pub struct InputManager {
    midi_manager: Option<MidiDeviceManager>,
    gamepad_manager: Option<GamepadDeviceManager>,
    mode: InputMode,
}

pub struct GamepadDeviceManager {
    gamepad_id: Arc<Mutex<Option<gilrs::GamepadId>>>,
    gamepad_name: Arc<Mutex<Option<String>>>,
    is_connected: Arc<AtomicBool>,
    stop_polling: Arc<AtomicBool>,
    polling_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}
```

### Synchronization Mechanisms

- **Arc<Mutex<T>>**: Shared mutable state (gamepad ID, name, thread handles)
- **Arc<AtomicBool>**: Lock-free connection status and stop flags
- **mpsc channels**: Lock-free event passing between threads
- **tokio::spawn**: Async task for MIDI conversion
- **std::thread**: Blocking thread for gamepad polling (gilrs is synchronous)

## Error Handling

The InputManager provides graceful degradation in hybrid mode:

```rust
// In InputMode::Both:
match midi_mgr.connect(...) {
    Ok(_) => { /* MIDI connected */ },
    Err(e) => {
        warn!("Failed to connect MIDI (continuing with gamepad): {}", e);
        // Gamepad connection attempt continues
    }
}

match gamepad_mgr.connect(...) {
    Ok(_) => { /* Gamepad connected */ },
    Err(e) => {
        warn!("Failed to connect gamepad (continuing with MIDI): {}", e);
        // Return OK if MIDI connected
    }
}

// Only fail if BOTH connections failed
if status_messages.is_empty() {
    return Err("No input devices could be connected".to_string());
}
```

## Performance Characteristics

| Metric | MIDI | Gamepad | Hybrid |
|--------|------|---------|--------|
| Event Latency | <1ms | <2ms | <2ms |
| Polling Rate | Callback-driven | 1ms (1000Hz) | Both |
| CPU Usage (idle) | <0.1% | <0.5% | <0.6% |
| Memory Overhead | ~200KB | ~1MB | ~1.2MB |
| Thread Count | 1 (callback) | 1 (polling) | 2 |

## Code Examples

### Example 1: MidiOnly Mode

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut manager = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::MidiOnly
    );

    manager.connect(event_tx, command_tx)?;

    while let Some(event) = event_rx.recv().await {
        println!("MIDI Event: {:?}", event);
    }

    Ok(())
}
```

### Example 2: GamepadOnly Mode

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut manager = InputManager::new(
        None, // No MIDI device
        true,
        InputMode::GamepadOnly
    );

    manager.connect(event_tx, command_tx)?;

    while let Some(event) = event_rx.recv().await {
        println!("Gamepad Event: {:?}", event);
    }

    Ok(())
}
```

### Example 3: Hybrid Mode (Both)

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};
use conductor_core::events::InputEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut manager = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::Both
    );

    let status = manager.connect(event_tx, command_tx)?;
    println!("Connected: {}", status);

    while let Some(event) = event_rx.recv().await {
        match event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                if pad < 128 {
                    println!("MIDI pad {} pressed (velocity: {})", pad, velocity);
                } else {
                    println!("Gamepad button {} pressed", pad - 128);
                }
            }
            InputEvent::EncoderTurned { encoder, value, .. } => {
                if encoder < 128 {
                    println!("MIDI CC {} changed to {}", encoder, value);
                } else {
                    println!("Gamepad axis {} = {}", encoder - 128, value);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

### Example 4: Device Enumeration

```rust
use conductor_daemon::input_manager::InputManager;

fn main() -> Result<(), String> {
    // List all available gamepads
    println!("Available gamepads:");
    let gamepads = InputManager::list_gamepads()?;

    for (id, name, uuid) in gamepads {
        println!("  - {:?}: {} (UUID: {})", id, name, uuid);
    }

    Ok(())
}
```

## Integration with EventProcessor

The InputManager produces `InputEvent` instances that flow directly into the `EventProcessor`:

```rust
// conductor-core/src/event_processor.rs
impl EventProcessor {
    pub fn process(&mut self, event: InputEvent) -> Vec<ProcessedEvent> {
        match event {
            InputEvent::PadPressed { pad, velocity, time } => {
                // Detect velocity levels, long press, double-tap, chords
                self.process_pad_press(pad, velocity, time)
            }
            InputEvent::EncoderTurned { encoder, value, time } => {
                // Detect encoder direction, acceleration
                self.process_encoder(encoder, value, time)
            }
            // ... other event types
        }
    }
}
```

The EventProcessor doesn't care if the event came from MIDI or a gamepad—it processes all `InputEvent` instances identically using the ID range to determine device type when needed.

## Testing

The InputManager includes comprehensive unit tests:

```bash
# Run InputManager tests
cargo test -p conductor-daemon input_manager

# Test specific mode creation
cargo test -p conductor-daemon test_input_manager_creation_midi_only
cargo test -p conductor-daemon test_input_manager_creation_gamepad_only
cargo test -p conductor-daemon test_input_manager_creation_both

# Test MIDI → InputEvent conversion
cargo test -p conductor-daemon test_convert_midi_note_on
cargo test -p conductor-daemon test_convert_midi_cc
```

## Future Enhancements

Potential future improvements to the InputManager:

1. **Multiple Gamepad Support**: Connect multiple gamepads simultaneously
2. **Device Prioritization**: Configurable priority when events collide
3. **Custom ID Ranges**: Allow users to remap ID ranges via config
4. **Hot-Swapping**: Dynamic device addition/removal without restart
5. **Input Filtering**: Filter specific buttons/axes before EventProcessor
6. **Virtual Devices**: Create virtual MIDI/gamepad devices for testing

## Related Documentation

- [Gamepad Support Guide](../guides/gamepad-support.md) - User-facing gamepad setup
- [Architecture Overview](architecture.md) - Overall system architecture
- [Event Processing Pipeline](architecture.md#event-processing-pipeline) - Event flow details
- [Device Templates Guide](../guides/device-templates.md) - Pre-configured templates
- [Configuration Reference](../configuration/overview.md) - Config file syntax

## Terminology

**Game Controllers (HID)**: The standard term used throughout Conductor documentation for HID input devices. This includes:

- **Gamepads**: Xbox, PlayStation, Nintendo Switch Pro controllers (primary examples)
- **Joysticks**: Flight sticks, arcade sticks
- **Racing Wheels**: Steering wheel controllers
- **HOTAS**: Hands-On Throttle-And-Stick systems
- **Custom Controllers**: DIY Arduino-based controllers, specialized input devices

All SDL2-compatible HID game controllers are supported via the gilrs library.

## Summary

The InputManager is a critical architectural component that:

1. **Unifies** MIDI and HID inputs into a single event stream
2. **Separates** ID ranges to prevent conflicts (0-127 vs 128-255)
3. **Abstracts** protocol differences behind `InputEvent`
4. **Enables** hybrid workflows with both MIDI and gamepad devices
5. **Provides** flexible device selection via `InputMode`

This design allows Conductor to support a wide range of input devices while maintaining a clean, protocol-agnostic processing pipeline.
