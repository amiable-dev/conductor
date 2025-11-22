# MIDIMon v2.0.0 - macOS Testing Guide

**Last Updated**: 2025-11-15
**Platform**: macOS 10.15+
**Architecture**: Intel (x86_64) and Apple Silicon (arm64)

---

## Quick Start Testing (5 minutes)

### Prerequisites Check

```bash
# Navigate to project
cd /Users/christopherjoseph/projects/amiable/midimon

# Check Rust is installed
rustc --version  # Should show 1.70+

# Check Node.js is installed (for GUI)
node --version   # Should show 18+
npm --version

# Check if you have a MIDI device connected
ls /dev/cu.* | grep -i midi
```

### Build Everything

```bash
# Build the entire workspace (core + daemon + GUI)
cargo build --release --workspace

# This builds:
# - midimon-core (engine library)
# - midimon-daemon (background service + CLI tools)
# - midimon-gui (Tauri app)
```

**Expected output**:
```
   Compiling midimon-core v2.0.0
   Compiling midimon-daemon v2.0.0
   Compiling midimon-gui v2.0.0
    Finished release [optimized] target(s) in 45.23s
```

**Build times**:
- Clean build: ~45-60 seconds
- Incremental: ~5-10 seconds

---

## Testing Path 1: CLI/Daemon Testing (No MIDI Device Required)

### 1. Test the Daemon

```bash
# Start the daemon
./target/release/midimon

# Expected output:
# MIDIMon v2.0.0 starting...
# Available MIDI ports:
# 0: IAC Driver Bus 1
# 1: ...
#
# Select port number (or 'q' to quit):
```

**If you don't have a MIDI device**:
- macOS has a built-in virtual MIDI device: "IAC Driver"
- Enable it: Audio MIDI Setup.app → Window → Show MIDI Studio → IAC Driver → Device is online ✓

### 2. Test Daemon Commands

Open a **second terminal** while daemon is running:

```bash
# Check daemon status
./target/release/midimonctl status

# Expected output:
# Daemon Status:
# - Lifecycle: Running
# - Uptime: 5s
# - Events processed: 0
# - Device: Not connected

# Test config reload (hot-reload)
./target/release/midimonctl reload

# Expected output:
# ✓ Configuration reloaded successfully
# Reload time: 3ms

# Test ping (latency check)
./target/release/midimonctl ping

# Expected output:
# ✓ Daemon responded in 1ms

# Validate config
./target/release/midimonctl validate

# Expected output:
# ✓ Configuration is valid
```

### 3. Test Config Hot-Reload

```bash
# 1. Make a small change to config
echo "# Test comment" >> ~/.config/midimon/config.toml

# 2. Reload daemon
./target/release/midimonctl reload

# 3. Check it reloaded in <10ms
# ✓ Configuration reloaded successfully
# Reload time: 3ms
```

---

## Testing Path 2: GUI Testing (Visual Configuration)

### 1. Build the GUI

```bash
cd midimon-gui

# Install frontend dependencies
cd ui
npm install

# Build the frontend
npm run build

# Go back to GUI root
cd ..

# Build the Tauri app
cargo tauri build --debug  # Debug build for testing
```

**Expected output**:
```
    Finished 2 bundles at:
        /Users/.../midimon-gui/src-tauri/target/release/bundle/dmg/MIDIMon_2.0.0_x64.dmg
        /Users/.../midimon-gui/src-tauri/target/release/bundle/macos/MIDIMon.app
```

### 2. Run the GUI in Development Mode

```bash
# From midimon-gui directory
cargo tauri dev
```

