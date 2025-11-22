# LED Feedback Configuration

Conductor provides rich LED feedback for MIDI controllers with comprehensive support for RGB control (HID devices) and basic on/off control (MIDI devices).

## Quick Start

### Command-Line Usage

The fastest way to enable LED feedback is via command-line flags:

```bash
# Start with reactive velocity feedback (recommended)
cargo run --release 2 --led reactive

# Try different schemes
cargo run --release 2 --led rainbow
cargo run --release 2 --led static
cargo run --release 2 --led off
```

### Available Schemes

Choose from 10 built-in lighting schemes:

- **off** - All LEDs disabled
- **static** - Solid color based on current mode
- **breathing** - Slow 2-second breathing effect
- **pulse** - Fast 500ms pulse
- **rainbow** - Rainbow cycle across pads
- **wave** - Wave pattern animation
- **sparkle** - Random sparkles
- **reactive** - Velocity-based colors (green/yellow/red)
- **vumeter** - VU meter style (bottom-up)
- **spiral** - Spiral pattern animation

## Configuration File

### Basic Setup

Add LED settings to your `config.toml`:

```toml
[device]
name = "Mikro"
led_feedback = true  # Enable LED control

[led_settings]
scheme = "reactive"  # Default scheme
brightness = 255     # Full brightness (0-255)
```

### Reactive Scheme (Velocity Feedback)

The most commonly used scheme, providing color-coded velocity feedback:

```toml
[led_settings]
scheme = "reactive"

[led_settings.reactive]
# Velocity ranges and colors
soft_color = { r = 0, g = 255, b = 0 }      # Green (0-40)
medium_color = { r = 255, g = 255, b = 0 }  # Yellow (41-80)
hard_color = { r = 255, g = 0, b = 0 }      # Red (81-127)

# Fade behavior
fade_duration_ms = 1000  # Wait 1s before fading
fade_steps = 10          # Number of fade steps (smoother = more steps)
```

**Visual Example:**
- Soft tap → 🟢 Green LED
- Medium tap → 🟡 Yellow LED
- Hard hit → 🔴 Red LED
- After 1 second → Smooth fade to mode color

### Mode-Based Colors

Each mode can have its own color scheme:

```toml
[[modes]]
name = "Default"
color = "blue"
led_idle_brightness = 20      # Dim when idle
led_active_brightness = 255   # Full when pressed

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

**Supported Colors:**
- `"blue"` - (0, 100, 255)
- `"green"` - (0, 255, 0)
- `"red"` - (255, 0, 0)
- `"purple"` - (200, 0, 255)
- `"yellow"` - (255, 255, 0)
- `"cyan"` - (0, 255, 255)
- `"white"` - (255, 255, 255)
- Custom: `{ r = 128, g = 64, b = 200 }`

### Mode Transition Effects

Add visual effects when switching modes:

```toml
[[global_mappings]]
description = "Next Mode with Sweep Effect"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = 1
relative = true
transition_effect = "Sweep"
```

**Transition Effects:**

```toml
[led_settings.transitions]
enable_effects = true
flash_duration_ms = 150   # Flash effect (white → off → new color)
sweep_delay_ms = 30       # Sweep effect (left-to-right wave)
fadeout_steps = 10        # FadeOut effect (smooth color transition)
spiral_delay_ms = 15      # Spiral effect (center-outward)
```

**Available Effects:**
- **Flash** - Quick white flash (150ms) - fast mode switching
- **Sweep** - Left-to-right wave (120ms) - smooth sweep
- **FadeOut** - Fade old to new color (200ms) - smooth transition
- **Spiral** - Center-outward spiral (240ms) - artistic
- **None** - Instant change (0ms) - no animation

### Mode Indicator Pads

Dedicate specific pads to show the current mode:

```toml
[led_settings]
mode_indicator_pads = [13, 14, 15, 16]  # Bottom row
# Pad 13 = Mode 0 indicator
# Pad 14 = Mode 1 indicator
# Pad 15 = Mode 2 indicator
# Pad 16 = Mode 3 indicator
```

When in Mode 1, pad 14 will light up in green while others remain dim.

## Advanced Configuration

### Animation Settings

Fine-tune animation parameters for each scheme:

```toml
[led_settings.breathing]
cycle_duration_ms = 2000  # 2-second breathing cycle
min_brightness = 0        # Fully off at minimum
max_brightness = 255      # Full brightness at peak

[led_settings.rainbow]
speed = 60           # Degrees per second
saturation = 255     # Full saturation
brightness = 255     # Full brightness
hue_spacing = 22.5   # Degrees between pads

[led_settings.sparkle]
spawn_rate_ms = 100  # New sparkle every 100ms
fade_duration_ms = 200  # Sparkle fades in 200ms
max_active = 4       # Max 4 sparkles at once
```

### HID-Specific Settings (Mikro MK3)

```toml
[device]
name = "Mikro"
led_feedback = true
use_hid = true  # Force HID mode

