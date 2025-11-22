# midimon-core API Design

**Version**: 0.1.0-alpha
**Status**: Phase 1 - API Design
**Last Updated**: 2025-11-11

## Overview

This document defines the public API for the `midimon-core` library crate that will be extracted from the current monolithic implementation in Phase 2 of the migration plan. The core library provides a UI-agnostic event processing engine for MIDI/HID controller mapping.

## Design Goals

1. **UI Independence**: Core library has zero UI dependencies
2. **Integration Patterns**: Support CLI, daemon, and future GUI applications
3. **Thread Safety**: Safe concurrent access with clear ownership boundaries
4. **Hot Reload**: Prepare for config hot-reloading in Phase 3
5. **Backward Compatibility**: Maintain existing config.toml format
6. **Performance**: <1ms event latency, minimal allocations in hot path

## Public API Surface

### Module Structure

```rust
// Public modules
pub mod config;          // Configuration types and loading
pub mod engine;          // Main engine entry point
pub mod events;          // Event types and processing
pub mod mapping;         // Mapping engine
pub mod actions;         // Action types and execution
pub mod feedback;        // LED/visual feedback system
pub mod device;          // Device abstraction and profiles
pub mod error;           // Error types

// Private modules (implementation details)
mod event_processor;     // Event state machine
mod timing;              // Long press/double-tap detection
mod chord;               // Chord detection
mod velocity;            // Velocity range processing
```

### Main Engine API

The core entry point for applications integrating midimon-core:

```rust
/// Main MIDIMon engine
pub struct MidiMonEngine {
    // Private fields
}

impl MidiMonEngine {
    /// Create a new engine instance with the given configuration
    ///
    /// # Errors
    /// Returns error if config is invalid or device connections fail
    pub fn new(config: Config) -> Result<Self, EngineError>;

    /// Start the engine (blocks until stop is called)
    ///
    /// Connects to MIDI devices, initializes LED feedback, and begins
    /// processing events. This method blocks the current thread.
    ///
    /// # Thread Safety
    /// Safe to call from any thread, but should only be called once.
    pub fn start(&mut self) -> Result<(), EngineError>;

    /// Stop the engine gracefully
    ///
    /// Disconnects devices and releases resources. Pending events
    /// in the pipeline will be processed before shutdown.
    pub fn stop(&mut self) -> Result<(), EngineError>;

    /// Reload configuration without restarting the engine
    ///
    /// Hot-reloads mappings, device profiles, and settings. Active
    /// MIDI connections are preserved if device config unchanged.
    ///
    /// # Phase 3 Feature
    /// Full implementation deferred to Phase 3 (hot-reload support)
    pub fn reload_config(&mut self, config: Config) -> Result<(), EngineError>;

    /// Get current mode index
    pub fn current_mode(&self) -> u8;

    /// Switch to a different mode by index
    ///
    /// # Errors
    /// Returns error if mode index is out of bounds
    pub fn set_mode(&mut self, mode: u8) -> Result<(), EngineError>;

    /// Get a clone of the current configuration
    pub fn config(&self) -> Config;

    /// Get engine statistics (event counts, processing times)
    pub fn stats(&self) -> EngineStats;

    /// Register a callback for mode changes
    ///
    /// The callback will be invoked when the mode changes via encoder
    /// rotation or programmatic mode switching.
    pub fn on_mode_change<F>(&mut self, callback: F)
    where
        F: Fn(u8) + Send + 'static;

    /// Register a callback for action execution
    ///
    /// The callback receives the processed event and action before execution.
    /// Useful for logging, debugging, or external integration.
    pub fn on_action<F>(&mut self, callback: F)
    where
        F: Fn(&ProcessedEvent, &Action) + Send + 'static;
}

/// Engine runtime statistics
#[derive(Debug, Clone)]
pub struct EngineStats {
    pub events_processed: u64,
    pub actions_executed: u64,
    pub mode_changes: u64,
    pub avg_event_latency_us: u64,
    pub uptime: Duration,
}
```

