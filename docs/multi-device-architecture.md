# Multi-Device MIDI Architecture

**Date**: 2025-01-18
**Status**: 🔵 **DESIGN PROPOSAL** (not yet implemented)
**Current Implementation**: Single device only

## Executive Summary

MIDIMon currently supports **one MIDI input device at a time**. This document outlines the architecture changes required to support **multiple simultaneous MIDI devices** with device-specific mappings and routing.

## Current Architecture (Single Device)

```
Config
  └── device: DeviceConfig (singular)

EngineManager
  └── midi_device: Option<MidiDeviceManager> (one slot)
      └── connection → mpsc::channel → EngineManager.run()

EventProcessor (device-agnostic)
  └── Processes all events from single stream

MappingEngine
  └── No device awareness
  └── Maps events to actions without device filtering
```

**Limitations:**
- ✗ Cannot use pad controller + keyboard controller simultaneously
- ✗ Cannot route different devices to different actions
- ✗ No device-specific velocity curves or mappings
- ✗ Hot-swapping devices requires config reload

## Proposed Architecture (Multi-Device)

### Option A: Unified Event Stream (Simpler)

```
Config
  └── devices: Vec<DeviceConfig> (multiple)

EngineManager
  ├── midi_devices: HashMap<String, MidiDeviceManager>
  └── shared event channel (all devices → one stream)

EventProcessor
  └── Events tagged with device_id
  └── Device-aware gesture detection (per-device chord buffers)

MappingEngine
  └── Device filter on triggers
  └── Route by device_id or "any"
```

**Pros:**
- Simple event loop (one select! branch for all devices)
- Easy to implement device-agnostic mappings
- Minimal changes to existing pipeline

**Cons:**
- Event ordering not guaranteed across devices
- Device isolation harder (chords across devices would trigger)
- Shared channel could bottleneck with many devices

### Option B: Per-Device Streams (More Isolated)

```
Config
  └── devices: Vec<DeviceConfig>

EngineManager
  ├── midi_devices: HashMap<String, MidiDeviceManager>
  └── per-device channels (device_id → mpsc::channel)

EventProcessor
  └── Per-device instances (isolated state)
  └── Device-specific gesture detection

MappingEngine
  └── Device-scoped mappings
  └── Device router: device_id → Vec<Action>
```

**Pros:**
- True device isolation (no cross-device chords)
- Per-device backpressure handling
- Independent reconnection per device

**Cons:**
- More complex event loop (select! with dynamic branches)
- Harder to implement "any device" mappings
- More memory (per-device EventProcessor instances)

## Recommended Approach: Hybrid (Option C)

**Best of both worlds:**

```rust
// Config
pub struct Config {
    pub devices: Vec<DeviceConfig>,  // Changed from singular
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<Mapping>,
    // ...
}

pub struct DeviceConfig {
    pub id: String,              // NEW: Unique identifier
    pub name: String,            // MIDI port name (for discovery)
    pub auto_connect: bool,
    pub auto_reconnect: bool,
    pub port: Option<usize>,
    pub enabled: bool,           // NEW: Allow disabling without removing
}

// Trigger with optional device filter
pub enum Trigger {
    Note {
        note: u8,
        device: Option<String>,  // NEW: "pad-controller", "keyboard", or None (any)
    },
    // ... other variants
}

// Tagged MIDI events
pub struct TaggedMidiEvent {
    pub device_id: String,       // Which device sent this
    pub event: MidiEvent,        // The actual MIDI event
}
```

**Architecture:**

```
┌────────────────────────────────────────────────┐
│  Config: devices = [                           │
│    { id: "pads", name: "Maschine Mikro" },     │
│    { id: "keys", name: "MIDI Keyboard" }       │
│  ]                                             │
└────────────────────────────────────────────────┘
                      │
                      ▼
┌────────────────────────────────────────────────┐
│  EngineManager                                 │
│  ┌──────────────────────────────────────────┐ │
│  │  midi_devices: HashMap<String, Manager> │ │
│  │  - "pads" → MidiDeviceManager            │ │
│  │  - "keys" → MidiDeviceManager            │ │
│  └──────────────────────────────────────────┘ │
│                                                │
│  ┌──────────────────────────────────────────┐ │
│  │  Shared event channel                    │ │
│  │  mpsc::channel<TaggedMidiEvent>          │ │
│  │  - All devices send to same channel      │ │
│  └──────────────────────────────────────────┘ │
│                                                │
│  ┌──────────────────────────────────────────┐ │
│  │  EventProcessor (per-device instances)   │ │
│  │  HashMap<String, EventProcessor>         │ │
│  │  - Isolated chord buffers                │ │
│  │  - Device-specific timing                │ │
│  └──────────────────────────────────────────┘ │
└────────────────────────────────────────────────┘
                      │
                      ▼
┌────────────────────────────────────────────────┐
│  MappingEngine                                 │
│  - Filter by device_id before matching        │
│  - Support "any" device wildcard              │
└────────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Config & Core Types (Breaking Change)

**Files:**
- `midimon-core/src/config/types.rs`
- `midimon-core/src/events.rs`

**Changes:**
```rust
// Before
pub struct Config {
    pub device: DeviceConfig,
    // ...
}

