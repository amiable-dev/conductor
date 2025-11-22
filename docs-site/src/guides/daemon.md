# Daemon & Hot-Reload Guide

Conductor v2.0.0 introduces a production-ready background daemon service with lightning-fast configuration hot-reloading.

## Architecture Overview

The Conductor daemon runs as a background service, providing:

- **Hot-Reload**: Configuration changes applied in 0-10ms without restart
- **IPC Control**: Control via CLI (`conductorctl`) or GUI
- **State Persistence**: Automatic state saving with atomic writes
- **Crash Recovery**: Auto-restart with KeepAlive (when using LaunchAgent)
- **Menu Bar Integration**: macOS menu bar for quick actions

## Starting the Daemon

### Via GUI (Recommended)

Open the Conductor GUI app - the daemon starts automatically in the background.

### Via CLI

```bash
# Start daemon in foreground
conductor

# Start daemon in background
conductor &

# Check if daemon is running
ps aux | grep conductor | grep -v grep

# Check daemon status via IPC
conductorctl status
```

### Via LaunchAgent (Auto-Start)

See [macOS Installation Guide](../installation/macos.md#auto-start-on-login) for LaunchAgent setup.

## Controlling the Daemon

### conductorctl CLI

The `conductorctl` command provides full control over the daemon:

#### Status

```bash
# Human-readable status
conductorctl status

# Example output:
# Conductor Daemon Status:
#   State: Running
#   Uptime: 1h 23m 45s
#   Config: /Users/you/.config/conductor/config.toml
#   Last Reload: 2m 15s ago
#   IPC Socket: /tmp/conductor.sock
#   PID: 12345
```

**JSON output** (for scripting):

```bash
conductorctl status --json

# Example output:
# {
#   "state": "Running",
#   "uptime_seconds": 5025,
#   "config_path": "/Users/you/.config/conductor/config.toml",
#   "last_reload_seconds": 135,
#   "ipc_socket": "/tmp/conductor.sock",
#   "pid": 12345
# }
```

#### Reload Configuration

```bash
# Reload config (hot-reload in 0-10ms)
conductorctl reload

# Example output:
# Config reloaded successfully in 3ms
# Loaded 15 mappings across 3 modes
```

This applies configuration changes **without restarting** the daemon or interrupting active MIDI connections.

**Performance**: Config reload completes in **0-10ms** on average (vs. ~500ms for full restart).

#### Validate Configuration

```bash
# Validate config without reloading
conductorctl validate

# Example output (success):
# ✓ Config is valid
# Found 3 modes with 15 total mappings
# Found 2 global mappings
# No errors detected

# Example output (error):
# ✗ Config validation failed:
# Error in mode 'Default', mapping 3:
#   Invalid note number: 128 (must be 0-127)
```

This is useful for:
- Testing config changes before applying
- CI/CD validation
- Pre-commit hooks

#### Stop Daemon

```bash
# Graceful shutdown
conductorctl stop

# Example output:
# Daemon stopped gracefully
```

The daemon saves its state before shutting down.

#### Ping (Latency Check)

```bash
# Check IPC round-trip latency
conductorctl ping

# Example output:
# Pong! Round-trip: 0.43ms
```

This verifies the daemon is responsive and measures IPC performance.

## Configuration Hot-Reload

### How It Works

1. **File Watcher**: Monitors `~/.config/conductor/config.toml` for changes
2. **Debouncing**: 500ms debounce window to avoid redundant reloads
3. **Parsing**: Validates TOML syntax and config structure
4. **Atomic Swap**: Replaces active config with new one in a single operation
5. **Reload Time**: **0-10ms** typical (measured)

### Workflow

Edit your config file:

```bash
# Open in your editor
code ~/.config/conductor/config.toml

# Make changes and save
```

**Automatic reload** happens within 500-510ms of saving:

```
[2025-11-14 10:30:15] Config file changed
[2025-11-14 10:30:15] Waiting 500ms for debounce...
[2025-11-14 10:30:15] Reloading config...
[2025-11-14 10:30:15] Config reloaded successfully in 3ms
```

Or **manually trigger** reload:

```bash
conductorctl reload
```

### What Gets Reloaded

- ✅ All mappings (modes + global)
- ✅ Device settings
- ✅ LED schemes
- ✅ Advanced settings (timeouts, thresholds)
- ❌ IPC socket path (requires daemon restart)
- ❌ Auto-start settings (requires LaunchAgent reload)

### Error Handling

If config reload fails:

```bash
# Example: Invalid TOML syntax
conductorctl reload

# Output:
# ✗ Config reload failed: TOML parse error
# Error at line 42: expected '=' after key 'trigger'
# Previous config still active (no changes applied)
```

The daemon **keeps the previous valid config** and continues running.

## State Persistence

The daemon automatically saves state to `~/.config/conductor/state.json`:

### What Gets Saved

- Current mode
- Active LED scheme
- Per-app profile assignments
- Last known device connection

### Atomic Writes

State is saved using atomic writes to prevent corruption:

1. Write to temporary file: `state.json.tmp`
2. Verify write succeeded
3. Rename to `state.json` (atomic operation)
4. SHA256 checksum for integrity

### Emergency Save

The daemon registers signal handlers to save state on:
- `SIGTERM` (graceful shutdown)
- `SIGINT` (Ctrl+C)
- `SIGHUP` (hangup)

## IPC Protocol

The daemon uses **Unix domain sockets** for inter-process communication.

### Socket Location

Default: `/tmp/conductor.sock`

### Protocol

**Format**: JSON messages over Unix socket

**Request**:
```json
{
  "command": "reload",
  "args": {}
}
```

**Response**:
```json
{
  "status": "success",
  "message": "Config reloaded successfully",
  "duration_ms": 3
}
```

### Available Commands

- `status` - Get daemon status
- `reload` - Reload configuration
- `validate` - Validate config without reloading
- `stop` - Graceful shutdown
- `ping` - Latency check

### Security Limits

The IPC protocol enforces security limits to prevent abuse:

**Request Size Limit**: 1MB (1,048,576 bytes)

Requests exceeding this limit will be rejected with error code `1004` (InvalidRequest):

```json
{
  "status": "error",
  "error": {
    "code": 1004,
    "message": "Request too large: 1500000 bytes exceeds maximum of 1048576 bytes (1MB)",
    "details": {
      "request_size": 1500000,
      "max_size": 1048576,
      "security": "Request rejected to prevent memory exhaustion"
    }
  }
}
```

**Why this limit exists**: Prevents memory exhaustion denial-of-service attacks where an attacker sends arbitrarily large requests to consume daemon memory.

**Is 1MB enough?**: Yes. Typical IPC requests are:
- `status`: ~200 bytes
- `reload`: ~100 bytes
- `validate`: ~150 bytes
- `ping`: ~50 bytes

Even large configuration payloads are well under 100KB. The 1MB limit provides 10x safety margin.

## Performance Metrics

The daemon tracks and reports performance metrics:

### Config Reload Latency

```bash
conductorctl status --json | jq '.metrics.reload_latency_ms'
# Output: 3.2
```

**Targets**:
- ✅ <10ms: Excellent (target met in v2.0.0)
- ⚠️ 10-50ms: Acceptable
- ❌ >50ms: Investigate

### IPC Round-Trip

```bash
conductorctl ping
# Output: Pong! Round-trip: 0.43ms
```

**Targets**:
- ✅ <1ms: Excellent (target met in v2.0.0)
- ⚠️ 1-5ms: Acceptable
- ❌ >5ms: Investigate

### Memory Usage

```bash
ps aux | grep conductor | awk '{print $6/1024 " MB"}'
# Output: 8.2 MB
```

**Targets**:
- ✅ 5-10MB: Normal (daemon only)
- ⚠️ 10-20MB: Acceptable
- ❌ >20MB: Investigate memory leak

### CPU Usage

```bash
top -pid $(pgrep conductor) -stats cpu -l 1 | tail -1
# Output: 0.3%
```

**Targets**:
- ✅ <1%: Idle (no MIDI activity)
- ✅ <5%: Active (processing MIDI events)
- ⚠️ 5-10%: Heavy load
- ❌ >10%: Investigate

## Menu Bar Integration (macOS)

The daemon includes an optional menu bar icon:

### Features

- **Status indicator**: Running (green), Stopped (gray), Error (red)
- **Quick actions**:
  - Pause/Resume
  - Reload Config
  - Open GUI
  - Quit

### Enable Menu Bar

In GUI Settings:
1. Go to **Settings** → **General**
2. Enable **"Show menu bar icon"**
3. Click **Save**

Or edit config:
```toml
[settings]
show_menu_bar = true
```

## Troubleshooting

### Daemon Won't Start

**Check if already running**:
```bash
ps aux | grep conductor
```

If already running, stop it first:
```bash
conductorctl stop
```

**Check logs**:
```bash
# If using LaunchAgent
tail -f /tmp/conductor.err

# If running manually
conductor  # Run in foreground to see errors
```

### Config Won't Reload

**Validate config syntax**:
```bash
conductorctl validate
```

**Check file watcher**:
```bash
# Manually trigger reload
conductorctl reload
```

**Check file permissions**:
```bash
ls -l ~/.config/conductor/config.toml
# Should be readable by your user
```

### High Latency

**Check IPC performance**:
```bash
conductorctl ping
```

**Check system load**:
```bash
top -l 1 | head -10
```

**Restart daemon**:
```bash
conductorctl stop
conductor &
```

### State Not Persisting

**Check state file**:
```bash
ls -l ~/.config/conductor/state.json
cat ~/.config/conductor/state.json | jq
```

**Check file permissions**:
```bash
# State directory should be writable
ls -ld ~/.config/conductor
```

## Advanced Configuration

### Custom IPC Socket Path

Edit daemon config (future feature):
```toml
[daemon]
ipc_socket = "/tmp/my-custom-conductor.sock"
```

Then tell `conductorctl` to use it:
```bash
export MIDIMON_SOCKET=/tmp/my-custom-conductor.sock
conductorctl status
```

### Custom Config Path

Run daemon with custom config:
```bash
conductor --config /path/to/config.toml
```

Or set environment variable:
```bash
export MIDIMON_CONFIG=/path/to/config.toml
conductor
```

### Logging

Enable debug logging:
```bash
DEBUG=1 conductor
```

Output includes:
- Config reload events
- IPC requests/responses
- State persistence operations
- MIDI event processing (verbose)

## Next Steps

- [Per-App Profiles](./per-app-profiles.md) - Automatic profile switching
- [CLI Reference](../reference/cli.md) - Complete CLI documentation
- [Configuration Overview](../configuration/overview.md) - Config file reference
- [Performance Tuning](../advanced/performance.md) - Optimization guide

---

**Last Updated**: November 14, 2025 (v2.0.0)
