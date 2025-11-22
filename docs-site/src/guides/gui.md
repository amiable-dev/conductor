# GUI Configuration

The Conductor GUI provides a visual interface for configuring your MIDI controller without manually editing TOML files.

## Overview

The GUI application is built with Tauri v2 and provides:

- **Visual mode management** - Create and edit mapping modes with different button layouts
- **Mapping editor** - Configure triggers and actions with visual selectors
- **MIDI Learn** - Auto-detect trigger patterns by pressing device buttons
- **Device management** - Connect to MIDI devices and apply templates
- **Profile management** - Per-app profiles that switch automatically
- **Live event console** - Debug MIDI events in real-time
- **Settings panel** - Configure application preferences

## Getting Started

### Launch the GUI

```bash
# Start the GUI application
./conductor-gui

# Or if installed system-wide
conductor-gui
```

The GUI will appear in your system tray with quick access to:
- Status display
- Reload configuration
- Pause/resume processing
- Mode switching
- Open config file
- View logs

### Initial Setup

1. **Connect a Device**
   - Navigate to **Devices** tab
   - Select your MIDI controller from the list
   - Click **Connect**

2. **Choose a Template** (optional)
   - Click **Device Templates**
   - Select a pre-configured template for your device
   - Click **Apply Template**

3. **Create Your First Mode**
   - Navigate to **Modes** tab
   - Click **+ Add Mode**
   - Set a name and color
   - Click **Save**

## Mode Configuration

### Creating Modes

Modes allow different button mappings for different contexts (e.g., "Development", "Media", "Gaming").

1. Go to **Modes** tab
2. Click **+ Add Mode**
3. Fill in:
   - **Name**: Descriptive name (e.g., "Video Editing")
   - **Color**: Visual identifier (blue, green, purple, etc.)
4. Click **Save**

### Editing Modes

1. Select the mode from the list
2. Click **Edit Mode**
3. Modify settings:
   - Name
   - Color
   - Mode-specific mappings
4. Click **Save Changes**

### Deleting Modes

1. Select the mode
2. Click **Delete Mode**
3. Confirm deletion

**Note**: You cannot delete the last remaining mode.

## Mapping Configuration

### Creating Mappings

Mappings define what happens when you press a button or turn a knob.

1. Go to **Mappings** tab
2. Choose:
   - **Mode-specific** mappings (active only in selected mode)
   - **Global** mappings (active across all modes)
3. Click **+ Add Mapping**

### Using MIDI Learn

The fastest way to create mappings:

1. Click **🎹 MIDI Learn** button
2. Press/turn the button/knob on your device
3. Conductor detects the pattern (note, velocity, long press, etc.)
4. The trigger is auto-filled
5. Configure the action (what to do)
6. Click **Save Mapping**

See the [MIDI Learn guide](../getting-started/midi-learn.md) for details.

### Manual Trigger Configuration

If you prefer manual setup:

#### Trigger Types

- **Note**: Basic note on/off
  - Set note number (0-127)
  - Optional velocity range

- **Velocity Range**: Different actions for soft/medium/hard presses
  - Soft: 0-40
  - Medium: 41-80
  - Hard: 81-127

- **Long Press**: Hold detection
  - Set duration (default 2000ms)

- **Double Tap**: Quick double-press
  - Set timeout window (default 300ms)

- **Note Chord**: Multiple notes simultaneously
  - Add 2+ notes
  - Set detection window (default 100ms)

- **Encoder Turn**: Knob rotation
  - Set CC number
  - Choose direction (Clockwise/CounterClockwise)

- **Control Change (CC)**: MIDI CC messages
  - Set CC number
  - Optional value range

- **Aftertouch**: Pressure sensitivity
  - Optional minimum pressure

- **Pitch Bend**: Touch strip control
  - Optional value range

### Action Configuration

#### Action Types

- **Keystroke**: Keyboard shortcuts
  - Select modifiers (Ctrl, Alt, Shift, Super/Cmd)
  - Set key(s)

- **Text**: Type text strings
  - Enter text to type

- **Launch**: Open applications
  - Enter application name or path

- **Shell**: Execute shell commands
  - Enter command
  - Optional working directory

- **Volume Control**: System volume
  - Up/Down/Mute/Set

- **Mode Change**: Switch modes
  - Select target mode

- **Sequence**: Chain multiple actions
  - Add multiple actions in order

- **Delay**: Wait between actions
  - Set duration in milliseconds

