# Maschine Mikro MK3 LED System

## Overview

This document details the complete LED feedback system implementation for the Native Instruments Maschine Mikro MK3 controller, including the critical coordinate system mapping required for correct LED addressing.

## Hardware Architecture

### HID Communication

- **Report ID**: `0x80`
- **Buffer Size**: 81 bytes total
  - Byte 0: Report ID (0x80)
  - Bytes 1-38: Unknown/reserved
  - **Bytes 39-54**: 16 pad LED states (one byte per pad)
  - Bytes 55-80: Unknown/reserved

### LED Encoding

Each pad LED uses a single byte with bit-packed encoding:

```rust
led_byte = (color_index << 2) | (brightness & 0b11)
```

**Brightness Levels** (bits 0-1):
- `0b00` (0): Off
- `0b01` (1): Dim
- `0b10` (2): Normal
- `0b11` (3): Bright

**Color Indices** (bits 2-7):
- `1`: Red
- `2`: Orange
- `5`: Yellow
- `7`: Green
- `11`: Blue
- `13`: Purple
- `14`: Magenta
- `17`: White

Example encodings:
- `0x1C` (28) = Red at Normal brightness: `(1 << 2) | 2 = 0b011100`
- `0x46` (70) = White at Normal brightness: `(17 << 2) | 2 = 0b1000110`
- `0x2D` (45) = Green at Dim brightness: `(7 << 2) | 1 = 0b0101101`

## Critical Finding: Coordinate System Mismatch

### The Problem

The MK3 has a **fundamental mismatch** between logical pad numbering (from the device profile) and physical LED positions (hardware addressing). This manifests as a vertical flip in LED addressing.

### Coordinate Systems

**Profile Numbering** (Bottom-to-Top):
```
Pad Grid (Logical):
12  13  14  15    <- Top row (row 3)
 8   9  10  11
 4   5   6   7
 0   1   2   3    <- Bottom row (row 0)
```

**Hardware LED Addressing** (Top-to-Bottom):
```
LED Buffer (Physical):
 0   1   2   3    <- Top row (buffer offset 0-3)
 4   5   6   7
 8   9  10  11
12  13  14  15    <- Bottom row (buffer offset 12-15)
```

### The Solution: Coordinate Mapping

The `map_pad_to_led_position()` function applies a vertical flip transformation:

```rust
fn map_pad_to_led_position(pad_index: u8) -> u8 {
    let row = pad_index / 4;        // Extract row (0-3 bottom-up)
    let col = pad_index % 4;        // Extract column (0-3 left-right)
    let flipped_row = 3 - row;      // Flip vertically
    flipped_row * 4 + col           // Compute LED position
}
```

**Mapping Table**:
```
Pad Index -> LED Position
    0 (bottom-left)   -> 12 (bottom-left)
    1                 -> 13
    2                 -> 14
    3 (bottom-right)  -> 15 (bottom-right)
    4                 ->  8
    5                 ->  9
    6                 -> 10
    7                 -> 11
    8                 ->  4
    9                 ->  5
   10                 ->  6
   11                 ->  7
   12 (top-left)      ->  0 (top-left)
   13                 ->  1
   14                 ->  2
   15 (top-right)     ->  3 (top-right)
```

**Key Insight**: The corners (top-left, top-right, bottom-left, bottom-right) stay in the same visual positions, but the indexing is inverted vertically. This is why initial testing seemed partially correct but had the vertical axis backwards.

## Implementation Architecture

### Core Components

1. **`src/mikro_leds.rs`** (326 lines)
   - `MikroLedController`: HID device management
   - `map_pad_to_led_position()`: Coordinate transformation
   - `set_pad_indexed()`: Individual LED control with mapping
   - Effect functions: All lighting patterns

2. **`src/feedback.rs`** (192 lines)
   - `LightingScheme` enum: Pattern selection
   - `run_scheme()`: Pattern dispatcher
   - Velocity-to-color mapping for reactive mode

3. **`src/main.rs`** (623 lines)
   - Animation loop: 100ms update interval for non-reactive patterns
   - MIDI event handling with profile integration
   - Fade-out timing: 1000ms delay

4. **`src/device_profile.rs`** (267 lines)
   - Profile XML parsing (`.ncmm3` format)
   - `note_to_pad_index()`: MIDI note → pad index conversion
   - Page detection and switching (8 pages: A-H)

### Data Flow

```
MIDI Note Input
    ↓
profile.note_to_pad_index()
    ↓
Pad Index (0-15, bottom-up)
    ↓
map_pad_to_led_position()
    ↓
LED Position (0-15, top-down)
    ↓
buffer[39 + led_pos]
    ↓
HID Write (Report 0x80)
    ↓
Physical LED
```

## Lighting Patterns

All patterns use the LED position mapping to ensure correct display.

### Reactive Mode (Default)

Responds to pad presses with velocity-based colors:

- **Soft** (velocity < 50): Green Dim (`0x1D`)
- **Medium** (50 ≤ velocity < 100): Yellow Normal (`0x16`)
- **Hard** (velocity ≥ 100): Red Bright (`0x07`)

