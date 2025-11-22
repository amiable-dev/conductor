# Turn Any Controller Into a Supercharged Macro Pad

**What if every button on your Xbox controller could be a keyboard shortcut? What if tapping a MIDI pad softly did one thing, and hitting it hard did something else?**

Conductor is the missing link between your game controllers, MIDI devices, and your computer. It's open-source, blazingly fast (<1ms latency), and lets you create workflows that expensive macro pads can't touch.

**v3.0 is here**: MIDI + gamepads in one workflow. Use your $30 Xbox controller as a 15-button macro pad, or combine a MIDI controller with a racing wheel for creative hybrid setups.

✨ **The Killer Feature**: Velocity sensitivity. Press soft = copy, press hard = paste. One pad, multiple actions. Mind. Blown.

## What's Possible with Conductor

Transform your creative workflow in ways traditional macro tools can't:

### 🎵 Music Production
- **Velocity-sensitive recording**: Soft press = loop record, hard press = punch record
- **One pad, three actions**: Turn a $30 MIDI controller into a pressure-sensitive control surface
- **Gamepad as DAW navigator**: Use your Xbox controller for timeline navigation while keeping hands on MIDI keyboard

### 💻 Software Development
- **Git workflow on a pad**: Soft = status, medium = commit, hard = commit+push
- **Velocity-based scrolling**: Soft tap = 1 line, hard hit = jump 10 lines
- **Build triggers**: Press A button to build, hold B to run tests

### 🎮 Content Creation
- **Racing wheel for video editing**: Use pedals for effects control, wheel for timeline scrubbing
- **Gamepad for streaming**: 15+ shortcuts at your fingertips without touching keyboard
- **Form automation**: Fill complex forms with one button press

### ⚡ Power Users
- **Repurpose existing hardware**: That dusty Xbox controller? It's now a 15-button macro pad
- **Hot-reload everything**: Change configs on-the-fly, no restart needed (0-10ms reload)
- **Context-aware mappings**: Different actions based on active app, time of day, or system state

## Key Features

### Multi-Protocol Input (v3.0+)

**MIDI Controllers**:
- Full MIDI controller support with RGB LED feedback
- Native Instruments Maschine, Launchpad, APC Mini, and more
- SysEx support for advanced LED control

**HID Gamepads** (NEW!):
- Xbox 360/One/Series controllers
- PlayStation DualShock 4/DualSense controllers
- Nintendo Switch Pro Controller
- Button chords, analog sticks, triggers with full velocity sensitivity
- See the [Gamepad Support Guide](guides/gamepad-support.md) for details

**Coming Soon**:
- OSC (Open Sound Control) for networked devices
- Custom USB HID devices
- Keyboard/mouse intercept for hybrid workflows

### Core Capabilities (v2.0.0)

**Event Detection**:
- **4 Core Triggers**: Note, Velocity Range, Encoder, Control Change
- **5 Advanced Triggers**: Long Press, Double-Tap, Chord, Aftertouch, Pitch Bend
- **10 Action Types**: Keystroke, Text, Launch, Shell, Volume Control, and more

**Visual Interface (NEW!)**:
- **Tauri GUI**: Modern desktop app for visual configuration
- **MIDI Learn**: One-click auto-detection of MIDI inputs
- **Live Preview**: Real-time event monitoring and testing
- **Device Templates**: 6 built-in templates for popular controllers

**Daemon Infrastructure (NEW!)**:
- **Background Service**: Runs as system service with auto-start
- **Hot-Reload**: Config changes applied in 0-10ms without restart
- **IPC Control**: Control daemon via CLI or GUI
- **Per-App Profiles**: Automatic profile switching

**LED & Feedback**:
- **10 LED Schemes**: Reactive, Rainbow, Pulse, Breathing, and custom patterns
- **Multi-Mode System**: Switch between mapping sets on the fly
- **Device Profile Support**: Load Native Instruments Controller Editor configurations

### Performance

- Response latency: **<1ms** typical
- Memory footprint: **5-10MB**
- CPU usage: **<1%** idle, **<5%** active
- Binary size: **3-5MB** (optimized)

