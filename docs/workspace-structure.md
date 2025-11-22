# Workspace Structure Design

**Version**: 0.1.0-alpha
**Status**: Phase 2 - Workspace Design
**Last Updated**: 2025-11-11
**Dependencies**: [AMI-123 API Design](api-design.md)

## Overview

This document defines the Cargo workspace structure for the Phase 2 refactoring of MIDIMon from a monolithic binary to a multi-crate architecture. The design extracts a pure Rust engine library (`midimon-core`) that can be used by multiple frontends (CLI, daemon, GUI).

## Design Goals

1. **Clean Separation**: Engine logic completely independent of UI/display concerns
2. **Reusability**: Core library usable by CLI, daemon, and future GUI applications
3. **Backward Compatibility**: Existing config.toml format and CLI arguments preserved
4. **Minimal Disruption**: Current functionality remains intact during migration
5. **Future-Ready**: Structure supports Phase 3 (hot-reload) and Phase 4 (Tauri GUI)

## Workspace Layout

```
midimon/
├── Cargo.toml                      # Workspace root manifest
├── midimon-core/                   # Pure Rust engine library (Phase 2)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                  # Public API (see AMI-123)
│   │   ├── config.rs               # Config types and loading
│   │   ├── engine.rs               # MidiMonEngine main entry point
│   │   ├── events.rs               # MidiEvent, ProcessedEvent types
│   │   ├── event_processor.rs     # State machine (private)
│   │   ├── mappings.rs             # Mapping engine
│   │   ├── actions.rs              # Action types and execution
│   │   ├── feedback.rs             # FeedbackController trait
│   │   ├── mikro_leds.rs          # HID LED implementation
│   │   ├── midi_feedback.rs       # MIDI LED fallback
│   │   ├── device_profile.rs      # NI profile parser
│   │   ├── timing.rs               # Long-press/double-tap (private)
│   │   ├── chord.rs                # Chord detection (private)
│   │   ├── velocity.rs             # Velocity processing (private)
│   │   └── error.rs                # Error types
│   └── tests/
│       ├── integration_tests.rs
│       └── fixtures/
├── midimon-daemon/                 # CLI binary (current main.rs)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs                 # CLI entry point
│   │   ├── cli.rs                  # Argument parsing
│   │   ├── debug.rs                # Debug output formatting
│   │   └── bin/                    # Diagnostic tools
│   │       ├── midi_diagnostic.rs
│   │       ├── led_diagnostic.rs
│   │       ├── led_tester.rs
│   │       ├── pad_mapper.rs
│   │       ├── test_midi.rs
│   │       └── midi_simulator.rs
│   └── tests/
│       └── cli_tests.rs
├── midimon-gui/                    # Tauri UI (Phase 4 placeholder)
│   ├── Cargo.toml
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── ui/                         # Web UI (Svelte/React)
│       ├── package.json
│       └── src/
├── config/                         # Configuration templates
│   ├── default.toml                # Default config
│   ├── examples/
│   │   ├── maschine-mikro-mk3.toml
│   │   ├── velocity-layers.toml
│   │   └── chord-detection.toml
│   └── device_templates/
│       ├── maschine_mikro_mk3.toml
│       ├── launchpad_mini.toml
│       └── korg_nanokontrol.toml
├── docs/                           # Documentation
│   ├── api-design.md               # AMI-123 API design
│   ├── workspace-structure.md      # This document (AMI-124)
│   └── migration-guide.md          # Phase 2 migration steps
├── .research/                      # Implementation proposals (reference only)
│   ├── implementation-viewpoint-1.md
│   └── implementation-viewpoint-2.md
└── README.md                       # User-facing documentation
```

## Crate Dependency Graph