### Configuration API

```rust
/// Complete application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<Mapping>,
    pub advanced_settings: AdvancedSettings,
}

impl Config {
    /// Load configuration from a TOML file
    ///
    /// # Errors
    /// Returns error if file doesn't exist, cannot be read, or is invalid
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;

    /// Load configuration from a TOML string
    pub fn from_str(toml: &str) -> Result<Self, ConfigError>;

    /// Save configuration to a TOML file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError>;

    /// Convert to TOML string
    pub fn to_string(&self) -> Result<String, ConfigError>;

    /// Validate configuration (checks for invalid notes, trigger conflicts, etc.)
    pub fn validate(&self) -> Result<(), ConfigError>;

    /// Create a default configuration
    pub fn default() -> Self;
}

/// Device-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub auto_connect: bool,
    #[serde(default)]
    pub hid_enabled: bool,
    #[serde(default)]
    pub profile_path: Option<PathBuf>,
}

/// Mode definition with mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub color: Option<String>,
    pub mappings: Vec<Mapping>,
}

/// Advanced timing and detection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    #[serde(default = "default_chord_timeout")]
    pub chord_timeout_ms: u64,

    #[serde(default = "default_double_tap_timeout")]
    pub double_tap_timeout_ms: u64,

    #[serde(default = "default_hold_threshold")]
    pub hold_threshold_ms: u64,

    #[serde(default = "default_velocity_soft")]
    pub velocity_soft_max: u8,

    #[serde(default = "default_velocity_medium")]
    pub velocity_medium_max: u8,
}

fn default_chord_timeout() -> u64 { 50 }
fn default_double_tap_timeout() -> u64 { 300 }
fn default_hold_threshold() -> u64 { 2000 }
fn default_velocity_soft() -> u8 { 40 }
fn default_velocity_medium() -> u8 { 80 }
```

### Event Types

```rust
/// Raw MIDI event from device
#[derive(Debug, Clone)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8, time: Instant },
    NoteOff { note: u8, time: Instant },
    ControlChange { cc: u8, value: u8, time: Instant },
    Aftertouch { pressure: u8, time: Instant },
    PitchBend { value: u16, time: Instant },
    ProgramChange { program: u8, time: Instant },
}

/// Processed event with timing-based detection
#[derive(Debug, Clone)]
pub enum ProcessedEvent {
    /// Short tap (< 200ms)
    ShortPress { note: u8 },

    /// Medium press (200-1000ms)
    MediumPress { note: u8, duration_ms: u128 },

    /// Long press (> hold_threshold, default 2000ms)
    LongPress { note: u8, duration_ms: u128 },

    /// Hold detected (triggered at hold_threshold while still pressed)
    HoldDetected { note: u8 },

    /// Pad pressed with velocity information
    PadPressed { note: u8, velocity: u8, velocity_level: VelocityLevel },

    /// Pad released with hold duration
    PadReleased { note: u8, hold_duration_ms: u128 },

    /// Encoder rotation
    EncoderTurned { cc: u8, value: u8, direction: EncoderDirection, delta: u8 },

    /// Double tap detected (two presses within double_tap_timeout)
    DoubleTap { note: u8 },

    /// Chord detected (multiple notes pressed within chord_timeout)
    ChordDetected { notes: Vec<u8> },

    /// Aftertouch/pressure change
    AftertouchChanged { pressure: u8 },

    /// Pitch bend/touch strip
    PitchBendMoved { value: u16 },
}

/// Velocity sensitivity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VelocityLevel {
    Soft,    // 0-40 (configurable)
    Medium,  // 41-80 (configurable)
    Hard,    // 81-127
}

/// Encoder rotation direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncoderDirection {
    Clockwise,
    CounterClockwise,
}
```

### Trigger System

