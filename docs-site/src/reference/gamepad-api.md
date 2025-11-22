# Game Controllers (HID) - Rust API Documentation

This document provides comprehensive API documentation for Conductor v3.0's Game Controller (HID) support. These types enable integration of gamepads, joysticks, racing wheels, flight sticks, HOTAS setups, and other HID-compliant game controllers.

## Overview

Conductor v3.0 introduces a unified input system that supports both MIDI controllers and Game Controllers (HID) simultaneously. The architecture uses protocol-agnostic abstractions to enable hybrid setups where MIDI and gamepad inputs coexist without ID conflicts.

**Key Design Principles:**
- **Non-overlapping ID ranges**: Gamepad buttons use IDs 128-255, MIDI uses 0-127
- **Unified event stream**: Both protocols convert to `InputEvent` for consistent processing
- **Flexible device selection**: Support MIDI-only, gamepad-only, or hybrid (both) modes
- **Automatic reconnection**: Background monitoring with exponential backoff
- **Thread-safe**: Arc/Mutex patterns for concurrent access

## Architecture Diagram

```text
┌────────────────────────────────────────────────────────────────┐
│  InputManager (input_manager.rs)                               │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │  InputMode Selection                                     │ │
│  │  - MidiOnly / GamepadOnly / Both                         │ │
│  └──────────────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │  MidiDeviceManager        GamepadDeviceManager           │ │
│  │  - MIDI events (0-127)    - Gamepad events (128-255)     │ │
│  │  - Convert to InputEvent  - Native InputEvent            │ │
│  └──────────────────────────────────────────────────────────┘ │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │  Unified InputEvent Stream                               │ │
│  │  - Single mpsc channel for all inputs                    │ │
│  │  - Processed by EventProcessor                           │ │
│  │  - Dispatched to MappingEngine                           │ │
│  └──────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────┘
```

## Core Types

### InputMode

**Location**: `conductor-daemon/src/input_manager.rs`

Enum representing the device selection mode for the unified input system.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Use MIDI device only
    MidiOnly,
    /// Use gamepad device only
    GamepadOnly,
    /// Use both MIDI and gamepad simultaneously
    Both,
}
```

#### Variants

| Variant | Description | Use Case |
|---------|-------------|----------|
| `MidiOnly` | Connect to MIDI devices only | Traditional MIDI controller workflows |
| `GamepadOnly` | Connect to gamepad devices only | Pure gamepad macro pad setup |
| `Both` | Connect to both MIDI and gamepad | Hybrid setups (e.g., MIDI pads + gamepad stick navigation) |

#### Examples

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};

// MIDI-only setup (traditional)
let midi_manager = InputManager::new(
    Some("Maschine Mikro MK3".to_string()),
    true,  // auto_reconnect
    InputMode::MidiOnly
);

// Gamepad-only setup
let gamepad_manager = InputManager::new(
    None,
    true,
    InputMode::GamepadOnly
);

// Hybrid setup (both MIDI and gamepad)
let hybrid_manager = InputManager::new(
    Some("Maschine Mikro MK3".to_string()),
    true,
    InputMode::Both
);
```

---

### GamepadDeviceManager

**Location**: `conductor-daemon/src/gamepad_device.rs`

Manages the lifecycle of gamepad/HID device connections with automatic reconnection support and robust error handling.

#### Fields

```rust
pub struct GamepadDeviceManager {
    /// Whether to automatically reconnect on disconnect
    auto_reconnect: bool,

    /// Currently connected gamepad ID (Arc<Mutex<Option<gilrs::GamepadId>>>)
    gamepad_id: Arc<Mutex<Option<gilrs::GamepadId>>>,

    /// Currently connected gamepad name (Arc<Mutex<Option<String>>>)
    gamepad_name: Arc<Mutex<Option<String>>>,

    /// Whether currently connected (Arc<AtomicBool>)
    is_connected: Arc<AtomicBool>,

    /// Flag to signal polling thread to stop (Arc<AtomicBool>)
    stop_polling: Arc<AtomicBool>,

    /// Handle to polling thread (Arc<Mutex<Option<thread::JoinHandle<()>>>>)
    polling_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}
```

#### Constructor

```rust
pub fn new(auto_reconnect: bool) -> Self
```

Creates a new gamepad device manager.

**Parameters:**
- `auto_reconnect` - Whether to automatically reconnect on disconnect

**Returns:**
- A new `GamepadDeviceManager` instance (not yet connected)

