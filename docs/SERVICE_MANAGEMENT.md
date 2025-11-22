# MIDIMon Service Management

Comprehensive guide to installing, managing, and troubleshooting MIDIMon as a system service.

## Overview

MIDIMon can run as a LaunchAgent service on macOS, allowing it to:
- Start automatically on login
- Run in the background
- Persist across reboots
- Be managed via `midimonctl` commands

## Quick Start

```bash
# Install service (uses binary from target/release/midimon)
midimonctl install

# Install service and copy binary to /usr/local/bin (requires sudo for /usr/local/bin)
sudo midimonctl install --install-binary

# Check service status
midimonctl service-status

# Start the service
midimonctl start

# Stop the service
midimonctl stop

# Restart the service
midimonctl restart

# Uninstall service
midimonctl uninstall
```

## Service Management Commands

### Installation

#### `midimonctl install`

Installs MIDIMon as a LaunchAgent service.

**Options:**
- `--install-binary` - Also copy daemon binary to `/usr/local/bin` (requires sudo)
- `--force` - Reinstall even if already installed

**What it does:**
1. Locates the daemon binary (in `target/release/midimon` or `/usr/local/bin/midimon`)
2. Creates `~/Library/LaunchAgents/com.amiable.midimon.plist`
3. Replaces `USERNAME` placeholder with actual username
4. Sets correct binary path in plist
5. Creates log directory at `~/Library/Logs`
6. Loads and enables the service via `launchctl`

**Examples:**

```bash
# Install using binary from build directory
midimonctl install

# Install and copy binary to system location
sudo midimonctl install --install-binary

# Force reinstall
midimonctl install --force

# JSON output
midimonctl install --json
```

**Output:**

```
Installing MIDIMon service...
  ✓ Created log directory: /Users/you/Library/Logs
  ✓ Installed service plist: /Users/you/Library/LaunchAgents/com.amiable.midimon.plist

✓ MIDIMon service installed successfully
  Binary:  target/release/midimon
  Plist:   /Users/you/Library/LaunchAgents/com.amiable.midimon.plist
  Status:  Enabled (will start on login)

Use 'midimonctl start' to start the service now.
```

---

### Uninstallation

#### `midimonctl uninstall`

Removes MIDIMon service.

**Options:**
- `--remove-binary` - Also remove daemon binary from `/usr/local/bin`
- `--remove-logs` - Remove log files

**What it does:**
1. Stops the service via `launchctl unload`
2. Removes plist file
3. Optionally removes binary and logs

**Examples:**

```bash
# Uninstall service (keeps binary and logs)
midimonctl uninstall

# Uninstall and remove binary
midimonctl uninstall --remove-binary

# Uninstall and remove logs
midimonctl uninstall --remove-logs

# Remove everything
midimonctl uninstall --remove-binary --remove-logs
```

---

### Service Control

#### `midimonctl start`

Starts the daemon service.

**Options:**
- `--wait <seconds>` - Wait for daemon to be ready (default: 5 seconds)

**What it does:**
1. Checks if daemon is already running (via IPC ping)
2. Loads the service via `launchctl load`
3. Polls IPC endpoint to verify daemon is ready
4. Reports success or timeout

**Examples:**

```bash
# Start and wait up to 5 seconds for readiness
midimonctl start

# Start and wait up to 10 seconds
midimonctl start --wait 10

# Start without waiting
midimonctl start --wait 0
```

**Output:**

```
Starting MIDIMon service...
Waiting for daemon to be ready.. ✓
✓ Service started successfully
```

---

#### `midimonctl stop`

Stops the daemon service.

**Options:**
- `--force` - Force stop without graceful shutdown

**What it does:**
1. Attempts graceful shutdown via IPC (unless `--force`)
2. Unloads the service via `launchctl unload`
3. Verifies daemon is stopped

**Examples:**

