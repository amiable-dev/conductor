# LED System Reference

Conductor provides comprehensive LED feedback for supported MIDI controllers, with full RGB control for HID devices (Native Instruments Maschine Mikro MK3) and basic on/off control for standard MIDI devices.

## Overview

The LED system consists of four main features:

1. **F23: LED Lighting Schemes** - 10 pre-defined animation patterns
2. **F24: LED Velocity Feedback** - Color-coded response based on pad velocity
3. **F25: LED Mode Indicators** - Visual feedback for active mode
4. **F26: LED Fade Effects** - Smooth transitions and fade-out animations

## Feature F23: LED Lighting Schemes

Conductor supports 10 distinct lighting schemes that can be selected at startup or configured in your config file.

### Available Schemes

| Scheme | Description | Use Case | CPU Impact |
|--------|-------------|----------|------------|
| `off` | All LEDs disabled | Power saving, minimal distraction | 0% |
| `static` | Solid color based on current mode | Clear mode indication | <1% |
| `breathing` | Slow 2-second breathing effect | Ambient, relaxed workflow | ~1% |
| `pulse` | Fast 500ms pulse | High-energy, performance | ~1% |
| `rainbow` | Rainbow cycle across pads | Creative sessions, visual appeal | ~2% |
| `wave` | Wave animation pattern | Dynamic feedback | ~2% |
| `sparkle` | Random sparkles | Playful, attention-grabbing | ~3% |
| `reactive` | Velocity-based colors (most common) | Precise feedback, performance | ~1% |
| `vumeter` | VU meter style (bottom-up) | Audio visualization | ~2% |
| `spiral` | Spiral pattern animation | Artistic, mesmerizing | ~2% |

### Command-Line Usage

```bash
# Start with reactive scheme (recommended)
cargo run --release 2 --led reactive

# Rainbow animation
cargo run --release 2 --led rainbow

# Disable LEDs
cargo run --release 2 --led off

# Static mode colors
cargo run --release 2 --led static
```

### Configuration

```toml
[led_settings]
scheme = "reactive"  # Default scheme on startup
```

### Implementation Details

- **Update Rate**: 10fps (100ms per frame) to maintain low CPU usage
- **HID Devices**: Full RGB control with 16.7 million colors
- **MIDI Devices**: Schemes degrade gracefully to on/off
- **Performance**: All schemes designed for <3% CPU usage

---

## Feature F24: LED Velocity Feedback

Reactive LED feedback provides immediate visual confirmation of pad velocity, using color-coded responses.

### Velocity Color Mapping

| Velocity Range | Color | Visual | Meaning |
|----------------|-------|--------|---------|
| 0-40 | Green | 🟢 | Soft press |
| 41-80 | Yellow | 🟡 | Medium press |
| 81-127 | Red | 🔴 | Hard press |

### Behavior

**On Press:**
- Pad immediately lights up in velocity-appropriate color
- Brightness scales with velocity (40% base + 60% velocity-based)
- Color change happens within <1ms for responsive feedback

**On Release:**
- Fade-out begins after 1 second
- Fade duration: 200ms (configurable in future versions)
- Pad returns to mode color or off state

### Configuration Example

```toml
[led_settings]
scheme = "reactive"

# Future: Velocity threshold customization
[led_settings.reactive]
soft_color = { r = 0, g = 255, b = 0 }      # Green
medium_color = { r = 255, g = 255, b = 0 }  # Yellow
hard_color = { r = 255, g = 0, b = 0 }      # Red
fade_duration_ms = 1000
```

### Use Cases

- **Performance**: Immediate feedback on hit strength for drum pads
- **Practice**: Visual metronome or timing reference
- **Recording**: Confirm input without audio monitoring
- **Accessibility**: Visual alternative to audio feedback

---

## Feature F25: LED Mode Indicators

Visual feedback showing the currently active mode, helping users stay oriented when switching contexts.

### Mode Color Scheme

Each mode has an associated color that LEDs display when using `static` or `reactive` schemes:

| Mode | Default Color | RGB Values | Visual |
|------|---------------|------------|--------|
| Mode 0 (Default) | Blue | (0, 100, 255) | 🔵 |
| Mode 1 (Development) | Green | (0, 255, 0) | 🟢 |
| Mode 2 (Media) | Purple | (200, 0, 255) | 🟣 |

### Configuration