```rust
/// Trigger configuration (what activates a mapping)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    /// Basic note trigger with optional velocity range
    Note {
        note: u8,
        velocity_min: Option<u8>,
        velocity_max: Option<u8>,
    },

    /// Velocity-sensitive trigger (different actions per velocity level)
    VelocityRange {
        note: u8,
        soft: Option<ActionConfig>,
        medium: Option<ActionConfig>,
        hard: Option<ActionConfig>,
    },

    /// Long press detection
    LongPress {
        note: u8,
        duration_ms: Option<u64>,  // Override default hold_threshold
    },

    /// Double-tap detection
    DoubleTap {
        note: u8,
        window_ms: Option<u64>,  // Override default double_tap_timeout
    },

    /// Chord detection (multiple notes simultaneously)
    NoteChord {
        notes: Vec<u8>,
        window_ms: Option<u64>,  // Override default chord_timeout
    },

    /// Encoder rotation
    EncoderTurn {
        cc: u8,
        direction: Option<EncoderDirection>,  // None = either direction
    },

    /// Aftertouch/pressure trigger
    Aftertouch {
        threshold: u8,
    },

    /// Pitch bend trigger
    PitchBend {
        threshold: u16,
    },

    /// Control Change trigger
    CC {
        cc: u8,
        value_min: Option<u8>,
        value_max: Option<u8>,
    },
}
```

### Action System

```rust
/// Action to execute when trigger matches
#[derive(Debug, Clone)]
pub enum Action {
    /// Keyboard shortcut with modifiers
    Keystroke {
        keys: Vec<Key>,
        modifiers: Vec<Key>,
    },

    /// Type text string
    Text(String),

    /// Launch application
    Launch(String),

    /// Execute shell command
    Shell(String),

    /// System volume control
    VolumeControl(VolumeAction),

    /// Change mode
    ModeChange(u8),

    /// Sequence of actions (execute in order)
    Sequence(Vec<Action>),

    /// Delay between actions (in sequence)
    Delay(u64),

    /// Mouse click/movement
    MouseClick {
        button: Button,
        x: Option<i32>,
        y: Option<i32>,
    },

    /// Repeat an action N times
    Repeat {
        action: Box<Action>,
        count: usize,
    },

    /// Conditional action based on system state
    Conditional {
        condition: Condition,
        then_action: Box<Action>,
        else_action: Option<Box<Action>>,
    },
}

/// Volume control actions
#[derive(Debug, Clone, Copy)]
pub enum VolumeAction {
    Up,
    Down,
    Mute,
    Set(u8),  // 0-100
}

/// Conditional checks
#[derive(Debug, Clone)]
pub enum Condition {
    /// Check if application is running/focused
    AppActive(String),

    /// Check current time (e.g., "09:00-17:00")
    TimeRange { start: String, end: String },

    /// Check mode
    ModeEquals(u8),
}

/// Action executor trait (injected into engine)
pub trait ActionExecutor: Send + Sync {
    /// Execute an action
    fn execute(&mut self, action: &Action) -> Result<(), ActionError>;
}
```

### Feedback System

```rust
/// LED/visual feedback abstraction
pub trait FeedbackController: Send + Sync {
    /// Connect to feedback device
    fn connect(&mut self) -> Result<(), FeedbackError>;

    /// Set single pad color
    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), FeedbackError>;

    /// Show velocity-based feedback (auto color based on velocity)
    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), FeedbackError>;

    /// Set all pads to mode colors
    fn set_mode_colors(&mut self, mode: u8) -> Result<(), FeedbackError>;

    /// Show velocity feedback (flash/fade based on velocity)
    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), FeedbackError>;

    /// Flash pad (temporary highlight)
    fn flash_pad(&mut self, pad: u8, color: RGB, duration_ms: u64) -> Result<(), FeedbackError>;

    /// Ripple effect from pad
    fn ripple_effect(&mut self, start_pad: u8, color: RGB) -> Result<(), FeedbackError>;

    /// Clear all LEDs
    fn clear_all(&mut self) -> Result<(), FeedbackError>;

    /// Show long press feedback (progressive indicator)
    fn show_long_press_feedback(&mut self, pad: u8, elapsed_ms: u128) -> Result<(), FeedbackError>;

    /// Run lighting scheme
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), FeedbackError>;
}

/// RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// LED lighting schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightingScheme {
    Off,
    Static(u8),       // Static color based on mode
    Breathing,        // Slow breathing effect
    Pulse,            // Fast pulse effect
    Rainbow,          // Rainbow cycle
    Wave,             // Wave pattern
    Sparkle,          // Random sparkles
    Reactive,         // React to MIDI events only
    VuMeter,          // VU meter style (bottom-up)
    Spiral,           // Spiral pattern
}
```

