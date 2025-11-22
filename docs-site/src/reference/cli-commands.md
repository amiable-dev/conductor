# CLI Commands Reference

## Overview

Conductor provides a daemon service, control utility, and several diagnostic tools, all accessible via the command line. This reference covers all available commands, their options, and usage examples.

**v1.0.0+** introduces daemon architecture with background service and hot-reload capabilities.

## Daemon Service: conductor

The primary Conductor daemon service (v1.0.0+). Runs as a background process with config hot-reload.

### Basic Syntax

```bash
# Start daemon (via cargo)
cargo run --release --bin conductor [PORT] [OPTIONS]

# Start daemon (release binary)
./target/release/conductor [PORT] [OPTIONS]

# Or use systemd/launchd (see Installation)
systemctl --user start conductor  # Linux
launchctl load ~/Library/LaunchAgents/com.amiable.conductor.plist  # macOS
```

### Daemon Features (v1.0.0+)

- **Background Service**: Runs continuously in the background
- **Config Hot-Reload**: Reload configuration without restart (0-8ms latency)
- **State Persistence**: Saves state on shutdown, restores on startup
- **IPC Control**: Control via `conductorctl` utility
- **Auto-Recovery**: Graceful error handling and device reconnection

### Arguments

#### PORT (Required in some cases)

The MIDI input port number to connect to.

**Finding available ports**:
```bash
# List all MIDI ports
cargo run --release

# Or
./target/release/conductor
```

Output:
```
Available MIDI input ports:
0: USB MIDI Device
1: IAC Driver Bus 1
2: Maschine Mikro MK3 - Input
3: Digital Keyboard
```

**Usage**:
```bash
# Connect to port 2 (Maschine Mikro MK3)
cargo run --release 2

# Connect to port 0
cargo run --release 0
```

**Note**: If `auto_connect = true` in `config.toml`, the port argument is optional and Conductor will connect to the first available port.

### Options

#### --led, --lighting <SCHEME>

Select LED lighting scheme (for devices with LED support).

**Syntax**:
```bash
cargo run --release 2 --led <SCHEME>
# or
cargo run --release 2 --lighting <SCHEME>
```

**Available schemes**:
- `reactive` (default) - Velocity-based colors, fade out after release
- `rainbow` - Static rainbow gradient
- `breathing` - Breathing effect (all pads)
- `pulse` - Pulsing effect
- `wave` - Wave pattern with brightness gradient
- `sparkle` - Random twinkling LEDs
- `vumeter` - VU meter style gradient (green → yellow → red)
- `spiral` - Spiral/diagonal pattern
- `static` - Static single color
- `off` - LEDs disabled

**Examples**:
```bash
# Reactive mode (velocity-sensitive)
cargo run --release 2 --led reactive

# Rainbow gradient
cargo run --release 2 --led rainbow

# Breathing effect
cargo run --release 2 --lighting breathing

# Turn off LEDs
cargo run --release 2 --led off
```

**LED Behavior by Scheme**:

**reactive**:
- Soft press (velocity < 50): Green Dim
- Medium press (50 ≤ velocity < 100): Yellow Normal
- Hard press (velocity ≥ 100): Red Bright
- Fades out 1 second after release

**rainbow**:
- Static rainbow gradient across all pads
- No animation (constant colors)

**sparkle**:
- Random white LEDs
- 20% probability per pad per frame
- Updates every 100ms

**vumeter**:
- Green (bottom rows)
- Yellow/Orange (middle)
- Red (top)

**wave**:
- Blue with varying brightness
- Creates wave effect

#### --profile, -p <PATH>

Load a Native Instruments Controller Editor profile (.ncmm3 file).

**Syntax**:
```bash
cargo run --release 2 --profile <PATH>
# or
cargo run --release 2 -p <PATH>
```

**Examples**:
```bash
# Relative path
cargo run --release 2 --profile my-profile.ncmm3

# Absolute path
cargo run --release 2 --profile ~/Downloads/base-template-ni-mikro-mk3.ncmm3

# macOS default location
cargo run --release 2 --profile "$HOME/Documents/Native Instruments/Controller Editor/Profiles/my-profile.ncmm3"
```

**What profiles do**:
- Map physical pad positions to MIDI note numbers
- Support 8 pad pages (A-H) per profile
- Enable correct LED feedback for custom layouts
- Allow seamless integration with NI Controller Editor

**Creating profiles**:
1. Open Native Instruments Controller Editor
2. Select "Maschine Mikro MK3"
3. Edit pad pages (A-H)
4. Assign MIDI notes to each pad
5. Save as `.ncmm3` file
6. Use with `--profile` flag

