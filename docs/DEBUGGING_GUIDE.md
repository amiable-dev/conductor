# Debugging Guide: Maschine Mikro MK3 LED Issues

## Overview

This guide documents common issues encountered during LED implementation and their solutions. Use this as a reference when debugging similar MIDI/HID controller projects.

## Issue 1: LEDs Not Responding at All

### Symptoms
- MIDI input works (notes received)
- No LED activity whatsoever
- No HID errors reported

### Root Causes & Solutions

#### 1.1 HID Device Access Denied (macOS)

**Problem**: macOS denies access to HID devices already in use by other applications.

**Diagnosis**:
```rust
match hidapi.open(VENDOR_ID, PRODUCT_ID) {
    Ok(device) => { /* works */ }
    Err(e) => println!("Error: {}", e),  // "Device busy" or similar
}
```

**Solution**: Enable shared device access
```toml
# Cargo.toml
[dependencies]
hidapi = { version = "2.4", features = ["macos-shared-device"] }
```

**Why it works**: Uses `kIOHIDOptionsTypeSeizeDevice` flag to allow concurrent access.

**Test**: Keep NI Controller Editor open while running your app.

#### 1.2 Wrong Report ID

**Problem**: HID report sent with incorrect Report ID.

**Diagnosis**:
```rust
// Check what the hardware expects
// For MK3: Report ID must be 0x80
buffer[0] = 0x80;  // Report ID
```

**Test**: Try different report IDs (0x00, 0x01, 0x80, etc.) and observe HID write results.

#### 1.3 Incorrect Buffer Size

**Problem**: Buffer too small or too large for device.

**Diagnosis**:
```rust
let result = device.write(&buffer)?;
println!("Wrote {} bytes (expected 81)", result);
```

**Solution**: MK3 requires exactly 81 bytes:
- Byte 0: Report ID (0x80)
- Bytes 1-80: Data (pads at offset 39-54)

## Issue 2: Wrong Pads Lighting Up

### Symptoms
- LEDs respond but light incorrect pads
- Pattern appears rotated, flipped, or scrambled
- Some pads work correctly, others don't

### Root Causes & Solutions

#### 2.1 Using MIDI Notes Instead of Pad Indices

**Problem**: Directly using MIDI note numbers as LED addresses.

**Wrong**:
```rust
fn on_midi_note(note: u8) {
    // ❌ WRONG: note 12 doesn't necessarily map to LED 0
    set_led(note - 12, Color::Green);
}
```

**Correct**:
```rust
fn on_midi_note(note: u8) {
    // ✅ CORRECT: Use profile mapping
    if let Some(pad_index) = profile.note_to_pad_index(note, current_page) {
        let led_pos = map_pad_to_led_position(pad_index);
        set_led(led_pos, Color::Green);
    }
}
```

**Diagnosis**: Test with single-pad profile mappings to verify note→pad→LED chain.

#### 2.2 Coordinate System Mismatch (Vertical Flip)

**Problem**: LEDs appear vertically inverted.

**Symptoms**:
- Bottom pads light when top pads are pressed
- Top pads light when bottom pads are pressed
- Left-right orientation correct

**Diagnosis**:
```rust
// Test all corners
for (pad, expected_corner) in [(0, "bottom-left"), (3, "bottom-right"),
                                (12, "top-left"), (15, "top-right")] {
    println!("Press pad at {} - pad index {}", expected_corner, pad);
    set_led(pad, Color::White);  // Does it light the correct corner?
}
```

**Solution**: Apply vertical flip transformation
```rust
fn map_pad_to_led_position(pad_index: u8) -> u8 {
    let row = pad_index / 4;        // 0-3 (bottom to top in profile)
    let col = pad_index % 4;        // 0-3 (left to right)
    let flipped_row = 3 - row;      // Invert row
    flipped_row * 4 + col           // LED position (top to bottom)
}
```

**Root Cause**: Profile uses bottom-up numbering, hardware uses top-down addressing.

#### 2.3 Incorrect Grid Dimensions

**Problem**: Wrong calculation of row/column from index.

**Diagnosis**:
```rust
// Verify grid arithmetic
for i in 0..16 {
    let row = i / 4;
    let col = i % 4;
    println!("Pad {} -> row {}, col {}", i, row, col);
    assert!(row < 4 && col < 4);
}
```

## Issue 3: Incorrect Colors

### Symptoms
- LEDs light but wrong colors displayed
- Colors appear washed out or too bright
- Random colors instead of expected ones

