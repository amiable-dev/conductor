# System Utilities Plugin

Common system interactions via MIDI controller: clipboard, screenshots, notifications, window/audio control.

## Description

This plugin demonstrates practical system utilities that are commonly needed in daily workflows. It showcases how WASI capabilities (filesystem, subprocess, systemcontrol) enable powerful system integration.

## Current Status (v2.5)

**Note:** Reference implementation for v2.6. WASI Preview1 has limited subprocess/filesystem support. This demonstrates the interface design for v2.6's full capability implementation.

## Supported Actions

| Action | Description | Parameters | Capability |
|--------|-------------|------------|------------|
| `copy_to_clipboard` | Copy text to clipboard | `text` | subprocess |
| `paste_from_clipboard` | Paste from clipboard | None | subprocess |
| `take_screenshot` | Capture screenshot | `filename` | subprocess |
| `show_notification` | System notification | `title`, `message` | subprocess |
| `minimize_window` | Minimize active window | None | systemcontrol |
| `maximize_window` | Maximize active window | None | systemcontrol |
| `mute_system` | Mute system audio | None | systemcontrol |
| `unmute_system` | Unmute system audio | None | systemcontrol |

## Platform Support

Commands designed for cross-platform execution:
- **macOS:** pbcopy/pbpaste, screencapture, osascript
- **Linux:** xclip, scrot, notify-send, wmctrl, amixer
- **Windows:** PowerShell cmdlets, snippingtool

## Building

```bash
cargo build --target wasm32-wasip1 --release
```

Binary: `target/wasm32-wasip1/release/midimon_wasm_system_utils.wasm`

## Testing

```bash
# Unit tests (7/7 passing)
cargo test
```

## Plugin Info

- **Name:** system_utils
- **Version:** 0.1.0
- **Author:** Amiable Team
- **License:** MIT
- **Binary Size:** 60 KB
- **Capabilities:** filesystem, subprocess, systemcontrol
- **Test Coverage:** 7/7 passing (100%)

## Use Cases

- Quick screenshots during streams
- Clipboard automation for repetitive tasks
- System notifications for recording status
- Window management during multitasking
- Audio control for quick mute

## Development

See the [WASM Plugin Development Guide](../../docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md) for detailed information.