See [Device Profiles Documentation](../DEVICE_PROFILES.md) for complete guide.

#### --pad-page <PAGE>

Force a specific pad page when using a profile (instead of auto-detection).

**Syntax**:
```bash
cargo run --release 2 --profile <PATH> --pad-page <PAGE>
```

**Valid pages**: A, B, C, D, E, F, G, H (case-insensitive)

**Examples**:
```bash
# Force page A
cargo run --release 2 --profile my-profile.ncmm3 --pad-page A

# Force page H
cargo run --release 2 --profile my-profile.ncmm3 --pad-page h
```

**When to use**:
- Auto-detection not working correctly
- Want to lock to a specific page
- Testing specific page mappings
- Profile has identical notes across multiple pages

**Default behavior** (without `--pad-page`):
- Auto-detect active page from incoming MIDI notes
- Switch pages automatically when notes from different page detected

#### --config, -c <PATH>

Specify custom configuration file location.

**Syntax**:
```bash
cargo run --release 2 --config <PATH>
# or
cargo run --release 2 -c <PATH>
```

**Examples**:
```bash
# Use alternative config
cargo run --release 2 --config config-dev.toml

# Full path
cargo run --release 2 --config /etc/conductor/config.toml
```

**Default**: `./config.toml` (current directory)

#### --help, -h

Display help message with all available options.

**Syntax**:
```bash
cargo run --release -- --help
# or
./target/release/conductor --help
```

Note the `--` separator when using `cargo run`.

#### --version, -v

Display Conductor version.

**Syntax**:
```bash
cargo run --release -- --version
# or
./target/release/conductor --version
```

### Environment Variables

#### DEBUG=1

Enable verbose debug logging.

**Syntax**:
```bash
DEBUG=1 cargo run --release 2
```

**Output includes**:
- MIDI event details (note on/off, velocity, channel)
- HID connection status
- LED updates (buffer contents)
- Event processing (velocity detection, long press, chords)
- Mapping matches and action execution
- Mode changes
- Error details

**Example debug output**:
```
[DEBUG] Connected to MIDI port 2: Maschine Mikro MK3 - Input
[DEBUG] HID device opened successfully
[DEBUG] LED controller initialized
[DEBUG] Loaded config with 3 modes, 24 mappings
[DEBUG] Starting in mode 0: Default

[MIDI] NoteOn ch:0 note:12 vel:87
[DEBUG] Processed: Note(12) with velocity Medium
[DEBUG] Matched mapping: "Copy text" (mode: Default)
[DEBUG] Executing action: Keystroke(keys: "c", modifiers: ["cmd"])
[DEBUG] LED update: pad 0 -> color 7 (Green) brightness 2
[MIDI] NoteOff ch:0 note:12 vel:0
[DEBUG] LED fade: pad 0 cleared after 1000ms
```

**When to use**:
- Troubleshooting mapping issues
- Debugging note number mismatches
- Verifying LED control
- Understanding event processing
- Investigating performance issues

#### RUST_LOG

Control Rust logging levels (for development).

**Syntax**:
```bash
RUST_LOG=debug cargo run --release 2
RUST_LOG=trace cargo run --release 2
RUST_LOG=info cargo run --release 2
```

**Levels**:
- `error` - Only errors
- `warn` - Warnings and errors
- `info` - General information (default)
- `debug` - Debug information
- `trace` - Very verbose

**Filter by module**:
```bash
# Only log MIDI events
RUST_LOG=conductor::midi=debug cargo run --release 2

# Multiple modules
RUST_LOG=conductor::event_processor=debug,conductor::mappings=trace cargo run --release 2
```

### Complete Usage Examples

#### Example 1: Basic Usage

```bash
# List ports
cargo run --release

# Connect to port 2 with default settings
cargo run --release 2
```

#### Example 2: With LED Lighting

```bash
# Reactive mode (default)
cargo run --release 2 --led reactive

# Rainbow gradient
cargo run --release 2 --led rainbow

# Sparkle effect
cargo run --release 2 --led sparkle
```

#### Example 3: With Profile

```bash
# Auto-detect page
cargo run --release 2 --profile my-profile.ncmm3

# Force specific page
cargo run --release 2 --profile my-profile.ncmm3 --pad-page H

# With LED scheme
cargo run --release 2 --profile my-profile.ncmm3 --led reactive
```

#### Example 4: Debug Mode

```bash
# Enable debug output
DEBUG=1 cargo run --release 2

# With all options
DEBUG=1 cargo run --release 2 --profile my-profile.ncmm3 --led reactive
```

#### Example 5: Custom Config