### Root Causes & Solutions

#### 3.1 Wrong Color Encoding

**Problem**: Using RGB values instead of indexed colors.

**Wrong**:
```rust
// ❌ MK3 doesn't use RGB
let red_rgb = 0xFF0000;
buffer[39] = red_rgb as u8;  // Doesn't work!
```

**Correct**:
```rust
// ✅ Use indexed color encoding
const COLOR_RED: u8 = 1;
const BRIGHTNESS_NORMAL: u8 = 2;
let led_byte = (COLOR_RED << 2) | BRIGHTNESS_NORMAL;
buffer[39] = led_byte;  // 0x06 = Red at Normal brightness
```

**Reference**: See color index table in `LED_SYSTEM.md`

#### 3.2 Brightness Bits Incorrect

**Problem**: Brightness field not properly masked.

**Diagnosis**:
```rust
// Check encoding
let color = 7;  // Green
let brightness = 2;  // Normal
let encoded = (color << 2) | brightness;
println!("Encoded: 0x{:02X} (should be 0x1E)", encoded);
assert_eq!(encoded, 0x1E);
```

**Solution**: Ensure brightness is masked to 2 bits:
```rust
let led_byte = (color_index << 2) | (brightness & 0b11);
```

#### 3.3 Color Index Out of Range

**Problem**: Using undefined color indices.

**Diagnosis**:
```rust
// Valid color indices for MK3
const VALID_COLORS: &[u8] = &[1, 2, 5, 7, 11, 13, 14, 17];

fn validate_color(color: u8) {
    if !VALID_COLORS.contains(&color) {
        eprintln!("Warning: Color {} may not work correctly", color);
    }
}
```

## Issue 4: LEDs Stay Lit Too Long

### Symptoms
- LEDs don't turn off after pad release
- Multiple pads accumulate without clearing
- LEDs eventually all stay on

### Root Causes & Solutions

#### 4.1 No Fade-Out Implementation

**Problem**: Missing logic to clear LEDs after time delay.

**Solution**: Implement fade-out tracking
```rust
struct PadState {
    color: u8,
    brightness: u8,
    lit_at: Instant,
}

const FADEOUT_DELAY: Duration = Duration::from_millis(1000);

fn check_fadeouts(pad_states: &mut HashMap<u8, PadState>) {
    let now = Instant::now();
    for (pad_idx, state) in pad_states.iter() {
        if now.duration_since(state.lit_at) >= FADEOUT_DELAY {
            set_led(*pad_idx, 0);  // Clear LED
            pad_states.remove(pad_idx);
        }
    }
}
```

#### 4.2 LED Clear Not Working

**Problem**: Setting LED to "off" doesn't work.

**Diagnosis**:
```rust
// Test explicit off command
set_led(0, 0);  // Should turn off pad 0
thread::sleep(Duration::from_secs(1));
// Is LED off?
```

**Solution**: Ensure off encoding is correct
```rust
const LED_OFF: u8 = 0x00;  // Color 0, Brightness 0
buffer[39 + led_pos] = LED_OFF;
device.write(&buffer)?;
```

## Issue 5: Patterns Not Animating

### Symptoms
- Static patterns work (reactive mode)
- Animated patterns frozen or not updating
- Pattern appears once then stops

### Root Causes & Solutions

#### 5.1 Missing Update Loop

**Problem**: No continuous refresh for animated patterns.

**Solution**: Implement animation loop
```rust
let mut last_update = Instant::now();
let update_interval = Duration::from_millis(100);

loop {
    // Process MIDI events...
    
    // Update patterns (if not reactive mode)
    if lighting_scheme != LightingScheme::Reactive {
        let now = Instant::now();
        if now.duration_since(last_update) >= update_interval {
            feedback.run_scheme(&lighting_scheme)?;
            last_update = now;
        }
    }
}
```

**Frequency**: 100ms (10 FPS) is good balance between smoothness and HID overhead.

#### 5.2 Pattern Function Not Called

**Problem**: Scheme dispatcher doesn't route to pattern function.

**Diagnosis**:
```rust
pub fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<()> {
    println!("Running scheme: {:?}", scheme);  // Add logging
    match scheme {
        LightingScheme::Sparkle => self.sparkle_effect(),
        // ... other patterns
    }
}
```

#### 5.3 HID Write Blocking/Failing

**Problem**: HID writes block or fail silently in animation loop.

