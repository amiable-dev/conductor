# Input Learn Mode

**Input Learn** is the fastest way to create mappings in Conductor v3.0. Instead of manually entering note numbers, button IDs, or MIDI parameters, simply press a control on your device and let Conductor auto-detect everything.

## What is Input Learn?

Input Learn (formerly "MIDI Learn") is a one-click workflow that works with both **MIDI controllers** and **Game Controllers (HID)**:

1. **Click "Learn"** next to a trigger field
2. **Press a control** on your device (pad, button, stick, encoder, etc.)
3. **Conductor auto-fills** the trigger configuration
4. **Assign an action** and save

That's it! No need to know note numbers, button IDs, CC values, or MIDI channels.

## Supported Device Types

Input Learn works with:

### MIDI Controllers
- **Pad controllers**: Maschine, Launchpad, etc.
- **Keyboards**: MIDI keyboards with velocity sensitivity
- **Encoders/knobs**: Rotary encoders, faders
- **DJ controllers**: Mixers, CDJs

### Game Controllers (HID)
- **Gamepads**: Xbox, PlayStation, Nintendo Switch Pro
- **Joysticks**: Flight sticks, arcade sticks
- **Racing wheels**: Logitech, Thrustmaster, Fanatec
- **Flight controls**: HOTAS systems
- **Custom controllers**: Any SDL2-compatible HID device

## How to Use Input Learn

### Basic Workflow

1. **Open Conductor GUI** and ensure your device is connected

2. **Navigate to Mappings** panel

3. **Click "Add Mapping"** or edit an existing one

4. **Click the "Learn" button** next to the Trigger field

5. **Input Learn window opens** with a 10-second countdown:
   ```
   Input Learn Mode

   Waiting for input...
   Press any control on your device

   Time remaining: 8 seconds
   [Cancel]
   ```

6. **Press any control** on your MIDI or gamepad device:
   - **MIDI**: Pad, button, encoder/knob, fader, touch strip
   - **Gamepad**: Button, analog stick, trigger, D-pad

7. **Trigger auto-fills** (examples):

   **MIDI Pad**:
   ```
   Trigger Type: Note
   Note: 36
   Channel: 0
   ```

   **Gamepad Button**:
   ```
   Trigger Type: GamepadButton
   Button: 128
   ```

   **Analog Stick**:
   ```
   Trigger Type: GamepadAnalogStick
   Axis: 130
   Direction: Clockwise
   ```

8. **Assign an action** (Keystroke, Launch, Text, etc.)

9. **Click "Save"**

10. **Test it!** Press the same control - the action should execute

## Supported Trigger Types

Input Learn auto-detects and configures these trigger types:

### MIDI Triggers

#### 1. Note (Basic Pad/Button Press)

**What it detects**:
- Note number
- MIDI channel
- Velocity range (if applicable)

**Example**:
- Press pad → Auto-fills: `Note: 36, Channel: 0`

**Use cases**:
- Basic pad mappings
- Button presses
- Keyboard keys

#### 2. Velocity Range (Pressure-Sensitive)

**What it detects**:
- Note number
- Soft/medium/hard press patterns
- Velocity thresholds

**Example**:
- Press pad softly → `VelocityRange: Note 36, Min: 0, Max: 40`
- Press pad hard → `VelocityRange: Note 36, Min: 81, Max: 127`

**Use cases**:
- Different actions for soft vs hard hits
- Velocity-sensitive controls

#### 3. Long Press

**What it detects**:
- Note number
- Hold duration (auto-calculated from your press)

**Example**:
- Hold pad for 2 seconds → `LongPress: Note 36, Duration: 2000ms`

**Use cases**:
- Hold pad to open app
- Long press for alternate action

#### 4. Double-Tap

**What it detects**:
- Note number
- Double-tap timing window

**Example**:
- Tap pad twice quickly → `DoubleTap: Note 36, Window: 300ms`

**Use cases**:
- Quick double-tap for special actions
- Distinguishing single vs double taps

#### 5. Chord (Multiple Notes)