**Example:**

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;

let manager = GamepadDeviceManager::new(true);
assert!(!manager.is_connected());
```

#### Methods

##### `connect()`

```rust
pub fn connect(
    &mut self,
    event_tx: mpsc::Sender<InputEvent>,
    command_tx: mpsc::Sender<DaemonCommand>,
) -> Result<(gilrs::GamepadId, String), String>
```

Connects to the first available gamepad and starts the polling loop.

**Parameters:**
- `event_tx` - Channel sender for `InputEvent` messages
- `command_tx` - Channel sender for `DaemonCommand` messages (reconnection, etc.)

**Returns:**
- `Ok((GamepadId, Name))` - Tuple of gamepad ID and device name
- `Err(String)` - Error message if connection fails

**Errors:**
- gilrs initialization fails
- No gamepads are connected
- Already connected to a gamepad

**Example:**

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;
use tokio::sync::mpsc;

async fn connect_gamepad() -> Result<(), String> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut manager = GamepadDeviceManager::new(true);
    let (id, name) = manager.connect(event_tx, command_tx)?;

    println!("Connected to: {} (ID {:?})", name, id);

    // Process events
    while let Some(event) = event_rx.recv().await {
        println!("Received: {:?}", event);
    }

    Ok(())
}
```

##### `disconnect()`

```rust
pub fn disconnect(&mut self)
```

Disconnects from the current gamepad and stops the polling thread.

**Example:**

```rust
manager.disconnect();
assert!(!manager.is_connected());
```

##### `is_connected()`

```rust
pub fn is_connected(&self) -> bool
```

Returns whether the manager is currently connected to a gamepad.

**Example:**

```rust
if manager.is_connected() {
    println!("Gamepad is connected");
}
```

##### `get_gamepad_name()`

```rust
pub fn get_gamepad_name(&self) -> Option<String>
```

Returns the name of the currently connected gamepad, or `None` if not connected.

**Example:**

```rust
if let Some(name) = manager.get_gamepad_name() {
    println!("Connected to: {}", name);
}
```

##### `list_gamepads()` (static)

```rust
pub fn list_gamepads() -> Result<Vec<(gilrs::GamepadId, String, String)>, String>
```

Lists all connected gamepads. Returns a vector of `(GamepadId, Name, UUID)` tuples.

**Returns:**
- `Ok(Vec)` - List of connected gamepads
- `Err(String)` - Error if gilrs initialization fails

**Example:**

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;

fn show_gamepads() -> Result<(), String> {
    let gamepads = GamepadDeviceManager::list_gamepads()?;
    for (id, name, uuid) in gamepads {
        println!("Gamepad: {} (ID: {:?}, UUID: {})", name, id, uuid);
    }
    Ok(())
}
```

#### Thread Safety

The `GamepadDeviceManager` uses:
- `Arc<Mutex<>>` for shared state (gamepad ID, name, thread handle)
- `Arc<AtomicBool>` for lock-free flags (is_connected, stop_polling)
- Safe to share across threads

#### Polling Loop

The manager spawns a background thread that:
1. Polls for gamepad events at 1ms intervals
2. Converts gilrs events to `InputEvent`
3. Sends events through the provided channel
4. Detects disconnection and triggers reconnection if enabled

#### Reconnection Logic

When a gamepad disconnects and `auto_reconnect` is enabled:
1. Spawns a reconnection thread
2. Uses exponential backoff: 1s, 2s, 4s, 8s, 16s, 30s
3. Checks for available gamepads at each interval
4. Sends `DaemonCommand::ReconnectGamepad` when a device is found
5. Gives up after 6 attempts

---

### InputManager

**Location**: `conductor-daemon/src/input_manager.rs`

Unified manager for both MIDI and gamepad input devices. Provides a single `InputEvent` stream for all inputs.

#### Fields

```rust
pub struct InputManager {
    /// MIDI device manager (optional)
    midi_manager: Option<MidiDeviceManager>,

    /// Gamepad device manager (optional)
    gamepad_manager: Option<GamepadDeviceManager>,

