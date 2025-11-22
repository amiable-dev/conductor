# LED System

Conductor provides RGB LED feedback for controllers that support it, creating visual indicators for pad presses, modes, and system state.

## Overview

The LED system provides:

- **Real-time feedback**: LEDs respond instantly to pad presses
- **Mode visualization**: Different colors for different modes
- **Multiple schemes**: Reactive, rainbow, breathing, and more
- **Velocity sensitivity**: LED brightness/color reflects press intensity
- **HID support**: Direct RGB control for Maschine Mikro MK3 and similar devices
- **MIDI fallback**: Basic on/off for standard MIDI controllers

## Supported Devices

### Full RGB Support (HID)

- **Native Instruments Maschine Mikro MK3**: 16 RGB pads
- **Maschine MK3**: 16 RGB pads
- **Other HID RGB devices**: Configurable via device profiles

### MIDI LED Support

- **Launchpad series**: Note-based LED control
- **APC series**: CC-based LED control
- **Generic MIDI**: Via Note On/Off messages

## Lighting Schemes

### Reactive (Default)

LEDs respond to pad velocity and fade after release:

- **Soft** (0-40): Green
- **Medium** (41-80): Yellow
- **Hard** (81-127): Red
- **Fade time**: 1 second

```bash
# Enable reactive mode
conductor --led reactive 2
```

### Rainbow

Rotating rainbow pattern across all pads:

```bash
conductor --led rainbow 2
```

### Breathing

Pulsing effect synced across all pads:

```bash
conductor --led breathing 2
```

### Wave

Cascading wave pattern:

```bash
conductor --led wave 2
```

### Sparkle

Random twinkling effect:

```bash
conductor --led sparkle 2
```

### VU Meter

Bottom-to-top audio level visualization:

```bash
conductor --led vumeter 2
```

### Static

Solid color based on current mode:

```bash
conductor --led static 2
```

### Off

Disable LED feedback:

```bash
conductor --led off 2
```

## Configuration

### Global LED Settings

In `config.toml`:

```toml
[led_settings]
scheme = "reactive"
brightness = 100  # 0-100
fade_time_ms = 1000
enable_mode_colors = true
```

### Mode-Specific Colors

Each mode can have its own LED color theme:

```toml
[[modes]]
name = "Default"
color = "blue"  # LEDs tint blue when in this mode

[[modes]]
name = "Development"
color = "green"

[[modes]]
name = "Media"
color = "purple"
```

Available colors:
- `blue`, `green`, `purple`, `red`, `yellow`, `orange`, `pink`, `cyan`, `white`

### Per-Mapping LED Feedback

Individual mappings can override LED behavior:

```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 36

[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[modes.mappings.led]
color = "red"
brightness = 80
duration_ms = 500  # Override fade time
```

## HID LED Control (Maschine Mikro MK3)

### Direct RGB Access

Conductor uses HID for precise RGB control:

- **Latency**: <1ms response time
- **Color depth**: 24-bit RGB (16.7M colors)
- **Refresh rate**: 60Hz
- **Shared access**: Works alongside Native Instruments software

### LED Mapping

Physical pad layout to LED indices:

```
Pad Page A-H (16 pads):
┌────┬────┬────┬────┐
│ 36 │ 37 │ 38 │ 39 │  LED indices 0-3
├────┼────┼────┼────┤
│ 40 │ 41 │ 42 │ 43 │  LED indices 4-7
├────┼────┼────┼────┤
│ 44 │ 45 │ 46 │ 47 │  LED indices 8-11
├────┼────┼────┼────┤
│ 48 │ 49 │ 50 │ 51 │  LED indices 12-15
└────┴────┴────┴────┘
```

### Custom HID Patterns

Advanced users can create custom LED patterns:

```rust
// Example: Custom chase pattern
pub fn led_chase_pattern(leds: &mut MikroMK3LEDs, frame: u32) {
    let pos = (frame / 10) % 16;
    for i in 0..16 {
        if i == pos {
            leds.set_pad_rgb(i, 255, 0, 0); // Red
        } else if i == (pos + 1) % 16 {
            leds.set_pad_rgb(i, 128, 0, 0); // Dim red
        } else {
            leds.set_pad_rgb(i, 0, 0, 0); // Off
        }
    }
    leds.update();
}
```

## MIDI LED Control

### Standard MIDI Devices

For devices without HID RGB support, Conductor uses MIDI:

```toml
[led_settings]
use_midi_leds = true
midi_channel = 1
note_on_velocity = 127  # LED on brightness
note_off_velocity = 0   # LED off
```

### Launchpad-Style Control

Map LED colors to velocity values:

```toml
[led_settings.midi_colors]
red = 5
green = 21
yellow = 13
amber = 9
off = 12
```

### Custom MIDI LED Mapping

Define custom LED control messages:

```toml
[[led_settings.custom_mappings]]
pad = 36  # MIDI note
led_on = { type = "NoteOn", channel = 1, note = 36, velocity = 127 }
led_off = { type = "NoteOff", channel = 1, note = 36, velocity = 0 }
color_map = { red = 5, green = 21, yellow = 13 }
```

## GUI Configuration

### Via Settings Panel

1. Open Conductor GUI
2. Navigate to **Settings** tab
3. Scroll to **LED Configuration**
4. Select scheme from dropdown
5. Adjust brightness slider
6. Configure fade time
7. Enable/disable mode colors
8. Click **Save**

## Performance Optimization

### Reduce LED Updates

For battery-powered devices or performance optimization:

```toml
[led_settings]
update_rate_hz = 30  # Default: 60Hz
skip_intermediate_frames = true  # Only update on significant changes
```

### Disable LEDs Selectively

Turn off LEDs for specific modes:

```toml
[[modes]]
name = "Silent Mode"
color = "off"  # No LED feedback in this mode
```

## Troubleshooting

### LEDs Not Responding

1. **Check device support**:
   ```bash
   # List HID devices
   ls /dev/hidraw*  # Linux
   system_profiler SPUSBDataType  # macOS
   ```

2. **Verify permissions** (macOS):
   - System Settings → Privacy & Security → Input Monitoring
   - Grant access to Conductor

3. **Test with diagnostic tool**:
   ```bash
   cargo run --bin led_diagnostic
   ```

4. **Check HID access**:
   ```bash
   # macOS: Ensure shared device access
   DEBUG=1 conductor 2 --led reactive
   ```

### Wrong Colors

1. Verify color mapping in config
2. Check if device uses non-standard RGB order (some use GRB or BGR)
3. Try different lighting scheme
4. Calibrate brightness

### Flickering LEDs

1. Reduce update rate: `update_rate_hz = 30`
2. Enable frame skipping: `skip_intermediate_frames = true`
3. Check USB power supply
4. Disable other LED-controlling software

### LEDs Stuck On/Off

1. Restart Conductor
2. Power cycle MIDI device
3. Check for conflicting LED control (e.g., Native Instruments software)
4. Reset LEDs: `conductor --led off 2` then restart

## Advanced Customization

### Create Custom Schemes

Write custom lighting patterns in `~/.config/conductor/led_schemes/`:

```rust
// custom_pulse.rs
pub struct CustomPulse {
    frame: u32,
}

impl LedScheme for CustomPulse {
    fn update(&mut self, leds: &mut dyn PadFeedback) {
        self.frame += 1;
        let brightness = ((self.frame as f32 / 30.0).sin() * 127.0 + 128.0) as u8;

        for i in 0..16 {
            leds.set_pad_color(i, brightness, brightness, 255);
        }
    }
}
```

Load custom scheme:

```toml
[led_settings]
scheme = "custom:custom_pulse"
```

### Mode Transition Effects

Animate LED transitions when switching modes:

```toml
[led_settings.transitions]
enabled = true
duration_ms = 300
effect = "fade"  # fade, sweep, flash
```

## Best Practices

1. **Start with reactive**: Most intuitive for new users

2. **Match mode colors to usage**: Visual cues help remember mode purpose

3. **Test visibility**: Ensure LEDs visible in your lighting conditions

4. **Don't overdo it**: Complex animations can be distracting

5. **Battery consideration**: Disable LEDs for battery-powered setups

6. **Accessibility**: Use high-contrast colors for visibility

## Integration Examples

### LED Feedback for Success/Failure

Flash green on success, red on failure:

```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 36

[modes.mappings.action]
type = "Shell"
command = "./run_tests.sh"

[modes.mappings.led]
success_color = "green"
failure_color = "red"
flash_duration_ms = 500
```

### VU Meter for Audio Input

Display audio levels:

```bash
conductor --led vumeter --audio-input "System Audio" 2
```

### Custom Mode Indicators

Reserve specific pads for mode indication:

```toml
[led_settings.mode_indicators]
pad_36 = "mode_0"  # Blue when in mode 0
pad_37 = "mode_1"  # Green when in mode 1
pad_38 = "mode_2"  # Purple when in mode 2
always_on = true
```

## Next Steps

- Explore [GUI configuration](./gui.md) for visual LED setup
- Use [MIDI Learn](../getting-started/midi-learn.md) with LED feedback
- Set up [per-app profiles](./per-app-profiles.md) with different LED schemes
- Try the [event console](./event-console.md) to debug LED behavior
