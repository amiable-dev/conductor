# Phase 2 Migration Checklist: Monorepo Workspace Structure

**Version**: 1.0
**Status**: Ready for Phase 2 Execution
**Last Updated**: 2025-11-11
**Duration**: 3-4 weeks
**Target Completion**: ~2025-12-09

## Overview

This checklist provides a comprehensive, step-by-step guide for executing Phase 2 of the MIDIMon migration. It builds on the design documents (AMI-123, AMI-124, AMI-125, and this guide AMI-126) and provides specific commands, validation procedures, and rollback instructions.

**Success Criteria**: Create a working monorepo workspace with `midimon-core` library crate and `midimon-daemon` CLI binary, maintaining 100% backward compatibility with existing configurations and functionality.

---

## Pre-Migration Tasks (Preparation Phase)

**Estimated Duration**: 4-6 hours
**Responsible**: Tech Lead / Project Manager
**Blocking**: All subsequent phases

### Preparation Phase Checklist

- [ ] **1.1 - Backup Repository** (30 min)
  - **Action**: Create complete backup of working repository
  - **Command**:
    ```bash
    # Create backup archive
    cp -r /Users/christopherjoseph/projects/amiable/midimon ~/midimon-backup-v0.1.0

    # Verify backup
    ls -lah ~/midimon-backup-v0.1.0
    ```
  - **Validation**: Backup directory exists and contains all files
  - **Rollback**: Copy backup back if critical issues occur early
  - **Notes**: Keep for minimum 2 weeks after Phase 2 completion

- [ ] **1.2 - Verify Current Tests Pass** (30 min)
  - **Action**: Ensure all existing tests pass before migration
  - **Command**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon
    cargo test --all
    cargo test --release
    ```
  - **Expected Output**:
    ```
    test result: ok. X passed; 0 failed; 0 ignored; 0 measured
    ```
  - **Validation Criteria**:
    - All unit tests pass
    - All integration tests pass
    - No compilation warnings (address if any)
  - **If Fails**: Debug and fix before proceeding

- [ ] **1.3 - Create v0.1.0 Preservation Tag** (15 min)
  - **Action**: Tag current working implementation for reference
  - **Command**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon
    git add -A
    git commit -m "Preserve v0.1.0 state before Phase 2 migration

    - Current monolithic implementation
    - All features functional
    - Backward compatibility baseline
    - Safe rollback point"

    git tag -a v0.1.0-monolithic \
      -m "MIDIMon v0.1.0 - Monolithic Implementation (Pre-Phase 2)

    Current working state with all 26 features:
    - MIDI device connections
    - Velocity-sensitive mappings
    - Long press & double-tap detection
    - Chord detection
    - Encoder rotation with modes
    - RGB LED feedback
    - Device profiles support
    - Config file management

    Phase 2 will refactor to workspace structure while preserving functionality."

    git push origin v0.1.0-monolithic
    ```
  - **Validation**:
    - Tag created: `git tag -l | grep v0.1.0-monolithic`
    - Tag pushed to remote: `git ls-remote origin v0.1.0-monolithic`
  - **Notes**: This tag is the ultimate rollback point

- [ ] **1.4 - Review Dependency Documentation** (1 hour)
  - **Action**: Read all Phase 2 planning documents in order
  - **Documents to Review**:
    1. `/docs/api-design.md` (AMI-123) - Public API surface
    2. `/docs/workspace-structure.md` (AMI-124) - Directory layout
    3. `/docs/config-compatibility.md` (AMI-125) - Backward compatibility
    4. `/docs/phase2-migration-guide.md` - Technical migration steps
  - **Verification**: Understand:
    - [ ] What public vs private API will be
    - [ ] How dependencies flow between crates
    - [ ] What compatibility guarantees must be maintained
    - [ ] What the final structure will look like
  - **Notes**: Reference these documents throughout migration

- [ ] **1.5 - Setup Development Environment** (30 min)
  - **Action**: Ensure all development tools are available
  - **Commands**:
    ```bash
    # Check Rust version (need 1.70+)
    rustc --version
    cargo --version

    # Install/update tools
    rustup update
    cargo install cargo-watch
    cargo install cargo-tree

    # Verify clippy and fmt
    cargo clippy --version
    cargo fmt --version
    ```
  - **Expected Output**: All tools available, Rust ≥1.70
  - **If Missing**: Install via `rustup`