```bash
# Use development config
cargo run --release 2 --config config-dev.toml

# Use config from different directory
cargo run --release 2 --config ~/conductor-configs/work.toml
```

#### Example 6: Production Binary

```bash
# Build release
cargo build --release

# Run with all options
./target/release/conductor 2 \
    --profile ~/Documents/NI/my-profile.ncmm3 \
    --led reactive \
    --config ~/conductor-configs/production.toml
```

## Daemon Control: conductorctl

**New in v1.0.0** - Control and monitor the Conductor daemon service.

### Basic Syntax

```bash
# Via cargo
cargo run --release --bin conductorctl <COMMAND> [OPTIONS]

# Release binary
./target/release/conductorctl <COMMAND> [OPTIONS]
```

### Commands

#### status

Display daemon status, device info, and performance metrics.

**Syntax**:
```bash
conductorctl status [--json]
```

**Output** (human-readable):
```
Conductor Daemon Status
=====================

Lifecycle State: Running
Uptime: 2h 34m 17s

Device
------
Connected: Yes
Name: Maschine Mikro MK3 - Input
Port: 2
Last Event: 3s ago

Configuration
------------
Modes: 3 (Default, Development, Media)
Global Mappings: 12
Mode Mappings: 24 (8 per mode)
Config File: /Users/you/.config/conductor/config.toml
Last Reload: 15m ago

Performance Metrics
------------------
Config Reloads: 7
Average Reload Time: 3ms
Last Reload Time: 2ms
Performance Grade: Excellent

IPC Latency: <1ms
```

**JSON Output** (`--json`):
```bash
conductorctl status --json
```

```json
{
  "success": true,
  "data": {
    "lifecycle_state": "Running",
    "uptime_secs": 9257,
    "device": {
      "connected": true,
      "name": "Maschine Mikro MK3 - Input",
      "port": 2,
      "last_event_at": 1699900000
    },
    "config": {
      "modes": 3,
      "global_mappings": 12,
      "mode_mappings": 24
    },
    "performance": {
      "reload_count": 7,
      "avg_reload_ms": 3,
      "last_reload_ms": 2,
      "grade": "Excellent"
    }
  }
}
```

#### reload

Trigger configuration hot-reload without restarting the daemon.

**Syntax**:
```bash
conductorctl reload [--json]
```

**Features**:
- **Zero Downtime**: No interruption to MIDI processing
- **Fast**: 0-8ms reload latency (typically <3ms)
- **Atomic**: All-or-nothing config swap
- **Validated**: Config checked before reload

**Output**:
```
✓ Configuration reloaded successfully

Reload completed in 2ms
Modes: 3 (Default, Development, Media)
Global mappings: 12
Mode mappings: 24
```

**When to use**:
- After editing `config.toml`
- Testing new mappings
- Switching between config profiles
- Live development workflow

**Example workflow**:
```bash
# 1. Edit config
vim ~/.config/conductor/config.toml

# 2. Reload daemon
conductorctl reload

# 3. Test changes immediately (no restart needed!)
```

#### validate

Validate configuration file syntax without reloading.

**Syntax**:
```bash
conductorctl validate [--json]
```

**Output** (valid config):
```
✓ Configuration is valid

Modes: 3
Global mappings: 12
Total mappings: 36
```

**Output** (invalid config):
```
✗ Configuration validation failed

Error: Invalid trigger type 'NoteTap' at line 42
Expected one of: Note, VelocityRange, LongPress, DoubleTap,
                 NoteChord, EncoderTurn, Aftertouch, PitchBend, CC

Suggestion: Did you mean 'DoubleTap'?
```

**When to use**:
- Before committing config changes
- CI/CD validation
- Debugging config syntax errors
- Pre-flight checks

#### ping

Health check with latency measurement.

**Syntax**:
```bash
conductorctl ping [--json]
```

**Output**:
```
✓ Daemon is responsive
Latency: 0.4ms
```

**When to use**:
- Verify daemon is running
- Check IPC communication
- Monitor system responsiveness
- Health checks in scripts

#### shutdown

Gracefully shut down the **running daemon process** via IPC.

**Syntax**:
```bash
conductorctl shutdown [--json]
```

**Output**:
```
✓ Daemon stopped successfully

Uptime: 2h 34m 17s
State saved successfully
```

**What it does**:
- Sends IPC `Stop` command to running daemon
- Daemon saves state and exits gracefully
- Closes MIDI/HID connections cleanly
- Persists device state to disk

**When to use**:
- **Daemon is running** and you want to stop it
- Quick shutdown during development
- When you know daemon is responsive

**Requirements**:
- Daemon must be running
- IPC socket must be accessible
- No LaunchAgent/systemd interaction