```
┌─────────────────┐
│  midimon-gui    │  (Phase 4)
│  (Tauri UI)     │
└────────┬────────┘
         │
         │ depends on
         ▼
┌─────────────────┐
│ midimon-daemon  │  (Phase 2)
│ (CLI binary)    │
└────────┬────────┘
         │
         │ depends on
         ▼
┌─────────────────┐
│  midimon-core   │  (Phase 2)
│  (Engine lib)   │
└─────────────────┘
```

**Dependency Rules**:
- `midimon-core`: NO dependencies on CLI or GUI crates, NO UI dependencies (colored, chrono for display)
- `midimon-daemon`: Depends on `midimon-core` only
- `midimon-gui`: Depends on `midimon-core` only (not on daemon)

## Workspace Root Manifest

**File**: `Cargo.toml`

```toml
[workspace]
members = [
    "midimon-core",
    "midimon-daemon",
    "midimon-gui",
]
resolver = "2"

[workspace.package]
version = "0.2.0"
edition = "2021"
authors = ["MIDIMon Contributors"]
license = "MIT"
repository = "https://github.com/your-org/midimon"
homepage = "https://midimon.dev"

[workspace.dependencies]
# Core dependencies (shared across crates)
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
midir = "0.10"
enigo = "0.6"
hidapi = { version = "2.6", features = ["macos-shared-device"] }
quick-xml = { version = "0.37", features = ["serialize"] }
crossbeam-channel = "0.5"
thiserror = "1.0"

# CLI/display dependencies (NOT in core)
colored = "3.0"
chrono = "0.4"
ctrlc = "3.4"

# Testing dependencies
proptest = "1.5"
rstest = "0.23"
```

## midimon-core (Library Crate)

**Purpose**: Pure Rust engine library with zero UI dependencies.

### Cargo.toml

**File**: `midimon-core/Cargo.toml`

```toml
[package]
name = "midimon-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

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

# NO colored, chrono, or ctrlc in core

[dev-dependencies]
proptest.workspace = true
rstest.workspace = true

[features]
default = []
# Future feature flags
test-mocks = []
virtual-midi = []
```

### Public API (from AMI-123)

**Exported modules** (public):
- `pub mod config` - Config, Mode, Mapping, Trigger, Action types
- `pub mod engine` - MidiMonEngine
- `pub mod events` - MidiEvent, ProcessedEvent, VelocityLevel, EncoderDirection
- `pub mod actions` - Action, VolumeAction, Condition
- `pub mod feedback` - FeedbackController trait, RGB, LightingScheme
- `pub mod device` - DeviceProfile, PadPageMapping
- `pub mod error` - EngineError, ConfigError, ActionError, FeedbackError, ProfileError

**Private modules** (implementation details):
- `mod event_processor` - EventProcessor state machine
- `mod timing` - Long-press, double-tap detection
- `mod chord` - Chord detection logic
- `mod velocity` - Velocity range processing

### Module Migration Map

| Current File | Target Location | Visibility | Notes |
|-------------|----------------|-----------|-------|
| `src/config.rs` | `midimon-core/src/config.rs` | Public | Remove colored/chrono display logic |
| `src/event_processor.rs` | `midimon-core/src/event_processor.rs` | Private | Keep as-is |
| `src/mappings.rs` | `midimon-core/src/mappings.rs` | Public | Expose MappingEngine API |
| `src/actions.rs` | `midimon-core/src/actions.rs` | Public | Remove colored output |
| `src/feedback.rs` | `midimon-core/src/feedback.rs` | Public | Keep trait-based design |
| `src/mikro_leds.rs` | `midimon-core/src/mikro_leds.rs` | Private | Implements FeedbackController |
| `src/midi_feedback.rs` | `midimon-core/src/midi_feedback.rs` | Private | Fallback implementation |
| `src/device_profile.rs` | `midimon-core/src/device_profile.rs` | Public | Keep XML parsing |
| NEW: `src/timing.rs` | `midimon-core/src/timing.rs` | Private | Extract from event_processor |
| NEW: `src/chord.rs` | `midimon-core/src/chord.rs` | Private | Extract from event_processor |
| NEW: `src/velocity.rs` | `midimon-core/src/velocity.rs` | Private | Extract from event_processor |
| NEW: `src/engine.rs` | `midimon-core/src/engine.rs` | Public | Main entry point (from main.rs) |
| NEW: `src/error.rs` | `midimon-core/src/error.rs` | Public | Unified error types |

