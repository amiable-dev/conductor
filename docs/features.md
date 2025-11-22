# MIDIMon Feature Specifications
## Detailed Technical Specifications for Current and Future Features

**Version**: 1.3
**Last Updated**: 2025-11-11
**Purpose**: Comprehensive feature documentation for development and planning

---

## Document Structure

This document provides detailed specifications for:
1. **Current Features** (v0.1.0 - Monolithic) - Fully implemented and tested
2. **Target Features** (v2.0 - Modular) - Planned for migration
3. **Future Features** (v2.5+) - Roadmap items

Each feature includes:
- **Description**: What the feature does
- **User Story**: Why users need it
- **Technical Implementation**: How it works
- **API/Config Format**: Configuration and API examples
- **Edge Cases**: Known limitations and corner cases
- **Testing Criteria**: How to verify it works

---

## Current Features (v0.1.0)

### F1: Event Processing Pipeline

#### Description
Three-stage architecture that transforms raw MIDI bytes into executed actions with pattern detection and state management.

#### User Story
> As a user, I want my MIDI input to be intelligently processed so that I can use advanced triggers like long press and chords without manual timing logic.

#### Technical Implementation

**Stage 1: MIDI Input (main.rs:124-169)**
```rust
// Raw MIDI bytes → MidiEvent enum
let event = match message[0] & 0xF0 {
    0x90 if message[2] > 0 => MidiEvent::NoteOn {
        note: message[1],
        velocity: message[2],
        time: Instant::now(),
    },
    0x80 | 0x90 => MidiEvent::NoteOff {
        note: message[1],
        time: Instant::now(),
    },
    0xB0 => MidiEvent::ControlChange {
        cc: message[1],
        value: message[2],
        time: Instant::now(),
    },
    // ... other MIDI message types
};
```

**Stage 2: Event Processing (event_processor.rs)**
```rust
// MidiEvent → ProcessedEvent (detects patterns)
pub fn process(&mut self, event: MidiEvent) -> Vec<ProcessedEvent> {
    match event {
        MidiEvent::NoteOn { note, velocity, time } => {
            self.note_press_times.insert(note, time);

            // Check double-tap
            if let Some(&last_tap) = self.last_note_tap.get(&note) {
                if time.duration_since(last_tap) < self.double_tap_timeout {
                    results.push(ProcessedEvent::DoubleTap { note });
                }
            }

            // Detect velocity level
            let velocity_level = match velocity {
                0..=40 => VelocityLevel::Soft,
                41..=80 => VelocityLevel::Medium,
                81..=127 => VelocityLevel::Hard,
            };

            results.push(ProcessedEvent::PadPressed {
                note, velocity, velocity_level
            });
        }
        // ... other event types
    }
}
```

**Stage 3: Mapping & Execution (mappings.rs, actions.rs)**
```rust
// ProcessedEvent → Action → execution
if let Some(action) = mapping_engine.get_action_for_processed(&event, mode) {
    executor.execute(action);
}
```

#### Configuration Example
```toml
# No direct configuration - this is core architecture
# Configured indirectly through trigger/action mappings
```

#### Edge Cases
- **Simultaneous Events**: Chord buffer handles near-simultaneous presses
- **Timing Precision**: Uses `Instant::now()` for sub-millisecond accuracy
- **State Corruption**: HashMap-based state management prevents conflicts

#### Testing Criteria
- ✅ MIDI note on/off correctly parsed
- ✅ Velocity levels accurately detected (soft/medium/hard boundaries)
- ✅ Long press detected after threshold (default 2000ms)
- ✅ Double-tap detected within window (default 300ms)
- ✅ Chord detection for simultaneous presses (within 100ms)
- ✅ Latency <1ms from MIDI input to action execution

---

### F2: Velocity Sensitivity

#### Description
Three-tier velocity detection (Soft, Medium, Hard) enables different actions based on how hard a pad is pressed.

#### User Story
> As a music producer (Alex), I want to trigger different actions based on how hard I press a pad, so I can have play/pause on soft press and stop on hard press.

#### Technical Implementation

**Velocity Ranges:**
- **Soft**: 0-40 (MIDI velocity)
- **Medium**: 41-80
- **Hard**: 81-127

**Detection Logic (event_processor.rs:89-95):**
```rust
let velocity_level = match velocity {
    0..=40 => VelocityLevel::Soft,
    41..=80 => VelocityLevel::Medium,
    81..=127 => VelocityLevel::Hard,
    _ => VelocityLevel::Medium, // Fallback
};
```

**Trigger Matching (mappings.rs):**
```rust
CompiledTrigger::VelocityRange { note, min_velocity, max_velocity } => {
    if let ProcessedEvent::PadPressed { note: event_note, velocity, .. } = processed {
        *event_note == *note &&
        *velocity >= *min_velocity &&
        *velocity <= *max_velocity
    } else {
        false
    }
}
```

#### Configuration Example
```toml
# Soft press (velocity 1-40): Volume Down
[[modes.mappings]]
description = "Volume Down (soft press)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 1
max_velocity = 40
[modes.mappings.action]
type = "VolumeControl"
action = "Down"

# Hard press (velocity 80-127): Volume Up
[[modes.mappings]]
description = "Volume Up (hard press)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 80
max_velocity = 127
[modes.mappings.action]
type = "VolumeControl"
action = "Up"
```

#### Edge Cases
- **Velocity 0**: Treated as note off, not velocity level
- **Boundary Values**: 40/41 and 80/81 are exclusive (no overlap)
- **MIDI Device Variation**: Some devices have non-linear velocity curves
- **Overlapping Ranges**: User can create overlapping ranges in config (first match wins)

#### Testing Criteria
- ✅ Soft press (velocity 20) triggers soft action
- ✅ Medium press (velocity 60) triggers medium action
- ✅ Hard press (velocity 100) triggers hard action
- ✅ Boundary values (40, 41, 80, 81) correctly categorized
- ✅ Multiple velocity mappings on same note work independently

---

### F3: Long Press Detection

#### Description
Detects when a pad is held for a configurable duration (default 2000ms) to trigger alternative actions.

#### User Story
> As a developer (Sam), I want long press on "git commit" to prevent accidental commits, requiring intentional 1.5-second hold before executing.

#### Technical Implementation

**State Tracking (event_processor.rs:44-46):**
```rust
pub struct EventProcessor {
    note_press_times: HashMap<u8, Instant>,
    held_notes: HashMap<u8, Instant>,
    hold_threshold: Duration,
}
```

**Detection Logic:**
```rust
// On note on: record press time
MidiEvent::NoteOn { note, time, .. } => {
    self.note_press_times.insert(note, time);
    self.held_notes.insert(note, time);
}

// Periodic check (main.rs:400-414):
let hold_events = event_processor.check_holds();
for hold_event in hold_events {
    // hold_event is ProcessedEvent::LongPress
}

// check_holds() implementation:
pub fn check_holds(&mut self) -> Vec<ProcessedEvent> {
    let now = Instant::now();
    self.held_notes.iter()
        .filter_map(|(note, press_time)| {
            if now.duration_since(*press_time) >= self.hold_threshold {
                Some(ProcessedEvent::LongPress {
                    note: *note,
                    duration_ms: now.duration_since(*press_time).as_millis(),
                })
            } else {
                None
            }
        })
        .collect()
}
```

**Trigger Matching (mappings.rs):**
```rust
CompiledTrigger::LongPress { note, min_duration_ms } => {
    if let ProcessedEvent::LongPress { note: event_note, duration_ms } = processed {
        *event_note == *note && *duration_ms >= *min_duration_ms as u128
    } else {
        false
    }
}
```

#### Configuration Example
```toml
# Regular tap: Screenshot
[[modes.mappings]]
description = "Screenshot"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Keystroke"
keys = "4"
modifiers = ["cmd", "shift"]

# Long press (1000ms): Screen Recording
[[modes.mappings]]
description = "Start/Stop Screen Recording (long press)"
[modes.mappings.trigger]
type = "LongPress"
note = 14
min_duration_ms = 1000
[modes.mappings.action]
type = "Keystroke"
keys = "5"
modifiers = ["cmd", "shift"]

# Advanced: Custom duration for destructive actions
[[modes.mappings]]
description = "Quit Current App (long press 1.5s)"
[modes.mappings.trigger]
type = "LongPress"
note = 4
min_duration_ms = 1500
[modes.mappings.action]
type = "Keystroke"
keys = "q"
modifiers = ["cmd"]
```

#### Edge Cases
- **Release Before Threshold**: If released before duration, triggers normal Note event
- **Multiple Long Press Durations**: Can have multiple long press mappings with different durations on same note
- **Hold During Mode Switch**: Long press state resets on mode change
- **Chord + Long Press**: Not currently supported (chord detection takes precedence)

#### Testing Criteria
- ✅ Press and hold for 2000ms triggers long press event
- ✅ Release before 2000ms triggers regular note event
- ✅ Custom duration (1500ms) correctly configured
- ✅ Multiple long press mappings on same note with different durations work
- ✅ Long press event fires exactly once per hold

---

### F4: Double-Tap Detection

#### Description
Detects two quick successive presses within a configurable window (default 300ms) to trigger alternative actions.

#### User Story
> As a producer (Alex), I want double-tap on pad 16 to toggle fullscreen, making it easy to enter/exit fullscreen with a quick double-tap gesture.

#### Technical Implementation

**State Tracking (event_processor.rs:47):**
```rust
last_note_tap: HashMap<u8, Instant>,
double_tap_timeout: Duration,
```

**Detection Logic (event_processor.rs:76-86):**
```rust
MidiEvent::NoteOn { note, time, .. } => {
    // Check for double-tap
    if let Some(&last_tap_time) = self.last_note_tap.get(&note) {
        if time.duration_since(last_tap_time) < self.double_tap_timeout {
            results.push(ProcessedEvent::DoubleTap { note });
            self.last_note_tap.remove(&note); // Prevent triple-tap detection
        } else {
            self.last_note_tap.insert(note, time);
        }
    } else {
        self.last_note_tap.insert(note, time);
    }
}
```

#### Configuration Example
```toml
# Double-tap pad 16: Toggle fullscreen
[[modes.mappings]]
description = "Toggle Fullscreen (double tap)"
[modes.mappings.trigger]
type = "DoubleTap"
note = 16
max_interval_ms = 300
[modes.mappings.action]
type = "Keystroke"
keys = "f"
modifiers = ["ctrl", "cmd"]

# Advanced: Custom interval for slower double-tap
[[modes.mappings]]
description = "Confirm Action (slow double-tap)"
[modes.mappings.trigger]
type = "DoubleTap"
note = 20
max_interval_ms = 500
[modes.mappings.action]
type = "Text"
text = "Confirmed!"
```