#### stop

Stop the **LaunchAgent/systemd service** (tries graceful shutdown first).

**Syntax**:
```bash
conductorctl stop [--json] [--force]
```

**Output**:
```
Stopping Conductor service...
  Attempting graceful shutdown via IPC...
✓ Service stopped successfully
```

**What it does**:
1. **First**: Attempts graceful IPC shutdown (same as `shutdown` command)
2. **Waits**: 500ms for daemon to exit
3. **Then**: Unloads LaunchAgent (macOS) or stops systemd service (Linux)

**When to use**:
- **Service is installed** via `conductorctl install`
- Need to stop the background service
- Daemon may or may not be responsive
- Want to ensure service is fully stopped

**Requirements**:
- Service must be installed (via `conductorctl install`)
- macOS: LaunchAgent plist exists
- Linux: systemd unit exists

**Options**:
- `--force`: Skip graceful shutdown, immediately unload service

### Choosing Between shutdown and stop

| Scenario | Use Command | Why |
|----------|-------------|-----|
| Daemon running in foreground | `shutdown` | Faster, direct IPC |
| Daemon started manually (not service) | `shutdown` | No service to unload |
| Service installed and running | `stop` | Ensures service unloaded |
| Daemon not responding | `stop --force` | Bypasses IPC, forces unload |
| Development workflow | `shutdown` | Quick restarts |
| Production/installed service | `stop` | Proper service management |

**Decision tree**:
```
Is daemon installed as a service?
├─ No → Use `shutdown`
└─ Yes
   ├─ Is daemon responsive?
   │  ├─ Yes → Use `stop` (graceful)
   │  └─ No → Use `stop --force`
   └─ Running in foreground for testing? → Use `shutdown`
```

### Device Management Commands

#### list-devices

List all available MIDI input devices.

**Syntax**:
```bash
conductorctl list-devices [--json]
```

**Output**:
```
Available MIDI Devices
──────────────────────────────────────────────────
  [0] USB MIDI Device
  [1] IAC Driver Bus 1 (connected)
  [2] Maschine Mikro MK3 - Input
```

**When to use**:
- Find available MIDI ports
- Check which device is currently connected
- Troubleshoot device connectivity

#### set-device

Switch the daemon to a different MIDI device without restart.

**Syntax**:
```bash
conductorctl set-device <PORT> [--json]
```

**Example**:
```bash
# Switch to port 2
conductorctl set-device 2
```

**Output**:
```
✓ Switched to device at port 2
```

**When to use**:
- Switch between MIDI devices on the fly
- Test different controllers
- Recover from device disconnection

#### get-device

Show information about the currently connected MIDI device.

**Syntax**:
```bash
conductorctl get-device [--json]
```

**Output**:
```
Current MIDI Device
──────────────────────────────────────────────────
Status:     Connected
Name:       Maschine Mikro MK3 - Input
Port:       2
Last Event: 3s ago
```

**When to use**:
- Verify which device is active
- Check connection status
- Debug event reception issues

### Service Management Commands

**Note**: Service management commands are currently macOS-only (using LaunchAgent).

#### Understanding LaunchAgent Behavior

The Conductor LaunchAgent plist has `RunAtLoad=true`, which affects command behavior:

**Key insight**: `launchctl load` = load plist + start daemon immediately

| Command | launchctl operation | Starts daemon? | Auto-start on login? |
|---------|-------------------|----------------|---------------------|
| `install` | `load` | ✓ Yes | ✗ No |
| `start` | `load` | ✓ Yes | ✗ No |
| `enable` | `load -w` | ✓ Yes | ✓ Yes |
| `stop` | `unload` | ✗ Stops | ✗ No |
| `disable` | `unload -w` | ✗ Stops | ✗ No |

**Common patterns**:
- **Quick setup**: `install` → daemon runs but won't auto-start on reboot
- **Production setup**: `install` then `enable` → daemon runs now AND on every login
- **One-step production**: `enable` (if already installed) → starts + enables auto-start
- **Temporary disable**: `stop` → stops now but will auto-start on next login (if enabled)
- **Complete disable**: `disable` → stops now AND prevents auto-start

#### install

Install Conductor as a LaunchAgent service that starts automatically on login.

**Syntax**:
```bash
conductorctl install [--install-binary] [--force] [--json]
```

**Options**:
- `--install-binary`: Copy daemon binary to `/usr/local/bin/conductor`
- `--force`: Reinstall even if already installed

**What it does**:
1. Generates LaunchAgent plist from template
2. Copies plist to `~/Library/LaunchAgents/com.amiable.conductor.plist`
3. Optionally installs binary to `/usr/local/bin` (requires sudo)
4. Loads service with `launchctl`