```toml
[[modes]]
name = "Default"
color = "blue"
led_idle_brightness = 20      # Brightness when idle (0-255)
led_active_brightness = 255   # Brightness when pressed

[[modes]]
name = "Development"
color = "green"
led_idle_brightness = 30
led_active_brightness = 255

[[modes]]
name = "Media"
color = "purple"
led_idle_brightness = 15
led_active_brightness = 200
```

### Mode Transition Effects

When switching modes, you can apply visual transition effects:

```toml
[[global_mappings]]
description = "Next Mode with Flash"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1
relative = true
transition_effect = "Flash"  # Options: Flash, Sweep, FadeOut, Spiral, None
```

**Transition Effects:**
- **Flash**: Quick white flash (150ms) - best for rapid mode switching
- **Sweep**: Left-to-right wave (120ms) - smooth visual sweep
- **FadeOut**: Fade old color to new (200ms) - smooth transition
- **Spiral**: Center-outward spiral (240ms) - artistic transition

### Optional Mode Indicator Pads

You can dedicate specific pads to always show the current mode:

```toml
[led_settings]
mode_indicator_pads = [13, 14, 15, 16]  # Bottom row shows mode
# Pad 13 lights up for Mode 0
# Pad 14 lights up for Mode 1
# Pad 15 lights up for Mode 2
# Pad 16 lights up for Mode 3
```

---

## Feature F26: LED Fade Effects

Smooth transitions and fade animations provide polished visual feedback.

### Fade-Out on Release

After pressing a pad, the LED fades out gracefully instead of turning off instantly:

**Default Behavior:**
1. Pad pressed → Immediate color change (velocity-based)
2. Pad released → Wait 1 second
3. Fade begins → 200ms smooth fade to mode color or off

**Timeline:**
```
Press    Release            Fade Start         Complete
  |---------|-------------------|-----------------|
  0ms    Variable            +1000ms          +1200ms
  ↑                              ↑                 ↑
  Full brightness          Start fade         Dark/Mode color
```

### Configurable Fade Parameters

```toml
[led_settings.fade]
delay_after_release_ms = 1000  # How long to wait before fading
fade_duration_ms = 200         # How long the fade takes
steps = 10                     # Number of discrete fade steps
```

### Fade Applications

**Reactive Scheme:**
```rust
// Pseudo-code for reactive fade
on_pad_press(pad, velocity) {
    color = velocity_to_color(velocity)
    set_pad_color(pad, color)
    schedule_fade(pad, delay=1000ms, duration=200ms)
}
```

**Mode Transition:**
```rust
// Pseudo-code for mode transition fade
on_mode_change(old_mode, new_mode) {
    // FadeOut transition effect
    for brightness in (100% down_to 0%).step_by(10%) {
        set_all_pads(old_mode_color * brightness)
        wait(10ms)
    }
    for brightness in (0% up_to 100%).step_by(10%) {
        set_all_pads(new_mode_color * brightness)
        wait(10ms)
    }
}
```

---

## Device Support

### HID Devices (Full RGB Control)

**Native Instruments Maschine Mikro MK3:**
- ✅ All 10 lighting schemes
- ✅ Full RGB color spectrum (16.7M colors)
- ✅ Velocity-based colors
- ✅ Smooth fades and transitions
- ✅ Shared device mode (works alongside NI Controller Editor)

**Requirements:**
- macOS: Input Monitoring permission
- USB connection: Direct HID communication
- Drivers: Native Instruments drivers installed

### MIDI Devices (On/Off Only)

**Supported Controllers:**
- Novation Launchpad Mini/Pro
- Akai APC Mini/40
- Generic MIDI pad controllers

**Limitations:**
- ❌ RGB colors (on/off only)
- ❌ Smooth fades (instant on/off)
- ✅ Reactive feedback (on when pressed, off after delay)
- ✅ Mode indicators (basic)

**Note Range:** MIDI LEDs typically respond to notes C1-D#2 (36-51) for 16 pads

---

## Configuration Examples

### Minimal Configuration

```toml
[device]
name = "Mikro"
led_feedback = true

[led_settings]
scheme = "reactive"
```

### Advanced Configuration