#### Edge Cases
- **Triple Tap**: Third tap starts new double-tap detection (doesn't trigger twice)
- **Hold + Tap**: If first press held, second tap doesn't trigger double-tap
- **Chord Interference**: Double-tap takes precedence over chord detection
- **Max Interval**: User-configurable per mapping (default 300ms)

#### Testing Criteria
- ✅ Two taps within 300ms triggers double-tap event
- ✅ Two taps >300ms apart triggers two separate note events
- ✅ Triple tap only triggers one double-tap event (first two taps)
- ✅ Custom interval (500ms) correctly configured
- ✅ Double-tap event fires exactly once per double-tap

---

### F5: Chord Detection

#### Description
Detects multiple pads pressed nearly simultaneously (within configurable window, default 100ms) to trigger combined actions.

#### User Story
> As a developer (Sam), I want pressing pads 1+2 together to trigger "Save All Files", providing a safety mechanism for destructive or important actions.

#### Technical Implementation

**State Tracking (event_processor.rs:48-49):**
```rust
chord_buffer: Vec<(u8, Instant)>,
chord_timeout: Duration,
```

**Detection Logic (event_processor.rs:102-126):**
```rust
MidiEvent::NoteOn { note, time, .. } => {
    // Add to chord buffer
    self.chord_buffer.push((note, time));

    // Clean old notes from buffer
    self.chord_buffer.retain(|(_, t)| {
        time.duration_since(*t) < self.chord_timeout
    });

    // Check for complete chords
    if self.chord_buffer.len() >= 2 {
        let notes: Vec<u8> = self.chord_buffer.iter()
            .map(|(n, _)| *n)
            .collect();

        results.push(ProcessedEvent::ChordDetected { notes });
    }
}
```

**Trigger Matching (mappings.rs):**
```rust
CompiledTrigger::NoteChord { notes, max_interval_ms } => {
    if let ProcessedEvent::ChordDetected { notes: event_notes } = processed {
        // Check if all required notes are in the event
        notes.iter().all(|n| event_notes.contains(n))
    } else {
        false
    }
}
```

#### Configuration Example
```toml
# Chord (Pad 8 + 12): Force Quit Applications
[[modes.mappings]]
description = "Force Quit Applications (chord)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [8, 12]
max_interval_ms = 100
[modes.mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = ["cmd", "option"]

# Chord (Pad 1 + 2 + 3): Emergency Exit
[[global_mappings]]
description = "Emergency Exit (3-pad chord)"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 2, 3]
max_interval_ms = 150
[global_mappings.action]
type = "Shell"
command = "killall midimon"
```

#### Edge Cases
- **Partial Chords**: If only 1 of 2 notes pressed, triggers individual note events
- **Order Independence**: Notes can be pressed in any order
- **Extra Notes**: Chord matches even if additional notes pressed (subset matching)
- **Timeout Window**: Configurable per mapping (default 100ms from first note)
- **Chord + Individual**: Individual note mappings still trigger unless suppressed

#### Testing Criteria
- ✅ Two notes within 100ms triggers chord event
- ✅ Two notes >100ms apart triggers two separate note events
- ✅ Order of presses doesn't matter
- ✅ Custom interval (150ms) correctly configured
- ✅ Chord event fires once when all notes pressed
- ✅ Individual note events suppressed when chord detected

---

### F6: Encoder Direction Detection

#### Description
Detects encoder rotation direction (clockwise/counter-clockwise) and triggers different actions based on direction.

#### User Story
> As a producer (Alex), I want encoder 1 to switch modes (clockwise = next, counter-clockwise = previous) so I can easily navigate between different mapping sets.

#### Technical Implementation

**State Tracking (event_processor.rs:46):**
```rust
last_cc_values: HashMap<u8, u8>,
```

**Detection Logic (event_processor.rs:140-156):**
```rust
MidiEvent::ControlChange { cc, value, .. } => {
    let last_value = self.last_cc_values.get(&cc).copied().unwrap_or(0);

    let direction = if value > last_value {
        EncoderDirection::Clockwise
    } else if value < last_value {
        EncoderDirection::CounterClockwise
    } else {
        return results; // No change
    };

    let delta = ((value as i16) - (last_value as i16)).abs() as u8;

    results.push(ProcessedEvent::EncoderTurned {
        cc, value, direction, delta
    });

    self.last_cc_values.insert(cc, value);
}
```

#### Configuration Example
```toml
# Encoder 1 Clockwise: Next Mode
[[global_mappings]]
description = "Next Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1

# Encoder 1 Counter-Clockwise: Previous Mode
[[global_mappings]]
description = "Previous Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = 0

# Encoder 2: System Volume Control
[[global_mappings]]
description = "Volume Up"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
action = "Up"

[[global_mappings]]
description = "Volume Down"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "CounterClockwise"
[global_mappings.action]
type = "VolumeControl"
action = "Down"
```

#### Edge Cases
- **Absolute vs. Relative**: Assumes absolute CC values (0-127), not relative encoders
- **Wrap-Around**: 127→0 treated as decrease, 0→127 treated as increase
- **Delta Calculation**: Available for acceleration-based actions (future feature)
- **Fast Rotation**: Multiple CC messages may arrive in quick succession

#### Testing Criteria
- ✅ Clockwise rotation triggers Clockwise event
- ✅ Counter-clockwise rotation triggers CounterClockwise event
- ✅ Delta correctly calculated (difference between values)
- ✅ Wrap-around handling (127→0 and 0→127)
- ✅ Fast rotation doesn't drop events

---

### F7: LED Feedback System

#### Description
Comprehensive LED control system supporting 10 lighting schemes with full RGB control for HID devices and basic on/off for MIDI devices.

#### User Story
> As a performer (Morgan), I want my Launchpad LEDs to reflect my current mode and actions in real-time, providing visual feedback during live performances.

#### Technical Implementation

**Trait-Based Abstraction (feedback.rs:9-16):**
```rust
pub trait PadFeedback: Send {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn set_mode_colors(&mut self, mode: u8) -> Result<(), Box<dyn Error>>;
    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>>;
    fn clear_all(&mut self) -> Result<(), Box<dyn Error>>;
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>>;
}
```

**Device Factory (feedback.rs:59-75):**
```rust
pub fn create_feedback_device(
    device_name: &str,
    midi_port: Option<usize>,
    use_hid: bool
) -> Box<dyn PadFeedback> {
    if device_name.to_lowercase().contains("maschine") &&
       device_name.to_lowercase().contains("mikro") &&
       use_hid
    {
        Box::new(MikroMK3LEDs::new())
    } else {
        Box::new(MidiFeedback::new(midi_port))
    }
}
```

**HID Implementation (mikro_leds.rs):**
- Vendor ID: 0x17CC (Native Instruments)
- Product ID: 0x1700 (Maschine Mikro MK3)
- Report ID: 0x80 (LED control)
- Data Format: 3 bytes per pad (RGB) × 16 pads = 48 bytes

**MIDI Implementation (midi_feedback.rs):**
- Note On (velocity 127) = LED On
- Note Off (velocity 0) = LED Off
- Note range: C1 (36) to D#2 (51) for 16 pads

#### Configuration Example
```bash
# Command-line LED scheme selection
cargo run --release 2 --led reactive
cargo run --release 2 --led rainbow
cargo run --release 2 --led off
```

**Available Schemes:**
1. **off** - All LEDs disabled
2. **static** - Mode-based static colors
3. **breathing** - Slow breathing effect (2-second cycle)
4. **pulse** - Fast pulse effect (500ms cycle)
5. **rainbow** - Rainbow cycle across pads
6. **wave** - Wave pattern animation
7. **sparkle** - Random sparkles
8. **reactive** - Velocity-based response (green/yellow/red)
9. **vumeter** - VU meter style (bottom-up)
10. **spiral** - Spiral pattern animation

**Reactive Mode Detail (most commonly used):**
```rust
// Velocity → Color mapping
let color = match velocity {
    0..=40 => RGB { r: 0, g: 255, b: 0 },      // Soft = Green
    41..=80 => RGB { r: 255, g: 255, b: 0 },   // Medium = Yellow
    81..=127 => RGB { r: 255, g: 0, b: 0 },    // Hard = Red
};

// Fade-out after 1 second
let fade_out_delay = Duration::from_millis(1000);
```

#### Edge Cases
- **HID Shared Access**: Uses `macos-shared-device` feature to allow simultaneous access with NI Controller Editor
- **MIDI Compatibility**: Falls back to basic on/off for non-HID devices
- **Animation Performance**: Updates at 10fps to maintain <1% CPU usage
- **Mode Colors**: Hardcoded (Blue=Mode 0, Green=Mode 1, Purple=Mode 2)

#### Testing Criteria
- ✅ All 10 schemes render correctly
- ✅ Reactive mode shows correct colors for velocity levels
- ✅ Fade-out works (1 second after pad release)
- ✅ Mode switch updates LED colors
- ✅ HID and MIDI devices both supported
- ✅ Animation doesn't impact input latency
- ✅ Shared device access works with NI Controller Editor running

---

### F8: Mode System

#### Description
Multiple modes with independent mapping sets, allowing users to create context-specific control layouts (e.g., Default, Development, Media).

#### User Story
> As a developer (Sam), I want different modes for different contexts—one for git operations, one for testing, one for general productivity—so I can reuse the same pads for different purposes.

#### Technical Implementation

**Mode Storage (mappings.rs:7-9):**
```rust
pub struct MappingEngine {
    mode_mappings: HashMap<u8, Vec<CompiledMapping>>,
    global_mappings: Vec<CompiledMapping>,
}
```

**Mode Switching (main.rs:376-389):**
```rust
if let Some(new_mode) = mapping_engine.check_mode_change(&processed_event) {
    current_mode.store(new_mode, Ordering::Relaxed);
    println!("{} {}",
        "Mode changed to:".cyan().bold(),
        config.modes.get(new_mode as usize)
            .map(|m| m.name.as_str())
            .unwrap_or("Unknown")
            .yellow()
    );

    // Update LED feedback for new mode
    if lighting_scheme == LightingScheme::Static(0) ||
       lighting_scheme == LightingScheme::Reactive
    {
        let _ = feedback.set_mode_colors(new_mode);
    }
}
```

**Action Resolution (mappings.rs:195-218):**
```rust
pub fn get_action_for_processed(&self, event: &ProcessedEvent, mode: u8) -> Option<Action> {
    // 1. Check mode-specific mappings first
    if let Some(mode_mappings) = self.mode_mappings.get(&mode) {
        for mapping in mode_mappings {
            if self.trigger_matches_processed(&mapping.trigger, event) {
                return Some(mapping.action.clone());
            }
        }
    }

    // 2. Fall back to global mappings
    for mapping in &self.global_mappings {
        if self.trigger_matches_processed(&mapping.trigger, event) {
            return Some(mapping.action.clone());
        }
    }

    None
}
```

#### Configuration Example
```toml
# Mode 0: Default Mode (General Productivity)
[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Spotlight Search"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

# Mode 1: Development Mode (Git & Testing)
[[modes]]
name = "Development"
color = "green"

[[modes.mappings]]
description = "Run Tests"
[modes.mappings.trigger]
type = "Note"
note = 12  # Same pad, different action!
[modes.mappings.action]
type = "Shell"
command = "cargo test"

# Mode 2: Media Mode (Playback Control)
[[modes]]
name = "Media"
color = "purple"

[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 12  # Same pad again!
[modes.mappings.action]
type = "Keystroke"
keys = "F8"
modifiers = []

# Global Mappings (work in all modes)
[[global_mappings]]
description = "Next Mode (Encoder Clockwise)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1

[[global_mappings]]
description = "Emergency Exit (Hold 3s)"
[global_mappings.trigger]
type = "LongPress"
note = 1
min_duration_ms = 3000
[global_mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = []
```

#### Edge Cases
- **Mode Index**: Modes are 0-indexed (Mode 0, Mode 1, Mode 2, etc.)
- **Circular Switching**: Encoder can cycle through modes (0→1→2→0)
- **Global Precedence**: Global mappings checked after mode-specific mappings
- **Invalid Mode**: Switching to non-existent mode falls back to mode 0
- **State Persistence**: Mode state not persisted across restarts

#### Testing Criteria
- ✅ Mode switching via encoder works
- ✅ Different mappings for same pad in different modes
- ✅ Global mappings work in all modes
- ✅ Mode colors update LEDs correctly
- ✅ Mode persists during session
- ✅ Mode resets to 0 on restart

---

### F9: Device Profile Support

#### Description
Load Native Instruments Controller Editor profiles (.ncmm3 XML format) to map physical pad positions to MIDI notes, with support for multiple pad pages (A-H).

#### User Story
> As a producer (Alex), I want to load my custom Controller Editor template so MIDIMon uses the same pad layout I configured in NI software.

#### Technical Implementation

**Profile Parser (device_profile.rs):**
```rust
pub struct DeviceProfile {
    pub device_name: String,
    pub pad_pages: Vec<PadPageMapping>,
}

pub struct PadPageMapping {
    pub name: String, // "Pad Page A", "Pad Page B", etc.
    pub mappings: Vec<(usize, u8)>, // (pad_index, note_number)
}

impl DeviceProfile {
    pub fn from_ncmm3(path: &str) -> Result<Self> {
        // Parse XML using quick-xml
        let xml_content = std::fs::read_to_string(path)?;
        let mut reader = Reader::from_str(&xml_content);

        // Extract device name and pad mappings
        // ...
    }

    pub fn detect_page_for_note(&self, note: u8) -> Option<&PadPageMapping> {
        self.pad_pages.iter().find(|page| {
            page.mappings.iter().any(|(_, n)| *n == note)
        })
    }
}
```

**Auto-Detection (main.rs:295-318):**
```rust
// Auto-detect pad page from first MIDI note
if auto_detect_page {
    let needs_detection = if let Some(page) = &active_pad_page {
        page.note_to_pad_index(*note).is_none()
    } else {
        true
    };

    if needs_detection {
        if let Some(profile) = &device_profile {
            if let Some(detected_page) = profile.detect_page_for_note(*note) {
                active_pad_page = Some(detected_page.clone());
                println!("{} {} (notes {})",
                    "✓ Auto-detected pad page:".green(),
                    detected_page.name.yellow(),
                    detected_page.note_range().map(|(min, max)| format!("{}-{}", min, max))
                );
            }
        }
    }
}
```

#### Configuration Example
```bash
# Load profile with auto-detection
cargo run --release 2 --profile ~/Downloads/base-template-ni-mikro-mk3.ncmm3

# Load profile with specific pad page
cargo run --release 2 --profile mikro.ncmm3 --pad-page H

# Auto-load default profile from Downloads
# (Automatically checks ~/Downloads/base-template-ni-mikro-mk3.ncmm3)
cargo run --release 2
```

**Profile XML Structure (.ncmm3):**
```xml
<Controller>
  <Name>Maschine Mikro MK3</Name>
  <PadPages>
    <PadPage name="A">
      <Pad index="0" note="36"/>
      <Pad index="1" note="37"/>
      <!-- ... -->
    </PadPage>
    <PadPage name="H">
      <Pad index="0" note="84"/>
      <Pad index="1" note="85"/>
      <!-- ... -->
    </PadPage>
  </PadPages>
</Controller>
```

#### Edge Cases
- **Missing Profile**: Falls back to hardcoded Mikro MK3 defaults (notes 16-31)
- **Invalid XML**: Returns parse error, doesn't crash
- **Wrong Page**: Auto-detection re-detects if note not in current page
- **Multiple Devices**: Only one profile loaded per session (first device)
- **Profile Updates**: Requires app restart to reload profile

#### Testing Criteria
- ✅ .ncmm3 XML correctly parsed
- ✅ Pad page auto-detection works from first note
- ✅ Manual pad page selection via `--pad-page` works
- ✅ Falls back to defaults if profile missing
- ✅ LED addressing uses correct pad index from profile

---

## Target Features (v2.0 - Modular)

### F10: Menu Bar Application

#### Description
System tray application providing status, quick actions, and access to configuration without main window.

#### User Story
> As a user, I want MIDIMon to run in the background with quick access from the menu bar, so I can enable/disable or reload config without opening a full application.

#### Technical Implementation

**Tauri Tray (midimon-daemon):**
```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem, CustomMenuItem};

let quit = CustomMenuItem::new("quit", "Quit");
let pause = CustomMenuItem::new("pause", "Pause Mappings");
let reload = CustomMenuItem::new("reload", "Reload Config");
let configure = CustomMenuItem::new("configure", "Open Configuration...");

let tray_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new("status", "MIDIMon Active").disabled())
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(pause)
    .add_item(reload)
    .add_item(configure)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

let system_tray = SystemTray::new().with_menu(tray_menu);
```

**Menu Event Handling:**
```rust
.on_system_tray_event(|app, event| {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "pause" => {
                    let paused = ENGINE.get().unwrap().is_paused();
                    ENGINE.get().unwrap().set_paused(!paused);
                    // Update menu item text
                }
                "reload" => {
                    ENGINE.get().unwrap().reload_config();
                }
                "configure" => {
                    // Launch midimon-gui
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        _ => {}
    }
})
```

#### Configuration Example
```toml
# No direct configuration needed
# Menu bar app runs automatically at startup
```

#### Edge Cases
- **Multiple Instances**: Prevent multiple instances running simultaneously
- **Lost Focus**: Menu bar icon always visible even when app not frontmost
- **Status Updates**: Poll engine for status updates every 1 second

#### Testing Criteria
- 🔄 Menu bar icon appears on startup
- 🔄 Click icon shows menu
- 🔄 Pause/resume works
- 🔄 Reload config works
- 🔄 Open configuration launches GUI
- 🔄 Quit cleanly shuts down daemon

---

### F11: Visual Configuration UI (Tauri)

#### Description
Web-based GUI for visual device configuration, MIDI Learn, and action library, built with Tauri v2.

#### User Story
> As a producer (Alex) who's not comfortable with TOML, I want to configure my controller by dragging actions onto pads in a visual interface.

#### Technical Implementation

**Tauri Window (midimon-gui):**
```rust
tauri::Builder::default()
    .setup(|app| {
        let window = tauri::WindowBuilder::new(
            app,
            "main",
            tauri::WindowUrl::App("index.html".into())
        )
        .title("MIDIMon Configuration")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .build()?;

        Ok(())
    })
```

**Device Visualization (React/Svelte UI):**
```jsx
// Device component showing 4x4 grid for Mikro MK3
<DeviceLayout device="mikro-mk3">
  {pads.map((pad, index) => (
    <Pad
      key={index}
      number={index}
      color={pad.color}
      mapping={pad.mapping}
      onDrop={action => assignAction(index, action)}
      onClick={() => enterMIDILearn(index)}
    />
  ))}
</DeviceLayout>
```

**Action Library:**
```jsx
<ActionLibrary>
  <ActionCategory name="System">
    <ActionItem type="Keystroke" icon="⌘" draggable />
    <ActionItem type="Launch" icon="🚀" draggable />
    <ActionItem type="Shell" icon="⌨️" draggable />
  </ActionCategory>

  <ActionCategory name="Media">
    <ActionItem type="VolumeControl" icon="🔊" draggable />
    <ActionItem type="PlayPause" icon="⏯️" draggable />
  </ActionCategory>
</ActionLibrary>
```

**MIDI Learn Workflow:**
```jsx
function enterMIDILearn(padIndex) {
  setLearningPad(padIndex);

  // Listen for next MIDI event from engine
  listenForMIDI(event => {
    // Auto-fill trigger config
    setTrigger(padIndex, {
      type: "Note",
      note: event.note,
      velocity_min: 1,
      velocity_max: 127
    });

    setLearningPad(null);
  });
}
```

#### Configuration Example
```toml
# No TOML editing required!
# Configuration generated by GUI and saved to config.toml
```

#### Edge Cases
- **Concurrent Editing**: Lock config file while GUI open to prevent conflicts
- **Invalid Actions**: Validate actions before saving (e.g., non-existent apps)
- **Unsaved Changes**: Prompt user before closing with unsaved changes
- **Config Backup**: Auto-backup config before overwriting

#### Testing Criteria
- 🔄 GUI launches from menu bar
- 🔄 Device visualization renders correctly
- 🔄 Drag-and-drop action assignment works
- 🔄 MIDI Learn captures correct note
- 🔄 Save writes valid config.toml
- 🔄 Hot reload applies changes immediately

---

### F12: Hot Config Reload

#### Description
Watch config.toml for changes and reload mappings without restarting daemon, preserving current state.

#### User Story
> As a developer (Sam), I want to iterate on my config quickly by editing TOML and seeing changes immediately, without stopping and restarting the daemon.

#### Technical Implementation

**File Watcher (midimon-core):**
```rust
use notify::{Watcher, RecursiveMode, Result};
use std::sync::mpsc::channel;

fn watch_config(config_path: PathBuf, reload_tx: Sender<()>) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = notify::watcher(tx, Duration::from_secs(1))?;

    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    thread::spawn(move || {
        for event in rx {
            match event {
                DebouncedEvent::Write(_) | DebouncedEvent::Create(_) => {
                    let _ = reload_tx.send(());
                }
                _ => {}
            }
        }
    });

    Ok(())
}
```

**Reload Handler (midimon-daemon):**
```rust
impl Engine {
    pub fn reload_config(&mut self) -> Result<()> {
        // 1. Load new config
        let new_config = Config::load()?;

        // 2. Validate config
        new_config.validate()?;

        // 3. Compile new mappings
        let mut new_mapping_engine = MappingEngine::new();
        new_mapping_engine.load_from_config(&new_config);

        // 4. Atomically swap (preserves state)
        self.mapping_engine = new_mapping_engine;
        self.config = Arc::new(RwLock::new(new_config));

        // 5. Notify user
        println!("{}", "✓ Config reloaded successfully".green());

        Ok(())
    }
}
```

**State Preservation:**
- Current mode preserved
- Active pad presses tracked (don't lose mid-press)
- Timer state maintained
- LED state updated to match new config

#### Configuration Example
```toml
# Edit config.toml while daemon running
# Changes applied automatically within 1 second
```

#### Edge Cases
- **Invalid Config**: Reload fails, old config remains active, error logged
- **Syntax Error**: TOML parse error prevents reload, old config safe
- **Partial Edit**: Watcher debounces (waits 1 second) to avoid mid-edit reloads
- **Mode Change During Reload**: Mode preserved unless deleted in new config

#### Testing Criteria
- 🔄 Edit config.toml triggers reload within 1 second
- 🔄 Invalid config doesn't crash daemon
- 🔄 Current mode persisted across reload
- 🔄 Active pad presses handled correctly
- 🔄 LED state updates to match new config
- 🔄 Menu bar shows "Reloaded" notification

---

### F13: Per-App Profile Switching

#### Description
Auto-detect frontmost application and switch to app-specific profile, enabling context-aware mappings.

#### User Story
> As a producer (Alex), I want MIDIMon to automatically switch to my "Logic Pro" profile when Logic is frontmost, so I don't have to manually switch modes.

#### Technical Implementation

**Frontmost App Detection (macOS):**
```rust
use cocoa::appkit::{NSWorkspace, NSRunningApplication};
use cocoa::base::{id, nil};

fn get_frontmost_app() -> Option<String> {
    unsafe {
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let frontmost_app: id = msg_send![workspace, frontmostApplication];

        if frontmost_app == nil {
            return None;
        }

        let bundle_id: id = msg_send![frontmost_app, bundleIdentifier];

        if bundle_id == nil {
            return None;
        }

        let bundle_id_str = NSString::UTF8String(bundle_id);
        Some(CStr::from_ptr(bundle_id_str).to_string_lossy().into_owned())
    }
}
```

**Profile Matching:**
```rust
pub struct ProfileMatcher {
    app_profiles: HashMap<String, String>, // bundle_id -> profile_name
    default_profile: String,
}

impl ProfileMatcher {
    pub fn match_profile(&self, bundle_id: &str) -> &str {
        self.app_profiles.get(bundle_id)
            .map(|s| s.as_str())
            .unwrap_or(&self.default_profile)
    }
}
```

**Polling Loop (midimon-daemon):**
```rust
// Poll every 500ms for frontmost app changes
let mut last_bundle_id = None;

loop {
    if let Some(bundle_id) = get_frontmost_app() {
        if last_bundle_id.as_ref() != Some(&bundle_id) {
            let profile = profile_matcher.match_profile(&bundle_id);

            if profile != current_profile {
                engine.switch_profile(profile)?;
                println!("{} {}", "Switched to profile:".cyan(), profile.yellow());

                current_profile = profile.to_string();
            }

            last_bundle_id = Some(bundle_id);
        }
    }

    thread::sleep(Duration::from_millis(500));
}
```

#### Configuration Example
```toml
# Per-app profile mapping
[app_profiles]
"com.apple.logic10" = "Logic Pro"
"com.ableton.live" = "Ableton Live"
"com.microsoft.VSCode" = "Development"
"com.google.Chrome" = "Browser"
"*" = "Default"  # Fallback for unknown apps

# Profile definitions
[[profiles]]
name = "Logic Pro"
# ... mappings for Logic Pro

[[profiles]]
name = "Development"
# ... mappings for VS Code
```

#### Edge Cases
- **App Without Bundle ID**: Falls back to default profile
- **Multiple Windows**: Uses frontmost app, not specific window
- **Rapid Switching**: Debounce to avoid thrashing (500ms poll)
- **Profile Not Found**: Falls back to default profile
- **Background Apps**: Ignores menu bar apps and background processes

#### Testing Criteria
- 🔄 Switch to Logic Pro → profile changes automatically
- 🔄 Switch to VS Code → profile changes automatically
- 🔄 Switch to unknown app → uses default profile
- 🔄 Rapid app switching doesn't cause issues
- 🔄 Menu bar shows current profile name

---

## Future Features (v2.5+)

### F14: Virtual MIDI Output

#### Description
Expose MIDIMon as virtual MIDI device that can send MIDI to DAWs and other music software.

#### User Story
> As a performer (Morgan), I want to send MIDI from my actions back to Ableton, so I can trigger clips and control effects from MIDIMon.

#### Technical Implementation

**CoreMIDI Virtual Source (macOS):**
```rust
use coremidi::{Client, Destination, Source, PacketBuffer};

pub struct VirtualMIDIOutput {
    client: Client,
    source: Source,
}

impl VirtualMIDIOutput {
    pub fn new(name: &str) -> Result<Self> {
        let client = Client::new(name)?;
        let source = client.virtual_source(name, |_, _| {})?;

        Ok(Self { client, source })
    }

    pub fn send_note_on(&self, channel: u8, note: u8, velocity: u8) -> Result<()> {
        let mut buffer = PacketBuffer::new(0);
        buffer.push_data(&[
            0x90 | (channel & 0x0F),
            note & 0x7F,
            velocity & 0x7F
        ]);

        self.source.received(&buffer);
        Ok(())
    }
}
```

**Action Integration:**
```rust
Action::MIDIOut { channel, note, velocity } => {
    virtual_midi.send_note_on(channel, note, velocity)?;
}
```

#### Configuration Example
```toml
[virtual_midi]
enabled = true
port_name = "MIDIMon Out"

# Action: Send MIDI note
[[modes.mappings]]
description = "Trigger Ableton Clip"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "MIDIOut"
channel = 0
note = 60
velocity = 127
```

#### Testing Criteria
- 🔄 Virtual port appears in MIDI devices
- 🔄 DAW receives MIDI from MIDIMon
- 🔄 Note on/off correctly sent
- 🔄 Channel and velocity respected

---

### F15: Profile Marketplace

#### Description
Community platform for sharing and discovering device profiles and templates.

#### User Story
> As a producer (Alex), I want to download pre-made profiles for my controller instead of configuring from scratch.

#### Technical Implementation

**GitHub-Based Marketplace:**
```
midimon-profiles/
├── mikro-mk3/
│   ├── logic-pro.toml
│   ├── ableton-live.toml
│   └── general-productivity.toml
├── launchpad-mini/
│   ├── obs-streaming.toml
│   └── development.toml
└── manifest.json
```

**Profile Manager (GUI):**
```jsx
<ProfileMarketplace>
  <SearchBar placeholder="Search profiles..." />

  <ProfileCard
    name="Logic Pro Template"
    author="alextheproducer"
    downloads={523}
    rating={4.8}
    device="Mikro MK3"
    onInstall={() => downloadProfile("logic-pro.toml")}
  />
</ProfileMarketplace>
```

#### Testing Criteria
- 🔄 Browse available profiles
- 🔄 Download and install profile
- 🔄 Profile works after installation
- 🔄 Upload custom profile to marketplace

---

### F7: Aftertouch (Pressure) Trigger

#### Description
Detects pressure (aftertouch) applied to a pad after the initial press, enabling pressure-sensitive control.

#### User Story
> As a performer (Morgan), I want to apply pressure to pads to modulate effects so I can add expression without additional controls.

#### Technical Implementation

**MIDI Detection (event_processor.rs:122-135):**
```rust
MidiEvent::Aftertouch { note, pressure, time } => {
    results.push(ProcessedEvent::Aftertouch {
        note,
        pressure,
        time,
    });
}
```

**Trigger Matching (mappings.rs):**
```rust
CompiledTrigger::Aftertouch { note, min_pressure, max_pressure } => {
    if let ProcessedEvent::Aftertouch { note: event_note, pressure, .. } = processed {
        *event_note == *note &&
        pressure >= min_pressure &&
        pressure <= max_pressure
    } else {
        false
    }
}
```

#### Configuration Example
```toml
# Aftertouch on Pad 1: Increase effect intensity
[[modes.mappings]]
description = "Effect intensity via pressure"
[modes.mappings.trigger]
type = "Aftertouch"
note = 1
min_pressure = 64  # Trigger only on significant pressure
[modes.mappings.action]
type = "CC"
cc = 74  # Filter cutoff
value_from_pressure = true

# Pressure range mapping
[[modes.mappings]]
description = "Soft pressure action"
[modes.mappings.trigger]
type = "Aftertouch"
note = 2
min_pressure = 1
max_pressure = 64
[modes.mappings.action]
type = "Keystroke"
keys = "1"

# Hard pressure action
[[modes.mappings]]
description = "Hard pressure action"
[modes.mappings.trigger]
type = "Aftertouch"
note = 2
min_pressure = 65
max_pressure = 127
[modes.mappings.action]
type = "Keystroke"
keys = "2"
```

#### Pressure Curves

Different devices have different pressure sensitivities. Pressure curves can be applied to normalize feel across devices:

```rust
pub enum PressureCurve {
    Linear,                           // 1:1 mapping (default)
    Exponential { exponent: f32 },    // pressure^exponent (lighter touch emphasis)
    Logarithmic { base: f32 },        // log(pressure) / log(base) (harder touch emphasis)
    SCurve { midpoint: u8 },          // Sigmoid for natural feel
}

impl PressureMapper {
    pub fn apply_curve(&self, raw_pressure: u8) -> u8 {
        let normalized = raw_pressure as f32 / 127.0;

        let curved = match &self.curve {
            PressureCurve::Linear => normalized,
            PressureCurve::Exponential { exponent } => normalized.powf(*exponent),
            PressureCurve::Logarithmic { base } => {
                if normalized < 0.01 { 0.0 } else { normalized.log(*base) / (127.0_f32).log(*base) }
            }
            PressureCurve::SCurve { midpoint } => {
                let mid = *midpoint as f32 / 127.0;
                let x = (normalized - mid) * 10.0;
                1.0 / (1.0 + (-x).exp())
            }
        };

        (curved * 127.0).clamp(0.0, 127.0) as u8
    }
}
```

**Configuration with curves:**
```toml
[device.pressure_curve]
type = "Exponential"
exponent = 2.0  # Quadratic curve - lighter touches emphasized

# Or use S-curve for natural feel
[device.pressure_curve]
type = "SCurve"
midpoint = 64  # Inflection point at mid-pressure
```

#### Device Compatibility Matrix

| Device | Aftertouch Type | Notes |
|--------|----------------|-------|
| Maschine Mikro MK3 | Polyphonic | Per-pad pressure, 0-127 range, good sensitivity |
| Akai MPD Series | Polyphonic | Per-pad, requires firmware 1.5+ |
| Novation Launchpad Pro | Polyphonic | Per-pad, excellent sensitivity |
| Novation Launchpad Mini | ❌ None | No aftertouch support |
| Korg nanoPAD2 | ❌ None | No aftertouch support |
| Arturia KeyLab | Channel | Global aftertouch only (all keys share) |
| ROLI Seaboard | Polyphonic | 5D touch including pressure per key |
| Generic MIDI Keyboard | Channel | Most support channel aftertouch only |

**Polyphonic vs Channel Aftertouch:**
- **Polyphonic (0xA0)**: Each pad/key has independent pressure. MIDI message: `0xA0 [note] [pressure]`
- **Channel (0xD0)**: Single pressure value for entire device. MIDI message: `0xD0 [pressure]`

MIDIMon auto-detects the type from incoming MIDI messages and handles both transparently.

#### Continuous Message Handling Strategies

Aftertouch generates continuous messages while pressure is applied. Strategies for handling:

**1. Throttling (Rate Limiting):**
```rust
pub struct AftertouchThrottle {
    last_sent: Instant,
    min_interval: Duration,  // e.g., 50ms = 20 messages/sec max
}

impl AftertouchThrottle {
    pub fn should_send(&mut self, now: Instant) -> bool {
        if now.duration_since(self.last_sent) >= self.min_interval {
            self.last_sent = now;
            true
        } else {
            false
        }
    }
}
```

**2. Delta Threshold (Only on Significant Change):**
```rust
pub fn should_trigger(&self, new_pressure: u8) -> bool {
    let delta = (new_pressure as i16 - self.last_pressure as i16).abs();
    delta >= self.delta_threshold  // e.g., 5 = trigger only on ±5 pressure change
}
```

**3. Hysteresis (Prevent Jitter):**
```rust
pub struct PressureHysteresis {
    current_zone: PressureZone,
    low_threshold: u8,
    high_threshold: u8,
}

// Prevents oscillation between zones when pressure hovers near boundary
impl PressureHysteresis {
    pub fn update(&mut self, pressure: u8) -> Option<PressureZone> {
        match self.current_zone {
            PressureZone::Low if pressure > self.high_threshold => {
                self.current_zone = PressureZone::High;
                Some(PressureZone::High)
            }
            PressureZone::High if pressure < self.low_threshold => {
                self.current_zone = PressureZone::Low;
                Some(PressureZone::Low)
            }
            _ => None,  // Stay in current zone
        }
    }
}
```

**Configuration:**
```toml
[advanced_settings.aftertouch]
throttle_ms = 50        # Max 20 messages/sec
delta_threshold = 5     # Trigger only on ±5 change
hysteresis_gap = 10     # 10-point gap between zone boundaries
```

#### Edge Cases
- **Device Support**: Not all MIDI controllers support aftertouch (see compatibility matrix)
- **Polyphonic vs Channel**: Implementation auto-detects and supports both types
- **Pressure Resolution**: 0-127 range, sensitivity varies by device (see compatibility matrix)
- **Multiple Triggers**: Different actions can be mapped to different pressure ranges
- **Continuous Messages**: Aftertouch generates many messages - use throttling/delta/hysteresis strategies
- **Pressure Curves**: Apply curves for consistent feel across different controllers
- **Zero Pressure**: Some controllers send 0 pressure on release - filter these to avoid double-triggering
- **Stuck Pressure**: If pad physically stuck, pressure may continue - implement timeout

#### Testing Criteria
- ✅ Aftertouch message triggers mapped action
- ✅ Pressure threshold filtering works correctly
- ✅ Pressure range (min/max) correctly filters
- ✅ Value mapping from pressure to action parameter
- ✅ Multiple pads can have independent aftertouch mappings
- ✅ Channel aftertouch vs polyphonic aftertouch both supported
- ✅ Pressure curves apply correctly (exponential, logarithmic, S-curve)
- ✅ Throttling limits message rate as configured
- ✅ Delta threshold prevents excessive triggering
- ✅ Hysteresis prevents zone oscillation
- ✅ Device compatibility correctly detected
- ✅ Zero pressure on release doesn't double-trigger

---

### F8: PitchBend (Touch Strip) Trigger

#### Description
Detects pitch bend messages from touch strips or pitch wheels, enabling position-sensitive control.

#### User Story
> As a producer (Alex), I want to use the touch strip to scrub through timeline or control parameters so I can have smooth continuous control.

#### Technical Implementation

**MIDI Detection (event_processor.rs:137-143):**
```rust
MidiEvent::PitchBend { value, time } => {
    // PitchBend is 14-bit: 0-16383, center at 8192
    results.push(ProcessedEvent::PitchBend {
        value,  // Raw 14-bit value
        normalized: (value as f32 - 8192.0) / 8192.0,  // -1.0 to +1.0
        time,
    });
}
```

**Trigger Matching (mappings.rs):**
```rust
CompiledTrigger::PitchBend { min_value, max_value, center_deadzone } => {
    if let ProcessedEvent::PitchBend { value, .. } = processed {
        // Check if within deadzone around center (8192)
        let distance_from_center = (*value as i32 - 8192).abs();
        if distance_from_center < *center_deadzone as i32 {
            return false;  // In deadzone, don't trigger
        }

        value >= min_value && value <= max_value
    } else {
        false
    }
}
```

#### Configuration Example

**Basic Range Mapping:**
```toml
# Touch strip up: Increase volume
[[global_mappings]]
description = "Volume up via touch strip"
[global_mappings.trigger]
type = "PitchBend"
min_value = 8192  # Center
max_value = 16383  # Max up
center_deadzone = 100  # Ignore small movements near center
[global_mappings.action]
type = "VolumeControl"
direction = "Up"
value_from_bend = true

# Touch strip down: Decrease volume
[[global_mappings]]
description = "Volume down via touch strip"
[global_mappings.trigger]
type = "PitchBend"
min_value = 0
max_value = 8192  # Center
center_deadzone = 100
[global_mappings.action]
type = "VolumeControl"
direction = "Down"
value_from_bend = true

# Touch strip center detect: Reset action
[[modes.mappings]]
description = "Reset on center touch"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 8092  # Center - 100
max_value = 8292  # Center + 100
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]
```

**Advanced Range Mapping (Multi-Zone):**
```toml
# Divide touch strip into 5 zones for different actions
[[modes.mappings]]
description = "Zone 1: Far Left (0-3276)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 0
max_value = 3276  # 20% of range
[modes.mappings.action]
type = "Keystroke"
keys = "1"

[[modes.mappings]]
description = "Zone 2: Left (3277-6553)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 3277
max_value = 6553  # 40% of range
[modes.mappings.action]
type = "Keystroke"
keys = "2"

[[modes.mappings]]
description = "Zone 3: Center (6554-9830)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 6554
max_value = 9830  # 60% of range
[modes.mappings.action]
type = "Keystroke"
keys = "3"

[[modes.mappings]]
description = "Zone 4: Right (9831-13107)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 9831
max_value = 13107  # 80% of range
[modes.mappings.action]
type = "Keystroke"
keys = "4"

[[modes.mappings]]
description = "Zone 5: Far Right (13108-16383)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 13108
max_value = 16383  # 100% of range
[modes.mappings.action]
type = "Keystroke"
keys = "5"
```

**Scrubbing/Timeline Control:**
```toml
# Precise timeline scrubbing with normalized value mapping
[[modes.mappings]]
description = "DAW timeline scrub"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 0
max_value = 16383
center_deadzone = 50
[modes.mappings.action]
type = "Shell"
command = "osascript -e 'tell application \"Logic Pro\" to set song position to $NORMALIZED_VALUE'"
# $NORMALIZED_VALUE is -1.0 to +1.0 from ProcessedEvent::PitchBend
```

**Continuous Control with Throttling:**
```toml
# Smooth parameter control with throttling
[[modes.mappings]]
description = "Effect parameter (throttled)"
[modes.mappings.trigger]
type = "PitchBend"
min_value = 0
max_value = 16383
throttle_ms = 50  # Limit to 20 updates/sec
[modes.mappings.action]
type = "MidiCC"
cc = 74  # Filter cutoff
value_from_bend = true  # Map 0-16383 → 0-127
```

#### 14-bit Value Normalization Details

Pitch bend uses 14-bit precision (0-16383) instead of standard MIDI's 7-bit (0-127):

```rust
// MIDI pitch bend message: [0xE0 | channel] [LSB] [MSB]
// LSB = 7 least significant bits
// MSB = 7 most significant bits
// Combined: (MSB << 7) | LSB = 0 to 16383

pub fn parse_pitch_bend(lsb: u8, msb: u8) -> u16 {
    ((msb as u16) << 7) | (lsb as u16)
}

pub fn normalize_pitch_bend(raw_value: u16) -> f32 {
    // Convert 0-16383 to -1.0 to +1.0 (center at 8192)
    (raw_value as f32 - 8192.0) / 8192.0
}

pub fn denormalize_pitch_bend(normalized: f32) -> u16 {
    // Convert -1.0 to +1.0 back to 0-16383
    ((normalized * 8192.0) + 8192.0).clamp(0.0, 16383.0) as u16
}
```

**Precision Comparison:**
- **7-bit (CC)**: 128 discrete values → ~0.78% per step
- **14-bit (Pitch Bend)**: 16384 discrete values → ~0.006% per step → 128× more precise

#### Continuous Message Throttling

Pitch bend generates 100-1000+ messages per second during movement. Throttling strategies:

**1. Time-Based Throttling (Rate Limiting):**
```rust
pub struct PitchBendThrottle {
    last_sent: Instant,
    min_interval: Duration,
}

impl PitchBendThrottle {
    pub fn should_send(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_sent) >= self.min_interval {
            self.last_sent = now;
            true
        } else {
            false
        }
    }
}

// Usage in event processor
if throttle.should_send() {
    execute_action(&action, bend_value);
}
```

**2. Delta-Based Throttling (Change Threshold):**
```rust
pub struct PitchBendDelta {
    last_value: u16,
    threshold: u16,  // e.g., 50 = only trigger on ±50 change
}

impl PitchBendDelta {
    pub fn should_send(&mut self, new_value: u16) -> bool {
        let delta = (new_value as i32 - self.last_value as i32).abs();
        if delta >= self.threshold as i32 {
            self.last_value = new_value;
            true
        } else {
            false
        }
    }
}
```

**3. Zone-Based Triggering (Discrete Zones):**
```rust
pub fn get_bend_zone(value: u16, num_zones: u8) -> u8 {
    // Divide 0-16383 into discrete zones
    let zone_size = 16384 / num_zones as u16;
    (value / zone_size).min(num_zones as u16 - 1) as u8
}

// Only trigger when zone changes (not on every bend message)
```

**Configuration:**
```toml
[advanced_settings.pitch_bend]
throttle_ms = 50         # Max 20 messages/sec
delta_threshold = 100    # Trigger only on ±100 change (~0.6% of range)
use_zones = true         # Use zone-based triggering
num_zones = 16           # 16 discrete zones
```

#### Spring-Back Controller Behavior

Many controllers (Maschine Mikro MK3, most MIDI keyboards) have spring-loaded pitch bend that auto-returns to center:

**Implications:**
- **Transient Control**: Pitch bend returns to 8192 (center) when released
- **Not for Persistent State**: Don't use for toggle or latching actions
- **Event Flood on Release**: Spring-back generates many messages during return
- **Use Cases**: Volume swells, temporary pitch shifts, momentary effects

**Non-Spring-Back Controllers:**
- ROLI Seaboard: Touch strips don't spring back
- Some touch-strip MIDI controllers: Position persists until next touch
- Ribbon controllers: Often non-spring-back

**Handling Spring-Back:**
```rust
pub struct SpringBackDetector {
    last_value: u16,
    last_time: Instant,
    is_returning: bool,
}

impl SpringBackDetector {
    pub fn detect_spring_back(&mut self, new_value: u16) -> bool {
        let now = Instant::now();
        let time_delta = now.duration_since(self.last_time).as_millis();
        let value_delta = (new_value as i32 - self.last_value as i32).abs();

        // Detect rapid movement toward center (8192)
        let moving_to_center = (new_value as i32 - 8192).abs() < (self.last_value as i32 - 8192).abs();
        let rapid_change = value_delta > 500 && time_delta < 50;

        self.is_returning = moving_to_center && rapid_change;
        self.last_value = new_value;
        self.last_time = now;

        self.is_returning
    }
}

// Ignore actions during spring-back return
if !spring_back.detect_spring_back(bend_value) {
    execute_action(&action, bend_value);
}
```

#### Platform-Specific Considerations

**macOS:**
- High-resolution pitch bend supported natively
- CoreMIDI handles 14-bit precision correctly
- No special handling required

**Linux (ALSA):**
- Some ALSA drivers may report pitch bend incorrectly
- Verify MSB/LSB byte order in event parsing
- May need endianness correction on some architectures

**Windows:**
- Windows MIDI API supports 14-bit pitch bend
- Some USB MIDI drivers have latency issues (10-50ms)
- Consider using ASIO drivers for lower latency

**Cross-Platform Normalization:**
```rust
#[cfg(target_os = "linux")]
pub fn parse_pitch_bend_linux(lsb: u8, msb: u8) -> u16 {
    // Some Linux drivers reverse byte order
    if cfg!(target_endian = "big") {
        ((lsb as u16) << 7) | (msb as u16)  // Swapped
    } else {
        ((msb as u16) << 7) | (lsb as u16)  // Standard
    }
}
```

#### Edge Cases
- **Center Detection**: Center is 8192, not 0 - need proper normalization
- **14-bit Resolution**: Full range is 0-16383 (not 0-127 like other MIDI messages) - 128× more precise
- **Spring-Back**: Many controllers auto-return to center when released - detect and optionally ignore
- **Continuous Messages**: Generates 100-1000+ messages/sec during movement - use throttling strategies
- **Deadzone**: Small movements near center should be ignored to prevent unintended triggers
- **Relative vs Absolute**: Implementation treats as absolute position, not delta
- **Byte Order**: Some Linux ALSA drivers may reverse MSB/LSB byte order
- **Zone Boundaries**: Multi-zone mappings need small gaps to prevent oscillation
- **Flood on Release**: Spring-back return generates message flood - consider ignoring

#### Testing Criteria
- ✅ Pitch bend up (>8192) triggers correct action
- ✅ Pitch bend down (<8192) triggers correct action
- ✅ Center deadzone prevents spurious triggers
- ✅ Full range (0-16383) correctly detected
- ✅ Normalized value (-1.0 to +1.0) calculated correctly
- ✅ Multiple range triggers can coexist
- ✅ Value mapping to action parameters works
- ✅ 14-bit precision maintained (no quantization to 7-bit)
- ✅ Throttling limits message rate as configured
- ✅ Delta threshold prevents excessive triggering
- ✅ Zone-based triggering changes zones correctly
- ✅ Spring-back detection identifies return-to-center correctly
- ✅ Multi-zone mappings don't oscillate at boundaries
- ✅ Cross-platform byte order handled correctly

---

### F11: Text Typing Action

#### Description
Types arbitrary text strings, including unicode characters, special symbols, and escape sequences. Uses the `enigo` crate for cross-platform keyboard simulation.

#### User Story
> As a developer (Sam), I want to type common code snippets or boilerplate text with a single pad press so I can avoid repetitive typing.

#### Technical Implementation

**Action Definition (config.rs:49):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionConfig {
    Text { text: String },
    // ... other actions
}
```

**Action Execution (actions.rs:39-40):**
```rust
Action::Text(text) => {
    self.enigo.text(&text).unwrap();
}
```

**Platform Support:**
- **macOS**: Uses Accessibility APIs for text input simulation
- **Linux**: Uses X11/Wayland for keyboard event injection
- **Windows**: Uses Windows API for keyboard input

**Character-by-Character Typing:**
The `enigo` library types text character by character, which means:
- Unicode characters are supported (including emoji)
- Special characters are properly escaped
- Typing speed is controlled by the underlying OS keyboard repeat rate
- No clipboard is used (pure keyboard simulation)

#### Configuration Example

**Simple Text:**
```toml
[[modes.mappings]]
description = "Type email address"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Text"
text = "user@example.com"
```

**Multi-line Code Snippet:**
```toml
[[modes.mappings]]
description = "Type Rust match statement"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Text"
text = """
match value {
    Some(x) => println!(\"Got: {}\", x),
    None => println!(\"Got nothing\"),
}
"""
```

**Unicode and Emoji:**
```toml
[[modes.mappings]]
description = "Type formatted text with emoji"
[modes.mappings.trigger]
type = "Note"
note = 3
[modes.mappings.action]
type = "Text"
text = "✅ Task completed! 🎉"
```

**Special Characters:**
```toml
[[modes.mappings]]
description = "Type special characters"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "Text"
text = "!@#$%^&*()_+-=[]{}|;':\",./<>?"
```

**Combining with Sequence for Delayed Typing:**
```toml
[[modes.mappings]]
description = "Type username with delay for slow UIs"
[modes.mappings.trigger]
type = "Note"
note = 5
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Text", text = "user" },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "name123" }
]
```

#### Use Cases

1. **Email Signatures**: Type full email signatures with formatting
2. **Code Snippets**: Insert boilerplate code, function templates, or common patterns
3. **Text Expansion**: Abbreviation expansion for commonly used phrases
4. **Form Filling**: Auto-fill form fields with saved data
5. **Templates**: Insert document templates or structured text
6. **Internationalization**: Type text in multiple languages with proper character sets

#### Edge Cases

**Unicode Support:**
- Full UTF-8 support via enigo
- Emoji render correctly (✅ 🎉 💻)
- International characters work (é, ñ, 中文, 日本語)
- Right-to-left text (العربية, עברית) supported

**Escape Sequences:**
- Newlines (`\n`) in multiline TOML strings create actual line breaks
- Tabs (`\t`) create tab characters
- Quotes must be escaped in TOML (`\"` for double quotes)
- Backslashes must be doubled (`\\` for single backslash)

**TOML String Formats:**
```toml
# Basic string (single line, escaped quotes)
text = "Hello \"World\""

# Multiline string (literal newlines, escaped quotes)
text = """
Line 1
Line 2 with \"quotes\"
"""

# Literal string (no escaping, single quotes)
text = 'C:\Users\Documents\file.txt'
```

**Clipboard Interaction:**
- Does NOT use clipboard (clipboard contents unchanged)
- Types character-by-character via keyboard simulation
- More reliable than clipboard paste for Unicode

**Input Focus:**
- Requires active text field/input area
- No validation of focus state
- Text will type wherever cursor is currently active
- Recommendation: Use with `Delay` or `Keystroke` to ensure focus

**Timing Sensitivity:**
- Fast typing may overwhelm some UIs (web forms, remote desktop)
- Use `Sequence` with `Delay` actions to slow down typing if needed
- Example: Type slowly for laggy SSH connections or remote UIs

**Special Keys:**
- For modifier keys (Ctrl, Cmd, Alt, Shift), use `Keystroke` action instead
- For navigation keys (arrows, Enter, Tab), use `Keystroke` action
- Text action is for literal character insertion only

**Maximum Length:**
- No enforced limit on string length
- Very long strings (>10,000 chars) may cause UI lag
- Consider breaking up large text blocks with delays

**Keyboard Layout:**
- Typing respects active keyboard layout
- Characters typed may differ if non-US layout active
- Special characters (e.g., @, #, $) may vary by layout

#### Troubleshooting

**Text Not Typing:**
- Ensure target application has input focus
- Check if application accepts keyboard input (some secure fields block automation)
- Verify Input Monitoring permissions (macOS: System Settings > Privacy & Security)

**Unicode Characters Not Working:**
- Verify text is valid UTF-8 in config.toml
- Check if target application supports Unicode input
- On some terminals, set `LANG=en_US.UTF-8` environment variable

**Special Characters Wrong:**
- Escape special characters in TOML strings (`\"` for quotes, `\\` for backslash)
- Use TOML literal strings (`'...'`) to avoid escaping
- Use multiline strings (`"""..."""`) for complex text

**Text Types Too Fast:**
- Wrap in `Sequence` with `Delay` actions between text chunks
- Example: `[{type="Text", text="slow"}, {type="Delay", ms=100}]`

**Text Types in Wrong Application:**
- Add a focus-setting keystroke before text action
- Use `Launch` action to ensure correct app is frontmost
- Add delay after app launch to ensure it's ready

#### Testing Criteria

Implementation Status: ✅ Fully implemented (v0.1.0)

- ✅ Simple ASCII text types correctly
- ✅ Unicode characters (emoji, accents, CJK) type correctly
- ✅ Multiline text with newlines renders correctly
- ✅ Special characters (!@#$%^&*) type correctly
- ✅ Empty string doesn't cause errors (no-op)
- ✅ Very long strings (>1000 chars) complete successfully
- ✅ TOML escaping rules apply correctly
- ✅ Does not modify clipboard contents
- ✅ Works across different keyboard layouts
- ✅ Types at consistent speed determined by OS

#### Related Features

- **F10: Keystroke Action** - For modifier keys and special key combinations
- **F13: Sequence Action** - For combining Text with delays or other actions
- **F14: Delay Action** - For adding timing between text chunks

#### Future Enhancements (v2.0+)

- **Typing Speed Control**: Configurable delay between characters (`delay_ms` parameter)
- **Clipboard Mode**: Option to use clipboard paste instead of typing (faster, less reliable)
- **Variable Substitution**: Insert dynamic values (date, time, clipboard, environment variables)
- **Template Engine**: Support for placeholders and simple logic (e.g., `{{date}}`, `{{username}}`)
- **MIDI Learn for Text**: Record typing and save as text action
- **Macro Recording**: Record a sequence of keystrokes and convert to text

---

### F12: Launch Application Action

#### Description
Opens applications by name or path. Simple, cross-platform application launcher using system commands.

#### User Story
> As a streamer (Jordan), I want to launch OBS, Discord, and my streaming tools with one button press so I can start my stream setup quickly.

#### Technical Implementation

**Action Definition (config.rs:50):**
```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ActionConfig {
    Launch { app: String },  // App name or full path
    // ...
}
```

**Action Enum (actions.rs:16):**
```rust
#[derive(Debug, Clone)]
pub enum Action {
    Launch(String),  // Simple string-based launcher
    // ...
}
```

**Action Execution (actions.rs:83-107):**
```rust
fn launch_app(&self, app: &str) {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg(app)
            .spawn()
            .ok();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new(app)
            .spawn()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", app])
            .spawn()
            .ok();
    }
}
```

**Platform Behaviors:**
- **macOS**: Uses `open -a` command
  - Accepts application names (e.g., "Safari", "Visual Studio Code")
  - Accepts full paths to .app bundles
  - If app is running, brings it to the front
  - If app is not running, launches new instance
- **Linux**: Direct executable launch
  - Requires full path or executable in $PATH
  - No automatic .desktop file resolution
- **Windows**: Uses `cmd /C start` command
  - Accepts application names or paths
  - Uses Windows file associations

#### Configuration Example
```toml
# Launch by app name (macOS)
[[modes.mappings]]
description = "Open Logic Pro"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"

# Launch by app name (common apps)
[[modes.mappings]]
description = "Open Visual Studio Code"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

# Launch by full path (macOS .app bundle)
[[modes.mappings]]
description = "Open custom app"
[modes.mappings.trigger]
type = "Note"
note = 3
[modes.mappings.action]
type = "Launch"
app = "/Applications/Utilities/Terminal.app"

# Launch executable or script (full path)
[[modes.mappings]]
description = "Run custom script"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "Launch"
app = "/Users/username/scripts/stream-setup.sh"

# Launch multiple apps (via Sequence)
[[modes.mappings]]
description = "Start streaming suite"
[modes.mappings.trigger]
type = "Note"
note = 5
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "OBS" },
    { type = "Delay", ms = 2000 },
    { type = "Launch", app = "Discord" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Spotify" },
]
```

#### Use Cases

**1. Quick Launch DAWs and Production Tools**
```toml
# Producer workflow
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 10
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"

[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 11
[modes.mappings.action]
type = "Launch"
app = "Ableton Live"
```

**2. Development Environment Setup**
```toml
# Launch dev tools
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Visual Studio Code" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Terminal" },
    { type = "Delay", ms = 500 },
    { type = "Launch", app = "Safari" },
]
```

**3. Streaming/Recording Setup**
```toml
# One-button stream setup
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "OBS" },
    { type = "Delay", ms = 2000 },
    { type = "Launch", app = "Discord" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Spotify" },
    { type = "Delay", ms = 500 },
    { type = "Launch", app = "Elgato Stream Deck" },
]
```

**4. Context Switching**
```toml
# Switch between work contexts
[[modes.mappings]]
description = "Work mode"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Slack" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Calendar" },
]

[[modes.mappings]]
description = "Personal mode"
[modes.mappings.trigger]
type = "Note"
note = 15
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Discord" },
    { type = "Delay", ms = 1000 },
    { type = "Launch", app = "Spotify" },
]
```

#### Technical Details

**Process Spawning:**
- Uses `std::process::Command::spawn()` for non-blocking launch
- Returns immediately, doesn't wait for app to finish loading
- Errors are swallowed (`.ok()`) - fails silently

**Error Handling:**
- No validation that app exists before attempting launch
- Launch failures are silent (no user notification)
- Errors only visible in debug logs (if enabled)

**Application Detection:**
- macOS: `open -a` searches standard paths (/Applications, ~/Applications, /System/Applications)
- Linux: Relies on $PATH or absolute paths
- Windows: Uses system PATH and file associations

**Already-Running Behavior:**
- macOS: Brings existing app to front (doesn't launch duplicate)
- Linux: Typically launches new instance
- Windows: Depends on app's single-instance behavior

#### Edge Cases

**App Not Found:**
- No pre-validation - fails silently
- macOS: `open` command returns error code but action doesn't check it
- Recommendation: Test mappings manually before relying on them

**App Names with Spaces:**
- Automatically handled by Command API (no manual quoting needed)
- Examples: "Logic Pro", "Visual Studio Code", "Adobe Photoshop"

**Full vs Relative Paths:**
- Full paths work universally: `/Applications/Safari.app`
- Relative paths: Not recommended (dependent on working directory)
- App names: Platform-specific search paths

**Scripts and Executables:**
- macOS: Can launch .sh scripts if executable bit set (`chmod +x`)
- Linux: Can launch any executable in $PATH or by full path
- Windows: Can launch .exe, .bat, .cmd files

**Permissions:**
- May fail silently if app requires elevated permissions
- macOS: Apps requiring accessibility permissions may not function fully
- Linux: May require user to be in appropriate groups

**Launch Time:**
- Small apps (< 10MB): 100-500ms
- Large apps (> 100MB): 1-5 seconds
- DAWs and IDEs: 3-10 seconds
- Consider adding `Delay` actions in sequences

**Symlinks:**
- macOS: `open -a` resolves symlinks automatically
- Linux: Symlinks work if they point to valid executables
- Windows: Shortcuts (.lnk) work with `start` command

#### Implementation Notes

**Code Location:**
- Config definition: `src/config.rs:50`
- Action enum: `src/actions.rs:16`
- Execution logic: `src/actions.rs:83-107`
- Conversion: `src/actions.rs:139` (ActionConfig → Action)

**Platform Detection:**
- Uses `#[cfg(target_os = "...")]` compile-time flags
- Only one platform's code is included in final binary

**Future Enhancements (Not Yet Implemented):**
- Arguments support: `args: Vec<String>`
- Activation control: `activate: bool` (bring to front vs background)
- Error callbacks: Execute action on launch failure
- Conditional launch: "Launch only if not running"
- Bundle ID support (macOS): `bundle_id: String` for more precise targeting
- .desktop file support (Linux): Parse .desktop files for better app discovery

#### Testing Criteria

Implementation Status: ✅ Fully implemented (v0.1.0)

- ✅ Launch by app name (e.g., "Safari") works on macOS
- ✅ Launch by full path (e.g., "/Applications/Safari.app") works
- ✅ Multiple apps can be launched in sequence with delays
- ✅ App already running brings to front (macOS)
- ✅ Non-existent app fails silently without crashing
- ✅ Spaces in app name/path handled correctly
- ✅ Script execution works with full path and execute permissions
- ⏳ Arguments support (future enhancement)
- ⏳ Background launch without stealing focus (future enhancement)

#### Troubleshooting

**macOS: App Not Launching**
```bash
# Test manually with open command
open -a "Your App Name"

# Check if app exists
ls /Applications/ | grep -i "your app"

# Check system logs
log show --predicate 'process == "open"' --last 1m
```

**Linux: Executable Not Found**
```bash
# Check if executable is in PATH
which your-app

# Try full path
/usr/bin/your-app

# Check executable permissions
ls -l /path/to/your-app
chmod +x /path/to/your-app  # If needed
```

**Windows: Start Command Fails**
```cmd
REM Test manually
start "" "Your App"

# Check app associations
assoc .exe
```

**Debug Logging:**
```bash
# Enable debug mode to see launch attempts
DEBUG=1 cargo run --release 2
```

#### Related Features
- **F13: Shell Action** - Use for complex launch scenarios requiring arguments or environment setup
- **F10: Sequence Action** - Chain multiple launches with timing control
- **F19: Delay Action** - Add delays between launches for proper app initialization
- **F23: Conditional Action** (future) - Launch only if app not running

#### Performance Characteristics
- **Execution Time**: <1ms (spawn is non-blocking)
- **Launch Time**: 100ms - 10s (app-dependent)
- **Memory**: No additional memory overhead
- **CPU**: Negligible (handed off to system launcher)

#### Security Considerations
- **Path Injection**: Minimal risk (no shell expansion in Command API)
- **Privilege Escalation**: Cannot launch apps with higher privileges
- **Code Execution**: Can execute arbitrary binaries - use trusted configs only

---

### F14: Volume Control Action (Enhanced)

#### Description
Controls system volume with support for increase, decrease, mute/unmute, and setting absolute levels.

#### User Story
> As any user, I want to control system volume directly from my controller so I don't need to reach for keyboard or mouse.

#### Use Cases

**Producer (Alex) - Mixing and Monitoring:**
- Encoder for smooth volume adjustment while mixing
- Quick mute during client calls or unexpected interruptions
- Set volume to 50% for consistent monitoring level
- Velocity-sensitive volume (soft tap = small adjustment, hard tap = large adjustment)

**Developer (Sam) - Focus Mode:**
- Mute audio when entering deep focus
- Quick volume down when joining video calls
- Volume up when listening to music while coding
- Preset volume levels for different activities (30% for calls, 70% for music)

**Streamer (Jordan) - Live Streaming:**
- Encoder for real-time volume balancing during stream
- Quick mute for off-stream conversations
- Volume presets for different stream segments (intro music, gameplay, outro)
- Emergency mute for unexpected audio (ads, notifications)

**Presenter (Morgan) - Presentations:**
- Volume control without touching laptop during presentations
- Quick mute for audience questions
- Preset volumes for video playback vs microphone input
- Volume up/down for room acoustics adjustments

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    VolumeControl {
        operation: VolumeOperation,
        #[serde(default)]
        amount: Option<u8>,  // For Up/Down: increment (default 5)
                              // For Set: absolute level 0-100
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeOperation {
    Up,
    Down,
    Mute,
    Unmute,
    Toggle,  // Toggle mute state
    Set,     // Set to specific level
}
```

**Action Execution (actions.rs:137-180):**
```rust
Action::VolumeControl { operation, amount } => {
    #[cfg(target_os = "macos")]
    {
        match operation {
            VolumeOperation::Up => {
                let delta = amount.unwrap_or(5);
                let script = format!("set volume output volume ((output volume of (get volume settings)) + {})", delta);
                Command::new("osascript").args(&["-e", &script]).output().ok();
            }
            VolumeOperation::Down => {
                let delta = amount.unwrap_or(5);
                let script = format!("set volume output volume ((output volume of (get volume settings)) - {})", delta);
                Command::new("osascript").args(&["-e", &script]).output().ok();
            }
            VolumeOperation::Mute => {
                Command::new("osascript").args(&["-e", "set volume with output muted"]).output().ok();
            }
            VolumeOperation::Unmute => {
                Command::new("osascript").args(&["-e", "set volume without output muted"]).output().ok();
            }
            VolumeOperation::Toggle => {
                let script = "set currentMuted to output muted of (get volume settings)\nset volume output muted (not currentMuted)";
                Command::new("osascript").args(&["-e", script]).output().ok();
            }
            VolumeOperation::Set => {
                let level = amount.unwrap_or(50).min(100);
                let script = format!("set volume output volume {}", level);
                Command::new("osascript").args(&["-e", &script]).output().ok();
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Use amixer or pactl
        match operation {
            VolumeOperation::Up => {
                let delta = amount.unwrap_or(5);
                Command::new("pactl").args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("+{}%", delta)]).spawn().ok();
            }
            VolumeOperation::Down => {
                let delta = amount.unwrap_or(5);
                Command::new("pactl").args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("-{}%", delta)]).spawn().ok();
            }
            VolumeOperation::Mute => {
                Command::new("pactl").args(&["set-sink-mute", "@DEFAULT_SINK@", "1"]).spawn().ok();
            }
            VolumeOperation::Unmute => {
                Command::new("pactl").args(&["set-sink-mute", "@DEFAULT_SINK@", "0"]).spawn().ok();
            }
            VolumeOperation::Toggle => {
                Command::new("pactl").args(&["set-sink-mute", "@DEFAULT_SINK@", "toggle"]).spawn().ok();
            }
            VolumeOperation::Set => {
                let level = amount.unwrap_or(50).min(100);
                Command::new("pactl").args(&["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", level)]).spawn().ok();
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Use nircmd.exe (requires installation)
        match operation {
            VolumeOperation::Up => {
                let delta = (amount.unwrap_or(5) as f32 * 655.35) as u32;  // Convert % to 0-65535
                Command::new("nircmd.exe").args(&["changeVolume", &delta.to_string()]).spawn().ok();
            }
            // ... similar for other operations
        }
    }
}
```

#### Configuration Example
```toml
# Encoder for volume control
[[global_mappings]]
description = "Volume Up"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
amount = 2  # Small increments for smooth control

[[global_mappings]]
description = "Volume Down"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "CounterClockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Down"
amount = 2

# Mute toggle on pad
[[global_mappings]]
description = "Mute/Unmute"
[global_mappings.trigger]
type = "Note"
note = 16
[global_mappings.action]
type = "VolumeControl"
operation = "Toggle"

# Set volume to specific level
[[modes.mappings]]
description = "Set volume to 50%"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "VolumeControl"
operation = "Set"
amount = 50

# Velocity-based volume
[[modes.mappings]]
description = "Volume by velocity"
[modes.mappings.trigger]
type = "VelocityRange"
note = 9
ranges = [
    { min = 0, max = 40, action = { type = "VolumeControl", operation = "Set", amount = 25 } },
    { min = 41, max = 80, action = { type = "VolumeControl", operation = "Set", amount = 50 } },
    { min = 81, max = 127, action = { type = "VolumeControl", operation = "Set", amount = 75 } },
]
```

#### Platform Implementation Details

**macOS (AppleScript):**
```bash
# Volume control via osascript (built-in, no dependencies)
osascript -e "set volume output volume 50"             # Set to 50%
osascript -e "set volume output volume ((output volume of (get volume settings)) + 5)"  # +5%
osascript -e "set volume with output muted"            # Mute
osascript -e "set volume without output muted"         # Unmute

# Get current volume
osascript -e "output volume of (get volume settings)"  # Returns 0-100

# Get current mute state
osascript -e "output muted of (get volume settings)"   # Returns true/false
```

**Latency**: ~50-100ms per command (AppleScript overhead)
**Dependencies**: None (osascript built into macOS)
**Privileges**: No special permissions required

**Linux (PulseAudio - pactl):**
```bash
# Volume control via pactl
pactl set-sink-volume @DEFAULT_SINK@ +5%    # Relative increase
pactl set-sink-volume @DEFAULT_SINK@ -5%    # Relative decrease
pactl set-sink-volume @DEFAULT_SINK@ 50%    # Absolute set to 50%
pactl set-sink-mute @DEFAULT_SINK@ toggle   # Toggle mute
pactl set-sink-mute @DEFAULT_SINK@ 1        # Mute
pactl set-sink-mute @DEFAULT_SINK@ 0        # Unmute

# Get current volume
pactl get-sink-volume @DEFAULT_SINK@        # Returns percentage per channel

# Get current mute state
pactl get-sink-mute @DEFAULT_SINK@          # Returns "yes" or "no"
```

**Latency**: ~10-30ms per command
**Dependencies**:
- `pulseaudio` package (installed by default on Ubuntu/Fedora)
- `pactl` command-line tool (part of pulseaudio-utils)
**Privileges**: No special permissions required (user must be in `audio` group)

**Alternative (ALSA - amixer):**
```bash
# For systems without PulseAudio
amixer set Master 5%+     # Increase by 5%
amixer set Master 5%-     # Decrease by 5%
amixer set Master 50%     # Set to 50%
amixer set Master toggle  # Toggle mute
```

**Linux (Pipewire - wpctl):**
```bash
# Modern Linux systems (Fedora 34+, Ubuntu 22.10+)
wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+   # Increase
wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-   # Decrease
wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.5   # Set to 50% (0.0-1.0 range)
wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle  # Toggle mute
```

**Latency**: ~5-15ms per command (fastest option)
**Dependencies**: `pipewire` and `wireplumber` packages

**Windows (NirCmd):**
```powershell
# Volume control via nircmd.exe (requires installation)
nircmd.exe changesysvolume 3277    # Increase by ~5% (65535 * 0.05)
nircmd.exe changesysvolume -3277   # Decrease by ~5%
nircmd.exe setsysvolume 32767      # Set to ~50% (65535 / 2)
nircmd.exe mutesysvolume 1         # Mute
nircmd.exe mutesysvolume 0         # Unmute
nircmd.exe mutesysvolume 2         # Toggle mute

# Get current volume (no built-in command - use PowerShell)
powershell -Command "(New-Object -ComObject WScript.Shell).SendKeys([char]174)"  # Volume down key
```

**Latency**: ~20-50ms per command
**Dependencies**:
- `nircmd.exe` from NirSoft (download from https://www.nirsoft.net/utils/nircmd.html)
- Must be in PATH or full path specified
**Privileges**: No admin rights required for volume control

**Alternative (PowerShell with COM):**
```rust
// Using Windows audio COM API (no external dependencies)
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

pub fn set_volume_windows(level: f32) -> Result<(), WindowsError> {
    // Direct COM API calls - faster but more complex
    // Range: 0.0 to 1.0 (floating point)
}
```

**Latency**: ~5-10ms (fastest Windows option, but requires `windows` crate)

#### Dependency Installation

**Linux (Ubuntu/Debian):**
```bash
# PulseAudio (usually pre-installed)
sudo apt-get install pulseaudio pulseaudio-utils

# Or Pipewire (modern alternative)
sudo apt-get install pipewire wireplumber

# Or ALSA (minimal systems)
sudo apt-get install alsa-utils
```

**Linux (Fedora/RHEL):**
```bash
# PulseAudio
sudo dnf install pulseaudio pulseaudio-utils

# Or Pipewire (default on Fedora 34+)
sudo dnf install pipewire pipewire-utils
```

**Windows:**
```powershell
# Download nircmd.exe from https://www.nirsoft.net/utils/nircmd.html
# Extract to C:\Program Files\NirCmd\ or add to PATH

# Or install via Chocolatey
choco install nircmd
```

**Rust Implementation with Auto-Detection:**
```rust
pub enum AudioBackend {
    MacOS,           // osascript
    PulseAudio,      // pactl
    Pipewire,        // wpctl
    ALSA,            // amixer
    Windows,         // nircmd
    WindowsCOM,      // COM API (no deps)
}

impl AudioBackend {
    pub fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return AudioBackend::MacOS;

        #[cfg(target_os = "linux")]
        {
            if Command::new("pactl").arg("--version").output().is_ok() {
                return AudioBackend::PulseAudio;
            } else if Command::new("wpctl").arg("--version").output().is_ok() {
                return AudioBackend::Pipewire;
            } else if Command::new("amixer").arg("--version").output().is_ok() {
                return AudioBackend::ALSA;
            }
        }

        #[cfg(target_os = "windows")]
        {
            if Command::new("nircmd.exe").output().is_ok() {
                return AudioBackend::Windows;
            } else {
                return AudioBackend::WindowsCOM;  // Fallback to COM API
            }
        }
    }
}
```

#### Latency Characteristics Per Platform

| Platform | Method | Latency | Reliability | Notes |
|----------|--------|---------|-------------|-------|
| macOS | AppleScript | 50-100ms | ⭐⭐⭐⭐⭐ | Built-in, always works |
| Linux | pactl (PulseAudio) | 10-30ms | ⭐⭐⭐⭐ | Most common |
| Linux | wpctl (Pipewire) | 5-15ms | ⭐⭐⭐⭐⭐ | Modern systems |
| Linux | amixer (ALSA) | 15-40ms | ⭐⭐⭐ | Minimal systems |
| Windows | nircmd | 20-50ms | ⭐⭐⭐⭐ | Requires download |
| Windows | COM API | 5-10ms | ⭐⭐⭐⭐⭐ | No deps, complex |

**Recommendations:**
- **macOS**: Use AppleScript (only option, acceptable latency)
- **Linux**: Prefer Pipewire (`wpctl`) if available, fallback to PulseAudio (`pactl`)
- **Windows**: Use COM API for lowest latency, fallback to nircmd if available

#### Mute/Unmute State Management

**State Preservation:**
```rust
pub struct VolumeState {
    current_level: u8,          // 0-100
    is_muted: bool,
    pre_mute_level: Option<u8>, // Level before mute (for restore)
}

impl VolumeState {
    pub fn mute(&mut self) {
        if !self.is_muted {
            self.pre_mute_level = Some(self.current_level);
            self.is_muted = true;
            self.execute_platform_mute();
        }
    }

    pub fn unmute(&mut self) {
        if self.is_muted {
            self.is_muted = false;
            if let Some(level) = self.pre_mute_level {
                self.current_level = level;
            }
            self.execute_platform_unmute();
        }
    }

    pub fn toggle_mute(&mut self) {
        if self.is_muted {
            self.unmute();
        } else {
            self.mute();
        }
    }
}
```

**Configuration:**
```toml
[advanced_settings.volume]
sync_state_on_startup = true   # Query system volume on launch
poll_interval_ms = 1000         # Check for external volume changes
preserve_mute_level = true      # Remember volume before mute
```

#### Edge Cases
- **Platform Differences**: Uses AppleScript (macOS), pactl/wpctl/amixer (Linux), nircmd/COM (Windows)
- **Volume Range**: 0-100 on macOS/Linux, 0-65535 on Windows (converted internally)
- **Mute Independence**: Mute is separate from volume level - unmute restores previous level
- **Default Amount**: Up/Down default to 5% increments if not specified
- **Bounds Checking**: Volume clamped to 0-100 range
- **Missing Dependencies**: Linux requires pactl/wpctl/amixer, Windows requires nircmd (or use COM)
- **Latency**: AppleScript ~50-100ms, pactl ~10-30ms, wpctl ~5-15ms, nircmd ~20-50ms, COM ~5-10ms
- **External Changes**: Volume changed by other apps not reflected unless polling enabled
- **Multi-Channel**: Some systems have per-app volume - these commands affect master volume
- **Over-100%**: Some Linux systems allow >100% volume - clamped to 100 by default
- **Backend Detection**: Auto-detects available audio backend on Linux (PulseAudio/Pipewire/ALSA)

#### Testing Criteria
- ✅ Volume Up increases by specified amount
- ✅ Volume Down decreases by specified amount
- ✅ Volume stays within 0-100 bounds
- ✅ Mute toggles correctly
- ✅ Unmute restores previous volume
- ✅ Set operation sets exact level
- ✅ Default amount (5) works when not specified
- ✅ Works on target platform (macOS/Linux/Windows)
- ✅ Dependency detection works (pactl/wpctl/amixer/nircmd)
- ✅ Fallback to alternative backends on Linux works
- ✅ Latency is within expected range for platform
- ✅ State synchronization tracks external volume changes (if polling enabled)
- ✅ Pre-mute level restored correctly on unmute
- ✅ Multiple rapid volume commands don't queue up

---

### F15: Mode Change Action (Enhanced)

#### Description
Switches between different mapping modes with validation, LED feedback, and transition effects.

#### User Story
> As any user, I want to organize my mappings into modes (Default, Development, Media) so I can have different pad functions for different tasks.

#### Use Cases

**Developer (Sam) - Context Switching:**
- **Default Mode**: General productivity (browser shortcuts, window management)
- **Development Mode**: IDE shortcuts (build, test, debug, git commands)
- **Media Mode**: Audio/video controls (play/pause, volume, skip tracks)
- Encoder rotation for quick mode cycling while working

**Producer (Alex) - Production Workflow:**
- **Recording Mode**: Transport controls, record arm, input monitoring
- **Mixing Mode**: Volume faders, mute/solo, effect sends
- **Mastering Mode**: Compressor controls, EQ sweeps, limiter settings
- Chord combinations for instant mode jumps during sessions

**Streamer (Jordan) - Live Streaming:**
- **Pre-Stream Mode**: Setup checks, app launching, audio tests
- **Live Mode**: Scene switching, mute/unmute, alerts, donations
- **BRB Mode**: Limited controls, auto-mute, scene locks
- **Post-Stream Mode**: Save recordings, shutdown sequence, social media

**Designer (Taylor) - Creative Workflows:**
- **Sketch Mode**: Drawing tools, layer controls, color picker
- **Edit Mode**: Selection tools, transform, filters, masks
- **Export Mode**: Save presets, file formats, resolution settings
- Visual LED feedback shows current mode at a glance

**Presenter (Morgan) - Presentation Control:**
- **Setup Mode**: App launching, display settings, timer setup
- **Present Mode**: Slide navigation, laser pointer, annotations
- **Q&A Mode**: Microphone controls, chat moderation, polls
- **Wrap-Up Mode**: Thank you slides, contact info, survey links

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    ModeChange {
        mode: usize,  // Mode index (0-based)
        #[serde(default)]
        relative: bool,  // If true, mode is offset from current (+1, -1, etc.)
        #[serde(default)]
        transition_effect: Option<TransitionEffect>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionEffect {
    Flash,    // Quick flash
    Sweep,    // Sweep across pads
    FadeOut,  // Fade out then in
    None,
}
```

**Action Execution (actions.rs:182-195):**
```rust
Action::ModeChange { mode, relative, transition_effect } => {
    let new_mode = if *relative {
        // Relative mode change (e.g., +1 for next, -1 for previous)
        let delta = *mode as i32;
        let current = self.current_mode.load(Ordering::Relaxed) as i32;
        let num_modes = self.modes.len() as i32;
        ((current + delta).rem_euclid(num_modes)) as usize
    } else {
        // Absolute mode change
        (*mode).min(self.modes.len() - 1)
    };

    // Play transition effect if specified
    if let Some(effect) = transition_effect {
        self.feedback.play_transition_effect(effect, new_mode);
    }

    // Update mode
    let old_mode = self.current_mode.swap(new_mode, Ordering::Relaxed);

    // Update LED feedback for new mode
    let mode_color = self.modes[new_mode].color;
    self.feedback.set_mode_color(mode_color);

    debug!("Mode changed: {} -> {}", old_mode, new_mode);
}
```

#### Configuration Example
```toml
# Encoder to cycle modes (relative)
[[global_mappings]]
description = "Next Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1  # +1 offset
relative = true
transition_effect = "Sweep"

[[global_mappings]]
description = "Previous Mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = -1  # -1 offset (wraps around)
relative = true
transition_effect = "Sweep"

# Direct mode selection (absolute)
[[global_mappings]]
description = "Switch to Default Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 9]  # Top-left corners
[global_mappings.action]
type = "ModeChange"
mode = 0  # Mode 0 (Default)

[[global_mappings]]
description = "Switch to Development Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [2, 10]
[global_mappings.action]
type = "ModeChange"
mode = 1  # Mode 1 (Development)
transition_effect = "Flash"

[[global_mappings]]
description = "Switch to Media Mode"
[global_mappings.trigger]
type = "NoteChord"
notes = [3, 11]
[global_mappings.action]
type = "ModeChange"
mode = 2  # Mode 2 (Media)
transition_effect = "FadeOut"
```

#### Transition Effect Implementation

Transition effects provide visual feedback during mode changes. Each effect is implemented in the LED feedback system:

**Flash Effect (Quick Feedback):**
```rust
pub fn play_flash_effect(&mut self, new_mode: usize) {
    let new_color = self.mode_colors[new_mode];

    // 1. All pads white (50ms)
    self.set_all_pads([255, 255, 255]);
    thread::sleep(Duration::from_millis(50));

    // 2. All pads off (50ms)
    self.set_all_pads([0, 0, 0]);
    thread::sleep(Duration::from_millis(50));

    // 3. All pads new mode color (50ms)
    self.set_all_pads(new_color);
    thread::sleep(Duration::from_millis(50));

    // Total: 150ms
}
```

**Sweep Effect (Left-to-Right Wave):**
```rust
pub fn play_sweep_effect(&mut self, new_mode: usize) {
    let new_color = self.mode_colors[new_mode];
    let pad_grid = [[1,2,3,4], [5,6,7,8], [9,10,11,12], [13,14,15,16]];

    // Sweep columns left to right
    for col in 0..4 {
        for row in 0..4 {
            let pad = pad_grid[row][col];
            self.set_pad_color(pad, new_color);
        }
        thread::sleep(Duration::from_millis(30));
    }

    // Total: 120ms
}
```

**FadeOut Effect (Smooth Transition):**
```rust
pub fn play_fadeout_effect(&mut self, new_mode: usize) {
    let old_color = self.mode_colors[self.current_mode];
    let new_color = self.mode_colors[new_mode];

    // Fade out current color (10 steps, 10ms each)
    for i in (0..=10).rev() {
        let brightness = i as f32 / 10.0;
        let faded_color = [
            (old_color[0] as f32 * brightness) as u8,
            (old_color[1] as f32 * brightness) as u8,
            (old_color[2] as f32 * brightness) as u8,
        ];
        self.set_all_pads(faded_color);
        thread::sleep(Duration::from_millis(10));
    }

    // Fade in new color (10 steps, 10ms each)
    for i in 0..=10 {
        let brightness = i as f32 / 10.0;
        let faded_color = [
            (new_color[0] as f32 * brightness) as u8,
            (new_color[1] as f32 * brightness) as u8,
            (new_color[2] as f32 * brightness) as u8,
        ];
        self.set_all_pads(faded_color);
        thread::sleep(Duration::from_millis(10));
    }

    // Total: 200ms
}
```

**Spiral Effect (Center Outward):**
```rust
pub fn play_spiral_effect(&mut self, new_mode: usize) {
    let new_color = self.mode_colors[new_mode];

    // Spiral pattern from center outward
    let spiral_order = [6, 7, 11, 10, 9, 5, 1, 2, 3, 4, 8, 12, 16, 15, 14, 13];

    for &pad in &spiral_order {
        self.set_pad_color(pad, new_color);
        thread::sleep(Duration::from_millis(15));
    }

    // Total: 240ms
}
```

**Configuration:**
```toml
[advanced_settings.transitions]
enable_effects = true
flash_duration_ms = 150    # Total duration for flash effect
sweep_delay_ms = 30        # Delay per column in sweep
fadeout_steps = 10         # Number of fade steps
spiral_delay_ms = 15       # Delay per pad in spiral
```

#### LED Feedback Integration

Mode changes automatically update LED feedback to reflect the new mode's color scheme:

**Mode Color Definitions:**
```toml
[[modes]]
name = "Default"
color = "blue"             # [0, 100, 255] RGB
led_idle_brightness = 20   # Brightness when pad not pressed (0-255)
led_active_brightness = 255 # Brightness when pad pressed

[[modes]]
name = "Development"
color = "green"            # [0, 255, 0] RGB
led_idle_brightness = 30
led_active_brightness = 255

[[modes]]
name = "Media"
color = "purple"           # [200, 0, 255] RGB
led_idle_brightness = 15
led_active_brightness = 200
```

**LED Update on Mode Change:**
```rust
impl LedFeedbackController {
    pub fn on_mode_change(&mut self, old_mode: usize, new_mode: usize, transition: &TransitionEffect) {
        // 1. Play transition effect (if any)
        match transition {
            TransitionEffect::Flash => self.play_flash_effect(new_mode),
            TransitionEffect::Sweep => self.play_sweep_effect(new_mode),
            TransitionEffect::FadeOut => self.play_fadeout_effect(new_mode),
            TransitionEffect::Spiral => self.play_spiral_effect(new_mode),
            TransitionEffect::None => {},
        }

        // 2. Update mode color
        self.current_mode_color = self.mode_colors[new_mode];

        // 3. Apply new idle brightness to all pads
        let idle_brightness = self.mode_configs[new_mode].led_idle_brightness;
        self.set_idle_brightness(idle_brightness);

        // 4. Refresh all pad LEDs with new color scheme
        self.refresh_all_pads();

        // 5. Log mode change
        debug!("Mode changed: {} → {} (color: {:?})",
               self.mode_names[old_mode],
               self.mode_names[new_mode],
               self.current_mode_color);
    }
}
```

**Reactive LED Behavior Per Mode:**
```rust
// Each mode can have different reactive LED behavior
pub fn on_pad_press(&mut self, pad: u8, velocity: u8) {
    let mode_config = &self.mode_configs[self.current_mode];

    match mode_config.reactive_mode {
        ReactiveMode::VelocityColor => {
            // Different color based on velocity
            let color = self.velocity_to_color(velocity);
            self.set_pad_color(pad, color);
        }
        ReactiveMode::FixedColor => {
            // Always use mode color
            self.set_pad_color(pad, self.current_mode_color);
        }
        ReactiveMode::PulseOnPress => {
            // Pulse effect on press
            self.start_pulse_animation(pad, self.current_mode_color);
        }
    }
}
```

**LED State Persistence:**
```rust
pub struct LedState {
    current_mode: usize,
    pad_colors: HashMap<u8, [u8; 3]>,    // Current color per pad
    pad_animations: HashMap<u8, Animation>, // Active animations per pad
    mode_indicator_pads: Vec<u8>,         // Pads used to show current mode
}

impl LedState {
    // Show current mode on specific indicator pads
    pub fn update_mode_indicators(&mut self) {
        // Example: Use bottom row to show mode (1 lit pad per mode)
        for pad in &self.mode_indicator_pads {
            self.pad_colors.insert(*pad, [0, 0, 0]); // Clear all
        }

        // Light up indicator for current mode
        if let Some(&indicator_pad) = self.mode_indicator_pads.get(self.current_mode) {
            self.pad_colors.insert(indicator_pad, self.current_mode_color);
        }
    }
}
```

#### Advanced Mode Wrapping Examples

**Circular Mode Navigation:**
```toml
# Encoder for seamless mode cycling (wraps around)
[[global_mappings]]
description = "Next mode (circular)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1           # Offset +1
relative = true    # Relative to current mode
# Mode 0 +1 → Mode 1
# Mode 1 +1 → Mode 2
# Mode 2 +1 → Mode 0 (wraps)

[[global_mappings]]
description = "Previous mode (circular)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = -1          # Offset -1 (use as signed int)
relative = true
# Mode 0 -1 → Mode 2 (wraps backward)
# Mode 2 -1 → Mode 1
# Mode 1 -1 → Mode 0
```

**Mode Chords (Direct Access):**
```toml
# Press two corner pads simultaneously to jump to specific mode
[[global_mappings]]
description = "Jump to Default (Mode 0)"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 4]  # Top-left and top-right corners
[global_mappings.action]
type = "ModeChange"
mode = 0
transition_effect = "Flash"

[[global_mappings]]
description = "Jump to Development (Mode 1)"
[global_mappings.trigger]
type = "NoteChord"
notes = [13, 16]  # Bottom-left and bottom-right corners
[global_mappings.action]
type = "ModeChange"
mode = 1
transition_effect = "Sweep"
```

**Conditional Mode Change:**
```toml
# Change mode only if specific app is running
[[modes.mappings]]
description = "Switch to Media mode when Spotify running"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "Conditional"
condition = { AppRunning = { bundle_id = "com.spotify.client" } }
then_action = { type = "ModeChange", mode = 2, transition_effect = "FadeOut" }
else_action = { type = "Text", text = "Launch Spotify first!" }
```

#### Edge Cases
- **Mode Validation**: Mode index is clamped to valid range (0 to num_modes-1)
- **Relative Wrapping**: Relative mode changes wrap around using modulo (mode 2 +1 → mode 0 if only 3 modes)
- **Negative Relative**: mode=-1 with relative=true goes to previous mode (wraps backward)
- **Transition Timing**: Transition effects block briefly (Flash: 150ms, Sweep: 120ms, FadeOut: 200ms, Spiral: 240ms) before mode becomes active
- **LED Feedback**: Mode colors defined in mode config are applied automatically via `on_mode_change()`
- **Global Mappings**: Mode change mappings typically global so they work from any mode
- **Mode 0 Special**: Mode 0 is default on startup
- **Rapid Mode Changes**: Multiple rapid mode changes queue - consider debouncing
- **LED State Conflicts**: Active animations (e.g., reactive press) are cleared on mode change
- **Mode Indicators**: Optional mode indicator pads (e.g., bottom row) show current mode
- **Brightness Per Mode**: Each mode can define different idle/active brightness levels

#### Testing Criteria
- ✅ Absolute mode change (mode=1) switches to mode 1
- ✅ Relative mode change (mode=1, relative=true) cycles to next mode
- ✅ Negative relative (mode=-1, relative=true) goes to previous mode
- ✅ Mode wraps around at boundaries (last+1 → first, first-1 → last)
- ✅ Invalid mode index clamped to valid range
- ✅ LED color changes to new mode's color
- ✅ Transition effects play correctly (Flash, Sweep, FadeOut, Spiral)
- ✅ Transition durations match configuration
- ✅ Mappings from new mode become active immediately after switch
- ✅ Mode indicators update correctly
- ✅ Idle brightness changes per mode
- ✅ Active animations cleared on mode change
- ✅ Rapid mode changes handled gracefully (no corruption)

---

### F16: Action Sequence

#### Description
Executes multiple actions in order, with optional delays between actions.

#### User Story
> As a streamer (Jordan), I want one button to start my stream setup (launch OBS, wait for it to load, switch scenes, start recording) so I don't have to do each step manually.

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    Sequence {
        actions: Vec<ActionConfig>,  // List of actions to execute
        #[serde(default)]
        stop_on_error: bool,  // Stop if any action fails (default false)
    },
}
```

**Action Execution (actions.rs:197-220):**
```rust
Action::Sequence { actions, stop_on_error } => {
    debug!("Executing sequence of {} actions", actions.len());

    for (i, action) in actions.iter().enumerate() {
        debug!("Sequence step {}/{}", i + 1, actions.len());

        let result = self.execute_single(action);

        if result.is_err() && *stop_on_error {
            eprintln!("Sequence stopped at step {} due to error", i + 1);
            break;
        }
    }

    debug!("Sequence completed");
}

// Helper to execute single action
fn execute_single(&mut self, action: &Action) -> Result<(), String> {
    match action {
        Action::Keystroke { .. } => { /* ... */ Ok(()) }
        Action::Shell { .. } => { /* ... */ Ok(()) }
        // ... other actions
    }
}
```

#### Configuration Example
```toml
# Stream startup sequence
[[modes.mappings]]
description = "Start Stream Setup"
[modes.mappings.trigger]
type = "Note"
note = 1
velocity_min = 81  # Hard press
[modes.mappings.action]
type = "Sequence"
actions = [
    # Launch OBS
    { type = "Launch", app = "OBS" },

    # Wait for OBS to start
    { type = "Delay", duration_ms = 3000 },

    # Switch to streaming scene (Cmd+3)
    { type = "Keystroke", keys = "3", modifiers = ["cmd"] },

    # Wait a moment
    { type = "Delay", duration_ms = 500 },

    # Start recording (Cmd+R)
    { type = "Keystroke", keys = "r", modifiers = ["cmd"] },

    # Launch support apps in background
    { type = "Launch", app = "Discord", activate = false },
    { type = "Delay", duration_ms = 1000 },
    { type = "Launch", app = "Spotify", activate = false },

    # Send notification
    { type = "Shell", command = "osascript -e 'display notification \"Stream setup complete\" with title \"MIDIMon\"'" },
]

# Code snippet insertion sequence
[[modes.mappings]]
description = "Insert function template"
[modes.mappings.trigger]
type = "Note"
note = 5
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Text", text = "fn " },
    { type = "Delay", duration_ms = 100 },
    { type = "Text", text = "name" },
    { type = "Delay", duration_ms = 100 },
    { type = "Text", text = "() {\n    \n}" },
    { type = "Keystroke", keys = "up" },  # Move cursor up into function
]

# Multi-app window arrangement
[[modes.mappings]]
description = "Arrange workspace"
[modes.mappings.trigger]
type = "LongPress"
note = 8
hold_duration_ms = 1500
[modes.mappings.action]
type = "Sequence"
stop_on_error = true  # Stop if any window management fails
actions = [
    # Open apps
    { type = "Launch", app = "VS Code" },
    { type = "Delay", duration_ms = 2000 },

    # Tile left (Cmd+Ctrl+Left)
    { type = "Keystroke", keys = "left", modifiers = ["cmd", "ctrl"] },
    { type = "Delay", duration_ms = 500 },

    # Open browser
    { type = "Launch", app = "Safari" },
    { type = "Delay", duration_ms = 1000 },

    # Tile right
    { type = "Keystroke", keys = "right", modifiers = ["cmd", "ctrl"] },
]

# Emergency cleanup sequence
[[global_mappings]]
description = "Emergency: Close all and reset"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 2, 3, 4]  # 4-pad chord
[global_mappings.action]
type = "Sequence"
actions = [
    # Close all windows (Cmd+Opt+W)
    { type = "Keystroke", keys = "w", modifiers = ["cmd", "option"] },
    { type = "Delay", duration_ms = 1000 },

    # Reset mode to default
    { type = "ModeChange", mode = 0 },

    # Unmute audio
    { type = "VolumeControl", operation = "Unmute" },
]
```

#### Edge Cases
- **Nested Sequences**: Sequences can contain other sequences (recursive execution)
- **Delay Blocking**: Delay actions block the executor thread - use judiciously
- **Error Handling**: By default, errors are logged but sequence continues unless `stop_on_error=true`
- **Timing Sensitive**: Actions execute immediately after previous completes - use Delay for precise timing
- **Maximum Depth**: No enforced limit on sequence length or nesting depth
- **Action Context**: All actions in sequence share same context (mode, state, etc.)
- **LED Feedback**: Only first action in sequence triggers LED feedback
- **Interruption**: Sequences cannot be interrupted mid-execution (blocking)

#### Testing Criteria
- ✅ All actions in sequence execute in order
- ✅ Delays are respected and timing is accurate
- ✅ Nested sequences work correctly
- ✅ `stop_on_error=true` stops on first error
- ✅ `stop_on_error=false` continues after errors
- ✅ Empty sequence doesn't cause errors
- ✅ Very long sequences (>20 actions) complete successfully
- ✅ Sequences with mixed action types work

---

### F18: MouseClick Action

#### Description
Simulates mouse button clicks (left, right, middle) at current cursor position or specific coordinates.

#### User Story
> As a user, I want to trigger mouse clicks from my controller so I can interact with applications without touching the mouse.

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    MouseClick {
        button: MouseButton,
        #[serde(default)]
        click_type: ClickType,  // Single, Double, Triple
        #[serde(default)]
        x: Option<i32>,  // Optional absolute X position
        #[serde(default)]
        y: Option<i32>,  // Optional absolute Y position
        #[serde(default)]
        move_first: bool,  // Move to coordinates before clicking
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClickType {
    Single,
    Double,
    Triple,
}
```