```bash
# Graceful stop (sends IPC shutdown, then unloads)
midimonctl stop

# Force stop (immediately unloads)
midimonctl stop --force
```

---

#### `midimonctl restart`

Restarts the daemon service.

**Options:**
- `--wait <seconds>` - Wait for daemon to be ready (default: 5 seconds)

**What it does:**
1. Stops the service
2. Waits 500ms
3. Starts the service
4. Waits for daemon to be ready

**Examples:**

```bash
# Restart and wait for readiness
midimonctl restart

# Restart with custom wait timeout
midimonctl restart --wait 10
```

---

### Auto-Start Management

#### `midimonctl enable`

Enables auto-start on login.

**What it does:**
- Loads the service with `-w` flag, which enables auto-start

**Example:**

```bash
midimonctl enable
# Output: ✓ Service enabled (will start on login)
```

---

#### `midimonctl disable`

Disables auto-start on login.

**What it does:**
- Unloads the service with `-w` flag, which disables auto-start

**Example:**

```bash
midimonctl disable
# Output: ✓ Service disabled (will not start on login)
```

---

### Status Checking

#### `midimonctl service-status`

Shows comprehensive service installation and status.

**Information shown:**
- Installation status (Installed/Not Installed)
- Service label
- Plist path and existence
- Binary path and existence
- Loaded status (enabled/disabled)

**Example:**

```bash
midimonctl service-status
```

**Output:**

```
MIDIMon Service Status
──────────────────────────────────────────────────
Status:          Installed and Loaded
Service Label:   com.amiable.midimon
Plist:           /Users/you/Library/LaunchAgents/com.amiable.midimon.plist ✓
Binary:          /usr/local/bin/midimon ✗

Service is loaded (enabled)
```

**JSON output:**

```bash
midimonctl service-status --json
```

```json
{
  "installed": true,
  "plist_path": "/Users/you/Library/LaunchAgents/com.amiable.midimon.plist",
  "binary_exists": false,
  "binary_path": "/usr/local/bin/midimon",
  "loaded": true,
  "service_label": "com.amiable.midimon"
}
```

---

## Service Architecture

### LaunchAgent Plist

The generated plist file (`~/Library/LaunchAgents/com.amiable.midimon.plist`) configures:

**Service Identity:**
- Label: `com.amiable.midimon`
- Service type: User LaunchAgent

**Execution:**
- Program: Path to `midimon` binary
- Working directory: `/usr/local`

**Auto-Start Behavior:**
- `RunAtLoad`: true (start on load)
- `KeepAlive`: Restart on crash, not on clean exit
- `ThrottleInterval`: 5 seconds between restart attempts

**Process Configuration:**
- Priority: Nice value -5 (higher priority)
- Session type: Aqua (GUI session)
- Process type: Interactive

**Logging:**
- Stdout: `~/Library/Logs/midimon.log`
- Stderr: `~/Library/Logs/midimon.error.log`

**Environment:**
- `RUST_LOG=midimon=info`

**Resource Limits:**
- File descriptors: 512 (soft), 1024 (hard)

---

## Common Workflows

### First-Time Setup

```bash
# 1. Build the daemon
cargo build --release --bin midimon

# 2. Install service (uses target/release/midimon)
midimonctl install

# 3. Start the service
midimonctl start

# 4. Verify it's running
midimonctl status
```

---

### Production Deployment

```bash
# 1. Build release binary
cargo build --release --bin midimon

# 2. Install to system location (requires sudo)
sudo midimonctl install --install-binary

# 3. Start service
midimonctl start

# 4. Verify installation
midimonctl service-status
```

---

### Development Workflow

```bash
# After code changes:
cargo build --release --bin midimon
midimonctl restart
midimonctl status
```

---

### Viewing Logs

```bash
# Tail daemon logs
tail -f ~/Library/Logs/midimon.log

# View errors
tail -f ~/Library/Logs/midimon.error.log

# View all logs
tail -f ~/Library/Logs/midimon*.log
```

