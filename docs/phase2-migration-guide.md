# Phase 2 Migration Guide: Extracting midimon-core

**Status**: Planning (Phase 1)
**Target**: Phase 2 Implementation
**Last Updated**: 2025-11-11

## Overview

This guide documents the migration from the current monolithic binary to a workspace structure with `midimon-core` as a reusable library crate.

## Migration Goals

1. **Zero Breaking Changes**: Existing `config.toml` format unchanged
2. **Feature Preservation**: All current features work identically
3. **Clean Boundaries**: Clear public/private API separation
4. **UI Independence**: Core library has no UI dependencies (no `colored`, no `chrono` for display)
5. **Backward Compatibility**: CLI arguments and behavior preserved

## Workspace Structure

### Before (Phase 1)

```
midimon/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── event_processor.rs
│   ├── mappings.rs
│   ├── actions.rs
│   ├── feedback.rs
│   ├── mikro_leds.rs
│   ├── midi_feedback.rs
│   └── device_profile.rs
└── config.toml
```

### After (Phase 2)

```
midimon/
├── Cargo.toml                      # Workspace root
├── midimon-core/                   # Pure Rust engine crate (UI-free)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                  # Public API exports
│       ├── engine.rs               # MidiMonEngine (public)
│       ├── config.rs               # Config types (public)
│       ├── events.rs               # Event types (public)
│       ├── actions.rs              # Action types/executor (public)
│       ├── mapping.rs              # Mapping engine (public)
│       ├── feedback.rs             # Feedback traits (public)
│       ├── device.rs               # Device profiles (public)
│       ├── error.rs                # Error types (public)
│       ├── event_processor.rs     # State machine (PRIVATE)
│       ├── timing.rs               # Timing detection (PRIVATE)
│       ├── velocity.rs             # Velocity processing (PRIVATE)
│       ├── chord.rs                # Chord detection (PRIVATE)
│       ├── mikro_leds.rs           # HID LED impl (PRIVATE)
│       └── midi_feedback.rs        # MIDI LED impl (PRIVATE)
├── midimon-cli/                    # CLI binary (current functionality)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs                 # CLI entry point
└── config/
    └── default.toml                # Default config template
```

## Public vs Private API

### Public API (Exposed by midimon-core)

These types and functions will be exported in `midimon-core/src/lib.rs`:

```rust
// Core engine
pub use engine::MidiMonEngine;
pub use engine::EngineStats;

// Configuration
pub use config::{Config, DeviceConfig, Mode, Mapping, AdvancedSettings};
pub use config::{Trigger, ActionConfig};

// Events
pub use events::{MidiEvent, ProcessedEvent};
pub use events::{VelocityLevel, EncoderDirection};

// Actions
pub use actions::{Action, VolumeAction, Condition};
pub use actions::{ActionExecutor};  // Trait for custom executors

// Feedback
pub use feedback::{FeedbackController, RGB, LightingScheme};
pub use feedback::create_feedback_device;

// Device profiles
pub use device::{DeviceProfile, PadPageMapping};

// Errors
pub use error::{EngineError, ConfigError, ActionError, FeedbackError, ProfileError};
```

### Private Implementation (Hidden)

These modules will remain `mod` (not `pub mod`) in `midimon-core/src/lib.rs`:

```rust
// Event processing internals
mod event_processor;      // EventProcessor struct + state machine
mod timing;               // Long press/double-tap detection logic
mod velocity;             // Velocity range processing
mod chord;                // Chord detection buffer

// Device implementations
mod mikro_leds;           // MikroMK3LEDs (HID) - private, accessed via factory
mod midi_feedback;        // MidiFeedback (MIDI) - private, accessed via factory

// Utility/parsing
mod key_parser;           // parse_key, parse_modifier functions
mod midi_parser;          // Raw MIDI byte parsing
```

## Dependency Changes

### midimon-core/Cargo.toml

```toml
[package]
name = "midimon-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core MIDI/HID
midir = "0.10"
hidapi = { version = "2.6", features = ["macos-shared-device"] }

# Input simulation
enigo = { version = "0.2", default-features = false }

# Config parsing
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Device profiles
quick-xml = { version = "0.36", features = ["serialize"] }

# Concurrency
crossbeam-channel = "0.5"

# Error handling
thiserror = "1.0"

# REMOVED (moved to CLI layer):
# - colored (terminal output)
# - chrono (timestamp formatting)
# - ctrlc (signal handling)
```

### midimon-cli/Cargo.toml

```toml
[package]
name = "midimon-cli"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "midimon"
path = "src/main.rs"

[dependencies]
# Core engine
midimon-core = { path = "../midimon-core" }

# CLI-specific
colored = "2.1"
chrono = "0.4"
ctrlc = "3.4"
clap = { version = "4.5", features = ["derive"] }  # New: better CLI parsing
```

## Migration Steps

### Step 1: Create Workspace

**File**: `Cargo.toml` (root)

```toml
[workspace]
members = ["midimon-core", "midimon-cli"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Amiable"]
license = "MIT"
```

### Step 2: Create midimon-core

1. **Create directory structure**:
   ```bash
   mkdir -p midimon-core/src
   ```

2. **Move and refactor source files**:
   ```bash
   # Move core logic files
   cp src/config.rs midimon-core/src/
   cp src/event_processor.rs midimon-core/src/
   cp src/mappings.rs midimon-core/src/mapping.rs  # Rename
   cp src/actions.rs midimon-core/src/
   cp src/feedback.rs midimon-core/src/
   cp src/mikro_leds.rs midimon-core/src/
   cp src/midi_feedback.rs midimon-core/src/
   cp src/device_profile.rs midimon-core/src/device.rs  # Rename
   ```

