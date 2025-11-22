# Diagnostic Tools and Procedures

## Overview

Conductor includes a comprehensive suite of diagnostic tools for debugging connectivity, event processing, LED control, and configuration issues. This guide covers each tool in detail and provides systematic troubleshooting procedures.

## Quick Diagnostic Checklist

Before diving deep, run through this quick checklist:

```bash
# 1. Check USB/MIDI connectivity
cargo run --bin test_midi

# 2. Verify MIDI events are received
cargo run --bin midi_diagnostic 2

# 3. Test LED/HID access
cargo run --bin led_diagnostic

# 4. Find note numbers
cargo run --bin pad_mapper 2

# 5. Enable debug logging
DEBUG=1 cargo run --release 2
```

If all five succeed, the hardware and drivers are working correctly.

## Diagnostic Tool Reference

### 1. test_midi - MIDI Port Testing

**Purpose**: Verify MIDI connectivity, enumerate all available ports, test basic MIDI communication.

**Syntax**:
```bash
cargo run --bin test_midi
```

**What it does**:
1. Lists all MIDI input ports
2. Lists all MIDI output ports
3. Attempts to open port 2 (default)
4. Waits for a MIDI event (5-second timeout)
5. Reports success or failure

**Expected output** (working):
```
MIDI Port Test
==============

Available input ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Input
3: Digital Keyboard

Available output ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Output

Testing port 2 (input)...
✓ Successfully opened port: Maschine Mikro MK3 - Input

Waiting for MIDI event... (5 second timeout)
✓ Received MIDI event: NoteOn ch:0 note:12 vel:87

Connection test: PASSED
All MIDI functionality working correctly.
```

**Error output** (device not found):
```
Available input ports:
0: IAC Driver Bus 1
# Device missing!

Testing port 2 (input)...
✗ Error: Port index 2 out of range (only 1 ports available)

Connection test: FAILED
```

**Interpreting results**:

- **Device not listed**: USB/driver issue (see [Common Issues - MIDI Device Not Found](common-issues.md#midi-device-not-found))
- **Timeout on event**: Device connected but not sending data
  - Check device is in MIDI mode (not HID-only mode)
  - Verify pads/keys are functional
  - Try different port number
- **"Permission denied"**: User permission issue (Linux)
  - Add user to `audio` or `plugdev` group
  - Check udev rules

**When to use**:
- First step in any MIDI troubleshooting
- After connecting a new device
- After driver installation
- Verifying port numbers before running conductor

### 2. midi_diagnostic - MIDI Event Monitor

**Purpose**: Real-time visualization of all MIDI events for debugging event detection and mapping issues.

**Syntax**:
```bash
cargo run --bin midi_diagnostic <PORT>

# Example
cargo run --bin midi_diagnostic 2
```

**What it does**:
- Opens specified MIDI port
- Listens for all MIDI events
- Displays events in human-readable format
- Shows channel, note/cc number, velocity/value
- Runs until Ctrl+C

**Output format**:
```
Connected to MIDI port 2: Maschine Mikro MK3 - Input
Listening for MIDI events... (Ctrl+C to exit)

[16:32:45] [NoteOn]  ch:0 note:12 vel:87
[16:32:45] [NoteOff] ch:0 note:12 vel:0
[16:32:46] [NoteOn]  ch:0 note:13 vel:64
[16:32:46] [NoteOff] ch:0 note:13 vel:0
[16:32:47] [CC]      ch:0 cc:1 value:64
[16:32:48] [PitchBend] ch:0 value:8192
[16:32:49] [Aftertouch] ch:0 note:12 pressure:48
```

**Event types shown**:

- **NoteOn**: Note pressed
  - `ch`: MIDI channel (0-15)
  - `note`: Note number (0-127)
  - `vel`: Velocity (0-127)

- **NoteOff**: Note released
  - `vel` is usually 0, some devices use release velocity

- **CC** (Control Change): Knob, slider, encoder
  - `cc`: Controller number (0-127)
  - `value`: Value (0-127)

- **PitchBend**: Touch strip, pitch wheel
  - `value`: Bend amount (0-16383, center=8192)

- **Aftertouch**: Pressure sensitivity
  - `note`: Which note (if polyphonic)
  - `pressure`: Pressure value (0-127)

- **ProgramChange**: Program/patch change
  - `program`: Program number (0-127)

**Use cases**:

1. **Verify MIDI events are being sent**:
   ```bash
   cargo run --bin midi_diagnostic 2
   # Press pads - you should see NoteOn/NoteOff events
   ```

2. **Find note numbers for config**:
   ```bash
   cargo run --bin midi_diagnostic 2
   # Press pad → note:12 → use 12 in config.toml
   ```

3. **Debug why mappings aren't triggering**:
   ```bash
   # Compare MIDI diagnostic output with config.toml
   # If you see note:25 but config has note:12, they don't match
   ```

4. **Verify velocity ranges**:
   ```bash
   cargo run --bin midi_diagnostic 2
   # Soft tap:  vel:32  (0-40 range)
   # Medium:    vel:68  (41-80 range)
   # Hard hit:  vel:105 (81-127 range)
   ```

5. **Check encoder/knob CC numbers**:
   ```bash
   cargo run --bin midi_diagnostic 2
   # Turn encoder → [CC] ch:0 cc:1 value:65
   # Use cc:1 in EncoderTurn trigger
   ```

**Troubleshooting with midi_diagnostic**:

**Problem**: No events appear when pressing pads

**Solutions**:
- Wrong port number (try 0, 1, 2, 3, etc.)
- Device not sending MIDI (check device settings)
- USB cable issue (try different cable)
- Driver issue (reinstall drivers)

**Problem**: Wrong note numbers appearing

**Cause**: Device is on different pad page or has custom profile

**Solution**: Either:
- Change device to expected page
- Update config.toml with actual note numbers
- Use `--profile` flag with correct profile

**Problem**: Events appear but with unexpected values

**Example**: Expecting ch:0 but seeing ch:1

**Solution**: Update trigger channel in config:
```toml
[trigger]
type = "Note"
note = 12
channel = 1  # Add channel specification
```

### 3. led_diagnostic - LED/HID Testing

**Purpose**: Test LED control and HID device access for troubleshooting lighting issues.

**Syntax**:
```bash
cargo run --bin led_diagnostic
```

**What it does**:
1. Searches for HID device (Maschine Mikro MK3)
2. Attempts to open HID device
3. Tests LED control by cycling through all pads
4. Displays each step's success/failure

**Expected output** (working):
```
LED Diagnostic Tool
==================

Searching for Maschine Mikro MK3...
✓ Device found: Maschine Mikro MK3 (VID:17cc PID:1600)
  Serial: XXXXXXXX
  Manufacturer: Native Instruments
  Product: Maschine Mikro MK3

Opening HID device...
✓ HID device opened successfully
✓ Device supports shared access

Testing LED control...
- Lighting pad 0 (Red Bright)... ✓
- Lighting pad 1 (Orange Bright)... ✓
- Lighting pad 2 (Yellow Bright)... ✓
- Lighting pad 3 (Green Bright)... ✓
- Lighting pad 4 (Blue Bright)... ✓
- Lighting pad 5 (Purple Bright)... ✓
[... continues for all 16 pads ...]

Testing patterns...
✓ Rainbow pattern
✓ All pads lit (White)
✓ All pads cleared

LED Diagnostic: PASSED
All LED functionality working correctly.
```

**Error output** (permission denied - macOS):
```
Searching for Maschine Mikro MK3...
✓ Device found: Maschine Mikro MK3 (VID:17cc PID:1600)

Opening HID device...
✗ Failed to open HID device
  Error: Permission denied (os error 13)

  Possible causes:
  1. Input Monitoring permission not granted
  2. Device already in exclusive use
  3. Native Instruments drivers not installed

  Solutions:
  1. Grant Input Monitoring permission:
     System Settings → Privacy & Security → Input Monitoring
     Enable Terminal or conductor

  2. Close other applications using the device:
     - Native Instruments Controller Editor
     - Maschine software
     - Other MIDI software

  3. Install NI drivers:
     Download Native Access and install Maschine drivers

LED Diagnostic: FAILED
```

**Error output** (device not found):
```
Searching for Maschine Mikro MK3...
✗ Device not found

Checked for:
- Vendor ID: 0x17cc (Native Instruments)
- Product ID: 0x1600 (Maschine Mikro MK3)

Possible causes:
1. Device not connected
2. Wrong USB port or cable
3. Device powered off
4. HID driver not installed

Solutions:
1. Check USB connection:
   system_profiler SPUSBDataType | grep -i mikro

2. Try different USB port/cable

3. Power cycle device (unplug, wait 10s, replug)

LED Diagnostic: FAILED
```

**Interpreting results**:

- **Device found, opened, LEDs working**: All HID/LED functionality OK
- **Device found but won't open**: Permission or driver issue
- **Device not found**: USB/hardware issue
- **Opens but LEDs don't light**: Coordinate mapping or hardware issue

**When to use**:
- LEDs not working in main application
- After granting Input Monitoring permission (verify it worked)
- After installing NI drivers (verify driver installation)
- Testing LED hardware functionality
- Before reporting LED bugs

### 4. led_tester - Interactive LED Control

**Purpose**: Manual control of individual LEDs for testing coordinate mapping and hardware.

**Syntax**:
```bash
cargo run --bin led_tester
```

**Interactive mode**:
```
LED Tester - Interactive Mode
==============================
Device: Maschine Mikro MK3 (VID:17cc PID:1600)
Status: Connected

Commands:
  on <pad> <color> <brightness>   Turn on specific LED
  off <pad>                       Turn off specific LED
  all <color> <brightness>        Set all LEDs
  clear                           Clear all LEDs
  rainbow                         Show rainbow pattern
  test                            Cycle through all pads
  coords                          Show pad coordinate mapping
  help                            Show this help
  quit                            Exit

Pad numbers: 0-15
Colors: red, orange, yellow, green, blue, purple, magenta, white
Brightness: 0 (off), 1 (dim), 2 (normal), 3 (bright)

>
```

**Example session**:
```
> on 0 red 3
✓ Pad 0: Red Bright

> on 1 green 2
✓ Pad 1: Green Normal

> all blue 1
✓ All pads: Blue Dim

> rainbow
✓ Rainbow pattern displayed

> test
Testing all pads (press Ctrl+C to stop)...
Pad 0... Pad 1... Pad 2... [continues]

> coords
Pad Coordinate Mapping:
Pad Index -> LED Position

Physical Layout (bottom-up):
12  13  14  15  <- Top row
 8   9  10  11
 4   5   6   7
 0   1   2   3  <- Bottom row

LED Buffer (top-down):
 0   1   2   3  <- Top (buffer 0-3)
 4   5   6   7
 8   9  10  11
12  13  14  15  <- Bottom (buffer 12-15)

Mapping Table:
  Pad  0 -> LED 12    Pad  8 -> LED  4
  Pad  1 -> LED 13    Pad  9 -> LED  5
  Pad  2 -> LED 14    Pad 10 -> LED  6
  Pad  3 -> LED 15    Pad 11 -> LED  7
  Pad  4 -> LED  8    Pad 12 -> LED  0
  Pad  5 -> LED  9    Pad 13 -> LED  1
  Pad  6 -> LED 10    Pad 14 -> LED  2
  Pad  7 -> LED 11    Pad 15 -> LED  3

> clear
✓ All LEDs cleared

> quit
Goodbye!
```

**Use cases**:

1. **Test individual pad LEDs**:
   ```
   > on 0 white 3
   # Bottom-left pad should light up bright white
   ```

2. **Verify coordinate mapping**:
   ```
   > coords
   # Shows physical layout vs LED buffer
   > on 0 red 3
   # Verify bottom-left pad lights (should be LED buffer position 12)
   ```

3. **Test all colors**:
   ```
   > on 0 red 3
   > on 1 orange 3
   > on 2 yellow 3
   > on 3 green 3
   > on 4 blue 3
   > on 5 purple 3
   ```

4. **Brightness testing**:
   ```
   > on 0 white 0   # Off
   > on 0 white 1   # Dim
   > on 0 white 2   # Normal
   > on 0 white 3   # Bright
   ```

**When to use**:
- Debugging coordinate mapping issues
- Testing specific pad LEDs (identify dead LEDs)
- Understanding LED coordinate system
- Verifying color/brightness encoding

### 5. pad_mapper - Note Number Discovery

**Purpose**: Identify MIDI note numbers for physical pads to use in config.toml.

**Syntax**:
```bash
cargo run --bin pad_mapper <PORT>

# Example
cargo run --bin pad_mapper 2
```

**What it does**:
- Opens MIDI port
- Listens for NoteOn events
- Displays note number and velocity for each press
- Continues until Ctrl+C

**Output**:
```
Pad Mapper - Find Note Numbers for Your Controller
===================================================
Connected to port 2: Maschine Mikro MK3 - Input

Press pads in order (bottom-left to top-right) and write down note numbers.
Ctrl+C to exit when done.

Pad pressed: Note 12 (velocity: 87)
Pad pressed: Note 13 (velocity: 64)
Pad pressed: Note 14 (velocity: 92)
Pad pressed: Note 15 (velocity: 73)
Pad pressed: Note 8  (velocity: 55)
Pad pressed: Note 9  (velocity: 81)
[continues as you press pads...]
```

**Recommended workflow**:

1. **Run pad_mapper**:
   ```bash
   cargo run --bin pad_mapper 2
   ```

2. **Draw a grid** on paper:
   ```
   [ ] [ ] [ ] [ ]  <- Top row
   [ ] [ ] [ ] [ ]
   [ ] [ ] [ ] [ ]
   [ ] [ ] [ ] [ ]  <- Bottom row
   ```

3. **Press each pad** starting bottom-left, moving right:
   - Press bottom-left → write down note number (e.g., 12)
   - Press next pad right → write down note number (e.g., 13)
   - Continue for all 16 pads

4. **Result**:
   ```
   [12][13][14][15]  <- Top row
   [ 8][ 9][10][11]
   [ 4][ 5][ 6][ 7]
   [ 0][ 1][ 2][ 3]  <- Bottom row
   ```

   Wait, those look wrong! That's because your device might be on a different page or have a custom profile. The numbers you write down are the ones to use in config.toml.

5. **Update config.toml** with actual note numbers:
   ```toml
   [[modes.mappings]]
   description = "Bottom-left pad"
   [modes.mappings.trigger]
   type = "Note"
   note = 0  # Or whatever number you wrote down
   ```

**Use cases**:
- Initial setup of a new device
- Creating config.toml for the first time
- Verifying note numbers after changing device profile
- Debugging why specific pads don't work

**Tips**:
- Press pads gently and consistently (velocity affects detection)
- Write down numbers immediately (easy to forget)
- Take a photo of your paper grid for reference
- Some controllers send different notes on different "pages" or "banks"

## Advanced Debugging with DEBUG=1

Enable verbose debug logging to see internal event processing:

**Syntax**:
```bash
DEBUG=1 cargo run --release 2
```

### Debug Output Format

**MIDI Events**:
```
[16:32:45.123] [MIDI] NoteOn ch:0 note:12 vel:87
[16:32:45.124] [MIDI] NoteOff ch:0 note:12 vel:0
```

**Event Processing**:
```
[DEBUG] Processing NoteOn: note=12 vel=87
[DEBUG] Detected velocity range: Medium (41-80)
[DEBUG] Stored note 12 in press tracker (time: 16:32:45.123)
```

**Mapping Engine**:
```
[DEBUG] Checking 24 mappings in mode 'Default'
[DEBUG] Matched mapping: "Copy text" (trigger: Note(12))
[DEBUG] Compiled action: Keystroke { keys: "c", modifiers: ["cmd"] }
```

**Action Execution**:
```
[DEBUG] Executing Keystroke action
[DEBUG] Pressing key: C
[DEBUG] Holding modifiers: [Cmd]
[DEBUG] Releasing key: C
✓ Action executed successfully (1.2ms)
```

**LED Updates**:
```
[DEBUG] LED update request: pad=0 color=Green brightness=2
[DEBUG] Mapped pad 0 -> LED position 12
[DEBUG] HID buffer: [80 00 00 ... 1D ... 00] (81 bytes)
[DEBUG] HID write successful (4 bytes written)
```

**Mode Changes**:
```
[DEBUG] Mode change requested: next
[DEBUG] Current mode: 0 (Default)
[DEBUG] Switching to mode: 1 (Developer)
[INFO] Mode changed: Default -> Developer
```

**Timers and State**:
```
[DEBUG] Long press timer started: note=12 threshold=2000ms
[DEBUG] Note released after 1523ms (threshold: 2000ms)
[DEBUG] Long press NOT triggered (held too short)
```

```
[DEBUG] Double-tap window started: note=12 timeout=300ms
[DEBUG] Second tap detected within 287ms
[DEBUG] Double-tap triggered: note=12
```

```
[DEBUG] Chord detection: notes=[12, 13, 14] timeout=100ms
[DEBUG] All notes pressed within 47ms
[DEBUG] Chord triggered: [12, 13, 14]
```

### Interpreting Debug Output

#### Problem: Mapping not triggering

**Look for**:
```
[DEBUG] No mapping matched for event: Note(12)
```

**Possible causes**:
- Note number mismatch (check pad_mapper output)
- Wrong mode (check current mode in debug output)
- Trigger conditions not met (velocity, timing)

**Solution**: Compare debug output with config.toml

---

#### Problem: Action not executing

**Look for**:
```
[DEBUG] Matched mapping: "Test"
[ERROR] Failed to execute action: Command not found
```

**Cause**: Action configuration error (e.g., invalid shell command)

**Solution**: Fix action in config.toml

---

#### Problem: LEDs not updating

**Look for**:
```
[DEBUG] LED update request: pad=0
[ERROR] HID write failed: Device not open
```

**Cause**: HID device not accessible

**Solution**: Check permissions, run led_diagnostic

---

## USB Connection Verification

### macOS

**Check USB device enumeration**:
```bash
# List all USB devices
system_profiler SPUSBDataType

# Filter for your device
system_profiler SPUSBDataType | grep -i mikro

# More detailed output
system_profiler SPUSBDataType | grep -B 10 -A 10 "Mikro"
```

**Expected output**:
```
Maschine Mikro MK3:
  Product ID: 0x1600
  Vendor ID: 0x17cc  (Native Instruments)
  Version: 1.00
  Serial Number: XXXXXXXX
  Speed: Up to 12 Mb/s
  Manufacturer: Native Instruments
  Location ID: 0x14200000 / 5
  Current Available (mA): 500
  Current Required (mA): 100
  Extra Operating Current (mA): 0
```

**Check for device disappearance**:
```bash
# Monitor USB devices
log stream --predicate 'eventMessage contains "USB"' --level info
```

Then plug/unplug device - you should see connection events.

### Linux

**Check USB devices**:
```bash
# List USB devices
lsusb

# Filter for Native Instruments (VID: 17cc)
lsusb | grep 17cc

# Detailed info
lsusb -v -d 17cc:1600
```

**Check dmesg for USB events**:
```bash
# Show recent USB events
dmesg | grep -i usb | tail -20

# Monitor in real-time
dmesg -w | grep -i usb
```

**Check ALSA MIDI**:
```bash
# List ALSA MIDI devices
aconnect -l

# Or
amidi -l
```

### Windows

**Device Manager**:
1. Open Device Manager (Win+X → Device Manager)
2. Expand "Sound, video and game controllers"
3. Look for your MIDI device
4. Right-click → Properties → Check status

**Check MIDI devices via PowerShell**:
```powershell
Get-PnpDevice -Class MEDIA | Format-Table -AutoSize
```

## Interpreting Error Messages

### Common Error Patterns

#### "No MIDI input ports available"

**Meaning**: No MIDI devices detected at all

**Debug steps**:
1. `system_profiler SPUSBDataType | grep -i usb`
2. `cargo run --bin test_midi`
3. Check USB connection
4. Verify drivers installed

---

#### "Failed to open HID device"

**Meaning**: Device found but can't access HID interface

**Debug steps**:
1. Check permissions (Input Monitoring on macOS)
2. `cargo run --bin led_diagnostic`
3. Close other applications using device
4. Reinstall NI drivers

---

#### "TOML parse error at line X"

**Meaning**: Syntax error in config.toml

**Debug steps**:
1. Open config.toml in editor
2. Go to line X
3. Check for:
   - Missing quotes
   - Wrong brackets
   - Typos in field names
4. Validate at https://www.toml-lint.com/

---

#### "Missing field 'type' in trigger"

**Meaning**: Invalid config structure

**Debug steps**:
1. Find the mapping missing `type`
2. Add `type` field:
   ```toml
   [trigger]
   type = "Note"  # Add this
   note = 12
   ```

---

#### "Permission denied (os error 13)"

**Meaning**: Insufficient permissions

**macOS**: Grant Input Monitoring permission

**Linux**: Add user to `plugdev` group and create udev rules

**Windows**: Run as Administrator (not recommended for regular use)

---

## Systematic Troubleshooting Procedure

When something doesn't work, follow this systematic approach:

### Level 1: Hardware Connectivity

```bash
# 1. Check USB enumeration
system_profiler SPUSBDataType | grep -i mikro

# 2. Test MIDI ports
cargo run --bin test_midi

# 3. Monitor MIDI events
cargo run --bin midi_diagnostic 2
```

**If Level 1 fails**: Hardware or driver issue (see [Common Issues](common-issues.md))

### Level 2: Software Connectivity

```bash
# 4. Test HID access
cargo run --bin led_diagnostic

# 5. Find note numbers
cargo run --bin pad_mapper 2

# 6. Run with debug logging
DEBUG=1 cargo run --release 2
```

**If Level 2 fails**: Permission or configuration issue

### Level 3: Configuration Validation

```bash
# 7. Validate config syntax
cargo check

# 8. Test with minimal config
cat > test.toml << 'EOF'
[[modes]]
name = "Test"
[[modes.mappings]]
description = "Test"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Shell"
command = "say test"
EOF

cargo run --release 2 --config test.toml

# 9. Compare note numbers
# pad_mapper output vs config.toml
```

**If Level 3 fails**: Config syntax or mapping issue

### Level 4: Deep Debugging

```bash
# 10. Enable trace-level logging
RUST_LOG=trace cargo run --release 2

# 11. Check for resource exhaustion
# Activity Monitor (macOS) / Task Manager (Windows)

# 12. Test on different computer
# (isolates hardware vs software issues)
```

## Performance Profiling

### Response Time Analysis

```bash
DEBUG=1 cargo run --release 2 2>&1 | grep -E 'NoteOn|executed'
```

**Measure latency**:
```
[16:32:45.123456] NoteOn
[16:32:45.124789] Action executed (1.3ms)
```

Latency = 1.3ms (excellent, <5ms is good)

### CPU Usage Monitoring

**macOS**:
```bash
# Monitor CPU in real-time
top | grep conductor

# Or use Activity Monitor GUI
open -a "Activity Monitor"
```

**Expected CPU usage**:
- Idle: <1%
- Active (pad presses): 2-5%
- With animated LEDs: 3-7%

**If >10%**: Performance issue (check debug logging enabled)

### Memory Usage

```bash
# macOS
ps aux | grep conductor

# Expected RSS: 5-10 MB
```

**If >50MB**: Memory leak (file a bug report)

## Logs and Diagnostics Output

### Redirecting Output

**Save all output**:
```bash
cargo run --release 2 > conductor.log 2>&1
```

**Save only errors**:
```bash
cargo run --release 2 2> conductor.err
```

**Separate stdout and stderr**:
```bash
cargo run --release 2 > conductor.out 2> conductor.err
```

### Analyzing Logs

**Count events**:
```bash
grep -c "NoteOn" conductor.log
```

**Find errors**:
```bash
grep -i error conductor.log
grep -i failed conductor.log
```

**Extract timings**:
```bash
grep "Action executed" conductor.log | sed 's/.*(\(.*\)ms).*/\1/'
```

## See Also

- [Common Issues](common-issues.md) - Quick solutions to frequent problems
- [CLI Commands](../reference/cli-commands.md) - Complete command reference
- [Configuration Overview](../configuration/overview.md) - Config syntax validation

---

**Last Updated**: November 11, 2025
**Diagnostic Tool Version**: 0.1.0
