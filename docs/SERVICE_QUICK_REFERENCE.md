# MIDIMon Service Management - Quick Reference

## Installation

```bash
# Install service (uses binary from target/release/midimon)
midimonctl install

# Install + copy binary to /usr/local/bin (requires sudo)
sudo midimonctl install --install-binary

# Force reinstall
midimonctl install --force
```

## Service Control

```bash
# Start daemon
midimonctl start

# Stop daemon (graceful shutdown)
midimonctl stop

# Force stop (no graceful shutdown)
midimonctl stop --force

# Restart daemon
midimonctl restart
```

## Auto-Start

```bash
# Enable auto-start on login
midimonctl enable

# Disable auto-start
midimonctl disable
```

## Status

```bash
# Check service installation status
midimonctl service-status

# Check daemon runtime status
midimonctl status

# Quick health check
midimonctl ping

# JSON output
midimonctl service-status --json
```

## Uninstallation

```bash
# Uninstall service (keeps binary and logs)
midimonctl uninstall

# Uninstall + remove binary
midimonctl uninstall --remove-binary

# Uninstall + remove logs
midimonctl uninstall --remove-logs

# Remove everything
midimonctl uninstall --remove-binary --remove-logs
```

## Logs

```bash
# View logs
tail -f ~/Library/Logs/midimon.log
tail -f ~/Library/Logs/midimon.error.log

# View all logs
tail -f ~/Library/Logs/midimon*.log
```

## Common Workflows

### First-Time Setup

```bash
cargo build --release --bin midimon
midimonctl install
midimonctl start
midimonctl status
```

### Development

```bash
# After code changes
cargo build --release --bin midimon
midimonctl restart
```

### Production Deployment

```bash
cargo build --release --bin midimon
sudo midimonctl install --install-binary
midimonctl start
midimonctl enable
```

## Files

```
~/Library/LaunchAgents/com.amiable.midimon.plist  # Service configuration
~/Library/Logs/midimon.log                         # Stdout logs
~/Library/Logs/midimon.error.log                   # Stderr logs
/usr/local/bin/midimon                             # Binary (if installed)
target/release/midimon                             # Binary (development)
```

## Troubleshooting

```bash
# Check if service is loaded
launchctl list com.amiable.midimon

# View recent errors
tail -50 ~/Library/Logs/midimon.error.log

# Check binary exists
ls -la target/release/midimon

# Validate config
midimonctl validate

# Force restart
midimonctl stop --force && midimonctl start

# Reinstall
midimonctl uninstall && midimonctl install --force
```

## Platform Support

- ✅ **macOS** - Full support via LaunchAgent
- 🚧 **Linux** - Planned (systemd user services)
- 🚧 **Windows** - Planned (Windows Services)

## See Also

- [Full Documentation](SERVICE_MANAGEMENT.md)
- [IPC Protocol](IPC_PROTOCOL.md)
- [Configuration Guide](../CLAUDE.md#configuration-configtoml)