    /// Input mode selection
    mode: InputMode,
}
```

#### Constructor

```rust
pub fn new(
    midi_device_name: Option<String>,
    auto_reconnect: bool,
    mode: InputMode,
) -> Self
```

Creates a new unified input manager.

**Parameters:**
- `midi_device_name` - Name of MIDI device to connect to (`None` = first available)
- `auto_reconnect` - Enable automatic reconnection for both MIDI and gamepad
- `mode` - Input mode selection (`MidiOnly`, `GamepadOnly`, or `Both`)

**Example:**

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};

// MIDI + Gamepad hybrid setup
let manager = InputManager::new(
    Some("Maschine Mikro MK3".to_string()),
    true,
    InputMode::Both
);

// Gamepad-only setup
let gamepad_only = InputManager::new(
    None,
    true,
    InputMode::GamepadOnly
);
```

#### Methods

##### `connect()`

```rust
pub fn connect(
    &mut self,
    event_tx: mpsc::Sender<InputEvent>,
    command_tx: mpsc::Sender<DaemonCommand>,
) -> Result<String, String>
```

Connects to input devices based on the configured mode.

**Parameters:**
- `event_tx` - Channel sender for unified `InputEvent` stream
- `command_tx` - Channel sender for daemon commands

**Returns:**
- `Ok(String)` - Status message describing connected devices
- `Err(String)` - Error if no devices could be connected

**Example:**

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};
use tokio::sync::mpsc;