## Who is Conductor For?

### 🎹 Music Producers & Live Performers

**You have**: A MIDI controller (Maschine, Launchpad, APC Mini) that's not fully integrated into your workflow, or you're manually switching modes constantly during recording sessions.

**Conductor gives you**: Velocity-sensitive DAW control, per-app profile switching, RGB LED feedback that shows your current state, and hot-reload that lets you tweak mappings mid-session.

**Example workflow**:
> "Soft press on pad 1 = loop record, medium press = one-shot sample, hard press = toggle reverb. One pad, three actions. I don't switch modes anymore—velocity does it for me."

**[See Music Production Examples →](examples/music-production.md)**

---

### 💻 Software Developers & DevOps Engineers

**You have**: Too many terminal windows open, countless keyboard shortcuts to remember, and repetitive git workflows that waste 20-30 minutes daily.

**Conductor gives you**: One-button git operations, build/test triggers mapped to gamepad buttons, and context-aware shortcuts that change based on your active IDE.

**Example workflow**:
> "Press gamepad button A = git status. Hold for 2 seconds = commit and push with auto-generated message. My PlayStation controller saves me 30 minutes every day."

**[See Developer Workflows →](examples/developer-workflows.md)**

---

### 🎮 Streamers & Content Creators

**You have**: A $150-300 Stream Deck on your wishlist, or you're using keyboard shortcuts that break your flow during streams.

**Conductor gives you**: Professional stream controls for free using your existing Xbox/PlayStation controller, with velocity-sensitive audio fading and button chords for complex actions.

**Example workflow**:
> "Xbox A button = switch scene, B = mute mic, triggers = analog audio fade in/out. I saved $300 by repurposing my gamepad instead of buying a Stream Deck."

**[See Streaming Setup Guide →](examples/streaming.md)**

---

### 🎬 Video Editors & Post-Production

**You have**: Keyboard-heavy editing workflows that strain your wrists, and you wish timeline scrubbing felt more natural.

**Conductor gives you**: Analog pedal control for timeline speed and zoom, ergonomic button layouts on MIDI controllers or racing wheels, and hands-free transport control.

**Example workflow**:
> "My racing wheel's gas pedal controls timeline playback speed (0-200%). Brake pedal = zoom level. It sounds crazy, but it's incredibly intuitive and ergonomic."

**[See Video Editing Examples →](examples/video-editing.md)**

---

### ⚡ Power Users & Automation Enthusiasts

**You have**: Repetitive tasks you're sick of doing manually, forms that need filling out dozens of times daily, or app-specific shortcuts you can never remember.

**Conductor gives you**: Context-aware shortcuts that change based on active application, form automation that fills entire forms with one button press, and MIDI Learn that makes configuration visual and instant.

**Example workflow**:
> "I mapped long-press to 'fill entire web form with saved data.' Double-tap = refresh page. Velocity determines scroll speed. Saved hours every week."

**[See Automation Examples →](examples/automation.md)**

---

### 🕹️ Gamers Repurposing Controllers

**You have**: A dusty Xbox controller, old racing wheel, or HOTAS setup that you barely use anymore.

**Conductor gives you**: A second life for your gaming hardware as professional productivity tools. That $150 racing wheel becomes a $300 video editing controller.

**Example workflow**:
> "My Thrustmaster HOTAS from Star Citizen now controls my entire dev environment. 20+ shortcuts without touching the keyboard. Plus, it's just fun to use."

**[See Gaming Hardware Repurposing →](guides/gamepad-support.md)**

---

## Not Sure Where to Start?

**[Try the Quick Start Guide →](getting-started/quick-start.md)** - Set up your first mapping in 5 minutes

**[Browse Device Templates →](guides/device-templates.md)** - Pre-built configs for popular controllers

**[Explore Use Cases →](inspiration/success-stories.md)** - See how others use Conductor

## Why Conductor?

Unlike existing MIDI mapping tools, Conductor provides:

- **Multi-Protocol Support**: Use MIDI controllers AND gamepads in the same workflow (v3.0+)
- **Advanced Timing**: Long press, double-tap, chord detection out of the box
- **Velocity Sensitivity**: Different actions for soft/medium/hard pad hits
- **Full RGB Feedback**: Not just on/off LEDs, but animated schemes and reactive color
- **Modern Architecture**: Fast Rust core, hot-reload config, cross-platform
- **Open Source**: Fully customizable, extensible, community-driven

## Quick Examples

### MIDI Controller
```toml
# Press pad lightly for copy, hard for paste
[[modes.mappings]]
trigger = { type = "VelocityRange", note = 36 }
soft = { action = { type = "Keystroke", key = "C", modifiers = ["Cmd"] } }
hard = { action = { type = "Keystroke", key = "V", modifiers = ["Cmd"] } }

# Hold for 2 seconds to open terminal
[[modes.mappings]]
trigger = { type = "LongPress", note = 37, duration_ms = 2000 }
action = { type = "Launch", path = "/Applications/Utilities/Terminal.app" }
```

### Gamepad (v3.0+)
```toml
# Press A button to build your project
[[modes.mappings]]
trigger = { type = "GamepadButton", button = "South" }  # A on Xbox, X on PlayStation
action = { type = "Shell", command = "cargo build" }

# Hold B button for 1 second to run tests
[[modes.mappings]]
trigger = { type = "GamepadButton", button = "East", hold_ms = 1000 }
action = { type = "Shell", command = "cargo test" }
```

## Platform Support

- **macOS**: Full support (11+ Big Sur, Apple Silicon + Intel)
- **Linux**: Planned for Phase 4 (Q4 2025)
- **Windows**: Planned for Phase 4 (Q4 2025)

## Device Compatibility

### MIDI Controllers
- **Fully Supported**: Native Instruments Maschine Mikro MK3 (RGB LEDs, HID access)
- **MIDI-Only Support**: Any USB MIDI controller with basic LED feedback
- **Coming Soon**: Launchpad, APC Mini, Korg nanoKontrol, and more

### HID Gamepads (v3.0+)
- **Xbox**: Xbox 360, Xbox One, Xbox Series X/S controllers
- **PlayStation**: DualShock 4, DualSense (PS5)
- **Nintendo**: Switch Pro Controller
- **Generic**: Any gamepad with standard HID support

## Get Started

Ready to dive in? Check out the [Quick Start Guide](getting-started/quick-start.md) or [Installation Instructions](installation/macos.md).

## Project Status

Conductor is currently at **v3.0** with multi-protocol input support, production-ready daemon infrastructure, and visual GUI configuration.

**What's New in v3.0**:
- 🎮 **HID Gamepad Support**: Xbox, PlayStation, Switch Pro controllers
- 🎯 **Unified Input Manager**: MIDI + gamepad in single workflow
- 📦 **Controller Templates**: 3 official gamepad templates (Xbox, PS, Switch)
- 🔍 **MIDI Learn for Gamepads**: Auto-detect gamepad buttons
- ⚡ **Hot-Plug Detection**: Automatic reconnection with exponential backoff

**v2.0.0 Features**:
- 🎛️ **Tauri GUI**: Visual configuration editor with MIDI Learn mode
- 🔄 **Hot-Reload Daemon**: 0-10ms config reloads without restart
- 🎯 **Per-App Profiles**: Automatic profile switching based on active app
- 📊 **Live Event Console**: Real-time event monitoring
- 📦 **Device Templates**: 6 built-in MIDI templates

See the [Roadmap](resources/roadmap.md) for planned features and [Changelog](resources/changelog.md) for full release notes.

## Community

- **GitHub**: [https://github.com/amiable-dev/conductor](https://github.com/amiable-dev/conductor)
- **Discussions**: [https://github.com/amiable-dev/conductor/discussions](https://github.com/amiable-dev/conductor/discussions)
- **Issues**: [https://github.com/amiable-dev/conductor/issues](https://github.com/amiable-dev/conductor/issues)

## License

Conductor is open source software licensed under the **MIT License**. See [LICENSE](https://github.com/amiable-dev/conductor/blob/main/LICENSE) for details.

---

**Next**: [Install Conductor](installation/macos.md) | [Quick Start](getting-started/quick-start.md)