// After
pub struct Config {
    pub devices: Vec<DeviceConfig>,
    // ...
}

// New types
pub struct DeviceConfig {
    pub id: String,              // NEW
    pub name: String,
    pub auto_connect: bool,
    pub auto_reconnect: bool,
    pub port: Option<usize>,
    pub enabled: bool,           // NEW
}

pub struct TaggedMidiEvent {
    pub device_id: String,
    pub event: MidiEvent,
}
```

**Migration:**
```toml
# Old config.toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true

# New config.toml
[[devices]]
id = "main"
name = "Maschine Mikro MK3"
auto_connect = true
enabled = true
```

### Phase 2: EventProcessor Per-Device

**Files:**
- `midimon-daemon/src/daemon/engine_manager.rs`

**Changes:**
```rust
pub struct EngineManager {
    // Before
    event_processor: Arc<RwLock<EventProcessor>>,

    // After
    event_processors: Arc<RwLock<HashMap<String, EventProcessor>>>,
}

impl EngineManager {
    async fn process_midi_event(&mut self, tagged_event: TaggedMidiEvent) -> Result<()> {
        let device_id = &tagged_event.device_id;

        // Get device-specific processor (or create if first event)
        let processed_events = {
            let mut processors = self.event_processors.write().await;
            let processor = processors
                .entry(device_id.clone())
                .or_insert_with(EventProcessor::new);
            processor.process(tagged_event.event.clone())
        };

        // Rest of pipeline...
    }
}
```

### Phase 3: Multi-Device Manager

**Files:**
- `midimon-daemon/src/daemon/engine_manager.rs`

**Changes:**
```rust
pub struct EngineManager {
    // Before
    midi_device: Arc<Mutex<Option<MidiDeviceManager>>>,

    // After
    midi_devices: Arc<Mutex<HashMap<String, MidiDeviceManager>>>,
}

impl EngineManager {
    async fn connect_midi_devices(&mut self) -> Result<()> {
        let config = self.config.read().await;

        for device_config in &config.devices {
            if !device_config.enabled {
                continue;
            }

            let device_id = device_config.id.clone();
            info!("Connecting to device: {}", device_id);

            let mut manager = MidiDeviceManager::new(
                device_config.name.clone(),
                device_config.auto_reconnect,
            );

            // Clone device_id for callback
            let device_id_clone = device_id.clone();
            let event_tx = self.midi_event_tx.clone();

            // Create wrapper that tags events
            let (tagged_tx, tagged_rx) = mpsc::channel::<TaggedMidiEvent>(100);

            // Spawn task to tag events from this device
            let device_id_task = device_id.clone();
            tokio::spawn(async move {
                // Intercept events, tag them, forward to main channel
                // (Alternative: modify MidiDeviceManager callback to accept device_id)
            });

            manager.connect(event_tx, self.command_tx.clone())?;

            self.midi_devices.lock().await.insert(device_id, manager);
        }

        Ok(())
    }
}
```

### Phase 4: Device-Aware Triggers

**Files:**
- `midimon-core/src/config/types.rs`
- `midimon-core/src/mapping.rs`

**Changes:**
```rust
// Trigger with device filter
pub enum Trigger {
    Note {
        note: u8,
        velocity_range: Option<(u8, u8)>,
        device: Option<String>,  // NEW: None = any device
    },
    // ... other variants with device field
}

// MappingEngine filtering
impl MappingEngine {
    pub fn get_action(
        &self,
        tagged_event: &TaggedMidiEvent,
        mode: u8,
    ) -> Option<Action> {
        for mapping in &self.mappings {
            if self.trigger_matches(&mapping.trigger, tagged_event, mode) {
                return Some(mapping.action.clone());
            }
        }
        None
    }