**Action Execution (actions.rs:237-260):**
```rust
Action::MouseClick { button, click_type, x, y, move_first } => {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Move mouse to coordinates if specified
    if *move_first {
        if let (Some(x_pos), Some(y_pos)) = (x, y) {
            enigo.move_mouse(*x_pos, *y_pos, Abs).ok();
            thread::sleep(Duration::from_millis(50));  // Brief delay after move
        }
    }

    // Perform click(s)
    let button_enum = match button {
        MouseButton::Left => EnigoButton::Left,
        MouseButton::Right => EnigoButton::Right,
        MouseButton::Middle => EnigoButton::Middle,
    };

    let click_count = match click_type {
        ClickType::Single => 1,
        ClickType::Double => 2,
        ClickType::Triple => 3,
    };

    for i in 0..click_count {
        enigo.button(button_enum, Click).ok();
        if i < click_count - 1 {
            thread::sleep(Duration::from_millis(50));  // Delay between clicks
        }
    }

    debug!("Mouse {:?} {:?}", click_type, button);
}
```

#### Configuration Example
```toml
# Simple left click at current position
[[modes.mappings]]
description = "Left click"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "MouseClick"
button = "Left"

# Double-click
[[modes.mappings]]
description = "Double-click to open"
[modes.mappings.trigger]
type = "DoubleTap"
note = 2
max_interval_ms = 300
[modes.mappings.action]
type = "MouseClick"
button = "Left"
click_type = "Double"

# Right-click for context menu
[[modes.mappings]]
description = "Right-click context menu"
[modes.mappings.trigger]
type = "Note"
note = 3
[modes.mappings.action]
type = "MouseClick"
button = "Right"

# Click at specific coordinates (for UI automation)
[[modes.mappings]]
description = "Click 'Save' button"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "MouseClick"
button = "Left"
x = 1200
y = 800
move_first = true

# Sequence: Move and click multiple positions
[[modes.mappings]]
description = "Click through wizard"
[modes.mappings.trigger]
type = "LongPress"
note = 5
hold_duration_ms = 1000
[modes.mappings.action]
type = "Sequence"
actions = [
    # Click Next button
    { type = "MouseClick", button = "Left", x = 1000, y = 700, move_first = true },
    { type = "Delay", duration_ms = 500 },

    # Click Next again
    { type = "MouseClick", button = "Left", x = 1000, y = 700, move_first = true },
    { type = "Delay", duration_ms = 500 },

    # Click Finish
    { type = "MouseClick", button = "Left", x = 1000, y = 700, move_first = true },
]
```

