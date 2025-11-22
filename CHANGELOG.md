# Changelog

All notable changes to Conductor will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Windows and Linux platform support for app detection
- Advanced trigger types (Chord, Sequence)
- Action macros and scripting
- Cloud sync (optional)

## [3.0.0] - 2025-11-21

### 🎮 Multi-Protocol Input: Game Controller (HID) Support

**Major Release**: Conductor now supports game controllers (gamepads, joysticks, racing wheels, flight sticks, arcade controllers) alongside MIDI devices, enabling hybrid workflows with unified input management.

### Added - Multi-Protocol Input System

- **Game Controller (HID) Support**: Full support for SDL2-compatible game controllers
  - **Gamepads**: Xbox 360/One/Series, PlayStation DS4/DS5, Switch Pro Controller
  - **Joysticks**: Flight sticks, arcade sticks with analog axes and buttons
  - **Racing Wheels**: Logitech, Thrustmaster, and any SDL2-compatible wheel
  - **HOTAS**: Hands On Throttle And Stick systems for simulation
  - **Custom Controllers**: Any SDL2-compatible HID device
  - Button mapping (0-255, indexes 128-255 reserved for HID)
  - Axis support (LeftX, LeftY, RightX, RightY, LeftTrigger, RightTrigger)
  - Digital D-Pad mapping (Up, Down, Left, Right)

- **Unified InputManager**: Hybrid MIDI + HID input processing
  - Three operating modes: `MidiOnly`, `GamepadOnly`, `Both`
  - Unified event processing pipeline for MIDI and HID inputs
  - Independent device connection management
  - Graceful fallback when devices unavailable
  - Thread-safe concurrent device access

- **HID Trigger Types** (4 new types)
  - `GamepadButton`: Button press/release with button index (0-255)
  - `GamepadAxis`: Analog stick/trigger movement with threshold detection
  - `GamepadDPad`: Digital directional pad input (Up/Down/Left/Right)
  - `GamepadButtonCombo`: Multiple simultaneous button presses (chord detection)

### Added - Official Device Templates

- **6 Official Gamepad Templates**: Pre-configured mappings for popular controllers
  - `xbox-360-gamepad.toml` - Xbox 360 Controller
  - `xbox-one-gamepad.toml` - Xbox One/Series Controller
  - `playstation-ds4-gamepad.toml` - PlayStation DualShock 4
  - `playstation-ds5-gamepad.toml` - PlayStation DualSense (PS5)
  - `switch-pro-gamepad.toml` - Nintendo Switch Pro Controller
  - `generic-gamepad.toml` - Generic SDL2-compatible gamepad

- **Template Categories**: Organized by device type
  - `pad-controller` - Pad-based MIDI controllers
  - `gamepad` - Game controllers (Xbox, PlayStation, Switch, etc.)
  - `keyboard` - MIDI keyboard controllers
  - `mixer-controller` - DJ mixer-style controllers
  - More device types coming in future releases

### Added - GUI Integration

- **Template Selector Enhancement** (`config/templates/README.md`, 850+ lines)
  - Device type filtering in template browser
  - Gamepad category badge with controller icon
  - Search and filter by device name, vendor, or category
  - One-click template import with device type detection
  - Visual device type indicators (MIDI vs HID)

- **DevicesView Split**: Separate sections for MIDI and HID devices
  - **MIDI Controllers Section**: Lists connected MIDI input/output devices
  - **HID Game Controllers Section**: Lists connected game controllers with metadata
    - Controller name and vendor information
    - Button count and axis count display
    - Connection status indicators
    - Real-time connection/disconnection updates

- **MIDI Learn Integration**: MIDI Learn mode now supports HID devices
  - Capture gamepad button presses during MIDI Learn
  - Capture axis movements with threshold detection
  - Capture D-Pad directions
  - Auto-generate HID trigger configurations
  - Visual feedback during HID input capture

### Added - Daemon IPC Extensions

- **Extended Status Command**: IPC status now includes HID information
  - `input_mode` field: "midi-only", "gamepad-only", or "both"
  - `hid_devices` array: List of connected game controllers
    - Device name, vendor, product ID
    - Button count, axis count
    - Connection timestamp
  - Backward compatible with existing IPC clients

### Changed - Architecture

- **InputManager Modes**: Three operating modes for flexible workflows
  - `MidiOnly` (default): MIDI devices only (v2.x behavior)
  - `GamepadOnly`: HID game controllers only (no MIDI)
  - `Both`: Hybrid MIDI + HID workflows (unified event processing)

- **ID Range Allocation**: Clear separation of MIDI and HID identifiers
  - 0-127: Reserved for MIDI notes and CC messages
  - 128-255: Reserved for HID controller buttons and axes
  - Prevents conflicts in hybrid configurations

- **Event Processing Pipeline**: Unified handling of MIDI and HID events
  - Common `ProcessedEvent` enum for both input types
  - Unified trigger matching in mapping engine
  - Consistent velocity and timing detection across protocols

### Dependencies

- **Added**: `gilrs v0.10` - Cross-platform game controller library
  - Supports SDL2 game controller protocol
  - Works with gamepads, joysticks, racing wheels, flight sticks
  - Thread-safe device enumeration and event polling
  - Hot-plug support for USB controllers
  - Platform-specific backend (XInput on Windows, IOKit on macOS, evdev on Linux)

### Migration

- **100% Backward Compatible**: All existing MIDI configurations work unchanged
  - Default mode is `MidiOnly` (v2.x behavior)
  - No config changes required for MIDI-only workflows
  - Opt-in to HID support by setting `input_mode = "both"` or `input_mode = "gamepad-only"`

- **Migration Guide**: See `docs/MIGRATION_v2_to_v3.md` for complete guide
  - How to enable HID support
  - Converting MIDI mappings to HID mappings
  - Hybrid workflow examples
  - ID range best practices

### Documentation

- **User Guides** (2 new files, ~1,200 lines)
  - `docs/guides/gamepad-integration.md` - Complete game controller guide
    - Supported device types and vendors
    - Button and axis mapping reference
    - Configuration examples for common workflows
    - Troubleshooting and device detection
  - `docs/guides/hybrid-workflows.md` - MIDI + HID hybrid setup
    - When to use hybrid mode
    - Practical examples (DAW control, gaming macros, accessibility)
    - Best practices for ID range allocation

- **Configuration References** (1 new file, ~600 lines)
  - `docs/configuration/hid-triggers.md` - Complete TOML reference for HID triggers
    - GamepadButton, GamepadAxis, GamepadDPad, GamepadButtonCombo syntax
    - Threshold configuration for analog axes
    - Dead zone handling and sensitivity tuning
    - Validation rules and performance notes

- **Template Documentation** (`config/templates/README.md`, updated)
  - All 6 official gamepad templates documented
  - Device type categories explained
  - Template discovery and import instructions
  - Custom template creation guide

### Performance

- **HID Event Processing**: <0.5ms latency (comparable to MIDI)
- **Controller Polling**: 120 Hz default (8.3ms interval)
- **Memory Usage**: 10-15MB (5MB increase for HID support)
- **CPU Usage**: <2% idle, <6% active (1-2% increase for gamepad polling)
- **No impact on MIDI latency**: Still <1ms for MIDI events

### Testing

- **68 New Tests** (100% pass rate)
  - 15 InputManager tests (mode switching, device management)
  - 20 HID trigger tests (button, axis, D-Pad, combo)
  - 18 template loading tests (gamepad templates)
  - 15 GUI integration tests (DevicesView, MIDI Learn with HID)