**Expected behavior**:
1. Vite dev server starts (http://localhost:5173)
2. Tauri window opens with MIDIMon UI
3. Menu bar icon appears in system tray

**What to test**:

#### ✅ Menu Bar (System Tray)
- Click menu bar icon → Menu appears
- Check "Status: Checking..." updates to "Status: Running" or "Status: Stopped"
- Try "Reload Configuration" → Should reload daemon
- Try "Show MIDIMon" → Window focuses

#### ✅ Devices Tab
1. Click "Devices" in sidebar
2. Should see list of MIDI devices
3. Try "📋 Device Templates" button → Template selector opens
4. Select "Maschine Mikro MK3" or "Generic 25-Key MIDI Keyboard"
5. Click "Apply Template" → Config created

#### ✅ Modes Tab
1. Click "Modes" in sidebar
2. Should see "Default" mode
3. Click "+ Add Mode" → Create "Test Mode"
4. Set color to "green"
5. Click "Save" → Mode appears in list

#### ✅ Mappings Tab
1. Click "Mappings" in sidebar
2. Click "+ Add Mapping"
3. Click "🎹 MIDI Learn" button
4. **If you have a MIDI device**:
   - Press a pad/key → Trigger auto-detected
   - Click "Use This" → Trigger filled in
5. **If no MIDI device**:
   - Skip MIDI Learn
   - Manually select trigger type: "Note"
   - Set note number: 60 (Middle C)
6. Configure action:
   - Type: "Keystroke"
   - Modifiers: [Cmd]
   - Keys: "S"
7. Click "Save Mapping"

#### ✅ Settings Tab
1. Click "Settings" in sidebar
2. Check config file path displays
3. Click "📋" → Path copied to clipboard
4. Click "📊 Show Event Console"
5. **If MIDI device connected**: Press pads → Events appear
6. **If no device**: Events will be empty

### 3. Test Menu Bar Features

With GUI running:

1. **Status Updates**:
   - Menu bar icon shows status
   - Auto-updates every 2 seconds
   - Shows: Running/Stopped/Error/Paused

2. **Quick Actions**:
   - Click "Reload Configuration" → Daemon reloads
   - Click "Pause Processing" → Event processing stops
   - Click "Resume Processing" → Event processing resumes

3. **Mode Switching**:
   - Hover "Switch Mode" → Submenu appears
   - Click "Development" → Mode switches
   - Check daemon reloads

### 4. Test Without MIDI Device (Synthetic Testing)

```bash
# Use the MIDI diagnostic tool to send virtual events
./target/release/midi_diagnostic 0  # Use IAC Driver or port 0

# This will show you ALL MIDI events on that port
# You can use macOS's Audio MIDI Setup to send test notes
```

**To send test MIDI from macOS**:
1. Open **Audio MIDI Setup.app**
2. Window → Show MIDI Studio
3. Double-click "IAC Driver"
4. Check "Device is online"
5. Double-click IAC Driver in MIDI Studio
6. Click "Test" → Sends test notes

---

## Testing Path 3: MIDI Learn Testing (Requires MIDI Device)

### Prerequisites

You need **one of**:
- MIDI keyboard/controller (USB)
- Native Instruments Maschine Mikro MK3 (full RGB support)
- Virtual MIDI device (IAC Driver) + MIDI test app

### MIDI Learn Workflow Test

1. **Start daemon with MIDI device**:
   ```bash
   ./target/release/midimon
   # Select your MIDI device port number
   ```

2. **Open GUI**:
   ```bash
   cd midimon-gui
   cargo tauri dev
   ```

3. **Test MIDI Learn**:
   - Go to Mappings tab
   - Click "+ Add Mapping"
   - Click "🎹 MIDI Learn"
   - **Press a pad on your controller**
   - Should show: "Trigger captured!"
   - Displays detected pattern (e.g., "Note 36 - medium (41-80)")

4. **Test Pattern Detection**:

   **Long Press**:
   - Click "🎹 MIDI Learn"
   - Press and **hold** a pad for 1+ seconds
   - Should detect: "Long Press Note 36 (1520ms)"

   **Double Tap**:
   - Click "🎹 MIDI Learn"
   - Quickly press same pad twice (within 500ms)
   - Should detect: "Double Tap Note 36 (timeout: 350ms)"

   **Chord**:
   - Click "🎹 MIDI Learn"
   - Press 2+ pads simultaneously (within 100ms)
   - Should detect: "Chord [36, 37, 38] (window: 100ms)"

   **Encoder Turn**:
   - Click "🎹 MIDI Learn"
   - Turn a knob/encoder on your controller
   - Should detect: "Encoder CC 1 (direction: clockwise)"

---

## Testing Path 4: Device Template Testing

### 1. Test Template Auto-Detection

```bash
# List your MIDI devices
./target/release/midimonctl devices  # (if this command exists)

# Or use midir to list:
cargo run --bin test_midi
```

### 2. Test Template Application (GUI)

1. Open GUI → Devices tab
2. Click "📋 Device Templates"
3. **Should see 6 templates**:
   - Maschine Mikro MK3
   - Launchpad Mini
   - Korg nanoKONTROL2
   - Akai APC Mini
   - Arturia BeatStep
   - Generic 25-Key MIDI Keyboard

4. **Select a template** (e.g., Maschine Mikro MK3)
5. **Preview should show**:
   - Name, manufacturer, description
   - MIDI patterns it matches
   - Category (pad-controller)

6. **Click "Apply Template"**
7. **Expected**:
   - Config file updated at ~/.config/midimon/config.toml
   - Daemon reloads
   - Alert: "Configuration created from template!"

8. **Verify**:
   ```bash
   cat ~/.config/midimon/config.toml
   # Should contain template config
   ```

### 3. Test Template Categories

In Template Selector:
- Click "Pad Controller" tab → Shows pad controllers
- Click "Keyboard" tab → Shows keyboard template
- Click "Mixer" tab → Shows mixer controllers
- Click "All" tab → Shows all 6 templates

---

## Testing Path 5: Per-App Profile Testing (macOS Only)

### 1. Grant Permissions

**Critical**: macOS requires Accessibility permissions for app detection.

1. Open **System Settings** → Privacy & Security → Accessibility
2. Click **+** and add:
   - Terminal.app (if running from terminal)
   - MIDIMon.app (if running GUI)
3. Toggle the switches **ON**

### 2. Test App Detection

```bash
# Start the GUI
cd midimon-gui
cargo tauri dev
```

**In GUI**:
1. Go to Settings tab
2. Open browser console (Cmd+Option+I)
3. Run in console:
   ```javascript
   await invoke('get_frontmost_app')
   ```
4. **Should return**:
   ```json
   {
     "bundle_id": "com.google.Chrome",
     "name": "Google Chrome",
     "path": "/Applications/Google Chrome.app",
     "pid": 12345
   }
   ```

5. **Switch apps** (Cmd+Tab to different app)
6. Run command again → Should show new app

### 3. Test Profile Creation

1. Go to Devices tab
2. Click "🔄 Profiles"
3. Click "+ New Profile"
4. Enter:
   - Name: "VS Code Profile"
   - Bundle IDs: `com.microsoft.VSCode`
5. Click "Save"
6. **Profile appears in list**

### 4. Test Profile Switching

**Manual Switch**:
1. In Profile Manager
2. Click on "VS Code Profile"
3. Click "Activate"
4. **Check**:
   - Config reloads
   - Status shows: "Profile: VS Code Profile"

**Automatic Switch** (requires app detection):
1. Create 2 profiles:
   - "Chrome Profile" → `com.google.Chrome`
   - "VS Code Profile" → `com.microsoft.VSCode`
2. Start app monitoring (if not auto-started)
3. Switch to Chrome (Cmd+Tab)
4. **Profile should auto-switch** to Chrome Profile
5. Switch to VS Code
6. **Profile should auto-switch** to VS Code Profile

### 5. Test Profile Export/Import

**Export**:
1. Select a profile
2. Click "Export"
3. Choose format: JSON or TOML
4. Save file
5. **Verify file contents**:
   ```bash
   cat ~/Downloads/vscode-profile.toml
   ```

**Import**:
1. Click "Import"
2. Select exported file
3. Enter new name (or keep same)
4. Click "Import"
5. **Profile appears in list**

---

## Testing Path 6: LED Feedback (Maschine Mikro MK3 Only)

### Prerequisites
- Native Instruments Maschine Mikro MK3
- NI USB drivers installed
- Accessibility permissions granted

### 1. Test LED Diagnostic

```bash
# Run LED diagnostic tool
cargo run --bin led_diagnostic

# Expected:
# - All 16 pads cycle through colors
# - RGB test pattern appears
# - No errors about HID access
```

### 2. Test LED Schemes

**Edit config** (~/.config/midimon/config.toml):
```toml
[led_settings]
scheme = "reactive"
brightness = 100
fade_time_ms = 1000
```

**Try different schemes**:
```bash
# Start daemon
./target/release/midimon 2  # Port 2 for Mikro MK3

# Test schemes (Ctrl+C and restart with different scheme)
./target/release/midimon 2 --led reactive
./target/release/midimon 2 --led rainbow
./target/release/midimon 2 --led breathing
./target/release/midimon 2 --led wave
./target/release/midimon 2 --led sparkle
```

**Expected LED behavior**:
- **Reactive**: Pads light up on press (green/yellow/red by velocity), fade after 1s
- **Rainbow**: Rotating rainbow pattern across all pads
- **Breathing**: Pulsing effect, all pads sync
- **Wave**: Cascading wave left-to-right
- **Sparkle**: Random twinkling

---

## Testing Path 7: Event Console Testing

### 1. GUI Event Console

1. Open GUI → Settings tab
2. Click "📊 Show Event Console"
3. **If MIDI device connected**:
   - Press pads → Events appear in console
   - Shows: Note number, velocity, timestamp
   - Color-coded by event type

4. **Event types displayed**:
   - Note On/Off (blue)
   - Control Change (green)
   - Processed events (yellow)
   - Actions (purple)
   - Errors (red)

### 2. CLI Event Monitoring

```bash
# If CLI event command exists
./target/release/midimonctl events

# Shows real-time event stream
# Press Ctrl+C to stop
```

---

## Common Testing Issues & Fixes

### Issue 1: "Daemon not running"

```bash
# Check if daemon process exists
ps aux | grep midimon

# If not running, start it
./target/release/midimon
```

### Issue 2: "Failed to connect to daemon"

```bash
# Check IPC socket exists
ls -la /tmp/midimon.sock

# If missing, restart daemon
./target/release/midimonctl stop
./target/release/midimon
```

### Issue 3: GUI won't start

```bash
# Check Node.js dependencies installed
cd midimon-gui/ui
npm install

# Check Tauri CLI installed
cargo install tauri-cli

# Try development mode
cargo tauri dev
```

### Issue 4: "No MIDI devices found"

```bash
# Check MIDI devices in Audio MIDI Setup
open -a "Audio MIDI Setup"

# Enable IAC Driver (virtual MIDI)
# Window → Show MIDI Studio → IAC Driver → Device is online ✓

# List MIDI ports
cargo run --bin test_midi
```

### Issue 5: Permissions errors (macOS)

```bash
# Grant Input Monitoring permission
# System Settings → Privacy & Security → Input Monitoring
# Add Terminal.app or MIDIMon.app

# Grant Accessibility permission (for app detection)
# System Settings → Privacy & Security → Accessibility
# Add Terminal.app or MIDIMon.app
```

### Issue 6: Build errors

```bash
# Update Rust toolchain
rustup update

# Clean build
cargo clean
cargo build --release --workspace

# Check Rust version
rustc --version  # Should be 1.70+
```

---

## Performance Testing

### 1. Config Reload Speed

```bash
# Benchmark reload time (should be <10ms)
time ./target/release/midimonctl reload

# Expected output:
# ✓ Configuration reloaded successfully
# Reload time: 3ms
#
# real    0m0.015s
# user    0m0.005s
# sys     0m0.008s
```

### 2. IPC Latency

```bash
# Measure ping latency (should be <5ms)
for i in {1..10}; do
  ./target/release/midimonctl ping
done

# Expected: All responses <5ms
```

### 3. MIDI Processing Latency

**With MIDI device**:
1. Press pad on controller
2. Watch Event Console
3. **Latency should be <10ms** from physical press to event display

---

## Test Report Template

After testing, use this template to document results:

```markdown
# MIDIMon v2.0.0 Test Report

**Date**: 2025-11-15
**Tester**: [Your Name]
**Platform**: macOS [version]
**Architecture**: [Intel/Apple Silicon]
**MIDI Device**: [Your controller or "IAC Driver"]

## Build Status
- [ ] Workspace builds without errors
- [ ] Build time: ___s (clean), ___s (incremental)

## Daemon Testing
- [ ] Daemon starts successfully
- [ ] Config hot-reload works (<10ms)
- [ ] IPC ping latency <5ms
- [ ] Config validation works

## GUI Testing
- [ ] GUI launches in dev mode
- [ ] System tray icon appears
- [ ] Status updates (every 2s)
- [ ] All 4 tabs load (Devices, Modes, Mappings, Settings)

## MIDI Learn Testing
- [ ] Single note detection works
- [ ] Long press detection (>1000ms)
- [ ] Double tap detection (<500ms)
- [ ] Chord detection (<100ms)
- [ ] Encoder direction detection
- [ ] TOML config generated correctly

## Device Templates
- [ ] All 6 templates load
- [ ] Template selection works
- [ ] Template application works
- [ ] Daemon reloads after apply

## Per-App Profiles
- [ ] App detection works (with permissions)
- [ ] Profile creation works
- [ ] Manual profile switching works
- [ ] Automatic switching works
- [ ] Profile import/export works

## LED Feedback (if Mikro MK3)
- [ ] Reactive scheme works
- [ ] Rainbow scheme works
- [ ] LED colors accurate

## Event Console
- [ ] Events display in GUI
- [ ] Events color-coded correctly
- [ ] Real-time updates work

## Issues Found
[List any bugs or issues]

## Performance Metrics
- Config reload: ___ms
- IPC ping: ___ms
- MIDI latency: ___ms
```

---

## Next Steps After Testing

1. **Report Issues**: Create GitHub issues for any bugs found
2. **Document Use Cases**: Write down your specific workflows
3. **Share Feedback**: What worked well? What needs improvement?
4. **Request Features**: What's missing that you need?

---

## Quick Reference Commands

```bash
# Build everything
cargo build --release --workspace

# Start daemon
./target/release/midimon

# Daemon control
./target/release/midimonctl status
./target/release/midimonctl reload
./target/release/midimonctl validate
./target/release/midimonctl ping
./target/release/midimonctl stop

# GUI development
cd midimon-gui
cargo tauri dev

# GUI production build
cd midimon-gui
cargo tauri build

# Diagnostic tools
cargo run --bin midi_diagnostic 2
cargo run --bin led_diagnostic
cargo run --bin test_midi

# Check daemon process
ps aux | grep midimon
ls -la /tmp/midimon.sock

# View config
cat ~/.config/midimon/config.toml
```

---

**Happy Testing!** 🎹