    fn trigger_matches(
        &self,
        trigger: &Trigger,
        tagged_event: &TaggedMidiEvent,
        mode: u8,
    ) -> bool {
        match trigger {
            Trigger::Note { note, device, .. } => {
                // Check device filter
                if let Some(device_filter) = device {
                    if *device_filter != tagged_event.device_id {
                        return false; // Device mismatch
                    }
                }

                // Check note match
                match &tagged_event.event {
                    MidiEvent::NoteOn { note: n, .. } if n == note => true,
                    _ => false,
                }
            }
            // ... other trigger types
        }
    }
}
```

**Config Example:**
```toml
[[devices]]
id = "pads"
name = "Maschine Mikro MK3"

[[devices]]
id = "keyboard"
name = "MIDI Keyboard"

[[modes]]
name = "Production"

[[modes.mappings]]
trigger = { type = "Note", note = 36, device = "pads" }
action = { type = "Keystroke", key = "Space" }

[[modes.mappings]]
trigger = { type = "Note", note = 60, device = "keyboard" }
action = { type = "Keystroke", key = "A" }

[[modes.mappings]]
trigger = { type = "Note", note = 48 }  # No device = any device
action = { type = "Keystroke", key = "B" }
```

### Phase 5: IPC Commands

**New IPC Commands:**
```rust
pub enum IpcCommand {
    // Existing
    ListDevices,      // List available MIDI ports

    // New
    ListConnected,    // List currently connected devices
    ConnectDevice { id: String },      // Connect specific device by ID
    DisconnectDevice { id: String },   // Disconnect specific device
    EnableDevice { id: String },       // Enable in config
    DisableDevice { id: String },      // Disable in config
}
```

## Use Cases

### Use Case 1: Pad Controller + Keyboard

**Devices:**
- Maschine Mikro MK3 (pads, encoders)
- MIDI Keyboard (notes, mod wheel)

**Mappings:**
- Pads → Launch clips (Ableton)
- Keyboard notes → Play instruments
- Pads + Keyboard same note → Different actions

**Config:**
```toml
[[devices]]
id = "pads"
name = "Maschine Mikro MK3"
auto_connect = true

[[devices]]
id = "keys"
name = "Keystation"
auto_connect = true

[[modes.mappings]]
trigger = { type = "Note", note = 36, device = "pads" }
action = { type = "Keystroke", key = "F1" }  # Launch Clip 1

[[modes.mappings]]
trigger = { type = "Note", note = 36, device = "keys" }
action = { type = "Keystroke", key = "A" }   # Play Note A
```

### Use Case 2: Multi-Device Chaining

**Scenario:** Use encoders from one device, pads from another

**Devices:**
- Device A: Has encoders
- Device B: Has pads

**Mappings:**
```toml
[[modes.mappings]]
trigger = { type = "EncoderTurn", encoder = 1, device = "device-a" }
action = { type = "VolumeControl", direction = "Up" }

[[modes.mappings]]
trigger = { type = "Note", note = 60, device = "device-b" }
action = { type = "Keystroke", key = "Space" }
```

### Use Case 3: Device Failover

**Scenario:** Use backup device if primary fails

**Config:**
```toml
[[devices]]
id = "primary"
name = "Maschine Mikro MK3"
auto_connect = true
auto_reconnect = true

[[devices]]
id = "backup"
name = "Launchpad Mini"
auto_connect = true
enabled = false  # Only enable if primary fails

# Same note mappings for both devices
[[modes.mappings]]
trigger = { type = "Note", note = 36 }  # No device filter = any
action = { type = "Keystroke", key = "Space" }
```

**Runtime:**
1. Primary device active → processes events
2. Primary disconnects → EngineManager detects
3. IPC command: `enable-device backup`
4. Backup device connects → processes same mappings

## Performance Considerations

### Memory Impact

**Single Device:**
- 1 EventProcessor instance: ~4KB
- 1 MIDI channel: ~8KB
- Total: ~12KB

**Multi-Device (5 devices):**
- 5 EventProcessor instances: ~20KB
- 1 shared MIDI channel: ~8KB
- 5 MidiDeviceManager instances: ~10KB
- HashMap overhead: ~2KB
- **Total: ~40KB** (3.3x increase)

**Verdict:** Negligible impact (40KB in 5-10MB resident memory)

### Latency Impact

**Event Processing:**
- Device tagging: +0.01ms (HashMap lookup)
- Per-device EventProcessor: +0.1ms (isolated state)
- Device filtering in MappingEngine: +0.05ms
- **Total overhead: ~0.16ms** (well within 10ms budget)

**Worst Case (5 devices, all sending events simultaneously):**
- Shared channel bottleneck: 5 * 1ms = 5ms peak
- Still within 10ms target ✅

### Reconnection Handling

**Current (single device):**
- Disconnect → spawn reconnection thread
- Thread sends DaemonCommand::DeviceReconnected

**Multi-Device:**
- Disconnect → spawn reconnection thread per device
- Thread sends DaemonCommand::DeviceReconnected { device_id }
- EngineManager reconnects specific device (others unaffected)

**Isolation:** Device A failure doesn't block Device B events ✅

## Migration Path

### Backward Compatibility

**Option 1: Accept both formats** (Recommended)

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    // Accept singular device for backward compat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceConfig>,

    // New multi-device format
    #[serde(default)]
    pub devices: Vec<DeviceConfig>,

    // ... rest
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let mut config: Config = /* parse TOML */;

        // Migrate singular device to devices array
        if let Some(device) = config.device.take() {
            if config.devices.is_empty() {
                config.devices.push(DeviceConfig {
                    id: "main".to_string(),
                    name: device.name,
                    auto_connect: device.auto_connect,
                    auto_reconnect: device.auto_reconnect,
                    port: device.port,
                    enabled: true,
                });
            }
        }

        Ok(config)
    }
}
```