---

### Checking Service Health

```bash
# Quick check
midimonctl ping

# Detailed status
midimonctl status

# Service installation status
midimonctl service-status

# macOS launchctl status
launchctl list com.amiable.midimon
```

---

## Troubleshooting

### Service Won't Start

**Symptoms:**
- `midimonctl start` times out
- `midimonctl ping` fails

**Diagnosis:**

```bash
# Check if service is loaded
launchctl list | grep midimon

# Check logs for errors
tail -50 ~/Library/Logs/midimon.error.log

# Verify binary exists and is executable
ls -la target/release/midimon
file target/release/midimon
```

**Common causes:**
1. Binary not found - Rebuild with `cargo build --release --bin midimon`
2. Binary path incorrect in plist - Check plist, reinstall with `--force`
3. Permissions issue - Check binary is executable (`chmod +x`)
4. Config file error - Run `midimonctl validate`

---

### Service Crashes on Startup

**Diagnosis:**

```bash
# View crash logs
tail -100 ~/Library/Logs/midimon.error.log

# Check launchctl errors
launchctl error com.amiable.midimon

# Try running binary directly for detailed error
target/release/midimon
```

**Common causes:**
1. Config file syntax error - Validate config
2. MIDI device not found - Check available devices
3. Missing dependencies - Reinstall with Homebrew
4. Permission denied - Grant Input Monitoring permissions

---

### Service Won't Stop

**Diagnosis:**

```bash
# Force stop
midimonctl stop --force

# Check if process is running
ps aux | grep midimon

# Kill manually if needed
killall midimon
```

---

### Service Not Auto-Starting on Login

**Diagnosis:**

```bash
# Check if enabled
launchctl list com.amiable.midimon

# Check plist exists
ls -la ~/Library/LaunchAgents/com.amiable.midimon.plist

# Check RunAtLoad setting
plutil -p ~/Library/LaunchAgents/com.amiable.midimon.plist | grep RunAtLoad
```

**Fix:**

```bash
# Re-enable
midimonctl enable

# Or reinstall
midimonctl uninstall
midimonctl install
```

---

### Plist File Is Corrupt

**Symptoms:**
- `launchctl load` fails with XML parsing error

**Fix:**

```bash
# Validate plist XML
plutil ~/Library/LaunchAgents/com.amiable.midimon.plist

# Reinstall service
midimonctl uninstall
midimonctl install --force
```

---

### Binary Location Issues

**Problem:** Plist points to wrong binary location

**Diagnosis:**

```bash
# Check plist binary path
plutil -p ~/Library/LaunchAgents/com.amiable.midimon.plist | grep ProgramArguments -A 2
```

**Fix:**

```bash
# Reinstall with --force
midimonctl install --force

# Or install to /usr/local/bin
sudo midimonctl install --install-binary --force
```

---

## Platform Support

### macOS

**Full support** via LaunchAgent plists and `launchctl`.

**Requirements:**
- macOS 10.10+ (Yosemite or later)
- `launchctl` command-line tool

**Files:**
- Plist: `~/Library/LaunchAgents/com.amiable.midimon.plist`
- Logs: `~/Library/Logs/midimon.log`, `~/Library/Logs/midimon.error.log`
- Binary: `target/release/midimon` or `/usr/local/bin/midimon`

---

### Linux

**Future support** planned via systemd user services.

**Planned files:**
- Service: `~/.config/systemd/user/midimon.service`
- Logs: journalctl or `~/.local/share/midimon/logs/`
- Binary: `/usr/local/bin/midimon`

**Commands (future):**
```bash
midimonctl install   # Creates systemd service
midimonctl start     # systemctl --user start midimon
midimonctl stop      # systemctl --user stop midimon
midimonctl enable    # systemctl --user enable midimon
```

---

### Windows

**Future support** planned via Windows Services or scheduled tasks.