**Output**:
```
Installing Conductor service...
  ✓ Generated plist
  ✓ Installed to ~/Library/LaunchAgents/com.amiable.conductor.plist
  ✓ Loaded service

✓ Conductor service installed successfully

Next steps:
  • Start: conductorctl start
  • Enable auto-start: conductorctl enable
  • Check status: conductorctl service-status
```

**When to use**:
- First-time setup for background service
- Setting up production deployment
- Enable auto-start on login

**Example**:
```bash
# Basic install (daemon must already be built)
conductorctl install

# Install and copy binary to system location
sudo conductorctl install --install-binary

# Force reinstall
conductorctl install --force
```

#### uninstall

Remove Conductor service from LaunchAgent.

**Syntax**:
```bash
conductorctl uninstall [--remove-binary] [--remove-logs] [--json]
```

**Options**:
- `--remove-binary`: Also delete `/usr/local/bin/conductor`
- `--remove-logs`: Delete log files

**What it does**:
1. Stops service if running
2. Removes LaunchAgent plist
3. Optionally removes binary and logs

**Output**:
```
Uninstalling Conductor service...
  ✓ Stopped service
  ✓ Removed plist: ~/Library/LaunchAgents/com.amiable.conductor.plist

✓ Conductor service uninstalled successfully
```

**When to use**:
- Removing Conductor completely
- Clean uninstall before upgrade
- Troubleshooting installation issues

#### start

Start the LaunchAgent service.

**Syntax**:
```bash
conductorctl start [--wait <SECONDS>] [--json]
```

**Options**:
- `--wait <SECONDS>`: Wait up to N seconds for daemon to be ready (default: 5)

**What it does**:
1. Loads service with `launchctl load`
2. Waits for daemon to respond to IPC
3. Verifies daemon is running

**Output**:
```
Starting Conductor service...
Waiting for daemon to be ready... ✓
✓ Service started successfully
```

**When to use**:
- Start service after install
- Start service after stop
- Verify service starts correctly

**Example**:
```bash
# Start and wait up to 10 seconds
conductorctl start --wait 10
```

#### restart

Restart the LaunchAgent service (stop + start).

**Syntax**:
```bash
conductorctl restart [--wait <SECONDS>] [--json]
```

**What it does**:
1. Gracefully stops service
2. Waits 500ms
3. Starts service
4. Waits for daemon to be ready

**Output**:
```
Restarting Conductor service...
Stopping Conductor service...
✓ Service stopped successfully
Starting Conductor service...
✓ Service started successfully
```

**When to use**:
- Apply config changes that need full restart
- Recover from errors
- Test service lifecycle

#### enable

Enable auto-start on login AND start the daemon immediately.

**Syntax**:
```bash
conductorctl enable [--json]
```

**What it does**:
1. **Loads service** with `launchctl load -w` flag
2. **Starts daemon immediately** (because plist has `RunAtLoad=true`)
3. **Enables auto-start** on next login (persists across reboots)

**Output**:
```
✓ Service enabled (will start on login)
```

**Note**: This is equivalent to `start` + making it persistent across reboots.

**When to use**:
- **One-step setup**: Enable and start in single command
- Production deployment (start now + auto-start on reboot)
- After `disable` to re-enable everything

**Alternative**: If you only want to enable auto-start WITHOUT starting now, use `start` to load the service first, then the `-w` flag will be set.

#### disable

Disable auto-start on login AND stop the daemon immediately.

**Syntax**:
```bash
conductorctl disable [--json]
```

**What it does**:
1. **Unloads service** with `launchctl unload -w` flag
2. **Stops daemon immediately**
3. **Disables auto-start** on next login

**Output**:
```
✓ Service disabled (will not start on login)
```

**Note**: This is equivalent to `stop` + preventing auto-start on reboot.

**When to use**:
- **Complete shutdown**: Stop now + prevent auto-start
- Temporarily disable background service
- Development workflow
- Before uninstall

#### service-status

Show detailed service installation and runtime status.

**Syntax**:
```bash
conductorctl service-status [--json]
```

**Output**:
```
Conductor Service Status
──────────────────────────────────────────────────
Status:          Installed and Loaded
Service Label:   com.amiable.conductor
Plist:           ~/Library/LaunchAgents/com.amiable.conductor.plist ✓
Binary:          /usr/local/bin/conductor ✓

Service is loaded (enabled)
```

**When to use**:
- Verify service is installed correctly
- Check if auto-start is enabled
- Troubleshoot service issues
- Audit service configuration

### Global Options

#### --json

Output in JSON format (for scripting/automation).