- **Total Workspace Tests**: 213 tests passing (100% pass rate)
  - conductor-core: 60 tests (was 45)
  - conductor-daemon: 89 tests (was 74)
  - conductor-gui: 64 tests (was 26)

### Security

- **HID Device Sandboxing**: Controller access restricted to user-owned devices
- **Input Validation**: All button/axis values validated and clamped
- **No System Hooks**: Uses standard SDL2 APIs (no kernel extensions)
- **Permission Model**: Same Input Monitoring permission as MIDI (macOS)

### Platform Support

- **macOS**: Full support with IOKit backend
- **Linux**: Full support with evdev backend (udev rules may be required)
- **Windows**: Full support with XInput backend (Xbox controllers) and DirectInput fallback

### Known Limitations

- Gamepad LED control not yet implemented (planned for v3.1)
- Haptic feedback (vibration) not yet supported (planned for v3.1)
- Gyroscope/accelerometer data not exposed (planned for v3.2)
- Touchpad input (DS4/DS5) not yet supported (planned for v3.2)

### Breaking Changes

None - fully backward compatible with v2.7.0. All MIDI-only configurations work unchanged.

### Next Steps

- v3.1: Gamepad LED control and haptic feedback
- v3.2: Advanced HID features (gyroscope, touchpad)
- v3.3: Custom HID device profiles (beyond SDL2 gamepad mapping)

## [2.7.0] - 2025-11-19

### 🔐 Plugin Security & Verification

**Phase 6 (Part 4)**: Comprehensive security layer for WASM plugins with cryptographic signing, resource limiting, filesystem sandboxing, and enterprise-grade safety guarantees.

### Added - Plugin Signing & Verification

- **Ed25519 Digital Signatures** (486 lines) - Industry-standard cryptographic verification
  - 32-byte public keys, 64-byte signatures
  - SHA-256 binary integrity checking
  - Deterministic signing (identical inputs → identical signatures)
  - Embedded JSON metadata (signer name, email, timestamp, version)
  - Protection against tampering and unauthorized modifications

- **Three-Tier Trust Model** - Flexible security policies
  - **Unsigned**: Development and testing (security warnings displayed)
  - **Self-Signed**: Plugins signed with any valid key (authenticity verified)
  - **Trusted Keys**: Only allow plugins signed with pre-approved keys (maximum security)
  - Configurable per-plugin or system-wide
  - Trust store in `~/.conductor/trusted_keys.json`

- **CLI Signing Tool** (`conductor-sign`, 460 lines) - Complete key management and signing workflow
  - `generate-key` - Generate Ed25519 keypair with PEM encoding
  - `sign` - Sign WASM plugins with metadata embedding
  - `verify` - Verify plugin signatures and integrity
  - `trust add/remove/list` - Manage trusted key store
  - Portable PEM format for easy distribution
  - Integration with WASM plugin loader

### Added - Resource Limiting

- **Fuel Metering** - CPU instruction counting to prevent runaway plugins
  - Default: 100M instructions per execution
  - Configurable per-plugin (10M to 1B instructions)
  - Real-time tracking via wasmtime fuel API
  - Automatic termination on limit exceeded
  - Performance overhead: <1%

- **Memory Limits** - Prevent memory exhaustion
  - Default: 128 MB maximum memory per plugin
  - Configurable per-plugin (16 MB to 512 MB)
  - Linear memory growth constraints
  - Table growth limits (1024 elements default)
  - Protection against allocation attacks

### Added - Filesystem Sandboxing

- **Directory Preopening** (WASI) - Whitelist-based filesystem access
  - Explicit directory grants (read-only or read-write)
  - Path traversal prevention (no `../` escapes)
  - Default: No filesystem access unless explicitly granted
  - Per-plugin directory configuration
  - WASI preview1 standard compliance

### Added - Integration Tests

- **10 Plugin Signing Tests** (436 lines, 100% pass rate, 0.53s execution)
  - `test_sign_and_verify_workflow` - End-to-end signing workflow
  - `test_load_signed_plugin_with_self_signed_mode` - Self-signed loading
  - `test_reject_unsigned_plugin_when_required` - Signature enforcement
  - `test_reject_tampered_plugin` - Binary integrity detection
  - `test_reject_invalid_signature` - Wrong key rejection
  - `test_signature_metadata_format` - JSON metadata parsing
  - `test_load_unsigned_plugin_when_not_required` - Backward compatibility
  - `test_multiple_executions_with_signed_plugin` - Runtime verification
  - `test_key_size_validation` - Ed25519 key validation (32-byte enforcement)
  - `test_signature_deterministic` - Reproducible signatures

### Added - Documentation

- **mdBook WASM Plugin Documentation** (6,715 lines across 4 new pages)
  - `development/wasm-plugins.md` - Overview, architecture, security features
  - `development/wasm-plugin-development.md` - Complete development tutorial
  - `development/plugin-security.md` - 4-layer security architecture guide
  - `development/plugin-examples.md` - Real-world examples (Spotify, OBS, system utils)
  - Quick comparison tables (native vs WASM plugins)
  - Security checklists and best practices
  - Complete conductor-sign CLI reference
  - Configuration examples with all security modes

- **Technical Documentation** (648 lines)
  - `docs/v2.7-plugin-signing-complete.md` - Complete implementation report
  - Architecture diagrams with 4-layer security model
  - Performance benchmarks and overhead analysis
  - Integration guide for plugin developers

### Technical Details

- **Production Code**: ~1,400 lines across 3 new files
  - `conductor-core/src/plugin/signing.rs` (486 lines)
  - `conductor-daemon/src/bin/conductor-sign.rs` (460 lines)
  - `conductor-core/tests/plugin_signing_test.rs` (436 lines)

- **Dependencies Added**:
  - `ed25519-dalek v2.2` - Ed25519 signatures
  - `pem v3.0` - PEM encoding for keys
  - `base64 v0.22` - Base64 encoding for signatures

- **Test Coverage**: 10 integration tests (100% passing)
- **Build Time**: No measurable impact (still ~26s clean, ~4s incremental)
- **Runtime Overhead**:
  - Signature verification: <5ms on first load (one-time cost)
  - Fuel metering: <1% per execution
  - Memory tracking: Negligible

### Security Architecture

```
┌─────────────────────────────────────────────────┐
│  Security Layers                                │
│                                                 │
│  Layer 1: Cryptographic Verification           │
│  - Ed25519 digital signatures                   │
│  - SHA-256 integrity checking                   │
│                                                 │
│  Layer 2: Resource Limiting                     │
│  - CPU fuel metering (100M instructions)        │
│  - Memory limits (128 MB)                       │
│                                                 │
│  Layer 3: Filesystem Sandboxing                 │
│  - Directory preopening (WASI)                  │
│                                                 │
│  Layer 4: Capability System                     │
│  - Explicit permission model (from v2.3)        │
└─────────────────────────────────────────────────┘
```

### Usage

**Generate Keypair:**
```bash
conductor-sign generate-key ~/.conductor/my-key
# Creates: my-key.pem (private), my-key.pub.pem (public)
```

**Sign Plugin:**
```bash
conductor-sign sign my_plugin.wasm ~/.conductor/my-key \
  --name "Your Name" \
  --email "you@example.com"
# Creates: my_plugin.wasm.sig (detached signature)
```

**Verify Signature:**
```bash
conductor-sign verify my_plugin.wasm
# Output: Signature verified successfully (shows metadata)
```

**Manage Trust Store:**
```bash
# Add trusted key
conductor-sign trust add ~/.conductor/my-key.pub.pem "My Plugin"

# List trusted keys
conductor-sign trust list

# Remove trusted key
conductor-sign trust remove <public-key-hex>
```