3. **Create public API surface** (`midimon-core/src/lib.rs`):
   ```rust
   // Public modules
   pub mod config;
   pub mod engine;
   pub mod events;
   pub mod actions;
   pub mod mapping;
   pub mod feedback;
   pub mod device;
   pub mod error;

   // Private modules
   mod event_processor;
   mod timing;
   mod velocity;
   mod chord;
   mod mikro_leds;
   mod midi_feedback;

   // Re-exports for convenience
   pub use engine::MidiMonEngine;
   pub use config::Config;
   // ... (see Public API section above)
   ```

4. **Create engine.rs** (new file):
   - Extract engine logic from current `main.rs`
   - Remove UI dependencies (colored, chrono display)
   - Add callback support (`on_mode_change`, `on_action`)
   - Implement `MidiMonEngine` struct with public methods

5. **Create error.rs** (new file):
   ```rust
   use thiserror::Error;

   #[derive(Debug, Error)]
   pub enum EngineError {
       #[error("MIDI connection failed: {0}")]
       MidiConnectionFailed(String),
       // ... (see API design doc)
   }
   ```

6. **Refactor existing files**:
   - **config.rs**: Remove `colored` output, keep pure data types
   - **event_processor.rs**: Remove `colored` logging, keep state machine
   - **actions.rs**: Keep executor, remove display formatting
   - **feedback.rs**: Already trait-based, minimal changes

### Step 3: Create midimon-cli

1. **Create directory structure**:
   ```bash
   mkdir -p midimon-cli/src
   ```

2. **Create main.rs**:
   ```rust
   use midimon_core::{Config, MidiMonEngine};
   use colored::Colorize;
   use chrono::Local;
   use clap::Parser;

   #[derive(Parser)]
   #[command(name = "midimon")]
   #[command(about = "MIDI Macro Pad Controller")]
   struct Cli {
       /// MIDI port index to connect to
       port: Option<usize>,

       /// LED lighting scheme
       #[arg(long)]
       led: Option<String>,

       /// Device profile path
       #[arg(long)]
       profile: Option<String>,

       /// Pad page (A-H)
       #[arg(long)]
       pad_page: Option<char>,
   }

   fn main() -> Result<(), Box<dyn std::error::Error>> {
       let cli = Cli::parse();

       // Load config
       let config = Config::load("config.toml")?;

       // Create engine
       let mut engine = MidiMonEngine::new(config)?;

       // Register callbacks for CLI output
       engine.on_mode_change(|mode| {
           println!("{} {}",
               "Mode changed:".cyan().bold(),
               mode.to_string().yellow()
           );
       });

       engine.on_action(|event, action| {
           let timestamp = Local::now().format("%H:%M:%S%.3f");
           println!("{} {:?} -> {:?}",
               timestamp.to_string().dimmed(),
               event,
               action
           );
       });

       // Start engine (blocking)
       engine.start()?;

       Ok(())
   }
   ```

### Step 4: Testing Strategy

1. **Unit Tests** (midimon-core):
   - Config parsing/validation
   - Event processing state machine
   - Trigger matching
   - Velocity detection
   - Chord detection

2. **Integration Tests** (midimon-cli):
   - Full pipeline (MIDI → Action)
   - CLI argument parsing
   - Config hot-reload
   - Signal handling

3. **Regression Tests**:
   - Run existing test suite
   - Verify all features work identically
   - Check config.toml compatibility

### Step 5: Documentation Updates

Update these files:
- `README.md`: Add workspace structure section
- `CLAUDE.md`: Update architecture section
- `docs-site/src/development/architecture.md`: Already updated
- `docs-site/src/getting-started/quick-start.md`: Verify CLI commands unchanged

## Validation Checklist

Before merging Phase 2 changes:

- [ ] All existing features work identically
- [ ] Config.toml format unchanged (backward compatible)
- [ ] CLI arguments unchanged (backward compatible)
- [ ] Performance characteristics match (latency <1ms)
- [ ] All tests pass (unit + integration)
- [ ] Documentation updated
- [ ] No UI dependencies in midimon-core
- [ ] Public API documented with rustdoc
- [ ] Examples provided for integration
- [ ] GitHub Actions CI passes
- [ ] Manual testing on macOS completed

## Breaking Changes (None Expected)

**If any breaking changes are discovered during migration, document here and update semver accordingly.**

## Rollback Plan

If Phase 2 migration encounters critical issues:

1. **Git Tag**: Ensure `v0.1.0-monolithic` tag exists (created before migration)
2. **Revert**: `git checkout v0.1.0-monolithic`
3. **Hotfix Branch**: Create branch from tag for urgent fixes
4. **Migration Pause**: Address blockers before resuming

## Success Criteria

Phase 2 migration is successful when:

1. `cargo build --release` produces identical binary behavior
2. All existing config files load without errors
3. Event latency remains <1ms
4. All diagnostic tools work (`led_diagnostic`, `midi_diagnostic`)
5. LED feedback schemes work identically
6. Device profiles load correctly
7. All tests pass
8. Documentation reflects new structure
9. CI/CD pipeline updated and passing

## Next Steps (Phase 3)

Once Phase 2 is complete and validated:

1. Create `midimon-daemon/` crate
2. Add menu bar integration (macOS)
3. Implement config hot-reloading via `notify`
4. Add auto-start via LaunchAgent
5. Create system tray UI (Tauri or Cacao)

## References

- [API Design Document](api-design.md)
- [Architecture Overview](../docs-site/src/development/architecture.md)
- [Implementation Viewpoint 1](.research/implementation-viewpoint-1.md)
- [Implementation Viewpoint 2](.research/implementation-viewpoint-2.md)
- [Current monolithic implementation](../src/)