---

## Security Considerations

### File Permissions

**Plist file:**
- Location: User's `~/Library/LaunchAgents/`
- Permissions: User-readable, user-writable (managed by macOS)
- No sensitive data stored

**Binary:**
- Location (installed): `/usr/local/bin/midimon`
- Permissions: `rwxr-xr-x` (0755) - world-executable
- Installation requires `sudo` for system location

**Log files:**
- Location: `~/Library/Logs/`
- Permissions: User-readable, user-writable
- May contain configuration paths and MIDI device names

---

### IPC Socket

The daemon creates an IPC socket for `midimonctl` communication:

**Location:** `~/Library/Application Support/midimon/ipc.sock` (or `/tmp/midimon-{uid}/ipc.sock`)

**Permissions:** `0700` directory, user-only access

**Security features:**
- User-isolated directory (UID check)
- Unix domain socket (local only, no network exposure)
- Permission validation on directory creation

---

## Advanced Configuration

### Custom Binary Path

Edit plist manually to use custom binary location:

```bash
# Edit plist
nano ~/Library/LaunchAgents/com.amiable.midimon.plist

# Change ProgramArguments
<key>ProgramArguments</key>
<array>
    <string>/path/to/custom/midimon</string>
</array>

# Reload service
midimonctl restart
```

---

### Custom Log Location

Edit plist to change log paths:

```xml
<key>StandardOutPath</key>
<string>/custom/path/midimon.log</string>

<key>StandardErrorPath</key>
<string>/custom/path/midimon.error.log</string>
```

---

### Environment Variables

Add custom environment variables to plist:

```xml
<key>EnvironmentVariables</key>
<dict>
    <key>RUST_LOG</key>
    <string>midimon=debug</string>
    <key>CUSTOM_VAR</key>
    <string>value</string>
</dict>
```

---

### Disable Auto-Restart

To prevent service from restarting on crash:

```xml
<key>KeepAlive</key>
<false/>
```

---

## API Reference

### Exit Codes

All `midimonctl` service commands use standard exit codes:

- `0` - Success
- `1` - Error (see stderr for details)

### JSON Output Format

All commands support `--json` flag for machine-readable output.

**Success response:**

```json
{
  "status": "success",
  "field1": "value1",
  "field2": "value2"
}
```

**Error response:**

```json
{
  "status": "error",
  "error": "Error message here"
}
```

---

## Integration Examples

### Shell Script

```bash
#!/bin/bash
set -e

# Install and start MIDIMon
if ! midimonctl service-status --json | jq -e '.installed' > /dev/null; then
    echo "Installing MIDIMon service..."
    midimonctl install
fi

# Ensure service is running
if ! midimonctl ping &> /dev/null; then
    echo "Starting MIDIMon..."
    midimonctl start --wait 10
fi

echo "MIDIMon is ready"
```

---

### Ansible Playbook

```yaml
- name: Install MIDIMon service
  command: midimonctl install --install-binary
  become: yes
  args:
    creates: /Users/{{ ansible_user }}/Library/LaunchAgents/com.amiable.midimon.plist

- name: Start MIDIMon service
  command: midimonctl start
  become_user: "{{ ansible_user }}"
```

---

### GitHub Actions

```yaml
- name: Test MIDIMon service management
  run: |
    cargo build --release --bin midimon
    ./target/release/midimonctl install
    ./target/release/midimonctl service-status --json
    ./target/release/midimonctl uninstall
```

---

## Best Practices

### Development

1. **Use build directory binary** during development:
   ```bash
   midimonctl install  # Points to target/release/midimon
   ```

2. **Restart after code changes**:
   ```bash
   cargo build --release --bin midimon && midimonctl restart
   ```

3. **Monitor logs** during development:
   ```bash
   tail -f ~/Library/Logs/midimon*.log
   ```

---

### Production

