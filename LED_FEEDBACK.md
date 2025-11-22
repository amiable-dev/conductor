# LED Feedback System

## Overview

The MIDI Macro Pad now includes a comprehensive LED feedback system that provides visual indication of device status, mode changes, and MIDI events. The system supports both HID-based devices (like Maschine Mikro MK3) and standard MIDI devices with LED feedback.

## Architecture

### Unified Feedback Interface

The system uses a trait-based design (`PadFeedback`) that abstracts LED control across different device types:

- **HID Devices** (Maschine Mikro MK3): Full RGB LED control via `MikroMK3LEDs`
- **MIDI Devices**: Basic LED feedback via MIDI Note On/Off messages using `MidiFeedback`

### Key Components

1. **`feedback.rs`**: Unified feedback trait and device factory
2. **`mikro_leds.rs`**: HID LED control for Maschine Mikro MK3
3. **`midi_feedback.rs`**: Standard MIDI LED feedback

## LED Lighting Schemes

### Available Schemes

| Scheme | Description | Best For |
|--------|-------------|----------|
| `off` | All LEDs off | Minimal distraction |
| `static` | Static colors based on current mode | Mode indication |
| `breathing` | Slow breathing effect | Ambient lighting |
| `pulse` | Fast pulse effect | Active monitoring |
| `rainbow` | Rainbow cycle across pads | Demo/visual appeal |
| `wave` | Wave pattern | Visual flow |
| `sparkle` | Random sparkles | Decorative |
| `reactive` | React to MIDI events only | Performance feedback |
| `vumeter` | VU meter style (bottom-up) | Audio level indication |
| `spiral` | Spiral pattern | Creative visuals |

### Usage

```bash
# Default (reactive mode)
midimon 0

# Rainbow effect
midimon 0 --led rainbow

# Static mode colors
midimon 0 --led static

# VU meter style
midimon 0 --led vumeter

# Breathing effect
midimon 0 --led breathing

# Off (no LEDs)
midimon 0 --led off
```

### Command-Line Options

```
Usage: midimon [PORT] [--led SCHEME]

Arguments:
  PORT          MIDI device port number (default: 0)
  --led SCHEME  LED lighting scheme

Examples:
  midimon 0              # Connect to port 0 with reactive LEDs
  midimon --led rainbow  # Rainbow effect on default port
  midimon 1 --led pulse  # Port 1 with pulse effect
```

## Reactive Mode

The `reactive` scheme provides real-time visual feedback for MIDI events:

- **Pad Press**: LED brightness varies with velocity
  - Soft press (velocity < 40): Green, dimmed
  - Medium press (velocity 40-79): Yellow, medium brightness
  - Hard press (velocity ≥ 80): Red, full brightness
- **Pad Release**: LED turns off
- **Mode Change**: LEDs update to show new mode colors
- **Action Execution**: Brief flash on successful action

## Mode-Specific Colors

When using `static` or `reactive` schemes, each mode has distinct color themes:

- **Mode 0** (Default): Blue/Cyan theme
- **Mode 1** (Development): Green/Yellow theme
- **Mode 2** (Media): Purple/Magenta theme
- **Mode 3+**: White, dimmed

## Device Detection

The system automatically detects the device type:

1. **Maschine Mikro MK3**: Uses HID protocol for full RGB LED control
2. **Other MIDI Devices**: Uses standard MIDI Note On/Off for basic LED feedback

Detection is based on the MIDI device name. If "Maschine" and "Mikro" appear in the name, HID mode is activated.

## Advanced Features

### Visual Effects

- **Ripple Effect**: Expanding wave from pressed pad
- **Flash**: Temporary color change with auto-restore
- **Gradient**: Smooth color transitions
- **Long Press Feedback**: Color changes during hold

### Performance Optimizations

- Efficient HID report batching
- Minimal CPU usage for LED updates
- Non-blocking LED operations
- Automatic device reconnection

## Implementation Details

### HID Protocol (Mikro MK3)

- **Vendor ID**: `0x17CC` (Native Instruments)
- **Product ID**: `0x1700` (Maschine Mikro MK3)
- **Interface**: Interface 0 (LED control)
- **Report ID**: `0x80` (LED update)
- **Data Format**: RGB values for all 16 pads (3 bytes per pad)

### MIDI Protocol (Standard Devices)

- **Note Range**: C1 (36) to D#2 (51) for 16 pads
- **Velocity**: Maps to LED brightness (0-127)
- **Channel**: Defaults to channel 1

## Troubleshooting

### LEDs Not Working

1. **Check Device Connection**: Ensure device is properly connected
2. **HID Permissions**: On macOS, grant Input Monitoring permissions
3. **Device Support**: Verify device supports LED feedback
4. **Scheme Selection**: Try different schemes (some devices may not support all effects)

### Performance Issues

1. **Reduce Update Rate**: Use simpler schemes (`static`, `off`)
2. **Disable LEDs**: Use `--led off` for minimal overhead
3. **Check CPU Usage**: LED effects use minimal CPU but can add overhead

### Device-Specific Issues

#### Maschine Mikro MK3

- **HID Access**: May require admin privileges on some systems
- **Driver Conflicts**: Ensure Native Instruments software is not running
- **Interface Number**: Must use interface 0 for LED control

#### Standard MIDI Devices

- **Limited Support**: Only basic on/off feedback available
- **No RGB**: Cannot display colors, only brightness
- **Note Mapping**: Ensure device uses standard drum pad mapping

## Future Enhancements

Planned features for future releases:

- [ ] Custom color palettes
- [ ] User-definable lighting patterns
- [ ] Animation speed control
- [ ] Per-pad color mapping
- [ ] MIDI clock sync for animated effects
- [ ] Save/load lighting presets
- [ ] Live performance mode with scene-based lighting

## Examples

### Basic Usage

```bash
# Start with default reactive LEDs
cargo run --bin midimon -- 0

# Use rainbow effect for visual demo
cargo run --bin midimon -- 0 --led rainbow

# Minimal mode (no LEDs)
cargo run --bin midimon -- 0 --led off
```

### Advanced Usage

```bash
# Production performance setup
cargo run --bin midimon -- 0 --led reactive

# Studio ambient lighting
cargo run --bin midimon -- 0 --led breathing

# Visual presentation mode
cargo run --bin midimon -- 0 --led spiral
```

## API Reference

### PadFeedback Trait

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

### RGB Color Values

```rust
pub const OFF: RGB = RGB { r: 0, g: 0, b: 0 };
pub const WHITE: RGB = RGB { r: 255, g: 255, b: 255 };
pub const RED: RGB = RGB { r: 255, g: 0, b: 0 };
pub const GREEN: RGB = RGB { r: 0, g: 255, b: 0 };
pub const BLUE: RGB = RGB { r: 0, g: 0, b: 255 };
// ... and more
```

## Contributing

When adding new lighting schemes or effects:

1. Implement in `MikroMK3LEDs` first (full RGB support)
2. Add fallback behavior for standard MIDI devices
3. Update `LightingScheme` enum and `from_str()` parser
4. Add to help text in `print_usage()`
5. Document in this README

## License

Same as main project.