#### Edge Cases
- **Cursor Position**: If no coordinates specified, clicks at current cursor position
- **Coordinates**: x/y are absolute screen coordinates (top-left is 0,0)
- **Multi-Monitor**: Coordinates extend across all monitors (continuous coordinate space)
- **Move Timing**: 50ms delay after mouse move before click (some UIs need this)
- **Double-Click Timing**: 50ms delay between clicks in multi-click (standard double-click speed)
- **Platform Differences**: Coordinate systems may differ (macOS has origin at top-left)
- **Permissions**: May require accessibility permissions on macOS
- **Click-Hold**: For drag operations, use combination of button down/up actions

#### Testing Criteria
- ✅ Left click executes at current position
- ✅ Right click opens context menu
- ✅ Middle click works (if mouse has middle button)
- ✅ Double-click triggers double-click action in UI
- ✅ Triple-click selects line/paragraph
- ✅ Click at coordinates moves cursor then clicks
- ✅ Multi-monitor coordinate system works correctly
- ✅ Timing between clicks appropriate for UI recognition

---

### F17: Delay Action

#### Description
Introduces a configurable pause in action execution, enabling precise timing control in action sequences.

#### User Story
> As a streamer (Jordan), I want to add delays between actions in my stream setup sequence so that applications have time to fully load before the next action executes.

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    Delay {
        duration_ms: u64,  // Delay duration in milliseconds
    },
}
```

**Action Execution (actions.rs:222-235):**
```rust
Action::Delay { duration_ms } => {
    debug!("Delaying for {}ms", duration_ms);

    thread::sleep(Duration::from_millis(*duration_ms));

    debug!("Delay completed");
}
```

#### Configuration Example
```toml
# Simple delay in sequence
[[modes.mappings]]
description = "Launch app with startup delay"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Launch", app = "Logic Pro" },
    { type = "Delay", duration_ms = 3000 },  # Wait 3 seconds
    { type = "Keystroke", keys = "n", modifiers = ["cmd"] },  # New project
]

# Multiple delays for complex timing
[[modes.mappings]]
description = "Timed notification sequence"
[modes.mappings.trigger]
type = "LongPress"
note = 5
hold_duration_ms = 2000
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Text", text = "Starting countdown..." },
    { type = "Delay", duration_ms = 1000 },
    { type = "Text", text = " 3..." },
    { type = "Delay", duration_ms = 1000 },
    { type = "Text", text = " 2..." },
    { type = "Delay", duration_ms = 1000 },
    { type = "Text", text = " 1..." },
    { type = "Delay", duration_ms = 1000 },
    { type = "Text", text = " GO!" },
]

# UI automation with precise timing
[[modes.mappings]]
description = "Multi-step form fill"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "Sequence"
actions = [
    # Fill first field
    { type = "Text", text = "username@example.com" },
    { type = "Delay", duration_ms = 200 },

    # Tab to next field
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", duration_ms = 100 },

    # Fill password
    { type = "Text", text = "password123" },
    { type = "Delay", duration_ms = 200 },

    # Submit form
    { type = "Keystroke", keys = "return" },
]

# Avoid rapid-fire actions
[[modes.mappings]]
description = "Throttled key presses"
[modes.mappings.trigger]
type = "Note"
note = 10
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "down" },
    { type = "Delay", duration_ms = 50 },
    { type = "Keystroke", keys = "down" },
    { type = "Delay", duration_ms = 50 },
    { type = "Keystroke", keys = "down" },
]
```

#### Edge Cases
- **Blocking Behavior**: Delay blocks the executor thread - no other actions execute during delay
- **Minimum Duration**: 0ms is valid (no-op), useful for conditional sequences
- **Maximum Duration**: No enforced maximum, but very long delays (>60s) may appear as hangs
- **Accuracy**: Sleep accuracy varies by OS (typically ±1-10ms)
- **Interruption**: Delays cannot be interrupted or cancelled mid-execution
- **Sequence Context**: Delays only make sense within Sequence actions
- **Performance Impact**: Multiple short delays (<10ms) may accumulate timing errors
- **User Experience**: Delays >2000ms should have visual feedback (UI loading state)

#### Testing Criteria
- ✅ 100ms delay pauses execution for ~100ms (±10ms tolerance)
- ✅ 1000ms delay pauses execution for ~1000ms (±20ms tolerance)
- ✅ Zero delay (0ms) does not cause errors
- ✅ Very long delay (10000ms) completes successfully
- ✅ Sequence with multiple delays executes in correct order
- ✅ Delays in nested sequences work correctly
- ✅ Action after delay executes immediately when delay ends
- ✅ Measured duration matches configured duration within tolerance

---

### F19: Repeat Action

#### Description
Repeats a specified action or sequence multiple times, enabling automation of repetitive tasks.

#### User Story
> As a power user (Casey), I want to repeat an action 10 times without defining it 10 times in my config so I can automate repetitive workflows efficiently.

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    Repeat {
        action: Box<ActionConfig>,  // Action to repeat
        count: u32,                  // Number of times to repeat
        #[serde(default)]
        delay_between_ms: Option<u64>,  // Optional delay between repetitions
        #[serde(default)]
        stop_on_error: bool,  // Stop if repetition fails (default false)
    },
}
```

**Action Execution (actions.rs:262-280):**
```rust
Action::Repeat { action, count, delay_between_ms, stop_on_error } => {
    debug!("Repeating action {} times", count);

    for i in 0..*count {
        debug!("Repetition {}/{}", i + 1, count);

        let result = self.execute_single(action);

        if result.is_err() && *stop_on_error {
            eprintln!("Repeat stopped at iteration {} due to error", i + 1);
            break;
        }

        // Delay between repetitions (except after last)
        if i < count - 1 {
            if let Some(delay) = delay_between_ms {
                thread::sleep(Duration::from_millis(*delay));
            }
        }
    }

    debug!("Repeat completed");
}
```

#### Configuration Example
```toml
# Simple repeat: Press down arrow 10 times
[[modes.mappings]]
description = "Scroll down 10 lines"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "down" }

# Repeat with delay: Slow key presses
[[modes.mappings]]
description = "Slow scroll through list"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 200  # 200ms between each press
action = { type = "Keystroke", keys = "down" }

# Repeat complex action: Multiple clicks
[[modes.mappings]]
description = "Click button 5 times"
[modes.mappings.trigger]
type = "DoubleTap"
note = 3
max_interval_ms = 300
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 500
action = { type = "MouseClick", button = "Left" }

# Repeat sequence: Complex workflow automation
[[modes.mappings]]
description = "Batch process items"
[modes.mappings.trigger]
type = "LongPress"
note = 5
hold_duration_ms = 1500
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 1000
action = {
    type = "Sequence",
    actions = [
        { type = "Keystroke", keys = "down" },  # Select next item
        { type = "Delay", duration_ms = 100 },
        { type = "Keystroke", keys = "return" },  # Open item
        { type = "Delay", duration_ms = 500 },
        { type = "Keystroke", keys = "e", modifiers = ["cmd"] },  # Edit
        { type = "Delay", duration_ms = 200 },
        { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close
    ]
}

# Repeat with error handling
[[modes.mappings]]
description = "Try launching app 3 times"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
stop_on_error = false  # Continue even if app already running
action = { type = "Launch", app = "Xcode" }

# Velocity-based repeat count
[[modes.mappings]]
description = "Repeat based on velocity"
[modes.mappings.trigger]
type = "VelocityRange"
note = 10
ranges = [
    { min = 0, max = 40, action = { type = "Repeat", count = 3, action = { type = "Keystroke", keys = "down" } } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "down" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "down" } } },
]
```

#### Edge Cases
- **Zero Count**: `count = 0` is valid (no-op), action never executes
- **Single Count**: `count = 1` executes action once (no delay_between_ms applied)
- **Very Large Count**: No enforced maximum, but count >1000 may appear as hang or cause issues
- **Nested Repeats**: Repeat can contain Repeat (multiplicative effect: 10 × 10 = 100 executions)
- **Error Propagation**: Errors logged but ignored unless `stop_on_error = true`
- **Delay After Last**: No delay after final repetition (i < count - 1 check)
- **Blocking**: Entire repeat blocks executor thread - no interruption possible
- **Memory**: Large repeat counts with complex actions may consume significant memory
- **Performance**: Rapid repeats without delay may overwhelm target application

#### Testing Criteria
- ✅ Action repeats exact number of times specified
- ✅ `count = 0` does not execute action
- ✅ `count = 1` executes action once without delay
- ✅ Delay between repetitions is applied correctly
- ✅ No delay occurs after final repetition
- ✅ `stop_on_error = true` stops on first error
- ✅ `stop_on_error = false` continues after errors
- ✅ Nested repeats multiply correctly (count × count)
- ✅ Repeated sequences execute in correct order
- ✅ Very large count (100+) completes successfully

---

### F20: Conditional Action

#### Description
Executes actions based on runtime conditions such as application state, time of day, modifier keys, or custom variables.

#### User Story
> As a developer (Sam), I want different actions during work hours vs. personal time, so my controller adapts to my context without manual mode switching.

#### Technical Implementation

**Action Definition (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    Conditional {
        conditions: Vec<Condition>,
        #[serde(default)]
        operator: ConditionOperator,  // AND, OR (default AND)
        then_action: Box<ActionConfig>,
        #[serde(default)]
        else_action: Option<Box<ActionConfig>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    AppRunning { bundle_id: String },
    AppNotRunning { bundle_id: String },
    TimeRange { start: String, end: String },  // HH:MM format
    DayOfWeek { days: Vec<String> },  // "Mon", "Tue", etc.
    ModifierPressed { modifier: Modifier },  // Shift, Ctrl, Cmd, Option
    ModeActive { mode: usize },
    // Future: Variable { name: String, value: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    And,  // All conditions must be true
    Or,   // At least one condition must be true
}
```

**Action Execution (actions.rs:282-320):**
```rust
Action::Conditional { conditions, operator, then_action, else_action } => {
    // Evaluate all conditions
    let results: Vec<bool> = conditions.iter()
        .map(|cond| self.evaluate_condition(cond))
        .collect();

    // Apply operator
    let condition_met = match operator {
        ConditionOperator::And => results.iter().all(|&r| r),
        ConditionOperator::Or => results.iter().any(|&r| r),
    };

    debug!("Conditional: {} conditions, operator {:?}, result: {}",
           conditions.len(), operator, condition_met);

    // Execute appropriate action
    if condition_met {
        self.execute_single(then_action)?;
    } else if let Some(else_act) = else_action {
        self.execute_single(else_act)?;
    }
}

fn evaluate_condition(&self, condition: &Condition) -> bool {
    match condition {
        Condition::AppRunning { bundle_id } => {
            self.is_app_running(bundle_id)
        }
        Condition::AppNotRunning { bundle_id } => {
            !self.is_app_running(bundle_id)
        }
        Condition::TimeRange { start, end } => {
            self.is_time_in_range(start, end)
        }
        Condition::DayOfWeek { days } => {
            self.is_today_in_days(days)
        }
        Condition::ModifierPressed { modifier } => {
            self.is_modifier_pressed(modifier)
        }
        Condition::ModeActive { mode } => {
            self.current_mode.load(Ordering::Relaxed) == *mode
        }
    }
}
```

#### Configuration Example
```toml
# App-based conditional: Different actions for different apps
[[modes.mappings]]
description = "Context-aware play/pause"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.apple.Logic" }
]
then_action = { type = "Keystroke", keys = "space" }  # Logic: Play/Pause
else_action = { type = "Keystroke", keys = "space", modifiers = ["cmd"] }  # System: Play/Pause

# Time-based conditional: Work hours vs personal time
[[modes.mappings]]
description = "Time-aware launcher"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" },
    { type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
]
operator = "And"
then_action = { type = "Launch", app = "Slack" }  # Work app
else_action = { type = "Launch", app = "Discord" }  # Personal app

# Multiple conditions with OR: Launch if either app is running
[[modes.mappings]]
description = "Open project in active IDE"
[modes.mappings.trigger]
type = "Note"
note = 3
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.microsoft.VSCode" },
    { type = "AppRunning", bundle_id = "com.jetbrains.IntelliJ" }
]
operator = "Or"
then_action = { type = "Keystroke", keys = "o", modifiers = ["cmd"] }  # Open file
else_action = { type = "Launch", app = "Visual Studio Code" }  # Launch IDE first

# Modifier-based conditional: Shift for variant behavior
[[modes.mappings]]
description = "Delete or force-delete"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "ModifierPressed", modifier = "Shift" }
]
then_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd", "option"] }  # Force delete
else_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd"] }  # Normal delete

# Mode-based conditional: Different actions per mode
[[global_mappings]]
description = "Mode-aware shortcut"
[global_mappings.trigger]
type = "Note"
note = 5
[global_mappings.action]
type = "Conditional"
conditions = [
    { type = "ModeActive", mode = 1 }  # Development mode
]
then_action = { type = "Shell", command = "npm test" }
else_action = { type = "Keystroke", keys = "f5" }  # Refresh in other modes

# Complex nested conditional: Multi-level decision tree
[[modes.mappings]]
description = "Smart launcher with fallbacks"
[modes.mappings.trigger]
type = "LongPress"
note = 8
hold_duration_ms = 1000
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" }
]
then_action = {
    type = "Conditional",
    conditions = [
        { type = "AppRunning", bundle_id = "com.apple.Logic" }
    ],
    then_action = { type = "Keystroke", keys = "n", modifiers = ["cmd"] },  # New Logic project
    else_action = { type = "Launch", app = "Logic Pro" }
}
else_action = {
    type = "Conditional",
    conditions = [
        { type = "DayOfWeek", days = ["Sat", "Sun"] }
    ],
    then_action = { type = "Launch", app = "Spotify" },  # Weekend music
    else_action = { type = "Launch", app = "Safari" }  # Evening browsing
}

# App not running check
[[modes.mappings]]
description = "Launch only if not already running"
[modes.mappings.trigger]
type = "Note"
note = 10
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppNotRunning", bundle_id = "com.spotify.client" }
]
then_action = { type = "Launch", app = "Spotify" }
else_action = { type = "Keystroke", keys = "space" }  # Already running, just play/pause
```

#### Edge Cases
- **App Detection Performance**: Checking running apps has ~10-50ms overhead per check
- **Time Zone**: Uses system local time, not UTC
- **Time Range Wrapping**: Ranges crossing midnight require two separate conditions
- **Modifier Detection**: Polling-based, may miss very brief modifier presses
- **Mode Check**: Race condition if mode changes during evaluation
- **Empty Conditions**: Zero conditions defaults to true (then_action always executes)
- **No Else Action**: Valid, simply does nothing if conditions fail
- **Nested Depth**: No enforced limit on nesting, but deep nesting impacts readability
- **Variable System**: Planned but not yet implemented (placeholder in types)
- **App Bundle ID**: macOS-specific, different identifier systems on Linux/Windows

#### Testing Criteria
- ✅ Single condition (AppRunning) evaluates correctly
- ✅ TimeRange condition respects work hours
- ✅ DayOfWeek condition matches current day
- ✅ Modifier detection works for Shift, Cmd, Ctrl, Option
- ✅ Multiple conditions with AND operator (all must be true)
- ✅ Multiple conditions with OR operator (at least one true)
- ✅ then_action executes when conditions met
- ✅ else_action executes when conditions not met
- ✅ No else_action does nothing when conditions fail
- ✅ Nested conditionals work correctly (decision trees)
- ✅ Empty conditions list defaults to true
- ✅ App detection works for common applications

---

### F21: Multi-Mode System

#### Description
Organizes mappings into multiple independent modes, allowing users to switch contexts and have different pad functions for different tasks without reconfiguring.

#### User Story
> As any user, I want to organize my controller mappings into modes (Default, Development, Media) so that the same physical pads can serve different purposes in different contexts.

#### Technical Implementation

**Mode Structure (config.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub color: Color,  // LED theme for this mode
    pub mappings: Vec<MappingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<MappingConfig>,
    // ...
}
```

**Mode Management (mappings.rs:25-50):**
```rust
pub struct MappingEngine {
    modes: Vec<CompiledMode>,
    global_mappings: Vec<CompiledMapping>,
    current_mode: Arc<AtomicUsize>,
}

impl MappingEngine {
    pub fn get_action_for_processed(&self, event: &ProcessedEvent) -> Option<Action> {
        let mode_idx = self.current_mode.load(Ordering::Relaxed);

        // Check global mappings first (always active)
        if let Some(action) = self.match_global(event) {
            return Some(action);
        }

        // Check current mode mappings
        if let Some(mode) = self.modes.get(mode_idx) {
            if let Some(action) = mode.match_event(event) {
                return Some(action);
            }
        }

        None
    }

    pub fn switch_mode(&mut self, new_mode: usize) {
        let clamped = new_mode.min(self.modes.len() - 1);
        self.current_mode.store(clamped, Ordering::Relaxed);
    }
}
```

**Mode Colors (feedback.rs):**
```rust
// Each mode has a distinct LED color theme
const MODE_COLORS: [(u8, u8, u8); 3] = [
    (0, 0, 255),    // Mode 0: Blue (Default)
    (0, 255, 0),    // Mode 1: Green (Development)
    (255, 0, 255),  // Mode 2: Purple (Media)
];
```

#### Configuration Example
```toml
# Mode 0: Default - General productivity
[[modes]]
name = "Default"
color = { r = 0, g = 0, b = 255 }  # Blue

[[modes.mappings]]
description = "Launch Safari"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Launch"
app = "Safari"

[[modes.mappings]]
description = "Volume Up"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "VolumeControl"
operation = "Up"

# Mode 1: Development - Code & build shortcuts
[[modes]]
name = "Development"
color = { r = 0, g = 255, b = 0 }  # Green

[[modes.mappings]]
description = "Run Tests"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Shell"
command = "npm test"

[[modes.mappings]]
description = "Git Commit"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Shell"
command = "git commit"

# Mode 2: Media - Playback control
[[modes]]
name = "Media"
color = { r = 255, g = 0, b = 255 }  # Purple

[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Next Track"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "Keystroke"
keys = "right"
modifiers = ["cmd"]
```

#### Edge Cases
- **Mode Persistence**: Current mode resets to 0 on restart (no state persistence yet)
- **Mode Index Bounds**: Invalid mode index is clamped to valid range
- **Empty Mode**: Mode with zero mappings is valid (pads do nothing)
- **Mode Colors**: Limited to 3 predefined colors in v0.1.0, configurable in config
- **Maximum Modes**: No enforced limit, but UI assumes ≤5 modes for reasonable UX
- **Mode Switch Latency**: Atomic operation, effectively instant (<1ms)
- **Active Pads During Switch**: Pads held during mode switch may have undefined behavior

#### Testing Criteria
- ✅ Mode 0 activates on startup
- ✅ Mode-specific mappings only work in their mode
- ✅ Global mappings work in all modes
- ✅ Mode switch changes LED color theme
- ✅ Invalid mode index clamped to valid range
- ✅ Mode switch is instant (<1ms)
- ✅ Multiple modes can be defined and switched between
- ✅ Empty mode does not cause errors

---

### F22: Global Mappings

#### Description
Mappings that work across all modes, ensuring critical functions (emergency exit, volume control, mode switching) are always accessible.

#### User Story
> As any user, I want certain functions like volume control and emergency exit to work regardless of which mode I'm in, so I never lose access to critical controls.

#### Technical Implementation

**Global Mapping Priority (mappings.rs:52-75):**
```rust
impl MappingEngine {
    pub fn get_action_for_processed(&self, event: &ProcessedEvent) -> Option<Action> {
        // PRIORITY 1: Global mappings (checked first)
        if let Some(action) = self.match_global(event) {
            debug!("Matched global mapping");
            return Some(action);
        }

        // PRIORITY 2: Mode-specific mappings
        let mode_idx = self.current_mode.load(Ordering::Relaxed);
        if let Some(mode) = self.modes.get(mode_idx) {
            if let Some(action) = mode.match_event(event) {
                debug!("Matched mode {} mapping", mode_idx);
                return Some(action);
            }
        }

        debug!("No mapping found");
        None
    }

    fn match_global(&self, event: &ProcessedEvent) -> Option<Action> {
        for mapping in &self.global_mappings {
            if mapping.trigger_matches(event) {
                return Some(mapping.action.clone());
            }
        }
        None
    }
}
```

#### Configuration Example
```toml
# Global mappings defined at top level, outside [[modes]]

# Emergency exit: Always accessible
[[global_mappings]]
description = "Emergency Exit (3-pad chord)"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 2, 3]
max_interval_ms = 150
[global_mappings.action]
type = "Shell"
command = "killall midimon"

# Volume control: Always accessible
[[global_mappings]]
description = "Volume Up (Encoder CW)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Up"
amount = 2

[[global_mappings]]
description = "Volume Down (Encoder CCW)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "CounterClockwise"
[global_mappings.action]
type = "VolumeControl"
operation = "Down"
amount = 2

# Mute toggle: Always accessible
[[global_mappings]]
description = "Mute/Unmute"
[global_mappings.trigger]
type = "Note"
note = 16
[global_mappings.action]
type = "VolumeControl"
operation = "Toggle"

# Mode switching: Always accessible
[[global_mappings]]
description = "Next Mode (Encoder 1 CW)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1
relative = true

[[global_mappings]]
description = "Previous Mode (Encoder 1 CCW)"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = -1
relative = true

# Global chord for settings
[[global_mappings]]
description = "Open Settings (Corner pads)"
[global_mappings.trigger]
type = "NoteChord"
notes = [1, 16]
[global_mappings.action]
type = "Shell"
command = "open /Applications/System\\ Preferences.app"
```

#### Edge Cases
- **Priority Override**: Global mappings always override mode-specific mappings for same trigger
- **Mode Switch During Global**: Global actions can trigger mode switches
- **Empty Globals**: Zero global mappings is valid (all mappings mode-specific)
- **Duplicate Triggers**: If global and mode mapping have same trigger, global always wins
- **Performance**: Global mappings checked on every event (minimal overhead)
- **Conflict Detection**: No automatic detection of trigger conflicts between global and mode
- **Global Sequences**: Sequences in global mappings can include mode-change actions

#### Testing Criteria
- ✅ Global mappings work in mode 0
- ✅ Global mappings work in mode 1
- ✅ Global mappings work in mode 2
- ✅ Global mapping overrides mode mapping with same trigger
- ✅ Mode switch via global mapping works correctly
- ✅ Emergency exit chord works from any mode
- ✅ Volume control works from any mode
- ✅ Zero global mappings does not cause errors

---

### F23: RGB LED Feedback (HID)

#### Description
Full RGB LED control for HID devices (Maschine Mikro MK3) via direct USB communication, enabling rich visual feedback and animations.

#### User Story
> As a producer (Alex), I want my pads to light up in different colors for different modes and show velocity feedback when I press them, so I have immediate visual confirmation of my actions.

#### Technical Implementation

**HID Protocol (mikro_leds.rs:1-100):**
```rust
use hidapi::{HidApi, HidDevice};

pub struct MikroMK3LEDs {
    device: HidDevice,
    current_colors: [Color; 16],  // 16 pads
    scheme: LightingScheme,
}

// Maschine Mikro MK3 USB IDs
const VENDOR_ID: u16 = 0x17cc;   // Native Instruments
const PRODUCT_ID: u16 = 0x1600;  // Maschine Mikro MK3

// HID Report structure
const REPORT_ID: u8 = 0x80;
const REPORT_SIZE: usize = 51;   // 1 byte report ID + 50 bytes data

impl MikroMK3LEDs {
    pub fn new() -> Result<Self> {
        let api = HidApi::new()?;

        #[cfg(target_os = "macos")]
        let device = api.open_path_with_options(
            VENDOR_ID,
            PRODUCT_ID,
            hidapi::OpenOptions::new().use_shared_device(true)  // Shared access!
        )?;

        Ok(Self {
            device,
            current_colors: [Color::BLACK; 16],
            scheme: LightingScheme::Reactive,
        })
    }

    pub fn set_pad_color(&mut self, pad: u8, color: Color) -> Result<()> {
        if pad >= 16 {
            return Err("Pad index out of range");
        }

        self.current_colors[pad as usize] = color;
        self.send_colors()
    }

    fn send_colors(&mut self) -> Result<()> {
        let mut report = [0u8; REPORT_SIZE];
        report[0] = REPORT_ID;

        // Pack 16 RGB values (3 bytes each = 48 bytes)
        for (i, color) in self.current_colors.iter().enumerate() {
            let offset = 1 + (i * 3);
            report[offset] = color.r;
            report[offset + 1] = color.g;
            report[offset + 2] = color.b;
        }

        self.device.write(&report)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0 };
    pub const PURPLE: Color = Color { r: 255, g: 0, b: 255 };
}
```

**Reactive Feedback (mikro_leds.rs:200-250):**
```rust
pub fn on_pad_press(&mut self, pad: u8, velocity: u8) {
    let color = match velocity {
        0..=40 => Color::GREEN,       // Soft
        41..=80 => Color::YELLOW,     // Medium
        81..=127 => Color::RED,       // Hard
    };

    self.set_pad_color(pad, color).ok();

    // Schedule fade-out after 1 second
    self.schedule_fade(pad, 1000);
}

pub fn on_pad_release(&mut self, pad: u8) {
    // Fade handled by scheduled timer
}
```

#### Configuration Example
```toml
[device]
name = "Mikro"
led_feedback = true  # Enable HID LED control

[led_settings]
scheme = "reactive"  # Velocity-based colors
brightness = 255     # Full brightness (0-255)
fade_duration_ms = 1000  # 1 second fade after release

# LED scheme options:
# - "off" - All LEDs disabled
# - "static" - Mode-based colors
# - "reactive" - Velocity colors (green/yellow/red)
# - "rainbow" - Rainbow cycle animation
# - "pulse" - Breathing effect
```

#### Edge Cases
- **Shared Device Access**: Uses `macos-shared-device` feature to coexist with Controller Editor
- **USB Reconnect**: Device handle becomes invalid if USB disconnects
- **Brightness Control**: Full RGB range (0-255), no global brightness in v0.1.0
- **Update Rate**: Limited to ~10fps by USB bandwidth
- **Color Accuracy**: Colors may vary slightly by LED manufacturing batch
- **Permission Required**: macOS requires Input Monitoring permission
- **Memory**: 16 pads × 3 bytes = 48 bytes per update, negligible overhead
- **Platform Specific**: HID protocol only works for Mikro MK3, other devices need MIDI fallback

#### Testing Criteria
- ✅ All 16 pads can be individually controlled
- ✅ RGB colors display correctly (red, green, blue, yellow, purple)
- ✅ Velocity-based colors work (green=soft, yellow=medium, red=hard)
- ✅ Fade-out effect occurs 1 second after release
- ✅ Mode switch changes all pad colors
- ✅ Shared device mode works with Controller Editor running
- ✅ LED updates have <100ms latency
- ✅ No LED flickering during normal operation

---

### F24: MIDI LED Feedback (Fallback)

#### Description
Basic on/off LED control via standard MIDI Note messages for devices without HID support, providing universal compatibility.

#### User Story
> As a user with a standard MIDI controller (Launchpad, APC), I want basic LED feedback so I know which pads are active, even though my device doesn't support full RGB control.

#### Technical Implementation