async fn start_unified_input() -> Result<(), String> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut manager = InputManager::new(None, true, InputMode::Both);
    let status = manager.connect(event_tx, command_tx)?;

    println!("Connected: {}", status);
    // Output: "MIDI: Maschine Mikro MK3 (port 0) | Gamepad: Xbox Controller (ID 0)"

    // Process unified event stream
    while let Some(event) = event_rx.recv().await {
        match event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                if pad < 128 {
                    println!("MIDI pad {} pressed (velocity {})", pad, velocity);
                } else {
                    println!("Gamepad button {} pressed", pad);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

##### `is_connected()`

```rust
pub fn is_connected(&self) -> bool
```

Returns `true` if any input device is connected.

##### `get_status()`

```rust
pub fn get_status(&self) -> (bool, bool)
```

Returns connection status for both devices as `(midi_connected, gamepad_connected)`.

**Example:**

```rust
let (midi, gamepad) = manager.get_status();
println!("MIDI: {}, Gamepad: {}", midi, gamepad);
```

##### `disconnect()`

```rust
pub fn disconnect(&mut self)
```

Disconnects all input devices.

##### `mode()`

```rust
pub fn mode(&self) -> InputMode
```

Returns the current input mode.

##### `get_connected_gamepads()`

```rust
pub fn get_connected_gamepads(&self) -> Vec<(String, String)>
```

Returns a list of `(ID, Name)` tuples for connected gamepads.

##### `list_gamepads()` (static)

```rust
pub fn list_gamepads() -> Result<Vec<(gilrs::GamepadId, String, String)>, String>
```

Lists all available gamepads (delegates to `GamepadDeviceManager::list_gamepads()`).

---

## Event Types

### InputEvent

**Location**: `conductor-core/src/events.rs`

Protocol-agnostic input event abstraction. All gamepad events are converted to this type.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    /// Pad pressed (button on controller)
    PadPressed {
        pad: u8,
        velocity: u8,
        time: Instant,
    },

    /// Pad released (button released)
    PadReleased {
        pad: u8,
        time: Instant,
    },

    /// Encoder turned (analog stick or trigger)
    EncoderTurned {
        encoder: u8,
        value: u8,
        time: Instant,
    },

    /// Polyphonic aftertouch/pressure (specific pad)
    PolyPressure {
        pad: u8,
        pressure: u8,
        time: Instant,
    },

    /// Aftertouch/pressure (channel-wide)
    Aftertouch {
        pressure: u8,
        time: Instant,
    },

    /// Pitch bend/touch strip
    PitchBend {
        value: u16,
        time: Instant,
    },

    /// Program change
    ProgramChange {
        program: u8,
        time: Instant,
    },

    /// Generic control change
    ControlChange {
        control: u8,
        value: u8,
        time: Instant,
    },
}
```

#### Gamepad Event Mapping

Gamepad events are converted to `InputEvent` as follows:

| Gamepad Event | InputEvent Variant | ID Range | Notes |
|---------------|-------------------|----------|-------|
| Button Press | `PadPressed` | 128-255 | Velocity = 100 (default) |
| Button Release | `PadReleased` | 128-255 | N/A |
| Analog Stick | `EncoderTurned` | 128-131 | -1.0..1.0 → 0..127 (64 = center) |
| Analog Trigger | `EncoderTurned` | 132-133 | 0.0..1.0 → 0..127 |

---

## Button and Axis IDs

### Button ID Constants

**Location**: `conductor-core/src/gamepad_events.rs`

Gamepad buttons use IDs 128-255 to avoid conflicts with MIDI note numbers (0-127).

```rust
pub mod button_ids {
    // Face buttons (128-131)
    pub const SOUTH: u8 = 128;         // A (Xbox), Cross (PS), B (Nintendo)
    pub const EAST: u8 = 129;          // B (Xbox), Circle (PS), A (Nintendo)
    pub const WEST: u8 = 130;          // X (Xbox), Square (PS), Y (Nintendo)
    pub const NORTH: u8 = 131;         // Y (Xbox), Triangle (PS), X (Nintendo)

    // D-Pad (132-135)
    pub const DPAD_UP: u8 = 132;
    pub const DPAD_DOWN: u8 = 133;
    pub const DPAD_LEFT: u8 = 134;
    pub const DPAD_RIGHT: u8 = 135;

    // Shoulder buttons (136-137)
    pub const LEFT_SHOULDER: u8 = 136;  // L1, LB
    pub const RIGHT_SHOULDER: u8 = 137; // R1, RB

    // Stick clicks (138-139)
    pub const LEFT_THUMB: u8 = 138;    // L3
    pub const RIGHT_THUMB: u8 = 139;   // R3

    // Menu buttons (140-142)
    pub const START: u8 = 140;         // Start, Options, +
    pub const SELECT: u8 = 141;        // Back, Share, -
    pub const GUIDE: u8 = 142;         // Xbox, PS, Home

    // Trigger digital fallback (143-144)
    pub const LEFT_TRIGGER: u8 = 143;  // L2, LT (digital threshold)
    pub const RIGHT_TRIGGER: u8 = 144; // R2, RT (digital threshold)
}
```

### Cross-Platform Button Mapping

| ID | Standard Name | Xbox | PlayStation | Nintendo Switch |
|----|---------------|------|-------------|-----------------|
| 128 | SOUTH | A | Cross (×) | B |
| 129 | EAST | B | Circle (○) | A |
| 130 | WEST | X | Square (□) | Y |
| 131 | NORTH | Y | Triangle (△) | X |
| 132 | DPAD_UP | D-Pad Up | D-Pad Up | D-Pad Up |
| 133 | DPAD_DOWN | D-Pad Down | D-Pad Down | D-Pad Down |
| 134 | DPAD_LEFT | D-Pad Left | D-Pad Left | D-Pad Left |
| 135 | DPAD_RIGHT | D-Pad Right | D-Pad Right | D-Pad Right |
| 136 | LEFT_SHOULDER | LB | L1 | L |
| 137 | RIGHT_SHOULDER | RB | R1 | R |
| 138 | LEFT_THUMB | Left Stick | L3 | Left Stick |
| 139 | RIGHT_THUMB | Right Stick | R3 | Right Stick |
| 140 | START | Start | Options | + (Plus) |
| 141 | SELECT | Back | Share | - (Minus) |
| 142 | GUIDE | Xbox Button | PS Button | Home Button |
| 143 | LEFT_TRIGGER | LT (digital) | L2 (digital) | ZL (digital) |
| 144 | RIGHT_TRIGGER | RT (digital) | R2 (digital) | ZR (digital) |

### Encoder/Axis ID Constants

```rust
pub mod encoder_ids {
    // Analog stick axes (128-131)
    pub const LEFT_STICK_X: u8 = 128;
    pub const LEFT_STICK_Y: u8 = 129;
    pub const RIGHT_STICK_X: u8 = 130;
    pub const RIGHT_STICK_Y: u8 = 131;

    // Trigger axes (132-133)
    pub const LEFT_TRIGGER: u8 = 132;  // L2, LT analog value
    pub const RIGHT_TRIGGER: u8 = 133; // R2, RT analog value
}
```

### Axis Mapping Table

| ID | Axis Name | Range | Normalized | Notes |
|----|-----------|-------|------------|-------|
| 128 | LEFT_STICK_X | -1.0 to 1.0 | 0 to 127 | 64 = center, 0 = left, 127 = right |
| 129 | LEFT_STICK_Y | -1.0 to 1.0 | 0 to 127 | 64 = center, 0 = up, 127 = down |
| 130 | RIGHT_STICK_X | -1.0 to 1.0 | 0 to 127 | 64 = center, 0 = left, 127 = right |
| 131 | RIGHT_STICK_Y | -1.0 to 1.0 | 0 to 127 | 64 = center, 0 = up, 127 = down |
| 132 | LEFT_TRIGGER | 0.0 to 1.0 | 0 to 127 | Analog pressure (L2/LT) |
| 133 | RIGHT_TRIGGER | 0.0 to 1.0 | 0 to 127 | Analog pressure (R2/RT) |

**Note:** A 0.1 deadzone is applied to analog sticks to reduce drift. Values within ±0.1 of center return 64.

---

## Helper Functions

### Button Conversion

**Location**: `conductor-core/src/gamepad_events.rs`

#### `button_to_id()`

```rust
pub fn button_to_id(button: gilrs::Button) -> u8
```

Converts gilrs `Button` enum to Conductor button ID (128-255 range).

**Example:**

```rust
use gilrs::Button;
use conductor_core::gamepad_events::button_to_id;

let id = button_to_id(Button::South);
assert_eq!(id, 128); // SOUTH (A/Cross/B)
```

#### `button_pressed_to_input()`

```rust
pub fn button_pressed_to_input(button: gilrs::Button) -> InputEvent
```

Converts gilrs `ButtonPressed` event to `InputEvent::PadPressed` with default velocity 100.

**Example:**

```rust
use gilrs::Button;
use conductor_core::gamepad_events::button_pressed_to_input;

let event = button_pressed_to_input(Button::South);
// Returns: InputEvent::PadPressed { pad: 128, velocity: 100, time: now() }
```

#### `button_released_to_input()`

```rust
pub fn button_released_to_input(button: gilrs::Button) -> InputEvent
```

Converts gilrs `ButtonReleased` event to `InputEvent::PadReleased`.

### Axis Conversion

#### `axis_to_encoder_id()`

```rust
pub fn axis_to_encoder_id(axis: gilrs::Axis) -> u8
```

Converts gilrs `Axis` enum to Conductor encoder ID (128-133 range).

**Example:**

```rust
use gilrs::Axis;
use conductor_core::gamepad_events::axis_to_encoder_id;

let id = axis_to_encoder_id(Axis::LeftStickX);
assert_eq!(id, 128); // LEFT_STICK_X
```

#### `normalize_axis()`

```rust
pub fn normalize_axis(value: f32) -> u8
```

Normalizes gilrs axis values (-1.0 to 1.0) to MIDI-compatible range (0-127).

**Normalization Rules:**
- Input range: -1.0 to 1.0
- Output range: 0 to 127
- Center point: 64
- Deadzone: ±0.1 (returns 64 if within deadzone)

**Example:**

```rust
use conductor_core::gamepad_events::normalize_axis;

assert_eq!(normalize_axis(0.0), 64);   // Center
assert_eq!(normalize_axis(1.0), 127);  // Max right/up
assert_eq!(normalize_axis(-1.0), 0);   // Max left/down
assert_eq!(normalize_axis(0.05), 64);  // Deadzone (< 0.1)
```

#### `axis_changed_to_input()`

```rust
pub fn axis_changed_to_input(axis: gilrs::Axis, value: f32) -> InputEvent
```

Converts gilrs `AxisChanged` event to `InputEvent::EncoderTurned`.

**Example:**

```rust
use gilrs::Axis;
use conductor_core::gamepad_events::axis_changed_to_input;

let event = axis_changed_to_input(Axis::LeftStickX, 0.5);
// Returns: InputEvent::EncoderTurned { encoder: 128, value: 95, time: now() }
```

---

## Integration Examples

### Basic Gamepad Connection

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;
use tokio::sync::mpsc;
use conductor_core::events::InputEvent;
use conductor_daemon::DaemonCommand;

async fn basic_gamepad_example() -> Result<(), String> {
    // Create channels
    let (event_tx, mut event_rx) = mpsc::channel::<InputEvent>(1024);
    let (command_tx, _) = mpsc::channel::<DaemonCommand>(32);

    // Create manager with auto-reconnect
    let mut manager = GamepadDeviceManager::new(true);

    // Connect to first available gamepad
    let (gamepad_id, gamepad_name) = manager.connect(
        event_tx.clone(),
        command_tx.clone()
    )?;

    println!("Connected to gamepad: {} (ID {:?})", gamepad_name, gamepad_id);

    // Process events
    while let Some(event) = event_rx.recv().await {
        match event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                println!("Button {} pressed (velocity {})", pad, velocity);
            }
            InputEvent::PadReleased { pad, .. } => {
                println!("Button {} released", pad);
            }
            InputEvent::EncoderTurned { encoder, value, .. } => {
                println!("Encoder {} value: {}", encoder, value);
            }
            _ => {}
        }
    }

    Ok(())
}
```

### Hybrid MIDI + Gamepad Setup

```rust
use conductor_daemon::input_manager::{InputManager, InputMode};
use conductor_core::events::InputEvent;
use conductor_core::gamepad_events::button_ids;
use tokio::sync::mpsc;

async fn hybrid_example() -> Result<(), String> {
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    // Create hybrid manager (both MIDI and gamepad)
    let mut manager = InputManager::new(
        Some("Maschine Mikro MK3".to_string()),
        true,
        InputMode::Both
    );

    // Connect to both devices
    let status = manager.connect(event_tx, command_tx)?;
    println!("Connected: {}", status);

    // Process unified event stream
    while let Some(event) = event_rx.recv().await {
        match event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                if pad < 128 {
                    // MIDI pad (0-127)
                    println!("MIDI pad {} pressed (velocity {})", pad, velocity);
                } else {
                    // Gamepad button (128-255)
                    let button_name = match pad {
                        button_ids::SOUTH => "A/Cross",
                        button_ids::EAST => "B/Circle",
                        button_ids::WEST => "X/Square",
                        button_ids::NORTH => "Y/Triangle",
                        button_ids::START => "Start",
                        _ => "Unknown",
                    };
                    println!("Gamepad button {} ({}) pressed", pad, button_name);
                }
            }
            InputEvent::EncoderTurned { encoder, value, .. } => {
                if encoder < 128 {
                    // MIDI encoder/knob
                    println!("MIDI encoder {} value: {}", encoder, value);
                } else {
                    // Gamepad analog stick/trigger
                    println!("Gamepad axis {} value: {}", encoder, value);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

### Integrating with MappingEngine

```rust
use conductor_core::event_processor::EventProcessor;
use conductor_core::mapping::MappingEngine;
use conductor_core::config::Config;
use conductor_daemon::input_manager::{InputManager, InputMode};
use tokio::sync::mpsc;

async fn full_integration_example() -> Result<(), String> {
    // Load configuration
    let config = Config::load_from_path("config.toml")?;

    // Create event processor and mapping engine
    let mut event_processor = EventProcessor::new();
    let mut mapping_engine = MappingEngine::new(config);

    // Set up unified input
    let (event_tx, mut event_rx) = mpsc::channel(1024);
    let (command_tx, _) = mpsc::channel(32);

    let mut input_manager = InputManager::new(
        None,
        true,
        InputMode::Both
    );

    input_manager.connect(event_tx, command_tx)?;

    // Process events through the full pipeline
    while let Some(input_event) = event_rx.recv().await {
        // InputEvent → ProcessedEvent
        if let Some(processed) = event_processor.process(input_event) {
            // ProcessedEvent → Action execution
            mapping_engine.handle_event(&processed);
        }
    }

    Ok(())
}
```

### Listing Available Gamepads

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;

fn list_gamepads_example() -> Result<(), String> {
    let gamepads = GamepadDeviceManager::list_gamepads()?;

    if gamepads.is_empty() {
        println!("No gamepads connected");
    } else {
        println!("Connected gamepads:");
        for (id, name, uuid) in gamepads {
            println!("  - {} (ID: {:?}, UUID: {})", name, id, uuid);
        }
    }

    Ok(())
}
```

---

## Error Handling

### Common Errors

| Error | Cause | Solution |
|-------|-------|----------|
| "Failed to initialize gilrs" | SDL2 not available or system error | Install SDL2, check system permissions |
| "No gamepads connected" | No physical gamepad detected | Connect a gamepad, check USB connection |
| "Already connected to a gamepad" | Attempted to connect twice | Call `disconnect()` before reconnecting |
| "No input devices could be connected" | Both MIDI and gamepad failed | Check device connections, verify drivers |

### Handling Disconnections

The `GamepadDeviceManager` automatically handles disconnections when `auto_reconnect` is enabled:

```rust
let mut manager = GamepadDeviceManager::new(true); // auto_reconnect = true

// Disconnection is detected automatically
// Reconnection attempts occur with exponential backoff:
// 1s, 2s, 4s, 8s, 16s, 30s (max 6 attempts)

// Listen for reconnection commands
while let Some(command) = command_rx.recv().await {
    match command {
        DaemonCommand::ReconnectGamepad => {
            println!("Gamepad reconnected!");
            // Manager automatically reconnects
        }
        _ => {}
    }
}
```

### Manual Error Handling

```rust
use conductor_daemon::gamepad_device::GamepadDeviceManager;

fn safe_connect() {
    let mut manager = GamepadDeviceManager::new(false); // no auto-reconnect

    loop {
        match manager.connect(event_tx.clone(), command_tx.clone()) {
            Ok((id, name)) => {
                println!("Connected to: {}", name);
                break;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
                // Retry after 5 seconds
            }
        }
    }
}
```

---

## Configuration Examples

### TOML Configuration for Gamepad Mappings

```toml
# config.toml

[device]
name = "Hybrid Controller"
auto_connect = true

[[global_mappings]]
[global_mappings.trigger]
type = "Note"
note = 128  # Gamepad SOUTH button (A/Cross)

[[global_mappings.actions]]
type = "Keystroke"
key = "Space"

[[global_mappings]]
[global_mappings.trigger]
type = "Note"
note = 140  # Gamepad START button

[[global_mappings.actions]]
type = "Launch"
app = "Terminal"

[[global_mappings]]
[global_mappings.trigger]
type = "EncoderTurn"
encoder = 128  # Left stick X-axis
direction = "Clockwise"

[[global_mappings.actions]]
type = "VolumeControl"
action = "Up"
```

### Velocity-Sensitive Gamepad Triggers (Future Enhancement)

Currently, gamepad buttons use a fixed velocity of 100. Future versions may support pressure-sensitive triggers:

```toml
# Future feature (not yet implemented)
[[global_mappings]]
[global_mappings.trigger]
type = "VelocityRange"
note = 132  # Left trigger (analog)
min_velocity = 80
max_velocity = 127

[[global_mappings.actions]]
type = "Keystroke"
key = "F"
modifiers = ["Shift"]  # Hard press = Shift+F
```

---

## Thread Safety and Concurrency

### Arc/Mutex Patterns

The gamepad system uses Rust's `Arc<Mutex<>>` and `Arc<AtomicBool>` for safe concurrent access:

```rust
// Internal state (Arc<Mutex<>>)
gamepad_id: Arc<Mutex<Option<gilrs::GamepadId>>>
gamepad_name: Arc<Mutex<Option<String>>>
polling_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>

// Atomic flags (Arc<AtomicBool>)
is_connected: Arc<AtomicBool>
stop_polling: Arc<AtomicBool>
```

### Polling Thread Architecture

```text
┌─────────────────────────────────────────────────────────┐
│  Main Thread                                            │
│  - Creates GamepadDeviceManager                         │
│  - Calls connect()                                      │
│  - Receives InputEvents via mpsc channel                │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼ spawns
┌─────────────────────────────────────────────────────────┐
│  Polling Thread                                         │
│  - Polls gilrs at 1ms intervals                         │
│  - Converts gilrs events → InputEvent                   │
│  - Sends via mpsc::Sender<InputEvent>                   │
│  - Detects disconnection                                │
│  - Stops on stop_polling signal                         │
└─────────────────────────────────────────────────────────┘
                        │
                        ▼ spawns on disconnect
┌─────────────────────────────────────────────────────────┐
│  Reconnection Thread (if auto_reconnect = true)         │
│  - Exponential backoff (1s, 2s, 4s, 8s, 16s, 30s)      │
│  - Checks for available gamepads                        │
│  - Sends DaemonCommand::ReconnectGamepad when found     │
└─────────────────────────────────────────────────────────┘
```

---

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Polling Interval | 1ms | Balances latency and CPU usage |
| Reconnect Attempts | 6 | Exponential backoff schedule |
| Max Reconnect Time | ~60s | Sum of backoff delays |
| Event Channel Capacity | 1024 | Default mpsc buffer size |
| Event Latency | <5ms | gilrs → InputEvent → channel |
| CPU Usage (Idle) | <1% | Efficient polling loop |
| Memory Overhead | ~100KB | Per GamepadDeviceManager |

---

## Device Compatibility

### Tested Controllers

| Controller | Status | Notes |
|------------|--------|-------|
| Xbox One/Series Controllers | ✅ Fully Supported | SDL2 GameController mapping |
| PlayStation 4/5 DualShock/DualSense | ✅ Fully Supported | Standard button layout |
| Nintendo Switch Pro Controller | ✅ Fully Supported | Button labels differ (A/B swapped) |
| Generic USB Gamepads | ✅ Supported | May require custom SDL2 mapping |
| Logitech F310/F710 | ✅ Supported | Switch to XInput mode |
| 8BitDo Controllers | ✅ Supported | Use XInput/Switch mode |

### HID Device Types

The gamepad system supports any HID-compliant game controller:

- **Gamepads**: Xbox, PlayStation, Nintendo, generic USB pads
- **Joysticks**: Flight sticks, arcade sticks
- **Racing Wheels**: Logitech G29, Thrustmaster T300
- **HOTAS**: Hands-On Throttle-and-Stick setups
- **Custom Controllers**: Any SDL2-compatible HID device

### Platform Support

| Platform | Status | Requirements |
|----------|--------|--------------|
| macOS | ✅ Supported | Native HID support |
| Linux | ✅ Supported | SDL2 + udev rules |
| Windows | ✅ Supported | SDL2 + XInput |

---

## Debugging and Diagnostics

### Enabling Debug Logging

```bash
# Enable tracing logs
RUST_LOG=debug cargo run

# Filter for gamepad-specific logs
RUST_LOG=conductor_daemon::gamepad_device=trace cargo run
```

### Diagnostic Commands

```bash
# List connected gamepads
cargo run --bin list_gamepads

# Test gamepad input
cargo run --bin test_gamepad_input

# Monitor unified event stream
cargo run --bin event_console
```

### Example Debug Output

```
[DEBUG conductor_daemon::gamepad_device] Connecting to gamepad: Xbox Controller (ID: GamepadId(0))
[TRACE conductor_daemon::gamepad_device] Gamepad event: Event { id: GamepadId(0), event: ButtonPressed(South, 0) }
[DEBUG conductor_daemon::gamepad_device] Button 128 (SOUTH) pressed
[TRACE conductor_daemon::gamepad_device] Sent InputEvent::PadPressed { pad: 128, velocity: 100 }
[TRACE conductor_daemon::gamepad_device] Gamepad event: Event { id: GamepadId(0), event: AxisChanged(LeftStickX, 0.523, 0) }
[DEBUG conductor_daemon::gamepad_device] Encoder 128 (LEFT_STICK_X) value: 95
```

---

## Future Enhancements

### Planned Features (Not Yet Implemented)

1. **Pressure-Sensitive Buttons**: Variable velocity based on analog button pressure
2. **Gyroscope/Accelerometer Support**: Motion controls for advanced controllers
3. **Haptic Feedback**: Rumble/vibration control via actions
4. **Custom Button Mappings**: Override default button-to-ID mappings
5. **Multi-Controller Support**: Connect multiple gamepads simultaneously
6. **Per-Controller Profiles**: Different mappings for different gamepad models
7. **Axis Inversion/Scaling**: Fine-tune analog stick sensitivity
8. **Macro Recording**: Record gamepad input sequences

### Experimental Features

```rust
// Future API (not yet available)
pub struct GamepadConfig {
    pub deadzone: f32,
    pub sensitivity: f32,
    pub invert_y_axis: bool,
    pub button_mappings: HashMap<gilrs::Button, u8>,
}

impl GamepadDeviceManager {
    pub fn new_with_config(
        auto_reconnect: bool,
        config: GamepadConfig
    ) -> Self { /* ... */ }
}
```

---

## See Also

- [Gamepad Support Guide](../guides/gamepad-support.md) - User-facing documentation
- [Configuration Schema](config-schema.md) - TOML configuration reference
- [Trigger Types](trigger-types.md) - Available trigger configurations
- [Action Types](action-types.md) - Available action types
- [Architecture Overview](../development/architecture.md) - System design

---

## Glossary

| Term | Definition |
|------|------------|
| **HID** | Human Interface Device - USB standard for input devices |
| **gilrs** | Rust library for game controller input (built on SDL2) |
| **SDL2** | Simple DirectMedia Layer - cross-platform game controller API |
| **InputEvent** | Protocol-agnostic event abstraction |
| **GamepadId** | gilrs identifier for a specific connected gamepad |
| **Arc/Mutex** | Rust concurrency primitives for shared state |
| **mpsc** | Multi-Producer, Single-Consumer channel for thread communication |

---

**Last Updated**: 2025-11-21
**API Version**: v3.0
**Crate Versions**:
- `conductor-core`: 3.0.0
- `conductor-daemon`: 3.0.0
- `gilrs`: 0.11.0
