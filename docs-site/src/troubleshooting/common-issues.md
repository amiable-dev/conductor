# Common Issues and Solutions

## Overview

This guide covers the most frequently encountered issues when using Conductor, along with step-by-step solutions. For detailed diagnostic procedures, see [Diagnostics Guide](diagnostics.md).

## MIDI Device Not Found

### Symptoms

```
Error: No MIDI input ports available
```

Or:

```
Available MIDI input ports:
0: IAC Driver Bus 1
# Your device is missing from the list
```

### Causes

- USB cable not connected
- Device not powered on
- MIDI driver not installed (macOS/Windows)
- Device in wrong mode (some controllers have multiple modes)
- MIDI port disabled in system settings

### Solutions

#### 1. Check Physical Connection

```bash
# macOS: Check USB device enumeration
system_profiler SPUSBDataType | grep -i mikro
# or
system_profiler SPUSBDataType | grep -i midi

# Expected output:
# Maschine Mikro MK3:
#   Product ID: 0x1600
#   Vendor ID: 0x17cc (Native Instruments)
```

**If device not found**:
- Try a different USB port
- Try a different USB cable
- Power cycle the device (unplug, wait 10 seconds, replug)
- Check device is powered on (some controllers have power switches)

#### 2. Verify MIDI Setup (macOS)

```bash
# Open Audio MIDI Setup
open -a "Audio MIDI Setup"
```

In the **MIDI Studio** window (Window → Show MIDI Studio):
- Verify your device appears
- Check it's not grayed out (indicates active connection)
- If grayed out, right-click and select "Enable"

**Reset MIDI configuration** (if corrupted):
```bash
# Quit Audio MIDI Setup first
rm ~/Library/Preferences/com.apple.audio.AudioMIDISetup.plist
rm ~/Library/Preferences/ByHost/com.apple.audio.AudioMIDISetup.*
# Reopen Audio MIDI Setup
```

#### 3. Install/Reinstall Drivers