## midimon-daemon (Binary Crate)

**Purpose**: CLI application using midimon-core. Includes debug output, colored terminal, and diagnostic tools.

### Cargo.toml

**File**: `midimon-daemon/Cargo.toml`

```toml
[package]
name = "midimon-daemon"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

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

# CLI dependencies (NOT in core)
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

### Module Structure

| File | Purpose | Notes |
|------|---------|-------|
| `src/main.rs` | CLI entry point | Uses MidiMonEngine from core |
| `src/cli.rs` | Argument parsing | Port, LED scheme, profile args |
| `src/debug.rs` | Debug output | Colored terminal, event logging |
| `src/bin/*.rs` | Diagnostic tools | MIDI/LED diagnostics, testers |

### Migration Map

| Current File | Target Location | Notes |
|-------------|----------------|-------|
| `src/main.rs` (L1-50) | `midimon-daemon/src/cli.rs` | Argument parsing |
| `src/main.rs` (L51-100) | `midimon-daemon/src/debug.rs` | Debug output formatting |
| `src/main.rs` (L101-end) | `midimon-daemon/src/main.rs` | Main loop using MidiMonEngine |
| `src/bin/*.rs` | `midimon-daemon/src/bin/*.rs` | Move diagnostic tools as-is |

## midimon-gui (Future Tauri UI)

**Purpose**: Tauri-based menu bar application for Phase 4. Placeholder for now.

### Cargo.toml

**File**: `midimon-gui/Cargo.toml`

```toml
[package]
name = "midimon-gui"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
midimon-core = { path = "../midimon-core" }
tauri = { version = "2.0", features = ["macos-private-api"] }
tauri-plugin-autostart = "2.0"
tauri-plugin-shell = "2.0"
serde.workspace = true

[build-dependencies]
tauri-build = { version = "2.0" }
```

**Phase 4 features** (deferred):
- Menu bar tray icon
- Quick actions (Pause, Reload, Open Config)
- Visual config editor
- MIDI Learn mode
- Device profile management
- Hot-reload support

## Configuration Management

**Location**: `config/` directory at workspace root

### Directory Structure

```
config/
├── default.toml                    # Default config (copied to user dir)
├── examples/                       # Example configs for documentation
│   ├── maschine-mikro-mk3.toml    # Full Maschine Mikro MK3 setup
│   ├── velocity-layers.toml       # Velocity-sensitive mappings
│   ├── chord-detection.toml       # Chord-based triggers
│   └── multi-mode.toml            # Multiple mode configurations
└── device_templates/               # Device-specific templates
    ├── maschine_mikro_mk3.toml    # NI Maschine Mikro MK3
    ├── launchpad_mini.toml        # Novation Launchpad Mini
    └── korg_nanokontrol.toml      # Korg nanoKONTROL2
```

### Config Loading Strategy

1. **User config**: `~/Library/Application Support/MIDIMon/config.toml` (macOS)
2. **Fallback**: `config/default.toml` from repository
3. **Profile path**: Absolute or relative to config directory

**Backward compatibility**: Existing `config.toml` format unchanged.

## Build and Test Commands

### Workspace-Level Commands

```bash
# Build all crates
cargo build

# Build release (all crates)
cargo build --release

# Test all crates
cargo test

# Test specific crate
cargo test -p midimon-core
cargo test -p midimon-daemon

# Run clippy on all crates
cargo clippy --all-targets --all-features

# Check formatting
cargo fmt --all -- --check

# Generate documentation
cargo doc --workspace --no-deps --open
```

### Crate-Specific Commands

```bash
# Build just the core library
cargo build -p midimon-core

# Build just the CLI daemon
cargo build -p midimon-daemon --release

# Run the main CLI binary
cargo run -p midimon-daemon --release -- 2 --led reactive

# Run diagnostic tools
cargo run -p midimon-daemon --bin midi_diagnostic -- 2
cargo run -p midimon-daemon --bin led_diagnostic

# Test core library with verbose output
cargo test -p midimon-core -- --nocapture

# Run integration tests
cargo test -p midimon-daemon --test cli_tests
```

### Development Workflow

```bash
# Watch for changes and rebuild
cargo watch -x "build -p midimon-core"

# Run tests on save
cargo watch -x "test -p midimon-core"

# Continuous integration commands
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Documentation Generation

### Doc Structure

```bash
# Generate docs for all public APIs
cargo doc --workspace --no-deps

# Generate docs with private items (for development)
cargo doc --workspace --no-deps --document-private-items

# Open docs in browser
cargo doc --workspace --no-deps --open
```

### Documentation Targets

- **midimon-core**: Full public API documentation with examples
- **midimon-daemon**: CLI usage, diagnostic tools
- **midimon-gui**: (Phase 4) UI usage, keyboard shortcuts

### Doc Comments Strategy

**Public API** (midimon-core):
- Every public type, trait, function has doc comments
- Examples for main entry points (MidiMonEngine, Config)
- Safety notes for unsafe code (if any)
- Error documentation for all Result types

**Internal modules** (private):
- Implementation notes
- Invariants and assumptions
- Performance considerations

**CLI binary** (midimon-daemon):
- Usage examples in main.rs
- Diagnostic tool documentation in each bin/*.rs

## Migration Steps (Phase 2 Implementation)

### Step 1: Create Workspace Structure

```bash
# Create workspace directories
mkdir -p midimon-core/src
mkdir -p midimon-daemon/src/bin
mkdir -p midimon-gui/src-tauri/src
mkdir -p config/{examples,device_templates}

# Create workspace root Cargo.toml
# (as defined above)

# Create crate-level Cargo.toml files
# (as defined above)
```

### Step 2: Extract Core Library

1. **Copy modules to midimon-core/src/**:
   - config.rs (remove colored output)
   - event_processor.rs (keep as-is)
   - mappings.rs (expose public API)
   - actions.rs (remove colored output)
   - feedback.rs (keep trait-based design)
   - mikro_leds.rs (keep HID logic)
   - midi_feedback.rs (keep MIDI fallback)
   - device_profile.rs (keep XML parsing)

2. **Create new modules in midimon-core/src/**:
   - engine.rs (extract from main.rs)
   - error.rs (unified error types with thiserror)
   - timing.rs (extract timing logic from event_processor)
   - chord.rs (extract chord logic from event_processor)
   - velocity.rs (extract velocity logic from event_processor)

3. **Create lib.rs** with public API (see AMI-123)

4. **Update imports**: Change all `crate::` imports to module paths

5. **Remove UI dependencies**: Strip colored, chrono display logic

### Step 3: Create CLI Daemon

1. **Split main.rs**:
   - Argument parsing → `cli.rs`
   - Debug output → `debug.rs`
   - Main loop → `main.rs` (using MidiMonEngine)

2. **Move diagnostic tools**:
   - Copy `src/bin/*.rs` → `midimon-daemon/src/bin/*.rs`

3. **Update imports**: Use `midimon_core::{...}`

4. **Add colored output**: Restore colored/chrono in daemon only

### Step 4: Verify Build

```bash
# Build core library
cargo build -p midimon-core

# Run core tests
cargo test -p midimon-core

# Build CLI daemon
cargo build -p midimon-daemon

# Run integration tests
cargo test -p midimon-daemon

# Verify diagnostic tools work
cargo run -p midimon-daemon --bin test_midi
```

### Step 5: Update Documentation

1. Update README.md with new workspace structure
2. Update CLAUDE.md with build commands
3. Create migration-guide.md for contributors
4. Generate and review API documentation

### Step 6: Create Git Tag

```bash
# Tag the workspace migration
git add -A
git commit -m "Phase 2: Create workspace structure (AMI-124)

- Extract midimon-core library with public API
- Create midimon-daemon CLI binary
- Add midimon-gui placeholder for Phase 4
- Preserve backward compatibility with config.toml
- Maintain all existing features and diagnostics"

git tag -a v0.2.0-workspace -m "Phase 2: Workspace structure complete"
```

## Testing Strategy

### Core Library Tests

**Location**: `midimon-core/tests/`

- **Unit tests**: Embedded in each module (`mod tests`)
- **Integration tests**: `tests/integration_tests.rs`
- **Property tests**: Use proptest for event processing
- **Fixtures**: `tests/fixtures/` for test configs, profiles

```bash
# Run core library tests
cargo test -p midimon-core

# Run with proptest
cargo test -p midimon-core --features test-mocks
```

### CLI Daemon Tests

**Location**: `midimon-daemon/tests/`

- **CLI argument parsing**: Test all flag combinations
- **Diagnostic tools**: End-to-end tests for each bin
- **Integration**: Test engine integration with CLI

```bash
# Run daemon tests
cargo test -p midimon-daemon

# Test specific binary
cargo test -p midimon-daemon --bin midi_diagnostic
```

### Continuous Integration

**GitHub Actions** (future):
```yaml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features
      - run: cargo test --workspace
      - run: cargo build --workspace --release
```

## Performance Considerations

### Build Performance

- **Workspace resolver 2**: Enables faster dependency resolution
- **Codegen units**: Set to 1 for release (LTO optimization)
- **Incremental compilation**: Enabled by default for dev builds

### Runtime Performance

- **No overhead**: Core library has same performance as monolith
- **Zero-copy**: Event pipeline uses references, not clones
- **Lock-free**: crossbeam-channel for event passing

### Binary Size

- **Core library**: 1-2MB (compiled)
- **CLI daemon**: 3-5MB (with all diagnostics)
- **GUI (future)**: 10-15MB (with Tauri runtime)

## Troubleshooting

### Common Issues

**Issue**: `cargo build` fails with "cannot find crate"
- **Solution**: Ensure workspace members are correct in root Cargo.toml

**Issue**: Import errors after migration
- **Solution**: Change `crate::module` to `midimon_core::module`

**Issue**: Colored output not working in core
- **Solution**: Colored/chrono belong in daemon, not core

**Issue**: Tests fail with "file not found"
- **Solution**: Update test fixture paths to new structure

### Debug Commands

```bash
# Show workspace tree
cargo tree

# Check dependencies
cargo tree -p midimon-core

# Verify no cycles
cargo tree --workspace --no-dedupe

# Check for unused dependencies
cargo +nightly udeps --workspace
```

## Future Extensions (Phase 3+)

### Phase 3: Hot-Reload and Daemon

- Add `notify` crate for file watching
- Implement `MidiMonEngine::reload_config()`
- Add menu bar integration (Tauri or cacao)
- LaunchAgent for macOS startup

### Phase 4: GUI Application

- Build Tauri-based config editor
- Implement MIDI Learn mode
- Visual device mapping
- Profile management UI
- Live event console

### Phase 5: Advanced Features

- Virtual MIDI output
- Plugin system for custom actions
- Cloud profile sync
- Multi-device orchestration

## References

- [AMI-123: API Design](api-design.md) - Core library public API
- [AMI-122: Phase 2 Plan](../PHASE_EXECUTION_README.md) - Migration plan
- [Implementation Viewpoint 1](.research/implementation-viewpoint-1.md) - Monorepo proposal
- [Implementation Viewpoint 2](.research/implementation-viewpoint-2.md) - Alternative approach

---

**Next Steps**: Implement Phase 2 migration following the steps in "Migration Steps" section above.