1. **Install to system location**:
   ```bash
   sudo midimonctl install --install-binary
   ```

2. **Verify installation**:
   ```bash
   midimonctl service-status
   midimonctl ping
   ```

3. **Enable auto-start**:
   ```bash
   midimonctl enable
   ```

4. **Set up log rotation** (macOS handles this automatically for `~/Library/Logs`)

---

### Maintenance

1. **Check service health** regularly:
   ```bash
   midimonctl status
   ```

2. **Review logs** for errors:
   ```bash
   grep ERROR ~/Library/Logs/midimon.log
   ```

3. **Update service** after binary changes:
   ```bash
   cargo build --release --bin midimon
   midimonctl restart
   ```

4. **Backup configuration** before major changes:
   ```bash
   cp ~/.config/midimon/config.toml ~/.config/midimon/config.toml.backup
   ```

---

## FAQ

### Q: Do I need sudo for service management?

**A:** Only if you use `--install-binary` to copy the binary to `/usr/local/bin`. The plist installation to `~/Library/LaunchAgents/` does not require sudo.

---

### Q: Can I run multiple instances?

**A:** No, the service is designed to run as a singleton. Multiple instances would conflict over IPC socket and MIDI device access.

---

### Q: What happens if the service crashes?

**A:** The LaunchAgent will automatically restart the service after 5 seconds (configured via `ThrottleInterval`).

---

### Q: How do I upgrade the daemon?

```bash
# 1. Build new version
cargo build --release --bin midimon

# 2. Restart service (picks up new binary)
midimonctl restart
```

If you installed to `/usr/local/bin`:

```bash
# Reinstall binary
sudo midimonctl install --install-binary --force
midimonctl restart
```

---

### Q: How do I change the config while service is running?

Edit `~/.config/midimon/config.toml`, then:

```bash
midimonctl reload
```

The daemon watches the config file and auto-reloads (with 500ms debounce).

---

### Q: Where are the logs?

**Daemon logs:**
- `~/Library/Logs/midimon.log` (stdout)
- `~/Library/Logs/midimon.error.log` (stderr)

**View with:**

```bash
tail -f ~/Library/Logs/midimon.log
```

---

### Q: How do I uninstall everything?

```bash
midimonctl uninstall --remove-binary --remove-logs
```

This removes:
- Service plist
- Daemon binary (if `--remove-binary`)
- Log files (if `--remove-logs`)

**Manual cleanup** (if needed):

```bash
rm -rf ~/.config/midimon/
rm -rf ~/Library/Application\ Support/midimon/
```

---

## Comparison with Other Tools

### vs systemd (Linux)

| Feature | macOS LaunchAgent | systemd |
|---------|-------------------|---------|
| User service | ✓ | ✓ |
| Auto-start | ✓ | ✓ |
| Logging | File-based | journalctl |
| Restart on crash | ✓ | ✓ |
| GUI session | ✓ (Aqua) | ✓ (Wayland/X11) |

---

### vs Docker

MIDIMon **cannot** run in Docker because:
- Requires direct MIDI device access
- Needs HID device permissions
- Must run in user GUI session for macOS permissions

---

## Related Documentation

- [IPC Protocol](IPC_PROTOCOL.md) - Communication between midimonctl and daemon
- [Configuration](../CLAUDE.md#configuration-configtoml) - Config file format
- [Troubleshooting](TROUBLESHOOTING.md) - General troubleshooting guide

---

## Version History

**v2.0.0** (2025-01-16)
- Initial service management implementation
- macOS LaunchAgent support
- Commands: install, uninstall, start, stop, restart, enable, disable, service-status

---

## Contributing

To add support for additional platforms:

1. Implement service management in `midimonctl.rs`:
   ```rust
   #[cfg(target_os = "linux")]
   fn handle_install() { /* systemd implementation */ }
   ```

2. Add platform-specific service templates
3. Update this documentation
4. Add tests for new platform

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.