**Available for**: All commands

**Example**:
```bash
# Parse with jq
conductorctl status --json | jq '.data.device.connected'
# Output: true

# Check if reload succeeded
if conductorctl reload --json | jq -e '.success'; then
    echo "Reload successful"
fi
```

### Usage Examples

#### Example 1: First-Time Service Setup (Production)

```bash
# Build daemon
cargo build --release --bin conductor

# Install as LaunchAgent service (starts immediately)
conductorctl install

# Verify running
conductorctl status

# Enable auto-start on login (also starts if not running)
# Since install already started it, this just enables persistence
conductorctl enable

# Check service details
conductorctl service-status
```

**Alternative (simpler)**:
```bash
# Build daemon
cargo build --release --bin conductor

# One-step: Install service
conductorctl install

# One-step: Enable auto-start (if you want persistence across reboots)
conductorctl enable
```

**Simplest production setup**:
```bash
cargo build --release --bin conductor
conductorctl install --install-binary  # Copies to /usr/local/bin
conductorctl enable                     # Starts + enables auto-start
```

#### Example 2: Development Workflow

```bash
# Start daemon in foreground for testing
cargo run --release --bin conductor 2 --foreground

# In another terminal...

# Check status
conductorctl status

# Edit config
vim config.toml

# Hot-reload changes (zero downtime!)
conductorctl reload

# Test changes immediately

# Switch to different MIDI device
conductorctl list-devices
conductorctl set-device 1

# Stop when done
conductorctl shutdown
```

#### Example 3: Service Management Workflow

```bash
# Check if service is installed
conductorctl service-status

# Start service if not running
conductorctl start

# Check which MIDI device is active
conductorctl get-device

# Switch to different device without restart
conductorctl set-device 2

# Restart service (apply changes that need full restart)
conductorctl restart

# Disable auto-start temporarily
conductorctl disable

# Stop service
conductorctl stop
```

#### Example 4: Production Monitoring

```bash
#!/bin/bash
# monitor.sh - Health check script

# Check daemon health
if ! conductorctl ping --json | jq -e '.success'; then
    echo "Daemon not responding, restarting..."
    conductorctl restart
fi

# Get performance metrics
RELOAD_MS=$(conductorctl status --json | jq '.data.reload_stats.avg_reload_ms')
if [ "$RELOAD_MS" -gt 50 ]; then
    echo "Warning: Average reload time ${RELOAD_MS}ms (expected <50ms)"
fi

# Check MIDI device connectivity
CONNECTED=$(conductorctl get-device --json | jq '.data.device.connected')
if [ "$CONNECTED" != "true" ]; then
    echo "MIDI device disconnected!"
    # Try to reconnect
    conductorctl list-devices
fi
```

#### Example 5: Configuration Management

```bash
# Validate before deploy
if ! conductorctl validate --json | jq -e '.success'; then
    echo "Config validation failed"
    exit 1
fi

# Backup current config
cp ~/.config/conductor/config.toml ~/.config/conductor/config.toml.backup

# Deploy new config
cp config-v2.toml ~/.config/conductor/config.toml

# Apply changes (hot reload)
conductorctl reload

# Verify successful reload
conductorctl status

# If issues, rollback
# cp ~/.config/conductor/config.toml.backup ~/.config/conductor/config.toml
# conductorctl reload
```

#### Example 6: Automated Testing

```bash
#!/bin/bash
# test-config.sh

# Validate syntax
if ! conductorctl validate --json | jq -e '.data.valid'; then
    echo "Config validation failed"
    exit 1
fi

# Reload daemon
if ! conductorctl reload --json | jq -e '.success'; then
    echo "Reload failed"
    exit 1
fi

# Check reload performance
LATENCY=$(conductorctl status --json | jq '.data.reload_stats.last_reload_ms')
if [ "$LATENCY" -gt 10 ]; then
    echo "Warning: Reload took ${LATENCY}ms (expected <10ms)"
fi

# Verify device connection
CONNECTED=$(conductorctl get-device --json | jq '.data.device.connected')
if [ "$CONNECTED" != "true" ]; then
    echo "Error: MIDI device not connected"
    exit 1
fi

echo "✓ All checks passed"
```

#### Example 7: Complete Uninstall

```bash
# Stop service
conductorctl stop

# Disable auto-start
conductorctl disable

# Uninstall service (with cleanup)
conductorctl uninstall --remove-binary --remove-logs

# Verify removal
conductorctl service-status
```

## Diagnostic Tools

Conductor includes several diagnostic utilities for debugging and configuration.

### midi_diagnostic

Visualize all incoming MIDI events in real-time.