### Device Profiles

```rust
/// Device profile (maps physical pads to MIDI notes)
pub struct DeviceProfile {
    pub name: String,
    pub device_type: String,
    pub pad_pages: Vec<PadPageMapping>,
}

impl DeviceProfile {
    /// Load NI Controller Editor profile (.ncmm3 XML)
    pub fn from_ncmm3<P: AsRef<Path>>(path: P) -> Result<Self, ProfileError>;

    /// Get pad page by name (e.g., "Pad Page A")
    pub fn get_page_by_name(&self, name: &str) -> Option<&PadPageMapping>;

    /// Get pad page by letter (e.g., "A", "B")
    pub fn get_page_by_letter(&self, letter: char) -> Option<&PadPageMapping>;

    /// Auto-detect which pad page contains a note
    pub fn detect_page_for_note(&self, note: u8) -> Option<&PadPageMapping>;
}

/// Pad page mapping (note <-> pad index)
#[derive(Debug, Clone)]
pub struct PadPageMapping {
    pub name: String,
    note_to_pad: HashMap<u8, usize>,
    pad_to_note: Vec<u8>,
}

impl PadPageMapping {
    /// Convert MIDI note to pad index (0-15)
    pub fn note_to_pad_index(&self, note: u8) -> Option<usize>;

    /// Convert pad index to MIDI note
    pub fn pad_index_to_note(&self, pad_index: usize) -> Option<u8>;

    /// Get note range for this page
    pub fn note_range(&self) -> Option<(u8, u8)>;
}
```

### Error Types

```rust
/// Engine errors
#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("MIDI connection failed: {0}")]
    MidiConnectionFailed(String),

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Already running")]
    AlreadyRunning,

    #[error("Not running")]
    NotRunning,

    #[error("Invalid mode: {0}")]
    InvalidMode(u8),
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid trigger: {0}")]
    InvalidTrigger(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
}

/// Action execution errors
#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Application not found: {0}")]
    AppNotFound(String),
}

/// Feedback errors
#[derive(Debug, thiserror::Error)]
pub enum FeedbackError {
    #[error("Device not connected")]
    NotConnected,

    #[error("HID error: {0}")]
    HidError(String),

    #[error("MIDI error: {0}")]
    MidiError(String),
}

/// Profile errors
#[derive(Debug, thiserror::Error)]
pub enum ProfileError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("XML parse error: {0}")]
    XmlError(String),

    #[error("Invalid profile: {0}")]
    InvalidProfile(String),
}
```

## Integration Patterns

### CLI Application

```rust
use midimon_core::{Config, MidiMonEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load("config.toml")?;

    // Create and start engine
    let mut engine = MidiMonEngine::new(config)?;

    // Register Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Start engine (blocking)
    engine.start()?;

    // Wait for Ctrl+C
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    // Graceful shutdown
    engine.stop()?;
    Ok(())
}
```

### Daemon with Callbacks

```rust
use midimon_core::{Config, MidiMonEngine, ProcessedEvent, Action};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("config.toml")?;
    let mut engine = MidiMonEngine::new(config)?;

    // Register mode change callback
    engine.on_mode_change(|mode| {
        println!("Mode changed to: {}", mode);
        // Update menu bar icon, send notification, etc.
    });

    // Register action callback for logging
    engine.on_action(|event, action| {
        println!("Executing: {:?} -> {:?}", event, action);
    });

    engine.start()?;
    Ok(())
}
```