**What it detects**:
- All pressed notes
- Chord window (how fast notes must be pressed together)

**Example**:
- Press pads 36, 40, 43 together → `Chord: [36, 40, 43], Window: 100ms`

**Use cases**:
- Shortcuts requiring multiple pads
- Musical chord detection

#### 6. Encoder/Knob Rotation

**What it detects**:
- CC (Control Change) number
- Direction (Clockwise/Counterclockwise)
- Value range

**Example**:
- Turn encoder right → `EncoderTurn: CC 1, Direction: Clockwise`
- Turn encoder left → `EncoderTurn: CC 1, Direction: Counterclockwise`

**Use cases**:
- Volume control
- Scrolling
- Parameter adjustment

#### 7. Control Change (CC)

**What it detects**:
- CC number
- Value range
- Continuous vs momentary

**Example**:
- Move fader → `CC: 7, Range: 0-127`
- Press button → `CC: 64, Value: 127`

**Use cases**:
- Faders
- Knobs with CC messages
- Sustain pedals

#### 8. Aftertouch (Pressure)

**What it detects**:
- Note number (if channel aftertouch)
- Pressure threshold

**Example**:
- Press pad harder after initial hit → `Aftertouch: Note 36, Threshold: 64`

**Use cases**:
- Pressure-sensitive effects
- Dynamic parameter control

#### 9. Pitch Bend

**What it detects**:
- Pitch bend range
- Direction (Up/Down/Center)

**Example**:
- Move pitch wheel up → `PitchBend: Direction: Up, Threshold: 8192`

**Use cases**:
- Pitch wheel mappings
- Touch strip controls

### Game Controller (HID) Triggers

#### 10. GamepadButton

**What it detects**:
- Button ID (128-255 range)
- Device type (gamepad, joystick, wheel, etc.)

**Example**:
- Press A button (Xbox) → `GamepadButton: 128`
- Press Cross button (PlayStation) → `GamepadButton: 128`
- Press B button (Switch) → `GamepadButton: 128`

**Use cases**:
- Face button mappings (A, B, X, Y)
- Shoulder buttons (LB, RB, L1, R1)
- D-pad buttons
- Menu buttons (Start, Select, Home)

**Button ID Reference**:
- **Face buttons**: 128-131 (A/B/X/Y)
- **D-pad**: 132-135 (Up/Down/Left/Right)
- **Shoulder buttons**: 136-137 (LB/RB, L1/R1)
- **Stick clicks**: 138-139 (L3/R3)
- **Menu buttons**: 140-142 (Start/Select/Home)
- **Digital triggers**: 143-144 (LT/RT, L2/R2, ZL/ZR)

#### 11. GamepadButtonChord

**What it detects**:
- Multiple button IDs pressed simultaneously
- Chord timing window

**Example**:
- Press A + B together → `GamepadButtonChord: [128, 129], Window: 50ms`
- Press LB + RB together → `GamepadButtonChord: [136, 137], Window: 50ms`

**Use cases**:
- Multi-button shortcuts
- Mode switching combos
- Emergency actions

#### 12. GamepadAnalogStick

**What it detects**:
- Axis ID (128-131 for stick axes)
- Direction (Clockwise/CounterClockwise)
- Dead zone (automatic 10%)

**Example**:
- Move right stick right → `GamepadAnalogStick: Axis 130, Direction: Clockwise`
- Move left stick up → `GamepadAnalogStick: Axis 129, Direction: Clockwise`
- Move right stick left → `GamepadAnalogStick: Axis 130, Direction: CounterClockwise`

**Use cases**:
- Navigation controls
- Scrolling
- Cursor movement

**Stick Axes**:
- **128**: Left stick X-axis (left/right)
- **129**: Left stick Y-axis (up/down)
- **130**: Right stick X-axis (left/right)
- **131**: Right stick Y-axis (up/down)

#### 13. GamepadTrigger

**What it detects**:
- Trigger ID (132-133 for analog triggers)
- Threshold value (0-255 range)
- Pull depth detection

