# MIDIMon Technical Architecture
## System Design & Migration Strategy

**Version**: 1.0
**Last Updated**: 2025-11-11
**Purpose**: Technical architecture documentation for developers and contributors

---

## Table of Contents

1. [Current Architecture (v0.1.0)](#current-architecture-v010)
2. [Target Architecture (v2.0)](#target-architecture-v20)
3. [Data Flow](#data-flow)
4. [API Specifications](#api-specifications)
5. [Performance Characteristics](#performance-characteristics)
6. [Migration Plan](#migration-plan)
7. [Security & Privacy](#security--privacy)
8. [Platform Considerations](#platform-considerations)

---

## Current Architecture (v0.1.0)

### Overview

Single-binary Rust application with all functionality in one crate. Optimized for sub-millisecond latency and minimal memory footprint.

### System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                          MIDIMon v0.1.0                          │
│                      (Single Binary ~3-5MB)                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌────────────┐   ┌──────────────┐   ┌───────────────┐        │
│  │   MIDI     │   │    Event     │   │   Mapping     │        │
│  │   Input    ├──→│  Processor   ├──→│    Engine     │        │
│  │ (midir)    │   │ (patterns)   │   │  (rules)      │        │
│  └────────────┘   └──────────────┘   └───────┬───────┘        │
│                                                │                 │
│  ┌────────────┐   ┌──────────────┐            │                │
│  │    LED     │   │   Action     │            │                │
│  │  Feedback  │   │  Executor    │←───────────┘                │
│  │ (HID/MIDI) │   │ (enigo/shell)│                             │
│  └────────────┘   └──────────────┘                             │
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │             Config (config.toml - TOML format)             │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

#### main.rs (600 lines)
- **Entry Point**: Command-line parsing, device connection
- **MIDI Connection**: Setup midir input with callback
- **Event Loop**: Channel-based event routing
- **Coordination**: Ties all components together

```rust
// Key responsibilities:
- MIDI callback setup
- Event channel management (crossbeam-channel)
- Mode tracking (AtomicU8)
- LED feedback initialization
- Device profile loading
- Graceful shutdown (ctrlc handler)
```

#### config.rs (300 lines)
- **Structures**: `Config`, `Mode`, `Mapping`, `Trigger`, `Action`
- **TOML Parsing**: Serde deserialization
- **Validation**: Ensure valid configuration

```rust
pub struct Config {
    pub device: DeviceConfig,
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<Mapping>,
    pub advanced_settings: AdvancedSettings,
}
```

#### event_processor.rs (400 lines)
- **Pattern Detection**: Long press, double-tap, chords
- **State Management**: HashMap-based timers
- **Velocity Detection**: Soft/medium/hard categorization
- **Encoder Logic**: Direction detection from CC values

```rust
pub struct EventProcessor {
    note_press_times: HashMap<u8, Instant>,
    held_notes: HashMap<u8, Instant>,
    last_cc_values: HashMap<u8, u8>,
    last_note_tap: HashMap<u8, Instant>,
    chord_buffer: Vec<(u8, Instant)>,
}
```

#### mappings.rs (400 lines)
- **Mapping Engine**: Match events to actions
- **Compilation**: Convert config to runtime structures
- **Mode Management**: Per-mode and global mappings
- **Trigger Matching**: Pattern matching logic

```rust
pub struct MappingEngine {
    mode_mappings: HashMap<u8, Vec<CompiledMapping>>,
    global_mappings: Vec<CompiledMapping>,
}
```

#### actions.rs (500 lines)
- **Action Executor**: Execute all action types
- **Keystroke**: enigo for keyboard simulation
- **Shell**: Process spawning
- **Volume**: AppleScript integration (macOS)
- **Sequences**: Multi-action chains

```rust
pub enum Action {
    Keystroke { keys: Vec<Key>, modifiers: Vec<Key> },
    Text(String),
    Launch(String),
    Shell(String),
    VolumeControl(VolumeAction),
    ModeChange(u8),
    Sequence(Vec<Action>),
    // ... more
}
```

#### feedback.rs (200 lines)
- **Trait Definition**: `PadFeedback` abstraction
- **Device Factory**: Create HID or MIDI feedback
- **LED Schemes**: Enum and color definitions

```rust
pub trait PadFeedback: Send {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>>;
}
```

#### mikro_leds.rs (600 lines)
- **HID Communication**: hidapi with macos-shared-device
- **RGB Control**: 16 pads × 3 bytes (RGB)
- **Schemes**: All 10 lighting animations
- **Vendor Specific**: NI Mikro MK3 protocol

#### midi_feedback.rs (200 lines)
- **MIDI LED**: Note On/Off for LED control
- **Fallback**: Basic on/off for non-HID devices

#### device_profile.rs (300 lines)
- **XML Parser**: quick-xml for .ncmm3 files
- **Pad Mapping**: Physical position to MIDI note
- **Page Detection**: Auto-detect active pad page

### Dependencies

```toml
[dependencies]
midir = "0.10"                # MIDI I/O (CoreMIDI wrapper)
enigo = "0.6"                 # Keyboard/mouse simulation
hidapi = "2.4"                # HID device access
serde = "1.0"                 # Serialization
toml = "0.9"                  # Config parsing
quick-xml = "0.36"            # XML parsing
crossbeam-channel = "0.5"     # Lock-free channels
colored = "3.0"               # Terminal colors
ctrlc = "3.4"                 # Shutdown handling
chrono = "0.4"                # Timestamps
rand = "0.8"                  # LED randomization
```

### Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| **Binary Size** | 3-5 MB | With LTO and strip |
| **Memory (idle)** | 5-10 MB | Resident set size |
| **Memory (active)** | 10-15 MB | With LED animations |
| **CPU (idle)** | <1% | On modern Mac |
| **CPU (active)** | <5% | Sustained load |
| **Latency (input→action)** | <1ms | 95th percentile |
| **Latency (MIDI callback)** | <100μs | Internal processing |
| **LED Update Rate** | 10 FPS | Animated schemes |
| **Event Throughput** | 1000+ events/sec | Sustained |
| **Startup Time** | <500ms | To functional |

### File Structure

```
midimon/
├── Cargo.toml                 # Project manifest
├── config.toml                # User configuration
├── src/
│   ├── main.rs                # Entry point
│   ├── config.rs              # Config structures
│   ├── event_processor.rs     # Pattern detection
│   ├── mappings.rs            # Mapping engine
│   ├── actions.rs             # Action execution
│   ├── feedback.rs            # LED abstraction
│   ├── mikro_leds.rs          # HID LED control
│   ├── midi_feedback.rs       # MIDI LED control
│   ├── device_profile.rs      # Profile parser
│   └── bin/
│       ├── test_midi.rs       # Port tester
│       ├── midi_diagnostic.rs # Event viewer
│       ├── led_diagnostic.rs  # LED tester
│       ├── led_tester.rs      # LED scheme tester
│       └── pad_mapper.rs      # Pad mapping utility
├── target/                    # Build artifacts
├── CLAUDE.md                  # Development guide
├── LED_FEEDBACK.md            # LED documentation
└── README.md                  # User guide
```

---

## Target Architecture (v2.0)

### Overview

Workspace structure with three crates: reusable core engine, background daemon, and visual UI.

### System Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         MIDIMon v2.0 (Workspace)                         │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                         midimon-daemon (10MB)                            │
│                     Background Service + Menu Bar                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│  ┌──────────────────┐     ┌──────────────────┐                         │
│  │   LaunchAgent    │     │   Menu Bar App   │                         │
│  │  (Auto-start)    │     │   (NSStatusItem) │                         │
│  └──────────────────┘     └──────────────────┘                         │
│                                                                           │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                     midimon-core (lib)                             │ │
│  │  ┌──────────┐  ┌──────────┐  ┌─────────┐  ┌────────┐           │ │
│  │  │  Device  │  │  Events  │  │ Mapping │  │ Actions│           │ │
│  │  │   I/O    │→│ Processor│→│  Engine │→│ Executor│           │ │
│  │  └──────────┘  └──────────┘  └─────────┘  └────────┘           │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                           │
│  ┌──────────────────┐     ┌──────────────────┐                         │
│  │  Config Watcher  │     │  App Detection   │                         │
│  │   (hot reload)   │     │  (frontmost app) │                         │
│  └──────────────────┘     └──────────────────┘                         │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                         midimon-gui (Tauri v2)                           │
│                      Visual Configuration UI (5MB)                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│  ┌──────────────────┐     ┌──────────────────┐                         │
│  │  Device          │     │   Action         │                         │
│  │  Visualization   │     │   Library        │                         │
│  │  (interactive)   │     │  (drag-drop)     │                         │
│  └──────────────────┘     └──────────────────┘                         │
│                                                                           │
│  ┌──────────────────┐     ┌──────────────────┐                         │
│  │  MIDI Learn      │     │  Profile         │                         │
│  │  (capture)       │     │  Manager         │                         │
│  └──────────────────┘     └──────────────────┘                         │
│                                                                           │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │              Web UI (React/Svelte + Vite)                          │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                        Config & Templates                                │
├─────────────────────────────────────────────────────────────────────────┤
│  config/                                                                 │
│  ├── default.toml                                                       │
│  └── device_templates/                                                  │
│      ├── maschine_mikro_mk3.toml                                       │
│      ├── launchpad_mini.toml                                           │
│      └── korg_nanokontrol.toml                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Workspace Structure

```
midimon/
├── Cargo.toml                       # Workspace root
│
├── midimon-core/                    # Core engine (library crate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                   # Public API
│       ├── devices.rs               # Device abstraction (MIDI/HID)
│       ├── events.rs                # Event normalization
│       ├── state.rs                 # State machine (timers)
│       ├── mapping.rs               # Mapping engine
│       ├── actions.rs               # Action executor
│       ├── config.rs                # Config loading & watching
│       └── feedback.rs              # LED feedback
│
├── midimon-daemon/                  # Background service
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Daemon entry point
│       ├── menu_bar.rs              # macOS menu bar
│       ├── app_detection.rs         # Frontmost app detection
│       └── launch_agent.rs          # Auto-start installer
│
├── midimon-gui/                     # Visual configuration UI
│   ├── Cargo.toml                   # Tauri backend
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   ├── tauri.conf.json          # Tauri config
│   │   └── src/
│   │       ├── main.rs              # Tauri app
│   │       └── commands.rs          # Tauri commands
│   └── ui/                          # Web UI
│       ├── package.json
│       ├── vite.config.js
│       ├── index.html
│       └── src/
│           ├── App.tsx              # Main React/Svelte app
│           ├── components/
│           │   ├── DeviceView.tsx
│           │   ├── ActionLibrary.tsx
│           │   └── ProfileManager.tsx
│           └── api/
│               └── tauri.ts         # Tauri API wrapper
│
├── config/                          # Device templates
│   ├── default.toml
│   └── device_templates/
│       ├── maschine_mikro_mk3.toml
│       ├── launchpad_mini.toml
│       └── korg_nanokontrol.toml
│
├── docs/                            # Documentation
│   ├── PRD-main.md                  # Product requirements
│   ├── personas.md                  # User personas
│   ├── features.md                  # Feature specs
│   └── architecture.md              # This document
│
├── .research/                       # Implementation proposals
│   ├── implementation-viewpoint-1.md
│   └── implementation-viewpoint-2.md
│
├── CLAUDE.md                        # Development guide
└── README.md                        # User guide
```

### Component Responsibilities (v2.0)

#### midimon-core (library)

**Purpose**: Pure Rust engine with zero UI dependencies. Reusable in other projects.

**Public API:**
```rust
// lib.rs
pub mod devices;
pub mod events;
pub mod mapping;
pub mod actions;
pub mod config;

pub struct Engine {
    devices: DeviceManager,
    event_processor: EventProcessor,
    mapping_engine: MappingEngine,
    action_executor: ActionExecutor,
    config: Config,
}

impl Engine {
    pub fn new(config: Config) -> Result<Self>;
    pub fn start(&mut self) -> Result<()>;
    pub fn reload_config(&mut self) -> Result<()>;
    pub fn switch_profile(&mut self, profile: &str) -> Result<()>;
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn get_status(&self) -> EngineStatus;
}
```

**Key Features:**
- Device abstraction (MIDI + HID unified)
- Event normalization (InputEvent type)
- Hot config reload
- Profile switching
- Thread-safe (Arc + RwLock)

#### midimon-daemon (binary)

**Purpose**: Background service with menu bar integration. Runs at startup.

**Responsibilities:**
- Initialize midimon-core engine
- Create menu bar app (NSStatusItem)
- Handle menu actions (pause, reload, configure)
- Watch for frontmost app changes
- Install/uninstall LaunchAgent
- IPC with GUI (Tauri commands)

**Menu Bar API:**
```rust
// menu_bar.rs
pub struct MenuBarApp {
    engine: Arc<Mutex<Engine>>,
    status_item: NSStatusItem,
}

impl MenuBarApp {
    pub fn new(engine: Engine) -> Self;
    pub fn update_status(&mut self, status: &str);
    pub fn set_menu_items(&mut self, items: Vec<MenuItem>);
}

pub enum MenuAction {
    Pause,
    Resume,
    ReloadConfig,
    OpenConfiguration,
    Quit,
}
```

**Frontmost App Detection:**
```rust
// app_detection.rs
pub struct AppDetector {
    last_bundle_id: Option<String>,
    profile_matcher: ProfileMatcher,
}

impl AppDetector {
    pub fn poll(&mut self) -> Option<String> {
        // Returns bundle_id if changed
        get_frontmost_app_bundle_id()
    }

    pub fn match_profile(&self, bundle_id: &str) -> &str {
        self.profile_matcher.match_profile(bundle_id)
    }
}
```

#### midimon-gui (Tauri app)

**Purpose**: Visual configuration interface. Launched on-demand from menu bar.

**Tauri Commands:**
```rust
// commands.rs
#[tauri::command]
fn get_config() -> Result<Config, String>;

#[tauri::command]
fn save_config(config: Config) -> Result<(), String>;

#[tauri::command]
fn get_devices() -> Result<Vec<Device>, String>;

#[tauri::command]
fn start_midi_learn(pad_index: usize) -> Result<(), String>;

#[tauri::command]
fn get_profiles() -> Result<Vec<Profile>, String>;

#[tauri::command]
fn reload_daemon_config() -> Result<(), String>;
```

**UI Components:**
```typescript
// App.tsx
<DeviceView
  device={selectedDevice}
  pads={pads}
  onPadClick={handlePadClick}
  onPadDrop={handleActionDrop}
/>

<ActionLibrary
  categories={actionCategories}
  onDragStart={handleDragStart}
/>

<ProfileManager
  profiles={profiles}
  activeProfile={activeProfile}
  onSwitch={switchProfile}
  onCreate={createProfile}
/>

<MIDILearnDialog
  isOpen={isLearning}
  padIndex={learningPad}
  onCapture={handleMIDICapture}
/>
```

### Inter-Component Communication

#### Daemon ↔ Engine
```rust
// Direct API calls (same process)
engine.reload_config()?;
engine.switch_profile("Logic Pro")?;
engine.pause();
```

#### GUI ↔ Daemon
```rust
// Tauri IPC (invoke commands)
invoke('reload_daemon_config');
invoke('get_config');
invoke('save_config', { config });
```

#### Config Watcher → Engine
```rust
// File watcher events
use notify::Watcher;

let (tx, rx) = channel();
let mut watcher = watcher(tx, Duration::from_secs(1))?;
watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

// In daemon loop:
if let Ok(DebouncedEvent::Write(_)) = rx.try_recv() {
    engine.reload_config()?;
}
```

---

## Data Flow

### Event Processing Flow (Detailed)

```
┌──────────────────────────────────────────────────────────────────────┐
│                        MIDI Hardware Input                            │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Raw MIDI bytes (3 bytes)
                                    │ [Status, Data1, Data2]
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│                   MIDI Callback (midir)                               │
│  • Parse status byte (0x90 = Note On, 0xB0 = CC, etc.)              │
│  • Extract channel, note/CC, velocity/value                          │
│  • Timestamp with Instant::now()                                     │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ MidiEvent enum
                                    │ { NoteOn{note, velocity, time} }
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│               Crossbeam Channel (bounded 100)                         │
│  • Lock-free message passing                                         │
│  • Decouples MIDI callback from processing thread                    │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ MidiEvent
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│                    EventProcessor::process()                          │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  1. Velocity Detection                                          │ │
│  │     • Soft (0-40), Medium (41-80), Hard (81-127)               │ │
│  │                                                                  │ │
│  │  2. Double-Tap Detection                                        │ │
│  │     • Check last_note_tap HashMap                              │ │
│  │     • If < 300ms since last tap → DoubleTap event             │ │
│  │                                                                  │ │
│  │  3. Chord Detection                                             │ │
│  │     • Add note to chord_buffer                                  │ │
│  │     • Clean notes >100ms old                                    │ │
│  │     • If buffer.len() >= 2 → ChordDetected event               │ │
│  │                                                                  │ │
│  │  4. Long Press Tracking                                         │ │
│  │     • Store press time in held_notes HashMap                   │ │
│  │     • Periodic check (200ms) for held notes >2000ms            │ │
│  │                                                                  │ │
│  │  5. Encoder Direction                                           │ │
│  │     • Compare current CC value to last_cc_values               │ │
│  │     • Determine Clockwise or CounterClockwise                  │ │
│  └────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Vec<ProcessedEvent>
                                    │ { PadPressed{note, velocity, level},
                                    │   DoubleTap{note},
                                    │   ChordDetected{notes}, ... }
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│            MappingEngine::get_action_for_processed()                  │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  1. Check Mode-Specific Mappings                               │ │
│  │     • Lookup mode_mappings[current_mode]                        │ │
│  │     • Iterate through mappings                                  │ │
│  │     • Call trigger_matches_processed() for each                │ │
│  │                                                                  │ │
│  │  2. Check Global Mappings                                       │ │
│  │     • If no mode match, check global_mappings                  │ │
│  │     • Global mappings work across all modes                    │ │
│  │                                                                  │ │
│  │  3. Trigger Matching Logic                                      │ │
│  │     • Note trigger: match note number                          │ │
│  │     • VelocityRange: match note + velocity range               │ │
│  │     • LongPress: match note + duration threshold               │ │
│  │     • Chord: all notes present in event                        │ │
│  └────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Option<Action>
                                    │ { Keystroke{keys, modifiers},
                                    │   Shell{command}, ... }
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│                 ActionExecutor::execute()                             │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  Match on action type:                                          │ │
│  │                                                                  │ │
│  │  • Keystroke → enigo.key_sequence()                            │ │
│  │  • Text → enigo.text()                                         │ │
│  │  • Shell → Command::new().spawn()                              │ │
│  │  • Launch → open -a {bundle_id}                                │ │
│  │  • VolumeControl → osascript (AppleScript)                     │ │
│  │  • Sequence → execute actions sequentially with delays         │ │
│  │  • ModeChange → update current_mode AtomicU8                   │ │
│  └────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ Side Effects
                                    ▼
┌──────────────────────────────────────────────────────────────────────┐
│                       System Actions                                  │
│  • Keyboard events (via Accessibility API)                           │
│  • Mouse clicks (via Accessibility API)                              │
│  • Process spawning (shell commands)                                 │
│  • Application launching (Launch Services)                           │
│  • Volume control (CoreAudio + AppleScript)                          │
└──────────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────────┐
│                      Parallel: LED Feedback                           │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  On PadPressed event:                                           │ │
│  │    1. Map note to pad index (via device profile)               │ │
│  │    2. Calculate color from velocity                            │ │
│  │       • Soft (0-40) → Green                                     │ │
│  │       • Medium (41-80) → Yellow                                 │ │
│  │       • Hard (81-127) → Red                                     │ │
│  │    3. Send HID report or MIDI Note On                          │ │
│  │                                                                  │ │
│  │  On PadReleased event:                                          │ │
│  │    1. Schedule fade-out after 1000ms                           │ │
│  │    2. Set LED to OFF after delay                               │ │
│  │                                                                  │ │
│  │  On ModeChange:                                                 │ │
│  │    1. Update all pad colors to mode theme                      │ │
│  │       • Mode 0 → Blue                                           │ │
│  │       • Mode 1 → Green                                          │ │
│  │       • Mode 2 → Purple                                         │ │
│  └────────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────────┘
```

### Configuration Flow

```
┌──────────────────────────────────────────────────────────────────────┐
│                    config.toml (User edits)                           │
└──────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    │                               │
                    ▼                               ▼
         ┌──────────────────────┐      ┌──────────────────────┐
         │  Config::load()      │      │  notify::Watcher     │
         │  (TOML parse)        │      │  (file changes)      │
         └──────────────────────┘      └──────────────────────┘
                    │                               │
                    │                               │ DebouncedEvent::Write
                    ▼                               ▼
         ┌────────────────────────────────────────────────────┐
         │           Config struct (validated)                │
         │  • device: DeviceConfig                           │
         │  • modes: Vec<Mode>                               │
         │  • global_mappings: Vec<Mapping>                  │
         │  • advanced_settings: AdvancedSettings            │
         └────────────────────────────────────────────────────┘
                                    │
                                    ▼
         ┌────────────────────────────────────────────────────┐
         │   MappingEngine::load_from_config()               │
         │   • Compile triggers (Config → CompiledTrigger)   │
         │   • Compile actions (Config → Action)             │
         │   • Build HashMaps for fast lookup                │
         └────────────────────────────────────────────────────┘
                                    │
                                    ▼
         ┌────────────────────────────────────────────────────┐
         │         Runtime MappingEngine                      │
         │  mode_mappings: HashMap<u8, Vec<CompiledMapping>> │
         │  global_mappings: Vec<CompiledMapping>            │
         └────────────────────────────────────────────────────┘
```

---

## API Specifications

### midimon-core Public API (v2.0)

#### Engine API

```rust
/// Main engine interface
pub struct Engine {
    // Private fields
}

impl Engine {
    /// Create new engine instance
    pub fn new(config: Config) -> Result<Self, Error>;

    /// Start the engine (blocking)
    pub fn start(&mut self) -> Result<(), Error>;

    /// Stop the engine gracefully
    pub fn stop(&mut self) -> Result<(), Error>;

    /// Reload configuration from disk
    pub fn reload_config(&mut self) -> Result<(), Error>;

    /// Switch to a different profile
    pub fn switch_profile(&mut self, profile_name: &str) -> Result<(), Error>;

    /// Pause event processing (input still received, not processed)
    pub fn pause(&mut self);

    /// Resume event processing
    pub fn resume(&mut self);

    /// Check if engine is paused
    pub fn is_paused(&self) -> bool;

    /// Get current engine status
    pub fn get_status(&self) -> EngineStatus;

    /// Get list of connected devices
    pub fn get_devices(&self) -> Vec<DeviceInfo>;

    /// Enable MIDI Learn mode
    pub fn start_midi_learn(&mut self) -> Result<(), Error>;

    /// Stop MIDI Learn mode
    pub fn stop_midi_learn(&mut self) -> Result<(), Error>;

    /// Get last captured MIDI event (for MIDI Learn)
    pub fn get_last_midi_event(&self) -> Option<MidiEvent>;
}

/// Engine status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineStatus {
    pub running: bool,
    pub paused: bool,
    pub current_profile: String,
    pub current_mode: u8,
    pub connected_devices: Vec<String>,
    pub events_processed: u64,
    pub actions_executed: u64,
    pub uptime: Duration,
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    MIDI,
    HID,
}
```

#### Event API

```rust
/// Normalized input event (MIDI + HID unified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub device_id: String,
    pub element_id: String,
    pub action: ElementAction,
    pub value: i32,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ElementAction {
    Press,
    Release,
    Rotate,
    Value,
}

/// Processed event (after pattern detection)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessedEvent {
    ShortPress { note: u8 },
    MediumPress { note: u8, duration_ms: u128 },
    LongPress { note: u8, duration_ms: u128 },
    PadPressed { note: u8, velocity: u8, velocity_level: VelocityLevel },
    PadReleased { note: u8, hold_duration_ms: u128 },
    EncoderTurned { cc: u8, value: u8, direction: EncoderDirection, delta: u8 },
    DoubleTap { note: u8 },
    ChordDetected { notes: Vec<u8> },
    AftertouchChanged { pressure: u8 },
    PitchBendMoved { value: u16 },
}
```

#### Config API

```rust
/// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub profiles: Vec<Profile>,
    pub active_profile: String,
    pub advanced_settings: AdvancedSettings,
}

impl Config {
    /// Load config from default location
    pub fn load() -> Result<Self, Error>;

    /// Load config from specific path
    pub fn load_from(path: &Path) -> Result<Self, Error>;

    /// Save config to default location
    pub fn save(&self) -> Result<(), Error>;

    /// Save config to specific path
    pub fn save_to(&self, path: &Path) -> Result<(), Error>;

    /// Validate config (check for errors)
    pub fn validate(&self) -> Result<(), ValidationError>;

    /// Get default config path
    pub fn default_path() -> PathBuf;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub app_profiles: HashMap<String, String>,
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub color: Option<String>,
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub description: Option<String>,
    pub trigger: Trigger,
    pub action: ActionConfig,
}
```

### Tauri Commands API (GUI)

```rust
// src-tauri/src/commands.rs

/// Get current configuration
#[tauri::command]
pub fn get_config() -> Result<Config, String> {
    Config::load().map_err(|e| e.to_string())
}

/// Save configuration
#[tauri::command]
pub fn save_config(config: Config) -> Result<(), String> {
    config.validate().map_err(|e| e.to_string())?;
    config.save().map_err(|e| e.to_string())
}

/// Get connected devices
#[tauri::command]
pub fn get_devices() -> Result<Vec<DeviceInfo>, String> {
    Ok(ENGINE.get().unwrap().get_devices())
}

/// Start MIDI Learn for specific pad
#[tauri::command]
pub fn start_midi_learn(pad_index: usize) -> Result<(), String> {
    ENGINE.get().unwrap().start_midi_learn()
        .map_err(|e| e.to_string())
}

/// Stop MIDI Learn
#[tauri::command]
pub fn stop_midi_learn() -> Result<(), String> {
    ENGINE.get().unwrap().stop_midi_learn()
        .map_err(|e| e.to_string())
}

/// Get last captured MIDI event
#[tauri::command]
pub fn get_last_midi_event() -> Option<MidiEvent> {
    ENGINE.get().unwrap().get_last_midi_event()
}

/// Reload daemon configuration
#[tauri::command]
pub fn reload_daemon_config() -> Result<(), String> {
    ENGINE.get().unwrap().reload_config()
        .map_err(|e| e.to_string())
}

/// Get engine status
#[tauri::command]
pub fn get_status() -> EngineStatus {
    ENGINE.get().unwrap().get_status()
}

/// Pause/resume engine
#[tauri::command]
pub fn set_paused(paused: bool) {
    if paused {
        ENGINE.get().unwrap().pause();
    } else {
        ENGINE.get().unwrap().resume();
    }
}

/// Switch profile
#[tauri::command]
pub fn switch_profile(profile_name: String) -> Result<(), String> {
    ENGINE.get().unwrap().switch_profile(&profile_name)
        .map_err(|e| e.to_string())
}

/// Get available profiles
#[tauri::command]
pub fn get_profiles() -> Result<Vec<Profile>, String> {
    let config = Config::load().map_err(|e| e.to_string())?;
    Ok(config.profiles)
}

/// Create new profile
#[tauri::command]
pub fn create_profile(name: String) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;

    let profile = Profile {
        name: name.clone(),
        app_profiles: HashMap::new(),
        modes: vec![Mode {
            name: "Default".to_string(),
            color: Some("blue".to_string()),
            mappings: vec![],
        }],
        global_mappings: vec![],
    };

    config.profiles.push(profile);
    config.save().map_err(|e| e.to_string())
}

/// Delete profile
#[tauri::command]
pub fn delete_profile(name: String) -> Result<(), String> {
    let mut config = Config::load().map_err(|e| e.to_string())?;
    config.profiles.retain(|p| p.name != name);
    config.save().map_err(|e| e.to_string())
}
```

### IPC Protocol (Daemon ↔ GUI)

```typescript
// ui/src/api/tauri.ts

import { invoke } from '@tauri-apps/api/tauri';

export interface Config {
  device: DeviceConfig;
  profiles: Profile[];
  active_profile: string;
  advanced_settings: AdvancedSettings;
}

export interface EngineStatus {
  running: boolean;
  paused: boolean;
  current_profile: string;
  current_mode: number;
  connected_devices: string[];
  events_processed: number;
  actions_executed: number;
  uptime: number;
}

// Configuration
export async function getConfig(): Promise<Config> {
  return await invoke('get_config');
}

export async function saveConfig(config: Config): Promise<void> {
  await invoke('save_config', { config });
}

// Devices
export async function getDevices(): Promise<DeviceInfo[]> {
  return await invoke('get_devices');
}

// MIDI Learn
export async function startMIDILearn(padIndex: number): Promise<void> {
  await invoke('start_midi_learn', { padIndex });
}

export async function stopMIDILearn(): Promise<void> {
  await invoke('stop_midi_learn');
}

export async function getLastMIDIEvent(): Promise<MidiEvent | null> {
  return await invoke('get_last_midi_event');
}

// Engine Control
export async function getStatus(): Promise<EngineStatus> {
  return await invoke('get_status');
}

export async function setPaused(paused: boolean): Promise<void> {
  await invoke('set_paused', { paused });
}

export async function reloadConfig(): Promise<void> {
  await invoke('reload_daemon_config');
}

// Profiles
export async function getProfiles(): Promise<Profile[]> {
  return await invoke('get_profiles');
}

export async function switchProfile(profileName: string): Promise<void> {
  await invoke('switch_profile', { profileName });
}

export async function createProfile(name: string): Promise<void> {
  await invoke('create_profile', { name });
}

export async function deleteProfile(name: string): Promise<void> {
  await invoke('delete_profile', { name });
}
```

---

## Performance Characteristics

### Latency Budget

```
Total Latency Target: <1ms (MIDI input → Action execution)

┌────────────────────────────────────────────────────────┐
│  MIDI Callback                             <50μs       │
│  ├─ Parse MIDI bytes                       10μs        │
│  └─ Create MidiEvent + send channel        40μs        │
├────────────────────────────────────────────────────────┤
│  Channel Transfer                          <100μs      │
│  └─ Crossbeam bounded channel              100μs       │
├────────────────────────────────────────────────────────┤
│  Event Processing                          <200μs      │
│  ├─ Velocity detection                     20μs        │
│  ├─ Double-tap check (HashMap lookup)     30μs        │
│  ├─ Chord detection (Vec operations)      50μs        │
│  └─ Create ProcessedEvent                  100μs       │
├────────────────────────────────────────────────────────┤
│  Mapping Engine                            <300μs      │
│  ├─ Mode mappings lookup (HashMap)        50μs        │
│  ├─ Iterate mappings                       100μs       │
│  ├─ Trigger matching                       100μs       │
│  └─ Clone Action                           50μs        │
├────────────────────────────────────────────────────────┤
│  Action Execution                          <350μs      │
│  ├─ Pattern match on action type          10μs        │
│  ├─ enigo keystroke (Accessibility API)   200μs       │
│  ├─ Shell command spawn                    100μs       │
│  └─ LED feedback (async)                   40μs        │
└────────────────────────────────────────────────────────┘

Total: ~1000μs (1ms) worst case
Typical: ~500μs (0.5ms)
Best case: ~300μs (0.3ms)
```

### Memory Profile

```
┌────────────────────────────────────────────────────────┐
│  Component                        Memory Usage          │
├────────────────────────────────────────────────────────┤
│  Binary Code                      3-5 MB (stripped)    │
│  Config (loaded)                  50-100 KB             │
│  EventProcessor state             10-20 KB              │
│  MappingEngine                    50-100 KB             │
│  LED buffers                      5 KB                  │
│  Crossbeam channel                10 KB                 │
│  midir connection                 50 KB                 │
│  hidapi connection                50 KB                 │
│  Stack (per thread)               2 MB × 3 threads      │
├────────────────────────────────────────────────────────┤
│  Total (idle)                     5-10 MB RSS           │
│  Total (active)                   10-15 MB RSS          │
│  Total (with GUI)                 20-30 MB RSS          │
└────────────────────────────────────────────────────────┘
```

### Throughput

- **MIDI Events**: 1000+ events/second sustained
- **LED Updates**: 10 FPS (100ms per frame) for animations
- **Config Reload**: <100ms (small configs), <500ms (large configs)
- **Profile Switch**: <50ms (no visible lag)

---

## Migration Plan

### Phase 1: Preserve & Document (Weeks 1-2)

**Goal**: Zero knowledge loss of current implementation

**Tasks**:
1. ✅ Create CLAUDE.md with architecture documentation
2. ✅ Create PRD (this document) with product vision
3. ✅ Create user personas document
4. ✅ Create feature specifications
5. ✅ Create technical architecture document
6. [ ] Tag current implementation as `v0.1.0-monolithic`
7. [ ] Run comprehensive testing (all features verified)
8. [ ] Record demo video of current functionality
9. [ ] Document known issues and limitations

**Success Criteria**:
- Git tag created and pushed
- All documentation complete
- All tests passing
- Demo video published

### Phase 2: Extract Core Engine (Weeks 3-6)

**Goal**: Create midimon-core as reusable library

**Tasks**:
1. [ ] Create workspace Cargo.toml
   ```toml
   [workspace]
   members = ["midimon-core", "midimon-daemon", "midimon-gui"]
   resolver = "2"
   ```

2. [ ] Create midimon-core crate structure
3. [ ] Move and refactor code:
   - `src/config.rs` → `midimon-core/src/config.rs`
   - `src/event_processor.rs` → `midimon-core/src/events.rs`
   - `src/mappings.rs` → `midimon-core/src/mapping.rs`
   - `src/actions.rs` → `midimon-core/src/actions.rs`
   - `src/feedback.rs` → `midimon-core/src/feedback.rs`

4. [ ] Create unified device abstraction
   ```rust
   // midimon-core/src/devices.rs
   pub trait Device {
       fn connect(&mut self) -> Result<()>;
       fn poll_events(&mut self) -> Vec<InputEvent>;
   }

   pub struct MIDIDevice { /* ... */ }
   pub struct HIDDevice { /* ... */ }
   ```

5. [ ] Create Engine API (see API Specifications above)
6. [ ] Add comprehensive unit tests (>80% coverage)
7. [ ] Document public API with rustdoc
8. [ ] Verify zero UI dependencies in core

**Success Criteria**:
- `midimon-core` compiles independently
- All existing features work via core API
- Test coverage >80%
- Documentation complete
- Zero breaking changes to config.toml format

### Phase 3: Build Daemon & UI (Weeks 7-12)

**Goal**: Add menu bar daemon and Tauri GUI

**Tasks**:
1. [ ] Create midimon-daemon crate
2. [ ] Implement menu bar (NSStatusItem/Tauri tray)
3. [ ] Add LaunchAgent installer (macOS auto-start)
4. [ ] Implement frontmost app detection
5. [ ] Create midimon-gui crate with Tauri v2
6. [ ] Build React/Svelte UI:
   - Device visualization component
   - Action library component
   - Profile manager
   - MIDI Learn dialog
7. [ ] Implement Tauri commands (see API Specifications)
8. [ ] Add hot config reload (notify crate)
9. [ ] Implement per-app profile switching
10. [ ] Create preferences UI

**Success Criteria**:
- Daemon starts on login
- Menu bar shows status and quick actions
- GUI provides visual configuration
- MIDI Learn works end-to-end
- Config hot-reloads without restart
- Performance targets maintained (<1ms latency, <15MB memory)

### Phase 4: Enhanced Features (Weeks 13-20)

**Goal**: Add v2.0 exclusive features

**Tasks**:
1. [ ] Device template system (10+ controllers)
2. [ ] Profile import/export
3. [ ] Virtual MIDI output (coremidi)
4. [ ] Advanced conditional logic
5. [ ] Live event console
6. [ ] Profile marketplace integration
7. [ ] Community device templates (GitHub repo)
8. [ ] Linux support (initial port)
9. [ ] Performance profiling and optimization
10. [ ] Beta testing program (50+ users)

**Success Criteria**:
- 10+ device templates available
- Profile sharing functional
- Virtual MIDI tested with major DAWs
- Community contributing templates
- Beta users providing feedback
- Linux build working (basic features)

---

## Security & Privacy

### Permissions (macOS)

**Required**:
- **Accessibility** (TCC): For keystroke simulation (enigo)
  - Prompt user on first keystroke action
  - Provide link to System Settings → Privacy & Security → Accessibility

**Optional**:
- **Input Monitoring** (TCC): For HID device access (hidapi)
  - Required for full RGB LED control
  - Falls back to MIDI LED if denied

**Not Required**:
- Network access (no telemetry)
- Microphone/camera
- Location
- Contacts/calendar

### Data Privacy

**Local Data**:
- All configuration stored locally (`~/Library/Application Support/MIDIMon/`)
- No cloud sync by default
- No telemetry or analytics
- No network connections required

**Optional Features** (opt-in):
- Anonymous usage statistics (never personally identifiable)
- Crash reporting (opt-in)
- Profile marketplace (GitHub-based, user-controlled)

### Code Security

**Build Security**:
- Developer ID signing (macOS)
- Notarization for Gatekeeper
- Reproducible builds
- Dependency scanning (cargo-audit)

**Runtime Security**:
- No eval() or dynamic code execution
- Input validation on all config parsing
- Sandboxing where possible (Tauri)
- Minimal attack surface (no network server)

---

## Platform Considerations

### macOS (Primary Platform)

**Supported Versions**:
- macOS 12 Monterey (minimum)
- macOS 13 Ventura (recommended)
- macOS 14 Sonoma (recommended)
- macOS 15 Sequoia (testing)

**Architecture**:
- Universal binary (Apple Silicon + Intel)
- Optimized for M1/M2/M3

**System Integration**:
- CoreMIDI for MIDI I/O
- IOKit/hidapi for HID devices
- Accessibility API for input simulation
- NSWorkspace for app detection
- LaunchAgent for auto-start

**Known Issues**:
- HID access requires Input Monitoring permission (macOS 14+)
- Some MIDI devices require driver installation
- Gatekeeper may block unsigned builds

### Linux (Future)

**Planned Support**:
- Ubuntu 22.04+ (primary)
- Arch Linux
- Fedora

**Challenges**:
- ALSA vs. JACK vs. PipeWire for MIDI
- udev rules for HID access
- X11 vs. Wayland for input simulation
- No native menu bar (systray alternatives)

### Windows (Lower Priority)

**Planned Support**:
- Windows 10/11

**Challenges**:
- WinMM vs. Windows MIDI Services
- Different HID driver model
- System tray implementation
- Keyboard simulation (SendInput)

---

## Appendices

### A. Performance Benchmarks

```bash
# Run benchmarks
cargo bench

# Profile CPU usage
cargo flamegraph --bin midimon

# Memory profiling
valgrind --tool=massif target/release/midimon

# Latency testing
cargo run --bin latency_test
```

### B. Development Tools

**Required**:
- Rust 1.70+ (stable channel)
- Xcode Command Line Tools (macOS)
- cargo
- rustc

**Optional**:
- cargo-watch (hot reload)
- cargo-audit (security)
- cargo-flamegraph (profiling)
- rust-analyzer (IDE support)

### C. Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test '*'

# End-to-end tests
cargo test --test e2e

# Coverage
cargo tarpaulin --out Html
```

### D. References

- **Current Codebase**: `/src/*.rs`
- **Research Proposals**: `.research/implementation-viewpoint-{1,2}.md`
- **MIDI Specification**: [MIDI 1.0 Detailed Specification](https://www.midi.org/specifications/midi1-specifications)
- **CoreMIDI**: [Apple CoreMIDI Documentation](https://developer.apple.com/documentation/coremidi)
- **hidapi**: [hidapi GitHub](https://github.com/libusb/hidapi)
- **Tauri**: [Tauri v2 Documentation](https://v2.tauri.app/)

---

**Document History**:
- v1.0 (2025-11-11): Initial architecture documentation