**Purpose**: Debug MIDI connectivity, view raw MIDI data, verify device is sending events.

**Syntax**:
```bash
cargo run --bin midi_diagnostic [PORT]
```

**Example**:
```bash
# Connect to port 2
cargo run --bin midi_diagnostic 2
```

**Output**:
```
Connected to MIDI port 2: Maschine Mikro MK3 - Input
Listening for MIDI events... (Ctrl+C to exit)

[NoteOn]  ch:0 note:12 vel:87
[NoteOff] ch:0 note:12 vel:0
[NoteOn]  ch:0 note:13 vel:45
[NoteOff] ch:0 note:13 vel:0
[CC]      ch:0 cc:1 value:64
[PitchBend] ch:0 value:8192
```

**Event types shown**:
- `NoteOn` - Pad/key pressed
- `NoteOff` - Pad/key released
- `CC` - Control Change (knobs, sliders)
- `PitchBend` - Touch strip, pitch wheel
- `Aftertouch` - Pressure sensitivity
- `ProgramChange` - Program/patch change

**When to use**:
- Verify MIDI device is connected
- Find note numbers for pads/keys
- Debug why mappings aren't triggering
- Check velocity ranges
- Verify CC numbers for encoders/knobs

**Press Ctrl+C to exit.**

### led_diagnostic

Test LED functionality and HID connection.

**Purpose**: Verify HID access, test LED control, debug LED issues.

**Syntax**:
```bash
cargo run --bin led_diagnostic
```

**What it does**:
1. Attempts to open HID device
2. Displays connection status
3. Tests individual LED control
4. Cycles through all pads with different colors

**Output**:
```
LED Diagnostic Tool
==================

Searching for Maschine Mikro MK3...
✓ Device found: Maschine Mikro MK3 (17cc:1600)
✓ HID device opened successfully

Testing LED control...
- Lighting pad 0 (Red Bright)
- Lighting pad 1 (Green Bright)
- Lighting pad 2 (Blue Bright)
...
- Clearing all pads

✓ LED diagnostic complete
```

**Error output** (if HID not accessible):
```
✗ Failed to open HID device
  Possible causes:
  - Device not connected
  - Input Monitoring permission not granted
  - Native Instruments drivers not installed

  Solutions:
  1. Check USB connection
  2. Grant Input Monitoring permission (System Settings → Privacy & Security)
  3. Install NI drivers via Native Access
```

**When to use**:
- LEDs not working in main application
- Verify HID access before running conductor
- Test after granting permissions
- Debug LED coordinate mapping issues

### led_tester

Interactive LED testing tool.

**Purpose**: Manual control of individual LEDs for testing and debugging.

**Syntax**:
```bash
cargo run --bin led_tester
```

**Interactive mode**:
```
LED Tester - Interactive Mode
==============================

Commands:
  on <pad> <color> <brightness>  - Turn on LED
  off <pad>                      - Turn off LED
  all <color> <brightness>       - Set all LEDs
  clear                          - Clear all LEDs
  rainbow                        - Show rainbow pattern
  test                           - Cycle through all pads
  quit                           - Exit

> on 0 red 3
✓ Pad 0: Red Bright

> all blue 2
✓ All pads: Blue Normal

> rainbow
✓ Rainbow pattern displayed

> quit
```

**Colors**: red, orange, yellow, green, blue, purple, magenta, white

**Brightness**: 0 (off), 1 (dim), 2 (normal), 3 (bright)

**When to use**:
- Test specific pad LEDs
- Verify coordinate mapping
- Experiment with colors and brightness
- Debug LED patterns

### pad_mapper

Find MIDI note numbers for physical pads.

**Purpose**: Discover note numbers to use in `config.toml`.

**Syntax**:
```bash
cargo run --bin pad_mapper [PORT]
```

**Example**:
```bash
cargo run --bin pad_mapper 2
```

**Usage**:
1. Run the tool
2. Press each pad on your controller
3. Write down the note number displayed
4. Use those numbers in your config

**Output**:
```
Pad Mapper - Press pads to see note numbers
============================================
Connected to port 2: Maschine Mikro MK3 - Input

Press pads... (Ctrl+C to exit)

Pad pressed: Note 12 (velocity: 87)
Pad pressed: Note 13 (velocity: 64)
Pad pressed: Note 14 (velocity: 103)
Pad pressed: Note 15 (velocity: 52)
```

**Tips**:
- Press pads in order (bottom-left to top-right)
- Draw a grid on paper and write down note numbers
- Use this data to update `config.toml`

**When to use**:
- Setting up a new device
- Creating initial config
- Mapping custom devices
- Verifying profile note numbers

### test_midi