**Example**:
- Pull right trigger halfway → `GamepadTrigger: 133, Threshold: 128`
- Pull left trigger fully → `GamepadTrigger: 132, Threshold: 200`

**Use cases**:
- Pressure-sensitive actions
- Volume control
- Acceleration/braking (racing wheels)
- Throttle control (flight sticks)

**Trigger IDs**:
- **132**: Left trigger (L2, LT, ZL)
- **133**: Right trigger (R2, RT, ZR)

## Advanced Input Learn Features

### Countdown Timer

The Input Learn window shows a **10-second countdown**. This gives you time to:
- Position your hand
- Find the right control
- Try different velocities/pressures
- Test analog stick directions

**Countdown reaches zero** → Input Learn cancels automatically

### Cancellation

Click **"Cancel"** at any time to abort Input Learn without creating a mapping.

**Keyboard shortcut**: Press `Esc` to cancel

### Pattern Detection

Input Learn is **smart** - it detects patterns automatically:

#### MIDI Patterns

**Long Press Detection**:
- If you hold a pad for >1 second during Learn mode, Conductor suggests a Long Press trigger

**Double-Tap Detection**:
- If you tap a pad twice quickly, Conductor suggests a Double-Tap trigger

**Chord Detection**:
- If you press multiple pads within 100ms, Conductor suggests a Chord trigger

**Velocity Variation**:
- If you press the same pad with varying velocities, Conductor suggests a Velocity Range trigger

#### Game Controller Patterns

**Button Hold Detection**:
- Hold a button for >1 second → Suggests LongPress trigger

**Button Double-Tap Detection**:
- Tap a button twice quickly → Suggests DoubleTap trigger

**Multi-Button Detection**:
- Press multiple buttons simultaneously → Suggests GamepadButtonChord trigger

**Analog Stick Movement**:
- Move stick in any direction → Detects axis and direction automatically

**Trigger Pull Detection**:
- Pull analog trigger → Detects threshold and creates GamepadTrigger

### Velocity Range Suggestions

When Input Learn detects a note, it suggests velocity ranges:

```
Input Learn Complete!

Detected: Note 36

Suggested velocity ranges:
- Soft (0-40): Gentle tap
- Medium (41-80): Normal press
- Hard (81-127): Strong hit

Create separate mappings for each range?
[Yes] [No, use single Note trigger]
```

This makes it easy to create pressure-sensitive mappings.

## Device-Specific Workflows

### Gamepad Example (Xbox Controller)

**Goal**: Map A button to copy text (Cmd+C)

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press **A button** on Xbox controller
4. Auto-detects: `GamepadButton: 128`
5. Select Action: **Keystroke**
6. Use Keystroke Picker: Press `Cmd+C`
7. Save

**Result**: Pressing A button executes Cmd+C

### Flight Stick Example

**Goal**: Map trigger to enter key

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Pull trigger** on flight stick
4. Auto-detects: `GamepadButton: 128` (or GamepadTrigger if analog)
5. Action: **Keystroke** → `Return`
6. Save

**Result**: Pulling trigger presses Enter

### Racing Wheel Example

**Goal**: Map wheel rotation to browser navigation

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Turn wheel right**
4. Auto-detects: `GamepadAnalogStick: Axis 128, Direction: Clockwise`
5. Action: **Keystroke** → `Cmd+RightArrow` (forward in browser)
6. Save

**Result**: Turning wheel right navigates forward in browser

### HOTAS Example

**Goal**: Map throttle up to volume increase

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Move throttle up**
4. Auto-detects: `GamepadAxis: 129, Direction: Clockwise`
5. Action: **Volume Control** → **Up**
6. Save

**Result**: Moving throttle up increases system volume

### Hybrid MIDI + Gamepad

**Goal**: Use MIDI pad and gamepad button together

You can freely mix MIDI and gamepad inputs:

**MIDI Pad for Copy**:
1. Learn → Press MIDI pad 36 → Auto-detects `Note: 36`
2. Action: Keystroke `Cmd+C`

**Gamepad A Button for Paste**:
1. Learn → Press A button → Auto-detects `GamepadButton: 128`
2. Action: Keystroke `Cmd+V`

