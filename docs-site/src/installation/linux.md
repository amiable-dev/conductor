# Linux Installation Guide

## Overview

This guide walks through installing and configuring Conductor v3.0.0 on Linux. Conductor now includes multi-protocol input support (MIDI controllers + game controllers), a background daemon service with systemd integration, and a modern Tauri-based GUI for visual configuration.

**Installation Options**:
- **Option 1 (Recommended)**: Download pre-built binaries from [GitHub Releases](https://github.com/amiable-dev/conductor/releases)
- **Option 2**: Build from source (developers/advanced users)

Installation takes approximately 15-20 minutes.

**Supported Distributions**:
- Ubuntu 20.04+ (LTS recommended)
- Debian 11+ (Bullseye or later)
- Fedora 35+
- Arch Linux (rolling release)
- Other systemd-based distributions

## Option 1: Install Pre-Built Binaries (Recommended)

### 1. Download Conductor

Visit the [Releases Page](https://github.com/amiable-dev/conductor/releases/latest) and download:

**For GUI + Daemon (Recommended)**:
- `conductor-gui-linux-x86_64.tar.gz` - GUI application with daemon
- **OR** download daemon separately: `conductor-x86_64-unknown-linux-gnu.tar.gz`

### 2. Install the Binaries

```bash
# Extract the archive
tar xzf conductor-x86_64-unknown-linux-gnu.tar.gz

# Make binaries executable
chmod +x conductor conductorctl

# Move to PATH
sudo install -m 755 conductor /usr/local/bin/
sudo install -m 755 conductorctl /usr/local/bin/

# Verify installation
conductor --version
conductorctl --version
```

### 3. Install systemd Service (Optional)

```bash
# Create systemd user service directory
mkdir -p ~/.config/systemd/user

# Create service file
cat > ~/.config/systemd/user/conductor.service << 'EOF'
[Unit]
Description=Conductor Daemon - MIDI and Gamepad Macro Controller
After=network.target sound.target

[Service]
Type=simple
ExecStart=/usr/local/bin/conductor --foreground
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=default.target
EOF

# Reload systemd and enable service
systemctl --user daemon-reload
systemctl --user enable conductor
systemctl --user start conductor

# Check status
systemctl --user status conductor
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
- Gamepads: Xbox (360, One, Series X|S), PlayStation (DualShock 4, DualSense), Switch Pro Controller
- Joysticks: Flight sticks, arcade sticks
- Racing Wheels: Logitech, Thrustmaster, or any SDL2-compatible wheel
- HOTAS: Hands On Throttle And Stick systems
- Custom Controllers: Any SDL2-compatible HID device

You need at least one MIDI controller OR one game controller to use Conductor. Both can be used simultaneously.

#### 2. Software Requirements

**Rust Toolchain** (for building from source):

Conductor is written in Rust and requires the Rust compiler and Cargo build system.

**Check if Rust is already installed**:
```bash
rustc --version
cargo --version
```

If you see version numbers (e.g., `rustc 1.75.0`), skip to the next section.

**Install Rust using rustup**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and select the default installation. Then reload your shell:
```bash
source $HOME/.cargo/env
```

**Verify installation**:
```bash
rustc --version  # Should show: rustc 1.75.0 (or later)
cargo --version  # Should show: cargo 1.75.0 (or later)
```

**SDL2 Library** (for game controllers):

SDL2 is included via the `gilrs` v0.10 Rust crate. No additional installation required - it's built into Conductor automatically.

#### 3. Platform-Specific Requirements

**Ubuntu/Debian**:

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    libudev-dev \
    libusb-1.0-0-dev \
    libjack-jackd2-dev
```

**Fedora/RHEL**:

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    pkg-config \
    alsa-lib-devel \
    systemd-devel \
    libusbx-devel \
    jack-audio-connection-kit-devel
```

**Arch Linux**:

```bash
sudo pacman -S base-devel alsa-lib systemd-libs libusb jack2
```

#### 4. Game Controller Support (evdev)

**Install evdev and jstest**:

```bash
# Ubuntu/Debian
sudo apt install -y evdev joystick jstest-gtk

# Fedora/RHEL
sudo dnf install -y evdev joystick

# Arch Linux
sudo pacman -S linuxconsole
```

**Verify game controller detection**:
```bash
# List connected joysticks
ls /dev/input/js*

# Test a gamepad (if connected)
jstest /dev/input/js0
```

#### 5. udev Rules (Required for HID Access)

Create udev rules to allow non-root access to HID devices:

```bash
sudo tee /etc/udev/rules.d/50-conductor.rules << 'EOF'
# Native Instruments Maschine Mikro MK3
SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", MODE="0666", GROUP="plugdev"

# Generic MIDI devices
SUBSYSTEM=="usb", ATTRS{bInterfaceClass}=="01", ATTRS{bInterfaceSubClass}=="03", MODE="0666", GROUP="plugdev"

# Game Controllers (SDL2-compatible)
SUBSYSTEM=="input", ATTRS{name}=="*Xbox*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", ATTRS{name}=="*PlayStation*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", ATTRS{name}=="*PLAYSTATION*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", ATTRS{name}=="*DualShock*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", ATTRS{name}=="*DualSense*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", ATTRS{name}=="*Switch*", MODE="0666", GROUP="plugdev"

# Generic joystick/gamepad access
SUBSYSTEM=="input", KERNEL=="js[0-9]*", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="input", KERNEL=="event[0-9]*", MODE="0666", GROUP="plugdev"
EOF

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Add your user to plugdev group**:
```bash
sudo usermod -a -G plugdev $USER
sudo usermod -a -G input $USER

# Log out and back in for changes to take effect
```

**Verify group membership**:
```bash
groups | grep -E "plugdev|input"
```

### Building from Source

#### 1. Clone the Repository

```bash
# Choose a location for the project
cd ~/projects  # or wherever you keep code

# Clone the repository
git clone https://github.com/amiable-dev/conductor.git
cd conductor
```

#### 2. Build the Daemon

**Release build** (recommended for regular use):
```bash
# Build the entire workspace (daemon + core)
cargo build --release --workspace

# Or build just the daemon binary
cargo build --release --package conductor-daemon
```

The release build takes 2-5 minutes on modern hardware and produces an optimized binary (~3-5MB) in `target/release/conductor`.

**Build output**:
```
   Compiling conductor-core v3.0.0 (/home/you/projects/conductor/conductor-core)
   Compiling conductor-daemon v3.0.0 (/home/you/projects/conductor/conductor-daemon)
    Finished release [optimized] target(s) in 2m 14s
```

#### 3. Build the GUI (Optional)

```bash
# Install Node.js (if not already installed)
# Ubuntu/Debian
sudo apt install -y nodejs npm

# Fedora/RHEL
sudo dnf install -y nodejs npm

# Install frontend dependencies
cd conductor-gui/ui
npm ci

# Build the frontend
npm run build

# Build the Tauri backend
cd ../src-tauri
cargo build --release

# The GUI app bundle will be at:
# conductor-gui/src-tauri/target/release/conductor-gui
```

#### 4. Install Binaries

```bash
# Return to project root
cd ~/projects/conductor

# Install binaries
sudo install -m 755 target/release/conductor /usr/local/bin/
sudo install -m 755 target/release/conductorctl /usr/local/bin/

# Verify installation
conductor --version
conductorctl --version
```

## Verifying Device Connection

### Verifying MIDI Controller Connection

#### Check USB Device Enumeration

```bash
lsusb | grep -i "Native Instruments"
```

Expected output:
```
Bus 001 Device 010: ID 17cc:1600 Native Instruments Maschine Mikro MK3
```

#### Check ALSA MIDI Ports

```bash
aconnect -l
```

Expected output should list your MIDI controller:
```
client 24: 'Maschine Mikro MK3' [type=kernel]
    0 'Maschine Mikro MK3 MIDI 1'
```

#### Test MIDI Events

```bash
# Run diagnostic tool
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
- Check udev rules are loaded
- Verify user is in plugdev group

### Verifying Game Controller Connection

#### Check evdev Detection

```bash
# List input devices
ls -l /dev/input/js*
ls -l /dev/input/event*

# Example output:
# crw-rw---- 1 root plugdev 13, 0 Nov 21 10:00 /dev/input/js0
```

#### Check Permissions

```bash
# Check that your user has access
ls -l /dev/input/js0

# Should show group as plugdev with rw permissions
# crw-rw---- 1 root plugdev 13, 0 Nov 21 10:00 /dev/input/js0
```

#### Test Gamepad with jstest

```bash
# Install jstest if not already installed
sudo apt install -y joystick  # Ubuntu/Debian
sudo dnf install -y joystick  # Fedora
sudo pacman -S linuxconsole   # Arch

# Test the gamepad
jstest /dev/input/js0
```

You should see button and axis values update when you press buttons or move sticks.

**Press Ctrl+C to exit jstest**

#### Check via Conductor Status

```bash
# Start Conductor
conductor --foreground &

# Check status
conductorctl status

# Look for gamepad in device list
# Example output:
# Connected Devices:
#   - Xbox Wireless Controller (Gamepad)
```

#### Test Gamepad Events

Use Conductor's debug logging to verify gamepad inputs:

```bash
# Start Conductor with debug logging
DEBUG=1 conductor --foreground
```

Press buttons on your gamepad. You should see:
```
[GamepadButton] button:128 (A/Cross/B)
[GamepadButton] button:129 (B/Circle/A)
[GamepadAnalogStick] axis:128 value:255 (Left stick right)
```

If nothing appears:
- Check USB or Bluetooth connection
- Verify /dev/input/js* devices exist
- Check udev rules are loaded
- Verify user is in plugdev and input groups
- Try reconnecting the controller

#### Platform-Specific Troubleshooting

**Bluetooth Connection (BlueZ)**:

```bash
# Install Bluetooth tools
sudo apt install -y bluez bluez-tools  # Ubuntu/Debian
sudo dnf install -y bluez bluez-tools  # Fedora

# Enable Bluetooth service
sudo systemctl enable bluetooth
sudo systemctl start bluetooth

# Pair a controller using bluetoothctl
bluetoothctl
> scan on
> pair XX:XX:XX:XX:XX:XX
> connect XX:XX:XX:XX:XX:XX
> trust XX:XX:XX:XX:XX:XX
> exit
```

**Xbox Controllers**:
- Wireless controllers require `xpadneo` driver for best compatibility
- Install: `sudo apt install -y xpadneo` (Ubuntu/Debian)

**PlayStation Controllers**:
- Native support via `hid-playstation` kernel module
- For older systems, use `ds4drv`: `sudo pip3 install ds4drv`

**Permissions Issues**:
```bash
# If gamepad not accessible, check groups
groups

# Add user to required groups if missing
sudo usermod -a -G plugdev,input $USER

# Log out and log back in
```

## Configuration

### Using the GUI (Recommended)

v3.0.0 includes a visual configuration editor:

1. **Open Conductor GUI**:
   ```bash
   conductor-gui
   ```

2. **Connect your device** in the device panel (MIDI or gamepad)

3. **Use MIDI Learn mode**:
   - Click "Learn" next to any trigger
   - Press a button on your controller or gamepad
   - The trigger config auto-fills
   - Assign an action (keystroke, launch app, etc.)

4. **Save configuration** - automatically writes to `~/.config/conductor/config.toml`

See [GUI Quick Start](../getting-started/gui-quick-start.md) for detailed tutorial.

### Manual Configuration (Advanced)

If you prefer to edit `config.toml` manually:

**Config location**: `~/.config/conductor/config.toml`

**Create a minimal config**:
```bash
mkdir -p ~/.config/conductor

cat > ~/.config/conductor/config.toml << 'EOF'
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
command = "killall conductor"
EOF
```

This creates a basic configuration with:
- One mode (Default)
- One MIDI test mapping (pad 12 = Ctrl+C)
- One gamepad test mapping (A button = Ctrl+V)
- One emergency exit (hold pad 0 to quit)

**Hot-reload**: The daemon automatically reloads config within 0-10ms when you save changes.

## Running Conductor

### Using the Daemon with systemd

The recommended way to run Conductor on Linux:

**Start the service**:
```bash
systemctl --user start conductor
```

**Check status**:
```bash
systemctl --user status conductor
conductorctl status
```

**Enable auto-start on boot**:
```bash
systemctl --user enable conductor
```

**Control the daemon**:
```bash
# Reload configuration
conductorctl reload

# Stop daemon
systemctl --user stop conductor

# Restart daemon
systemctl --user restart conductor

# View logs
journalctl --user -u conductor -f
```

### Using the GUI

```bash
# Launch the GUI (starts daemon automatically)
conductor-gui
```

The GUI provides:
- Real-time event console
- Visual configuration editor
- Device status monitoring
- Daemon control (pause/resume/reload)

### Manual Mode (Development/Testing)

For testing or development:

```bash
# Run in foreground
conductor --foreground

# Run with debug logging
DEBUG=1 conductor --foreground

# Run with specific config
conductor --config ~/my-config.toml --foreground
```

## Troubleshooting

### Build Errors

**Error**: `error: linker 'cc' not found`

**Solution**: Install build tools:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora/RHEL
sudo dnf install gcc gcc-c++

# Arch Linux
sudo pacman -S base-devel
```

---

**Error**: `could not compile 'alsa-sys'`

**Solution**: Install ALSA development libraries:
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev

# Fedora/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

---

**Error**: `could not compile 'libudev-sys'`

**Solution**: Install udev development libraries:
```bash
# Ubuntu/Debian
sudo apt install libudev-dev

# Fedora/RHEL
sudo dnf install systemd-devel

# Arch Linux
sudo pacman -S systemd-libs
```

---

### Runtime Errors - MIDI

**Error**: `No MIDI input ports available`

**Solution**:
1. Check USB connection: `lsusb`
2. Verify ALSA sees device: `aconnect -l`
3. Check udev rules are loaded
4. Verify user is in plugdev group

---

**Error**: `Permission denied opening MIDI device`

**Solution**:
1. Check udev rules: `ls -l /dev/snd/*`
2. Add user to audio group: `sudo usermod -a -G audio $USER`
3. Log out and back in
4. Verify: `groups | grep audio`

---

### Runtime Errors - Game Controllers

**Error**: `Gamepad not detected`

**Solution**:
1. Check /dev/input: `ls -l /dev/input/js*`
2. Test with jstest: `jstest /dev/input/js0`
3. Check udev rules are loaded: `sudo udevadm control --reload-rules`
4. Verify groups: `groups | grep -E "plugdev|input"`
5. Check debug output: `DEBUG=1 conductor --foreground`

---

**Error**: `Permission denied: /dev/input/js0`

**Solution**:
1. Check file permissions: `ls -l /dev/input/js0`
2. Verify udev rules: `cat /etc/udev/rules.d/50-conductor.rules`
3. Add user to plugdev: `sudo usermod -a -G plugdev,input $USER`
4. Reload udev: `sudo udevadm control --reload-rules && sudo udevadm trigger`
5. Log out and back in

---

**Error**: `Gamepad buttons not responding`

**Solution**:
1. Use MIDI Learn to discover correct button IDs
2. Verify button IDs are in range 128-255 (not 0-127)
3. Test in jstest to verify hardware works
4. Check that gamepad appears in `conductorctl status`
5. Try USB connection instead of Bluetooth

---

**Error**: `Analog stick not working`

**Solution**:
1. Check axis IDs in jstest: `jstest /dev/input/js0`
2. Verify axis IDs match config (128-133)
3. Check dead zone settings
4. Use button triggers instead of analog for precise control

---

### Bluetooth Gamepad Issues

**Controller not pairing**:

```bash
# Install BlueZ tools
sudo apt install -y bluez bluez-tools

# Enable Bluetooth
sudo systemctl enable bluetooth
sudo systemctl start bluetooth

# Pair controller
bluetoothctl
> power on
> agent on
> default-agent
> scan on
# Wait for controller to appear
> pair XX:XX:XX:XX:XX:XX
> connect XX:XX:XX:XX:XX:XX
> trust XX:XX:XX:XX:XX:XX
> exit
```

**Controller connects but not detected**:
1. Check /dev/input: `ls /dev/input/js*`
2. Verify udev rules for Bluetooth devices
3. Check dmesg for errors: `dmesg | tail -20`
4. Try USB connection as fallback

### systemd Service Issues

**Service won't start**:

```bash
# Check service status
systemctl --user status conductor

# View logs
journalctl --user -u conductor -n 50

# Common issues:
# - Binary not in PATH
# - Config file missing or invalid
# - Permissions issues
```

**Service stops after logout**:

```bash
# Enable lingering to keep user services running
loginctl enable-linger $USER
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
3. Enable debug logging: `DEBUG=1 conductor --foreground`
4. Check system logs: `journalctl --user -u conductor -f`
5. File an issue on GitHub with:
   - Linux distribution and version
   - Kernel version (`uname -r`)
   - Device model (MIDI or gamepad)
   - Error messages from logs
   - Output of `cargo --version` and `rustc --version`

---

**Last Updated**: November 21, 2025 (v3.0.0)
**Linux Support**: systemd-based distributions
**Architecture**: x86_64, ARM64
**Input Support**: MIDI Controllers + Game Controllers (HID)