**MIDI Note Protocol (midi_feedback.rs:1-100):**
```rust
use midir::{MidiOutput, MidiOutputConnection};

pub struct MidiFeedback {
    connection: MidiOutputConnection,
    active_pads: [bool; 16],
}

impl MidiFeedback {
    pub fn new(port: &str) -> Result<Self> {
        let midi_out = MidiOutput::new("MIDIMon Feedback")?;
        let ports = midi_out.ports();

        let port_idx = ports.iter()
            .position(|p| midi_out.port_name(p).unwrap().contains(port))
            .ok_or("Port not found")?;

        let connection = midi_out.connect(&ports[port_idx], "feedback")?;

        Ok(Self {
            connection,
            active_pads: [false; 16],
        })
    }

    pub fn set_pad_led(&mut self, pad: u8, on: bool) -> Result<()> {
        // Map pad to MIDI note (C1 = 36, D#2 = 51 for 16 pads)
        let note = 36 + pad;

        // Note On (127) = LED on, Note Off (0) = LED off
        let velocity = if on { 127 } else { 0 };
        let msg = [0x90, note, velocity];  // Channel 1 Note On

        self.connection.send(&msg)?;
        self.active_pads[pad as usize] = on;

        Ok(())
    }

    pub fn clear_all(&mut self) -> Result<()> {
        for pad in 0..16 {
            self.set_pad_led(pad, false)?;
        }
        Ok(())
    }
}
```

**Fallback Detection (feedback.rs):**
```rust
pub fn create_feedback_device(config: &DeviceConfig) -> Box<dyn PadFeedback> {
    // Try HID first
    if let Ok(hid_device) = MikroMK3LEDs::new() {
        info!("Using HID RGB LED feedback");
        return Box::new(hid_device);
    }

    // Fallback to MIDI
    if let Ok(midi_device) = MidiFeedback::new(&config.name) {
        info!("Using MIDI LED feedback (on/off only)");
        return Box::new(midi_device);
    }

    // No feedback available
    warn!("No LED feedback available");
    Box::new(NoopFeedback)
}
```

#### Configuration Example
```toml
[device]
name = "Launchpad Mini"
led_feedback = true  # Enable MIDI LED feedback

[led_settings]
scheme = "static"  # Simple on/off (no RGB)
# Note: reactive, rainbow, etc. schemes degrade to on/off for MIDI devices

# MIDI LED mapping (device-specific)
# Most controllers use Note On velocity 127 = LED on, 0 = LED off
# Some may require specific colors via velocity values (127, 64, 32, etc.)
```

#### Edge Cases
- **No RGB Support**: Only on/off, no color or brightness control
- **Device-Specific Protocol**: Some devices use CC messages instead of Note On
- **Velocity Colors**: Some controllers map velocity to color (127=red, 64=green, etc.)
- **Note Range**: Assumes pads respond to notes 36-51 (C1-D#2)
- **Channel**: Assumes channel 1, some devices use other channels
- **Latency**: MIDI feedback has ~5-20ms latency
- **Feedback Loop**: Must not send to same port receiving input (creates MIDI loop)
- **Scheme Degradation**: Complex schemes (rainbow, pulse) simplified to on/off

#### Testing Criteria
- ✅ Pads light up when pressed
- ✅ Pads turn off when released (or after timeout)
- ✅ All 16 pads can be controlled
- ✅ No MIDI feedback loop occurs
- ✅ clear_all() turns off all LEDs
- ✅ Works with standard MIDI controllers (Launchpad, APC)
- ✅ Graceful degradation from HID to MIDI
- ✅ LED updates have <50ms latency

---

### F25: LED Lighting Schemes (10 schemes)

#### Description
Ten distinct LED animation and feedback patterns, ranging from static colors to dynamic animations, providing visual feedback and aesthetic appeal.

#### User Story
> As any user, I want different LED patterns to match my workflow or mood, from simple static colors for focused work to dynamic animations for creative sessions.

#### Technical Implementation

**Scheme Enum (feedback.rs):**
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LightingScheme {
    Off,        // All LEDs disabled
    Static,     // Mode-based static colors
    Breathing,  // Slow breathing effect (2s cycle)
    Pulse,      // Fast pulse effect (500ms cycle)
    Rainbow,    // Rainbow cycle across pads
    Wave,       // Wave pattern animation
    Sparkle,    // Random sparkles
    Reactive,   // Velocity-based response
    VuMeter,    // VU meter style (bottom-up)
    Spiral,     // Spiral pattern animation
}

impl LightingScheme {
    pub fn list_all() -> Vec<&'static str> {
        vec!["off", "static", "breathing", "pulse", "rainbow",
             "wave", "sparkle", "reactive", "vumeter", "spiral"]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(Self::Off),
            "static" => Some(Self::Static),
            "breathing" => Some(Self::Breathing),
            "pulse" => Some(Self::Pulse),
            "rainbow" => Some(Self::Rainbow),
            "wave" => Some(Self::Wave),
            "sparkle" => Some(Self::Sparkle),
            "reactive" => Some(Self::Reactive),
            "vumeter" => Some(Self::VuMeter),
            "spiral" => Some(Self::Spiral),
            _ => None,
        }
    }
}
```

**Scheme Implementations (mikro_leds.rs:200-400):**
```rust
impl MikroMK3LEDs {
    pub fn run_scheme(&mut self, scheme: LightingScheme) {
        match scheme {
            LightingScheme::Off => self.scheme_off(),
            LightingScheme::Static => self.scheme_static(),
            LightingScheme::Breathing => self.scheme_breathing(),
            LightingScheme::Pulse => self.scheme_pulse(),
            LightingScheme::Rainbow => self.scheme_rainbow(),
            LightingScheme::Wave => self.scheme_wave(),
            LightingScheme::Sparkle => self.scheme_sparkle(),
            LightingScheme::Reactive => self.scheme_reactive(),
            LightingScheme::VuMeter => self.scheme_vumeter(),
            LightingScheme::Spiral => self.scheme_spiral(),
        }
    }

    fn scheme_off(&mut self) {
        // All LEDs black
        for pad in 0..16 {
            self.set_pad_color(pad, Color::BLACK).ok();
        }
    }

    fn scheme_static(&mut self) {
        // Mode-based color (blue/green/purple)
        let mode_color = self.get_mode_color();
        for pad in 0..16 {
            self.set_pad_color(pad, mode_color).ok();
        }
    }

    fn scheme_breathing(&mut self) {
        // 2-second breathing cycle
        let t = (Instant::now() - self.start_time).as_secs_f32();
        let brightness = ((t * PI).sin() * 0.5 + 0.5) * 255.0;

        let color = Color {
            r: (brightness * 0.3) as u8,
            g: (brightness * 0.3) as u8,
            b: brightness as u8,
        };

        for pad in 0..16 {
            self.set_pad_color(pad, color).ok();
        }
    }

    fn scheme_rainbow(&mut self) {
        // Cycle through hue for each pad
        let t = Instant::now() - self.start_time;
        let offset = (t.as_secs_f32() * 60.0) as u16;  // 60 degrees/second

        for pad in 0..16 {
            let hue = (offset + (pad as u16 * 22)) % 360;  // 22.5 deg spacing
            let color = Color::from_hsv(hue, 255, 255);
            self.set_pad_color(pad, color).ok();
        }
    }

    fn scheme_reactive(&mut self) {
        // Velocity-based: green/yellow/red + fade after 1s
        // Implementation in on_pad_press/release callbacks
    }

    fn scheme_vumeter(&mut self) {
        // Bottom-up meter based on audio or activity level
        let level = self.get_activity_level();  // 0-16
        for pad in 0..16 {
            let color = if pad < level {
                Color::GREEN
            } else {
                Color::BLACK
            };
            self.set_pad_color(pad, color).ok();
        }
    }

    fn scheme_sparkle(&mut self) {
        // Random sparkles every 100ms
        if self.should_update_sparkle() {
            let pad = rand::random::<u8>() % 16;
            let color = Color::from_hsv(rand::random::<u16>() % 360, 255, 255);
            self.set_pad_color(pad, color).ok();

            // Fade after 200ms
            self.schedule_fade(pad, 200);
        }
    }
}
```

#### Configuration Example
```toml
# Command-line scheme selection
# cargo run --release 2 --led reactive

# Or in config.toml
[led_settings]
scheme = "reactive"  # Default scheme

# Scheme-specific settings (future)
[led_settings.reactive]
soft_color = { r = 0, g = 255, b = 0 }
medium_color = { r = 255, g = 255, b = 0 }
hard_color = { r = 255, g = 0, b = 0 }
fade_duration_ms = 1000

[led_settings.rainbow]
speed = 60  # degrees per second
saturation = 255
brightness = 255