```toml
[device]
name = "Mikro"
led_feedback = true

[led_settings]
scheme = "reactive"
brightness = 255

[led_settings.reactive]
soft_color = { r = 0, g = 255, b = 0 }
medium_color = { r = 255, g = 255, b = 0 }
hard_color = { r = 255, g = 0, b = 0 }
fade_duration_ms = 1000

[led_settings.transitions]
enable_effects = true
flash_duration_ms = 150
sweep_delay_ms = 30
fadeout_steps = 10
spiral_delay_ms = 15

[[modes]]
name = "Default"
color = "blue"
led_idle_brightness = 20
led_active_brightness = 255

[[modes]]
name = "Development"
color = "green"
led_idle_brightness = 30
led_active_brightness = 255
```

---

## Troubleshooting

### LEDs Not Working (Mikro MK3)

1. **Check permissions** (macOS):
   - System Settings → Privacy & Security → Input Monitoring
   - Enable permission for Terminal or your IDE

2. **Verify HID connection**:
   ```bash
   DEBUG=1 cargo run --release 2 --led reactive
   # Look for "✓ Connected to Mikro MK3 LED interface"
   ```

3. **Check for conflicts**:
   - Close Native Instruments Controller Editor
   - Unplug and replug USB cable
   - Try different USB port

### Wrong Colors

- **Issue**: Colors don't match expected values
- **Cause**: LED manufacturing variance (±10% color accuracy)
- **Solution**: This is normal, colors may vary slightly between units

### Flickering LEDs

- **Issue**: LEDs flicker or strobe
- **Cause**: Update rate too high or USB bandwidth saturation
- **Solution**: Use simpler scheme (reactive or static) instead of complex animations

### Mode Colors Not Showing

- **Issue**: All pads same color regardless of mode
- **Cause**: Using non-mode-aware scheme
- **Solution**: Use `static` or `reactive` scheme, not `rainbow` or `sparkle`

---

## Performance Considerations

### CPU Usage by Scheme

| Scheme | CPU (Idle) | CPU (Active) | Memory |
|--------|------------|--------------|--------|
| off | 0% | 0% | Minimal |
| static | <1% | <1% | <1MB |
| reactive | <1% | ~1% | <1MB |
| breathing | ~1% | ~1% | <1MB |
| rainbow | ~2% | ~2% | <1MB |
| sparkle | ~3% | ~3% | ~1MB |

### USB Bandwidth

- **Update Rate**: 10fps (100ms per frame)
- **Data per Update**: 51 bytes (1 byte report ID + 50 bytes RGB data)
- **Bandwidth**: 510 bytes/sec (~0.004 Mbps)
- **Impact**: Negligible on USB 2.0 (480 Mbps)

### Recommendations

- **Performance Mode**: Use `reactive` or `static` for lowest CPU usage
- **Visual Appeal**: Use `rainbow` or `spiral` for presentations/demos
- **Battery (USB-powered hubs)**: Use `off` or `static` to reduce power draw

---

## API Reference

### Trait Interface

```rust
pub trait PadFeedback: Send {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>>;
    fn set_mode_colors(&mut self, mode: u8) -> Result<(), Box<dyn Error>>;
    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>>;
    fn flash_pad(&mut self, pad: u8, color: RGB, duration_ms: u64) -> Result<(), Box<dyn Error>>;
    fn ripple_effect(&mut self, start_pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn clear_all(&mut self) -> Result<(), Box<dyn Error>>;
    fn show_long_press_feedback(&mut self, pad: u8, elapsed_ms: u128) -> Result<(), Box<dyn Error>>;
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>>;
}
```

### RGB Color Structure

```rust
pub struct RGB {
    pub r: u8,  // Red: 0-255
    pub g: u8,  // Green: 0-255
    pub b: u8,  // Blue: 0-255
}

impl RGB {
    pub const OFF: RGB = RGB { r: 0, g: 0, b: 0 };
    pub const RED: RGB = RGB { r: 255, g: 0, b: 0 };
    pub const GREEN: RGB = RGB { r: 0, g: 255, b: 0 };
    pub const BLUE: RGB = RGB { r: 0, g: 0, b: 255 };
    pub const YELLOW: RGB = RGB { r: 255, g: 255, b: 0 };
    pub const PURPLE: RGB = RGB { r: 255, g: 0, b: 255 };
}
```

---

## See Also

- [Configuration → LED Feedback](../configuration/led-feedback.md)
- [Device Support → Maschine Mikro MK3](../devices/mikro-mk3.md)
- [Reference → Action Types](action-types.md) (ModeChange action)
- [Troubleshooting → Common Issues](../troubleshooting/common-issues.md)