Test MIDI port connectivity and enumerate devices.

**Purpose**: Quick MIDI connectivity test, list all available ports.

**Syntax**:
```bash
cargo run --bin test_midi
```

**Output**:
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

Press a pad to test... (5 second timeout)
✓ Received MIDI event: NoteOn ch:0 note:12 vel:87

Connection test: PASSED
```

**When to use**:
- Verify MIDI device detected
- Check port numbers before running conductor
- Debug connectivity issues
- Confirm MIDI cable is working

## Command Quick Reference

| Command | Purpose | Example |
|---------|---------|---------|
| **Daemon Service (v1.0.0+)** |||
| `conductor [PORT]` | Start daemon service | `cargo run --release --bin conductor 2` |
| `--led <SCHEME>` | Set LED scheme | `conductor 2 --led rainbow` |
| `--profile <PATH>` | Load profile | `conductor 2 --profile my.ncmm3` |
| `--pad-page <PAGE>` | Force pad page | `conductor 2 --pad-page H` |
| `--config <PATH>` | Custom config | `conductor 2 --config dev.toml` |
| **Daemon Control (v1.0.0+)** |||
| `conductorctl status` | Show daemon status | `conductorctl status` |
| `conductorctl reload` | Hot-reload config | `conductorctl reload` |
| `conductorctl validate` | Validate config | `conductorctl validate` |
| `conductorctl ping` | Health check | `conductorctl ping` |
| `conductorctl shutdown` | Stop daemon (IPC) | `conductorctl shutdown` |
| `conductorctl stop` | Stop service (LaunchAgent) | `conductorctl stop --force` |
| **Device Management** |||
| `conductorctl list-devices` | List MIDI devices | `conductorctl list-devices` |
| `conductorctl set-device` | Switch MIDI device | `conductorctl set-device 2` |
| `conductorctl get-device` | Show current device | `conductorctl get-device` |
| **Service Management (macOS)** |||
| `conductorctl install` | Install LaunchAgent | `conductorctl install --install-binary` |
| `conductorctl uninstall` | Remove service | `conductorctl uninstall --remove-logs` |
| `conductorctl start` | Start service | `conductorctl start --wait 10` |
| `conductorctl restart` | Restart service | `conductorctl restart` |
| `conductorctl enable` | Enable auto-start | `conductorctl enable` |
| `conductorctl disable` | Disable auto-start | `conductorctl disable` |
| `conductorctl service-status` | Service status | `conductorctl service-status` |
| **Global Options** |||
| `--json` | JSON output | `conductorctl status --json` |
| **Diagnostic Tools** |||
| `DEBUG=1` | Enable debug log | `DEBUG=1 conductor 2` |
| `midi_diagnostic` | View MIDI events | `cargo run --bin midi_diagnostic 2` |
| `led_diagnostic` | Test LEDs | `cargo run --bin led_diagnostic` |
| `led_tester` | Interactive LED test | `cargo run --bin led_tester` |
| `pad_mapper` | Find note numbers | `cargo run --bin pad_mapper 2` |
| `test_midi` | Test MIDI ports | `cargo run --bin test_midi` |

## Common Workflows

### First-Time Setup

```bash
# 1. List available ports
cargo run --release

# 2. Test connectivity
cargo run --bin test_midi

# 3. Map pads to note numbers
cargo run --bin pad_mapper 2

# 4. Run with basic settings
cargo run --release 2

# 5. Test LEDs
cargo run --release 2 --led rainbow
```

### Troubleshooting

```bash
# 1. Check MIDI events are received
cargo run --bin midi_diagnostic 2

# 2. Enable debug logging
DEBUG=1 cargo run --release 2

# 3. Test LED control
cargo run --bin led_diagnostic

# 4. Verify port numbers
cargo run --bin test_midi
```

### Production Use

```bash
# Build optimized binary
cargo build --release

# Run with all options
./target/release/conductor 2 \
    --profile ~/profiles/work.ncmm3 \
    --led reactive \
    --config ~/configs/production.toml

# Or use shell script
#!/bin/bash
DEBUG=0 ./target/release/conductor 2 \
    --profile "$HOME/Documents/NI/my-profile.ncmm3" \
    --led reactive \
    > /tmp/conductor.log 2>&1 &
```

## See Also

- [macOS Installation](../installation/macos.md) - Platform-specific setup
- [Building Guide](../installation/building.md) - Build from source
- [Diagnostics Guide](../troubleshooting/diagnostics.md) - Detailed troubleshooting
- [Configuration Overview](../configuration/overview.md) - Config file structure

---

**Last Updated**: November 14, 2025
**Binary Version**: 1.0.0