[led_settings.breathing]
cycle_duration_ms = 2000
min_brightness = 0
max_brightness = 255
```

#### Edge Cases
- **Performance**: Complex schemes (rainbow, sparkle) use more CPU (~2-3%)
- **Update Rate**: Limited to 10fps to avoid USB bandwidth saturation
- **MIDI Devices**: Only off, static, reactive work (degraded to on/off)
- **Scheme Persistence**: Scheme choice not saved, resets on restart
- **Animation Sync**: No synchronization between multiple devices
- **Battery**: N/A (Mikro MK3 is USB powered)
- **Accessibility**: Flashing animations may not be suitable for photosensitive users
- **Customization**: v0.1.0 schemes are hardcoded, configurable in v2.0

#### Testing Criteria
- ✅ Off scheme disables all LEDs
- ✅ Static scheme shows solid mode color
- ✅ Breathing scheme cycles smoothly over 2 seconds
- ✅ Pulse scheme flashes at 500ms intervals
- ✅ Rainbow scheme shows distinct colors on each pad
- ✅ Wave scheme animates across pads
- ✅ Sparkle scheme shows random flashes
- ✅ Reactive scheme responds to velocity (green/yellow/red)
- ✅ VuMeter scheme displays activity level
- ✅ Spiral scheme animates in spiral pattern
- ✅ Scheme switch is instant (<100ms)
- ✅ No LED flickering or tearing

---

### F26: Device Profile Support (.ncmm3)

#### Description
Loads Native Instruments Controller Editor profiles to map physical pad positions to MIDI notes, supporting different pad pages and custom controller configurations.

#### User Story
> As a Mikro MK3 user, I want to use my existing Controller Editor profiles so that my physical pad layout matches my configuration without manual note mapping.

#### Technical Implementation

**Profile Structure (device_profile.rs:1-50):**
```rust
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceProfile {
    #[serde(rename = "NI_CONTROLLER_MAP")]
    pub controller_map: ControllerMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ControllerMap {
    pub version: String,
    pub author: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Page {
    pub name: String,  // "A", "B", "C", ..., "H"
    pub pads: Vec<PadMapping>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PadMapping {
    pub index: u8,      // Physical position (0-15)
    pub note: u8,       // MIDI note (0-127)
    pub channel: u8,    // MIDI channel (0-15)
}
```

**Profile Loading (device_profile.rs:51-150):**
```rust
impl DeviceProfile {
    pub fn load(path: &Path) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;
        let profile: DeviceProfile = from_str(&xml)?;

        info!("Loaded profile: {} pages", profile.controller_map.pages.len());
        Ok(profile)
    }

    pub fn get_note_for_pad(&self, pad_index: u8, page: &str) -> Option<u8> {
        let page = self.controller_map.pages.iter()
            .find(|p| p.name == page)?;

        page.pads.iter()
            .find(|p| p.index == pad_index)
            .map(|p| p.note)
    }

    pub fn detect_active_page(&self, recent_notes: &[u8]) -> Option<String> {
        // Find which page contains the most recent notes
        for page in &self.controller_map.pages {
            let page_notes: Vec<u8> = page.pads.iter().map(|p| p.note).collect();

            let matches = recent_notes.iter()
                .filter(|n| page_notes.contains(n))
                .count();

            if matches >= 3 {  // At least 3 matching notes
                return Some(page.name.clone());
            }
        }

        None
    }
}
```

**Integration (main.rs:50-80):**
```rust
// Load profile if specified
let profile = if let Some(profile_path) = args.profile {
    Some(DeviceProfile::load(&profile_path)?)
} else {
    None
};

// Auto-detect or use specified page
let active_page = if let Some(page) = args.pad_page {
    page
} else if let Some(prof) = &profile {
    prof.detect_active_page(&recent_notes).unwrap_or("A".to_string())
} else {
    "A".to_string()  // Default to page A
};

info!("Using pad page: {}", active_page);
```

#### Configuration Example
```bash
# Load profile and auto-detect page
cargo run --release 2 --profile ~/Downloads/mikro-template.ncmm3

# Load profile with specific page
cargo run --release 2 --profile mikro-template.ncmm3 --pad-page H

# Without profile (uses hardcoded defaults)
cargo run --release 2
```

**Profile XML Format (.ncmm3):**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<NI_CONTROLLER_MAP>
    <AUTHOR>Your Name</AUTHOR>
    <VERSION>1.0</VERSION>
    <DEVICE>Maschine Mikro MK3</DEVICE>

    <PAGE name="A">
        <PAD index="0" note="36" channel="0"/>
        <PAD index="1" note="37" channel="0"/>
        <!-- ... 14 more pads ... -->
        <PAD index="15" note="51" channel="0"/>
    </PAGE>

    <PAGE name="B">
        <PAD index="0" note="52" channel="0"/>
        <!-- ... different notes ... -->
    </PAGE>

    <!-- Pages C-H... -->
</NI_CONTROLLER_MAP>
```

#### Edge Cases
- **Missing Profile**: Falls back to hardcoded default mapping (C1-D#2, notes 36-51)
- **Invalid XML**: Profile load fails gracefully, uses defaults
- **Page Not Found**: Falls back to page A
- **Auto-Detection Failure**: Uses page A if fewer than 3 matching notes
- **Note Range**: No validation that notes are in valid range (0-127)
- **Channel Ignored**: v0.1.0 assumes channel 0, multi-channel support in v2.0
- **Profile Format**: Only supports NI .ncmm3 format, not generic MIDI mapping
- **Hot Reload**: Profile changes require restart in v0.1.0
- **Multiple Devices**: v0.1.0 supports one profile only

#### Testing Criteria
- ✅ Profile loads successfully from .ncmm3 file
- ✅ Physical pad maps to correct MIDI note per profile
- ✅ Auto-detection selects correct page after pressing pads
- ✅ Manual page selection overrides auto-detection
- ✅ Invalid profile path falls back to defaults
- ✅ Malformed XML handled gracefully
- ✅ All 8 pages (A-H) can be loaded and used
- ✅ Profile works with custom note mappings

---

## Target Features (v2.0)

The following features are planned for version 2.0 and represent the next major milestone in MIDIMon's evolution toward a user-friendly, GUI-driven MIDI mapping solution.

---

### TF1: MIDI Learn Mode

#### Description
Interactive MIDI learning system that captures incoming MIDI events and automatically generates configuration mappings, eliminating manual note number lookup and config file editing.

#### User Story
> As Sam (Developer), I want to click a "Learn" button in the GUI, press a pad on my controller, and have the mapping automatically created, so that I can configure my device without looking up MIDI note numbers in hex editors or documentation.

#### Technical Implementation

**State Machine (src/midi_learn.rs:1-250):**
```rust
pub enum LearnState {
    Idle,
    Learning { target_action: Option<ActionConfig>, timeout: Instant },
    Captured { event: MidiEvent, target_action: Option<ActionConfig> },
    Configured { mapping: MappingConfig },
}

pub struct MidiLearn {
    state: LearnState,
    timeout_duration: Duration,
    event_filter: Option<MidiEventFilter>,
}

impl MidiLearn {
    pub fn start_learning(&mut self, target_action: Option<ActionConfig>) {
        self.state = LearnState::Learning {
            target_action,
            timeout: Instant::now() + self.timeout_duration,
        };
    }

    pub fn capture_event(&mut self, event: MidiEvent) -> Result<(), LearnError> {
        if let LearnState::Learning { target_action, .. } = &self.state {
            // Filter out unwanted events (clock, active sensing, etc.)
            if self.should_ignore_event(&event) {
                return Ok(());
            }

            self.state = LearnState::Captured {
                event,
                target_action: target_action.clone(),
            };
            Ok(())
        } else {
            Err(LearnError::NotInLearningState)
        }
    }

    pub fn generate_mapping(&self) -> Result<MappingConfig, LearnError> {
        if let LearnState::Captured { event, target_action } = &self.state {
            let trigger = self.event_to_trigger(event)?;
            let action = target_action.clone()
                .ok_or(LearnError::NoActionSpecified)?;

            Ok(MappingConfig {
                description: format!("Learned: {:?}", event),
                trigger,
                action,
            })
        } else {
            Err(LearnError::NoEventCaptured)
        }
    }

    fn event_to_trigger(&self, event: &MidiEvent) -> Result<Trigger, LearnError> {
        match event {
            MidiEvent::NoteOn { note, .. } => Ok(Trigger::Note {
                note: *note,
                velocity_range: None,
            }),
            MidiEvent::ControlChange { controller, .. } => Ok(Trigger::CC {
                controller: *controller,
                value_range: None,
            }),
            MidiEvent::PitchBend { .. } => Ok(Trigger::PitchBend {
                range: None,
            }),
            // ... other event types
            _ => Err(LearnError::UnsupportedEventType),
        }
    }

    fn should_ignore_event(&self, event: &MidiEvent) -> bool {
        matches!(event,
            MidiEvent::Clock |
            MidiEvent::ActiveSensing |
            MidiEvent::NoteOff { .. }
        )
    }

    pub fn check_timeout(&mut self) -> bool {
        if let LearnState::Learning { timeout, .. } = self.state {
            if Instant::now() > timeout {
                self.cancel();
                return true;
            }
        }
        false
    }

    pub fn cancel(&mut self) {
        self.state = LearnState::Idle;
    }
}
```

**GUI Integration (src-tauri/src/learn_controller.rs:1-100):**
```rust
#[tauri::command]
async fn start_midi_learn(
    state: State<'_, AppState>,
    action: Option<ActionConfig>,
) -> Result<(), String> {
    let mut learn = state.midi_learn.lock().await;
    learn.start_learning(action);
    Ok(())
}

#[tauri::command]
async fn get_learn_state(
    state: State<'_, AppState>,
) -> Result<LearnStateResponse, String> {
    let learn = state.midi_learn.lock().await;
    Ok(learn.state.clone().into())
}

#[tauri::command]
async fn apply_learned_mapping(
    state: State<'_, AppState>,
    mode: String,
) -> Result<MappingConfig, String> {
    let mut learn = state.midi_learn.lock().await;
    let mapping = learn.generate_mapping()
        .map_err(|e| e.to_string())?;

    // Add mapping to config
    let mut config = state.config.lock().await;
    config.add_mapping(&mode, mapping.clone())?;
    config.save()?;

    learn.cancel();
    Ok(mapping)
}
```

#### Configuration Example

**Before Learn (manual config):**
```toml
[[modes.mappings]]
description = "Launch Logic Pro"
[modes.mappings.trigger]
type = "Note"
note = 12  # Had to look this up!
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"
```

**After Learn (auto-generated):**
```toml
# UI Flow:
# 1. Click "Add Mapping" button in GUI
# 2. Select action: "Launch" → "Logic Pro"
# 3. Click "Learn Trigger"
# 4. Press pad on controller
# 5. GUI shows: "Captured: Note 12"
# 6. Click "Save"
# 7. Mapping auto-generated in config.toml

[[modes.mappings]]
description = "Learned: NoteOn(note=12, velocity=80)"
[modes.mappings.trigger]
type = "Note"
note = 12  # Auto-detected!
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"
```

**Learn with Velocity Detection:**
```rust
// If user presses pad softly during learn, capture velocity range
// Captured: NoteOn(note=12, velocity=35)
// Auto-suggests: VelocityRange trigger with Soft mapping
[[modes.mappings]]
description = "Learned: Soft press on Pad 12"
[modes.mappings.trigger]
type = "VelocityRange"
note = 12
level = "Soft"  # Detected from velocity=35
[modes.mappings.action]
type = "Launch"
app = "TextEdit"
```

**Learn with CC Messages:**
```toml
# GUI: Click "Learn" → Turn encoder → Captured: CC(controller=1, value=65)
[[modes.mappings]]
description = "Learned: CC #1 (Modulation Wheel)"
[modes.mappings.trigger]
type = "CC"
controller = 1
[modes.mappings.action]
type = "VolumeControl"
action = "Up"
```

**Timeout Handling:**
```rust
// config.toml
[advanced_settings]
midi_learn_timeout_ms = 5000  # Cancel learn after 5s of inactivity

// GUI behavior:
// - Shows countdown timer: "Waiting for MIDI input... (4s)"
// - On timeout: "Learn cancelled. No MIDI event received."
// - User can click "Cancel" button to exit early
```

#### Edge Cases
- **Multiple Events During Learn**: Captures first valid event, ignores subsequent events until learn is confirmed or cancelled
- **Clock/Timing Messages**: Filters out MIDI clock (0xF8), active sensing (0xFE), and other real-time messages
- **Note Off Events**: Ignores Note Off, only captures Note On to avoid confusion
- **Duplicate Mappings**: Warns if trigger already exists in current mode before saving
- **Learn Timeout**: Automatically cancels learn after configurable timeout (default 5s)
- **Invalid Action**: Returns error if target action is not specified before starting learn
- **Multi-Channel Controllers**: Captures MIDI channel along with note/CC number
- **Concurrent Learn Sessions**: Only one learn session active at a time (mutex protected)
- **Config File Conflicts**: Validates learned mapping doesn't conflict with global mappings
- **Live Edit Mode**: Can learn while main engine is running without disrupting active mappings

#### Testing Criteria
- ✅ Start learn mode via GUI command
- ✅ Capture Note On event and generate Note trigger
- ✅ Capture CC event and generate CC trigger
- ✅ Capture PitchBend event and generate PitchBend trigger
- ✅ Ignore MIDI clock and active sensing during learn
- ✅ Timeout cancels learn after 5 seconds
- ✅ User can manually cancel learn before timeout
- ✅ Learned mapping saves to correct mode in config.toml
- ✅ Velocity detection suggests VelocityRange trigger for soft/hard presses
- ✅ Duplicate trigger warning shown before overwriting existing mapping
- ✅ MIDI channel captured for multi-channel controllers
- ✅ Learn mode disabled during learn (no concurrent sessions)

---

### TF2: Config Hot-Reload

#### Description
Automatic detection and reloading of configuration file changes without restarting the application, enabling rapid iteration and live testing of mapping adjustments.

#### User Story
> As Sam (Developer), I want to edit my config.toml file in VS Code and see changes take effect immediately, so that I can rapidly test and refine my mappings without restarting MIDIMon.

#### Technical Implementation

**File Watcher (src/config_watcher.rs:1-200):**
```rust
use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event, EventKind};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant};

pub struct ConfigWatcher {
    watcher: notify::RecommendedWatcher,
    config_path: PathBuf,
    debounce_duration: Duration,
    last_reload: Option<Instant>,
}

impl ConfigWatcher {
    pub fn new(config_path: PathBuf) -> NotifyResult<(Self, Receiver<ConfigEvent>)> {
        let (tx, rx) = channel();
        let debounce_duration = Duration::from_millis(500);

        let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
            if let Ok(event) = res {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        let _ = tx.send(ConfigEvent::Changed);
                    }
                    EventKind::Remove(_) => {
                        let _ = tx.send(ConfigEvent::Deleted);
                    }
                    _ => {}
                }
            }
        })?;

        watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

        Ok((
            Self {
                watcher,
                config_path,
                debounce_duration,
                last_reload: None,
            },
            rx,
        ))
    }

    pub fn should_reload(&mut self) -> bool {
        if let Some(last) = self.last_reload {
            if Instant::now().duration_since(last) < self.debounce_duration {
                return false; // Debounce rapid changes
            }
        }
        self.last_reload = Some(Instant::now());
        true
    }
}

pub enum ConfigEvent {
    Changed,
    Deleted,
}
```

**Reload Handler (src/main.rs:250-350):**
```rust
fn handle_config_reload(
    config_path: &Path,
    engine: &mut MappingEngine,
    led_feedback: &mut dyn PadFeedback,
) -> Result<(), ReloadError> {
    println!("🔄 Config file changed, reloading...");

    // 1. Preserve current state
    let current_mode = engine.current_mode();
    let active_notes = engine.get_active_notes(); // For held pads

    // 2. Validate new config before applying
    let new_config = match Config::load(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("❌ Config validation failed: {}", e);
            eprintln!("   Keeping previous configuration.");
            return Err(ReloadError::ValidationFailed(e));
        }
    };

    // 3. Apply new config
    engine.update_config(new_config.clone());

    // 4. Restore mode if it still exists
    if engine.has_mode(current_mode) {
        engine.set_mode(current_mode);
    } else {
        eprintln!("⚠️  Previous mode '{}' no longer exists, switching to default", current_mode);
        engine.set_mode(0);
    }

    // 5. Update LED scheme if changed
    if let Some(new_scheme) = &new_config.led_scheme {
        led_feedback.set_scheme(new_scheme.clone());
    }

    // 6. Handle active notes (avoid stuck keys)
    for note in active_notes {
        engine.handle_note_release(note);
    }

    println!("✅ Config reloaded successfully");
    led_feedback.flash_success(); // Visual confirmation

    Ok(())
}

// Main event loop integration
fn run_event_loop(/* ... */) {
    let (config_watcher, config_rx) = ConfigWatcher::new(config_path.clone())
        .expect("Failed to start config watcher");

    loop {
        select! {
            recv(midi_rx) -> msg => {
                // Handle MIDI events
            }
            recv(config_rx) -> event => {
                if let Ok(ConfigEvent::Changed) = event {
                    if config_watcher.should_reload() {
                        let _ = handle_config_reload(
                            &config_path,
                            &mut engine,
                            led_feedback.as_mut(),
                        );
                    }
                }
            }
        }
    }
}
```

#### Configuration Example

**Enable Hot-Reload (config.toml):**
```toml
[advanced_settings]
# Enable automatic config reloading (default: true)
hot_reload_enabled = true

# Debounce duration to avoid rapid reloads during editing (milliseconds)
hot_reload_debounce_ms = 500

# Validate config before applying (prevents broken configs from loading)
validate_before_reload = true
```

**Reload Workflow Example:**
```rust
// Initial config.toml:
[[modes.mappings]]
description = "Launch Chrome"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Launch"
app = "Chrome"

// --- User edits config.toml in VS Code ---
// Changes: app = "Firefox"
// Saves file (Cmd+S)

// --- MIDIMon detects change ---
// Output:
// 🔄 Config file changed, reloading...
// ✅ Config reloaded successfully
// [LED feedback flashes green]

// --- User presses pad immediately ---
// Firefox launches instead of Chrome (no restart needed!)
```

**Validation Error Handling:**
```toml
# User accidentally breaks TOML syntax:
[[modes.mappings]
description = "Broken config (missing closing bracket)"

# MIDIMon output:
# 🔄 Config file changed, reloading...
# ❌ Config validation failed: TOML parse error at line 45
#    Keeping previous configuration.
# [LED feedback flashes red]
#
# Previous config still active - no disruption!
```

**State Preservation Example:**
```rust
// Scenario: User is in Mode 1, holding pad 5 (long press in progress)
// User edits config.toml and saves

// MIDIMon behavior:
// 1. Detects active long press on pad 5
// 2. Cancels timer to avoid stuck action
// 3. Reloads config
// 4. Restores Mode 1 (if it still exists)
// 5. Releases held notes to avoid stuck keys
// 6. User can immediately press pad 5 again with new mapping
```

#### Edge Cases
- **Rapid Edits**: Debouncing (500ms default) prevents multiple reloads when editor saves temporary files
- **Invalid TOML Syntax**: Validation fails, previous config retained, error logged with line number
- **Deleted Config File**: Detects deletion, keeps previous config until file is restored
- **Mode Removed During Edit**: If current mode deleted, falls back to default mode (index 0)
- **Active Long Press**: Cancels in-progress timers to avoid stuck actions after reload
- **Held Pads**: Releases active notes to prevent stuck keys in system
- **LED Scheme Change**: Updates LED scheme immediately without restarting LED thread
- **Global Mappings Changed**: Global mappings apply immediately across all modes
- **Concurrent Edits**: Only reloads after debounce period, last change wins
- **Symlink Changes**: Follows symlinks and detects changes to linked files

#### Testing Criteria
- ✅ Config file change triggers reload automatically
- ✅ Debouncing prevents multiple reloads within 500ms
- ✅ Invalid TOML syntax rejected with error message
- ✅ Previous config retained if validation fails
- ✅ Current mode preserved after reload if mode still exists
- ✅ Falls back to default mode if current mode removed
- ✅ Active long press timers cancelled before reload
- ✅ Held notes released to prevent stuck keys
- ✅ LED scheme updates immediately on change
- ✅ Global mappings apply immediately after reload
- ✅ Config deletion detected and previous config retained
- ✅ Success/failure feedback shown via LED flash

---

### TF3: Menu Bar UI (Tauri)

#### Description
System tray/menu bar application providing quick access to status, controls, and configuration without requiring a full GUI window, following native macOS/Windows/Linux system tray patterns.

#### User Story
> As Alex (Producer), I want a menu bar icon showing MIDIMon's status with quick actions for pause/resume and reload, so that I can control the app without switching away from my DAW.

#### Technical Implementation

**Tauri Tray Implementation (src-tauri/src/tray.rs:1-200):**
```rust
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;

pub fn create_system_tray() -> SystemTray {
    let status = CustomMenuItem::new("status".to_string(), "● Active")
        .disabled(); // Non-clickable status indicator

    let pause = CustomMenuItem::new("pause".to_string(), "⏸ Pause");
    let reload = CustomMenuItem::new("reload".to_string(), "🔄 Reload Config");
    let configure = CustomMenuItem::new("configure".to_string(), "⚙️ Open Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(pause)
        .add_item(reload)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(configure)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "pause" => {
                    let state = app.state::<AppState>();
                    let mut engine = state.engine.lock().unwrap();

                    if engine.is_paused() {
                        engine.resume();
                        update_tray_item(app, "pause", "⏸ Pause");
                        update_tray_item(app, "status", "● Active");
                    } else {
                        engine.pause();
                        update_tray_item(app, "pause", "▶️ Resume");
                        update_tray_item(app, "status", "⏸ Paused");
                    }
                }
                "reload" => {
                    let state = app.state::<AppState>();
                    match reload_config(state) {
                        Ok(_) => show_notification(app, "Config reloaded successfully"),
                        Err(e) => show_notification(app, &format!("Reload failed: {}", e)),
                    }
                }
                "configure" => {
                    // Open settings window
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        SystemTrayEvent::LeftClick { .. } => {
            // macOS: Show menu on left click
            // Windows: Show window on left click
            #[cfg(target_os = "macos")]
            {} // Menu already shown by system

            #[cfg(not(target_os = "macos"))]
            {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
        }
        _ => {}
    }
}

fn update_tray_item(app: &tauri::AppHandle, id: &str, new_title: &str) {
    app.tray_handle()
        .get_item(id)
        .set_title(new_title)
        .unwrap();
}

fn show_notification(app: &tauri::AppHandle, message: &str) {
    use tauri::api::notification::Notification;

    Notification::new(&app.config().tauri.bundle.identifier)
        .title("MIDIMon")
        .body(message)
        .show()
        .ok();
}
```

**Dynamic Status Updates (src-tauri/src/main.rs:50-120):**
```rust
// Update tray status based on engine events
async fn start_status_monitor(app: tauri::AppHandle) {
    let state = app.state::<AppState>();
    let mut status_rx = state.engine_status_rx.clone();

    tokio::spawn(async move {
        while let Some(status) = status_rx.recv().await {
            match status {
                EngineStatus::Active => {
                    update_tray_item(&app, "status", "● Active");
                    update_tray_icon(&app, "active.png");
                }
                EngineStatus::Paused => {
                    update_tray_item(&app, "status", "⏸ Paused");
                    update_tray_icon(&app, "paused.png");
                }
                EngineStatus::Processing => {
                    update_tray_item(&app, "status", "⚡ Processing");
                    // Temporary visual feedback, returns to Active after 500ms
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    update_tray_item(&app, "status", "● Active");
                }
                EngineStatus::Error(msg) => {
                    update_tray_item(&app, "status", &format!("❌ Error: {}", msg));
                    update_tray_icon(&app, "error.png");
                }
            }
        }
    });
}

fn update_tray_icon(app: &tauri::AppHandle, icon_name: &str) {
    let icon_path = format!("icons/{}", icon_name);
    app.tray_handle()
        .set_icon(tauri::Icon::File(icon_path.into()))
        .ok();
}
```

#### Configuration Example

**Tauri Config (src-tauri/tauri.conf.json):**
```json
{
  "tauri": {
    "systemTray": {
      "iconPath": "icons/tray-icon.png",
      "iconAsTemplate": true,
      "menuOnLeftClick": true,
      "title": "MIDIMon"
    },
    "windows": [
      {
        "title": "MIDIMon Settings",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "visible": false,
        "decorations": true,
        "alwaysOnTop": false,
        "skipTaskbar": false
      }
    ]
  }
}
```

**Menu Bar States:**
```
Active State:
┌─────────────────────┐
│ ● Active            │ ← Green dot
├─────────────────────┤
│ ⏸ Pause             │
│ 🔄 Reload Config    │
├─────────────────────┤
│ ⚙️ Open Settings    │
├─────────────────────┤
│ Quit                │
└─────────────────────┘

Paused State:
┌─────────────────────┐
│ ⏸ Paused            │ ← Yellow dot
├─────────────────────┤
│ ▶️ Resume           │ ← Changed to resume
│ 🔄 Reload Config    │
├─────────────────────┤
│ ⚙️ Open Settings    │
├─────────────────────┤
│ Quit                │
└─────────────────────┘

Processing State (temporary):
┌─────────────────────┐
│ ⚡ Processing       │ ← Brief flash on action
├─────────────────────┤
│ ⏸ Pause             │
│ 🔄 Reload Config    │
├─────────────────────┤
│ ⚙️ Open Settings    │
├─────────────────────┤
│ Quit                │
└─────────────────────┘

Error State:
┌─────────────────────┐
│ ❌ Error: No device │ ← Red dot
├─────────────────────┤
│ ⏸ Pause             │
│ 🔄 Reload Config    │
├─────────────────────┤
│ ⚙️ Open Settings    │
├─────────────────────┤
│ Quit                │
└─────────────────────┘
```

**macOS-Specific Behavior:**
```rust
// Keep app in menu bar only (no dock icon)
#[cfg(target_os = "macos")]
fn configure_macos_app(app: &mut tauri::App) {
    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};

    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory
        );
    }
}

// Template icon adapts to light/dark mode
// iconAsTemplate: true in tauri.conf.json
// Icon automatically inverts in dark mode
```

#### Edge Cases
- **Menu Bar Icon Not Showing**: Falls back to notification-based control if tray unavailable
- **Multiple Instances**: Checks for existing instance, shows "Already running" notification
- **Lost Focus on Settings Open**: Brings settings window to front and focuses it
- **System Dark Mode Toggle**: Template icon adapts automatically without restart
- **Notification Permission Denied**: Logs errors silently, continues operation without notifications
- **Tray Icon Update Failure**: Logs error but doesn't crash, status updates continue
- **Platform-Specific Menu Order**: Respects platform conventions (macOS vs Windows vs Linux)
- **Long Status Messages**: Truncates status text to avoid menu overflow
- **Rapid Status Changes**: Debounces updates to avoid menu flicker
- **Settings Window Already Open**: Focuses existing window instead of creating duplicate

#### Testing Criteria
- ✅ Tray icon appears in system menu bar/tray
- ✅ Menu shows current status (Active/Paused/Processing/Error)
- ✅ Pause toggles to Resume and back
- ✅ Reload triggers config hot-reload
- ✅ Open Settings shows settings window and focuses it
- ✅ Quit cleanly shuts down application
- ✅ Status updates in real-time on engine events
- ✅ Notification shown on reload success/failure
- ✅ Icon adapts to macOS light/dark mode (template icon)
- ✅ Windows: Left click shows settings window
- ✅ macOS: App runs in menu bar only (no dock icon)
- ✅ Processing status briefly flashes, returns to Active

---

### TF4: Visual Config Editor

#### Description
Tauri-based graphical interface for creating and editing MIDI mappings with drag-and-drop action assignment, device visualization, and MIDI Learn integration, eliminating manual TOML editing for non-technical users.

#### User Story
> As Alex (Producer), I want to drag actions onto a visual representation of my controller and see my mappings laid out spatially, so that I can configure my device without editing text files or understanding TOML syntax.

#### Technical Implementation

**Tauri v2 Architecture (src-tauri/src/main.rs:1-80):**
```rust
use tauri::Manager;

#[tauri::command]
async fn load_config(state: State<'_, AppState>) -> Result<ConfigDto, String> {
    let config = state.config.lock().await;
    Ok(config.clone().into())
}

#[tauri::command]
async fn save_config(
    state: State<'_, AppState>,
    config: ConfigDto,
) -> Result<(), String> {
    let mut app_config = state.config.lock().await;
    *app_config = config.into();
    app_config.save().map_err(|e| e.to_string())?;

    // Trigger hot-reload
    state.reload_tx.send(()).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn add_mapping(
    state: State<'_, AppState>,
    mode: String,
    mapping: MappingDto,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.add_mapping(&mode, mapping.into())?;
    Ok(())
}

#[tauri::command]
async fn remove_mapping(
    state: State<'_, AppState>,
    mode: String,
    mapping_index: usize,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.remove_mapping(&mode, mapping_index)?;
    Ok(())
}

#[tauri::command]
async fn list_actions() -> Result<Vec<ActionTemplate>, String> {
    Ok(ActionLibrary::list_all())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let state = AppState::new();
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_config,
            save_config,
            add_mapping,
            remove_mapping,
            list_actions,
            start_midi_learn,
            get_learn_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Device Visualization Component (ui/src/components/DeviceView.svelte):**
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  export let device: Device;
  export let mappings: Mapping[];

  let pads: PadElement[] = [];
  let selectedPad: number | null = null;

  onMount(async () => {
    // Load device layout (16 pads for Maschine Mikro MK3)
    pads = await invoke('get_device_layout', { deviceName: device.name });
  });

  function handlePadClick(padIndex: number) {
    selectedPad = padIndex;
    // Show mapping editor for this pad
  }

  function getMappingForPad(padIndex: number): Mapping | null {
    const note = pads[padIndex].note;
    return mappings.find(m => m.trigger.note === note);
  }

  function getPadColor(padIndex: number): string {
    const mapping = getMappingForPad(padIndex);
    if (mapping) {
      return getActionColor(mapping.action.type);
    }
    return '#333'; // Unmapped
  }
</script>

<div class="device-view">
  <div class="pad-grid">
    {#each pads as pad, i}
      <button
        class="pad"
        class:selected={selectedPad === i}
        style="background-color: {getPadColor(i)}"
        on:click={() => handlePadClick(i)}
      >
        <span class="pad-number">{i + 1}</span>
        {#if getMappingForPad(i)}
          <span class="pad-action">
            {getMappingForPad(i).action.type}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  {#if selectedPad !== null}
    <MappingEditor
      pad={pads[selectedPad]}
      mapping={getMappingForPad(selectedPad)}
      on:save={handleMappingSave}
      on:delete={handleMappingDelete}
    />
  {/if}
</div>

<style>
  .pad-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-gap: 8px;
    max-width: 400px;
  }

  .pad {
    aspect-ratio: 1;
    border: 2px solid #555;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .pad:hover {
    transform: scale(1.05);
    border-color: #fff;
  }

  .pad.selected {
    border-color: #0af;
    box-shadow: 0 0 20px rgba(0, 170, 255, 0.5);
  }
</style>
```

**Action Library UI (ui/src/components/ActionLibrary.svelte):**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let actions: ActionTemplate[] = [];
  let searchQuery = '';

  onMount(async () => {
    actions = await invoke('list_actions');
  });

  $: filteredActions = actions.filter(a =>
    a.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    a.category.toLowerCase().includes(searchQuery.toLowerCase())
  );

  function handleDragStart(event: DragEvent, action: ActionTemplate) {
    event.dataTransfer.setData('action', JSON.stringify(action));
  }
</script>

<div class="action-library">
  <input
    type="text"
    placeholder="Search actions..."
    bind:value={searchQuery}
  />

  <div class="action-categories">
    {#each Object.entries(groupByCategory(filteredActions)) as [category, categoryActions]}
      <div class="category">
        <h3>{category}</h3>
        <div class="action-list">
          {#each categoryActions as action}
            <div
              class="action-item"
              draggable="true"
              on:dragstart={(e) => handleDragStart(e, action)}
            >
              <span class="action-icon">{action.icon}</span>
              <span class="action-name">{action.name}</span>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Example action library structure:
- Keyboard
  - Keystroke (⌨️)
  - Text (📝)
- Application
  - Launch (🚀)
  - Shell (🖥️)
- Media
  - Volume Up (🔊)
  - Volume Down (🔉)
  - Mute (🔇)
- System
  - Mode Change (🔄)
  - Delay (⏱️)
-->
```

**MIDI Learn Integration (ui/src/components/MIDILearn.svelte):**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let learnState: 'idle' | 'learning' | 'captured' = 'idle';
  let capturedEvent: MidiEvent | null = null;
  let countdown = 5;

  async function startLearn(targetAction: Action) {
    await invoke('start_midi_learn', { action: targetAction });
    learnState = 'learning';

    // Poll for learn state
    const interval = setInterval(async () => {
      const state = await invoke('get_learn_state');

      if (state.captured) {
        learnState = 'captured';
        capturedEvent = state.event;
        clearInterval(interval);
      } else if (state.timeout) {
        learnState = 'idle';
        clearInterval(interval);
      }

      countdown--;
      if (countdown <= 0) {
        learnState = 'idle';
        clearInterval(interval);
      }
    }, 1000);
  }

  async function applyLearn() {
    await invoke('apply_learned_mapping', { mode: currentMode });
    learnState = 'idle';
    // Refresh mappings
  }
</script>

{#if learnState === 'learning'}
  <div class="learn-overlay">
    <div class="learn-prompt">
      <h2>Press any button on your controller...</h2>
      <p>Timeout: {countdown}s</p>
      <button on:click={() => { learnState = 'idle'; }}>Cancel</button>
    </div>
  </div>
{:else if learnState === 'captured'}
  <div class="learn-captured">
    <h3>Captured:</h3>
    <p>Note {capturedEvent.note}, Velocity {capturedEvent.velocity}</p>
    <button on:click={applyLearn}>Apply</button>
    <button on:click={() => { learnState = 'idle'; }}>Cancel</button>
  </div>
{/if}
```

#### Configuration Example

**Workflow: Add New Mapping**
```
1. User clicks pad #5 on device visualization
2. Mapping editor shows:
   ┌─────────────────────────────┐
   │ Pad 5 (Note 12)             │
   │                             │
   │ Trigger:                    │
   │ ○ Note Press                │
   │ ○ Long Press (2000ms)       │
   │ ○ Double Tap (300ms)        │
   │ ○ Velocity Sensitive        │
   │                             │
   │ Action: [Drag here or Learn]│
   │                             │
   │ [Learn MIDI] [Save] [Delete]│
   └─────────────────────────────┘

3. User drags "Launch" action from library
4. Action dialog opens:
   ┌─────────────────────────────┐
   │ Launch Application          │
   │                             │
   │ Application: [Browse...]    │
   │ ▸ /Applications/Logic Pro.app│
   │                             │
   │ [Cancel] [Save]             │
   └─────────────────────────────┘

5. User clicks Save
6. Config auto-saves to config.toml
7. Hot-reload triggers immediately
8. Pad #5 turns blue on device visualization
```

**Generated TOML (behind the scenes):**
```toml
[[modes.mappings]]
description = "Launch Logic Pro (Pad 5)"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"
```

#### Edge Cases
- **Unsaved Changes**: Shows dialog: "You have unsaved changes. Save before closing?"
- **Invalid Action Config**: Validates fields before saving, shows inline errors
- **Conflicting Mappings**: Warns if trigger already mapped in current mode
- **MIDI Learn Timeout**: Shows "No MIDI event received" notification, returns to editor
- **Device Not Connected**: Shows "Device offline" indicator, allows editing anyway
- **Large Config Files**: Paginates mode list for configs with 10+ modes
- **Drag-Drop Outside Bounds**: Cancels drag operation, no action added
- **Live Event Feedback**: Shows visual feedback on pads when MIDI events received
- **Config Parse Error**: Shows error dialog with line number, prevents corrupted config
- **Concurrent Edits**: Detects external config changes, prompts: "Config changed externally. Reload?"

#### Testing Criteria
- ✅ Device visualization shows 16 pads in 4x4 grid
- ✅ Mapped pads show color-coded action type
- ✅ Click pad opens mapping editor
- ✅ Drag action from library onto pad creates mapping
- ✅ MIDI Learn button starts learn mode with countdown
- ✅ Captured MIDI event auto-fills trigger fields
- ✅ Save mapping writes to config.toml and triggers hot-reload
- ✅ Delete mapping removes from config and refreshes UI
- ✅ Unsaved changes dialog shown on window close
- ✅ Conflicting mapping warning shown before overwrite
- ✅ Live event console shows real-time MIDI events
- ✅ External config changes detected and prompt shown

---

### TF5: Device Template System

#### Description
Pre-configured device templates providing default layouts, MIDI mappings, and metadata for popular controllers, enabling quick setup with community-shareable template files.

#### User Story
> As Jordan (Streamer), I want to select "Native Instruments Maschine Mikro MK3" from a template list and have all 16 pads pre-mapped with starter actions, so that I can get up and running in seconds without manually configuring every pad.

#### Technical Implementation

**Template Format (config/device_templates/maschine_mikro_mk3.toml):**
```toml
[template_metadata]
name = "Native Instruments Maschine Mikro MK3"
author = "MIDIMon Team"
version = "1.0.0"
description = "Default template for Maschine Mikro MK3 with 16 RGB pads"
device_type = "midi+hid"
created_date = "2025-01-15"
tags = ["native-instruments", "maschine", "controller"]

[device]
name = "Mikro MK3"
vendor_id = 0x17cc  # Native Instruments
product_id = 0x1600 # Maschine Mikro MK3
midi_port_pattern = "Maschine.*MK3"  # Regex for auto-detection

[device.layout]
type = "grid"
rows = 4
cols = 4
total_pads = 16

# Physical pad layout with MIDI note mappings
[[device.layout.pads]]
index = 0
position = [0, 0]  # Row 0, Col 0
note = 0
name = "Pad 1"

[[device.layout.pads]]
index = 1
position = [0, 1]
note = 1
name = "Pad 2"

# ... (repeat for all 16 pads)

[device.features]
velocity_sensitive = true
rgb_leds = true
aftertouch = false
pitch_bend = true
encoders = ["Main", "Tempo"]

# Default mode configurations
[[modes]]
name = "Starter"
description = "Beginner-friendly mappings for common tasks"
color = "blue"

[[modes.mappings]]
description = "Launch Chrome (Pad 1)"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"

[[modes.mappings]]
description = "Launch VS Code (Pad 2)"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

[[modes.mappings]]
description = "Volume Up (Pad 13)"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "VolumeControl"
action = "Up"

[[modes.mappings]]
description = "Volume Down (Pad 14)"
[modes.mappings.trigger]
type = "Note"
note = 13
[modes.mappings.action]
type = "VolumeControl"
action = "Down"

[[modes.mappings]]
description = "Mute (Pad 15)"
[modes.mappings.trigger]
type = "Note"
note = 14
[modes.mappings.action]
type = "VolumeControl"
action = "Mute"

[[modes.mappings]]
description = "Mode Switch (Pad 16)"
[modes.mappings.trigger]
type = "LongPress"
note = 15
duration_ms = 1000
[modes.mappings.action]
type = "ModeChange"
action = "Next"

[[global_mappings]]
description = "Emergency Exit (Encoder + Pad 16)"
[global_mappings.trigger]
type = "NoteChord"
notes = [15, 100]  # Pad 16 + Encoder button
[global_mappings.action]
type = "Shell"
command = "killall MIDIMon"
```

**Template Loader (src/templates.rs:1-150):**
```rust
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTemplate {
    pub template_metadata: TemplateMetadata,
    pub device: DeviceInfo,
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<MappingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub device_type: String,
    pub created_date: String,
    pub tags: Vec<String>,
}

pub struct TemplateLibrary {
    templates_dir: PathBuf,
}

impl TemplateLibrary {
    pub fn new() -> Result<Self, TemplateError> {
        let templates_dir = Self::get_templates_directory()?;
        fs::create_dir_all(&templates_dir)?;
        Ok(Self { templates_dir })
    }

    fn get_templates_directory() -> Result<PathBuf, TemplateError> {
        // Platform-specific template directories
        #[cfg(target_os = "macos")]
        let base_dir = dirs::home_dir()
            .ok_or(TemplateError::HomeDirNotFound)?
            .join("Library/Application Support/MIDIMon/templates");

        #[cfg(target_os = "linux")]
        let base_dir = dirs::config_dir()
            .ok_or(TemplateError::ConfigDirNotFound)?
            .join("midimon/templates");

        #[cfg(target_os = "windows")]
        let base_dir = dirs::config_dir()
            .ok_or(TemplateError::ConfigDirNotFound)?
            .join("MIDIMon/templates");

        Ok(base_dir)
    }

    pub fn list_templates(&self) -> Result<Vec<TemplateMetadata>, TemplateError> {
        let mut templates = Vec::new();

        for entry in fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                let template = self.load_template(&path)?;
                templates.push(template.template_metadata);
            }
        }

        Ok(templates)
    }

    pub fn load_template(&self, path: &Path) -> Result<DeviceTemplate, TemplateError> {
        let contents = fs::read_to_string(path)?;
        let template: DeviceTemplate = toml::from_str(&contents)?;
        Self::validate_template(&template)?;
        Ok(template)
    }

    fn validate_template(template: &DeviceTemplate) -> Result<(), TemplateError> {
        // Validate required fields
        if template.template_metadata.name.is_empty() {
            return Err(TemplateError::InvalidTemplate("Name is required".into()));
        }

        // Validate device layout
        if template.device.layout.total_pads == 0 {
            return Err(TemplateError::InvalidTemplate("Device must have at least one pad".into()));
        }

        // Validate MIDI note ranges
        for mode in &template.modes {
            for mapping in &mode.mappings {
                // Ensure notes are in valid MIDI range (0-127)
                // ... validation logic
            }
        }

        Ok(())
    }

    pub fn apply_template(
        &self,
        template: &DeviceTemplate,
        config_path: &Path,
    ) -> Result<(), TemplateError> {
        // Convert template to user config
        let config = Config {
            device: template.device.clone(),
            modes: template.modes.clone(),
            global_mappings: template.global_mappings.clone(),
            ..Default::default()
        };

        // Save to user's config.toml
        let toml_str = toml::to_string_pretty(&config)?;
        fs::write(config_path, toml_str)?;

        Ok(())
    }

    pub fn export_template(
        &self,
        config: &Config,
        metadata: TemplateMetadata,
        output_path: &Path,
    ) -> Result<(), TemplateError> {
        let template = DeviceTemplate {
            template_metadata: metadata,
            device: config.device.clone(),
            modes: config.modes.clone(),
            global_mappings: config.global_mappings.clone(),
        };

        let toml_str = toml::to_string_pretty(&template)?;
        fs::write(output_path, toml_str)?;

        Ok(())
    }
}
```

#### Configuration Example

**Built-in Templates (config/device_templates/):**
```
device_templates/
├── maschine_mikro_mk3.toml       # Native Instruments Maschine Mikro MK3
├── launchpad_mini.toml           # Novation Launchpad Mini
├── launchpad_pro.toml            # Novation Launchpad Pro
├── apc_mini.toml                 # Akai APC Mini
├── apc40.toml                    # Akai APC40
├── nanopad2.toml                 # Korg nanoPAD2
├── nanokontrol2.toml             # Korg nanoKONTROL2
└── beatstep.toml                 # Arturia BeatStep
```

**GUI Template Selection:**
```
First Launch Screen:
┌─────────────────────────────────────────┐
│ Welcome to MIDIMon!                     │
│                                         │
│ Select your MIDI controller:            │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 🎹 Native Instruments Maschine Mikro│ │
│ │    MK3 (16 RGB pads)                │ │
│ │    ★★★★★ Recommended                 │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 🎹 Novation Launchpad Mini          │ │
│ │    (64 pads, basic LEDs)            │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ ┌─────────────────────────────────────┐ │
│ │ 🎹 Akai APC Mini                    │ │
│ │    (64 pads + 9 faders)             │ │
│ └─────────────────────────────────────┘ │
│                                         │
│ [Import Custom Template]                │
│ [Start from Scratch]                    │
└─────────────────────────────────────────┘
```

**Import Custom Template:**
```bash
# User downloads template from community
# Example: streaming-setup-mikro.toml from GitHub

# CLI import
midimon import-template ~/Downloads/streaming-setup-mikro.toml

# GUI import
# File → Import Template → Select file → Apply
```

**Export User Config as Template:**
```bash
# Share your config with others
midimon export-template \
  --name "My Streaming Setup" \
  --author "Jordan" \
  --description "Optimized for OBS and streaming" \
  --output ~/Desktop/my-template.toml
```

#### Edge Cases
- **Missing Template File**: Shows "Template not found" error, falls back to blank config
- **Invalid TOML Syntax**: Validation fails with line number, template not applied
- **Incompatible Device**: Warns if template designed for different controller (e.g., 64-pad template on 16-pad device)
- **Conflicting Note Mappings**: Detects overlapping triggers, shows warning before import
- **Outdated Template Version**: Attempts to migrate older template formats to current schema
- **Custom Application Paths**: Resolves app names across platforms (e.g., "Chrome" → "/Applications/Google Chrome.app" on macOS)
- **Template Metadata Missing**: Uses filename as template name, shows as "Unknown Author"
- **Duplicate Template Names**: Appends "(2)" to name to avoid overwriting
- **Large Template Files**: Validates total mappings count (warns if >500 mappings)
- **Template Without Modes**: Creates default "Main" mode if none specified

#### Testing Criteria
- ✅ List all built-in templates from templates/ directory
- ✅ Load template and parse metadata, device info, mappings
- ✅ Apply template creates valid config.toml
- ✅ Import custom template from file browser
- ✅ Export current config as template with metadata
- ✅ Validate template before applying (TOML syntax, note ranges)
- ✅ Show template preview before applying
- ✅ Detect incompatible device types and warn user
- ✅ Resolve platform-specific application paths
- ✅ Template version migration from older formats
- ✅ Community template discovery (GitHub integration)
- ✅ Template tags for search/filter (e.g., "streaming", "daw", "productivity")

---

### TF6: Frontmost App Detection

#### Description
Platform-specific detection of the frontmost (focused) application to enable context-aware MIDI mappings that automatically adapt based on which application the user is currently using.

#### User Story
> As Alex (Producer), I want my controller to automatically switch profiles when I switch between Logic Pro and OBS, so that the same pads do different things depending on which app I'm focused on.

#### Technical Implementation

**macOS Implementation (src/platform/macos/frontmost_app.rs:1-100):**
```rust
use cocoa::appkit::{NSRunningApplication, NSWorkspace};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::{msg_send, sel, sel_impl};

pub struct FrontmostAppDetector {
    workspace: id,
    poll_interval: Duration,
    last_bundle_id: Option<String>,
    last_check: Instant,
}

impl FrontmostAppDetector {
    pub fn new(poll_interval: Duration) -> Result<Self, PlatformError> {
        unsafe {
            let workspace: id = msg_send![
                class!(NSWorkspace),
                sharedWorkspace
            ];

            Ok(Self {
                workspace,
                poll_interval,
                last_bundle_id: None,
                last_check: Instant::now(),
            })
        }
    }

    pub fn check_frontmost_app(&mut self) -> Option<AppInfo> {
        // Debounce checks
        if self.last_check.elapsed() < self.poll_interval {
            return None;
        }
        self.last_check = Instant::now();

        unsafe {
            let _pool = NSAutoreleasePool::new(nil);

            let frontmost_app: id = msg_send![
                self.workspace,
                frontmostApplication
            ];

            if frontmost_app == nil {
                return None;
            }

            // Get bundle identifier
            let bundle_id: id = msg_send![frontmost_app, bundleIdentifier];
            if bundle_id == nil {
                return None;
            }

            let bundle_id_str = nsstring_to_string(bundle_id);

            // Get localized name
            let localized_name: id = msg_send![frontmost_app, localizedName];
            let name = if localized_name != nil {
                nsstring_to_string(localized_name)
            } else {
                bundle_id_str.clone()
            };

            // Check if changed
            let changed = self.last_bundle_id.as_ref() != Some(&bundle_id_str);
            self.last_bundle_id = Some(bundle_id_str.clone());

            if changed {
                Some(AppInfo {
                    bundle_id: bundle_id_str,
                    name,
                    changed,
                })
            } else {
                None
            }
        }
    }
}

fn nsstring_to_string(ns_string: id) -> String {
    unsafe {
        let c_str: *const i8 = msg_send![ns_string, UTF8String];
        std::ffi::CStr::from_ptr(c_str)
            .to_string_lossy()
            .into_owned()
    }
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub bundle_id: String,
    pub name: String,
    pub changed: bool,
}
```

**Linux Implementation (src/platform/linux/frontmost_app.rs:1-80):**
```rust
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::RustConnection;

pub struct FrontmostAppDetector {
    conn: RustConnection,
    root: Window,
    net_active_window: Atom,
    net_wm_name: Atom,
    poll_interval: Duration,
    last_window: Option<Window>,
}

impl FrontmostAppDetector {
    pub fn new(poll_interval: Duration) -> Result<Self, PlatformError> {
        let (conn, screen_num) = x11rb::connect(None)?;
        let screen = &conn.setup().roots[screen_num];
        let root = screen.root;

        // Get atoms
        let net_active_window = conn.intern_atom(false, b"_NET_ACTIVE_WINDOW")?.reply()?.atom;
        let net_wm_name = conn.intern_atom(false, b"_NET_WM_NAME")?.reply()?.atom;

        Ok(Self {
            conn,
            root,
            net_active_window,
            net_wm_name,
            poll_interval,
            last_window: None,
        })
    }

    pub fn check_frontmost_app(&mut self) -> Option<AppInfo> {
        // Get active window
        let active_window = self.conn
            .get_property(
                false,
                self.root,
                self.net_active_window,
                AtomEnum::WINDOW,
                0,
                1,
            )
            .ok()?
            .reply()
            .ok()?;

        let window: Window = active_window.value32()?.next()?;

        // Check if changed
        if Some(window) == self.last_window {
            return None;
        }
        self.last_window = Some(window);

        // Get window name
        let name_prop = self.conn
            .get_property(false, window, self.net_wm_name, AtomEnum::ANY, 0, 1024)
            .ok()?
            .reply()
            .ok()?;

        let name = String::from_utf8_lossy(&name_prop.value).into_owned();

        Some(AppInfo {
            bundle_id: format!("window:{}", window),
            name,
            changed: true,
        })
    }
}
```

**Windows Implementation (src/platform/windows/frontmost_app.rs:1-60):**
```rust
use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};
use winapi::shared::windef::HWND;

pub struct FrontmostAppDetector {
    poll_interval: Duration,
    last_hwnd: Option<isize>,
}

impl FrontmostAppDetector {
    pub fn new(poll_interval: Duration) -> Result<Self, PlatformError> {
        Ok(Self {
            poll_interval,
            last_hwnd: None,
        })
    }

    pub fn check_frontmost_app(&mut self) -> Option<AppInfo> {
        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            let hwnd_val = hwnd as isize;

            if Some(hwnd_val) == self.last_hwnd {
                return None;
            }
            self.last_hwnd = Some(hwnd_val);

            // Get window title
            let mut title: [u16; 512] = [0; 512];
            let len = GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);

            if len == 0 {
                return None;
            }

            let name = String::from_utf16_lossy(&title[..len as usize]);

            Some(AppInfo {
                bundle_id: format!("hwnd:{}", hwnd_val),
                name,
                changed: true,
            })
        }
    }
}
```

**Integration with Engine (src/main.rs:350-450):**
```rust
fn run_with_app_detection(/* ... */) {
    let mut app_detector = FrontmostAppDetector::new(Duration::from_millis(500))
        .expect("Failed to initialize app detector");

    let mut current_profile: Option<String> = None;

    loop {
        select! {
            recv(midi_rx) -> msg => {
                // Handle MIDI events
            }
            default(timeout) => {
                // Check for app changes
                if let Some(app_info) = app_detector.check_frontmost_app() {
                    println!("🔄 App changed: {}", app_info.name);

                    // Look up profile for this app
                    if let Some(profile_name) = config.get_profile_for_app(&app_info.bundle_id) {
                        if Some(&profile_name) != current_profile.as_ref() {
                            println!("   Switching to profile: {}", profile_name);

                            if let Err(e) = engine.load_profile(&profile_name) {
                                eprintln!("❌ Failed to load profile: {}", e);
                            } else {
                                current_profile = Some(profile_name);
                                led_feedback.flash_mode_change();
                            }
                        }
                    } else {
                        // Fall back to default profile
                        if current_profile.is_some() {
                            println!("   No profile for {}, using default", app_info.name);
                            engine.load_default_profile();
                            current_profile = None;
                        }
                    }
                }
            }
        }
    }
}
```

#### Configuration Example

**Per-App Profile Mappings (config.toml):**
```toml
[app_profiles]
# Enable automatic profile switching based on frontmost app
enabled = true

# Polling interval (milliseconds)
poll_interval_ms = 500

# Debounce duration to avoid rapid switching
debounce_ms = 200

# App bundle ID → profile name mappings
[app_profiles.mappings]
"com.apple.Logic" = "daw-profile"
"com.apple.FinalCut" = "video-editing"
"com.obsproject.obs-studio" = "streaming"
"com.google.Chrome" = "browser"
"com.microsoft.VSCode" = "development"
"com.figma.Desktop" = "design"

# Fallback profile if no match found (optional)
default_profile = "general"

# Example profiles referenced above:

[[profiles]]
name = "daw-profile"
description = "Optimized for Logic Pro"
[[profiles.modes]]
name = "Transport"
# ... DAW-specific mappings

[[profiles]]
name = "streaming"
description = "OBS and streaming controls"
[[profiles.modes]]
name = "Scenes"
# ... OBS-specific mappings
```

**Platform-Specific Bundle IDs:**
```toml
# macOS: Use bundle identifier
[app_profiles.mappings]
"com.apple.Logic" = "daw-profile"
"com.apple.Safari" = "browser"

# Linux: Use window class or name pattern (regex)
[app_profiles.mappings]
"firefox" = "browser"
"code-oss" = "development"

# Windows: Use process name or window title pattern
[app_profiles.mappings]
"chrome.exe" = "browser"
"Code.exe" = "development"
```

**Example: Automatic DAW Profile Switch**
```toml
# User launches Logic Pro
# MIDIMon detects: com.apple.Logic
# Output:
# 🔄 App changed: Logic Pro
#    Switching to profile: daw-profile
# [Pads now control DAW transport, mixer, etc.]

# User switches to OBS
# MIDIMon detects: com.obsproject.obs-studio
# Output:
# 🔄 App changed: OBS Studio
#    Switching to profile: streaming
# [Pads now control OBS scenes, mic mute, etc.]
```

#### Edge Cases
- **Rapid App Switching**: Debouncing (200ms default) prevents excessive profile changes
- **Unknown App**: Falls back to default profile if no mapping found for frontmost app
- **App Without Bundle ID**: Uses fallback identifier (window handle, process name, etc.)
- **Permission Denied (macOS)**: Requires Accessibility permissions, shows error with instructions
- **X11 Not Available (Linux)**: Falls back to Wayland detection or disables feature with warning
- **Profile Load Failure**: Logs error but continues with previous profile
- **Polling Overhead**: Configurable interval (500ms default) balances responsiveness and CPU usage
- **Multi-Monitor Focus**: Detects focused window regardless of monitor
- **Full-Screen Apps**: Continues detection even when app is full-screen
- **Virtual Desktops**: Tracks focus across virtual desktops/spaces

#### Testing Criteria
- ✅ Detects frontmost app change within poll interval
- ✅ Extracts bundle ID on macOS, window class on Linux, process name on Windows
- ✅ Switches profile when app matches configured mapping
- ✅ Falls back to default profile for unmapped apps
- ✅ Debouncing prevents rapid profile switching
- ✅ Permissions error shown with instructions (macOS Accessibility)
- ✅ Profile switch triggers LED feedback
- ✅ Polling interval configurable (100ms-5000ms)
- ✅ No profile switch if same app remains focused
- ✅ Works across multiple monitors
- ✅ Works in full-screen mode
- ✅ Logs app changes for debugging

---

### TF7: Per-App Profiles

#### Description
Configuration system for defining multiple named profiles with independent mode/mapping sets, enabling automatic or manual switching to different control schemes based on the active application context.

#### User Story
> As Alex (Producer), I want to create separate profiles for Logic Pro, OBS, and browsing, each with completely different pad layouts, so that my controller adapts to my workflow instead of me memorizing different mappings.

#### Technical Implementation

**Profile Structure (src/profiles.rs:1-200):**
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub description: String,
    pub author: Option<String>,
    pub version: Option<String>,
    pub priority: ProfilePriority,
    pub modes: Vec<Mode>,
    pub global_mappings: Vec<MappingConfig>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProfilePriority {
    App,      // Triggered by specific app (highest priority)
    User,     // Manually selected by user
    Global,   // Always active (lowest priority)
    Default,  // Fallback when no other profile matches
}

pub struct ProfileManager {
    profiles: HashMap<String, ProfileConfig>,
    active_profile: Option<String>,
    profile_stack: Vec<String>,  // For priority management
    default_profile: String,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            active_profile: None,
            profile_stack: Vec::new(),
            default_profile: "default".to_string(),
        }
    }

    pub fn load_profiles(&mut self, config: &Config) -> Result<(), ProfileError> {
        for profile in &config.profiles {
            self.profiles.insert(profile.name.clone(), profile.clone());
        }
        Ok(())
    }

    pub fn activate_profile(
        &mut self,
        profile_name: &str,
        priority: ProfilePriority,
    ) -> Result<&ProfileConfig, ProfileError> {
        let profile = self.profiles.get(profile_name)
            .ok_or(ProfileError::ProfileNotFound(profile_name.to_string()))?;

        // Update active profile
        self.active_profile = Some(profile_name.to_string());

        // Manage priority stack
        match priority {
            ProfilePriority::App => {
                // App profiles override everything except user
                self.profile_stack.retain(|p| {
                    self.profiles.get(p).map(|pf| pf.priority) != Some(ProfilePriority::App)
                });
                self.profile_stack.push(profile_name.to_string());
            }
            ProfilePriority::User => {
                // User selection overrides app profiles
                self.profile_stack.clear();
                self.profile_stack.push(profile_name.to_string());
            }
            _ => {}
        }

        Ok(profile)
    }

    pub fn deactivate_app_profile(&mut self) -> Option<String> {
        // Remove app-triggered profile and restore previous
        self.profile_stack.retain(|p| {
            self.profiles.get(p).map(|pf| pf.priority) != Some(ProfilePriority::App)
        });

        // Restore previous profile or default
        if let Some(prev_profile) = self.profile_stack.last() {
            self.active_profile = Some(prev_profile.clone());
            Some(prev_profile.clone())
        } else {
            self.active_profile = Some(self.default_profile.clone());
            Some(self.default_profile.clone())
        }
    }

    pub fn get_active_profile(&self) -> Option<&ProfileConfig> {
        self.active_profile.as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    pub fn list_profiles(&self) -> Vec<&ProfileConfig> {
        self.profiles.values().collect()
    }
}
```

**Profile-Based Mapping Engine (src/mappings.rs:500-600):**
```rust
impl MappingEngine {
    pub fn load_profile(&mut self, profile: &ProfileConfig) -> Result<(), MappingError> {
        println!("📂 Loading profile: {}", profile.name);

        // Clear current state
        self.active_timers.clear();
        self.last_press_times.clear();

        // Load modes from profile
        self.modes = profile.modes.clone();
        self.global_mappings = profile.global_mappings.clone();

        // Reset to first mode
        self.current_mode = 0;

        // Compile mappings
        for mode in &self.modes {
            for mapping in &mode.mappings {
                self.compile_mapping(mapping)?;
            }
        }

        println!("   ✅ Loaded {} modes, {} global mappings",
                 self.modes.len(),
                 self.global_mappings.len());

        Ok(())
    }

    pub fn switch_profile(&mut self, profile_name: &str) -> Result<(), MappingError> {
        let profile = self.profile_manager.activate_profile(
            profile_name,
            ProfilePriority::User,
        )?;

        self.load_profile(profile)
    }
}
```

#### Configuration Example

**Multi-Profile Config (config.toml):**
```toml
# Default profile (always available)
[[profiles]]
name = "default"
description = "General-purpose mappings"
priority = "Default"

[[profiles.modes]]
name = "Main"
color = "blue"

[[profiles.modes.mappings]]
description = "Launch Chrome"
[profiles.modes.mappings.trigger]
type = "Note"
note = 0
[profiles.modes.mappings.action]
type = "Launch"
app = "Google Chrome"

# DAW profile (activated when Logic Pro is frontmost)
[[profiles]]
name = "daw-profile"
description = "Logic Pro controls"
priority = "App"

[[profiles.modes]]
name = "Transport"
color = "green"

[[profiles.modes.mappings]]
description = "Play/Pause"
[profiles.modes.mappings.trigger]
type = "Note"
note = 0
[profiles.modes.mappings.action]
type = "Keystroke"
keys = "space"

[[profiles.modes.mappings]]
description = "Record"
[profiles.modes.mappings.trigger]
type = "Note"
note = 1
[profiles.modes.mappings.action]
type = "Keystroke"
keys = "r"

[[profiles.modes]]
name = "Mixer"
color = "yellow"

[[profiles.modes.mappings]]
description = "Volume Up (Selected Track)"
[profiles.modes.mappings.trigger]
type = "Note"
note = 4
[profiles.modes.mappings.action]
type = "Keystroke"
keys = "up"
modifiers = ["ctrl"]

# Streaming profile (activated when OBS is frontmost)
[[profiles]]
name = "streaming"
description = "OBS Studio controls"
priority = "App"

[[profiles.modes]]
name = "Scenes"
color = "purple"

[[profiles.modes.mappings]]
description = "Switch to Scene 1"
[profiles.modes.mappings.trigger]
type = "Note"
note = 0
[profiles.modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["cmd", "shift"]

[[profiles.modes.mappings]]
description = "Mute Mic"
[profiles.modes.mappings.trigger]
type = "Note"
note = 15
[profiles.modes.mappings.action]
type = "Keystroke"
keys = "m"
modifiers = ["cmd", "shift"]

# App → Profile mapping
[app_profiles]
enabled = true
[app_profiles.mappings]
"com.apple.Logic" = "daw-profile"
"com.obsproject.obs-studio" = "streaming"
```

**Manual Profile Switching:**
```toml
# Map encoder to cycle through profiles
[[global_mappings]]
description = "Cycle profiles (Encoder Long Press)"
[global_mappings.trigger]
type = "EncoderTurn"
encoder = "Main"
direction = "Clockwise"
hold_duration_ms = 1000
[global_mappings.action]
type = "ProfileSwitch"
action = "Next"

# Map chord to specific profile
[[global_mappings]]
description = "Switch to DAW profile (Pads 15+16)"
[global_mappings.trigger]
type = "NoteChord"
notes = [14, 15]
[global_mappings.action]
type = "ProfileSwitch"
profile = "daw-profile"
```

**Profile Priority Example:**
```rust
// Scenario 1: User manually selects "streaming" profile
// Priority: User
engine.switch_profile("streaming")?;

// Scenario 2: User switches to Logic Pro
// Frontmost app detector triggers:
// Priority: App → "daw-profile" overrides user selection
profile_manager.activate_profile("daw-profile", ProfilePriority::App)?;

// Scenario 3: User switches to Chrome (no app profile)
// Frontmost app detector:
// No app profile for Chrome → restore previous user selection ("streaming")
profile_manager.deactivate_app_profile()?;
```

#### Edge Cases
- **Profile Not Found**: Shows error, continues with current profile
- **Circular Profile References**: Detects and rejects circular dependencies
- **Missing Modes**: Profile with zero modes rejected during validation
- **State Preservation**: Active long press timers cancelled when switching profiles
- **Profile Priority Conflict**: User-selected profile overrides app-triggered profile
- **Rapid App Switching**: Debouncing prevents rapid profile thrashing
- **Invalid Profile Format**: Validates profile structure before loading
- **Profile Version Mismatch**: Attempts migration from older profile formats
- **Empty Profile Stack**: Falls back to default profile when stack is empty
- **Profile Load During Action**: Completes current action before switching profiles

#### Testing Criteria
- ✅ Load multiple profiles from config.toml
- ✅ Activate profile by name and load its modes/mappings
- ✅ Switch between profiles manually via action
- ✅ Automatic profile switch on app change (via TF6)
- ✅ Profile priority system (User > App > Global > Default)
- ✅ State preservation during profile switch (cancel timers)
- ✅ Profile stack management (push/pop priorities)
- ✅ Fallback to default profile for unmapped apps
- ✅ LED feedback shows profile change
- ✅ Profile list available in GUI
- ✅ Profile validation before loading
- ✅ Profile switch action type in config

---

### TF8: Auto-Start on Boot

#### Description
Platform-specific system integration to automatically launch MIDIMon in the background when the system starts, providing seamless always-on MIDI mapping functionality.

#### User Story
> As Jordan (Streamer), I want MIDIMon to start automatically when I turn on my computer, so that my controller works immediately without manually launching the app every time.

#### Technical Implementation

**macOS LaunchAgent (installer/macos/com.midimon.agent.plist):**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.midimon.agent</string>

    <key>ProgramArguments</key>
    <array>
        <string>/Applications/MIDIMon.app/Contents/MacOS/midimon</string>
        <string>--background</string>
    </array>

    <key>RunAtLoad</key>
    <true/>

    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>

    <key>StandardOutPath</key>
    <string>/tmp/midimon.log</string>

    <key>StandardErrorPath</key>
    <string>/tmp/midimon.error.log</string>

    <key>EnvironmentVariables</key>
    <dict>
        <key>PATH</key>
        <string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
    </dict>
</dict>
</plist>
```

**Installer Integration (src/installer/autostart.rs:1-200):**
```rust
use std::path::PathBuf;
use std::fs;

pub struct AutostartManager {
    config_dir: PathBuf,
}

impl AutostartManager {
    pub fn new() -> Result<Self, AutostartError> {
        let config_dir = Self::get_autostart_dir()?;
        fs::create_dir_all(&config_dir)?;
        Ok(Self { config_dir })
    }

    #[cfg(target_os = "macos")]
    fn get_autostart_dir() -> Result<PathBuf, AutostartError> {
        let home = dirs::home_dir()
            .ok_or(AutostartError::HomeDirNotFound)?;
        Ok(home.join("Library/LaunchAgents"))
    }

    #[cfg(target_os = "linux")]
    fn get_autostart_dir() -> Result<PathBuf, AutostartError> {
        let config = dirs::config_dir()
            .ok_or(AutostartError::ConfigDirNotFound)?;
        Ok(config.join("autostart"))
    }

    #[cfg(target_os = "windows")]
    fn get_autostart_dir() -> Result<PathBuf, AutostartError> {
        // Use registry instead of directory
        Ok(PathBuf::new())
    }

    #[cfg(target_os = "macos")]
    pub fn enable(&self, app_path: &Path) -> Result<(), AutostartError> {
        let plist_path = self.config_dir.join("com.midimon.agent.plist");

        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.midimon.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>--background</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
</dict>
</plist>"#,
            app_path.display()
        );

        fs::write(&plist_path, plist_content)?;

        // Load the agent
        std::process::Command::new("launchctl")
            .args(&["load", plist_path.to_str().unwrap()])
            .status()?;

        println!("✅ Autostart enabled (LaunchAgent)");
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn enable(&self, app_path: &Path) -> Result<(), AutostartError> {
        let desktop_path = self.config_dir.join("midimon.desktop");

        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=MIDIMon
Comment=MIDI Controller Mapping
Exec={}
Icon=midimon
Terminal=false
Categories=Audio;Utility;
StartupNotify=false
X-GNOME-Autostart-enabled=true"#,
            app_path.display()
        );

        fs::write(&desktop_path, desktop_content)?;
        println!("✅ Autostart enabled (~/.config/autostart)");
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn enable(&self, app_path: &Path) -> Result<(), AutostartError> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let run_key = hkcu.open_subkey_with_flags(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            KEY_WRITE,
        )?;

        run_key.set_value("MIDIMon", &app_path.to_str().unwrap())?;
        println!("✅ Autostart enabled (Registry)");
        Ok(())
    }

    pub fn disable(&self) -> Result<(), AutostartError> {
        #[cfg(target_os = "macos")]
        {
            let plist_path = self.config_dir.join("com.midimon.agent.plist");

            if plist_path.exists() {
                // Unload the agent
                std::process::Command::new("launchctl")
                    .args(&["unload", plist_path.to_str().unwrap()])
                    .status()
                    .ok();

                fs::remove_file(&plist_path)?;
            }
        }

        #[cfg(target_os = "linux")]
        {
            let desktop_path = self.config_dir.join("midimon.desktop");
            if desktop_path.exists() {
                fs::remove_file(&desktop_path)?;
            }
        }

        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run_key = hkcu.open_subkey_with_flags(
                r"Software\Microsoft\Windows\CurrentVersion\Run",
                KEY_WRITE,
            )?;

            run_key.delete_value("MIDIMon").ok();
        }

        println!("✅ Autostart disabled");
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.config_dir.join("com.midimon.agent.plist").exists()
        }

        #[cfg(target_os = "linux")]
        {
            self.config_dir.join("midimon.desktop").exists()
        }

        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let run_key = hkcu.open_subkey(
                r"Software\Microsoft\Windows\CurrentVersion\Run"
            ).ok();

            run_key.and_then(|key| key.get_value::<String, _>("MIDIMon").ok()).is_some()
        }
    }
}
```

**GUI Integration (src-tauri/src/preferences.rs:50-100):**
```rust
#[tauri::command]
async fn toggle_autostart(
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let autostart = AutostartManager::new()
        .map_err(|e| e.to_string())?;

    if enabled {
        let app_path = std::env::current_exe()
            .map_err(|e| e.to_string())?;

        autostart.enable(&app_path)
            .map_err(|e| e.to_string())?;
    } else {
        autostart.disable()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn get_autostart_status() -> Result<bool, String> {
    let autostart = AutostartManager::new()
        .map_err(|e| e.to_string())?;

    Ok(autostart.is_enabled())
}
```

#### Configuration Example

**Preferences UI (Settings → General):**
```
┌─────────────────────────────────┐
│ General Settings                │
│                                 │
│ ☑ Launch at startup             │
│   Start MIDIMon automatically   │
│   when you log in               │
│                                 │
│ ☑ Start minimized to menu bar   │
│   Hide window on launch         │
│                                 │
│ ☐ Show splash screen            │
│   Display startup animation     │
│                                 │
│ [Save] [Cancel]                 │
└─────────────────────────────────┘
```

**CLI Installation:**
```bash
# Enable autostart
midimon autostart enable

# Disable autostart
midimon autostart disable

# Check status
midimon autostart status
# Output: Autostart is enabled (LaunchAgent)
```

**Uninstaller Integration:**
```bash
# Uninstall script should clean up autostart
#!/bin/bash
# uninstall.sh

echo "🗑️  Removing MIDIMon..."

# Disable autostart
midimon autostart disable 2>/dev/null || true

# Remove application
rm -rf /Applications/MIDIMon.app

# Remove config (optional)
read -p "Remove configuration files? (y/n) " -n 1 -r
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -rf ~/Library/Application\ Support/MIDIMon
fi

echo "✅ MIDIMon uninstalled"
```

#### Edge Cases
- **Installer Permission Denied**: Shows error with instructions for manual setup
- **Registry Access Denied (Windows)**: Requires administrator privileges, prompts user
- **LaunchAgent Already Exists**: Overwrites existing plist with confirmation
- **App Path Changed**: Updates autostart path when app is moved
- **Multiple Instances**: Prevents duplicate autostart entries
- **Background Mode**: Runs with `--background` flag to hide window
- **Crash on Startup**: KeepAlive restarts app on crash (macOS)
- **Uninstaller Cleanup**: Removes autostart entry during uninstall
- **Upgrade**: Preserves autostart setting across version upgrades
- **Symlink Handling**: Resolves symlinks to canonical app path

#### Testing Criteria
- ✅ Enable autostart creates platform-specific entry
- ✅ Disable autostart removes entry
- ✅ Status check returns correct enabled/disabled state
- ✅ App launches on system boot/login
- ✅ Background mode hides window on autostart
- ✅ Autostart persists across system reboots
- ✅ Uninstaller removes autostart entry
- ✅ Upgrade preserves autostart setting
- ✅ Permission errors shown with instructions
- ✅ macOS: LaunchAgent plist created in ~/Library/LaunchAgents
- ✅ Linux: .desktop file created in ~/.config/autostart
- ✅ Windows: Registry entry created in HKCU\...\Run

---

### TF9: Live Event Console

#### Description
Real-time event monitoring interface displaying the full MIDI → Processed Event → Action resolution pipeline, enabling debugging, testing, and understanding of mapping behavior.

#### User Story
> As Sam (Developer), I want to see a live stream of MIDI events, how they're processed, and which actions they trigger, so that I can debug why my mappings aren't working as expected.

#### Technical Implementation

**Event Logger (src/event_logger.rs:1-150):**
```rust
use crossbeam_channel::{Sender, Receiver, unbounded};
use std::time::{Instant, SystemTime};

#[derive(Debug, Clone)]
pub struct EventLogEntry {
    pub timestamp: SystemTime,
    pub elapsed_ms: u64,
    pub event_type: EventType,
    pub details: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum EventType {
    MidiRaw,          // Raw MIDI bytes
    MidiParsed,       // Parsed MidiEvent
    Processed,        // ProcessedEvent (velocity/timing detection)
    MappingResolved,  // Matched mapping found
    ActionExecuted,   // Action executed successfully
    ActionFailed,     // Action execution failed
    NoMapping,        // No mapping found for event
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct EventLogger {
    tx: Sender<EventLogEntry>,
    rx: Receiver<EventLogEntry>,
    start_time: Instant,
    max_entries: usize,
}

impl EventLogger {
    pub fn new(max_entries: usize) -> Self {
        let (tx, rx) = unbounded();
        Self {
            tx,
            rx,
            start_time: Instant::now(),
            max_entries,
        }
    }

    pub fn log(&self, event_type: EventType, details: String, level: LogLevel) {
        let entry = EventLogEntry {
            timestamp: SystemTime::now(),
            elapsed_ms: self.start_time.elapsed().as_millis() as u64,
            event_type,
            details,
            level,
        };

        self.tx.send(entry).ok();
    }

    pub fn log_midi_raw(&self, bytes: &[u8]) {
        self.log(
            EventType::MidiRaw,
            format!("{:02X?}", bytes),
            LogLevel::Debug,
        );
    }

    pub fn log_midi_event(&self, event: &MidiEvent) {
        self.log(
            EventType::MidiParsed,
            format!("{:?}", event),
            LogLevel::Debug,
        );
    }

    pub fn log_processed_event(&self, event: &ProcessedEvent) {
        self.log(
            EventType::Processed,
            format!("{:?}", event),
            LogLevel::Info,
        );
    }

    pub fn log_mapping_resolved(&self, mapping: &MappingConfig) {
        self.log(
            EventType::MappingResolved,
            format!("{} → {:?}", mapping.description, mapping.action),
            LogLevel::Info,
        );
    }

    pub fn log_action_executed(&self, action: &Action) {
        self.log(
            EventType::ActionExecuted,
            format!("{:?}", action),
            LogLevel::Info,
        );
    }

    pub fn log_action_failed(&self, action: &Action, error: &str) {
        self.log(
            EventType::ActionFailed,
            format!("{:?} - Error: {}", action, error),
            LogLevel::Error,
        );
    }

    pub fn log_no_mapping(&self, event: &ProcessedEvent) {
        self.log(
            EventType::NoMapping,
            format!("No mapping for: {:?}", event),
            LogLevel::Warning,
        );
    }

    pub fn get_entries(&self) -> Vec<EventLogEntry> {
        let mut entries = Vec::new();
        while let Ok(entry) = self.rx.try_recv() {
            entries.push(entry);
            if entries.len() >= self.max_entries {
                entries.remove(0); // FIFO buffer
            }
        }
        entries
    }
}
```

**GUI Event Console (ui/src/components/EventConsole.svelte):**
```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let events: EventLogEntry[] = [];
  let filter: EventType | 'all' = 'all';
  let paused = false;
  let interval: number;

  onMount(() => {
    // Poll for new events every 100ms
    interval = setInterval(async () => {
      if (!paused) {
        const newEvents = await invoke('get_event_log');
        events = [...events, ...newEvents].slice(-1000); // Keep last 1000
      }
    }, 100);
  });

  onDestroy(() => {
    clearInterval(interval);
  });

  function clearEvents() {
    events = [];
    invoke('clear_event_log');
  }

  function togglePause() {
    paused = !paused;
  }

  function exportEvents() {
    const json = JSON.stringify(events, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `midimon-events-${Date.now()}.json`;
    a.click();
  }

  $: filteredEvents = filter === 'all'
    ? events
    : events.filter(e => e.event_type === filter);

  function getEventColor(type: EventType): string {
    switch (type) {
      case 'MidiRaw': return '#666';
      case 'MidiParsed': return '#888';
      case 'Processed': return '#0af';
      case 'MappingResolved': return '#0f0';
      case 'ActionExecuted': return '#0c0';
      case 'ActionFailed': return '#f00';
      case 'NoMapping': return '#fa0';
      default: return '#fff';
    }
  }
</script>

<div class="event-console">
  <div class="console-header">
    <h3>Live Event Console</h3>
    <div class="controls">
      <select bind:value={filter}>
        <option value="all">All Events</option>
        <option value="MidiParsed">MIDI Events</option>
        <option value="Processed">Processed Events</option>
        <option value="MappingResolved">Mappings</option>
        <option value="ActionExecuted">Actions</option>
        <option value="ActionFailed">Errors</option>
      </select>
      <button on:click={togglePause}>
        {paused ? '▶️ Resume' : '⏸ Pause'}
      </button>
      <button on:click={clearEvents}>🗑️ Clear</button>
      <button on:click={exportEvents}>💾 Export</button>
    </div>
  </div>

  <div class="event-list">
    {#each filteredEvents as event}
      <div class="event-entry" style="border-left: 3px solid {getEventColor(event.event_type)}">
        <span class="timestamp">[{event.elapsed_ms}ms]</span>
        <span class="event-type">{event.event_type}</span>
        <span class="details">{event.details}</span>
      </div>
    {/each}
  </div>
</div>

<style>
  .event-console {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: #1a1a1a;
    color: #eee;
    font-family: 'Courier New', monospace;
  }

  .console-header {
    display: flex;
    justify-content: space-between;
    padding: 10px;
    border-bottom: 1px solid #333;
  }

  .event-list {
    flex: 1;
    overflow-y: auto;
    padding: 10px;
  }

  .event-entry {
    padding: 5px;
    margin-bottom: 2px;
    border-left: 3px solid #666;
    font-size: 12px;
  }

  .timestamp {
    color: #888;
    margin-right: 10px;
  }

  .event-type {
    font-weight: bold;
    margin-right: 10px;
  }
</style>
```

**Tauri Commands (src-tauri/src/event_log.rs:1-50):**
```rust
#[tauri::command]
async fn get_event_log(state: State<'_, AppState>) -> Result<Vec<EventLogEntry>, String> {
    let logger = state.event_logger.lock().await;
    Ok(logger.get_entries())
}

#[tauri::command]
async fn clear_event_log(state: State<'_, AppState>) -> Result<(), String> {
    let logger = state.event_logger.lock().await;
    logger.clear();
    Ok(())
}

#[tauri::command]
async fn set_event_filter(
    state: State<'_, AppState>,
    filter: Vec<EventType>,
) -> Result<(), String> {
    let mut logger = state.event_logger.lock().await;
    logger.set_filter(filter);
    Ok(())
}
```

#### Configuration Example

**Event Console View:**
```
┌─────────────────────────────────────────────────────────────┐
│ Live Event Console                    [All ▼] [⏸][🗑️][💾] │
├─────────────────────────────────────────────────────────────┤
│ [1234ms] MidiParsed     NoteOn { note: 12, velocity: 85 }  │
│ [1235ms] Processed      Velocity(note=12, level=Hard)      │
│ [1236ms] MappingResolved Launch Logic Pro → Launch         │
│ [1240ms] ActionExecuted Launch("Logic Pro")                │
│ [2567ms] MidiParsed     NoteOn { note: 13, velocity: 45 }  │
│ [2568ms] Processed      Velocity(note=13, level=Medium)    │
│ [2569ms] NoMapping      No mapping for: Velocity(note=13)  │
│ [3890ms] MidiParsed     ControlChange { cc: 1, value: 65 } │
│ [3891ms] Processed      CC(controller=1, value=65)         │
│ [3892ms] MappingResolved Volume Up → VolumeControl(Up)     │
│ [3895ms] ActionExecuted VolumeControl(Up)                  │
│ [4123ms] MidiParsed     NoteOn { note: 0, velocity: 120 }  │
│ [4124ms] Processed      Velocity(note=0, level=Hard)       │
│ [4125ms] MappingResolved Launch Chrome → Launch            │
│ [4128ms] ActionFailed   Launch("Chrome") - Error: Not found│
└─────────────────────────────────────────────────────────────┘
```

**Performance Monitoring View:**
```
┌─────────────────────────────────────────────────────┐
│ Performance Stats (Last 60s)                        │
├─────────────────────────────────────────────────────┤
│ MIDI Events:          245                           │
│ Processed Events:     245                           │
│ Mappings Resolved:    198                           │
│ Actions Executed:     195                           │
│ Actions Failed:       3                             │
│ No Mapping:           47                            │
│                                                     │
│ Avg Processing Time:  0.8ms                         │
│ Peak Processing Time: 3.2ms                         │
│ Event Rate:           4.1/sec                       │
└─────────────────────────────────────────────────────┘
```

#### Edge Cases
- **High Event Rate**: Buffers last N events (1000 default), drops oldest on overflow
- **Console Paused**: Continues buffering events while paused, resumes on unpause
- **Filter Active**: Shows only matching events, but continues logging all types
- **Export Large Logs**: Prompts to confirm export if >10000 events
- **GUI Not Open**: Event logging continues in background regardless of GUI state
- **Memory Usage**: Circular buffer prevents unbounded memory growth
- **Timestamp Overflow**: Uses relative timestamps (ms since start) to avoid overflow
- **Concurrent Access**: Thread-safe logging with lock-free channel
- **Event Flood**: Rate limiting if event rate exceeds 1000/sec
- **Clear During Export**: Export captures snapshot, unaffected by clear

#### Testing Criteria
- ✅ Events appear in console in real-time (<100ms latency)
- ✅ Filter dropdown shows only matching event types
- ✅ Pause stops console updates but continues buffering
- ✅ Clear removes all events from console
- ✅ Export saves events as JSON file
- ✅ Event colors match type (MIDI=gray, Action=green, Error=red)
- ✅ Timestamp shows relative milliseconds since start
- ✅ Circular buffer keeps last 1000 events
- ✅ Performance stats show accurate counts and timing
- ✅ Console works while main engine is processing events
- ✅ No events dropped during high-rate scenarios
- ✅ Event details show full resolution pipeline

---

### TF10: Profile Sharing/Export

#### Description
Export and import functionality for profiles and mappings, enabling users to share custom configurations via files or community repositories, with version compatibility checking and metadata preservation.

#### User Story
> As Jordan (Streamer), I want to export my optimized OBS streaming profile and share it on GitHub, so that other streamers can use my setup without recreating all the mappings manually.

#### Technical Implementation

**Export Format (profile_export.rs:1-150):**
```rust
use serde::{Deserialize, Serialize};
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedProfile {
    pub metadata: ExportMetadata,
    pub profile: ProfileConfig,
    pub compatibility: CompatibilityInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub created_date: DateTime<Utc>,
    pub midimon_version: String,
    pub tags: Vec<String>,
    pub github_url: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub min_midimon_version: String,
    pub device_requirements: Vec<DeviceRequirement>,
    pub platform_specific: bool,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceRequirement {
    pub device_name: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub min_pads: usize,
}

pub struct ProfileExporter {
    midimon_version: String,
}

impl ProfileExporter {
    pub fn new() -> Self {
        Self {
            midimon_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn export_profile(
        &self,
        profile: &ProfileConfig,
        metadata: ExportMetadata,
        output_path: &Path,
    ) -> Result<(), ExportError> {
        let exported = ExportedProfile {
            metadata,
            profile: profile.clone(),
            compatibility: self.detect_compatibility(profile),
        };

        // Serialize to JSON with pretty printing
        let json = serde_json::to_string_pretty(&exported)?;
        std::fs::write(output_path, json)?;

        println!("✅ Profile exported to: {}", output_path.display());
        Ok(())
    }

    pub fn import_profile(
        &self,
        import_path: &Path,
    ) -> Result<ExportedProfile, ExportError> {
        let json = std::fs::read_to_string(import_path)?;
        let exported: ExportedProfile = serde_json::from_str(&json)?;

        // Validate compatibility
        self.validate_compatibility(&exported.compatibility)?;

        Ok(exported)
    }

    fn detect_compatibility(&self, profile: &ProfileConfig) -> CompatibilityInfo {
        let device_requirements = self.extract_device_requirements(profile);
        let platform_specific = self.detect_platform_specific_actions(profile);

        CompatibilityInfo {
            min_midimon_version: self.midimon_version.clone(),
            device_requirements,
            platform_specific,
            platform: if platform_specific {
                Some(std::env::consts::OS.to_string())
            } else {
                None
            },
        }
    }

    fn extract_device_requirements(&self, profile: &ProfileConfig) -> Vec<DeviceRequirement> {
        // Analyze mappings to determine device requirements
        let mut max_note: u8 = 0;
        for mode in &profile.modes {
            for mapping in &mode.mappings {
                if let Trigger::Note { note, .. } = mapping.trigger {
                    max_note = max_note.max(note);
                }
            }
        }

        let min_pads = (max_note + 1) as usize;

        vec![DeviceRequirement {
            device_name: "Generic MIDI Controller".to_string(),
            vendor_id: None,
            product_id: None,
            min_pads,
        }]
    }

    fn detect_platform_specific_actions(&self, profile: &ProfileConfig) -> bool {
        // Check for platform-specific actions (Shell commands, app paths, etc.)
        for mode in &profile.modes {
            for mapping in &mode.mappings {
                if self.is_platform_specific(&mapping.action) {
                    return true;
                }
            }
        }
        false
    }

    fn is_platform_specific(&self, action: &ActionConfig) -> bool {
        match action {
            ActionConfig::Shell { .. } => true,
            ActionConfig::Launch { app } => {
                // Check if app path is absolute (platform-specific)
                Path::new(app).is_absolute()
            }
            ActionConfig::Keystroke { modifiers, .. } => {
                // macOS uses "cmd", Windows uses "win"
                modifiers.contains(&"cmd".to_string()) || modifiers.contains(&"win".to_string())
            }
            _ => false,
        }
    }

    fn validate_compatibility(&self, compat: &CompatibilityInfo) -> Result<(), ExportError> {
        // Check MIDIMon version
        let current_version = semver::Version::parse(&self.midimon_version)?;
        let min_version = semver::Version::parse(&compat.min_midimon_version)?;

        if current_version < min_version {
            return Err(ExportError::IncompatibleVersion {
                required: compat.min_midimon_version.clone(),
                current: self.midimon_version.clone(),
            });
        }

        // Warn about platform-specific profiles
        if compat.platform_specific {
            if let Some(platform) = &compat.platform {
                if platform != std::env::consts::OS {
                    println!("⚠️  Warning: Profile is optimized for {}, you are on {}",
                             platform, std::env::consts::OS);
                }
            }
        }

        Ok(())
    }
}
```

**GUI Export Dialog (ui/src/components/ProfileExport.svelte):**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { save } from '@tauri-apps/api/dialog';

  export let profile: ProfileConfig;

  let metadata = {
    name: profile.name,
    author: '',
    description: '',
    version: '1.0.0',
    tags: [],
    github_url: '',
    license: 'MIT',
  };

  async function handleExport() {
    const filePath = await save({
      defaultPath: `${metadata.name.replace(/\s+/g, '-').toLowerCase()}.json`,
      filters: [{
        name: 'MIDIMon Profile',
        extensions: ['json']
      }]
    });

    if (filePath) {
      await invoke('export_profile', {
        profile,
        metadata,
        outputPath: filePath,
      });
    }
  }
</script>

<div class="export-dialog">
  <h2>Export Profile</h2>

  <label>
    Profile Name:
    <input type="text" bind:value={metadata.name} />
  </label>

  <label>
    Author:
    <input type="text" bind:value={metadata.author} />
  </label>

  <label>
    Description:
    <textarea bind:value={metadata.description} rows="3"></textarea>
  </label>

  <label>
    Version:
    <input type="text" bind:value={metadata.version} />
  </label>

  <label>
    Tags (comma-separated):
    <input type="text" placeholder="streaming, obs, productivity" />
  </label>

  <label>
    GitHub URL (optional):
    <input type="url" bind:value={metadata.github_url} />
  </label>

  <label>
    License:
    <select bind:value={metadata.license}>
      <option value="MIT">MIT</option>
      <option value="Apache-2.0">Apache 2.0</option>
      <option value="GPL-3.0">GPL 3.0</option>
      <option value="CC0-1.0">Public Domain (CC0)</option>
    </select>
  </label>

  <button on:click={handleExport}>Export</button>
</div>
```

#### Configuration Example

**Exported Profile (streaming-obs.json):**
```json
{
  "metadata": {
    "name": "OBS Streaming Control",
    "author": "Jordan (Streamer)",
    "description": "Complete OBS control setup for streamers with scene switching, mic mute, and source toggles",
    "version": "1.2.0",
    "created_date": "2025-01-15T10:30:00Z",
    "midimon_version": "0.2.0",
    "tags": ["streaming", "obs", "twitch", "youtube"],
    "github_url": "https://github.com/jordanstreamer/midimon-obs-profile",
    "license": "MIT"
  },
  "profile": {
    "name": "streaming",
    "description": "OBS Studio controls",
    "priority": "App",
    "modes": [
      {
        "name": "Scenes",
        "color": "purple",
        "mappings": [
          {
            "description": "Switch to Scene 1",
            "trigger": { "type": "Note", "note": 0 },
            "action": { "type": "Keystroke", "keys": "1", "modifiers": ["cmd", "shift"] }
          }
        ]
      }
    ],
    "global_mappings": []
  },
  "compatibility": {
    "min_midimon_version": "0.2.0",
    "device_requirements": [
      {
        "device_name": "Generic MIDI Controller",
        "vendor_id": null,
        "product_id": null,
        "min_pads": 16
      }
    ],
    "platform_specific": true,
    "platform": "macos"
  }
}
```

**Import Workflow:**
```bash
# User downloads profile from GitHub
curl -O https://github.com/jordanstreamer/midimon-obs-profile/raw/main/streaming-obs.json

# Import via CLI
midimon import-profile streaming-obs.json

# Output:
# ✅ Profile imported: OBS Streaming Control
# ⚠️  Warning: Profile is optimized for macos, you are on linux
# 📦 16 pads required (your device has 16 pads)
# ✅ Profile added to config
```

**Community Profile Marketplace (GitHub-based):**
```
# midimon-profiles repository structure:
midimon-profiles/
├── README.md
├── profiles/
│   ├── streaming/
│   │   ├── obs-basic.json
│   │   ├── obs-advanced.json
│   │   └── twitch-alerts.json
│   ├── daw/
│   │   ├── logic-pro-transport.json
│   │   ├── ableton-live-session.json
│   │   └── fl-studio-mixer.json
│   └── productivity/
│       ├── vscode-shortcuts.json
│       ├── photoshop-tools.json
│       └── browser-nav.json
└── index.json  # Searchable profile index
```

#### Edge Cases
- **Missing Metadata**: Uses defaults (author="Unknown", version="1.0.0")
- **Invalid Version String**: Rejects import with error message
- **Incompatible Version**: Shows warning, allows import if user confirms
- **Platform Mismatch**: Warns but allows import (user may need to adjust paths)
- **Missing Device**: Checks device requirements, warns if current device has fewer pads
- **Conflicting Profile Name**: Prompts to rename or overwrite existing profile
- **Large Profile Files**: Validates file size (<10MB) before import
- **Malformed JSON**: Shows parse error with line number
- **Absolute Paths**: Warns about platform-specific absolute paths during export
- **GitHub Integration**: Optional, falls back to manual file download if unavailable

#### Testing Criteria
- ✅ Export profile creates valid JSON file with metadata
- ✅ Import profile loads and validates compatibility
- ✅ Version compatibility check rejects older MIDIMon versions
- ✅ Platform-specific warning shown for cross-platform imports
- ✅ Device requirements validated against current device
- ✅ Metadata preserved through export/import cycle
- ✅ Profile name conflict handled with rename/overwrite prompt
- ✅ Absolute paths detected and flagged as platform-specific
- ✅ Tags enable search/filter in profile marketplace
- ✅ GitHub URL opens in browser from GUI
- ✅ License information displayed during import
- ✅ Export includes full profile with all modes and mappings

---

## Future Features (v2.5+)

The following features are conceptual designs for future versions (v2.5 and beyond), representing advanced capabilities and extensibility for power users and developers.

---

### FF1: Virtual MIDI Output (Enhanced)

#### Description
Bidirectional virtual MIDI port allowing MIDIMon to send MIDI messages to DAWs and other MIDI software, enabling controller-to-software and software-to-controller communication for advanced integration workflows.

#### User Story
> As Alex (Producer), I want my MIDI controller to send MIDI CC messages to Logic Pro's virtual instruments, so that I can control synth parameters directly from my pads without using keyboard shortcuts.

#### Technical Implementation

**Virtual MIDI Port Creation (src/virtual_midi.rs:1-200):**
```rust
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};

pub struct VirtualMidiOutput {
    connection: Arc<Mutex<Option<MidiOutputConnection>>>,
    port_name: String,
}

impl VirtualMidiOutput {
    pub fn new(port_name: &str) -> Result<Self, MidiError> {
        let midi_out = MidiOutput::new("MIDIMon Virtual Output")?;

        // Create virtual port (platform-specific)
        #[cfg(target_os = "macos")]
        let connection = {
            // CoreMIDI creates virtual source
            midi_out.create_virtual(port_name)?
        };

        #[cfg(target_os = "linux")]
        let connection = {
            // ALSA creates virtual port
            midi_out.create_virtual(port_name)?
        };

        #[cfg(target_os = "windows")]
        let connection = {
            // Windows: Use loopMIDI or similar virtual MIDI driver
            // Fall back to first available output port
            let ports = midi_out.ports();
            if ports.is_empty() {
                return Err(MidiError::NoPortsAvailable);
            }
            midi_out.connect(&ports[0], port_name)?
        };

        Ok(Self {
            connection: Arc::new(Mutex::new(Some(connection))),
            port_name: port_name.to_string(),
        })
    }

    pub fn send_note_on(&self, channel: u8, note: u8, velocity: u8) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            let msg = [0x90 | (channel & 0x0F), note & 0x7F, velocity & 0x7F];
            c.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_note_off(&self, channel: u8, note: u8) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            let msg = [0x80 | (channel & 0x0F), note & 0x7F, 0x00];
            c.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_cc(&self, channel: u8, controller: u8, value: u8) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            let msg = [0xB0 | (channel & 0x0F), controller & 0x7F, value & 0x7F];
            c.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_program_change(&self, channel: u8, program: u8) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            let msg = [0xC0 | (channel & 0x0F), program & 0x7F];
            c.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_pitch_bend(&self, channel: u8, value: u16) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            let lsb = (value & 0x7F) as u8;
            let msb = ((value >> 7) & 0x7F) as u8;
            let msg = [0xE0 | (channel & 0x0F), lsb, msb];
            c.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_sysex(&self, data: &[u8]) -> Result<(), MidiError> {
        let mut conn = self.connection.lock().unwrap();
        if let Some(ref mut c) = *conn {
            // Ensure SysEx starts with 0xF0 and ends with 0xF7
            let mut msg = vec![0xF0];
            msg.extend_from_slice(data);
            if msg.last() != Some(&0xF7) {
                msg.push(0xF7);
            }
            c.send(&msg)?;
        }
        Ok(())
    }
}
```

**Action Integration (src/actions.rs:800-900):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    // ... existing actions
    MidiOutput {
        message_type: MidiMessageType,
        channel: u8,
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MidiMessageType {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    CC { controller: u8, value: u8 },
    ProgramChange { program: u8 },
    PitchBend { value: u16 },
    SysEx { data: Vec<u8> },
}

impl ActionExecutor {
    fn execute_midi_output(&self, msg: &MidiMessageType, channel: u8) -> Result<(), ActionError> {
        let virtual_out = self.virtual_midi.lock().unwrap();

        match msg {
            MidiMessageType::NoteOn { note, velocity } => {
                virtual_out.send_note_on(channel, *note, *velocity)?;
            }
            MidiMessageType::NoteOff { note } => {
                virtual_out.send_note_off(channel, *note)?;
            }
            MidiMessageType::CC { controller, value } => {
                virtual_out.send_cc(channel, *controller, *value)?;
            }
            MidiMessageType::ProgramChange { program } => {
                virtual_out.send_program_change(channel, *program)?;
            }
            MidiMessageType::PitchBend { value } => {
                virtual_out.send_pitch_bend(channel, *value)?;
            }
            MidiMessageType::SysEx { data } => {
                virtual_out.send_sysex(data)?;
            }
        }

        Ok(())
    }
}
```

#### Configuration Example

**Virtual MIDI Output (config.toml):**
```toml
[virtual_midi]
enabled = true
port_name = "MIDIMon Output"
default_channel = 1

# Example: Pad press sends MIDI note to DAW
[[modes.mappings]]
description = "Trigger MIDI Note C3 (Pad 1)"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "MidiOutput"
message_type = { NoteOn = { note = 60, velocity = 100 } }
channel = 1

# Example: Velocity-sensitive MIDI output
[[modes.mappings]]
description = "Velocity-sensitive MIDI trigger"
[modes.mappings.trigger]
type = "VelocityRange"
note = 1
level = "Soft"
[modes.mappings.action]
type = "MidiOutput"
message_type = { NoteOn = { note = 62, velocity = 40 } }
channel = 1

[[modes.mappings]]
description = "Velocity-sensitive MIDI trigger (Hard)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 1
level = "Hard"
[modes.mappings.action]
type = "MidiOutput"
message_type = { NoteOn = { note = 62, velocity = 127 } }
channel = 1

# Example: Control Change (knob/fader simulation)
[[modes.mappings]]
description = "Send CC #1 (Modulation)"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "MidiOutput", message_type = { CC = { controller = 1, value = 127 } }, channel = 1 },
    { type = "Delay", duration_ms = 100 },
    { type = "MidiOutput", message_type = { CC = { controller = 1, value = 0 } }, channel = 1 },
]

# Example: Program Change (switch synth presets)
[[modes.mappings]]
description = "Load Preset 5"
[modes.mappings.trigger]
type = "Note"
note = 8
[modes.mappings.action]
type = "MidiOutput"
message_type = { ProgramChange = { program = 5 } }
channel = 1
```

**DAW Integration Example:**
```toml
# Logic Pro: Map virtual MIDI to track input
# 1. In Logic: Track → Input → MIDIMon Output
# 2. Enable recording on track
# 3. Press pad → MIDI note recorded

# Ableton Live: Map virtual MIDI to instrument
# 1. In Ableton: Preferences → MIDI → Enable "MIDIMon Output"
# 2. Add instrument track
# 3. Set MIDI From: MIDIMon Output
# 4. Press pad → triggers instrument

# FL Studio: Map virtual MIDI to channel
# 1. In FL: Options → MIDI Settings → Enable "MIDIMon Output"
# 2. Right-click instrument → Link to controller
# 3. Press pad → triggers channel
```

#### Edge Cases
- **Virtual Port Not Available**: Falls back to first available MIDI output port with warning
- **DAW Not Listening**: MIDI messages sent but not received, no error (fire-and-forget)
- **MIDI Channel Overflow**: Validates channel range (1-16), rejects invalid channels
- **Note Off Missing**: Auto-sends note off after configurable duration (default 100ms)
- **Rapid MIDI Flooding**: Rate limiting to prevent MIDI buffer overflow (max 1000 msgs/sec)
- **SysEx Too Large**: Validates SysEx size (<1KB), splits larger messages
- **Port Name Collision**: Appends "(2)" if port name already exists
- **Platform Without Virtual MIDI**: Shows error on Windows without virtual MIDI driver
- **Bidirectional Routing**: Prevents MIDI feedback loops (ignore messages from virtual output)
- **DAW Crashes**: Virtual port persists, reconnects when DAW restarts

#### Testing Criteria
- ✅ Virtual MIDI port appears in system MIDI devices
- ✅ Note On message sent to virtual port
- ✅ Note Off message sent after note release
- ✅ CC message sent with correct controller and value
- ✅ Program Change message switches DAW preset
- ✅ Pitch Bend message sent with 14-bit value
- ✅ SysEx message sent with proper framing (0xF0...0xF7)
- ✅ Velocity-sensitive MIDI output based on trigger velocity
- ✅ MIDI channel configurable per mapping
- ✅ DAW receives and responds to MIDI messages
- ✅ No MIDI feedback loops when bidirectional routing enabled
- ✅ Rate limiting prevents MIDI buffer overflow

---

### FF2: Velocity Curves

#### Description
Customizable velocity response curves allowing per-device calibration and adjustment of velocity sensitivity, enabling precise control over dynamics and feel for different controller hardware.

#### User Story
> As Alex (Producer), I want to adjust the velocity curve of my controller pads because they feel too sensitive for soft playing, so that I can get more nuanced dynamics without accidentally triggering hard velocity actions.

#### Technical Implementation

**Velocity Curve Types (src/velocity_curves.rs:1-200):**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VelocityCurve {
    Linear,                           // 1:1 mapping
    Exponential { exponent: f32 },    // velocity^exponent
    Logarithmic { base: f32 },        // log(velocity) / log(base)
    SCurve { midpoint: u8, steepness: f32 }, // Sigmoid curve
    Custom { points: Vec<(u8, u8)> }, // Bezier curve through points
    Fixed { value: u8 },              // Always outputs fixed velocity
}

pub struct VelocityMapper {
    curve: VelocityCurve,
    input_range: (u8, u8),   // Min/max input velocity
    output_range: (u8, u8),  // Min/max output velocity
}

impl VelocityMapper {
    pub fn new(curve: VelocityCurve) -> Self {
        Self {
            curve,
            input_range: (0, 127),
            output_range: (0, 127),
        }
    }

    pub fn map(&self, input_velocity: u8) -> u8 {
        // Normalize input to 0.0-1.0
        let normalized = self.normalize_input(input_velocity);

        // Apply curve
        let curved = match &self.curve {
            VelocityCurve::Linear => normalized,

            VelocityCurve::Exponential { exponent } => {
                normalized.powf(*exponent)
            }

            VelocityCurve::Logarithmic { base } => {
                if normalized == 0.0 {
                    0.0
                } else {
                    (normalized.ln() / base.ln()).max(0.0).min(1.0)
                }
            }

            VelocityCurve::SCurve { midpoint, steepness } => {
                let mid = *midpoint as f32 / 127.0;
                let x = (normalized - mid) * steepness;
                1.0 / (1.0 + (-x).exp())
            }

            VelocityCurve::Custom { points } => {
                self.interpolate_custom_curve(normalized, points)
            }

            VelocityCurve::Fixed { value } => {
                *value as f32 / 127.0
            }
        };

        // Denormalize to output range
        self.denormalize_output(curved)
    }

    fn normalize_input(&self, value: u8) -> f32 {
        let (min, max) = self.input_range;
        let clamped = value.clamp(min, max);
        (clamped - min) as f32 / (max - min) as f32
    }

    fn denormalize_output(&self, normalized: f32) -> u8 {
        let (min, max) = self.output_range;
        let scaled = normalized * (max - min) as f32 + min as f32;
        scaled.round() as u8
    }

    fn interpolate_custom_curve(&self, x: f32, points: &[(u8, u8)]) -> f32 {
        // Linear interpolation between custom points
        if points.is_empty() {
            return x;
        }

        let x_scaled = x * 127.0;

        for window in points.windows(2) {
            let (x1, y1) = (window[0].0 as f32, window[0].1 as f32);
            let (x2, y2) = (window[1].0 as f32, window[1].1 as f32);

            if x_scaled >= x1 && x_scaled <= x2 {
                let t = (x_scaled - x1) / (x2 - x1);
                let y = y1 + t * (y2 - y1);
                return y / 127.0;
            }
        }

        x // Fallback
    }
}
```

**Per-Device Curve Configuration (src/config.rs:200-250):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub velocity_curve: Option<VelocityCurve>,
    pub velocity_input_range: Option<(u8, u8)>,
    pub velocity_output_range: Option<(u8, u8)>,
}

impl Config {
    pub fn get_velocity_mapper(&self) -> VelocityMapper {
        if let Some(curve) = &self.device.velocity_curve {
            let mut mapper = VelocityMapper::new(curve.clone());

            if let Some(range) = self.device.velocity_input_range {
                mapper.input_range = range;
            }

            if let Some(range) = self.device.velocity_output_range {
                mapper.output_range = range;
            }

            mapper
        } else {
            VelocityMapper::new(VelocityCurve::Linear)
        }
    }
}
```

#### Configuration Example

**Velocity Curve Presets (config.toml):**
```toml
[device]
name = "Maschine Mikro MK3"

# Linear curve (default, no adjustment)
velocity_curve = "Linear"

# Exponential curve (harder to reach high velocities)
[device.velocity_curve]
type = "Exponential"
exponent = 2.0  # Quadratic (0.5 = square root, 2.0 = squared, 3.0 = cubic)

# Logarithmic curve (easier to reach high velocities)
[device.velocity_curve]
type = "Logarithmic"
base = 2.0

# S-Curve (smooth transition with adjustable midpoint)
[device.velocity_curve]
type = "SCurve"
midpoint = 64    # Velocity at curve midpoint (0-127)
steepness = 5.0  # How steep the curve is (higher = sharper transition)

# Custom curve (define your own points)
[device.velocity_curve]
type = "Custom"
points = [
    [0, 0],      # Very soft → 0
    [20, 10],    # Soft → 10
    [64, 64],    # Medium → 64 (linear midpoint)
    [100, 110],  # Hard → 110
    [127, 127],  # Very hard → 127
]

# Fixed velocity (ignore input, always output same value)
[device.velocity_curve]
type = "Fixed"
value = 80  # Always output velocity 80

# Input/Output range adjustment
velocity_input_range = [10, 120]   # Ignore velocities below 10, cap at 120
velocity_output_range = [20, 100]  # Scale output to 20-100 range
```

**Practical Curve Examples:**
```toml
# Example 1: Pads too sensitive (reduce high velocities)
[device.velocity_curve]
type = "Exponential"
exponent = 1.5

# Example 2: Pads not sensitive enough (boost low velocities)
[device.velocity_curve]
type = "Logarithmic"
base = 1.5

# Example 3: Fine control in middle range
[device.velocity_curve]
type = "SCurve"
midpoint = 64
steepness = 3.0

# Example 4: Drummer-friendly (emphasize hard hits)
[device.velocity_curve]
type = "Custom"
points = [
    [0, 0],
    [40, 20],    # Compress low velocities
    [80, 60],    # Linear middle
    [120, 110],  # Expand high velocities
    [127, 127],
]

# Example 5: Ignore velocity (always trigger at same level)
[device.velocity_curve]
type = "Fixed"
value = 100
```

#### Edge Cases
- **Invalid Curve Parameters**: Validates exponent/base ranges, rejects invalid values
- **Custom Points Out of Order**: Sorts points by X value before interpolation
- **Duplicate Custom Points**: Removes duplicates, keeps first occurrence
- **Division by Zero**: Handles edge cases in logarithmic and S-curve calculations
- **Velocity Overflow**: Clamps output to 0-127 MIDI range
- **Input Range Inversion**: Validates min < max, swaps if inverted
- **Empty Custom Points**: Falls back to linear curve with warning
- **Extreme Exponents**: Caps exponent range (0.1-10.0) to prevent numerical issues
- **Real-Time Curve Switching**: Applies new curve immediately on config reload
- **Per-Mapping Overrides**: Allows velocity curve override per mapping (future enhancement)

#### Testing Criteria
- ✅ Linear curve produces 1:1 velocity mapping
- ✅ Exponential curve with exponent=2.0 produces quadratic response
- ✅ Logarithmic curve boosts low velocities
- ✅ S-Curve produces smooth sigmoid transition
- ✅ Custom curve interpolates correctly between points
- ✅ Fixed velocity outputs constant value regardless of input
- ✅ Input range constrains velocity detection
- ✅ Output range scales mapped velocities
- ✅ Velocity values clamped to 0-127 MIDI range
- ✅ Invalid curve parameters rejected with error
- ✅ Curve changes apply immediately on config reload
- ✅ GUI curve editor shows visual preview of mapping

---

### FF3: Advanced Conditionals

#### Description
Extended conditional logic system with time-based conditions, state variables, and complex boolean expressions, enabling sophisticated context-aware automation beyond basic app/modifier detection.

#### User Story
> As Sam (Developer), I want to create a conditional that only triggers between 9 AM and 5 PM on weekdays when VS Code is active, so that my work-mode mappings don't interfere with personal use.

#### Technical Implementation

**Extended Condition Types (src/conditionals.rs:1-300):**
```rust
use serde::{Deserialize, Serialize};
use chrono::{Timelike, Datelike, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    // Existing conditions from F20
    AppRunning { bundle_id: String },
    TimeRange { start: String, end: String },
    DayOfWeek { days: Vec<String> },
    ModifierPressed { modifier: String },
    ModeActive { mode: String },

    // New advanced conditions
    DateTime {
        date_range: Option<(String, String)>,
        time_range: Option<(String, String)>,
        days: Option<Vec<String>>,
    },
    Variable {
        name: String,
        operator: ComparisonOperator,
        value: VariableValue,
    },
    LastActionTime {
        action_id: String,
        within_ms: u64,
    },
    Counter {
        name: String,
        operator: ComparisonOperator,
        threshold: i64,
    },
    BatteryLevel {
        operator: ComparisonOperator,
        percentage: u8,
    },
    NetworkConnected,
    ScreenLocked,
    IdleTime {
        operator: ComparisonOperator,
        seconds: u64,
    },
    Not { condition: Box<Condition> },
    And { conditions: Vec<Condition> },
    Or { conditions: Vec<Condition> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

pub struct ConditionEvaluator {
    variables: HashMap<String, VariableValue>,
    counters: HashMap<String, i64>,
    last_action_times: HashMap<String, Instant>,
}

impl ConditionEvaluator {
    pub fn evaluate(&self, condition: &Condition) -> bool {
        match condition {
            Condition::DateTime { date_range, time_range, days } => {
                let now = Local::now();

                // Check date range
                if let Some((start, end)) = date_range {
                    // Parse dates and check if now is within range
                    // Implementation details...
                }

                // Check time range
                if let Some((start, end)) = time_range {
                    let (start_h, start_m) = self.parse_time(start);
                    let (end_h, end_m) = self.parse_time(end);

                    let now_mins = now.hour() * 60 + now.minute();
                    let start_mins = start_h * 60 + start_m;
                    let end_mins = end_h * 60 + end_m;

                    if now_mins < start_mins || now_mins > end_mins {
                        return false;
                    }
                }

                // Check days of week
                if let Some(days) = days {
                    let today = now.weekday().to_string();
                    if !days.contains(&today) {
                        return false;
                    }
                }

                true
            }

            Condition::Variable { name, operator, value } => {
                if let Some(var_value) = self.variables.get(name) {
                    self.compare_values(var_value, operator, value)
                } else {
                    false
                }
            }

            Condition::Counter { name, operator, threshold } => {
                if let Some(count) = self.counters.get(name) {
                    self.compare_int(*count, operator, *threshold)
                } else {
                    false
                }
            }

            Condition::LastActionTime { action_id, within_ms } => {
                if let Some(last_time) = self.last_action_times.get(action_id) {
                    last_time.elapsed().as_millis() < *within_ms as u128
                } else {
                    false
                }
            }

            Condition::Not { condition } => {
                !self.evaluate(condition)
            }

            Condition::And { conditions } => {
                conditions.iter().all(|c| self.evaluate(c))
            }

            Condition::Or { conditions } => {
                conditions.iter().any(|c| self.evaluate(c))
            }

            // ... other conditions
            _ => false,
        }
    }

    pub fn set_variable(&mut self, name: String, value: VariableValue) {
        self.variables.insert(name, value);
    }

    pub fn increment_counter(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn reset_counter(&mut self, name: &str) {
        self.counters.insert(name.to_string(), 0);
    }
}
```

**Action Types for State Management:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionConfig {
    // ... existing actions

    SetVariable {
        name: String,
        value: VariableValue,
    },

    IncrementCounter {
        name: String,
        amount: Option<i64>,
    },

    ResetCounter {
        name: String,
    },
}
```

#### Configuration Example

**Complex Conditionals (config.toml):**
```toml
# Example 1: Work hours on weekdays
[[modes.mappings]]
description = "Work-mode mapping (weekdays 9-5)"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Conditional"
operator = "And"
conditions = [
    { DateTime = { time_range = ["09:00", "17:00"], days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"] } },
    { AppRunning = { bundle_id = "com.microsoft.VSCode" } },
]
then_action = { type = "Launch", app = "Slack" }
else_action = { type = "Launch", app = "Safari" }

# Example 2: Counter-based trigger (only after 3 presses)
[[modes.mappings]]
description = "Increment counter on press"
[modes.mappings.trigger]
type = "Note"
note = 1
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "IncrementCounter", name = "pad1_count" },
    {
        type = "Conditional",
        conditions = [{ Counter = { name = "pad1_count", operator = "GreaterOrEqual", threshold = 3 } }],
        then_action = { type = "Shell", command = "say 'Three presses!'" },
    },
]

# Example 3: Reset counter on different pad
[[modes.mappings]]
description = "Reset counter"
[modes.mappings.trigger]
type = "Note"
note = 2
[modes.mappings.action]
type = "ResetCounter"
name = "pad1_count"

# Example 4: Variable-based state machine
[[modes.mappings]]
description = "Toggle recording state"
[modes.mappings.trigger]
type = "Note"
note = 3
[modes.mappings.action]
type = "Conditional"
conditions = [{ Variable = { name = "recording", operator = "Equal", value = { Boolean = true } } }]
then_action = {
    type = "Sequence",
    actions = [
        { type = "SetVariable", name = "recording", value = { Boolean = false } },
        { type = "Keystroke", keys = "r", modifiers = ["cmd"] },  # Stop recording
    ]
}
else_action = {
    type = "Sequence",
    actions = [
        { type = "SetVariable", name = "recording", value = { Boolean = true } },
        { type = "Keystroke", keys = "r", modifiers = ["cmd"] },  # Start recording
    ]
}

# Example 5: Complex nested conditions
[[modes.mappings]]
description = "Smart workspace switcher"
[modes.mappings.trigger]
type = "Note"
note = 4
[modes.mappings.action]
type = "Conditional"
operator = "And"
conditions = [
    {
        Or = {
            conditions = [
                { DateTime = { time_range = ["06:00", "12:00"] } },  # Morning
                { BatteryLevel = { operator = "GreaterThan", percentage = 50 } },  # Good battery
            ]
        }
    },
    { Not = { condition = { ScreenLocked = {} } } },  # Screen unlocked
    { IdleTime = { operator = "LessThan", seconds = 300 } },  # Active in last 5 min
]
then_action = { type = "Shell", command = "open -a 'Activity Monitor'" }
```

#### Edge Cases
- **Invalid Time Format**: Validates HH:MM format, rejects malformed times
- **Date Parsing Errors**: Shows error for invalid date strings
- **Undefined Variables**: Returns false for conditions checking non-existent variables
- **Counter Overflow**: Caps counters at i64::MAX to prevent overflow
- **Circular Conditionals**: Detects and prevents infinite recursion
- **Performance**: Limits condition nesting depth (max 10 levels)
- **Timezone Handling**: Uses local system timezone for all time-based conditions
- **State Persistence**: Variables and counters reset on app restart (unless saved)
- **Concurrent Modifications**: Thread-safe access to shared state
- **Complex Boolean Logic**: Supports arbitrary AND/OR/NOT nesting

#### Testing Criteria
- ✅ DateTime condition evaluates correctly for time ranges
- ✅ Variable condition compares values correctly
- ✅ Counter increments and compares against threshold
- ✅ Not condition inverts result correctly
- ✅ And condition requires all sub-conditions true
- ✅ Or condition requires at least one sub-condition true
- ✅ Nested conditions evaluate correctly (AND of ORs, etc.)
- ✅ SetVariable action updates variable state
- ✅ IncrementCounter action increases counter
- ✅ ResetCounter action sets counter to zero
- ✅ Battery level condition detects low/high battery
- ✅ Idle time condition detects system activity

---

### FF4: Plugin Architecture

#### Description
Extensible plugin system allowing third-party developers to create custom trigger types, action types, and processing logic through a stable Rust API with dynamic loading and sandboxing.

#### User Story
> As a developer, I want to create a custom trigger that detects specific MIDI SysEx messages from my hardware, so that I can extend MIDIMon's functionality without modifying its source code.

#### Technical Implementation

**Plugin API Traits (src/plugin_api.rs:1-200):**
```rust
use std::any::Any;
use serde_json::Value;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn author(&self) -> &str;
    fn description(&self) -> &str;

    fn on_load(&mut self) -> Result<(), PluginError>;
    fn on_unload(&mut self) -> Result<(), PluginError>;
}

pub trait TriggerPlugin: Plugin {
    fn trigger_type_id(&self) -> &str;

    fn should_trigger(
        &self,
        event: &MidiEvent,
        config: &Value,
    ) -> Result<bool, PluginError>;

    fn config_schema(&self) -> Value;
}

pub trait ActionPlugin: Plugin {
    fn action_type_id(&self) -> &str;

    fn execute(
        &self,
        config: &Value,
        context: &ActionContext,
    ) -> Result<(), PluginError>;

    fn config_schema(&self) -> Value;
}

pub trait ProcessorPlugin: Plugin {
    fn process_event(
        &self,
        event: MidiEvent,
    ) -> Result<Vec<ProcessedEvent>, PluginError>;
}

pub struct ActionContext {
    pub velocity: Option<u8>,
    pub mode: usize,
    pub timestamp: Instant,
}
```

**Plugin Loader (src/plugin_loader.rs:1-250):**
```rust
use libloading::{Library, Symbol};
use std::path::Path;

type PluginCreate = unsafe fn() -> *mut dyn Plugin;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>,
    plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self {
            plugins: Vec::new(),
            libraries: Vec::new(),
            plugin_dir,
        }
    }

    pub fn load_all(&mut self) -> Result<(), PluginError> {
        for entry in std::fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if self.is_plugin_library(&path) {
                match self.load_plugin(&path) {
                    Ok(_) => println!("✅ Loaded plugin: {}", path.display()),
                    Err(e) => eprintln!("❌ Failed to load plugin {}: {}", path.display(), e),
                }
            }
        }
        Ok(())
    }

    unsafe fn load_plugin(&mut self, path: &Path) -> Result<(), PluginError> {
        let lib = Library::new(path)?;

        let create: Symbol<PluginCreate> = lib.get(b"create_plugin")?;
        let plugin = Box::from_raw(create());

        // Validate plugin
        self.validate_plugin(&plugin)?;

        // Initialize plugin
        plugin.on_load()?;

        println!("   Name: {}", plugin.name());
        println!("   Version: {}", plugin.version());
        println!("   Author: {}", plugin.author());

        self.plugins.push(plugin);
        self.libraries.push(lib);

        Ok(())
    }

    fn is_plugin_library(&self, path: &Path) -> bool {
        match path.extension().and_then(|s| s.to_str()) {
            Some("so") => cfg!(target_os = "linux"),
            Some("dylib") => cfg!(target_os = "macos"),
            Some("dll") => cfg!(target_os = "windows"),
            _ => false,
        }
    }

    fn validate_plugin(&self, plugin: &Box<dyn Plugin>) -> Result<(), PluginError> {
        // Check name not empty
        if plugin.name().is_empty() {
            return Err(PluginError::InvalidPlugin("Name cannot be empty".into()));
        }

        // Check version format
        semver::Version::parse(plugin.version())
            .map_err(|_| PluginError::InvalidPlugin("Invalid version format".into()))?;

        // Check for name collisions
        for existing in &self.plugins {
            if existing.name() == plugin.name() {
                return Err(PluginError::PluginAlreadyLoaded(plugin.name().to_string()));
            }
        }

        Ok(())
    }

    pub fn get_trigger_plugin(&self, type_id: &str) -> Option<&dyn TriggerPlugin> {
        for plugin in &self.plugins {
            if let Some(trigger_plugin) = plugin.as_any().downcast_ref::<dyn TriggerPlugin>() {
                if trigger_plugin.trigger_type_id() == type_id {
                    return Some(trigger_plugin);
                }
            }
        }
        None
    }
}
```

**Example Plugin (plugins/sysex_trigger/src/lib.rs):**
```rust
use midimon_plugin_api::*;
use serde_json::{json, Value};