**No conflicts**: MIDI uses IDs 0-127, gamepads use IDs 128-255

## Troubleshooting

### No Input Detected

**Symptoms**: Input Learn countdown reaches zero without detecting anything

**Solutions**:
1. **Check device connection**:
   - MIDI: Ensure device shows in Device Panel
   - Gamepad: Verify connection in system settings
2. **Check Event Console**: Open Event Console and verify events are being received
3. **Try different control**: Some controls may not send events (e.g., mode buttons)
4. **Restart device**: Disconnect and reconnect the device

### Wrong Button/Note Detected

**Symptoms**: Input Learn detects incorrect button ID or note number

**Solutions**:
1. **Verify in Event Console**: Check what events are actually being sent
2. **Check button ID range**:
   - MIDI: 0-127
   - Gamepad: 128-255
3. **Load device template**: Use a pre-configured template for your controller
4. **Manual override**: Click "Advanced" and manually enter the correct ID

### Gamepad Not Recognized

**Symptoms**: Gamepad buttons don't trigger Input Learn

**Solutions**:
1. **Ensure SDL2 compatibility**: Check if gamepad is SDL2-compatible
2. **Check system recognition**:
   - macOS: System Settings → Game Controllers
   - Linux: `ls /dev/input/js*`
   - Windows: Devices and Printers
3. **Try USB instead of Bluetooth** (or vice versa)
4. **Restart Conductor daemon**: `conductorctl stop && conductor --foreground`

### Multiple Events Detected

**Symptoms**: Input Learn shows "Multiple events detected, please try again"

**Solutions**:
1. **Press only one control** at a time
2. **Wait for button/pad to release** before pressing again
3. **Disable auto-repeat**: Some controllers send rapid-fire messages

### Analog Stick Not Detected

**Symptoms**: Moving stick doesn't trigger Input Learn

**Solutions**:
1. **Move stick beyond dead zone**: Move at least 15% from center
2. **Check axis ID**: Ensure using correct stick (left vs right)
3. **Verify in Event Console**: See if axis events are being received
4. **Try different direction**: Some sticks may have faulty axes

### Trigger Pull Not Detected

**Symptoms**: Pulling analog trigger doesn't work

**Solutions**:
1. **Pull trigger fully**: Some triggers need >50% pull
2. **Check threshold**: Try different pull depths
3. **Use digital trigger instead**: Try the digital trigger button (LT/RT button)
4. **Verify in Event Console**: Check if trigger axis events appear

### Velocity Not Detected

**Symptoms**: Input Learn only creates Note trigger, not Velocity Range

**Solutions**:
1. **Vary velocity**: Try pressing soft, medium, and hard during different Learn attempts
2. **Manual configuration**: Create Velocity Range trigger manually after Learn
3. **Check controller**: Some pads don't send velocity (always velocity 127)

## Examples

### Example 1: Basic MIDI Pad Mapping

**Goal**: Map pad to copy text (Cmd+C)

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press pad → Auto-fills `Note: 36`
4. Select Action: **Keystroke**
5. Use Keystroke Picker: Press `Cmd+C`
6. Save

**Result**: Pressing pad executes Cmd+C

### Example 2: Gamepad Multi-Button Combo

**Goal**: LB + RB switches to Media mode

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press **LB + RB together** → Auto-fills `GamepadButtonChord: [136, 137]`
4. Action: **Mode Change** → Select "Media"
5. Save

**Result**: Pressing LB + RB together switches to Media mode

### Example 3: Velocity-Sensitive Volume

**Goal**: Soft press = volume down, hard press = volume up

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. Press pad **softly** → Auto-fills `VelocityRange: Note 36, Min: 0, Max: 40`
4. Action: **Volume Control** → **Down**
5. Save

6. Click "Add Mapping" again
7. Click "Learn"
8. Press **same pad hard** → Auto-fills `VelocityRange: Note 36, Min: 81, Max: 127`
9. Action: **Volume Control** → **Up**
10. Save

**Result**: Soft press = volume down, hard press = volume up