**Configuration (Trusted Keys Mode):**
```toml
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { WasmPlugin = {
    path = "~/.conductor/wasm-plugins/my_plugin.wasm",
    signature_policy = "trusted_keys_only",  # Require pre-approved keys
    max_fuel = 50000000,  # 50M instructions
    max_memory_mb = 64,   # 64 MB limit
    allowed_dirs = [
        { path = "~/.conductor/plugin-data", writable = true }
    ]
}}
```

### Performance

- **Signature Verification**: <5ms (one-time on load)
- **Fuel Metering Overhead**: <1% per execution
- **Memory Tracking**: Negligible overhead
- **No impact on MIDI event processing latency**: Still <1ms

### Breaking Changes

None - fully backward compatible with v2.6.0. Unsigned plugins continue to work with security warnings.

### Migration Guide

1. Pull latest code: `git pull origin main`
2. Build release: `cargo build --release --workspace`
3. Install CLI tool: `cargo install --path conductor-daemon --bin conductor-sign`
4. (Optional) Generate signing keys: `conductor-sign generate-key ~/.conductor/my-key`
5. (Optional) Sign existing plugins: `conductor-sign sign plugin.wasm ~/.conductor/my-key`
6. (Optional) Configure trust store for maximum security

### Security Considerations

- **Unsigned plugins**: Display security warnings but execute (backward compatibility)
- **Self-signed plugins**: Verify signature authenticity, no pre-approval needed
- **Trusted keys mode**: Maximum security - only execute plugins from approved developers
- **Resource limits**: Prevent denial-of-service attacks from runaway plugins
- **Filesystem sandboxing**: Prevent unauthorized file access
- **No network sandboxing yet**: WASM plugins can make network requests if capability granted

### Known Issues

None

### Next Steps

- v2.8: Plugin marketplace with discovery and distribution
- v2.9: Network sandboxing for WASM plugins
- v3.0: Windows and Linux platform support for app detection

## [2.3.0] - 2025-01-18

### 🔌 Plugin Architecture

**Phase 6**: Extensible plugin system allowing third-party developers to create custom actions through dynamically loaded shared libraries with capability-based security.

### Added - Core Plugin Infrastructure

- **ActionPlugin Trait** (335 lines) - Core plugin interface with 7 methods
  - `name()`, `version()`, `description()`, `author()`, `license()` - Metadata methods
  - `execute()` - Main execution method with params and context
  - `capabilities()` - Capability requirements declaration
  - `initialize()` / `shutdown()` - Optional lifecycle hooks

- **Plugin Loader** (259 lines) - Dynamic library loading via libloading
  - Platform-specific binary support (.dylib/.so/.dll)
  - Symbol resolution for `_create_plugin` C-ABI function
  - Version compatibility checking
  - Safe trait object handling

- **Plugin Discovery** (440 lines) - Manifest-based plugin registry
  - Scans `~/.conductor/plugins/` for `plugin.toml` manifests
  - TOML-based plugin metadata parsing
  - Plugin registry with HashMap storage
  - Duplicate detection and validation

- **Capability System** (172 lines) - Permission-based security model
  - **6 Capability Types**: Network, Filesystem, Audio, Midi, Subprocess, SystemControl
  - **3 Risk Levels**: Low (auto-grant), Medium, High (explicit approval)
  - Auto-grant for safe capabilities (Network, Audio, Midi)
  - Per-plugin capability tracking

### Added - Plugin Manager

- **PluginManager** (645 lines) - Lifecycle and execution management
  - Thread-safe with Arc<RwLock<HashMap>>> for concurrent access
  - Plugin lifecycle: discover → load → initialize → execute → shutdown → unload
  - SHA256 binary verification (optional)
  - Execution statistics (call count, failures, latency)
  - Error handling with comprehensive error types

- **Action::Plugin Integration** - Seamless action execution
  - New `Action::Plugin { plugin, params }` variant
  - TriggerContext propagation (velocity, mode, timestamp)
  - JSON parameter support via serde_json::Value
  - Backward compatible with existing actions

### Added - GUI Plugin Manager

- **Plugin Management UI** (850 lines) - Complete plugin control interface
  - Plugin discovery and listing with metadata cards
  - Load/unload controls for lifecycle management
  - Enable/disable toggles for plugin availability
  - Capability grant/revoke with risk level indicators
  - Execution statistics display (calls, failures, latency)
  - Search and filtering by name, type, capabilities
  - Risk level badges (color-coded: green/yellow/red)

- **Tauri Backend Commands** (274 lines) - 11 plugin management commands
  - `plugin_discover` - Scan for new plugins
  - `plugin_list_available` / `plugin_list_loaded` - List plugins
  - `plugin_get_metadata` - Fetch plugin details
  - `plugin_load` / `plugin_unload` - Lifecycle control
  - `plugin_enable` / `plugin_disable` - Toggle availability
  - `plugin_grant_capability` / `plugin_revoke_capability` - Permission management
  - `plugin_get_stats` - Get execution metrics

### Added - Example Plugin

- **HTTP Request Plugin** (265 lines + 200 lines docs) - Reference implementation
  - HTTP methods: GET, POST, PUT, DELETE
  - Custom headers support
  - JSON body with velocity substitution (`{velocity}` placeholder)
  - Error handling and logging
  - 5 unit tests covering all features
  - Complete README with usage examples

### Added - Documentation

- **Plugin Development Guide** (850+ lines) - Comprehensive tutorial
  - Quick start guide with step-by-step instructions
  - Complete API reference
  - Capability system explanation
  - Testing strategies
  - Distribution instructions
  - Best practices and troubleshooting

- **mdbook Integration** - Added to documentation site
  - `/development/plugin-development.md` - Developer guide
  - Integration with existing documentation structure

### Technical Details

- **Production Code**: ~5,800 lines across 11 new files
- **Test Coverage**: 42 plugin-specific tests (100% passing)
- **Dependencies Added**: libloading, sha2
- **Build Time**: No measurable impact (still ~26s clean, ~4s incremental)
- **Runtime Overhead**: <0.1ms per plugin execution

### Security

- Capability-based permission system prevents unauthorized access
- Risk-level assessment (Low/Medium/High) with auto-grant logic
- SHA256 checksum verification for binary integrity
- Plugins run in same process (not sandboxed) - trust required
- GUI displays risk levels clearly with color-coded badges

### Performance

- Plugin loading: ~10-50ms per plugin (one-time cost)
- Discovery: ~5ms for 10 plugins
- Execution overhead: <0.1ms per action
- No impact on existing action types

### Breaking Changes

None - fully backward compatible with v2.2.0

### Migration Guide

1. Pull latest code
2. Run `cargo build --release`
3. Create `~/.conductor/plugins/` directory
4. Install plugins as needed
5. Use GUI Plugin Manager to manage plugins

### Known Issues

None

## [2.2.0] - 2025-11-18

### 🎯 Velocity Curves & Advanced Conditionals

**Phase 5 (Part 2)**: Context-aware mappings and velocity-sensitive controls enabling dynamic workflows that adapt to time, application context, and input intensity.

### Added - Advanced Conditionals System

- **10 Condition Types**: Build complex conditional logic for context-aware actions
  - `Always` / `Never` - Testing and debugging conditions
  - `TimeRange` - Time-based workflows (HH:MM format, supports midnight crossing)
  - `DayOfWeek` - Day-based workflows (1=Monday through 7=Sunday)
  - `AppRunning` - Process detection (macOS, Linux via `pgrep`)
  - `AppFrontmost` - Active window detection (macOS via NSWorkspace)
  - `ModeIs` - Current mode matching for mode-aware actions
  - `And` / `Or` - Logical operators with short-circuit evaluation
  - `Not` - Logical negation for inverted conditions

