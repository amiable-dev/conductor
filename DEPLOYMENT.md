# MIDIMon Deployment Guide

This guide covers deploying MIDIMon daemon as a background service on macOS and Linux systems.

## Table of Contents

- [Quick Start](#quick-start)
- [macOS LaunchAgent](#macos-launchagent)
- [Linux systemd](#linux-systemd)
- [Manual Installation](#manual-installation)
- [Configuration](#configuration)
- [Monitoring and Logs](#monitoring-and-logs)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)

---

## Quick Start

### Prerequisites

- Rust toolchain (1.70+)
- MIDI device connected (e.g., Maschine Mikro MK3)
- macOS 11+ or Linux with systemd

### Build from Source

```bash
# Clone repository
git clone https://github.com/amiable/midimon.git
cd midimon

# Build release binaries
cargo build --release --workspace

# Install binaries (requires sudo)
sudo install -m 755 target/release/midimon /usr/local/bin/
sudo install -m 755 target/release/midimonctl /usr/local/bin/

# Install man pages
sudo mkdir -p /usr/local/share/man/man1
sudo install -m 644 midimon-daemon/docs/*.1 /usr/local/share/man/man1/
sudo mandb  # Update man page database (Linux)
```

---

## macOS LaunchAgent

### Installation

1. **Copy LaunchAgent plist**:
   ```bash
   # Edit the plist to replace USERNAME with your username
   sed "s/USERNAME/$USER/g" midimon-daemon/launchd/com.amiable.midimon.plist > \
     ~/Library/LaunchAgents/com.amiable.midimon.plist
   ```

2. **Create required directories**:
   ```bash
   mkdir -p ~/.config/midimon
   mkdir -p ~/.local/state/midimon
   mkdir -p ~/Library/Logs
   ```

3. **Create default configuration** (if not exists):
   ```bash
   cat > ~/.config/midimon/config.toml <<'EOF'
   [device]
   name = "Mikro"
   auto_connect = true

   [advanced_settings]
   chord_timeout_ms = 50
   double_tap_timeout_ms = 300
   hold_threshold_ms = 2000

   [[modes]]
   name = "Default"
   color = "blue"

   [[modes.mappings]]
   trigger = { type = "Note", note = 60 }
   action = { type = "Keystroke", keys = "cmd+space" }
   EOF
   ```

4. **Grant Input Monitoring permissions**:
   - Open **System Settings → Privacy & Security → Input Monitoring**
   - Add `/usr/local/bin/midimon` if prompted

5. **Load and start the service**:
   ```bash
   launchctl load ~/Library/LaunchAgents/com.amiable.midimon.plist
   launchctl start com.amiable.midimon
   ```

### Management

```bash
# Check status
launchctl list | grep midimon

# Stop service
launchctl stop com.amiable.midimon

# Restart service
launchctl stop com.amiable.midimon
launchctl start com.amiable.midimon

# Unload (disable auto-start)
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist

# View logs
tail -f ~/Library/Logs/midimon.log
tail -f ~/Library/Logs/midimon.error.log
```

### LaunchAgent Configuration

The plist file configures:
- **RunAtLoad**: Start at login
- **KeepAlive**: Restart if crashed (5s throttle)
- **Nice -5**: Higher priority for low latency
- **LimitLoadToSessionType Aqua**: Run in GUI session (required for input simulation)

---

## Linux systemd

### Installation

1. **Install system-wide** (recommended):
   ```bash
   # Copy service file
   sudo cp midimon-daemon/systemd/midimon.service /etc/systemd/system/

   # Reload systemd
   sudo systemctl daemon-reload
   ```

2. **Or install user-level**:
   ```bash
   # Copy to user systemd directory
   mkdir -p ~/.config/systemd/user
   cp midimon-daemon/systemd/midimon.service ~/.config/systemd/user/

   # Reload user systemd
   systemctl --user daemon-reload
   ```

3. **Create required directories**:
   ```bash
   mkdir -p ~/.config/midimon
   mkdir -p ~/.local/state/midimon
   mkdir -p ~/.local/share/midimon/logs
   ```

4. **Create default configuration** (see macOS section above)

5. **Set up udev rules for MIDI device access**:
   ```bash
   # Create udev rule for Maschine Mikro MK3
   echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTRS{idProduct}=="1600", MODE="0666"' | \
     sudo tee /etc/udev/rules.d/99-maschine-mikro.rules

   # Reload udev rules
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

6. **Enable and start service**:
   ```bash
   # System-wide
   sudo systemctl enable midimon
   sudo systemctl start midimon

   # Or user-level
   systemctl --user enable midimon
   systemctl --user start midimon
   ```

### Management

```bash
# System-wide commands
sudo systemctl status midimon
sudo systemctl restart midimon
sudo systemctl stop midimon
sudo systemctl disable midimon

# User-level commands
systemctl --user status midimon
systemctl --user restart midimon
systemctl --user stop midimon
systemctl --user disable midimon

# View logs
journalctl -u midimon -f
# Or for user service:
journalctl --user -u midimon -f
```

### systemd Configuration

The service file configures:
- **After=network.target sound.target**: Start after audio system
- **Restart=on-failure**: Restart if crashed (5s delay)
- **Nice=-5**: Higher priority
- **Security hardening**: NoNewPrivileges, ProtectSystem, etc.

---

## Manual Installation

### Running in Foreground

For testing or development:

```bash
# Run with default config
midimon

# Run with custom config
midimon --config /path/to/config.toml

# Run with debug logging
midimon --log-level debug

# Run without IPC (no midimonctl control)
midimon --no-ipc
```

### Running in Background

```bash
# Start in background
nohup midimon &> ~/.local/share/midimon/logs/midimon.log &

# Save PID
echo $! > ~/.local/state/midimon/midimon.pid

# Stop daemon
kill $(cat ~/.local/state/midimon/midimon.pid)
```

---

## Configuration

### Configuration File Location

Default: `~/.config/midimon/config.toml`

Override with `--config` flag or symlink to another location:
```bash
ln -s /path/to/my/config.toml ~/.config/midimon/config.toml
```

### Hot-Reload

The daemon automatically detects config file changes and reloads within 10ms:

```bash
# Edit config
vim ~/.config/midimon/config.toml

# Changes are auto-detected and applied
# Or force reload:
midimonctl reload
```

### Validation

Test config before applying:
```bash
midimonctl validate --config ~/.config/midimon/config.toml
```

---

## Monitoring and Logs

### Status Monitoring

```bash
# Check daemon status
midimonctl status

# JSON output for scripting
midimonctl --json status | jq

# Ping test
midimonctl ping
```

### Performance Monitoring

```bash
# Watch reload performance
watch -n 1 'midimonctl status | grep -A5 "Reload Performance"'

# Export metrics
midimonctl --json status | jq .data.reload_stats
```

### Log Files

**macOS**:
- Stdout: `~/Library/Logs/midimon.log`
- Stderr: `~/Library/Logs/midimon.error.log`

**Linux systemd**:
- Journal: `journalctl -u midimon`

**Manual deployment**:
- `~/.local/share/midimon/logs/midimon.log`

### Log Rotation

**macOS**:
```bash
# Add to /etc/newsyslog.d/midimon.conf
/Users/*/Library/Logs/midimon*.log 644 7 * @T00 GZ
```

**Linux**:
```bash
# Create /etc/logrotate.d/midimon
/home/*/.local/share/midimon/logs/*.log {
    daily
    rotate 7
    compress
    missingok
    notifempty
}
```

---

## Troubleshooting

### Daemon Won't Start

1. **Check MIDI device connection**:
   ```bash
   # macOS
   system_profiler SPUSBDataType | grep -i mikro

   # Linux
   lsusb | grep -i "native instruments"
   ```

2. **Verify binary permissions**:
   ```bash
   ls -l /usr/local/bin/midimon
   # Should be: -rwxr-xr-x
   ```

3. **Check logs**:
   ```bash
   # macOS
   tail -50 ~/Library/Logs/midimon.error.log

   # Linux
   journalctl -u midimon -n 50
   ```

4. **Test manual startup**:
   ```bash
   midimon --log-level debug
   # Look for specific error messages
   ```

### Config Not Reloading

1. **Check file watcher**:
   ```bash
   # Verify config file exists and is readable
   ls -l ~/.config/midimon/config.toml

   # Test manual reload
   midimonctl reload
   ```

2. **Validate config syntax**:
   ```bash
   midimonctl validate
   ```

3. **Check logs for reload errors**:
   ```bash
   # Look for "Config reloaded" messages
   grep "Config reloaded" ~/Library/Logs/midimon.log
   ```

### IPC Connection Errors

1. **Check socket exists**:
   ```bash
   ls -l /tmp/midimon.sock
   ```

2. **Verify daemon is running**:
   ```bash
   ps aux | grep midimon
   ```

3. **Check socket permissions**:
   ```bash
   # Should be: srwxr-xr-x (socket)
   ls -l /tmp/midimon.sock
   ```

4. **Test connectivity**:
   ```bash
   midimonctl ping
   ```

### High CPU Usage

1. **Check event flood**:
   ```bash
   # Monitor events processed
   watch -n 1 'midimonctl status | grep Events'
   ```

2. **Profile reload performance**:
   ```bash
   midimonctl status | grep -A5 "Reload Performance"
   # Grade should be A or B (under 50ms)
   ```

3. **Simplify config**:
   - Reduce number of mappings
   - Disable unused modes
   - Increase debounce timeouts

### Permission Errors (macOS)

1. **Grant Input Monitoring**:
   - System Settings → Privacy & Security → Input Monitoring
   - Add `/usr/local/bin/midimon`

2. **Reset permissions**:
   ```bash
   tccutil reset SystemPolicyAllFiles
   # Restart daemon
   ```

### Permission Errors (Linux)

1. **Check user groups**:
   ```bash
   groups $USER
   # Should include: audio, plugdev (or dialout)
   ```

2. **Add user to required groups**:
   ```bash
   sudo usermod -aG audio $USER
   sudo usermod -aG plugdev $USER
   # Log out and back in
   ```

3. **Verify udev rules**:
   ```bash
   cat /etc/udev/rules.d/99-maschine-mikro.rules
   ```

---

## Uninstallation

### macOS

```bash
# Stop and unload service
launchctl stop com.amiable.midimon
launchctl unload ~/Library/LaunchAgents/com.amiable.midimon.plist

# Remove files
rm ~/Library/LaunchAgents/com.amiable.midimon.plist
sudo rm /usr/local/bin/midimon
sudo rm /usr/local/bin/midimonctl
sudo rm /usr/local/share/man/man1/midimon*.1

# Optional: Remove data
rm -rf ~/.config/midimon
rm -rf ~/.local/state/midimon
rm -rf ~/.local/share/midimon
rm ~/Library/Logs/midimon*.log
```

### Linux

```bash
# Stop and disable service
sudo systemctl stop midimon
sudo systemctl disable midimon

# Or for user service:
systemctl --user stop midimon
systemctl --user disable midimon

# Remove files
sudo rm /etc/systemd/system/midimon.service
# Or: rm ~/.config/systemd/user/midimon.service
sudo systemctl daemon-reload

sudo rm /usr/local/bin/midimon
sudo rm /usr/local/bin/midimonctl
sudo rm /usr/local/share/man/man1/midimon*.1

# Optional: Remove data
rm -rf ~/.config/midimon
rm -rf ~/.local/state/midimon
rm -rf ~/.local/share/midimon
```

---

## Performance Characteristics

Expected performance on modern hardware:

- **Startup time**: <500ms
- **MIDI event latency**: <1ms
- **Config reload time**: 0-10ms typical (Grade A: <20ms)
- **Memory usage**: 5-10MB
- **CPU usage**: <1% idle, <5% active
- **Binary size**: ~3-5MB

### Benchmarks

Run the reload benchmark:
```bash
cargo bench --package midimon-daemon
```

Results on Apple M1:
- 2 modes, 10 mappings: 0-2ms
- 5 modes, 50 mappings: 2-5ms
- 10 modes, 100 mappings: 5-8ms

All configs achieve **Grade A** performance (<20ms).

---

## Support

- **Documentation**: https://github.com/amiable/midimon
- **Issues**: https://github.com/amiable/midimon/issues
- **Man Pages**: `man midimon`, `man midimonctl`

---

## License

Copyright 2025 Amiable (Christopher Joseph)

Licensed under the MIT License. See LICENSE file for details.