- [ ] **1.6 - Create Feature Branch** (10 min)
  - **Action**: Create isolated branch for Phase 2 work
  - **Command**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon
    git checkout -b phase-2/workspace-migration
    git push -u origin phase-2/workspace-migration
    ```
  - **Validation**: Branch created and pushed to remote
  - **Notes**: All Phase 2 work happens on this branch

---

## Workspace Creation (Step 1)

**Estimated Duration**: 1-2 hours
**Responsible**: Build Engineer / Tech Lead
**Dependencies**: Preparation phase complete

### Workspace Creation Checklist

- [ ] **2.1 - Create Workspace Root Manifest** (30 min)
  - **Action**: Create new `Cargo.toml` at repository root with workspace configuration
  - **Current State**:
    - Single `Cargo.toml` defining the monolithic binary
  - **Target State**:
    - Workspace `Cargo.toml` at root
    - Individual crate manifests in subdirectories
  - **File Path**: `/Users/christopherjoseph/projects/amiable/midimon/Cargo.toml`
  - **Commands**:
    ```bash
    # Backup current Cargo.toml
    mv Cargo.toml Cargo.toml.v0.1.0-backup

    # Create new workspace root (see details below)
    cat > Cargo.toml << 'EOF'
    [workspace]
    members = [
        "midimon-core",
        "midimon-daemon",
    ]
    resolver = "2"

    [workspace.package]
    version = "0.2.0"
    edition = "2021"
    authors = ["MIDIMon Contributors"]
    license = "MIT"
    repository = "https://github.com/amiable-dev/midimon"
    homepage = "https://midimon.dev"

    [workspace.dependencies]
    # Core dependencies
    serde = { version = "1.0", features = ["derive"] }
    toml = "0.8"
    midir = "0.10"
    enigo = { version = "0.2", default-features = false }
    hidapi = { version = "2.6", features = ["macos-shared-device"] }
    quick-xml = { version = "0.36", features = ["serialize"] }
    crossbeam-channel = "0.5"
    thiserror = "1.0"

    # CLI/display dependencies (NOT in core)
    colored = "2.1"
    chrono = "0.4"
    ctrlc = "3.4"

    # Testing
    proptest = "1.5"
    rstest = "0.23"
    EOF
    ```
  - **Validation**:
    ```bash
    cargo metadata --format-version 1 | grep workspace
    ```
  - **Success**: Workspace metadata contains member crates

- [ ] **2.2 - Create Directory Structure** (30 min)
  - **Action**: Create subdirectories for workspace crates
  - **Commands**:
    ```bash
    # Create core library structure
    mkdir -p midimon-core/src
    mkdir -p midimon-core/tests
    mkdir -p midimon-core/tests/fixtures

    # Create daemon binary structure
    mkdir -p midimon-daemon/src/bin
    mkdir -p midimon-daemon/tests

    # Create config directory
    mkdir -p config/examples
    mkdir -p config/device_templates

    # Verify structure
    tree -L 2 -d -a
    ```
  - **Expected Output**:
    ```
    midimon/
    ├── midimon-core/
    │   ├── src/
    │   └── tests/
    ├── midimon-daemon/
    │   ├── src/
    │   └── tests/
    └── config/
    ```
  - **Validation**: All directories exist

- [ ] **2.3 - Create midimon-core/Cargo.toml** (15 min)
  - **Action**: Define core library crate manifest
  - **File Path**: `/Users/christopherjoseph/projects/amiable/midimon/midimon-core/Cargo.toml`
  - **Content**:
    ```toml
    [package]
    name = "midimon-core"
    version.workspace = true
    edition.workspace = true
    authors.workspace = true
    license.workspace = true
    description = "MIDIMon core MIDI mapping engine (UI-independent)"

    [lib]
    name = "midimon_core"
    path = "src/lib.rs"

    [dependencies]
    # Core engine dependencies
    midir.workspace = true
    enigo.workspace = true
    serde.workspace = true
    toml.workspace = true
    hidapi.workspace = true
    quick-xml.workspace = true
    crossbeam-channel.workspace = true
    thiserror.workspace = true

    [dev-dependencies]
    proptest.workspace = true
    rstest.workspace = true

    [features]
    default = []
    test-mocks = []
    ```
  - **Validation**:
    ```bash
    cd midimon-core && cargo metadata | grep '"name":"midimon-core"'
    ```
  - **Success**: Crate metadata shows midimon-core

- [ ] **2.4 - Create midimon-daemon/Cargo.toml** (15 min)
  - **Action**: Define CLI daemon binary crate manifest
  - **File Path**: `/Users/christopherjoseph/projects/amiable/midimon/midimon-daemon/Cargo.toml`
  - **Content**:
    ```toml
    [package]
    name = "midimon-daemon"
    version.workspace = true
    edition.workspace = true
    authors.workspace = true
    license.workspace = true
    description = "MIDIMon CLI daemon and diagnostic tools"

    [[bin]]
    name = "midimon"
    path = "src/main.rs"

    # Diagnostic tools
    [[bin]]
    name = "midi_diagnostic"
    path = "src/bin/midi_diagnostic.rs"

    [[bin]]
    name = "led_diagnostic"
    path = "src/bin/led_diagnostic.rs"

    [[bin]]
    name = "led_tester"
    path = "src/bin/led_tester.rs"

    [[bin]]
    name = "pad_mapper"
    path = "src/bin/pad_mapper.rs"

    [[bin]]
    name = "test_midi"
    path = "src/bin/test_midi.rs"

    [[bin]]
    name = "midi_simulator"
    path = "src/bin/midi_simulator.rs"

    [dependencies]
    # Core engine
    midimon-core = { path = "../midimon-core" }

    # CLI dependencies
    colored.workspace = true
    chrono.workspace = true
    ctrlc.workspace = true

    [dev-dependencies]
    rstest.workspace = true

    [profile.release]
    opt-level = 3
    lto = true
    codegen-units = 1
    strip = true
    ```
  - **Validation**:
    ```bash
    cd midimon-daemon && cargo metadata | grep '"name":"midimon-daemon"'
    ```
  - **Success**: Crate metadata shows midimon-daemon

- [ ] **2.5 - Verify Workspace Build** (20 min)
  - **Action**: Test workspace configuration before code migration
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Check workspace structure
    cargo metadata --format-version 1 | jq '.workspace_members'

    # List members
    cargo tree --workspace

    # Try workspace build (will fail - no code yet, that's OK)
    cargo build --workspace 2>&1 | head -20
    ```
  - **Expected Output**:
    - Workspace members listed: `midimon-core`, `midimon-daemon`
    - Build fails with "cannot find `lib.rs`" - expected, code migration next
  - **If Fails**: Check Cargo.toml syntax, fix paths

---

## Core Library Extraction (Step 2)

**Estimated Duration**: 4-6 hours
**Responsible**: Senior Dev / Architect
**Dependencies**: Workspace creation complete

### Core Library Extraction Checklist