**Diagnosis**:
```rust
match device.write(&buffer) {
    Ok(n) => println!("Wrote {} bytes", n),
    Err(e) => eprintln!("HID write failed: {}", e),
}
```

**Solution**: Use non-blocking writes or handle errors gracefully
```rust
let _ = device.write(&buffer);  // Continue on error
```

## Issue 6: Profile Not Loading

### Symptoms
- "Failed to load profile" error
- Profile loads but wrong mappings
- Some pads not mapped

### Root Causes & Solutions

#### 6.1 File Path Incorrect

**Diagnosis**:
```bash
# Check file exists
ls -l "path/to/profile.ncmm3"

# Check permissions
chmod +r "path/to/profile.ncmm3"
```

**Solution**: Use absolute paths or verify working directory
```rust
let profile_path = std::fs::canonicalize(profile_arg)?;
println!("Loading profile from: {}", profile_path.display());
```

#### 6.2 Invalid XML Structure

**Diagnosis**:
```bash
# Validate XML syntax
xmllint --noout "profile.ncmm3"
```

**Common issues**:
- Missing closing tags
- Invalid attribute values
- Wrong element nesting

#### 6.3 Incomplete Mapping

**Problem**: Profile has fewer than 16 pads mapped.

**Diagnosis**:
```rust
pub fn validate_profile(profile: &DeviceProfile) {
    for (i, page) in profile.pages.iter().enumerate() {
        if page.mappings.len() != 16 {
            eprintln!("Warning: Page {} only has {} mappings", 
                     i, page.mappings.len());
        }
    }
}
```

## Debugging Tools & Techniques

### 1. HID Traffic Monitoring

**macOS**: Use USB Prober (part of Additional Tools for Xcode)
```bash
# Install if needed
xcode-select --install
```

**Linux**: Use `usbmon`
```bash
sudo modprobe usbmon
sudo cat /sys/kernel/debug/usb/usbmon/1u
```

### 2. MIDI Monitoring

**macOS**: Use MIDI Monitor app
```bash
brew install --cask midi-monitor
```

**Linux**: Use `aseqdump`
```bash
aseqdump -p "Mikro MK3"
```

### 3. Logging Strategy

```rust
// Add debug logging at key points
println!("MIDI: Note {} velocity {} channel {}", note, velocity, channel);
println!("Profile: Page {} pad {}", page, pad_index);
println!("LED: Position {} color {} brightness {}", led_pos, color, brightness);
println!("HID: Wrote {} bytes", result);
```

**Environment variable control**:
```bash
# Enable debug logs
RUST_LOG=debug cargo run

# Or use log crate
RUST_LOG=midimon=debug cargo run
```

### 4. Visual Testing Patterns

Create test patterns to verify LED mapping:

```rust
// Test 1: Single pad sweep
for i in 0..16 {
    clear_all_leds();
    set_led(i, Color::White, Brightness::Bright);
    thread::sleep(Duration::from_millis(500));
}

// Test 2: Corners only
for corner in [0, 3, 12, 15] {
    set_led(corner, Color::Red, Brightness::Bright);
}

// Test 3: Rows
for row in 0..4 {
    for col in 0..4 {
        let led = row * 4 + col;
        set_led(led, RAINBOW[row], Brightness::Normal);
    }
}
```

## Common Gotchas

1. **Off-by-one errors**: Pad indices are 0-15, not 1-16
2. **Row-major vs column-major**: Grid math must be consistent
3. **Byte order**: HID buffer uses specific byte positions
4. **Timing**: HID writes may need small delays between updates
5. **State management**: Track which LEDs are lit to avoid redundant writes
6. **Profile caching**: Reload profile if mappings change
7. **Page switching**: Reset state when page changes
8. **Error propagation**: Don't panic on HID errors, log and continue

## Quick Diagnostic Checklist

When LEDs aren't working correctly:

- [ ] HID device opens successfully
- [ ] MIDI notes are received
- [ ] Profile loads without errors
- [ ] Note-to-pad mapping returns valid indices
- [ ] Pad-to-LED mapping applied
- [ ] LED encoding correct (color index << 2 | brightness)
- [ ] Buffer offset correct (39-54 for pads)
- [ ] Report ID set (0x80 for MK3)
- [ ] HID write succeeds (returns 81)
- [ ] Animation loop running (for animated patterns)
- [ ] Fade-out logic working (for reactive mode)

---

**Last Updated**: November 10, 2025
**Tested On**: macOS Sonoma 14.x, Maschine Mikro MK3 hardware