[led_settings.hid]
shared_device = true  # Allow NI Controller Editor access
vendor_id = 0x17cc    # Native Instruments
product_id = 0x1700   # Maschine Mikro MK3
```

### MIDI Feedback Settings

For standard MIDI controllers:

```toml
[device]
name = "Launchpad Mini"
led_feedback = true

[led_settings.midi]
note_offset = 36  # C1 (MIDI note 36) = Pad 1
channel = 0       # MIDI channel 0-15
on_velocity = 127 # Velocity for "LED on"
off_velocity = 0  # Velocity for "LED off"
```

## Platform-Specific Notes

### macOS

**HID devices require Input Monitoring permission:**

1. Open System Settings → Privacy & Security
2. Select "Input Monitoring"
3. Enable permission for Terminal or your IDE

**Shared device mode:**
```toml
[led_settings.hid]
shared_device = true  # Works alongside NI Controller Editor
```

### Linux

**HID devices may require udev rules:**

```bash
# Create udev rule for Mikro MK3
sudo nano /etc/udev/rules.d/99-maschine-mikro-mk3.rules

# Add this line:
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1700", MODE="0666"

# Reload rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Windows

**HID devices work out of the box, but may require:**

- Native Instruments drivers installed
- USB device permissions (Windows 10+)

## Use Cases

### Performance/Live Use

```toml
[led_settings]
scheme = "reactive"  # Immediate velocity feedback
brightness = 255     # Full brightness for stage visibility
```

### Studio/Production

```toml
[led_settings]
scheme = "static"    # Subtle, non-distracting
brightness = 50      # Dim for low-light studio
```

### Creative/Visual

```toml
[led_settings]
scheme = "rainbow"   # Eye-catching animations
brightness = 200     # Bright but not overwhelming
```

### Focused Work

```toml
[led_settings]
scheme = "off"       # No distractions
```

## Troubleshooting

### LEDs Not Responding

1. **Check device connection:**
   ```bash
   DEBUG=1 cargo run --release 2 --led reactive
   # Look for: "✓ Connected to Mikro MK3 LED interface"
   ```

2. **Verify permissions** (macOS HID):
   - System Settings → Privacy → Input Monitoring

3. **Try MIDI fallback:**
   ```toml
   [device]
   led_feedback = true
   use_hid = false  # Force MIDI mode
   ```

### Wrong Colors

- **Issue**: Colors don't match expected RGB values
- **Cause**: LED manufacturing variance (±10%)
- **Solution**: Normal behavior, slight color variations expected

### Flickering

- **Issue**: LEDs flicker or strobe
- **Cause**: Update rate too high or USB bandwidth saturation
- **Solution**: Use simpler scheme (`reactive` or `static`)

### Mode Colors Not Showing

- **Issue**: All pads same color regardless of mode
- **Cause**: Using non-mode-aware scheme
- **Solution**: Use `static` or `reactive` scheme

### Performance Impact

- **Issue**: High CPU usage
- **Cause**: Complex animation scheme
- **Solution**: Switch to `reactive` or `static` (<1% CPU)

## Examples

### Example 1: Simple Reactive Setup

```toml
[device]
name = "Mikro"
led_feedback = true

[led_settings]
scheme = "reactive"

[[modes]]
name = "Default"
color = "blue"

[[modes]]
name = "Development"
color = "green"
```

### Example 2: Advanced Multi-Mode with Transitions

```toml
[device]
name = "Mikro"
led_feedback = true

[led_settings]
scheme = "reactive"
brightness = 255

[led_settings.transitions]
enable_effects = true
flash_duration_ms = 150

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

[[modes]]
name = "Media"
color = "purple"
led_idle_brightness = 15
led_active_brightness = 200

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
transition_effect = "Flash"
```

### Example 3: Studio Mode with Indicators

```toml
[device]
name = "Mikro"
led_feedback = true

[led_settings]
scheme = "static"
brightness = 50
mode_indicator_pads = [13, 14, 15, 16]

[[modes]]
name = "Recording"
color = "red"
led_idle_brightness = 10

[[modes]]
name = "Editing"
color = "green"
led_idle_brightness = 10

[[modes]]
name = "Mixing"
color = "blue"
led_idle_brightness = 10
```

## Performance

### CPU Usage by Scheme

| Scheme | CPU (Idle) | CPU (Active) |
|--------|------------|--------------|
| off | 0% | 0% |
| static | <1% | <1% |
| reactive | <1% | ~1% |
| breathing | ~1% | ~1% |
| rainbow | ~2% | ~2% |
| sparkle | ~3% | ~3% |

### Memory

- **Static schemes**: <1MB
- **Animated schemes**: <1MB
- **Per-pad state**: ~50 bytes per pad

### Update Rate

- **HID devices**: 10fps (100ms per frame)
- **MIDI devices**: Event-driven (no polling)

## See Also

- [Reference → LED System](../reference/led-system.md) - Complete LED feature reference
- [Reference → Action Types](../reference/action-types.md) - ModeChange action
- [Device Support → Maschine Mikro MK3](../devices/mikro-mk3.md) - HID device details
- [Configuration → Modes](modes.md) - Mode system configuration