**macOS/Windows**:
1. Download [Native Access](https://www.native-instruments.com/en/support/downloads/)
2. Sign in (free account)
3. Install drivers for your device
4. Restart computer after installation
5. Reconnect device

**Linux**:
- Most MIDI controllers work with built-in ALSA drivers
- Install `alsa-utils`: `sudo apt install alsa-utils`
- List devices: `aconnect -l`

#### 4. Test with System Tools

**macOS**:
```bash
# List MIDI devices using system_profiler
system_profiler SPUSBDataType | grep -B 10 -A 10 MIDI

# Test with conductor diagnostic
cargo run --bin test_midi
```

**Linux**:
```bash
# List ALSA MIDI devices
aconnect -l

# Test with amidi
amidi -l
```

#### 5. Check Port Numbers

```bash
# List all available ports
cargo run --release

# Try each port number
cargo run --bin midi_diagnostic 0
cargo run --bin midi_diagnostic 1
cargo run --bin midi_diagnostic 2
# ... press a pad/key on your controller to verify connection
```

### Still Not Working?

1. Try a different computer (to rule out hardware failure)
2. Test with manufacturer's software (e.g., NI Controller Editor)
3. Check for firmware updates
4. Contact manufacturer support

## LEDs Not Working

### Symptoms

- Pads press correctly but LEDs don't light up
- LEDs flash once then go dark
- Wrong pads lighting up
- LEDs stuck on or flickering

### Causes

- Input Monitoring permission not granted (macOS)
- HID driver not installed
- Profile/coordinate mapping issues
- HID device already in use by another application
- Incorrect LED scheme selected

### Solutions

#### 1. Grant Input Monitoring Permission (macOS)

**Step-by-step**:

1. Run Conductor:
   ```bash
   cargo run --release 2
   ```

2. macOS will show a permission dialog

3. Click **Open System Settings**

4. In **Privacy & Security** → **Input Monitoring**:
   - Find `conductor` or `Terminal` (if running via cargo)
   - Toggle switch to **ON**
   - If switch is already ON, toggle OFF then ON again

5. Restart Conductor:
   ```bash
   cargo run --release 2
   ```

**Verify permission granted**:
```bash
DEBUG=1 cargo run --release 2
```

Look for:
```
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
```

If you see "Failed to open HID device", permission is not granted.

#### 2. Test LED Hardware

```bash
# Run LED diagnostic tool
cargo run --bin led_diagnostic
```

**Expected output**:
```
✓ Device found: Maschine Mikro MK3
✓ HID device opened successfully
✓ Testing LED control...
✓ LED diagnostic complete
```

**Error output**:
```
✗ Failed to open HID device
```

If this fails, see [Diagnostics Guide](diagnostics.md) for HID troubleshooting.

#### 3. Install/Verify Native Instruments Drivers

**macOS/Windows**:
1. Open Native Access
2. Verify "Maschine" or controller-specific software is installed
3. Check for updates
4. Reinstall if necessary
5. Restart computer

**Test after driver installation**:
```bash
cargo run --bin led_diagnostic
```

#### 4. Try Different LED Schemes

Some schemes might not work due to profile issues:

```bash
# Try rainbow (doesn't require note mapping)
cargo run --release 2 --led rainbow

# Try static
cargo run --release 2 --led static

# Try reactive (default)
cargo run --release 2 --led reactive
```

If rainbow/static work but reactive doesn't, it's a profile/mapping issue.

#### 5. Fix Coordinate Mapping Issues

**Wrong pads lighting up** indicates coordinate mapping problems.

**Solution**: Use a device profile:

```bash
# Auto-detect page
cargo run --release 2 --profile path/to/profile.ncmm3

# Force specific page
cargo run --release 2 --profile profile.ncmm3 --pad-page A
```

Create a profile using Native Instruments Controller Editor if you don't have one.

See [Device Profiles Documentation](../DEVICE_PROFILES.md) for complete guide.

#### 6. Check Shared Device Access

If Controller Editor is running simultaneously:

**macOS**: Conductor uses shared device access (should work)

**Verify**:
```bash
# Check Cargo.toml includes:
hidapi = { version = "2.4", features = ["macos-shared-device"] }
```

If LEDs work when Controller Editor is closed but not when it's running:
- Update to latest Conductor version
- Rebuild from source: `cargo build --release`

### Advanced Debugging

Enable debug logging and watch for LED updates:

```bash
DEBUG=1 cargo run --release 2 --led reactive
```

Press pads and look for:
```
[DEBUG] LED update: pad 0 -> color 7 (Green) brightness 2
[DEBUG] HID write: 81 bytes
```

If you see LED updates logged but LEDs don't light:
- Hardware issue (try different USB port)
- Firmware issue (update device firmware)
- Driver issue (reinstall NI drivers)

## Events Not Triggering

### Symptoms

- Pads press but no actions execute
- Some pads work, others don't
- LEDs work but actions don't fire
- Actions trigger randomly or inconsistently

### Causes

- Note numbers don't match config.toml
- Wrong mode active
- Trigger conditions not met (velocity, timing)
- Config syntax errors
- Event processing disabled

### Solutions

#### 1. Verify Note Numbers

```bash
# Find actual note numbers
cargo run --bin pad_mapper 2
```

Press each pad and write down the note numbers.

**Compare with config.toml**:
```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 12  # Must match the actual note number from pad_mapper
```

If they don't match, update config.toml with correct note numbers.

#### 2. Check Active Mode

Conductor prints the current mode to console:

```
Mode changed: Default -> Developer
Currently in mode: Developer
```

**Verify you're in the expected mode**:
- Check encoder hasn't switched modes accidentally
- Verify mode-specific mappings are in the correct mode section
- Try using global mappings for testing

**Force to Default mode**:
```toml
[[global_mappings]]
description = "Reset to default mode"
[global_mappings.trigger]
type = "Note"
note = 0
[global_mappings.action]
type = "ModeChange"
mode = "Default"
```

#### 3. Test with Simple Mapping

Add a simple test mapping to verify basic functionality:

```toml
[[global_mappings]]
description = "Test mapping - Should print to console"
[global_mappings.trigger]
type = "Note"
note = 0  # Bottom-left pad
[global_mappings.action]
type = "Shell"
command = "echo 'Mapping works!' && say 'Mapping works'"
```

If this works, the issue is with specific trigger/action configuration.

#### 4. Enable Debug Logging

```bash
DEBUG=1 cargo run --release 2
```

Press pads and watch for:

**Good output** (mapping working):
```
[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] Matched mapping: "Copy text" (mode: Default)
[DEBUG] Executing action: Keystroke(keys: "c", modifiers: ["cmd"])
✓ Action executed successfully
```

**Bad output** (no match):
```
[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] No mapping matched for event
```

If no mapping matched:
- Note number mismatch
- Wrong mode
- Trigger conditions not met

#### 5. Check Velocity/Timing Requirements

**VelocityRange** triggers require specific velocity:

```toml
[trigger]
type = "VelocityRange"
note = 12
min_velocity = 81  # Only triggers on HARD press (81-127)
max_velocity = 127
```

**Solution**: Press harder or adjust velocity range:
```toml
min_velocity = 0   # Accept any velocity
max_velocity = 127
```

**LongPress** triggers require holding:

```toml
[trigger]
type = "LongPress"
note = 12
hold_duration_ms = 2000  # Must hold for 2 seconds
```

**Solution**: Hold longer or reduce threshold:
```toml
hold_duration_ms = 500  # Only 0.5 seconds
```

**DoubleTap** requires two quick taps:

```toml
[trigger]
type = "DoubleTap"
note = 12
double_tap_timeout_ms = 300  # Must tap twice within 300ms
```

**Solution**: Tap faster or increase timeout:
```toml
double_tap_timeout_ms = 500  # 0.5 seconds between taps
```

#### 6. Validate Config Syntax

```bash
# Check for TOML syntax errors
cargo check

# Or use online validator
# https://www.toml-lint.com/
```

Common syntax errors:
- Missing quotes around strings
- Wrong bracket type (`[]` vs `[[]]`)
- Typos in field names
- Missing required fields

#### 7. Test MIDI Events

Verify your device is sending events:

```bash
cargo run --bin midi_diagnostic 2
```

Press pads - you should see:
```
[NoteOn]  ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
```

**If no events appear**:
- MIDI connection issue (see "MIDI Device Not Found" above)
- Device in wrong mode
- Device needs reset (power cycle)

### Still Not Working?

Create a minimal test config:

```bash
cat > test-config.toml << 'EOF'
[[modes]]
name = "Test"

[[modes.mappings]]
description = "Test"
[modes.mappings.trigger]
type = "Note"
note = 0
[modes.mappings.action]
type = "Shell"
command = "say 'test works'"
EOF

cargo run --release 2 --config test-config.toml
```

Press pad 0. If you hear "test works", the system is functioning.

## Profile Detection Issues

### Symptoms

- "Failed to load profile" error
- Wrong pads lighting up
- LEDs work without profile but not with profile
- Auto-page detection not working

### Causes

- Profile file not found
- Invalid XML format
- Wrong pad page active
- Note numbers not in profile
- Profile path has spaces or special characters

### Solutions

#### 1. Verify Profile File Exists

```bash
# Check file exists
ls -la path/to/profile.ncmm3

# Try absolute path
cargo run --release 2 --profile "$HOME/Downloads/profile.ncmm3"

# Escape spaces in path
cargo run --release 2 --profile "My\ Profile.ncmm3"
```

#### 2. Validate Profile XML

Open the .ncmm3 file in a text editor and verify it's valid XML:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<DeviceProfile>
  <DeviceProperties>
    <Name>My Profile</Name>
    <Type>MASCHINE_MIKRO_MK3</Type>
  </DeviceProperties>

  <Mapping>
    <PageList>
      <Page name="Pad Page A">
        <!-- ... -->
      </Page>
    </PageList>
  </Mapping>
</DeviceProfile>
```

**Common issues**:
- Missing `<?xml` declaration
- Unclosed tags
- Invalid characters
- Corrupted file

**Fix**: Re-export from Controller Editor or use a backup.

#### 3. Force Specific Pad Page

Auto-detection might fail if notes overlap between pages:

```bash
# Instead of auto-detect
cargo run --release 2 --profile profile.ncmm3

# Force page A
cargo run --release 2 --profile profile.ncmm3 --pad-page A

# Try each page
for page in A B C D E F G H; do
    echo "Testing page $page"
    cargo run --release 2 --profile profile.ncmm3 --pad-page $page
    sleep 5
    killall conductor
done
```

#### 4. Create New Profile

If profile is corrupted:

1. Open **Native Instruments Controller Editor**
2. Select **Maschine Mikro MK3**
3. Create a simple profile:
   - Page A: Notes 12-27 (chromatic)
   - Page B: Notes 36-51 (drums)
4. Save as `test-profile.ncmm3`
5. Test: `cargo run --release 2 --profile test-profile.ncmm3`

#### 5. Debug Profile Loading

```bash
DEBUG=1 cargo run --release 2 --profile profile.ncmm3
```

Look for:
```
[DEBUG] Loading profile: profile.ncmm3
[DEBUG] Profile loaded: My Profile (MASCHINE_MIKRO_MK3)
[DEBUG] Found 8 pad pages
[DEBUG] Page A: 16 pads mapped (notes 12-27)
```

**If you see errors**:
```
[ERROR] Failed to parse profile: XML error at line 42
[ERROR] Profile validation failed: Missing required element
```

Fix the profile XML or create a new one.

#### 6. Verify Note Numbers Match Profile

```bash
# Run pad mapper
cargo run --bin pad_mapper 2
```

Press pads and verify the notes are in your profile.

If note 50 is pressed but your profile only has notes 12-27, no LED will light.

**Solution**: Either:
- Update profile to include note 50
- Change hardware to send notes 12-27 (in Controller Editor)

## Game Controllers (HID) Issues (v3.0+)

### Overview

Conductor v3.0 introduced support for all SDL2-compatible HID devices including gamepads (Xbox, PlayStation, Nintendo Switch Pro), joysticks, racing wheels, flight sticks, HOTAS controllers, and custom controllers. This section covers common issues specific to game controller integration.

For gamepad configuration guidance, see the [Gamepad Support Guide](../guides/gamepad-support.md).

### Gamepad Not Detected

#### Symptoms

- No gamepad shown in Event Console
- `conductorctl status` doesn't list gamepad
- Button presses have no effect
- "No compatible gamepad detected" message

#### Causes

- USB/Bluetooth connection not established
- System not recognizing controller
- SDL2 compatibility issues
- Insufficient permissions (macOS Input Monitoring)
- Missing drivers (Windows)
- Controller in incompatible mode

#### Solutions

##### 1. Verify Physical Connection

**USB Connection**:
```bash
# macOS: Check USB device enumeration
system_profiler SPUSBDataType | grep -i xbox
system_profiler SPUSBDataType | grep -i playstation
system_profiler SPUSBDataType | grep -i controller

# Linux: Check USB devices
lsusb | grep -i xbox
lsusb | grep -i sony
lsusb | grep -i nintendo

# Windows: Device Manager
# Devices and Printers > Game Controllers
```

**Bluetooth Connection**:
- Verify controller is in pairing mode (usually hold PS/Xbox button + Share)
- Check system Bluetooth settings show controller as connected
- Try USB connection first to rule out Bluetooth issues
- Some wireless adapters require specific drivers (Xbox Wireless Adapter on macOS)

**If device not found**:
- Try a different USB port (prefer USB 3.0)
- Try a different USB cable (some cables are charge-only)
- Power cycle the controller (turn off, wait 10 seconds, turn on)
- Check controller battery is charged (wireless controllers)
- Remove and re-pair Bluetooth connection

##### 2. Check System Recognition

**macOS**:
```bash
# Check via System Settings
# System Settings > General > Game Controllers

# Verify controller appears in system report
system_profiler SPUSBDataType | grep -B 5 -A 10 "Xbox\|PlayStation\|Nintendo"
```

**Linux**:
```bash
# Check joystick devices
ls -la /dev/input/js*
# Expected: /dev/input/js0, /dev/input/js1, etc.

# Test with jstest (install: sudo apt install joystick)
jstest /dev/input/js0

# Check evdev access
ls -la /dev/input/event*

# Verify permissions
groups | grep input
```

**Windows**:
```
1. Open "Set up USB game controllers" (search in Start menu)
2. Verify controller appears in list
3. Click "Properties" to test buttons
4. If shows "Unknown device", driver issue
```

##### 3. Verify SDL2 Compatibility

Conductor uses SDL2 gamepad mappings. Most modern controllers are compatible, but some require specific configurations.

**Test SDL2 detection**:
```bash
# Enable debug logging to see SDL2 detection
DEBUG=1 conductor --foreground

# Look for:
[DEBUG] SDL2 initialized
[DEBUG] Found gamepad: Xbox 360 Controller (ID: 0)
[DEBUG] Gamepad mapping: 030000005e040000...(SDL2 GUID)
```

**Known compatible controllers**:
- Xbox 360, Xbox One, Xbox Series X|S (all models)
- PlayStation DualShock 4, DualSense (PS5)
- Nintendo Switch Pro Controller
- Steam Controller
- Generic USB/Bluetooth gamepads with standard layout

**If incompatible**:
- Check [SDL_GameControllerDB](https://github.com/gabomdq/SDL_GameControllerDB) for your controller
- Some controllers need to be in specific mode (XInput vs DirectInput on Windows)
- Custom controllers may need manual SDL2 mapping file

##### 4. Grant Input Monitoring Permission (macOS)

Game controllers require Input Monitoring permission, just like MIDI HID devices.

**Step-by-step**:
1. Run Conductor:
   ```bash
   conductor --foreground
   ```

2. macOS shows permission dialog for Input Monitoring

3. Click **Open System Settings**

4. In **Privacy & Security** > **Input Monitoring**:
   - Find `conductor` or `Terminal` (if running via cargo)
   - Toggle switch to **ON**
   - If already ON, toggle OFF then ON to reset

5. Restart Conductor:
   ```bash
   conductor --foreground
   ```

**Verify permission**:
```bash
DEBUG=1 conductor --foreground
```

Look for:
```
[DEBUG] Input Monitoring permission: Granted
[DEBUG] Gamepad access: Enabled
```

If you see "Input Monitoring permission denied", permission not granted.

##### 5. Install/Verify Drivers

**macOS**:
- **Xbox controllers**: Generally work out-of-box
- **Xbox Wireless Adapter**: Requires [360Controller](https://github.com/360Controller/360Controller/releases) driver
- **PlayStation controllers**: Work via Bluetooth, some features need [DS4Windows](https://ryochan7.github.io/ds4windows-site/) equivalent for macOS
- **Switch Pro**: Works out-of-box via USB or Bluetooth

**Linux**:
```bash
# Install joystick/gamepad support
sudo apt install joystick xboxdrv

# Load xpad kernel module (Xbox controllers)
sudo modprobe xpad

# For Steam Controller
sudo apt install steam-devices

# Check kernel drivers loaded
lsmod | grep -E "xpad|joydev|evdev"
```

**Windows**:
- **Xbox controllers**: Use official Xbox drivers (usually automatic via Windows Update)
- **PlayStation controllers**: Require [DS4Windows](https://ryochan7.github.io/ds4windows-site/) for full functionality
- **Switch Pro**: Works but may need [BetterJoy](https://github.com/Davidobot/BetterJoy)
- Check Device Manager for "Unknown device" under Game Controllers

##### 6. Test with Conductor Event Console

```bash
# Start daemon
conductor --foreground

# In another terminal, watch events
conductorctl events --follow
```

Press buttons on your gamepad - you should see:
```
[GAMEPAD] Button: 128 (A/Cross/B) | State: Pressed
[GAMEPAD] Button: 128 | State: Released
```

**If no events appear**:
- Controller not detected by SDL2
- Permission issue (macOS)
- Driver issue (Windows/Linux)
- Controller needs reset (see next section)

##### 7. Reset Controller

Many connection issues resolve with a controller reset:

**Xbox Controllers**:
```
1. Hold Xbox button for 10 seconds (powers off)
2. Wait 10 seconds
3. Press Xbox button to power on
4. Reconnect to PC
```

**PlayStation Controllers**:
```
1. Find small reset button on back (near L2)
2. Use paperclip, press and hold 5 seconds
3. Reconnect via USB
4. Re-pair Bluetooth if needed
```

**Switch Pro Controller**:
```
1. Press and hold Sync button (top left) for 5 seconds
2. Release and press Home button
3. Reconnect via USB or re-pair Bluetooth
```

### Buttons Not Triggering

#### Symptoms

- Controller detected but button presses don't trigger actions
- Some buttons work, others don't
- Event Console shows button events but mappings don't execute
- Actions trigger randomly or on wrong buttons

#### Causes

- Button IDs don't match config (0-127 vs 128-255 range)
- Wrong trigger type used
- MIDI Learn didn't detect button
- Mode mismatch
- Trigger conditions not met

#### Solutions

##### 1. Verify Button ID Range

**Critical**: Gamepad buttons use IDs **128-255**, not MIDI's 0-127 range.

**Common mistake**:
```toml
# WRONG - This is a MIDI note, not gamepad button
[modes.mappings.trigger]
type = "Note"
note = 0  # MIDI range (0-127)

# CORRECT - Gamepad button ID
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad range (128-255)
```

**ID Ranges**:
- **MIDI devices**: 0-127 (notes, CC)
- **Gamepad buttons**: 128-255
- No overlap, no conflicts

##### 2. Use Event Console to Find Button IDs

```bash
# Start Event Console
conductorctl events --follow --type gamepad
```

Press each button and note the ID:
```
[GAMEPAD] Button: 128 | State: Pressed  # A (Xbox) / Cross (PS) / B (Switch)
[GAMEPAD] Button: 129 | State: Pressed  # B (Xbox) / Circle (PS) / A (Switch)
[GAMEPAD] Button: 132 | State: Pressed  # D-Pad Up
[GAMEPAD] Button: 136 | State: Pressed  # LB / L1 / L
```

Update your config with actual IDs:
```toml
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Use the exact ID from Event Console
[modes.mappings.action]
type = "Keystroke"
keys = "Return"
```

##### 3. Use MIDI Learn for Automatic Detection

**GUI Method** (Recommended):
1. Open Conductor GUI
2. Navigate to mappings
3. Click "Learn" button next to trigger field
4. Press button on gamepad
5. Conductor auto-generates correct trigger config

**Pattern Detection**:
- Press once → `GamepadButton`
- Press twice quickly → `DoubleTap`
- Hold button → `LongPress`
- Press multiple buttons → `GamepadButtonChord`

See [MIDI Learn Guide](../getting-started/midi-learn.md) for details.

##### 4. Check Trigger Type Matches Input

**Button triggers require GamepadButton type**:
```toml
# Face buttons, D-pad, shoulders, etc.
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
```

**Stick triggers require GamepadAnalogStick type**:
```toml
# Left/right stick movement
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X-axis
direction = "Clockwise"
```

**Trigger triggers require GamepadTrigger type**:
```toml
# L2/R2, LT/RT, ZL/ZR
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger
threshold = 128
```

##### 5. Enable Debug Logging

```bash
DEBUG=1 conductor --foreground
```

Press buttons and watch for:

**Good output** (button detected, mapping matched):
```
[GAMEPAD] Button 128 pressed (A/Cross/B)
[DEBUG] Processed: GamepadButton(128)
[DEBUG] Matched mapping: "Confirm Action" (mode: Default)
[DEBUG] Executing action: Keystroke(keys: "Return")
✓ Action executed successfully
```

**Bad output** (button detected, no mapping):
```
[GAMEPAD] Button 128 pressed (A/Cross/B)
[DEBUG] Processed: GamepadButton(128)
[DEBUG] No mapping matched for event
```

If no mapping matched:
- Button ID mismatch (check config vs Event Console)
- Wrong mode active
- Wrong trigger type
- Trigger conditions not met (velocity, timing)

##### 6. Test with Simple Mapping

Add a simple test mapping to verify basic functionality:

```toml
[[global_mappings]]
description = "Test gamepad - A button"
[global_mappings.trigger]
type = "GamepadButton"
button = 128  # A button (Xbox) / Cross (PS) / B (Switch)
[global_mappings.action]
type = "Shell"
command = "echo 'Gamepad works!' && say 'Gamepad works'"
```

If this works, issue is with specific trigger/action configuration.

### Analog Stick Drift / False Triggers

#### Symptoms

- Actions trigger without touching stick
- Constant movement detected
- Stick "stuck" in one direction
- Unwanted repeated actions

#### Causes

- Hardware stick drift (worn potentiometers)
- Dead zone too small
- Threshold too sensitive
- Stick calibration issue

#### Solutions

##### 1. Automatic Dead Zone (10%)

Conductor automatically applies a 10% dead zone to prevent false triggers from stick drift.

**How it works**:
- Stick center: 128 (0-255 range)
- Dead zone: 115-141 (10% in each direction)
- Values in dead zone treated as 128 (neutral)

This prevents small drift values from triggering actions.

##### 2. Check Hardware Drift

**Test stick in system settings**:
- **macOS**: System Settings > Game Controllers > Properties
- **Linux**: `jstest /dev/input/js0`
- **Windows**: "Set up USB game controllers" > Properties > Test

**Look for**:
- Stick position drifts without touching
- Values don't return to center (128)
- Erratic movement when stationary

If hardware drift exceeds 10% (values outside 115-141 range), hardware issue.

##### 3. Increase Trigger Threshold

Instead of analog stick trigger, use button-based threshold:

```toml
# Instead of this (too sensitive)
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130  # Right stick X
direction = "Clockwise"

# Use this (requires more movement)
[modes.mappings.trigger]
type = "GamepadButton"
button = 135  # D-Pad right (more deliberate)
```

Or increase threshold for analog triggers:
```toml
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 130
direction = "Clockwise"
# Note: Dead zone is automatic, but ensure actions require significant movement
```

##### 4. Calibrate Controller

**Windows**:
```
1. Open "Set up USB game controllers"
2. Select your controller
3. Click "Properties" > "Settings"
4. Click "Calibrate"
5. Follow calibration wizard
```

**Linux**:
```bash
# Install joystick calibration tool
sudo apt install joystick

# Run calibration
jscal /dev/input/js0

# Save calibration
sudo jscal-store /dev/input/js0
```

**macOS**:
- No built-in calibration tool
- Consider third-party tools or controller-specific software
- Hardware drift may require controller replacement

##### 5. Hardware Solutions

If drift persists after software fixes:
- **Clean the stick**: Compressed air around stick base
- **Replace stick module**: iFixit guides for most controllers
- **Replace controller**: Modern controllers have drift issues (especially Joy-Cons, DualSense)
- **Use D-pad instead**: More reliable for discrete directions

### Analog Trigger Not Responding

#### Symptoms

- Pulling trigger has no effect
- Some trigger positions work, others don't
- Digital trigger press works but analog doesn't
- Trigger fires at wrong pressure level

#### Causes

- Threshold too high (requires full pull)
- Threshold too low (triggers immediately)
- Wrong trigger type (digital vs analog)
- Trigger axis ID incorrect
- Hardware trigger issue

#### Solutions

##### 1. Adjust Threshold Value

**Threshold range**: 0-255 (0 = not pressed, 255 = fully pressed)

**Common thresholds**:
```toml
# Very sensitive (25% pull)
threshold = 64

# Medium sensitivity (50% pull) - RECOMMENDED
threshold = 128

# Requires deep pull (75%)
threshold = 192

# Almost full pull (90%)
threshold = 230
```

**Start with medium and adjust**:
```toml
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger
threshold = 128  # Start here
```

If no response: Lower threshold (64, 32)
If too sensitive: Raise threshold (192, 230)

##### 2. Verify Trigger vs Button

**Analog triggers** (L2/R2, LT/RT, ZL/ZR):
```toml
# Use GamepadTrigger for pressure sensitivity
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132  # Left trigger (L2, LT, ZL)
threshold = 128

[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Right trigger (R2, RT, ZR)
threshold = 128
```

**Digital triggers** (fully pressed):
```toml
# Use GamepadButton for on/off detection
[modes.mappings.trigger]
type = "GamepadButton"
button = 143  # Left trigger digital (L2, LT, ZL)

[modes.mappings.trigger]
type = "GamepadButton"
button = 144  # Right trigger digital (R2, RT, ZR)
```

**When to use which**:
- **GamepadTrigger**: Variable pressure (volume control, throttle, gradual actions)
- **GamepadButton**: On/off only (simpler, more reliable)

##### 3. Test in Event Console

```bash
conductorctl events --follow --type gamepad
```

Pull trigger slowly and watch output:
```
[GAMEPAD] Trigger: 133 | Value: 0   # Not pressed
[GAMEPAD] Trigger: 133 | Value: 64  # 25% pressed
[GAMEPAD] Trigger: 133 | Value: 128 # 50% pressed
[GAMEPAD] Trigger: 133 | Value: 192 # 75% pressed
[GAMEPAD] Trigger: 133 | Value: 255 # Fully pressed
```

**If no events appear**:
- Hardware trigger issue
- Controller not sending analog data (check controller mode)
- Driver issue (Windows: might be in DirectInput mode, need XInput)

**If values don't reach 255**:
- Trigger might have limited range
- Lower threshold accordingly
- Or use digital button trigger instead

##### 4. Debug Trigger Detection

```bash
DEBUG=1 conductor --foreground
```

Pull trigger and look for:
```
[GAMEPAD] Trigger 133 value: 150
[DEBUG] Threshold: 128 (met)
[DEBUG] Matched mapping: "Volume Up"
[DEBUG] Executing action: VolumeControl(Up)
✓ Action executed
```

If threshold never met, value isn't reaching threshold:
- Lower threshold: `threshold = 64`
- Or check hardware with Event Console

### Hybrid MIDI + Gamepad Conflicts

#### Symptoms

- MIDI or gamepad works alone, but not together
- Wrong device responds to mapping
- Actions trigger on wrong button/pad
- Mode switching affects wrong device

#### Causes

- ID range overlap (using 0-127 for gamepad)
- Config doesn't separate MIDI vs gamepad mappings
- Trigger type mismatch
- Device priority confusion

#### Solutions

##### 1. Understand ID Separation

**No conflicts by design**:
- **MIDI devices**: IDs 0-127 (notes, CC, pitch bend, aftertouch)
- **Gamepad devices**: IDs 128-255 (buttons, sticks, triggers)

**This works seamlessly**:
```toml
# MIDI mapping - Pad 0 (note 36)
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 36  # MIDI range (0-127)
[modes.mappings.action]
type = "Keystroke"
keys = "1"

# Gamepad mapping - A button
[[modes.mappings]]
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Gamepad range (128-255)
[modes.mappings.action]
type = "Keystroke"
keys = "2"
```

##### 2. Fix ID Range Errors

**Common mistake - using MIDI IDs for gamepad**:
```toml
# WRONG - This triggers on MIDI note 10, not gamepad button
[modes.mappings.trigger]
type = "GamepadButton"
button = 10  # Wrong range!

# CORRECT - Gamepad button IDs start at 128
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A button
```

##### 3. Separate MIDI and Gamepad Modes

Organize by device type for clarity:

```toml
[[modes]]
name = "MIDI Controls"
color = "blue"

[[modes.mappings]]
description = "MIDI Pad 1"
[modes.mappings.trigger]
type = "Note"
note = 36

[[modes]]
name = "Gamepad Controls"
color = "green"

[[modes.mappings]]
description = "Gamepad A Button"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
```

Switch modes based on which device you're using.

##### 4. Mode Switching with Both Devices

**Use different controls for mode switching**:

```toml
# MIDI encoder for mode switching
[[global_mappings]]
description = "Encoder: Next mode"
[global_mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = "Next"

# Gamepad chord for mode switching
[[global_mappings]]
description = "LB+RB: Next mode"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]  # LB + RB
timeout_ms = 50
[global_mappings.action]
type = "ModeChange"
mode = "Next"
```

This allows mode switching from either device without conflicts.

##### 5. Verify with Event Console

```bash
conductorctl events --follow
```

Test both devices:
```
# Press MIDI pad
[MIDI] NoteOn ch:0 note:36 vel:87

# Press gamepad button
[GAMEPAD] Button: 128 | State: Pressed
```

Ensure correct event type appears for each device.

### Device Disconnection / Reconnection

#### Symptoms

- "Gamepad disconnected" message
- Controller stops responding mid-session
- Need to restart daemon after unplugging
- Wireless controller loses connection

#### Causes

- Wireless interference or battery low
- USB cable disconnected
- Bluetooth timeout
- System power management
- Driver issue

#### Solutions

##### 1. Auto-Reconnection Behavior

Conductor automatically handles reconnection:

**What happens**:
1. Controller disconnects (USB unplugged, Bluetooth drops, battery dies)
2. Daemon logs: `[WARN] Gamepad disconnected (ID: 0)`
3. Daemon continues running, monitoring for reconnection
4. Controller reconnects
5. Daemon logs: `[INFO] Gamepad reconnected (ID: 0)`
6. Mappings resume automatically

**No action needed** in most cases - just reconnect the controller.

##### 2. Verify Auto-Reconnection

```bash
# Watch daemon logs
DEBUG=1 conductor --foreground

# Disconnect controller (unplug USB or power off)
# You'll see:
[WARN] Gamepad disconnected (ID: 0)
[DEBUG] Polling for reconnection...

# Reconnect controller
# You'll see:
[INFO] Gamepad detected: Xbox 360 Controller (ID: 0)
[INFO] Gamepad reconnected successfully
```

##### 3. Manual Reconnection Steps

If auto-reconnection fails:

**USB Controllers**:
```bash
1. Unplug USB cable
2. Wait 5 seconds
3. Plug back in
4. Check Event Console for events
```

**Bluetooth Controllers**:
```bash
1. Power off controller (hold PS/Xbox button)
2. Open system Bluetooth settings
3. Remove/forget the controller
4. Put controller in pairing mode
5. Re-pair to system
6. Test in Event Console
```

##### 4. Daemon Restart (Last Resort)

If reconnection doesn't work:

```bash
# Stop daemon
conductorctl stop

# Wait 2 seconds
sleep 2

# Start daemon
conductor --foreground

# Verify gamepad detected
conductorctl status
```

##### 5. Prevent Wireless Disconnections

**Check battery level**:
- Low battery causes disconnections
- Keep controllers charged
- Use wired connection for critical work

**Reduce interference**:
- Keep controller within 10 feet of receiver
- Avoid metal objects between controller and PC
- Turn off other Bluetooth devices
- Use USB connection if interference persists

**Disable system power management**:

**macOS**:
```
System Settings > Battery > Options
Uncheck "Put hard disks to sleep when possible"
```

**Linux**:
```bash
# Disable USB autosuspend for controller
echo -1 | sudo tee /sys/bus/usb/devices/.../power/autosuspend
```

**Windows**:
```
Device Manager > Universal Serial Bus controllers
Right-click USB Root Hub > Properties > Power Management
Uncheck "Allow the computer to turn off this device to save power"
```

### Platform-Specific Gamepad Issues

#### macOS

##### Xbox Wireless Adapter Not Working

**Problem**: Xbox controller via Wireless Adapter not detected

**Solution**:
```bash
# Install 360Controller driver
# Download from: https://github.com/360Controller/360Controller/releases

# Or via Homebrew
brew install --cask 360controller

# Restart system
sudo reboot

# Verify detection
system_profiler SPUSBDataType | grep -i xbox
```

##### Permission Dialog Keeps Appearing

**Problem**: Input Monitoring permission prompt appears repeatedly

**Solution**:
```bash
# Grant permission to Terminal instead of conductor binary
# This persists across rebuilds when running via cargo

# Or code-sign the binary
codesign --force --deep --sign - target/release/conductor
```

#### Linux

##### Insufficient Permission to Access /dev/input

**Problem**: `Permission denied` when accessing gamepad

**Solution**:
```bash
# Add user to input group
sudo usermod -a -G input $USER

# Create udev rule for gamepads
sudo tee /etc/udev/rules.d/50-gamepad.rules << 'EOF'
# Xbox controllers
SUBSYSTEM=="usb", ATTRS{idVendor}=="045e", MODE="0666", GROUP="input"

# PlayStation controllers
SUBSYSTEM=="usb", ATTRS{idVendor}=="054c", MODE="0666", GROUP="input"

# Nintendo controllers
SUBSYSTEM=="usb", ATTRS{idVendor}=="057e", MODE="0666", GROUP="input"

# Generic HID gamepads
SUBSYSTEM=="input", ATTRS{name}=="*Controller*", MODE="0666", GROUP="input"
SUBSYSTEM=="input", ATTRS{name}=="*Gamepad*", MODE="0666", GROUP="input"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Log out and back in (or reboot)
```

##### Joystick Device Not Created

**Problem**: `/dev/input/js0` doesn't exist

**Solution**:
```bash
# Load joydev kernel module
sudo modprobe joydev

# Make permanent (add to /etc/modules)
echo "joydev" | sudo tee -a /etc/modules

# Verify joystick devices
ls -la /dev/input/js*
```

##### Xbox Controller Not Recognized

**Problem**: Xbox controller via USB not working

**Solution**:
```bash
# Install xboxdrv
sudo apt install xboxdrv

# Load xpad module
sudo modprobe xpad

# Test with jstest
jstest /dev/input/js0
```

#### Windows

##### Controller Shows as "Unknown Device"

**Problem**: Device Manager shows gamepad as "Unknown device"

**Solution**:
```
1. Open Device Manager
2. Right-click "Unknown device"
3. Select "Update driver"
4. Choose "Search automatically for drivers"
5. Or download from manufacturer:
   - Xbox: Windows Update installs automatically
   - PlayStation: Install DS4Windows
   - Switch Pro: Install BetterJoy
```

##### DS4Windows Conflict

**Problem**: PlayStation controller works in DS4Windows but not Conductor

**Solution**:
```
DS4Windows emulates Xbox controller, which Conductor can detect.

Option 1: Use DS4Windows (controller appears as Xbox)
- Keep DS4Windows running
- Conductor sees it as Xbox controller
- Use Xbox button IDs (128-255)

Option 2: Native PlayStation support
- Close DS4Windows
- Restart Conductor
- Use native PlayStation support
- Same button IDs (128-255) work with either
```

##### XInput vs DirectInput Mode

**Problem**: Gamepad not detected in one mode

**Solution**:
```
Some controllers have mode switches:
- XInput mode: Modern Windows support (preferred)
- DirectInput mode: Legacy support

Look for X/D switch on controller or hold button combo:
- Usually: Start + Back for 3 seconds switches mode
- LED indicator changes when mode switches

Conductor works best with XInput mode on Windows.
```

### Getting Additional Help

If your gamepad issue isn't covered here:

1. **Check Event Console**:
   ```bash
   conductorctl events --follow --type gamepad
   ```
   Verify button presses appear

2. **Enable Debug Logging**:
   ```bash
   DEBUG=1 conductor --foreground
   ```
   Look for SDL2 detection messages

3. **Collect Information**:
   - OS version (macOS 14.2, Ubuntu 22.04, Windows 11)
   - Conductor version: `conductor --version`
   - Controller model (Xbox Series X, DualSense, etc.)
   - Connection type (USB, Bluetooth, Wireless Adapter)
   - Error messages from debug log
   - Output of:
     ```bash
     conductorctl status
     system_profiler SPUSBDataType | grep -i controller  # macOS
     lsusb | grep -i controller  # Linux
     ```

4. **File GitHub Issue**:
   - Include all collected information above
   - Attach relevant portions of debug log
   - Describe expected vs actual behavior
   - See [Support Resources](../resources/support.md)

**Related Documentation**:
- [Gamepad Support Guide](../guides/gamepad-support.md) - Configuration reference
- [Event Console Guide](../guides/event-console.md) - Real-time debugging
- [MIDI Learn Guide](../getting-started/midi-learn.md) - Auto-detect buttons
- [Device Templates](../guides/device-templates.md) - Pre-configured gamepad setups

## Platform-Specific Issues

### macOS: Permission Dialogs Keep Appearing

**Cause**: Binary changes (recompiling) invalidates permissions

**Solution**: Grant permission to Terminal.app instead:
1. System Settings → Privacy & Security → Input Monitoring
2. Add `Terminal` (or your terminal emulator)
3. Run via `cargo run` - permission persists across rebuilds

**Alternative**: Code-sign the binary:
```bash
codesign --force --deep --sign - target/release/conductor
```

### macOS: "Cannot be opened because the developer cannot be verified"

**Solution**:
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine target/release/conductor

# Or allow in System Settings
# Right-click binary → Open → Allow
```

### Linux: Permission Denied (USB/HID Access)

**Cause**: User not in `plugdev` group or missing udev rules

**Solution**:

1. Add udev rules:
```bash
sudo tee /etc/udev/rules.d/50-conductor.rules << 'EOF'
# Native Instruments Maschine Mikro MK3
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTRS{bInterfaceClass}=="01", ATTRS{bInterfaceSubClass}=="03", MODE="0666", GROUP="plugdev"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

2. Add user to plugdev:
```bash
sudo usermod -a -G plugdev $USER
```

3. Log out and back in

4. Test:
```bash
cargo run --release 2
```

### Windows: MIDI Device Not Recognized

**Cause**: Driver not installed or generic USB driver used

**Solution**:
1. Open Device Manager
2. Look for "MIDI Device" or your controller under "Sound, video and game controllers"
3. Right-click → Update Driver
4. Choose manufacturer driver (not generic USB)
5. Or install via Native Access

## Performance Issues

### High CPU Usage

**Symptom**: Conductor using >10% CPU

**Causes**:
- Debug logging enabled
- Animated LED scheme with high update rate
- Shell actions running slowly
- Event processing loop not optimizing

**Solutions**:

1. Disable debug logging:
   ```bash
   # Don't use DEBUG=1 in production
   cargo run --release 2
   ```

2. Use simpler LED scheme:
   ```bash
   # Instead of animated schemes
   cargo run --release 2 --led reactive  # or off
   ```

3. Optimize shell actions:
   ```bash
   # Avoid long-running commands in mappings
   # Bad:
   command = "sleep 10 && echo done"

   # Good:
   command = "echo done &"  # Background process
   ```

4. Build release mode:
   ```bash
   # Debug builds are 20-30% slower
   cargo build --release
   ./target/release/conductor 2
   ```

### High Latency (Delayed Response)

**Symptom**: Actions trigger 50-100ms+ after pad press

**Solutions**:

1. Use release build (not debug)
2. Check system load (Activity Monitor/Task Manager)
3. Close unnecessary applications
4. Check MIDI buffer settings in Audio MIDI Setup (macOS)

**Verify latency**:
```bash
DEBUG=1 cargo run --release 2
```

Watch timestamps:
```
[16:32:45.123] NoteOn received
[16:32:45.124] Action executed  # Should be <2ms
```

If >10ms, investigate system performance.

## Getting Additional Help

If your issue isn't covered here:

1. Check [Diagnostics Guide](diagnostics.md) for detailed troubleshooting
2. Enable debug logging: `DEBUG=1 cargo run --release 2`
3. Run diagnostic tools:
   ```bash
   cargo run --bin midi_diagnostic 2
   cargo run --bin led_diagnostic
   cargo run --bin test_midi
   ```
4. Collect information:
   - macOS/Linux/Windows version
   - Conductor version: `cargo --version`
   - Device model
   - Error messages (full output)
   - Debug log output
5. File an issue on GitHub with collected information

---

**Last Updated**: November 21, 2025
**Status**: Actively maintained
