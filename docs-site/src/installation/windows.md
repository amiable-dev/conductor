# Windows Installation

## Overview

This guide walks through installing and configuring Conductor v3.0.0 on Windows. Conductor now includes multi-protocol input support (MIDI controllers + game controllers), a background daemon service, and a modern Tauri-based GUI for visual configuration.

**Installation Options**:
- **Option 1 (Recommended)**: Download pre-built binaries from [GitHub Releases](https://github.com/amiable-dev/conductor/releases)
- **Option 2**: Build from source (developers/advanced users)

Installation takes approximately 15-20 minutes.

**Supported Windows Versions**:
- Windows 11 (recommended)
- Windows 10 (1903 or later)
- Windows Server 2019+

## Option 1: Install Pre-Built Binaries (Recommended)

### 1. Download Conductor

Visit the [Releases Page](https://github.com/amiable-dev/conductor/releases/latest) and download:

**For GUI + Daemon (Recommended)**:
- `conductor-gui-windows-x86_64.zip` - GUI application with daemon
- **OR** download daemon separately: `conductor-x86_64-pc-windows-msvc.zip`

### 2. Install the Binaries

```powershell
# Extract the archive
Expand-Archive -Path conductor-x86_64-pc-windows-msvc.zip -DestinationPath C:\Program Files\Conductor

# Add to PATH (PowerShell as Administrator)
$env:Path += ";C:\Program Files\Conductor"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)

# Verify installation
conductor --version
conductorctl --version
```

### 3. Install as Windows Service (Optional)

```powershell
# Run as Administrator
# Create scheduled task to auto-start Conductor
schtasks /create /tn "Conductor Daemon" /tr "C:\Program Files\Conductor\conductor.exe" /sc onlogon /rl highest
```

**Skip to [Hardware Requirements](#hardware-requirements)**

---

## Option 2: Build from Source

### Prerequisites

#### 1. Hardware Requirements

Conductor v3.0 supports two types of input devices:

**MIDI Controllers**:
- Native Instruments Maschine Mikro MK3 (recommended, full RGB LED support)
- Generic MIDI controllers (keyboard controllers, pad controllers, etc.)
- USB-MIDI or MIDI over Bluetooth

**Game Controllers (HID)** (v3.0+):
- Gamepads: Xbox (360, One, Series X|S - native support), PlayStation (DualShock 4, DualSense), Switch Pro Controller
- Joysticks: Flight sticks, arcade sticks
- Racing Wheels: Logitech, Thrustmaster, or any DirectInput/XInput compatible wheel
- HOTAS: Hands On Throttle And Stick systems
- Custom Controllers: Any SDL2-compatible HID device

You need at least one MIDI controller OR one game controller to use Conductor. Both can be used simultaneously.

#### 2. Software Requirements

**Rust Toolchain** (for building from source):

Conductor is written in Rust and requires the Rust compiler and Cargo build system.

**Check if Rust is already installed**:
```powershell
rustc --version
cargo --version
```

If you see version numbers (e.g., `rustc 1.75.0`), skip to the next section.

**Install Rust using rustup**:
1. Download rustup-init.exe from https://rustup.rs/
2. Run the installer
3. Follow the prompts and select the default installation
4. Restart your terminal/PowerShell

**Verify installation**:
```powershell
rustc --version  # Should show: rustc 1.75.0 (or later)
cargo --version  # Should show: cargo 1.75.0 (or later)
```

**SDL2 Library** (for game controllers):

SDL2 is included via the `gilrs` v0.10 Rust crate. No additional installation required - it's built into Conductor automatically.

#### 3. Platform-Specific Requirements

**Microsoft C++ Build Tools** (Required):

Required for compiling native Rust dependencies.

**Option A - Visual Studio Build Tools** (Recommended):
1. Download from https://visualstudio.microsoft.com/downloads/
2. Install "Desktop development with C++" workload
3. Restart your terminal

**Option B - Full Visual Studio**:
1. Download Visual Studio Community (free)
2. Install "Desktop development with C++" workload

**Verify installation**:
```powershell
where cl
# Should show: C:\Program Files\Microsoft Visual Studio\...\cl.exe
```

#### 4. Game Controller Support

**Windows Game Controllers**:

Most game controllers work natively on Windows without additional drivers:

**Xbox Controllers**:
- Xbox 360: Native XInput support
- Xbox One: Native XInput support via USB or Xbox Wireless Adapter
- Xbox Series X|S: Native XInput support via USB, Bluetooth, or Xbox Wireless Adapter
- **No additional drivers required**

**PlayStation Controllers**:
- DualShock 4: Native DirectInput support via USB or Bluetooth
- DualSense (PS5): Native DirectInput support via USB or Bluetooth
- **For XInput emulation** (optional): Install DS4Windows from https://ds4-windows.com/

**Switch Pro Controller**:
- Native DirectInput support via USB or Bluetooth
- **For XInput emulation** (optional): Install BetterJoy or reWASD

**Generic Controllers**:
- Most USB and Bluetooth gamepads work via DirectInput
- SDL2 provides automatic mapping for 100+ controller types

**Verify game controller detection**:
```powershell
# Open Windows Settings
start ms-settings:devices-controllersandgamedevices

# Or open Game Controllers directly
control joy.cpl
```

Your controller should appear in the list. Click "Properties" to test buttons and axes.

#### 5. Device-Specific Drivers (Optional)

**Native Instruments Drivers** (for Maschine Mikro MK3 only):

If using a Maschine Mikro MK3, install Native Instruments drivers for full RGB LED support.

**Download and install**:
1. Visit [Native Instruments Downloads](https://www.native-instruments.com/en/support/downloads/)
2. Download **Native Access** (the NI installation manager)
3. Install Native Access and sign in (free account)
4. In Native Access, install:
   - **Maschine** software (includes drivers)
   - **Controller Editor** (for creating custom profiles, optional)

**Verify driver installation**:
```powershell
# Check Device Manager
devmgmt.msc

# Look for "Maschine Mikro MK3" under:
# - Sound, video and game controllers
# - Universal Serial Bus devices
```

### Building from Source

#### 1. Clone the Repository

```powershell
# Choose a location for the project
cd ~\Projects  # or wherever you keep code

# Clone the repository
git clone https://github.com/amiable-dev/conductor.git
cd conductor
```

#### 2. Build the Daemon

**Release build** (recommended for regular use):
```powershell
# Build the entire workspace (daemon + core)
cargo build --release --workspace

# Or build just the daemon binary
cargo build --release --package conductor-daemon
```

The release build takes 3-7 minutes on modern hardware and produces an optimized binary (~3-5MB) in `target\release\conductor.exe`.

**Build output**:
```
   Compiling conductor-core v3.0.0 (C:\Users\you\Projects\conductor\conductor-core)
   Compiling conductor-daemon v3.0.0 (C:\Users\you\Projects\conductor\conductor-daemon)
    Finished release [optimized] target(s) in 3m 42s
```

#### 3. Build the GUI (Optional)

```powershell
# Install Node.js (if not already installed)
# Download from https://nodejs.org/ (LTS version)

# Install frontend dependencies
cd conductor-gui\ui
npm ci

# Build the frontend
npm run build

# Build the Tauri backend
cd ..\src-tauri
cargo build --release

# The GUI exe will be at:
# conductor-gui\src-tauri\target\release\conductor-gui.exe
```

#### 4. Install Binaries

```powershell
# Return to project root
cd ~\Projects\conductor

# Create installation directory
New-Item -ItemType Directory -Force -Path "C:\Program Files\Conductor"

# Copy binaries (requires Administrator)
Copy-Item target\release\conductor.exe "C:\Program Files\Conductor\"
Copy-Item target\release\conductorctl.exe "C:\Program Files\Conductor\"

# Add to PATH (PowerShell as Administrator)
$env:Path += ";C:\Program Files\Conductor"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)

# Verify installation (restart terminal first)
conductor --version
conductorctl --version
```

## Verifying Device Connection

### Verifying MIDI Controller Connection

#### Check USB Device in Device Manager

```powershell
# Open Device Manager
devmgmt.msc
```

Look for your MIDI controller under:
- **Sound, video and game controllers**
- **Universal Serial Bus devices**

**Example**: "Maschine Mikro MK3" should appear

#### Check MIDI Ports

```powershell
# List available MIDI ports
conductor --list-ports

# Or use test_midi diagnostic
cargo run --bin test_midi
```

#### Test MIDI Events

```powershell
# Run diagnostic tool (replace 0 with your port number)
cargo run --bin midi_diagnostic 0

# Or if installed:
midi_diagnostic 0
```

Press pads on your controller. You should see:
```
[NoteOn] ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
```

If nothing appears:
- Check USB connection
- Verify correct port number (try 0, 1, 2, etc.)
- Install device drivers
- Check Device Manager for errors

### Verifying Game Controller Connection

#### Check Windows Game Controllers

```powershell
# Open Game Controllers panel
control joy.cpl

# Or via Settings
start ms-settings:devices-controllersandgamedevices
```

Your controller should appear in the list. Select it and click "Properties" to test:
- Button presses
- Analog stick movements
- Trigger pulls
- D-pad inputs

#### Check via Device Manager

```powershell
# Open Device Manager
devmgmt.msc
```

Look for your controller under:
- **Xbox Peripherals** (Xbox controllers)
- **Human Interface Devices** (Generic gamepads)
- **Bluetooth** (Wireless controllers)

#### Check via Conductor Status

```powershell
# Start Conductor
Start-Process conductor

# Check status
conductorctl status

# Look for gamepad in device list
# Example output:
# Connected Devices:
#   - Xbox Wireless Controller (Gamepad)
```

#### Test Gamepad Events

Use Conductor's debug logging to verify gamepad inputs:

```powershell
# Start Conductor with debug logging
$env:DEBUG=1
conductor --foreground
```

Press buttons on your gamepad. You should see:
```
[GamepadButton] button:128 (A/Cross/B)
[GamepadButton] button:129 (B/Circle/A)
[GamepadAnalogStick] axis:128 value:255 (Left stick right)
```

If nothing appears:
- Check USB or Bluetooth connection
- Verify controller appears in Game Controllers (joy.cpl)
- Check battery level (wireless controllers)
- Try reconnecting the controller
- Restart Conductor

#### Platform-Specific Troubleshooting

**Xbox Wireless Adapter**:
- Requires Xbox Wireless Adapter for Windows
- USB adapter available from Microsoft or third-party
- Automatic driver installation on Windows 10+

**Bluetooth Connection**:
1. Open Settings → Devices → Bluetooth & other devices
2. Click "Add Bluetooth or other device"
3. Select "Bluetooth"
4. Put controller in pairing mode:
   - **Xbox**: Hold pair button until LED flashes
   - **PlayStation**: Hold Share + PS button
   - **Switch Pro**: Hold sync button on top
5. Select controller from list
6. Wait for pairing to complete

**DS4Windows for PlayStation Controllers** (Optional):
1. Download from https://ds4-windows.com/
2. Install and run DS4Windows
3. Connect DualShock 4 or DualSense
4. DS4Windows provides XInput emulation and additional features

**Permissions**:
- Windows does not require special permissions for gamepad access
- If issues persist, run Conductor as Administrator (not recommended long-term)

## Configuration

### Using the GUI (Recommended)

v3.0.0 includes a visual configuration editor:

1. **Open Conductor GUI**:
   ```powershell
   conductor-gui
   ```

2. **Connect your device** in the device panel (MIDI or gamepad)

3. **Use MIDI Learn mode**:
   - Click "Learn" next to any trigger
   - Press a button on your controller or gamepad
   - The trigger config auto-fills
   - Assign an action (keystroke, launch app, etc.)

4. **Save configuration** - automatically writes to `%USERPROFILE%\.config\conductor\config.toml`

See [GUI Quick Start](../getting-started/gui-quick-start.md) for detailed tutorial.

### Manual Configuration (Advanced)

If you prefer to edit `config.toml` manually:

**Config location**: `%USERPROFILE%\.config\conductor\config.toml`

**Create a minimal config**:
```powershell
# Create config directory
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.config\conductor"

# Create config file
@"
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Test mapping - Copy"
[modes.mappings.trigger]
type = "Note"
note = 12
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["ctrl"]

[[modes.mappings]]
description = "Gamepad A button - Paste"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A/Cross/B button
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["ctrl"]

[[global_mappings]]
description = "Emergency exit (hold pad 0 for 3 seconds)"
[global_mappings.trigger]
type = "LongPress"
note = 0
hold_duration_ms = 3000
[global_mappings.action]
type = "Shell"
command = "taskkill /IM conductor.exe /F"
"@ | Out-File -FilePath "$env:USERPROFILE\.config\conductor\config.toml" -Encoding utf8
```

This creates a basic configuration with:
- One mode (Default)
- One MIDI test mapping (pad 12 = Ctrl+C)
- One gamepad test mapping (A button = Ctrl+V)
- One emergency exit (hold pad 0 to quit)

**Hot-reload**: The daemon automatically reloads config within 0-10ms when you save changes.

## Running Conductor

### Using the GUI

```powershell
# Launch the GUI (starts daemon automatically)
conductor-gui
```

The GUI provides:
- Real-time event console
- Visual configuration editor
- Device status monitoring
- Daemon control (pause/resume/reload)

### Using the Daemon

```powershell
# Run in foreground
conductor --foreground

# Run in background
Start-Process conductor

# Check status
conductorctl status

# Control daemon
conductorctl reload  # Reload configuration
conductorctl stop    # Stop daemon
```

### Auto-Start on Login

**Option 1 - Scheduled Task**:
```powershell
# Create scheduled task (run as Administrator)
$action = New-ScheduledTaskAction -Execute "C:\Program Files\Conductor\conductor.exe"
$trigger = New-ScheduledTaskTrigger -AtLogon
$principal = New-ScheduledTaskPrincipal -UserId $env:USERNAME -LogonType Interactive -RunLevel Highest
Register-ScheduledTask -Action $action -Trigger $trigger -Principal $principal -TaskName "Conductor Daemon" -Description "Auto-start Conductor daemon on login"
```

**Option 2 - Startup Folder**:
```powershell
# Create shortcut in Startup folder
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\Conductor.lnk")
$Shortcut.TargetPath = "C:\Program Files\Conductor\conductor.exe"
$Shortcut.Save()
```

## Troubleshooting

### Build Errors

**Error**: `link.exe not found`

**Solution**: Install Visual Studio Build Tools:
1. Download from https://visualstudio.microsoft.com/downloads/
2. Install "Desktop development with C++" workload
3. Restart terminal

---

**Error**: `could not compile 'windows-sys'`

**Solution**: Update Rust toolchain:
```powershell
rustup update
cargo clean
cargo build --release
```

---

### Runtime Errors - MIDI

**Error**: `No MIDI input ports available`

**Solution**:
1. Check USB connection
2. Open Device Manager and verify device appears
3. Install device drivers
4. Restart Windows

---

**Error**: `Failed to open MIDI device`

**Solution**:
1. Close other MIDI applications (DAWs, etc.)
2. Disconnect and reconnect device
3. Restart Windows
4. Try different USB port

---

### Runtime Errors - Game Controllers

**Error**: `Gamepad not detected`

**Solution**:
1. Open Game Controllers (joy.cpl) and verify controller appears
2. Test controller in Properties dialog
3. Check battery level (wireless)
4. Try USB connection instead of Bluetooth
5. Run Conductor as Administrator (temporary test)
6. Check debug output: `$env:DEBUG=1; conductor --foreground`

---

**Error**: `Gamepad buttons not responding`

**Solution**:
1. Use MIDI Learn to discover correct button IDs
2. Verify button IDs are in range 128-255 (not 0-127)
3. Test controller in joy.cpl
4. For PlayStation controllers, try DS4Windows
5. Check that gamepad appears in `conductorctl status`

---

**Error**: `Analog stick not working`

**Solution**:
1. Check axis IDs in joy.cpl Properties
2. Verify axis IDs match config (128-133)
3. Check dead zone settings
4. Calibrate controller in joy.cpl
5. Use button triggers instead of analog for precise control

---

### Bluetooth Gamepad Issues

**Controller not pairing**:
1. Open Settings → Bluetooth & other devices
2. Remove old pairings
3. Put controller in pairing mode
4. Pair as new device
5. Test in joy.cpl

**Controller disconnects randomly**:
1. Check battery level
2. Update Bluetooth drivers
3. Move USB Bluetooth adapter away from other USB 3.0 devices
4. Use USB cable instead

**Controller lag or latency**:
1. Use USB cable for lowest latency
2. Use Xbox Wireless Adapter instead of Bluetooth
3. Update controller firmware
4. Close background applications

### Windows Firewall

If running Conductor across network (advanced):

```powershell
# Allow Conductor through firewall
New-NetFirewallRule -DisplayName "Conductor" -Direction Inbound -Program "C:\Program Files\Conductor\conductor.exe" -Action Allow
```

## Next Steps

Now that Conductor v3.0.0 is installed and running:

### For GUI Users
1. **Learn the GUI**: Read [GUI Quick Start Guide](../getting-started/gui-quick-start.md)
2. **MIDI Learn Tutorial**: See [MIDI Learn Mode](../getting-started/midi-learn.md)
3. **Device Templates**: Check [Using Device Templates](../guides/device-templates.md)
4. **Per-App Profiles**: Set up [Application-Specific Profiles](../guides/per-app-profiles.md)
5. **Gamepad Setup**: Read [Gamepad Support Guide](../guides/gamepad-support.md) (v3.0+)

### For CLI Users
1. **Daemon Control**: Read [Daemon & Hot-Reload Guide](../guides/daemon.md)
2. **CLI Reference**: See [conductorctl Commands](../reference/cli.md)
3. **Manual Configuration**: Check [Configuration Overview](../configuration/overview.md)
4. **Advanced Actions**: Explore [Actions Reference](../reference/actions.md)

### For All Users
- **Gamepad Support**: [Gamepad Support Guide](../guides/gamepad-support.md) (v3.0+)
- **Troubleshooting**: [Common Issues](../troubleshooting/common-issues.md)
- **Diagnostic Tools**: [Debugging Guide](../troubleshooting/diagnostics.md)

## Getting Help

If you encounter issues:

1. Check [Common Issues](../troubleshooting/common-issues.md)
2. Use [Diagnostic Tools](../troubleshooting/diagnostics.md)
3. Enable debug logging: `$env:DEBUG=1; conductor --foreground`
4. Check Event Viewer for errors
5. File an issue on GitHub with:
   - Windows version
   - Device model (MIDI or gamepad)
   - Error messages
   - Output of `cargo --version` and `rustc --version`

---

**Last Updated**: November 21, 2025 (v3.0.0)
**Windows Support**: Windows 10 (1903+), Windows 11, Server 2019+
**Architecture**: x86_64
**Input Support**: MIDI Controllers + Game Controllers (HID)