### Example 4: Long Press to Launch App

**Goal**: Hold pad for 2 seconds to open Spotify

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Hold pad for 2+ seconds** → Auto-fills `LongPress: Note 36, Duration: 2000ms`
4. Action: **Launch** → Select `/Applications/Spotify.app`
5. Save

**Result**: Holding pad for 2 seconds opens Spotify

### Example 5: Encoder for Volume

**Goal**: Turn encoder to adjust volume

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Turn encoder right** → Auto-fills `EncoderTurn: CC 1, Direction: Clockwise`
4. Action: **Volume Control** → **Up**
5. Save

6. Click "Add Mapping" again
7. Click "Learn"
8. **Turn encoder left** → Auto-fills `EncoderTurn: CC 1, Direction: Counterclockwise`
9. Action: **Volume Control** → **Down**
10. Save

**Result**: Turning encoder controls system volume

### Example 6: Analog Stick Navigation

**Goal**: Right stick controls browser navigation

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Move right stick right** → Auto-fills `GamepadAnalogStick: Axis 130, Direction: Clockwise`
4. Action: **Keystroke** → `Cmd+RightArrow`
5. Save

6. Click "Add Mapping"
7. Click "Learn"
8. **Move right stick left** → Auto-fills `GamepadAnalogStick: Axis 130, Direction: CounterClockwise`
9. Action: **Keystroke** → `Cmd+LeftArrow`
10. Save

**Result**: Moving right stick navigates forward/back in browser

### Example 7: Racing Wheel Throttle

**Goal**: Wheel triggers control volume

1. Click "Add Mapping"
2. Click "Learn" next to Trigger
3. **Pull right trigger** → Auto-fills `GamepadTrigger: 133, Threshold: 128`
4. Action: **Volume Control** → **Up**
5. Save

**Result**: Pulling right trigger increases volume

## Tips & Best Practices

### Tip 1: Use Event Console

Before starting Input Learn:
1. Open **Event Console**
2. Press your control
3. Verify the event appears

This helps debug "Learn not detecting" issues.

### Tip 2: Learn in Context

When creating **per-app profiles**, Learn with that app in focus:
1. Switch to target app (e.g., Logic Pro)
2. Switch back to Conductor GUI
3. Use Input Learn
4. Assign action relevant to that app

### Tip 3: Batch Learn

Create multiple mappings quickly:
1. Click "Learn"
2. Press control
3. Assign action
4. Save
5. Immediately click "Add Mapping" and repeat

### Tip 4: Device Templates First

Before manual Learn:
1. Check if a device template exists for your controller
2. Load template to get 90% of mappings
3. Use Learn to customize the remaining 10%

### Tip 5: Test Immediately

After creating a mapping:
1. Click "Save"
2. **Immediately test** by pressing the control
3. Verify action executes correctly
4. Adjust if needed

### Tip 6: Gamepad Button IDs

Remember the ID ranges:
- **MIDI**: 0-127 (notes, CC, etc.)
- **Gamepad**: 128-255 (buttons, axes, triggers)
- **No overlap**: Both can coexist in same config

### Tip 7: Analog Stick Dead Zones

Analog sticks have 10% dead zones:
- Center position won't trigger
- Move at least 15% from center
- Prevents false triggers

### Tip 8: Hybrid Setups

Combine MIDI and gamepad strengths:
- **MIDI pads**: Velocity-sensitive music actions
- **Gamepad buttons**: Navigation and shortcuts
- **MIDI encoders**: Fine parameter control
- **Gamepad triggers**: Pressure-sensitive volume

## Next Steps

- [Gamepad Support Guide](../guides/gamepad-support.md) - Complete gamepad documentation
- [Device Templates](../guides/device-templates.md) - Pre-configured controller mappings
- [Trigger Reference](../reference/triggers.md) - All trigger types explained
- [Action Reference](../reference/actions.md) - All action types explained
- [Per-App Profiles](../guides/per-app-profiles.md) - Automatic profile switching

---

**Last Updated**: November 21, 2025 (v3.0)