- **Conditional Action Type**: Execute different actions based on runtime conditions
  - `then_action` - Action executed when conditions are true
  - `else_action` - Action executed when conditions are false
  - Nested conditions support (unlimited depth)
  - Real-time condition evaluation with <1ms latency

### Added - Velocity Mapping System

- **4 Velocity Mapping Types**: Transform trigger velocity to action-specific values
  - `Fixed` - Constant velocity output (ignore input velocity)
  - `PassThrough` - 1:1 direct mapping (velocity unchanged)
  - `Linear` - Custom min/max range scaling with configurable bounds
  - `Curve` - Non-linear transformations with intensity control:
    - **Exponential**: `output = input^(1-intensity)` - Boost soft hits
    - **Logarithmic**: `log(1 + intensity × input) / log(1 + intensity)` - Compress hard hits
    - **S-Curve**: Sigmoid function with intensity-controlled steepness

- **Integration with SendMIDI**: Velocity mapping applies to MIDI output messages
  - Map trigger velocity → MIDI NoteOn velocity dynamically
  - Real-time curve calculation with <0.1ms overhead
  - Visual curve preview in GUI

### Added - Mode Context Propagation

- **TriggerContext Enhancement**: Actions now receive current mode information
  - `current_mode: Option<usize>` field added to TriggerContext
  - Enables `ModeIs` condition evaluation
  - Backward compatible (optional field)

### Added - GUI Components

- **ConditionalActionEditor** (596 lines)
  - Visual condition builder for all 10 condition types
  - Time picker for TimeRange conditions
  - Day selector for DayOfWeek conditions
  - App selector with process detection
  - Logical operator composition (And/Or/Not)
  - Nested condition support with tree view
  - Real-time validation with error display

- **VelocityMappingSelector**
  - Curve type selector (Fixed/PassThrough/Linear/Curve)
  - Real-time curve preview graph (SVG visualization)
  - 64-point curve sampling for smooth preview
  - Interactive parameter controls (min/max/intensity)
  - Visual feedback for curve shape

### Documentation

- **User Guides** (2 new files, ~1,000 lines)
  - `docs-site/src/guides/velocity-curves.md` - Complete velocity mapping guide
    - All 4 mapping types with mathematical formulas
    - Practical use cases and examples
    - GUI configuration instructions
    - Tips and best practices
  - `docs-site/src/guides/context-aware.md` - Context-aware mappings guide
    - All 10 condition types documented
    - Platform support notes (macOS/Linux/Windows)
    - Real-world practical examples
    - Nested condition patterns

- **Configuration References** (2 new files, ~800 lines)
  - `docs-site/src/configuration/curves.md` - Complete TOML reference for velocity mappings
    - Parameter constraints and validation rules
    - Intensity parameter guide
    - Default behavior documentation
  - `docs-site/src/configuration/conditionals.md` - Complete TOML reference for conditions
    - All 10 condition types with syntax examples
    - Nested conditions documentation
    - Validation rules and performance notes

- **Tutorial** (1 new file, ~500 lines)
  - `docs-site/src/tutorials/dynamic-workflows.md` - Step-by-step workflow tutorial
    - Beginner: Time-based app launcher
    - Intermediate: Velocity-sensitive DAW control
    - Advanced: Multi-condition smart assistant
    - Best practices and debugging tips

- **Updated Files**
  - `docs-site/src/configuration/actions.md` - Updated Conditional action reference
  - `docs-site/src/SUMMARY.md` - Added new guides and tutorial section

### Performance

- **Condition Evaluation**: <1ms for most conditions
  - TimeRange/DayOfWeek: Very fast (system time lookup)
  - ModeIs: Very fast (string comparison)
  - AppFrontmost: Very fast (<1ms, native API)
  - AppRunning: Moderate (~10ms, subprocess call)
  - And/Or: Short-circuit evaluation for efficiency

- **Velocity Curve Calculation**: <0.1ms
  - No performance impact on MIDI event processing
  - Memory usage: 5-10MB (no increase from v2.1)

### Testing

- **145 Workspace Tests Passing** (100% pass rate)
  - conductor-core: 45 tests
  - conductor-daemon: 74 tests
  - conductor-gui: 26 tests (1 ignored)
- No regressions from v2.0 or v2.1
- Comprehensive condition evaluation test coverage
- Velocity curve calculation unit tests

### Changed

- `TriggerContext` struct extended with optional `current_mode` field (backward compatible)
- `ActionConfig` enum extended with `Conditional` variant
- Condition evaluation system added to `conductor-daemon/src/conditions.rs` (425 lines)

### Security

- Shell commands properly sanitized in conditional execution
- Safe system APIs for app detection (pgrep, NSWorkspace)
- Time parsing validated with error handling
- No user code execution in condition evaluation

## [2.1.0] - 2025-11-17

### 🎹 Virtual MIDI Output

**Phase 5 (Part 1)**: Full MIDI output support enabling DAW control, hardware synth integration, and MIDI routing capabilities.

### Added - Virtual MIDI Port Creation

- **Platform-Specific Virtual Port Support**
  - macOS: CoreMIDI virtual sources via IAC Driver
  - Linux: ALSA/JACK virtual port creation
  - Windows: Physical port support (virtual requires loopMIDI driver)
  - Auto-detection of virtual vs. physical ports

### Added - MidiOutputManager

- **Core MIDI Output Engine** (`conductor-core/src/midi_output.rs`, 618 lines)
  - 11 public methods for port management
  - Connection pooling for multiple output ports
  - Thread-safe message queueing with `Arc<Mutex<VecDeque>>`
  - Platform-conditional compilation for virtual port support
  - Comprehensive error handling with `EngineError::MidiOutput` variants

- **Public API**:
  - `create_virtual_port(port_name: &str)` - Create named virtual MIDI port
  - `list_output_ports()` - List all available MIDI output ports
  - `connect_to_port(port_index: usize)` - Connect to output port by index
  - `send_message(port_index: usize, message: &[u8])` - Send raw MIDI bytes
  - `disconnect_port(port_index: usize)` - Close specific port connection
  - `disconnect_all()` - Close all active connections

### Added - SendMIDI Action Type

- **6 MIDI Message Types** - Full MIDI 1.0 channel voice message support
  - `NoteOn` (0x90) - Trigger notes with velocity (0-127)
  - `NoteOff` (0x80) - Release notes
  - `CC` (Control Change, 0xB0) - Continuous controllers (CC 0-127, value 0-127)
  - `ProgramChange` (0xC0) - Preset/patch selection (0-127)
  - `PitchBend` (0xE0) - 14-bit pitch wheel control (-8192 to +8191)
  - `Aftertouch` (0xD0) - Channel pressure (0-127)

- **Configuration Flexibility**
  - MIDI channel selection (0-15, displayed as 1-16 in UI)
  - 19 message type aliases for readable configs (e.g., "note-on", "control-change")
  - Sensible defaults (note=60/Middle C, velocity=100, channel=0)
  - Comprehensive parameter validation with detailed error messages

- **MIDI Spec Compliance**
  - Status byte channel masking (0-15)
  - Data byte masking (0-127, 7-bit values)
  - 14-bit pitch bend encoding (LSB/MSB)
  - Out-of-range value clamping
  - Proper message framing per MIDI 1.0 specification

### Added - ActionExecutor Integration