- **Mouse Click**: Simulate mouse input
  - Left/Right/Middle button
  - Single/Double/Triple click

- **Repeat**: Repeat an action
  - Set count

### Editing Mappings

1. In the **Mappings** tab
2. Click the mapping to edit
3. Modify trigger or action
4. Click **Save Changes**

### Deleting Mappings

1. Select the mapping
2. Click **Delete**
3. Confirm deletion

## Device Management

### Connecting Devices

1. Go to **Devices** tab
2. View available MIDI devices
3. See connection status
4. Monitor daemon status (running, uptime, events processed)

### Using Device Templates

Templates provide pre-configured mappings for popular controllers:

1. Click **📋 Device Templates**
2. Browse available templates:
   - Maschine Mikro MK3
   - Launchpad Mini
   - Korg nanoKONTROL
   - Custom templates
3. Select a template
4. Click **Apply**
5. Reload daemon configuration

### Profile Management

Profiles let you switch entire configurations:

1. Click **🔄 Profiles**
2. View available profiles
3. Switch manually or enable per-app automatic switching
4. Export/import profiles for backup

## Per-App Profiles

Automatically switch profiles based on the frontmost application.

See the [Per-App Profiles guide](./per-app-profiles.md) for complete setup.

## Live Event Console

Debug MIDI events in real-time:

1. Go to **Settings** tab
2. Click **📊 Show Event Console**
3. View live MIDI events:
   - Note on/off
   - Velocity values
   - Control changes
   - Timing information
4. Use for troubleshooting and discovering note numbers

## Settings

### Application Settings

Configure Conductor preferences:

- **Auto-start**: Launch on system startup
- **Log Level**: Control logging verbosity (debug, info, warn, error)
- **Theme**: Light/dark theme (future)

### Configuration File

View and edit the raw config file:

1. See the file path
2. Click **📋** to copy path to clipboard
3. Click **📝** to open in your default editor

## Menu Bar (System Tray)

The menu bar icon provides quick access:

### Menu Options

- **Status**: View daemon state
- **Reload Configuration**: Hot-reload config changes
- **Pause Processing**: Temporarily disable event processing
- **Resume Processing**: Re-enable event processing
- **Switch Mode**: Quick mode switching
  - Default
  - Development
  - Media
- **View Logs**: Open system logs
- **Open Config File**: Edit configuration
- **Quit Conductor**: Stop daemon and quit

### Icon States

- **🟢 Running**: Daemon active and processing events
- **🟡 Paused**: Processing paused
- **🔴 Stopped**: Daemon not running
- **⚠️ Error**: Error state

## Keyboard Shortcuts

Global shortcuts (when GUI is focused):

- **Cmd/Ctrl + R**: Reload configuration
- **Cmd/Ctrl + Q**: Quit application
- **Cmd/Ctrl + ,**: Open settings

## Tips & Best Practices

1. **Use MIDI Learn** for faster setup - it's more accurate than manual entry

2. **Test mappings immediately** after creation - press the button to verify

3. **Start with global mappings** for frequently used actions (volume, mode switch)

4. **Use descriptive names** for modes and mappings - future you will thank you

5. **Export your config regularly** - back up your work

6. **Use the event console** when troubleshooting - see exactly what MIDI data is coming in

7. **Organize with modes** - keep related mappings together

8. **Device templates save time** - start with a template and customize

## Troubleshooting

### GUI Won't Connect to Daemon

1. Check daemon is running: `conductorctl status`
2. Start daemon if needed: `conductor`
3. Check IPC socket exists: `ls /tmp/conductor.sock`
4. Restart daemon: `conductorctl stop && conductor`

### Mappings Not Saving

1. Check file permissions on config file
2. Verify config path in Settings tab
3. Check daemon logs for errors
4. Try manual edit to verify TOML syntax

### MIDI Events Not Detected

1. Check device connection in Devices tab
2. Use event console to verify MIDI data
3. Ensure correct MIDI port selected
4. Check device permissions (Input Monitoring on macOS)

### Menu Bar Icon Missing

- macOS: Check System Settings → Privacy & Security → Accessibility
- Linux: Ensure system tray extension installed
- Try restarting the GUI application

## Next Steps

- Learn about [MIDI Learn mode](../getting-started/midi-learn.md)
- Set up [per-app profiles](./per-app-profiles.md)
- Explore [device templates](./device-templates.md)
- Configure [LED feedback](./led-system.md)
- Use the [event console](./event-console.md) for debugging