Fade-out: 1000ms delay, then clear LED

### Animated Patterns

Update every 100ms via animation loop:

**Rainbow**: Static rainbow gradient
```rust
colors = [Red, Orange, Orange, Yellow, Yellow, Green, Green, Blue, 
          Blue, Blue, Purple, Purple, Magenta, Magenta, White, White]
```

**Sparkle**: Random white LEDs (20% probability)
- Generates new random pattern each frame
- Creates twinkling effect

**Wave**: Blue with brightness gradient
```rust
brightness[i] = (i % 4) as u8  // 0-3 brightness levels
```

**VU Meter**: Green→Yellow→Orange→Red gradient
```rust
Row 0-1: Green, Row 2: Yellow/Orange, Row 3: Red
```

**Spiral**: Purple/Magenta/Blue diagonal pattern

### Static Patterns

**Breathing**: All pads Blue Dim (`0x2D`)

**Pulse**: All pads Cyan Normal (Blue + Green mixing via alternating)

## Configuration

### CLI Arguments

```bash
# Reactive mode (default)
./midimon --profile "path/to/profile.ncmm3"

# Specific lighting pattern
./midimon --led rainbow --profile "..."
./midimon --lighting sparkle --profile "..."  # Alias works too

# Available schemes
rainbow, breathing, pulse, wave, sparkle, vumeter, spiral, reactive, off, static
```

### Profile Integration

The MK3 uses `.ncmm3` XML profiles with 8 pad pages (A-H). Each page maps 16 MIDI notes to the 16 pads. The system automatically:

1. Detects which page is active based on incoming MIDI notes
2. Maps MIDI notes to pad indices (0-15)
3. Applies LED position transformation
4. Sends HID updates

**Critical**: Always use pad indices from profile mapping, never raw MIDI notes for LED addressing.

## Development Learnings

### HID Access on macOS

**Problem**: Default `hidapi` on macOS cannot access devices already in use by other applications (e.g., NI Controller Editor).

**Solution**: Enable the `macos-shared-device` feature:
```toml
[dependencies]
hidapi = { version = "2.4", features = ["macos-shared-device"] }
```

This uses `IOHIDDeviceOpen` with `kIOHIDOptionsTypeSeizeDevice` flag to allow concurrent access.

### Why the Vertical Flip Exists

The mismatch likely stems from different design perspectives:

1. **Profile/Software View**: Musicians think bottom-to-top (like piano keys, lower to higher)
2. **Hardware/Engineering View**: Buffer addressing naturally goes top-to-bottom (like reading order)

NI's profiles use the musician-friendly bottom-up numbering, while the hardware uses standard top-down buffer addressing.

### Debugging Techniques

1. **Test patterns**: Start with single-LED tests to verify addressing
   ```rust
   // Light one pad at a time to verify mapping
   for i in 0..16 {
       set_pad_indexed(i, Color::White, Brightness::Bright);
       sleep(500ms);
   }
   ```

2. **Corner verification**: Check corners match visual positions
   - Pad 0 should be bottom-left
   - Pad 15 should be top-right

3. **Pattern validation**: Use distinctive patterns (rainbow, vumeter) to spot coordinate issues

4. **Log HID writes**: Print buffer contents to verify encoding
   ```rust
   println!("LED UPDATE: Writing {} bytes (pads: {:?})", 
            len, &buffer[39..55]);
   ```

## Performance Considerations

- **Update Rate**: 100ms (10 FPS) for animated patterns is smooth enough while not overloading HID
- **Fade-out Delay**: 1000ms provides good visual feedback without LEDs staying lit too long
- **Buffer Reuse**: Single 81-byte buffer reused for all writes to minimize allocations

## Future Enhancements

Possible improvements:

1. **Brightness Control**: Add per-pattern brightness scaling
2. **Custom Patterns**: Load patterns from config file
3. **Audio Reactivity**: Sync patterns to audio input level
4. **Multi-page Support**: Different patterns per pad page
5. **Transition Effects**: Smooth color transitions between patterns

## References

- Original LED protocol discovered from: [r00tman/maschine-mikro-mk3-driver](https://github.com/r00tman/maschine-mikro-mk3-driver)
- NI Controller Editor: `.ncmm3` format (XML-based device profiles)
- hidapi documentation: [libusb/hidapi](https://github.com/libusb/hidapi)

## Testing Checklist

When modifying LED code, verify:

- [ ] All 16 pads light correctly in reactive mode
- [ ] Corner LEDs are in correct visual positions
- [ ] Fade-out timing works (1000ms)
- [ ] All lighting patterns display correctly
- [ ] Animation is smooth (no flickering)
- [ ] HID writes succeed (check for errors)
- [ ] Works with Controller Editor running (shared access)
- [ ] Profile auto-detection works for all pages

---

**Last Updated**: November 10, 2025
**Implementation Status**: ✅ Complete and fully functional