**Old configs work unchanged:**
```toml
[device]
name = "Maschine Mikro MK3"
auto_connect = true
```

**Automatically migrated to:**
```toml
[[devices]]
id = "main"
name = "Maschine Mikro MK3"
auto_connect = true
enabled = true
```

### Breaking Changes (if any)

**API Changes:**
```rust
// EngineManager::new() signature unchanged
// connect_midi_device() → connect_midi_devices() (internal only)
```

**Config Changes:**
- Recommend migration to `[[devices]]` format
- Old `[device]` format deprecated but supported

## Testing Strategy

### Unit Tests

**New Tests:**
1. Config parsing with `[[devices]]` array
2. Backward compat test (singular `[device]`)
3. TaggedMidiEvent serialization
4. Device filter matching in MappingEngine
5. Per-device EventProcessor isolation

**Files:**
- `midimon-core/tests/multi_device_config_test.rs`
- `midimon-core/tests/device_filter_test.rs`
- `midimon-daemon/tests/multi_device_integration_test.rs`

### Integration Tests

**Scenarios:**
1. **Two devices, different mappings**
   - Device A note 36 → Action X
   - Device B note 36 → Action Y
   - Verify correct routing

2. **Device failover**
   - Connect primary device
   - Disconnect primary
   - Reconnect primary
   - Verify seamless transition

3. **Concurrent events**
   - Both devices send events simultaneously
   - Verify no events dropped
   - Verify correct action execution

4. **IPC device management**
   - `list-connected` → returns all active devices
   - `disconnect-device pads` → disconnects only pads
   - `connect-device pads` → reconnects pads

## Future Enhancements

### MIDI Merge/Split

**Merge:** Combine multiple devices into virtual device
```toml
[[virtual_devices]]
id = "merged"
sources = ["device-a", "device-b"]
merge_strategy = "union"  # All events from both devices
```

**Split:** Route one device to multiple virtual devices
```toml
[[virtual_devices]]
id = "pads-only"
source = "maschine"
filter = { type = "Note", note_range = [36, 52] }

[[virtual_devices]]
id = "encoders-only"
source = "maschine"
filter = { type = "ControlChange" }
```

### Device Groups

**Group devices for simultaneous mappings:**
```toml
[[device_groups]]
id = "controllers"
devices = ["maschine", "launchpad"]

[[modes.mappings]]
trigger = { type = "Note", note = 36, device_group = "controllers" }
action = { type = "Keystroke", key = "Space" }
```

### MIDI Routing Matrix

**GUI for visual device routing:**
```
Device A → [Filter] → [Transform] → Action Set 1
         ↘ [Filter] → [Transform] → Action Set 2

Device B → [Merge with A] → Action Set 3
```

## Conclusion

**Current Status:** ❌ Single device only

**Multi-Device Support:**
- ✅ Architecturally feasible
- ✅ Minimal performance impact
- ✅ Backward compatible (with migration)
- ⏳ Requires ~1-2 weeks implementation

**Recommended Next Steps:**
1. Implement Phase 1 (Config changes with backward compat)
2. Implement Phase 2 (Per-device EventProcessor)
3. Implement Phase 3 (Multi-device manager)
4. Test with 2 devices before expanding
5. Add IPC commands for runtime management

**Estimated Effort:**
- Phase 1-2: 2-3 days
- Phase 3-4: 3-4 days
- Phase 5 + Testing: 2-3 days
- **Total: 7-10 days**