pub struct SysExTriggerPlugin;

impl Plugin for SysExTriggerPlugin {
    fn name(&self) -> &str {
        "SysEx Trigger"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn author(&self) -> &str {
        "Your Name"
    }

    fn description(&self) -> &str {
        "Triggers on specific SysEx messages"
    }

    fn on_load(&mut self) -> Result<(), PluginError> {
        println!("SysEx Trigger plugin loaded!");
        Ok(())
    }

    fn on_unload(&mut self) -> Result<(), PluginError> {
        println!("SysEx Trigger plugin unloaded!");
        Ok(())
    }
}

impl TriggerPlugin for SysExTriggerPlugin {
    fn trigger_type_id(&self) -> &str {
        "SysEx"
    }

    fn should_trigger(&self, event: &MidiEvent, config: &Value) -> Result<bool, PluginError> {
        if let MidiEvent::SysEx { data } = event {
            if let Some(pattern) = config.get("pattern").and_then(|v| v.as_array()) {
                let pattern_bytes: Vec<u8> = pattern
                    .iter()
                    .filter_map(|v| v.as_u64().map(|n| n as u8))
                    .collect();

                // Check if data starts with pattern
                return Ok(data.starts_with(&pattern_bytes));
            }
        }
        Ok(false)
    }

    fn config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "array",
                    "items": { "type": "integer", "minimum": 0, "maximum": 255 },
                    "description": "SysEx byte pattern to match"
                }
            },
            "required": ["pattern"]
        })
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(SysExTriggerPlugin))
}
```

#### Configuration Example

**Plugin Configuration (config.toml):**
```toml
[plugins]
enabled = true
plugin_dir = "~/.midimon/plugins"
auto_load = true