### Hot-Reload Support (Phase 3)

```rust
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config.toml";
    let config = Config::load(config_path)?;
    let mut engine = MidiMonEngine::new(config)?;

    // Watch config file for changes
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1))?;
    watcher.watch(config_path, RecursiveMode::NonRecursive)?;

    // Spawn config reload thread
    let engine_handle = Arc::new(Mutex::new(engine));
    let reload_handle = engine_handle.clone();
    thread::spawn(move || {
        while let Ok(_event) = rx.recv() {
            if let Ok(new_config) = Config::load(config_path) {
                let mut engine = reload_handle.lock().unwrap();
                let _ = engine.reload_config(new_config);
                println!("Config reloaded");
            }
        }
    });

    // Start engine
    let mut engine = engine_handle.lock().unwrap();
    engine.start()?;
    Ok(())
}
```

## Thread Safety and Concurrency

### Ownership Model

- **Engine**: Owns MIDI connections, event processor, mapping engine, action executor
- **Config**: Wrapped in `Arc<RwLock<Config>>` for shared read access and hot-reload
- **Callbacks**: Stored as `Box<dyn Fn(...) + Send + 'static>`
- **Feedback**: Wrapped in `Arc<Mutex<dyn FeedbackController>>` for thread-safe LED updates

### Event Pipeline Threading

```
┌─────────────────────┐
│   MIDI Callback     │  (midir thread - lock-free)
│   [NoteOn/CC/etc]   │
└──────────┬──────────┘
           │ crossbeam_channel::send
           ▼
┌─────────────────────┐
│  Event Processor    │  (dedicated thread)
│  [State Machine]    │
│  - Long press       │
│  - Double-tap       │
│  - Chord detection  │
└──────────┬──────────┘
           │ ProcessedEvent
           ▼
┌─────────────────────┐
│  Mapping Engine     │  (same thread)
│  [Match triggers]   │
└──────────┬──────────┘
           │ Action
           ▼
┌─────────────────────┐
│  Action Executor    │  (same thread)
│  [Execute action]   │
└─────────────────────┘
```

### Async Considerations

- **Phase 1-2**: Synchronous API (blocking `start()`)
- **Phase 3+**: Consider async engine API with `tokio::spawn`
- **Current**: `crossbeam-channel` for lock-free event passing

## Migration Notes

### Phase 2: Extraction

**What becomes public:**
- `Config`, `Mode`, `Mapping`, `Trigger`, `ActionConfig`
- `MidiMonEngine` with `new()`, `start()`, `stop()`
- `MidiEvent`, `ProcessedEvent`, `Action`
- `FeedbackController` trait
- `DeviceProfile`, `PadPageMapping`

**What stays private:**
- `EventProcessor` (internal state machine)
- Timing detection (`timing.rs`)
- Chord detection (`chord.rs`)
- Key parsing (`parse_key`, `parse_modifier`)
- HID/MIDI connection details

**Dependencies to extract:**
- Keep: `midir`, `enigo`, `serde`, `toml`, `crossbeam-channel`, `hidapi`
- Remove from core: `colored`, `chrono` (logging moved to application layer)
- Add: `thiserror` for error types

### Backward Compatibility

- Existing `config.toml` format unchanged
- CLI arguments preserved
- LED feedback behavior identical
- Device profile format compatible

## Next Steps

1. **Phase 2**: Create `midimon-core/` crate with this API
2. **Phase 2**: Refactor current code into core library
3. **Phase 2**: Create `midimon-cli/` binary using core
4. **Phase 3**: Add `midimon-daemon/` with menu bar
5. **Phase 4**: Add `midimon-gui/` with Tauri UI

## References

- Current implementation: `src/*.rs`
- Research documents: `.research/implementation-viewpoint-1.md`, `implementation-viewpoint-2.md`
- Migration plan: `CLAUDE.md` (Workspace Structure section)
