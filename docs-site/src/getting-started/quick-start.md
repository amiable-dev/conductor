# Quick Start Guide

Get Conductor v3.0+ up and running in under 5 minutes using the visual GUI.

## Prerequisites

- **macOS 11.0+** (Big Sur or later) - Intel or Apple Silicon
- **Input device**:
  - MIDI controller (any USB MIDI device - Maschine Mikro MK3 recommended for full RGB LED support), OR
  - Game controller (Xbox, PlayStation, Switch Pro, joystick, racing wheel, HOTAS, or any SDL2-compatible HID device)
- **10 minutes** for installation (3-5 minutes for gamepad setup with templates)

## Step 1: Download Conductor

Visit the [GitHub Releases page](https://github.com/amiable-dev/conductor/releases/latest) and download:

**For most users** (recommended):
- `conductor-gui-macos-universal.tar.gz` - GUI application (includes daemon)

**For CLI-only users**:
- `conductor-aarch64-apple-darwin.tar.gz` (Apple Silicon)
- `conductor-x86_64-apple-darwin.tar.gz` (Intel)

## Step 2: Install the GUI

```bash
# Extract the downloaded file
tar xzf conductor-gui-macos-universal.tar.gz

# Move to Applications folder
mv "Conductor GUI.app" /Applications/

# Open the app
open /Applications/"Conductor GUI.app"
```

**First launch**: macOS will ask for permissions. Grant **Input Monitoring** permission when prompted (required for LED control and device access).

## Step 3: Connect Your Device

1. **Plug in your MIDI controller** via USB

2. **In the Conductor GUI**, go to the **Device Connection** panel

3. Your device should appear in the list. Click **Connect**

4. The status bar at the bottom should show: **"Connected to [Your Device]"**

If your device doesn't appear:
- Check USB cable
- Try a different USB port
- See [Troubleshooting Device Connection](../troubleshooting/device-connection.md)

## Step 4: Create Your First Mapping with MIDI Learn

The fastest way to create a mapping is using **MIDI Learn mode**:

1. **Click "Add Mapping"** in the Mappings panel

2. **Click "Learn"** next to the Trigger field

3. **Press any pad/button** on your MIDI controller

4. The trigger configuration **auto-fills** with the detected input

5. **Choose an action**:
   - **Keystroke** - Press a key (e.g., Cmd+C for copy)
   - **Launch** - Open an application
   - **Text** - Type text
   - **Shell** - Run a command

6. **Click "Save"**

7. **Test it!** Press the same pad/button - the action should execute

**Example**: Map pad to open Spotify:
- Trigger: Note 36 (auto-detected via MIDI Learn)
- Action: Launch → `/Applications/Spotify.app`

## Step 5: Explore Advanced Features

### Velocity Sensitivity

Map different actions based on how hard you hit a pad:

1. Select trigger type: **Velocity Range**
2. Use MIDI Learn to detect the note
3. Set velocity ranges:
   - **Soft** (0-40): Pause playback
   - **Hard** (81-127): Skip track

### Long Press Detection

Hold a pad for 2+ seconds to trigger a different action:

1. Select trigger type: **Long Press**
2. Use MIDI Learn
3. Set **Hold Duration**: 2000ms
4. Action: Open Calculator

### Chord Detection

Press multiple pads simultaneously:

1. Select trigger type: **Chord**
2. Use MIDI Learn and press all pads within 100ms
3. Action: Launch your favorite app

## Step 6: Use Device Templates

Skip manual configuration with built-in device templates:

1. Go to **Settings** → **Device Templates**

2. **Select your controller**:
   - Maschine Mikro MK3
   - Launchpad Mini
   - APC Mini
   - Korg nanoKONTROL2
   - Novation Launchkey Mini
   - AKAI MPK Mini

3. Click **Load Template**

4. The template loads pre-configured mappings for common workflows

5. **Customize** as needed using MIDI Learn

## Quick Start with Game Controllers (v3.0+)

Conductor v3.0+ supports game controllers (gamepads, joysticks, racing wheels, HOTAS, etc.) as macro input devices alongside MIDI controllers. You can use them individually or together!

### Supported Device Types

- **Gamepads**: Xbox, PlayStation, Nintendo Switch Pro (templates available)
- **Joysticks**: Flight sticks, arcade sticks (manual config)
- **Racing Wheels**: Logitech, Thrustmaster (manual config)
- **HOTAS**: Throttle and stick controllers (manual config)
- **Custom Controllers**: Any SDL2-compatible HID device

### Option 1: Gamepad with Template (Fastest - ~3 minutes)

The quickest way to set up an Xbox, PlayStation, or Switch Pro controller:

1. **Connect your gamepad** via USB or Bluetooth

2. **Verify it's recognized** by your system:
   - macOS: System Settings → Game Controllers
   - Linux: `ls /dev/input/js*`
   - Windows: Devices and Printers

3. **Open Conductor GUI** → **Device Templates**

4. **Filter by "🎮 Game Controllers"**

5. **Select your controller**:
   - Xbox 360/One/Series X|S
   - PlayStation DualShock 4/DualSense (PS5)
   - Nintendo Switch Pro Controller

6. **Click "Create Config"** → **Reload daemon**

7. **Test buttons!** Press any button - the mapped action should execute

**Time to completion**: ~3 minutes from connection to working mappings

### Option 2: Manual Setup (Joysticks, Wheels, Other - ~10 minutes)

For non-gamepad controllers (flight sticks, racing wheels, HOTAS):

1. **Connect your HID device** via USB

2. **Verify system recognition** (same commands as above)

3. **Open Conductor GUI** → **MIDI Learn**

4. **Create mappings using MIDI Learn**:
   - Click "Add Mapping"
   - Click "Learn" next to Trigger
   - Press a button or move an axis on your controller
   - Conductor auto-detects the input
   - Choose an action (Keystroke, Launch, Shell, etc.)
   - Click "Save"

5. **Repeat for all buttons/axes** you want to map

6. **Test your mappings!**

**Time to completion**: ~10 minutes for 10-15 button mappings

### Example: Flight Stick Setup

**Trigger button → Launch flight simulator**:
```toml
[[modes.mappings]]
description = "Flight stick trigger: Launch Flight Sim"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Auto-detected via MIDI Learn
[modes.mappings.action]
type = "Launch"
path = "/Applications/Flight Simulator.app"
```

**Hat switch → Arrow keys**:
```toml
[[modes.mappings]]
description = "Hat up: View up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132  # D-Pad/Hat up
[modes.mappings.action]
type = "Keystroke"
keys = "UpArrow"
```

### Example: Racing Wheel Setup

**Wheel rotation → Steering**:
```toml
[[modes.mappings]]
description = "Wheel right: Steer right"
[modes.mappings.trigger]
type = "GamepadAnalogStick"
axis = 128  # Wheel axis (auto-detected)
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
```

**Pedals → Gas/Brake**:
```toml
[[modes.mappings]]
description = "Gas pedal: Accelerate"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133  # Gas pedal axis
threshold = 64  # 25% pedal press
[modes.mappings.action]
type = "Keystroke"
keys = "w"
```

### Platform-Specific Notes

**macOS**:
- Most gamepads work via USB or Bluetooth without drivers
- Xbox Wireless Adapter may require driver installation
- Grant **Input Monitoring** permission when prompted

**Linux**:
- Ensure `jstest` recognizes your controller: `jstest /dev/input/js0`
- May need udev rules for device permissions
- Install `xdotool` for keystroke simulation

**Windows**:
- Xbox controllers work natively
- PlayStation controllers may need DS4Windows or similar
- Check Device Manager for driver status

### Hybrid MIDI + Gamepad Setup

You can use MIDI controllers and gamepads **simultaneously**:

- **MIDI devices**: Button IDs 0-127
- **Gamepad devices**: Button IDs 128-255
- **No conflicts** - they work together seamlessly!

**Example**: Use Maschine pads for music production, gamepad for application shortcuts.

### Troubleshooting Game Controllers

**Controller not detected**:
1. Check USB/Bluetooth connection
2. Verify system recognition (see commands above)
3. Try USB instead of Bluetooth (or vice versa)
4. Restart Conductor daemon: `conductorctl stop && conductorctl reload`

**Buttons not working**:
1. Use MIDI Learn to verify button IDs (should be 128-255)
2. Check Event Console for incoming events
3. Ensure no conflicting mappings exist

**Analog stick too sensitive**:
- Conductor uses a 10% automatic dead zone
- For more precision, use discrete buttons (D-Pad) instead of analog sticks

For more details, see the [Gamepad Support Guide](../guides/gamepad-support.md).

## Step 7: Enable Auto-Start (Optional)

Run Conductor automatically when you log in:

1. Go to **Settings** → **General**

2. Enable **"Start Conductor on login"**

3. Click **Save**

The daemon will now start automatically in the background every time you log in.

## Using the Daemon CLI (Optional)

For advanced users who prefer terminal control:

```bash
# Check daemon status
conductorctl status

# Reload configuration (hot-reload in 0-10ms)
conductorctl reload

# Stop daemon
conductorctl stop

# Validate config without reloading
conductorctl validate

# Ping daemon (check latency)
conductorctl ping
```

See [Daemon & CLI Guide](../guides/daemon.md) for full details.

## Per-App Profiles (Automatic Profile Switching)

Conductor v2.0.0 can automatically switch configurations based on which application is active:

1. Go to **Per-App Profiles** in the GUI

2. **Add a new profile**:
   - Application: Select an app (e.g., "Visual Studio Code")
   - Profile: Select a config profile

3. When you switch to that application, Conductor automatically loads the configured profile

4. **Example use cases**:
   - **Logic Pro**: Pads control DAW functions (play, record, etc.)
   - **VS Code**: Pads trigger common shortcuts (run, debug, search)
   - **Chrome**: Pads control tabs and navigation

## Live Event Console

Debug your mappings in real-time:

1. Go to **Event Console** in the GUI

2. **Watch MIDI events** as they happen:
   - Note On/Off
   - Velocity values
   - Control Change
   - Pitch Bend
   - Aftertouch

3. **Filter events** by type or note number

4. **Export logs** for debugging

This is invaluable for troubleshooting "why isn't my mapping working?"

## Troubleshooting

### Device Not Found

1. **Check USB connection**:
   ```bash
   system_profiler SPUSBDataType | grep -i midi
   ```

2. **Restart the daemon**:
   ```bash
   conductorctl stop
   open /Applications/"Conductor GUI.app"
   ```

3. **Check Audio MIDI Setup**:
   ```bash
   open -a "Audio MIDI Setup"
   ```

### LEDs Not Working

1. **Ensure Native Instruments drivers are installed** (for Maschine controllers)

2. **Grant Input Monitoring permission**:
   - System Settings → Privacy & Security → Input Monitoring
   - Enable for "Conductor GUI"

3. **Check LED scheme** in GUI Settings

4. **View debug logs** in Event Console

### Mappings Not Triggering

1. **Use Event Console** to verify MIDI events are being received

2. **Use MIDI Learn** to verify the correct note numbers

3. **Check mode** - is the mapping in the current mode or global?

4. **Reload config**:
   ```bash
   conductorctl reload
   ```

### Permission Denied (macOS)

If you see "Permission denied" errors:

1. **System Settings** → **Privacy & Security** → **Input Monitoring**
2. Add **"Conductor GUI"** to the list
3. Restart the GUI app

## 🎉 Congratulations! You're Now a Conductor Power User

You've just unlocked:
- ✅ Visual configuration with Input Learn
- ✅ Hot-reload (0-10ms config changes)
- ✅ Per-app profiles
- ✅ v3.0 gamepad + MIDI support

### What's Next?

#### 🚀 Level Up Your Setup
- **[Explore Device Templates](../guides/device-templates.md)** - Load pre-built configs for popular controllers
- **[Try Velocity Sensitivity](../guides/velocity-curves.md)** - One pad, three actions based on press strength
- **[Set Up Hybrid Mode](../configuration/examples.md#hybrid-midi-gamepad-configuration-v30)** - Combine MIDI + gamepad simultaneously

#### 💡 Get Inspired
- **[Configuration Examples](../configuration/examples.md)** - Copy-paste ready workflows for DAWs, development, streaming
- **[Gamepad Support Guide](../guides/gamepad-support.md)** - Turn your Xbox controller into a macro pad
- **[LED System Guide](../guides/led-system.md)** - Add visual feedback to your controller

#### 📖 Go Deeper
- **[Triggers Reference](../configuration/triggers.md)** - All 15+ trigger types explained
- **[Actions Reference](../configuration/actions.md)** - Complete action type catalog
- **[Context-Aware Mappings](../guides/context-aware.md)** - App-based, time-based, conditional actions

#### 🤝 Join the Community
- **[GitHub Discussions](https://github.com/amiable/conductor/discussions)** - Ask questions, share configs
- **[Report Issues](https://github.com/amiable/conductor/issues)** - Found a bug? Request a feature?
- **[Contribute](../development/contributing.md)** - Help make Conductor better

**Need help?** Check the [FAQ](../troubleshooting/faq.md) or [Troubleshooting Guide](../troubleshooting/common-issues.md).

**Loving Conductor?** ⭐ Star us on [GitHub](https://github.com/amiable/conductor) to support the project!