# Whitelist specific plugins (security)
allowed_plugins = [
    "SysEx Trigger",
    "OSC Output",
    "MIDI Filter",
]

# Example: Use custom trigger from plugin
[[modes.mappings]]
description = "Custom SysEx trigger"
[modes.mappings.trigger]
type = "SysEx"  # Plugin-provided type
pattern = [0xF0, 0x43, 0x12, 0x00, 0xF7]
[modes.mappings.action]
type = "Shell"
command = "echo 'SysEx received!'"
```

**Plugin Development:**
```bash
# Create new plugin
cargo new --lib midimon-plugin-sysex
cd midimon-plugin-sysex

# Add dependencies
cargo add midimon-plugin-api serde_json

# Build plugin
cargo build --release

# Install plugin
cp target/release/libmidimon_plugin_sysex.dylib ~/.midimon/plugins/

# MIDIMon will auto-load on next startup
```

#### Edge Cases
- **Plugin Crashes**: Isolates plugin errors, continues operation without crashed plugin
- **ABI Incompatibility**: Validates plugin API version, rejects incompatible plugins
- **Circular Dependencies**: Detects and prevents plugins loading other plugins
- **Resource Leaks**: Calls `on_unload()` during shutdown to cleanup resources
- **Name Collisions**: Rejects plugins with duplicate names
- **Security**: Sandboxing limits plugin access to filesystem/network (future)
- **Performance**: Plugins run in main thread, slow plugins block event processing
- **Hot Reload**: Unload/reload plugins without restarting MIDIMon
- **Version Conflicts**: Allows multiple versions of same plugin if different type IDs
- **Dynamic Config**: Plugin config changes apply immediately via hot-reload

#### Testing Criteria
- ✅ Plugin API compiles against stable Rust ABI
- ✅ Plugin loads from .so/.dylib/.dll file
- ✅ create_plugin() function exports plugin instance
- ✅ Plugin on_load() called during initialization
- ✅ Plugin on_unload() called during shutdown
- ✅ Custom trigger plugin matches MIDI events correctly
- ✅ Custom action plugin executes actions correctly
- ✅ Plugin validation rejects invalid plugins
- ✅ Plugin isolation prevents crashes from affecting main app
- ✅ Multiple plugins coexist without conflicts
- ✅ Plugin config changes reload plugin behavior
- ✅ Plugin directory auto-discovered on startup

---

## Feature Summary

### Current (v0.1.0)
✅ Event Processing Pipeline (F1)
✅ Velocity Sensitivity (F2)
✅ Long Press Detection (F3)
✅ Double-Tap Detection (F4)
✅ Chord Detection (F5)
✅ Encoder Direction Detection (F6)
✅ Aftertouch Trigger (F7)
✅ PitchBend Trigger (F8)
✅ CC Trigger (F9)
✅ Keystroke Action (F10)
✅ Text Action (F11)
✅ Launch Application (F12)
✅ Shell Command (F13)
✅ Volume Control (F14)
✅ Mode Change (F15)
✅ Action Sequence (F16)
✅ Delay Action (F17)
✅ MouseClick Action (F18)
✅ Repeat Action (F19)
✅ Conditional Action (F20)
✅ Multi-Mode System (F21)
✅ Global Mappings (F22)
✅ RGB LED Feedback (F23)
✅ MIDI LED Feedback (F24)
✅ LED Lighting Schemes (F25)
✅ Device Profile Support (F26)

### Target (v2.0)
🔄 Menu Bar Application
🔄 Visual Configuration UI
🔄 Hot Config Reload
🔄 Per-App Profile Switching

### Future (v2.5+)
🔄 Virtual MIDI Output
🔄 Profile Marketplace
🔄 Advanced Conditionals
🔄 Variable System
🔄 API Integration

---

**Document History:**
- v1.0 (2025-11-11): Initial feature specifications (F1-F9)
- v1.1 (2025-11-11): Added 8 missing feature specifications (F7-F8 enhanced, F11-F12, F14-F16, F18)
- v1.2 (2025-11-11): Added Phase 1-2 specifications (F17, F19-F26) - 12 features total
  - F17: Delay Action (full spec)
  - F19: Repeat Action (full spec)
  - F20: Conditional Action (full spec)
  - F21: Multi-Mode System (full spec)
  - F22: Global Mappings (full spec)
  - F23: RGB LED Feedback (full spec)
  - F24: MIDI LED Feedback (full spec)
  - F25: LED Lighting Schemes (full spec)
  - F26: Device Profile Support (full spec)
- v1.3 (2025-11-11): ✅ Complete - Added Phase 3-5 specifications (30 features total)
  - Phase 3 (TF1-TF10): MIDI Learn, Config Hot-Reload, Menu Bar UI, Visual Config Editor, Device Templates, Frontmost App Detection, Per-App Profiles, Auto-Start on Boot, Live Event Console, Profile Sharing/Export
  - Phase 4 (FF1-FF4): Virtual MIDI Output (enhanced), Velocity Curves, Advanced Conditionals, Plugin Architecture
  - Phase 5 (F7, F8, F14, F15 enhancements): Added pressure curves, device compatibility matrix, continuous message handling (F7); 14-bit normalization, throttling strategies, spring-back detection (F8); platform implementation details, latency characteristics, dependency requirements (F14); transition effects, LED feedback integration, mode wrapping (F15)
  - **Final Status**: 51/51 features (100%) fully specified with comprehensive documentation