- [ ] **3.1 - Copy Core Modules** (1 hour)
  - **Action**: Move existing source files to midimon-core
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Copy core modules (these will be modified)
    cp src/config.rs midimon-core/src/
    cp src/event_processor.rs midimon-core/src/
    cp src/mappings.rs midimon-core/src/mapping.rs
    cp src/actions.rs midimon-core/src/
    cp src/feedback.rs midimon-core/src/
    cp src/mikro_leds.rs midimon-core/src/
    cp src/midi_feedback.rs midimon-core/src/
    cp src/device_profile.rs midimon-core/src/device.rs

    # Verify all files copied
    ls -la midimon-core/src/
    ```
  - **Expected Files**:
    - config.rs
    - event_processor.rs
    - mapping.rs
    - actions.rs
    - feedback.rs
    - mikro_leds.rs
    - midi_feedback.rs
    - device.rs
  - **Validation**: All files present
  - **Notes**: Files will be refactored next

- [ ] **3.2 - Create New Core Modules** (1 hour)
  - **Action**: Create error types, engine, and extracted logic modules
  - **File: midimon-core/src/error.rs** (new)
    ```bash
    cat > midimon-core/src/error.rs << 'EOF'
    use thiserror::Error;

    /// Engine-level errors
    #[derive(Debug, Error)]
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

    /// Configuration parsing errors
    #[derive(Debug, Error)]
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
    #[derive(Debug, Error)]
    pub enum ActionError {
        #[error("Execution failed: {0}")]
        ExecutionFailed(String),

        #[error("Invalid key: {0}")]
        InvalidKey(String),

        #[error("Application not found: {0}")]
        AppNotFound(String),
    }

    /// LED feedback errors
    #[derive(Debug, Error)]
    pub enum FeedbackError {
        #[error("Device not connected")]
        NotConnected,

        #[error("HID error: {0}")]
        HidError(String),

        #[error("MIDI error: {0}")]
        MidiError(String),
    }

    /// Device profile errors
    #[derive(Debug, Error)]
    pub enum ProfileError {
        #[error("IO error: {0}")]
        IoError(#[from] std::io::Error),

        #[error("XML parse error: {0}")]
        XmlError(String),

        #[error("Invalid profile: {0}")]
        InvalidProfile(String),
    }
    EOF
    ```
  - **File: midimon-core/src/events.rs** (new)
    ```bash
    cat > midimon-core/src/events.rs << 'EOF'
    use std::time::Instant;

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
        ShortPress { note: u8 },
        MediumPress { note: u8, duration_ms: u128 },
        LongPress { note: u8, duration_ms: u128 },
        HoldDetected { note: u8 },
        PadPressed { note: u8, velocity: u8, velocity_level: VelocityLevel },
        PadReleased { note: u8, hold_duration_ms: u128 },
        EncoderTurned { cc: u8, value: u8, direction: EncoderDirection, delta: u8 },
        DoubleTap { note: u8 },
        ChordDetected { notes: Vec<u8> },
        AftertouchChanged { pressure: u8 },
        PitchBendMoved { value: u16 },
    }

    /// Velocity sensitivity levels
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum VelocityLevel {
        Soft,
        Medium,
        Hard,
    }

    /// Encoder rotation direction
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EncoderDirection {
        Clockwise,
        CounterClockwise,
    }
    EOF
    ```
  - **File: midimon-core/src/engine.rs** (new)
    - Copy engine logic from current `src/main.rs` (main event loop)
    - Remove colored/chrono display output
    - Add callback support for mode changes and action execution
    - Add `start()` and `stop()` methods
    - See `docs/api-design.md` for full specification
  - **Validation**: All three files compile
    ```bash
    cd midimon-core && cargo check
    ```

- [ ] **3.3 - Create lib.rs Public API** (1.5 hours)
  - **Action**: Define public API surface in lib.rs
  - **File Path**: `midimon-core/src/lib.rs`
  - **Content**:
    ```bash
    cat > midimon-core/src/lib.rs << 'EOF'
    // Public modules
    pub mod config;
    pub mod engine;
    pub mod events;
    pub mod actions;
    pub mod mapping;
    pub mod feedback;
    pub mod device;
    pub mod error;

    // Private modules (implementation details)
    mod event_processor;
    mod timing;
    mod velocity;
    mod chord;
    mod mikro_leds;
    mod midi_feedback;

    // Re-exports for convenience
    pub use engine::MidiMonEngine;
    pub use config::Config;
    pub use events::{MidiEvent, ProcessedEvent, VelocityLevel, EncoderDirection};
    pub use actions::{Action, VolumeAction, Condition};
    pub use feedback::{FeedbackController, RGB, LightingScheme};
    pub use device::{DeviceProfile, PadPageMapping};
    pub use error::{EngineError, ConfigError, ActionError, FeedbackError, ProfileError};
    EOF
    ```
  - **Validation**: Public modules compile
    ```bash
    cd midimon-core && cargo check
    ```

- [ ] **3.4 - Remove UI Dependencies from Core** (1 hour)
  - **Action**: Strip colored/chrono display logic from copied files
  - **Files to Modify**:
    - `midimon-core/src/config.rs` - Remove colored output in display impl
    - `midimon-core/src/actions.rs` - Remove colored output in execution logging
    - `midimon-core/src/event_processor.rs` - Remove chrono timestamps from debug output
  - **Pattern**:
    ```rust
    // REMOVE:
    // use colored::Colorize;
    // use chrono::Local;
    // println!("{}", "text".green());
    // let ts = Local::now().format("%H:%M:%S");

    // KEEP:
    // Core logic, pure data structures, error types
    ```
  - **Command to Find Display Code**:
    ```bash
    grep -r "colored\|chrono\|Colorize" midimon-core/src/
    ```
  - **Validation**: No colored/chrono imports remain
    ```bash
    cd midimon-core && cargo check 2>&1 | grep -i "colored\|chrono"
    ```

- [ ] **3.5 - Fix Import Paths** (1 hour)
  - **Action**: Update module references in extracted files
  - **Changes**:
    - Change `crate::config` → `crate::config`
    - Change `crate::actions` → `crate::actions`
    - Change `use super::` references appropriately
  - **Commands**:
    ```bash
    cd midimon-core/src

    # Find all crate:: references
    grep -n "crate::" *.rs | head -20

    # Fix imports (may need manual edits)
    # Ensure all references point to existing modules
    ```
  - **Validation**:
    ```bash
    cd midimon-core && cargo check
    ```
  - **Expected**: All modules found, no import errors

- [ ] **3.6 - Verify Core Library Builds** (30 min)
  - **Action**: Full check and build of core library
  - **Commands**:
    ```bash
    cd midimon-core

    # Check for compilation errors
    cargo check

    # Build debug
    cargo build

    # Build release
    cargo build --release

    # Generate docs
    cargo doc --no-deps --open

    # Run any existing tests
    cargo test --lib
    ```
  - **Expected Output**:
    - No compilation errors
    - All tests pass (if any exist)
    - Documentation generated successfully
  - **If Compilation Fails**:
    - Check error messages carefully
    - Likely issues:
      - Missing module declarations in lib.rs
      - Incorrect import paths
      - Visibility issues (pub vs private)
    - Reference `docs/api-design.md` for module structure

---

## CLI Daemon Creation (Step 3)

**Estimated Duration**: 2-3 hours
**Responsible**: Senior Dev / Architect
**Dependencies**: Core library extraction complete

### CLI Daemon Creation Checklist

- [ ] **4.1 - Create CLI Main Entry Point** (1 hour)
  - **Action**: Create new main.rs that uses midimon-core
  - **File Path**: `midimon-daemon/src/main.rs`
  - **Strategy**:
    1. Extract argument parsing from current main.rs
    2. Extract debug output/formatting logic
    3. Extract main event loop
    4. Adapt to use MidiMonEngine from midimon-core
  - **Basic Template**:
    ```bash
    cat > midimon-daemon/src/main.rs << 'EOF'
    use midimon_core::{Config, MidiMonEngine};
    use colored::Colorize;
    use chrono::Local;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Parse command line arguments
        let (port, led_scheme, profile_path) = parse_cli_args();

        // Load configuration
        let mut config = Config::load("config.toml")?;

        // Update config from CLI arguments
        if let Some(profile) = profile_path {
            config.device.profile_path = Some(profile.into());
        }

        // Create and start engine
        let mut engine = MidiMonEngine::new(config)?;

        // Register mode change callback for colored output
        engine.on_mode_change(|mode| {
            let msg = format!("Mode changed: {}", mode);
            println!("{}", msg.cyan().bold());
        });

        // Register action callback for event logging
        engine.on_action(|event, action| {
            let ts = Local::now().format("%H:%M:%S%.3f").to_string().dimmed();
            println!("{} {:?} → {:?}", ts, event, action);
        });

        // Setup Ctrl+C handler
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("\n{}", "Shutting down...".yellow());
            r.store(false, Ordering::SeqCst);
        })?;

        // Start engine (blocking call)
        engine.start()?;

        // Wait for shutdown signal
        while running.load(Ordering::SeqCst) {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Stop engine
        engine.stop()?;
        println!("{}", "MIDIMon stopped.".green());

        Ok(())
    }

    fn parse_cli_args() -> (Option<usize>, Option<String>, Option<String>) {
        let args: Vec<String> = std::env::args().collect();
        // Parse port, --led, --profile arguments
        // Reference: current src/main.rs lines 1-50
        (None, None, None)
    }
    EOF
    ```
  - **Validation**:
    ```bash
    cd midimon-daemon && cargo check
    ```

- [ ] **4.2 - Move Diagnostic Tools** (30 min)
  - **Action**: Copy existing diagnostic binaries to daemon crate
  - **Commands**:
    ```bash
    # Copy diagnostic tools
    cp src/bin/*.rs midimon-daemon/src/bin/

    # Verify all tools copied
    ls -la midimon-daemon/src/bin/
    ```
  - **Expected Files**:
    - midi_diagnostic.rs
    - led_diagnostic.rs
    - led_tester.rs
    - pad_mapper.rs
    - test_midi.rs
    - midi_simulator.rs
  - **Validation**: All files present in daemon/src/bin/

- [ ] **4.3 - Update Diagnostic Tool Imports** (30 min)
  - **Action**: Update imports in diagnostic tools to use midimon_core
  - **Pattern Changes**:
    ```rust
    // OLD (in src/bin/*)
    // use crate::config::Config;
    // use crate::feedback::FeedbackController;

    // NEW (in midimon-daemon/src/bin/*)
    // use midimon_core::Config;
    // use midimon_core::FeedbackController;
    ```
  - **Commands**:
    ```bash
    # Find files that need updating
    grep -r "use crate::" midimon-daemon/src/bin/

    # Fix imports manually or with sed
    sed -i 's/use crate::/use midimon_core::/g' midimon-daemon/src/bin/*.rs
    ```
  - **Validation**:
    ```bash
    cd midimon-daemon && cargo check
    ```

- [ ] **4.4 - Verify Daemon Builds** (20 min)
  - **Action**: Full build of CLI daemon
  - **Commands**:
    ```bash
    cd midimon-daemon

    # Check compilation
    cargo check

    # Build debug
    cargo build

    # Build release
    cargo build --release

    # List all binaries
    cargo build --release 2>&1 | grep "Finished"
    ```
  - **Expected Output**:
    - All binaries compile successfully
    - Finished dev/release build messages
  - **If Fails**:
    - Check for import errors
    - Verify midimon-core dependency is correct
    - Look for feature flag issues

---

## Module Migration (Step 4)

**Estimated Duration**: 3-4 hours
**Responsible**: Senior Dev
**Dependencies**: Core library and daemon basic structure complete

### Module Migration Checklist

- [ ] **5.1 - Migrate event_processor.rs** (1 hour)
  - **Action**: Extract timing/chord/velocity logic to separate modules
  - **Current State**: All logic in single event_processor.rs
  - **Target State**: Modular with timing.rs, chord.rs, velocity.rs
  - **Steps**:
    1. Create timing.rs with long-press/double-tap detection
    2. Create chord.rs with chord detection logic
    3. Create velocity.rs with velocity range processing
    4. Update event_processor.rs to use these modules
    5. Keep event_processor.rs PRIVATE (not exported in lib.rs)
  - **Commands**:
    ```bash
    # Extract timing detection logic
    cat > midimon-core/src/timing.rs << 'EOF'
    // Long-press and double-tap detection logic
    // Extract from event_processor.rs
    // Keep implementation details private
    EOF

    # Extract chord detection
    cat > midimon-core/src/chord.rs << 'EOF'
    // Chord detection logic
    // Extract from event_processor.rs
    EOF

    # Extract velocity processing
    cat > midimon-core/src/velocity.rs << 'EOF'
    // Velocity range processing
    // Extract from event_processor.rs
    EOF
    ```
  - **Validation**:
    ```bash
    cd midimon-core && cargo check
    ```
  - **Notes**: These remain PRIVATE modules - not exported in lib.rs

- [ ] **5.2 - Migrate config.rs to public API** (45 min)
  - **Action**: Ensure Config types are properly public
  - **Verification**:
    ```bash
    grep -n "^pub struct\|^pub enum" midimon-core/src/config.rs
    ```
  - **Required Exports**:
    - ✅ `pub struct Config`
    - ✅ `pub struct DeviceConfig`
    - ✅ `pub struct Mode`
    - ✅ `pub struct Mapping`
    - ✅ `pub struct AdvancedSettings`
    - ✅ `pub enum Trigger`
    - ✅ `pub enum ActionConfig`
  - **Validation**:
    ```bash
    cd midimon-core && cargo doc --no-deps
    ```

- [ ] **5.3 - Migrate actions.rs to public API** (45 min)
  - **Action**: Ensure Action types are properly public
  - **Verification**:
    ```bash
    grep -n "^pub struct\|^pub enum\|^pub trait" midimon-core/src/actions.rs
    ```
  - **Required Exports**:
    - ✅ `pub enum Action`
    - ✅ `pub enum VolumeAction`
    - ✅ `pub enum Condition`
    - ✅ `pub trait ActionExecutor`
  - **Validation**:
    ```bash
    cd midimon-core && cargo doc --no-deps
    ```

- [ ] **5.4 - Migrate feedback.rs to public API** (45 min)
  - **Action**: Ensure Feedback trait and types are properly public
  - **Verification**:
    ```bash
    grep -n "^pub struct\|^pub enum\|^pub trait" midimon-core/src/feedback.rs
    ```
  - **Required Exports**:
    - ✅ `pub trait FeedbackController`
    - ✅ `pub struct RGB`
    - ✅ `pub enum LightingScheme`
    - ✅ `pub fn create_feedback_device()`
  - **Validation**:
    ```bash
    cd midimon-core && cargo doc --no-deps
    ```

- [ ] **5.5 - Migrate device.rs (device_profile.rs) to public API** (45 min)
  - **Action**: Ensure DeviceProfile types are properly public
  - **Verification**:
    ```bash
    grep -n "^pub struct" midimon-core/src/device.rs
    ```
  - **Required Exports**:
    - ✅ `pub struct DeviceProfile`
    - ✅ `pub struct PadPageMapping`
  - **Implementation Methods**:
    - ✅ `pub fn from_ncmm3()`
    - ✅ `pub fn get_page_by_name()`
    - ✅ `pub fn detect_page_for_note()`
  - **Validation**:
    ```bash
    cd midimon-core && cargo doc --no-deps
    ```

- [ ] **5.6 - Migrate mapping.rs (mappings.rs) to public API** (1 hour)
  - **Action**: Expose necessary mapping engine APIs
  - **Current State**: mappings.rs is internal
  - **Target State**: Public APIs for matching and executing mappings
  - **Minimum Exports**:
    - `pub struct MappingEngine` (or as private detail in engine)
    - Matching logic via engine callbacks
  - **Notes**: Most mapping logic stays private, exposed through MidiMonEngine
  - **Validation**:
    ```bash
    cd midimon-core && cargo check
    ```

---

## API Implementation (Step 5)

**Estimated Duration**: 2-3 hours
**Responsible**: Senior Dev
**Dependencies**: Module migration complete

### API Implementation Checklist

- [ ] **6.1 - Implement MidiMonEngine struct** (1.5 hours)
  - **Action**: Create main public engine entry point
  - **Location**: `midimon-core/src/engine.rs`
  - **Required Methods**:
    ```rust
    pub fn new(config: Config) -> Result<Self, EngineError>;
    pub fn start(&mut self) -> Result<(), EngineError>;
    pub fn stop(&mut self) -> Result<(), EngineError>;
    pub fn reload_config(&mut self, config: Config) -> Result<(), EngineError>;
    pub fn current_mode(&self) -> u8;
    pub fn set_mode(&mut self, mode: u8) -> Result<(), EngineError>;
    pub fn config(&self) -> Config;
    pub fn stats(&self) -> EngineStats;
    pub fn on_mode_change<F>(&mut self, callback: F) where F: Fn(u8) + Send + 'static;
    pub fn on_action<F>(&mut self, callback: F) where F: Fn(&ProcessedEvent, &Action) + Send + 'static;
    ```
  - **Reference**: `/docs/api-design.md` lines 42-113
  - **Validation**:
    ```bash
    cd midimon-core && cargo doc --no-deps | grep "MidiMonEngine"
    ```

- [ ] **6.2 - Implement Config::load and friends** (45 min)
  - **Action**: Ensure Config struct has public API methods
  - **Required Methods**:
    ```rust
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;
    pub fn from_str(toml: &str) -> Result<Self, ConfigError>;
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError>;
    pub fn validate(&self) -> Result<(), ConfigError>;
    ```
  - **Validation**: Methods compile and are documented
    ```bash
    cd midimon-core && cargo doc --no-deps
    ```

- [ ] **6.3 - Implement error type conversions** (30 min)
  - **Action**: Ensure error types properly implement From/Into
  - **Commands**:
    ```bash
    cd midimon-core && cargo check
    ```
  - **Verification**: No type conversion errors in daemon
    ```bash
    cd midimon-daemon && cargo check
    ```

- [ ] **6.4 - Add doc comments to public API** (1 hour)
  - **Action**: Document all public types and functions
  - **Pattern**:
    ```rust
    /// Create a new engine instance with the given configuration
    ///
    /// # Errors
    /// Returns error if config is invalid or device connections fail
    pub fn new(config: Config) -> Result<Self, EngineError> {
        // ...
    }
    ```
  - **Command**:
    ```bash
    # Check for missing doc comments
    cd midimon-core && cargo clippy -- -W missing-docs
    ```
  - **Validation**: All public items documented
    ```bash
    cargo doc --no-deps --open
    ```

---

## Testing (Step 6)

**Estimated Duration**: 3-4 hours
**Responsible**: QA / Test Engineer
**Dependencies**: All modules migrated and API implemented

### Testing Checklist

- [ ] **7.1 - Unit Tests: Core Library** (1 hour)
  - **Action**: Run core library unit tests
  - **Commands**:
    ```bash
    cd midimon-core

    # Run all unit tests
    cargo test --lib

    # Run with output
    cargo test --lib -- --nocapture

    # Test specific module
    cargo test config:: -- --nocapture
    cargo test event_processor:: -- --nocapture
    cargo test mapping:: -- --nocapture
    ```
  - **Expected Output**: All tests pass
    ```
    test result: ok. X passed; 0 failed
    ```
  - **Common Failures**:
    - Module path issues (fix imports)
    - Missing public items (add pub to structs/functions)
    - Type conversion errors (implement From/Into)
  - **If Failures**: Debug and fix before proceeding

- [ ] **7.2 - Unit Tests: Daemon Binary** (30 min)
  - **Action**: Run daemon tests (if any)
  - **Commands**:
    ```bash
    cd midimon-daemon
    cargo test --lib
    ```
  - **Expected Output**: All tests pass (or none exist yet)

- [ ] **7.3 - Integration Tests: Config Loading** (30 min)
  - **Action**: Verify config loading works end-to-end
  - **Test Script**:
    ```bash
    cd midimon-daemon

    # Try loading existing config
    cat > test_config_load.rs << 'EOF'
    #[test]
    fn test_config_loads() {
        let config = midimon_core::Config::load("../../config.toml")
            .expect("Failed to load config.toml");
        assert_eq!(config.device.name, "Mikro");
    }
    EOF

    # Run test
    cargo test test_config_loads
    ```
  - **Expected**: Config loads successfully

- [ ] **7.4 - Integration Tests: Engine Creation** (30 min)
  - **Action**: Verify engine can be created and configured
  - **Test Script**:
    ```bash
    cd midimon-core

    cat > tests/engine_creation.rs << 'EOF'
    #[test]
    fn test_engine_creates() {
        let config = midimon_core::Config::default();
        let engine = midimon_core::MidiMonEngine::new(config);
        assert!(engine.is_ok());
    }
    EOF

    cargo test --test engine_creation
    ```
  - **Expected**: Engine creation succeeds

- [ ] **7.5 - Backward Compatibility Tests** (1 hour)
  - **Action**: Verify existing config still works
  - **Test Scenarios**:
    ```bash
    # Test 1: Load current config.toml
    cargo run -p midimon-daemon -- --help

    # Test 2: List MIDI ports (non-interactive test)
    cargo run -p midimon-daemon --bin test_midi

    # Test 3: LED diagnostic
    cargo run -p midimon-daemon --bin led_diagnostic --release

    # Test 4: MIDI diagnostic (just check it starts)
    timeout 2 cargo run -p midimon-daemon --bin midi_diagnostic -- 2 || true
    ```
  - **Expected Output**:
    - Help text displays
    - MIDI ports listed
    - LED connections tested
    - Diagnostic tools start (timeout OK)

- [ ] **7.6 - Feature Parity Test** (1 hour)
  - **Action**: Verify all v0.1.0 features still work
  - **Features to Test**:
    - [ ] Config loading
    - [ ] Mode switching
    - [ ] Velocity detection
    - [ ] Long press detection
    - [ ] Chord detection
    - [ ] LED feedback
    - [ ] Device profile loading
    - [ ] Action execution
  - **Manual Testing**:
    1. Start daemon: `cargo run -p midimon-daemon --release -- 2`
    2. Press pads and verify:
       - LEDs respond correctly
       - Mode changes work
       - All documented actions trigger
  - **Success Criteria**: All features work identically to v0.1.0

---

## Validation & Verification (Step 7)

**Estimated Duration**: 2-3 hours
**Responsible**: Tech Lead / QA
**Dependencies**: All testing complete

### Validation Checklist

- [ ] **8.1 - Build Verification** (30 min)
  - **Action**: Verify clean builds from scratch
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Clean build
    cargo clean
    cargo build --workspace

    # Release build
    cargo build --workspace --release

    # Check all binaries exist
    ls -lh target/release/midimon*
    ```
  - **Expected Output**:
    - ✅ midimon (main CLI)
    - ✅ midi_diagnostic
    - ✅ led_diagnostic
    - ✅ led_tester
    - ✅ pad_mapper
    - ✅ test_midi
    - ✅ midi_simulator
  - **File Sizes**: Should match v0.1.0 (±5%)

- [ ] **8.2 - Dependency Check** (15 min)
  - **Action**: Verify no circular dependencies or unwanted imports
  - **Commands**:
    ```bash
    # Show dependency tree
    cargo tree --workspace

    # Check for colored/chrono in midimon-core
    cargo tree -p midimon-core | grep -i "colored\|chrono"
    # Should return nothing

    # Check for core dependency in daemon
    cargo tree -p midimon-daemon | grep "midimon-core"
    # Should show dependency
    ```
  - **Validation**:
    - No colored/chrono in midimon-core output
    - midimon-daemon depends on midimon-core

- [ ] **8.3 - Clippy & Formatting** (15 min)
  - **Action**: Ensure code quality standards
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Check formatting
    cargo fmt --all -- --check
    # Fix if needed: cargo fmt --all

    # Run clippy
    cargo clippy --all-targets --all-features -- -D warnings

    # Fix warnings if any
    cargo clippy --fix --allow-dirty --all-targets
    ```
  - **Expected Output**: No warnings or errors

- [ ] **8.4 - Documentation Generation** (15 min)
  - **Action**: Verify all documentation generates correctly
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Generate full docs
    cargo doc --workspace --no-deps

    # Verify public API documented
    cargo doc --workspace --no-deps --open

    # Check for broken links
    cargo doc --workspace --no-deps 2>&1 | grep -i "warning"
    ```
  - **Validation**: Docs generate without warnings

- [ ] **8.5 - Run Full Test Suite** (30 min)
  - **Action**: Execute all tests across workspace
  - **Commands**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Run all tests
    cargo test --workspace

    # Run with output (slower)
    cargo test --workspace -- --nocapture

    # Run doc tests
    cargo test --doc
    ```
  - **Expected Output**: All tests pass
    ```
    test result: ok. N passed; 0 failed
    ```
  - **If Failures**: Fix and re-run

- [ ] **8.6 - Config Compatibility Test** (30 min)
  - **Action**: Verify backward compatibility with v0.1.0 config
  - **Test Cases**:
    ```bash
    # Test 1: Load current config
    cargo run -p midimon-daemon -- --help

    # Test 2: Verify config.toml loads
    cat > /tmp/test_config.rs << 'EOF'
    #[cfg(test)]
    mod tests {
        use midimon_core::Config;

        #[test]
        fn test_load_current_config() {
            let config = Config::load("config.toml");
            assert!(config.is_ok(), "Failed to load config.toml: {:?}", config.err());

            let cfg = config.unwrap();
            assert!(!cfg.device.name.is_empty());
            assert!(!cfg.modes.is_empty());
        }
    }
    EOF

    cargo test
    ```
  - **Validation**:
    - ✅ config.toml loads without errors
    - ✅ All sections parse correctly
    - ✅ No warnings about deprecated fields

---

## Documentation Updates (Step 8)

**Estimated Duration**: 2-3 hours
**Responsible**: Tech Writer / Dev
**Dependencies**: Code migration complete

### Documentation Checklist

- [ ] **9.1 - Update README.md** (45 min)
  - **Action**: Document new workspace structure
  - **Sections to Update**:
    - [ ] Building section (add workspace commands)
    - [ ] Running section (explain midimon-core vs midimon-daemon)
    - [ ] Project Structure section (show new layout)
    - [ ] Architecture section (link to api-design.md)
  - **File**: `/Users/christopherjoseph/projects/amiable/midimon/README.md`
  - **Example Additions**:
    ```markdown
    ## Architecture

    MIDIMon v0.2.0+ is organized as a Cargo workspace with two main crates:

    - **midimon-core**: Pure Rust MIDI engine library (zero UI dependencies)
    - **midimon-daemon**: CLI application and diagnostic tools using midimon-core

    See [Architecture Documentation](docs-site/src/development/architecture.md) for details.
    ```

- [ ] **9.2 - Update CLAUDE.md** (1 hour)
  - **Action**: Update development guide with new structure
  - **Sections to Update**:
    - [ ] Build commands (add workspace examples)
    - [ ] Running commands (explain crate selection)
    - [ ] Architecture section (reference new structure)
    - [ ] Project Status section (update Phase 2 as complete)
  - **File**: `/Users/christopherjoseph/projects/amiable/midimon/CLAUDE.md`
  - **Example Additions**:
    ```markdown
    ## Workspace Structure (Phase 2)

    The project is now organized as a Cargo workspace:

    ```bash
    # Build specific crate
    cargo build -p midimon-core
    cargo build -p midimon-daemon --release

    # Run CLI daemon
    cargo run -p midimon-daemon --release -- 2 --led reactive

    # Run diagnostic tools
    cargo run -p midimon-daemon --bin midi_diagnostic -- 2
    ```
    ```

- [ ] **9.3 - Update Architecture Documentation** (45 min)
  - **Action**: Update public-facing architecture docs
  - **File**: `/docs-site/src/development/architecture.md`
  - **Sections**:
    - [ ] Add Phase 2 completion status
    - [ ] Add workspace structure diagram
    - [ ] Add midimon-core public API reference
    - [ ] Update crate dependency graph
  - **Links to Add**:
    - `/docs/api-design.md` (AMI-123)
    - `/docs/workspace-structure.md` (AMI-124)

- [ ] **9.4 - Create Migration Guide** (30 min)
  - **Action**: Create guide for users and developers
  - **File**: `/docs/phase2-migration-complete.md`
  - **Content**:
    - What changed
    - Why it matters
    - How to upgrade (it's automatic - backward compatible)
    - New capabilities (future hot-reload, daemon, GUI)
    - Troubleshooting

- [ ] **9.5 - Update API Documentation** (15 min)
  - **Action**: Ensure all generated docs are referenced
  - **Commands**:
    ```bash
    # Generate module-level docs
    cargo doc -p midimon-core --no-deps --open
    ```
  - **Verification**: All public modules documented

---

## Git & Completion (Step 9)

**Estimated Duration**: 1-2 hours
**Responsible**: Tech Lead
**Dependencies**: All validation complete

### Git & Completion Checklist

- [ ] **10.1 - Commit All Changes** (30 min)
  - **Action**: Create comprehensive commit with all changes
  - **Command**:
    ```bash
    cd /Users/christopherjoseph/projects/amiable/midimon

    # Stage all changes
    git add -A

    # Create detailed commit
    git commit -m "Phase 2: Extract midimon-core workspace structure (AMI-126)

    Major Changes:
    - Create Cargo workspace with midimon-core and midimon-daemon crates
    - Extract pure Rust engine library (midimon-core) with zero UI dependencies
    - Refactor CLI daemon to use core library (midimon-daemon)
    - Create clean public API surface for library consumers
    - Remove colored/chrono display logic from core
    - Preserve all v0.1.0 features and backward compatibility

    Workspace Structure:
    - midimon-core/: Pure engine library (public API in lib.rs)
    - midimon-daemon/: CLI binary with colored output and diagnostics
    - config/: Configuration templates (unchanged format)

    Public API Exports:
    - Config, Mode, Mapping, Trigger, ActionConfig
    - MidiMonEngine with new, start, stop, reload_config methods
    - MidiEvent, ProcessedEvent event types
    - Action, VolumeAction execution types
    - FeedbackController trait for LED implementations
    - DeviceProfile for NI Controller Editor support
    - Error types: EngineError, ConfigError, ActionError, FeedbackError

    Testing:
    - All unit tests pass
    - All integration tests pass
    - Backward compatibility verified with v0.1.0 config
    - All diagnostic tools operational
    - Feature parity with v0.1.0 confirmed

    Documentation:
    - Updated README.md with workspace structure
    - Updated CLAUDE.md with build/run commands
    - Updated api-design.md (AMI-123)
    - Updated workspace-structure.md (AMI-124)
    - Generated complete rustdoc for public API

    Related Issues:
    - AMI-123: API Design ✅
    - AMI-124: Workspace Structure ✅
    - AMI-125: Backward Compatibility ✅
    - AMI-126: Phase 2 Migration Checklist (this) ✅"
    ```
  - **Validation**:
    ```bash
    git log -1 --oneline
    ```

- [ ] **10.2 - Create Phase 2 Completion Tag** (15 min)
  - **Action**: Tag workspace migration completion
  - **Command**:
    ```bash
    git tag -a v0.2.0-workspace \
      -m "MIDIMon Phase 2: Workspace Structure Migration

    Workspace extraction complete with:
    - midimon-core: Pure MIDI engine library
    - midimon-daemon: CLI using midimon-core

    100% backward compatible with v0.1.0 configs and features.
    Ready for Phase 3 (hot-reload and daemon enhancements)."

    git push origin v0.2.0-workspace
    ```
  - **Validation**:
    ```bash
    git tag -l | grep v0.2.0
    git ls-remote origin v0.2.0-workspace
    ```

- [ ] **10.3 - Create Phase 2 Release Notes** (30 min)
  - **Action**: Document changes for release
  - **File**: `/docs/phase2-release-notes.md`
  - **Content Structure**:
    ```markdown
    # MIDIMon v0.2.0 Release Notes

    ## Overview
    Phase 2 refactors MIDIMon into a modular workspace structure while maintaining
    100% backward compatibility with v0.1.0.

    ## What's New

    ### Architecture Improvements
    - Extracted `midimon-core` library for programmatic use
    - Separated concerns: pure engine vs UI/CLI
    - Enables future GUI applications (Phase 3)
    - Foundation for hot-reload support (Phase 3)

    ### New Capabilities
    - Engine can be embedded in other applications
    - Callback-based event system for custom integrations
    - Clean public API following Rust best practices

    ### Backward Compatibility
    - ✅ All config.toml files from v0.1.0 work unchanged
    - ✅ CLI arguments identical
    - ✅ All 26 features preserved
    - ✅ Performance unchanged

    ## Migration Guide
    No migration needed! Just upgrade and continue using as before.

    ## Under the Hood
    - Workspace resolver 2 for optimal builds
    - LTO and codegen optimizations in release builds
    - Zero runtime overhead from modularization
    ```

- [ ] **10.4 - Push to Remote** (10 min)
  - **Action**: Push branch and tags to GitHub
  - **Commands**:
    ```bash
    # Push branch
    git push origin phase-2/workspace-migration

    # Push tags
    git push origin v0.2.0-workspace

    # Verify
    git ls-remote origin | grep "phase-2\|v0.2.0"
    ```
  - **Validation**: Branch and tags appear on GitHub

- [ ] **10.5 - Create Pull Request** (15 min)
  - **Action**: Create PR for Phase 2 completion
  - **Details**:
    - **Title**: "Phase 2: Extract midimon-core workspace structure (AMI-126)"
    - **Base Branch**: main
    - **Head Branch**: phase-2/workspace-migration
    - **Description**: Use commit message from step 10.1
    - **Labels**: phase-2, architecture, workspace
    - **Reviewers**: Tech lead, team leads
  - **PR Checklist**:
    - [ ] All checks pass (CI/CD)
    - [ ] All tests pass
    - [ ] Documentation updated
    - [ ] Backward compatibility verified
  - **Merge Strategy**: Squash or rebase to main

- [ ] **10.6 - Merge to Main** (5 min)
  - **Action**: Merge Phase 2 changes to main branch
  - **Command**:
    ```bash
    # After PR approval
    git checkout main
    git pull origin main
    git merge phase-2/workspace-migration
    git push origin main
    ```
  - **Verification**: Phase 2 commit appears on main

---

## Rollback Procedures

### If Compilation Fails

**Timing**: During Workspace Creation or Module Migration

**Recovery**:
```bash
# Revert uncommitted changes
git checkout .

# Or: Revert to v0.1.0-monolithic tag
git reset --hard v0.1.0-monolithic
git clean -fd

# Identify and fix issue
# - Check Cargo.toml syntax (TOML parser)
# - Verify file paths (case sensitivity)
# - Check module declarations in lib.rs

# Retry after fixes
cargo check
```

### If Tests Fail

**Timing**: During Testing phase

**Recovery**:
```bash
# Run tests with verbose output
cargo test -- --nocapture 2>&1 | head -50

# Common issues:
# 1. Import errors: Check module paths
#    - Fix: Update imports to reference correct modules
# 2. Missing items: Check pub visibility
#    - Fix: Add 'pub' to exported structs/functions
# 3. Type errors: Check API compatibility
#    - Fix: Reference api-design.md for correct types

# Retry tests
cargo test --workspace
```

### If Backward Compatibility Breaks

**Timing**: During Validation phase

**Recovery**:
```bash
# Load v0.1.0 config to diagnose
cargo run -p midimon-daemon -- 2

# If config.toml doesn't load:
# 1. Check config parsing in midimon-core/src/config.rs
# 2. Verify all Trigger and Action types still parsed
# 3. Check AdvancedSettings optional defaults

# Ensure fields are optional with #[serde(default)]
#[derive(Deserialize)]
pub struct AdvancedSettings {
    #[serde(default = "default_chord_timeout")]
    pub chord_timeout_ms: u64,
}

# Retry config loading
cargo test config::
```

### If Binary Size Increases Significantly

**Timing**: During Build Verification

**Investigation**:
```bash
# Compare binary sizes
ls -lh target/release/midimon*
# Should be ±5% of v0.1.0 size

# If larger:
cargo build --release
strip -x target/release/midimon
ls -lh target/release/midimon

# Check for duplicate dependencies
cargo tree --duplicates
```

### Full Rollback (Extreme Cases)

**Only use if Phase 2 is fundamentally broken after merging:**

```bash
# 1. Create safe point
git tag v0.2.0-rollback-attempt

# 2. Revert to known-good state
git reset --hard v0.1.0-monolithic
git push --force origin main  # DANGEROUS - only if absolutely necessary

# 3. Create incident report
cat > /tmp/phase2-incident.md << 'EOF'
# Phase 2 Rollback Incident

## What Failed
[Describe critical issue]

## When
[Date/time of discovery]

## Impact
[Who was affected]

## Lessons Learned
[What to do differently]

## Next Steps
[Plan for retry]
EOF

# 4. Schedule retrospective
# Contact team for post-mortem meeting
```

---

## Success Criteria & Sign-Off

### Phase 2 Completion Criteria

All of the following must be true:

- [ ] **Workspace Structure**
  - [ ] Root Cargo.toml defines workspace with 2 members
  - [ ] midimon-core compiles independently
  - [ ] midimon-daemon compiles independently
  - [ ] `cargo build --workspace` succeeds

- [ ] **API Surface**
  - [ ] All public API documented in lib.rs
  - [ ] All types from api-design.md exported
  - [ ] No colored/chrono in midimon-core
  - [ ] Error types properly defined with thiserror

- [ ] **Testing**
  - [ ] All unit tests pass (`cargo test --workspace`)
  - [ ] All integration tests pass
  - [ ] Config backward compatibility verified
  - [ ] All diagnostic tools compile and run

- [ ] **Documentation**
  - [ ] README.md updated with new structure
  - [ ] CLAUDE.md updated with workspace commands
  - [ ] api-design.md reflects implementation
  - [ ] workspace-structure.md reflects implementation
  - [ ] Rustdoc generated without warnings

- [ ] **Quality**
  - [ ] `cargo fmt --all` passes
  - [ ] `cargo clippy --all-targets` passes (no warnings)
  - [ ] No circular dependencies
  - [ ] Binary size within 5% of v0.1.0

- [ ] **Backward Compatibility**
  - [ ] v0.1.0 config.toml loads without errors
  - [ ] All 26 features work identically
  - [ ] CLI arguments unchanged
  - [ ] No deprecation warnings

- [ ] **Git**
  - [ ] v0.1.0-monolithic tag exists (preservation)
  - [ ] v0.2.0-workspace tag created (completion)
  - [ ] All changes committed to main
  - [ ] PR merged with proper documentation

### Sign-Off Template

```markdown
## Phase 2 Completion Sign-Off

**Date**: [YYYY-MM-DD]
**Duration**: [X hours / X days]
**Completed By**: [Name]
**Reviewed By**: [Name]

### Verification
- [ ] Checklist items: 100 complete
- [ ] Tests: All passing
- [ ] Backward compatibility: Verified
- [ ] Documentation: Complete
- [ ] Git: Main branch updated

### Notes
[Any issues encountered and resolved]

### Sign-Off
- **Tech Lead**: _________________ Date: _______
- **QA Lead**: _________________ Date: _______
- **Product Owner**: __________ Date: _______

### Phase 3 Readiness
Phase 3 (Hot-Reload & Daemon) can proceed immediately.
- [ ] API stable and documented
- [ ] Core engine ready for GUI integration
- [ ] Test infrastructure in place
```

---

## Appendix: Quick Reference Commands

### Build Commands
```bash
# Build all crates
cargo build --workspace

# Build release
cargo build --workspace --release

# Build specific crate
cargo build -p midimon-core
cargo build -p midimon-daemon --release
```

### Test Commands
```bash
# Test all
cargo test --workspace

# Test specific crate
cargo test -p midimon-core
cargo test -p midimon-daemon

# Test with output
cargo test --workspace -- --nocapture
```

### Diagnostic Commands
```bash
# Run main CLI
cargo run -p midimon-daemon --release -- 2

# Run diagnostic tools
cargo run -p midimon-daemon --bin midi_diagnostic -- 2
cargo run -p midimon-daemon --bin led_diagnostic
cargo run -p midimon-daemon --bin test_midi
```

### Quality Assurance
```bash
# Format check
cargo fmt --all -- --check

# Lint check
cargo clippy --all-targets -- -D warnings

# Documentation
cargo doc --workspace --no-deps --open

# Dependency tree
cargo tree --workspace
```

### Git Commands
```bash
# Create tag
git tag -a v0.2.0-workspace -m "Phase 2 complete"

# Push tag
git push origin v0.2.0-workspace

# View tags
git tag -l

# Create PR
gh pr create --title "Phase 2: ..." --body "..."
```

---

## Timeline Estimate

| Phase | Duration | Cumulative | Status |
|-------|----------|-----------|--------|
| Preparation | 4-6 hours | 4-6h | Ready |
| Workspace Creation | 1-2 hours | 5-8h | Ready |
| Core Extraction | 4-6 hours | 9-14h | Ready |
| Daemon Creation | 2-3 hours | 11-17h | Ready |
| Module Migration | 3-4 hours | 14-21h | Ready |
| API Implementation | 2-3 hours | 16-24h | Ready |
| Testing | 3-4 hours | 19-28h | Ready |
| Validation | 2-3 hours | 21-31h | Ready |
| Documentation | 2-3 hours | 23-34h | Ready |
| Git & Completion | 1-2 hours | 24-36h | Ready |
| **Total** | | **24-36 hours** | **Ready** |

**Recommended Execution**: 3-4 weeks part-time, or 1 intensive week full-time

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-11 | Initial comprehensive Phase 2 checklist |

---

## References

- **AMI-123**: [API Design](api-design.md)
- **AMI-124**: [Workspace Structure](workspace-structure.md)
- **AMI-125**: [Config Compatibility](config-compatibility.md)
- **AMI-126**: [Phase 2 Migration Checklist](phase2-migration-checklist.md) (this document)
- **Phase 2 Migration Guide**: [phase2-migration-guide.md](phase2-migration-guide.md)
- **CLAUDE.md**: Project development guide
- **GitHub**: https://github.com/amiable-dev/midimon

