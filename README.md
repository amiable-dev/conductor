# Conductor

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://github.com/amiable-dev/conductor/workflows/CI/badge.svg)](https://github.com/amiable-dev/conductor/actions)
[![codecov](https://codecov.io/gh/amiable-dev/conductor/branch/main/graph/badge.svg)](https://codecov.io/gh/amiable-dev/conductor)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://amiable-dev.github.io/conductor/)

Transform MIDI controllers and game controllers into advanced macro pads with multi-protocol input, velocity sensitivity, LED feedback, daemon architecture, and visual configuration GUI.

**v3.0.0**: Multi-protocol input support - MIDI Controllers + Game Controllers (HID)
**v2.0.0**: Full-featured Tauri GUI with MIDI Learn, per-app profiles, device templates, and live event console
**v1.0.0**: Production-ready daemon with 0-10ms config reloads, IPC control, and auto-start support

![Conductor Demo](docs/images/hero-demo.gif)
*Velocity-sensitive RGB LED feedback on Native Instruments Maschine Mikro MK3*

## Features

### Multi-Protocol Input (v3.0.0 NEW!)

Conductor now supports multiple input protocols through a unified input system:

#### MIDI Controllers (v1.0+)
- **Full MIDI Support** - All MIDI message types (Note, CC, Aftertouch, Pitch Bend)
- **RGB LED Feedback** - Full HID-based LED control for supported devices
- **Velocity Sensitivity** - Different actions for soft/medium/hard presses
- **Device Templates** - 6 built-in templates for popular MIDI controllers

#### Game Controllers (HID) (v3.0+)
- **Gamepads**: Xbox (360, One, Series X|S), PlayStation (DualShock 4, DualSense), Switch Pro Controller
- **Joysticks**: Flight sticks, arcade sticks (any SDL2-compatible device)
- **Racing Wheels**: Logitech, Thrustmaster, any SDL2-compatible racing wheel
- **HOTAS**: Hands On Throttle And Stick systems
- **Custom Controllers**: Any SDL2-compatible HID device
- **Official Templates**: 3 built-in templates (Xbox, PlayStation, Switch Pro)
- **Analog Support**: Triggers, analog sticks with threshold detection
- **Button Chords**: Multi-button combinations for complex macros

#### Unified Input System
- **Hybrid Workflows** - Use MIDI controller + gamepad simultaneously
- **Hot-Plug Detection** - Automatic device connection/disconnection handling
- **MIDI Learn Mode** - One-click auto-detection works with both MIDI and gamepad inputs
- **Protocol-Agnostic** - Same event processing for all input types
- **Non-Overlapping IDs** - MIDI (0-127), Gamepad (128-255), no conflicts

### Visual Configuration (v2.0.0)
- **Tauri GUI** - Modern desktop application for visual configuration
- **MIDI Learn Mode** - One-click auto-detection of MIDI and gamepad inputs
- **Device Templates** - 9 built-in templates (6 MIDI + 3 gamepad)
- **Per-App Profiles** - Automatic profile switching based on active application
- **Live Event Console** - Real-time input event monitoring and debugging
- **Settings Panel** - Configure auto-start, theme, and preferences

### Core Features
- **Background Daemon** - Runs as a system service with auto-start
- **Hot-Reload** - Configuration changes detected and applied in 0-10ms
- **IPC Control** - Control daemon via `conductorctl` CLI or GUI
- **Multi-mode operation** - Switch between different mapping sets
- **Configurable mappings** - Visual editor or TOML-based configuration
- **Ultra-low latency** - Sub-millisecond input response, <20ms config reload
- **Cross-platform** - Works on macOS and Linux (systemd/launchd)

### Enhanced Event Detection
- **Velocity Sensitivity** - Different actions for soft/medium/hard presses
- **Long Press Detection** - Hold actions with configurable thresholds
- **Double-Tap Detection** - Quick double-tap triggers
- **Chord Detection** - Multiple buttons/pads pressed simultaneously
- **Encoder Direction** - Clockwise/counter-clockwise detection
- **Analog Sticks** - Directional detection with dead zones
- **Analog Triggers** - Threshold-based trigger actions
- **Aftertouch Support** - Pressure-sensitive actions (MIDI)
- **Pitch Bend Support** - Touch strip integration (MIDI)

### LED Feedback System
- **Visual Feedback** - Real-time LED feedback on supported MIDI devices
- **Multiple Schemes** - Rainbow, pulse, breathing, reactive, and more
- **Mode Indication** - Color-coded modes for easy identification
- **Velocity Visualization** - LED brightness matches pad velocity
- **HID Support** - Full RGB control for Maschine Mikro MK3
- **MIDI LED** - Basic feedback for standard MIDI devices

> 📖 **See [LED_FEEDBACK.md](LED_FEEDBACK.md) for complete LED system documentation**

## Hardware Compatibility

### MIDI Controllers

| Device | Status | LED Feedback | Notes |
|--------|--------|--------------|-------|
| Native Instruments Maschine Mikro MK3 | ✅ Full Support | RGB (HID) | Recommended |
| Generic MIDI Controllers | ✅ Supported | Basic (MIDI) | Most features work |
| Akai APC Mini | ⚠️ Untested | Basic (MIDI) | Should work |
| Novation Launchpad | ⚠️ Untested | Basic (MIDI) | Should work |

### Game Controllers (v3.0+)

#### Gamepads (Official Templates Available)

| Controller | Template | Status | Platform Compatibility |
|-----------|----------|--------|----------------------|
| Xbox Controller (360, One, Series X\|S) | ✅ Official | ✅ Full Support | Windows, macOS, Linux |
| PlayStation Controller (DualShock 4, DualSense) | ✅ Official | ✅ Full Support | Windows, macOS, Linux |
| Nintendo Switch Pro Controller | ✅ Official | ✅ Full Support | Windows, macOS, Linux |
| Generic SDL2-Compatible Gamepads | ⚠️ Manual Config | ✅ Supported | Windows, macOS, Linux |

#### Joysticks & Flight Controllers (Manual Configuration)

| Device Type | Template | Status | Notes |
|------------|----------|--------|-------|
| Flight Sticks | ⚠️ Coming Soon | ✅ Supported | Any SDL2-compatible device works |
| Arcade Sticks | ⚠️ Coming Soon | ✅ Supported | Button mapping via MIDI Learn |
| HOTAS Systems | ⚠️ Coming Soon | ✅ Supported | Full analog axis support |

#### Racing Wheels (Manual Configuration)

| Device Type | Template | Status | Notes |
|------------|----------|--------|-------|
| Logitech Wheels | ⚠️ Coming Soon | ✅ Supported | Steering, pedals, buttons |
| Thrustmaster Wheels | ⚠️ Coming Soon | ✅ Supported | All SDL2 axes supported |
| Generic Racing Wheels | ⚠️ Coming Soon | ✅ Supported | Any SDL2-compatible wheel |

**Want to add support for your device?** See [CONTRIBUTING.md](CONTRIBUTING.md#device-support)

## Installation

### From Binary (Recommended)

Download the latest release for your platform:
- [macOS (Intel)](https://github.com/amiable-dev/conductor/releases/latest/download/conductor-macos-intel)
- [macOS (Apple Silicon)](https://github.com/amiable-dev/conductor/releases/latest/download/conductor-macos-arm)
- [Linux (x86_64)](https://github.com/amiable-dev/conductor/releases/latest/download/conductor-linux-x86_64)

```bash
# Install binaries
sudo install -m 755 conductor /usr/local/bin/
sudo install -m 755 conductorctl /usr/local/bin/

# macOS: Install as LaunchAgent
launchctl load ~/Library/LaunchAgents/com.amiable.conductor.plist

# Linux: Install as systemd service
systemctl --user enable conductor
systemctl --user start conductor
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for complete installation and service setup guides.

### From Source

```bash
# Clone the repository
git clone https://github.com/amiable-dev/conductor.git
cd conductor

# Build the workspace (all 3 packages)
cargo build --release --workspace

# Install binaries
sudo install -m 755 target/release/conductor /usr/local/bin/
sudo install -m 755 target/release/conductorctl /usr/local/bin/

# Install man pages
sudo mkdir -p /usr/local/share/man/man1
sudo install -m 644 conductor-daemon/docs/*.1 /usr/local/share/man/man1/
```

**Workspace Structure** (v1.0.0):

Conductor uses a modular 3-package Cargo workspace:

```
conductor/
├── conductor-core/       # Pure Rust engine library
│   ├── Public API for embedding (30+ types)
│   ├── Zero UI dependencies
│   └── Event processing, mapping, actions
├── conductor-daemon/     # Background daemon + diagnostic tools
│   ├── Main daemon binary (conductor)
│   ├── CLI control tool (conductorctl)
│   └── 6 diagnostic binaries
└── conductor/            # Backward compatibility layer
    └── Re-exports conductor-core (v0.1.0 tests only)
```

**Package Guide**:
- **Use conductor-core** when embedding Conductor as a library
- **Use conductor-daemon** for standalone CLI/daemon usage
- **Use conductor (root)** only for v0.1.0 backward compatibility

**Public API Example**:
```rust
use conductor_core::{Config, MappingEngine, EventProcessor, ActionExecutor};

let config = Config::load("config.toml")?;
let mut engine = MappingEngine::new();
// Process MIDI events, map to actions, execute...
```

**Build Commands**:
```bash
# Build entire workspace (all 3 packages)
cargo build --workspace

# Build specific package
cargo build -p conductor-core
cargo build -p conductor-daemon

# Test workspace
cargo test --workspace
```

**Requirements:**
- Rust 1.70+ ([Install via rustup](https://rustup.rs/))
- macOS 11+ or Linux with systemd

## Quick Start

### Quick Start - MIDI Controllers

#### Daemon Mode (Recommended)

1. **Install binaries** (see Installation above)
2. **Create config** at `~/.config/conductor/config.toml`
3. **Start daemon**:
   ```bash
   # macOS
   launchctl load ~/Library/LaunchAgents/com.amiable.conductor.plist

   # Linux
   systemctl --user start conductor
   ```
4. **Control daemon**:
   ```bash
   conductorctl status   # Check daemon status
   conductorctl reload   # Reload configuration
   conductorctl ping     # Test connectivity
   ```
5. **Edit config** - Changes are auto-detected and reloaded in <10ms!

#### Manual Mode (Development/Testing)

1. **Connect your MIDI controller** (e.g., Native Instruments Maschine Mikro MK3)
2. **Install necessary drivers** (Native Instruments Controller Editor for NI devices)
3. **Run directly**:
   ```bash
   conductor --config config.toml --log-level debug
   ```
4. **Press pads** to trigger macros!

### Quick Start - Game Controllers (Template-Based)

For gamepads with official templates (Xbox, PlayStation, Switch Pro):

1. **Open Conductor GUI** (or use CLI with template config)
2. **Select Device Template**:
   - Navigate to "Device Templates" section
   - Filter by category: "Gamepad Controllers"
   - Choose your controller:
     - **Xbox Controller** (Xbox 360, One, Series X|S)
     - **PlayStation Controller** (DualShock 4, DualSense)
     - **Switch Pro Controller**
3. **Generate Configuration**:
   - Click "Create from Template"
   - Templates include pre-configured modes:
     - **Desktop Mode**: Navigation, window management, shortcuts
     - **Media Mode**: Playback control, volume adjustment
     - **Additional Modes**: Browser (Switch), Gaming (PlayStation)
4. **Connect Gamepad** and press buttons to trigger actions
5. **Customize** via MIDI Learn or manual TOML editing

**Pre-configured Features**:
- Face buttons → Enter, Escape, Copy, Paste
- D-Pad → Arrow keys
- Shoulder buttons → Tab navigation
- Guide/Home button → Spotlight search
- Analog triggers → Volume control
- Button chords → Mode switching

### Quick Start - Game Controllers (Manual Configuration)

For joysticks, racing wheels, HOTAS, or custom controllers:

1. **Create Base Configuration**:
   ```bash
   # Start with minimal config
   cp config/examples/gamepad-basic.toml ~/.config/conductor/config.toml
   ```

2. **Use MIDI Learn to Map Buttons**:
   - Open Conductor GUI
   - Enable "MIDI Learn" mode
   - Click on a mapping you want to configure
   - Press the button/axis on your controller
   - Pattern detection will auto-suggest GamepadButton or GamepadAnalogStick
   - Save the mapping

3. **Manual TOML Configuration Example**:
   ```toml
   [[modes]]
   name = "Flight Mode"
   color = "blue"

   [[modes.mappings]]
   description = "Fire primary weapon"
   [modes.mappings.trigger]
   type = "GamepadButton"
   button = 128  # First trigger button
   [modes.mappings.action]
   type = "Keystroke"
   keys = "space"

   [[modes.mappings]]
   description = "Pitch control via stick"
   [modes.mappings.trigger]
   type = "GamepadAnalogStick"
   axis = 129  # Y-axis
   direction = "Up"
   threshold = 0.5
   [modes.mappings.action]
   type = "Keystroke"
   keys = "w"
   ```

4. **Test Your Configuration**:
   ```bash
   conductorctl reload
   # Move sticks, press buttons to trigger actions
   ```

**Button ID Reference** (for manual config):
- **Face Buttons**: 128-131
- **D-Pad**: 132-135
- **Shoulders**: 136-137 (bumpers), 143-144 (triggers)
- **Stick Clicks**: 138-139
- **Menu/System**: 140-142
- **Analog Axes**: 128-133 (sticks + triggers)

> 📖 **See technical documentation for complete button/axis mapping reference**

## Configuration

Edit `config.toml` to customize your mappings. The enhanced configuration supports:

### Basic Note Trigger (MIDI)
```toml
[[modes.mappings]]
description = "Spotlight Search"
[modes.mappings.trigger]
type = "Note"
note = 12
velocity_min = 1  # Optional
velocity_max = 127  # Optional
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]
```

### Gamepad Button Trigger (v3.0)
```toml
[[modes.mappings]]
description = "Gamepad A Button"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # South button (A/Cross/B)
[modes.mappings.action]
type = "Keystroke"
keys = "space"
```

### Gamepad Button Chord (v3.0)
```toml
[[modes.mappings]]
description = "Quick Save (LB + A)"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 128]  # LB + A
max_interval_ms = 100
[modes.mappings.action]
type = "Keystroke"
keys = "s"
modifiers = ["ctrl"]
```

### Gamepad Analog Stick (v3.0)
```toml
[[modes.mappings]]
description = "Left stick up → W key"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 129  # Left stick Y-axis
direction = "Up"
threshold = 0.5
[modes.mappings.action]
type = "Keystroke"
keys = "w"
```

### Gamepad Analog Trigger (v3.0)
```toml
[[modes.mappings]]
description = "Right trigger → Volume up"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger analog axis
threshold = 0.3
[modes.mappings.action]
type = "VolumeControl"
action = "Up"
```

### Velocity-Sensitive Actions (MIDI)
```toml
# Soft press
[[modes.mappings]]
description = "Volume Down (soft)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 1
max_velocity = 40
[modes.mappings.action]
type = "VolumeControl"
action = "Down"

# Hard press
[[modes.mappings]]
description = "Volume Up (hard)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 13
min_velocity = 80
max_velocity = 127
[modes.mappings.action]
type = "VolumeControl"
action = "Up"
```

### Long Press
```toml
[[modes.mappings]]
description = "Quit App (long press)"
[modes.mappings.trigger]
type = "LongPress"
note = 4
min_duration_ms = 1500
[modes.mappings.action]
type = "Keystroke"
keys = "q"
modifiers = ["cmd"]
```

### Double-Tap
```toml
[[modes.mappings]]
description = "Fullscreen (double tap)"
[modes.mappings.trigger]
type = "DoubleTap"
note = 16
max_interval_ms = 300
[modes.mappings.action]
type = "Keystroke"
keys = "f"
modifiers = ["ctrl", "cmd"]
```

### Chord Detection (MIDI)
```toml
[[modes.mappings]]
description = "Force Quit (chord)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [8, 12]  # Both pads must be pressed
max_interval_ms = 100
[modes.mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = ["cmd", "option"]
```

### Encoder Actions (MIDI)
```toml
[[global_mappings]]
description = "Volume Up"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 2
direction = "Clockwise"
[global_mappings.action]
type = "VolumeControl"
action = "Up"
```

## Daemon Control

### conductorctl CLI

Control the daemon from the command line:

```bash
# Check daemon status
conductorctl status

# Reload configuration
conductorctl reload

# Test connectivity
conductorctl ping

# Stop daemon
conductorctl stop

# Validate config before applying
conductorctl validate --config new-config.toml

# JSON output for scripting
conductorctl --json status | jq .data.uptime_secs
```

### Performance Monitoring

```bash
# View reload performance
conductorctl status | grep -A5 "Reload Performance"

# Output includes:
# - Last reload time (ms)
# - Average reload time
# - Fastest/slowest reloads
# - Performance grade (A-F)
```

See `man conductorctl` for full command reference.

## Diagnostic Tools

### MIDI Diagnostic Tool
Visualize all MIDI events from your controller:
```bash
midi_diagnostic 2  # If installed
# Or: cargo run --bin midi_diagnostic 2
```

Features:
- Real-time event visualization
- Velocity bars
- Hold duration tracking
- Beautiful colored output

### Test MIDI Ports
List all available MIDI devices:
```bash
test_midi  # If installed
# Or: cargo run --bin test_midi
```

## Action Types

### Basic Actions
- **Keystroke** - Simulate keyboard shortcuts
- **Text** - Type text strings
- **Launch** - Open applications
- **Shell** - Execute shell commands
- **Delay** - Add delays between actions
- **MouseClick** - Simulate mouse clicks

### Advanced Actions
- **VolumeControl** - System volume control (Up/Down/Mute/Set)
- **ModeChange** - Switch between mapping modes
- **Sequence** - Chain multiple actions
- **Repeat** - Repeat an action multiple times
- **Conditional** - Execute based on conditions

## Modes

The system supports multiple modes, each with its own set of mappings:

1. **Default Mode** - General productivity shortcuts
2. **Development Mode** - Git commands, build tools
3. **Media Mode** - Music and video controls
4. **Custom Modes** - Create your own!

Switch modes using:
- Encoder rotation (MIDI)
- Specific button/pad combinations
- CC messages (MIDI)
- Button chords (Gamepad)

## Troubleshooting

### MIDI Device Not Found
1. Check if device is connected: `system_profiler SPUSBDataType | grep -i mikro`
2. Install necessary drivers (e.g., Native Instruments Controller Editor)
3. Check Audio MIDI Setup: `open -a "Audio MIDI Setup"`

### Gamepad Not Detected (v3.0)
1. Check if gamepad is connected and recognized by OS
2. Test with system gamepad settings or Steam Big Picture
3. Ensure SDL2-compatible drivers are installed
4. Check debug output: `conductor --log-level debug`
5. Verify gamepad shows in system controller list

### High Latency
- Build in release mode: `cargo build --release`
- Close unnecessary applications
- Check CPU usage

### Events Not Triggering
- Run diagnostic tool to verify input events
- Check config.toml for correct button/note numbers
- Verify velocity/duration thresholds
- For gamepads: Check button ID mapping (MIDI Learn recommended)

## Advanced Configuration

### Timing Settings
```toml
[advanced_settings]
chord_timeout_ms = 50        # Max time between notes for chord
double_tap_timeout_ms = 300  # Max time between taps
hold_threshold_ms = 2000     # Time before hold is detected
```

### Conditional Actions
```toml
[[modes.mappings]]
description = "Context-aware action"
[modes.mappings.trigger]
type = "Note"
note = 20
[modes.mappings.action]
type = "Conditional"
[modes.mappings.action.condition]
type = "AppRunning"
app_name = "Terminal"
[modes.mappings.action.then_action]
type = "Keystroke"
keys = "c"
modifiers = ["ctrl"]
[modes.mappings.action.else_action]
type = "Launch"
app = "Terminal"
```

## Development

### Project Structure
```
conductor/
├── src/
│   ├── main.rs           # Main application
│   ├── config.rs         # Configuration structures
│   ├── actions.rs        # Action execution
│   ├── mappings.rs       # Trigger mapping engine
│   ├── event_processor.rs # Enhanced event processing
│   └── bin/
│       ├── test_midi.rs  # MIDI port tester
│       └── midi_diagnostic.rs # Diagnostic tool
├── config.toml           # User configuration
└── Cargo.toml           # Dependencies
```

### Adding New Trigger Types
1. Add to `Trigger` enum in `config.rs`
2. Add to `ProcessedEvent` enum in `event_processor.rs`
3. Update `EventProcessor::process()` to detect the trigger
4. Update `MappingEngine::trigger_matches_processed()` to handle matching

### Adding New Action Types
1. Add to `ActionConfig` enum in `config.rs`
2. Add to `Action` enum in `actions.rs`
3. Update `ActionExecutor::execute()` to handle the action

## Performance

- **Input Event Latency**: < 1ms typical
- **Config Reload Time**: 0-10ms typical (Grade A: <20ms target)
- **Startup Time**: < 500ms
- **Memory Usage**: 5-10MB
- **CPU Usage**: < 1% idle, < 5% active
- **Binary Size**: ~3-5MB (release build with LTO)

Run benchmarks:
```bash
cargo bench --package conductor-daemon
```

Typical benchmark results (Apple M1):
- 2 modes, 10 mappings: 0-2ms reload
- 5 modes, 50 mappings: 2-5ms reload
- 10 modes, 100 mappings: 5-8ms reload

All achieve **Grade A** performance (<20ms).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- How to report bugs
- How to propose features
- Development setup guide
- Coding standards
- Pull request process

Check out [good first issues](https://github.com/amiable-dev/conductor/labels/good-first-issue) to get started.

## Documentation

- **[Deployment Guide](DEPLOYMENT.md)** - Complete installation and service setup
- **[Full Documentation](https://amiable-dev.github.io/conductor/)** - Complete user guide
- **[LED Feedback System](LED_FEEDBACK.md)** - LED control documentation
- **[Configuration Reference](docs/configuration.md)** - TOML configuration guide
- **[API Documentation](https://docs.rs/conductor)** - Rust API docs
- **Man Pages**: `man conductor`, `man conductorctl`

## Community & Support

- **[Discussions](https://github.com/amiable-dev/conductor/discussions)** - Ask questions, share configs
- **[Issues](https://github.com/amiable-dev/conductor/issues)** - Bug reports, feature requests
- **[Security](SECURITY.md)** - Security vulnerability reporting

## Roadmap

### ✅ Phase 1 - v0.2.0 (Complete)
- Workspace architecture (conductor-core, conductor-daemon, conductor)
- Pure Rust engine library with zero UI dependencies
- 339 tests, all passing

### ✅ Phase 2 - v1.0.0 (Complete)
- Background daemon with IPC server
- Config hot-reload with 0-10ms latency
- CLI control tool (conductorctl)
- systemd/launchd integration
- Comprehensive documentation and deployment guides

### ✅ Phase 3 - v2.0.0 (Complete)
- Tauri-based visual configurator
- MIDI Learn mode (click → press → auto-map)
- Per-app profiles with frontmost app detection
- Device templates for popular controllers
- Live event console

### ✅ Phase 4 - v3.0.0 (Complete)
- Multi-protocol input system (MIDI + HID/Gamepad)
- Unified InputEvent abstraction
- Gamepad device templates (Xbox, PlayStation, Switch Pro)
- MIDI Learn support for gamepad buttons
- Hot-plug detection for game controllers

### 🚀 Phase 5 - v4.0.0 (Future)
- OSC protocol support
- Keyboard intercept (system-wide hotkeys)
- Custom USB device support
- Plugin system for community extensions

### 🔮 Phase 6 - v5.0.0 (Vision)
- Virtual MIDI output for DAW integration
- Profile sharing/export marketplace
- WebSocket API for remote control
- AI-powered natural language configuration
- Workflow pattern recognition

See [.research/](https://github.com/amiable-dev/conductor/tree/main/.research) for detailed implementation proposals.

## License

Conductor is licensed under the [MIT License](LICENSE).

Copyright (c) 2025 Amiable

## Credits

Built with:
- [midir](https://github.com/Boddlnagg/midir) - MIDI I/O
- [gilrs](https://gitlab.com/gilrs-project/gilrs) - Game controller input (v3.0+)
- [enigo](https://github.com/enigo-rs/enigo) - Input simulation
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [serde](https://serde.rs/) - Serialization