- **MIDI Message Encoding** (`conductor-daemon/src/action_executor.rs`, ~280 lines)
  - Complete byte-level MIDI encoding for all 6 message types
  - Channel byte manipulation (0x00-0x0F)
  - Data byte validation and masking (0x7F)
  - 14-bit pitch bend conversion (split into LSB/MSB bytes)
  - Error handling for invalid parameters
  - Integration with existing action execution pipeline

### Added - GUI Components

- **Tauri Commands** (AMI-268, 224 lines)
  - `list_midi_output_ports()` - Lists all MIDI output ports with metadata
  - `test_midi_output(port, note, velocity, duration)` - Send test MIDI message
  - `validate_send_midi_action(action_config)` - Validate SendMIDI configurations
  - AppState integration with MidiOutputManager

- **MidiOutputSelector Component** (AMI-269, 450 lines)
  - Port selection dropdown with auto-refresh
  - Virtual/physical port badges (🔷 blue for virtual, 🔌 green for physical)
  - Platform badges (🍎 macOS, 🐧 Linux, 🪟 Windows)
  - Test output button (sends Middle C for verification)
  - Error/empty/loading state handling
  - Dark theme matching existing Conductor GUI

- **SendMidiActionEditor Component** (AMI-270, 800 lines)
  - All 6 MIDI message type editors:
    - Note On/Off: Note slider with musical note names (C4, D#5, etc.)
    - Control Change: Common CC dropdown (Volume, Pan, Modulation, etc.)
    - Program Change: Preset selector (0-127)
    - Pitch Bend: Bidirectional indicator (-8192 to +8191)
    - Aftertouch: Pressure control (0-127)
  - MIDI channel selector (1-16 display)
  - Dynamic parameter fields (change based on message type)
  - Real-time validation with 300ms debounce
  - Color-coded indicators (velocity bar, pitch bend direction)
  - Integration with MidiOutputSelector
  - Readonly mode for viewing existing configs

- **Svelte Store Integration**
  - `midiOutputPortsStore` - Centralized port state management
  - `api.midiOutput.*` - API namespace for MIDI output operations
  - Real-time port refresh and validation

### Documentation

- **User Guide** (`docs/send-midi-action-guide.md`, ~580 lines)
  - Quick start tutorial (3 easy steps)
  - All 6 message types with practical examples
  - Platform-specific setup instructions:
    - macOS: IAC Driver configuration
    - Linux: ALSA/JACK virtual port creation
    - Windows: loopMIDI driver installation
  - Troubleshooting guide for common MIDI issues
  - MIDI reference tables:
    - Common CC numbers (Volume, Pan, Modulation, etc.)
    - General MIDI drum map (kick=36, snare=38, etc.)
    - MIDI note numbers with musical notation

- **Example Configurations** (2 files, ~830 lines)
  - `config/examples/daw-control-ableton.toml` (~450 lines)
    - 3 modes: Instruments, Mixer, Effects
    - 21+ real-world DAW control mappings
    - MIDI panic sequence (all notes off)
    - Arpeggio pattern examples
  - `config/examples/hardware-synth-control.toml` (~380 lines)
    - 4 modes: Performance, Sound Design, Presets, Multi-Synth Routing
    - 27+ mappings for external hardware synths
    - Chord stacking examples (power chords, triads)
    - Multi-output routing for multiple synths

- **Technical Documentation** (~4,500 lines across 7 files)
  - Architecture design document
  - Implementation completion report
  - GUI integration reports (AMI-268, AMI-269, AMI-270)
  - Final verification report
  - Platform support matrix

### Testing

- **47 New Tests** (100% pass rate)
  - 7 unit tests for MidiOutputManager
  - 18 doctests for API documentation examples
  - 10 integration tests for SendMIDI action (TOML parsing, validation, encoding)
  - 12 unit tests for ActionExecutor MIDI encoding
  - All edge cases covered (invalid channels, out-of-range values, etc.)

### Performance

- MIDI message encoding: <0.1ms per message
- Port connection: <10ms
- Memory usage: 5-10MB (no significant increase)
- Zero latency overhead on MIDI event processing

### Security

- MIDI output restricted to configured ports only
- Data byte masking prevents buffer overruns
- Port index validation prevents out-of-bounds access
- Error messages do not expose system internals

## [2.0.0] - 2025-11-14

### 🎉 Major Release: Tauri GUI & Visual Configuration

**Phase 4 Complete**: Full-featured visual configuration interface built with Tauri v2, providing an intuitive GUI for MIDI mapping management, MIDI Learn mode, per-app profiles, and real-time debugging.

### Added - Visual Configuration Editor

- **Mode-Based Config Management**: Create and manage modes with color coding
  - Visual mode editor with inline editing
  - Drag-and-drop mapping organization
  - Real-time validation and preview
  - Color-coded mode indicators

- **Mapping List UI**: CRUD operations for MIDI mappings
  - Add, edit, delete mappings
  - Type-specific trigger and action selectors
  - Live preview of trigger events
  - Automatic validation and error highlighting

- **Trigger Selector**: Visual selector with type-specific configuration
  - Note, CC, VelocityRange, LongPress, DoubleTap, EncoderTurn, PitchBend, Aftertouch
  - Context-aware form fields for each trigger type
  - Real-time parameter validation

- **Action Selector**: Visual selector with type-specific configuration
  - Keystroke, Text, Launch, Shell, VolumeControl, ModeChange, Sequence, etc.
  - Keystroke picker with live key capture
  - Application launcher with file browser
  - Shell command editor with syntax highlighting

### Added - MIDI Learn Mode

- **One-Click MIDI Learn**: Auto-detect MIDI inputs with single click
  - 10-second countdown timer with cancel option
  - Auto-detection of trigger type (Note, CC, VelocityRange, etc.)
  - Support for all trigger types
  - Visual feedback during learning
  - Automatic config generation from captured events

### Added - Per-App Profile System

- **Automatic Profile Switching**: Context-aware mapping based on frontmost app
  - macOS frontmost app detection via NSWorkspace
  - Profile auto-switching when app focus changes
  - Profile caching with SHA256-based validation
  - Profile import/export (JSON and TOML formats)
  - Profile discovery and auto-registration
  - Profile manager UI with visual indicators

### Added - Device Template Library

- **6 Built-in Controller Templates**: Pre-configured mappings for popular devices
  - Native Instruments Maschine Mikro MK3
  - Novation Launchpad Mini MK3
  - KORG nanoKONTROL2
  - Akai APC Mini
  - Arturia BeatStep
  - Generic 25-Key MIDI Keyboard
- Auto-detection via MIDI device name pattern matching
- Category filtering (pad-controller, keyboard, mixer-controller)
- Template browser with search and filter
- One-click config generation from templates

### Added - Live Event Console

- **Real-time MIDI Event Monitoring**: Debug MIDI inputs in real-time
  - Color-coded event types (NoteOn=green, CC=blue, PitchBend=purple, etc.)
  - Filter by event type and channel
  - Pause/resume functionality
  - Event count tracking
  - Raw MIDI byte display (hex format)
  - Note name display (C4, D#5, etc.)
  - Timestamp with millisecond precision

### Added - Settings Panel

- **Application Preferences**: Configure GUI behavior
  - Auto-start on login (UI ready, OS integration TBD)
  - Theme selection (Light/Dark/System, UI ready)
  - MIDI Learn timeout adjustment (5-60 seconds)
  - Event buffer size control (100-10,000 events)
  - Log level configuration (Error/Warn/Info/Debug)
  - About section with version and links

### Added - Menu Bar Integration

- **Native System Tray**: Platform-specific menu bar
  - macOS: Native NSApplication menu bar
  - Quick actions: Pause, Reload, Configure, Quit
  - Status indicators: Running, Stopped, Error
  - Minimize to tray functionality

### Technical Stack

- **Backend**: Tauri v2.9.3 with Rust
  - 40+ Tauri commands for IPC
  - Thread-safe state with Arc<RwLock<>>
  - JSON-based IPC protocol
  - Event streaming for real-time updates

- **Frontend**: Svelte 5.1.9 with Vite 6.4.1
  - 14 custom UI components
  - TypeScript for type safety
  - Reactive state management
  - Fast builds (~400ms)

### Performance

- Daemon IPC: <1ms round-trip
- MIDI Learn start: <50ms
- Profile switching: <100ms
- Memory usage: ~60MB total
- Frontend build: <500ms

### Platform Support

- **macOS**: Full support with native integration
- **Linux**: Basic support (app detection TBD)
- **Windows**: Basic support (app detection TBD)

### Issues Completed (26/26)

**Week 1-2**: AMI-158-166 (Tauri Setup & Infrastructure)
**Week 3**: AMI-171-174 (MIDI Learn Mode)
**Week 4**: AMI-175-180 (Visual Config Editor)
**Week 5**: AMI-181-184 (Per-App Profiles)
**Week 6**: AMI-185-187 (Polish & Release)

### Known Limitations

- Documentation site not yet updated (deferred to Phase 5)
- Auto-start OS integration pending (UI complete)
- Theme switching implementation pending (UI complete)
- App detection macOS-only (Linux/Windows TBD)
- Drag-and-drop mapping reorder planned but not implemented

## [1.0.0] - 2025-01-13

### 🎉 Major Release: Production-Ready Daemon

**Phase 3 Complete**: Full daemon architecture with hot-reload, IPC control, and service integration. This is the first production-ready release with zero-downtime configuration updates.

### Added - Daemon Infrastructure

- **Background Daemon Service**: Runs as persistent background service
  - Unix domain socket IPC for inter-process communication
  - Graceful shutdown with SIGTERM/SIGINT handling
  - State persistence across restarts (`~/.local/state/conductor/daemon.state`)
  - 8-state lifecycle machine (Initializing → Running → Reloading → Degraded → etc.)
  - Atomic config swaps using Arc<RwLock<>> pattern

### Added - Configuration Hot-Reload

- **Zero-Downtime Config Reload**: Changes detected and applied in 0-10ms typical
  - File system watcher with 500ms debounce window
  - Automatic change detection on config file save
  - Phase-by-phase timing (config load, mapping compile, atomic swap)
  - Performance grading system:
    - Grade A (<20ms): Excellent - Imperceptible
    - Grade B (21-50ms): Good - Target performance
    - Grade C (51-100ms): Acceptable
    - Grade D (101-200ms): Poor - Investigate
    - Grade F (>200ms): Unacceptable
  - Running statistics (fastest, slowest, average reload times)
  - Reload counter and performance history

### Added - CLI Control Tool (conductorctl)

- **Command-Line Interface**: Control daemon from terminal or scripts
  - `status` - Query daemon state, uptime, events processed, reload stats
  - `reload` - Force immediate configuration reload
  - `ping` - Test connectivity and measure IPC latency
  - `stop` - Gracefully stop daemon
  - `validate [--config PATH]` - Validate configuration files
  - Dual output modes:
    - Human-readable: Colored terminal output with Unicode symbols
    - JSON: Machine-readable for scripting (`--json` flag)
  - Verbose logging mode (`--verbose` flag)

### Added - Service Integration

- **systemd Service Template** (`conductor-daemon/systemd/conductor.service`):
  - User-level service support
  - Auto-restart on failure (5s throttle, max 5 bursts per 5 minutes)
  - Security hardening (NoNewPrivileges, ProtectSystem=strict, ProtectHome=read-only)
  - Resource limits (1024 file descriptors, 64 processes)
  - Journal logging integration
  - ExecReload support via conductorctl

- **macOS LaunchAgent** (`conductor-daemon/launchd/com.amiable.conductor.plist`):
  - Run at login with LaunchAgent plist
  - Crash recovery with 5s throttled restart
  - Process priority configuration (Nice -5 for low latency)
  - Log file rotation to `~/Library/Logs/conductor.log`
  - GUI session integration (LimitLoadToSessionType: Aqua)

### Added - Documentation

- **Man Pages**: Professional Unix manual pages
  - `conductor(1)` - Daemon manual (trigger types, action types, config format)
  - `conductorctl(1)` - CLI tool reference (commands, options, examples)
  - Installation to `/usr/local/share/man/man1/`

- **DEPLOYMENT.md**: Comprehensive deployment guide (500+ lines)
  - Quick start instructions
  - Platform-specific installation (macOS LaunchAgent, Linux systemd)
  - Service management commands
  - Configuration management
  - Monitoring and log analysis
  - Troubleshooting guide with common issues
  - Performance benchmarking guide
  - Uninstallation procedures

### Added - Engine Enhancements

- **Performance Metrics** (`daemon/types.rs`):
  - Config load timing (ms)
  - Mapping compilation timing (ms)
  - Atomic swap timing (ms)
  - Total reload duration (ms)
  - Performance grade calculation (A-F)

- **Daemon Statistics** (`daemon/types.rs`):
  - Events processed counter
  - Actions executed counter
  - Error tracking since start
  - Config reload counter
  - Uptime tracking (seconds)
  - Reload performance history

### Added - Testing & Benchmarking

- **Reload Benchmark Suite** (`conductor-daemon/benches/reload_benchmark.rs`):
  - Multiple config sizes (2-10 modes, 10-100 mappings)
  - 10 iterations per test for statistical reliability
  - Average, min, max timing measurements
  - Performance grading validation

- **Daemon Integration Tests**:
  - IPC protocol tests (request/response cycle)
  - Config reload tests (atomic swaps, no downtime)
  - State machine transition tests
  - Error handling tests
  - 45 tests total, all passing (1 marked `#[ignore]` for CI flakiness)

### Changed - Architecture

- **conductor-daemon** structure:
  - Added `src/daemon/` module (7 files, ~2,000 lines)
    - `service.rs` - Main daemon service loop
    - `engine_manager.rs` - Engine lifecycle management
    - `config_watcher.rs` - File system watching with debouncing
    - `ipc.rs` - IPC server and client
    - `state.rs` - State persistence and socket path logic
    - `types.rs` - IPC protocol types, metrics, statistics
    - `error.rs` - Daemon-specific error types
  - Added `src/bin/conductorctl.rs` - CLI control tool (360 lines)
  - Added `src/bin/conductor_menubar.rs` - Menu bar foundation (262 lines, incomplete)
  - Added `benches/reload_benchmark.rs` - Performance benchmarking (166 lines)

- **IPC Client API** (`daemon/ipc.rs`):
  - Added `IpcClient::new(socket_path)` for custom socket paths
  - Added `IpcClient::send_command(command, args)` for generic command sending
  - Existing methods (`ping`, `status`, `reload`, `stop`) now use generic API

### Changed - Performance

**Config Reload Optimization**: 5-6x faster than 50ms target

Benchmark results (Apple M1 MacBook Pro):

| Config Size | Reload Time | Grade | Improvement |
|-------------|-------------|-------|-------------|
| 2 modes, 10 mappings | 0-2ms | A | 10-25x faster |
| 5 modes, 50 mappings | 2-5ms | A | 10-25x faster |
| 10 modes, 100 mappings | 5-8ms | A | 6-10x faster |

**All configurations achieve Grade A performance** (<20ms).

### Fixed

- **notify-debouncer-full API**: Updated to v0.4 API (deprecated `.watcher()` and `.cache()` methods)
- **Config Format**: Fixed Keystroke action format in benchmarks (string keys, not array)
- **Import Warnings**: Removed unused imports from daemon modules
- **Test Reliability**: Marked file watcher test as `#[ignore]` for CI stability (file watching is inherently timing-sensitive)

### Known Issues

- **Menu Bar UI**: Foundation created but incomplete
  - Send/Sync issues with `tray-icon` crate on macOS
  - Platform-specific threading model constraints
  - Requires platform-specific implementations or Tauri framework
  - Documented for future Phase 3 work

- **Windows Support**: Not yet implemented
  - IPC requires named pipes implementation
  - Service integration requires Windows Service framework
  - Planned for future release

### Migration Guide

#### From v0.2.0 to v1.0.0

**No breaking changes** - All v0.2.0 configurations work identically.

**New daemon features to adopt**:

1. **Install as Service** (recommended):
   ```bash
   # macOS
   launchctl load ~/Library/LaunchAgents/com.amiable.conductor.plist

   # Linux
   systemctl --user enable conductor
   systemctl --user start conductor
   ```

2. **Use conductorctl for Control**:
   ```bash
   conductorctl status   # Check daemon health
   conductorctl reload   # Apply config changes
   conductorctl ping     # Test connectivity
   ```

3. **Enable Hot-Reload**:
   - Edit `~/.config/conductor/config.toml`
   - Changes automatically detected and applied in <10ms
   - No daemon restart needed

**Manual mode still supported**:
```bash
conductor --config config.toml --log-level debug
```

### Dependencies

#### New Dependencies
- `tokio` (1.40) - Async runtime for daemon event loop
- `interprocess` (2.2) - Cross-platform IPC (Unix sockets)
- `notify` (7.0) - File system change notifications
- `notify-debouncer-full` (0.4) - Debounced file events
- `tray-icon` (0.19) - System tray integration (foundation)
- `dirs` (5.0) - Standard directory paths (XDG Base Directory)
- `uuid` (1.0) - Request ID generation for IPC
- `sha2` (0.10) - Config checksums for integrity verification
- `tracing` (0.1) - Structured logging
- `tracing-subscriber` (0.3) - Log formatting and filtering

#### Updated Dependencies
- All workspace dependencies remain at v0.2.0 versions

### Performance Metrics

**Measured on Apple M1 MacBook Pro**:

- **MIDI Event Latency**: <1ms (unchanged)
- **Config Reload Time**: 0-10ms typical (Grade A: <20ms)
- **Startup Time**: <500ms
- **Memory Usage**: 5-10MB (unchanged)
- **CPU Usage**: <1% idle, <5% active (unchanged)
- **Binary Size**: ~3-5MB (unchanged)

### Contributors

- Christopher Joseph (@christopherjoseph) - All v1.0.0 features

### Release Artifacts

- conductor-v1.0.0-macos-arm64.tar.gz (Apple Silicon)
- conductor-v1.0.0-macos-x86_64.tar.gz (Intel)
- conductor-v1.0.0-linux-x86_64.tar.gz (Linux)
- checksums.txt (SHA256)

## [0.2.0] - 2025-11-12

### Overview

**Phase 2 Complete**: Workspace architecture migration with zero breaking changes. Conductor now uses a modular 3-package workspace structure, enabling better code organization, faster builds, and preparing for future GUI integration.

**100% Backward Compatible**: All v0.1.0 configs, features, and workflows work identically in v0.2.0.

### Added - Architecture

- **conductor-core**: Pure Rust engine library (zero UI dependencies)
  - Public API for embedding in other applications
  - Structured error types using `thiserror`
  - Comprehensive rustdoc documentation
  - 30+ public types exported
- **conductor-daemon**: CLI daemon + 6 diagnostic tools
  - Main `conductor` binary
  - `midi_diagnostic`, `led_diagnostic`, `led_tester`
  - `pad_mapper`, `test_midi`, `midi_simulator`
- **conductor** (root): Backward compatibility layer
  - Re-exports conductor-core types
  - Maintains v0.1.0 import paths
  - Zero breaking changes for existing tests

### Added - Testing

- **25 new integration tests** (339 tests total, was 314)
  - 8 API integration tests (public API surface)
  - 7 backward compatibility tests
  - 10 error handling tests (across crate boundaries)
- **100% feature validation**: All 26 features tested and working
- **Config compatibility tests**: All v0.1.0 configs validated

### Changed - Performance

- **Build time**: 11.92s clean build (was 15-20s) - **25-40% faster** ✨
  - Workspace parallelization across 3 packages
  - Improved incremental compilation
- **Test execution**: 28.8s (was ~30s) - **4% faster**
  - Parallel test execution per package
- **Binary size**: Unchanged (869K main binary)

### Changed - Internal Structure

- Renamed `src/mappings.rs` → `conductor-core/src/mapping.rs`
- Renamed `src/device_profile.rs` → `conductor-core/src/device.rs`
- Added `conductor-core/src/error.rs` (structured error types)
- Split monolithic src/ into modular workspace packages
- Removed UI dependencies (colored, chrono) from core library

### Documentation

- **CLAUDE.md**: Updated with workspace architecture and Phase 2 status
- **README.md**: Updated installation and build commands
- **mdbook**: Updated architecture diagrams
- **Rustdoc**: Comprehensive API documentation in conductor-core
- **Migration Guide**: docs/MIGRATION_v0.1_to_v0.2.md

### Validation

- **Feature Parity**: 26/26 features validated ✅
- **Config Compatibility**: 15 compatibility tests passing ✅
- **Breaking Changes**: 0 (zero) ✅
- **Test Coverage**: 339/339 tests passing (100%) ✅

### Migration Notes

**For Users**: No action required. All configs and workflows work identically.

**For Developers**: Update build commands:
```bash
# Old
cargo build --release
cargo test

# New
cargo build --release --workspace
cargo test --workspace
```

See `docs/MIGRATION_v0.1_to_v0.2.md` for complete guide.

## [0.1.0-monolithic] - 2025-11-11

### Overview

Initial public release of Conductor, preserving the complete working monolithic implementation with all 26 features before migration to workspace structure. This release establishes the foundation for open source development and community contributions.

### Added - Core Triggers (4)

- **Note Trigger**: Basic note on/off detection with optional velocity range filtering
- **VelocityRange Trigger**: Different actions for soft (0-40), medium (41-80), and hard (81-127) velocity levels
- **EncoderTurn Trigger**: Encoder rotation detection with clockwise/counterclockwise direction
- **CC (Control Change) Trigger**: MIDI Control Change message handling

### Added - Advanced Triggers (5)

- **LongPress Trigger**: Configurable hold duration detection (default 2000ms)
- **DoubleTap Trigger**: Quick double-tap detection with configurable window (default 300ms)
- **NoteChord Trigger**: Multiple simultaneous note detection (default 100ms chord window)
- **Aftertouch Trigger**: Pressure sensitivity detection for supported devices
- **PitchBend Trigger**: Touch strip/pitch wheel detection with range support

### Added - Actions (10)

- **Keystroke Action**: Keyboard shortcuts with full modifier support (Cmd, Ctrl, Alt, Shift)
- **Text Action**: Type text strings with automatic character conversion
- **Launch Action**: Open applications and files with system default handlers
- **Shell Action**: Execute shell commands and scripts with full environment access
- **VolumeControl Action**: System volume adjustment (Up, Down, Mute, Set to value)
- **ModeChange Action**: Switch between mapping modes with LED feedback
- **Sequence Action**: Chain multiple actions with timing control
- **Delay Action**: Add timing delays between actions (milliseconds)
- **MouseClick Action**: Simulate mouse button clicks (Left, Right, Middle)
- **Repeat Action**: Execute an action multiple times with optional delays

### Added - LED Feedback System (10 Schemes)

- **Off**: All LEDs disabled
- **Static**: Solid color display with configurable RGB values
- **Breathing**: Smooth pulsing fade in/out effect
- **Pulse**: Quick flash effect for event triggers
- **Rainbow**: Animated rainbow color cycle across pads
- **Wave**: Wave pattern sweeping across pad grid
- **Sparkle**: Random sparkle/twinkle effects
- **Reactive**: Velocity-sensitive color feedback (green=soft, yellow=medium, red=hard) with 1-second fade
- **VU Meter**: Audio level meter visualization
- **Spiral**: Spiral pattern animation from center outward

### Added - System Features (7)

- **Multi-Mode System**: Support for multiple mapping modes (Default, Development, Media, etc.) with independent configurations
- **Global Mappings**: Mappings that work across all modes (e.g., emergency exit, encoder volume control)
- **Device Profile Support**: Load Native Instruments Controller Editor profiles (.ncmm3 XML format)
- **Auto-Detect Pad Page**: Automatically detect active pad page (A-H) from incoming MIDI events
- **HID Shared Device Access**: Concurrent access with Native Instruments Controller Editor using `hidapi` with `macos-shared-device` feature
- **Graceful Shutdown**: Clean MIDI connection closure and LED reset on exit (Ctrl+C handling)
- **Debug Logging**: Environment variable DEBUG=1 enables detailed event and processing logs

### Added - Diagnostic Tools (4)

- **midi_diagnostic**: Visualize all incoming MIDI events with formatted display
- **led_diagnostic**: Test RGB LED functionality and HID connection
- **led_tester**: Interactive LED scheme testing utility
- **pad_mapper**: Utility for mapping physical pad positions to MIDI notes

### Added - Documentation

- README.md with quick start guide and feature overview
- CLAUDE.md with comprehensive project instructions and architecture
- LED_FEEDBACK.md with LED system documentation
- CODE_OF_CONDUCT.md (Contributor Covenant v2.1)
- CONTRIBUTING.md with contribution guidelines
- GOVERNANCE.md defining project structure and decision-making
- MAINTAINERS.md listing current maintainers
- ROADMAP.md outlining project vision and development phases
- SECURITY.md with vulnerability reporting process
- Example config.toml with common mapping patterns

### Added - Developer Infrastructure

- GitHub Actions CI/CD pipeline (build, test, clippy, format checks)
- Issue templates (bug report, feature request, device support, documentation)
- Pull request template with comprehensive checklist
- SUPPORT.md documenting support channels
- Pre-commit hook setup for code quality
- VS Code configuration (.vscode/settings.json, launch.json, tasks.json)
- Build scripts (scripts/build.sh, test.sh, dev-setup.sh, clean.sh)
- .editorconfig for cross-editor consistency
- rust-toolchain.toml pinning Rust version

### Added - Legal & Compliance

- MIT License with copyright notice
- Copyright headers in all source files
- NOTICE file with third-party attributions
- THIRD_PARTY_LICENSES.md documenting all dependency licenses
- Trademark disclaimer for Native Instruments references
- SPDX license identifier in Cargo.toml

### Performance

- Response latency: <1ms typical for MIDI event processing
- Memory footprint: 5-10MB steady state
- CPU usage: <1% idle, <5% during active use
- Binary size: 3-5MB (release build with LTO and stripping)

### Platform Support

- macOS 11+ (Big Sur and later)
- Apple Silicon (ARM64) and Intel (x86_64) architectures
- Requires Input Monitoring permission for HID device access

### Device Compatibility

- **Fully Supported**: Native Instruments Maschine Mikro MK3 (RGB LEDs, HID access, profile support)
- **MIDI-Only Support**: Any USB MIDI controller with basic LED feedback via MIDI Note messages
- **Profile Support**: .ncmm3 files from Native Instruments Controller Editor

### Known Limitations

- macOS only (Linux and Windows support planned for Phase 4)
- Single device support (multi-device planned for Phase 4)
- No GUI for configuration (Tauri UI planned for Phase 3)
- Config changes require restart (hot reload planned for Phase 2)
- No virtual MIDI output (planned for Phase 4)

### Dependencies

Major external crates:
- midir 0.9 - Cross-platform MIDI I/O
- enigo 0.2 - Keyboard/mouse input simulation
- hidapi 2.6 - HID device access with macOS shared device support
- serde 1.0 + toml 0.8 - Configuration parsing
- quick-xml 0.36 - XML profile parsing (.ncmm3 files)
- crossbeam-channel 0.5 - Lock-free event channels
- colored 2.1 - Terminal output formatting
- ctrlc 3.4 - Graceful shutdown handling

All dependencies use MIT, Apache-2.0, or BSD-compatible licenses.

### Migration Path

This v0.1.0-monolithic release preserves the working single-binary implementation before architectural migration to workspace structure (Phase 2-4). Future versions will maintain backward compatibility with existing config.toml files.

### Contributors

- Christopher Joseph (@christopherjoseph) - Project Lead & Creator

### Release Artifacts

- conductor-v0.1.0-macos-arm64.tar.gz (Apple Silicon)
- conductor-v0.1.0-macos-x86_64.tar.gz (Intel)
- checksums.txt (SHA256)

---

## Version History

- **v3.0.0** (2025-11-21): Multi-protocol input with game controller support 🎮
- **v2.7.0** (2025-11-19): Plugin security & verification ✨
- **v2.3.0** (2025-01-18): Plugin architecture
- **v2.2.0** (2025-11-18): Velocity curves & conditionals
- **v2.1.0** (2025-11-17): Virtual MIDI output
- **v2.0.0** (2025-11-14): Tauri GUI & visual config
- **v1.0.0** (2025-01-13): Production daemon with hot-reload
- **v0.2.0** (2025-11-12): Workspace architecture migration
- **v0.1.0-monolithic** (2025-11-11): Initial public release with 26 features
- **Unreleased**: Next version in development

---

## Changelog Guidelines

This changelog follows [Keep a Changelog](https://keepachangelog.com/) format:

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be-removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security vulnerability fixes

Version numbers follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes to config format or public API
- **MINOR**: New features, backward-compatible
- **PATCH**: Bug fixes, performance improvements

[Unreleased]: https://github.com/amiable-dev/conductor/compare/v3.0.0...HEAD
[3.0.0]: https://github.com/amiable-dev/conductor/releases/tag/v3.0.0
[2.7.0]: https://github.com/amiable-dev/conductor/releases/tag/v2.7.0
[2.3.0]: https://github.com/amiable-dev/conductor/releases/tag/v2.3.0
[2.2.0]: https://github.com/amiable-dev/conductor/releases/tag/v2.2.0
[2.1.0]: https://github.com/amiable-dev/conductor/releases/tag/v2.1.0
[2.0.0]: https://github.com/amiable-dev/conductor/releases/tag/v2.0.0
[1.0.0]: https://github.com/amiable-dev/conductor/releases/tag/v1.0.0
[0.2.0]: https://github.com/amiable-dev/conductor/releases/tag/v0.2.0
[0.1.0-monolithic]: https://github.com/amiable-dev/conductor/releases/tag/v0.1.0-monolithic
