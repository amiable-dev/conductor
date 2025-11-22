# MIDI Output Troubleshooting

This guide helps you diagnose and resolve common issues with Conductor's MIDI output functionality, including SendMIDI actions, virtual MIDI ports, and DAW integration.

## Quick Diagnostic Checklist

Before diving into specific issues, run through this quick checklist:

- [ ] Virtual MIDI port is created and online (IAC Driver, loopMIDI, ALSA)
- [ ] DAW has MIDI input enabled for the virtual port
- [ ] Conductor config specifies the correct port name (case-sensitive)
- [ ] MIDI messages are being sent (check `conductor --verbose` logs)
- [ ] DAW is receiving MIDI (check DAW's MIDI monitor)
- [ ] Correct MIDI channel is used (usually Channel 0)
- [ ] No other applications are blocking the MIDI port

## Common Issues

### Issue 1: "Port not found" Error

**Symptoms**:
```
ERROR: MIDI output port "IAC Driver Bus 1" not found
Available ports: []
```

**Possible Causes**:
1. Virtual MIDI port not created or offline
2. Port name mismatch (case-sensitive)
3. Conductor started before port was created
4. Permissions issue (Linux/Windows)

**Solutions**:

#### macOS (IAC Driver)

1. **Verify IAC Driver is online**:
   ```bash
   # Open Audio MIDI Setup
   open -a "Audio MIDI Setup"
   ```
   - Window → Show MIDI Studio
   - Double-click **IAC Driver**
   - Check **"Device is online"**
   - Click **Apply**

2. **List available MIDI ports** to verify the exact name:
   ```bash
   # Use conductor diagnostic tool
   cargo run --bin test_midi
   ```

3. **Restart Conductor daemon**:
   ```bash
   conductorctl stop
   conductor
   ```

#### Windows (loopMIDI)

1. **Verify loopMIDI is running**:
   - Check system tray for loopMIDI icon
   - If not running, launch loopMIDI from Start Menu

2. **Verify port exists** in loopMIDI:
   - Open loopMIDI application
   - Ensure at least one port is listed (e.g., "Conductor Virtual")

3. **Check exact port name** (case-sensitive):
   - Note the exact name shown in loopMIDI
   - Update `config.toml` to match exactly:
     ```toml
     [modes.mappings.action.then_action]
     type = "SendMIDI"
     port = "Conductor Virtual"  # Must match exactly
     ```

4. **Restart Conductor**:
   ```bash
   conductorctl stop
   conductor
   ```

#### Linux (ALSA)

1. **Verify virtual MIDI port exists**:
   ```bash
   aconnect -l
   ```
   Look for "Virtual Raw MIDI" or similar.

2. **Load ALSA virtual MIDI module** if missing:
   ```bash
   sudo modprobe snd-virmidi
   ```

3. **Verify port name** and update config:
   ```bash
   # List MIDI ports
   aconnect -l
   ```
   Update `config.toml` with the exact port name.

4. **Permissions** (if port access denied):
   ```bash
   # Add user to audio group
   sudo usermod -a -G audio $USER

   # Log out and log back in for changes to take effect
   ```

---

### Issue 2: Messages Sent But Not Received by DAW

**Symptoms**:
- Conductor logs show messages being sent
- DAW doesn't respond (no transport control, no parameter changes)
- No error messages

**Diagnosis**:

1. **Enable verbose logging** in Conductor:
   ```bash
   conductor --verbose
   ```
   Look for lines like:
   ```
   [DEBUG] Sending MIDI: NoteOn(note=60, velocity=100, channel=0) to port "IAC Driver Bus 1"
   ```

2. **Check DAW's MIDI monitor**:
   - **Logic Pro**: View → Show MIDI Environment → Monitor
   - **Ableton Live**: Preferences → Link, Tempo & MIDI → MIDI Ports (check Track/Remote enabled)
   - **Reaper**: View → MIDI Device Diagnostics

**Possible Causes & Solutions**:

#### Cause 1: DAW MIDI Input Not Enabled

**Logic Pro**:
1. Open **Logic Pro → Settings → MIDI → Inputs** (⌘,)
2. Locate your virtual MIDI port
3. Check the box to enable it
4. Close Settings

**Ableton Live**:
1. Open **Preferences → Link, Tempo & MIDI**
2. In **MIDI Ports** section, find your virtual port
3. Enable **Track** and **Remote** for the input port
4. Close Preferences

**Reaper**:
1. Open **Preferences → MIDI Devices**
2. Find your virtual MIDI port in the input list
3. Enable **Enable input from this device**
4. Click **OK**

#### Cause 2: Wrong MIDI Channel

**Solution**: Most DAWs use Channel 0 (sometimes labeled as "Channel 1" in DAW UI). Verify your config:

```toml
[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 60
velocity = 100
channel = 0  # Change to match DAW expectations
```

**Test different channels** (0-15) to find the correct one.

#### Cause 3: Wrong CC Numbers or Note Numbers

**Solution**: Use DAW's MIDI Learn feature to discover correct values:

**Logic Pro**:
1. **Logic Pro → Control Surfaces → Learn Assignment**
2. Click the parameter to control
3. Note the CC number shown (update your config)

**Ableton Live**:
1. Click **MIDI** button (top-right, or press ⌘M)
2. Click the parameter to control
3. Press your MIDI controller (Ableton learns the mapping)
4. Note the CC/Note number (if you want to replicate in config)

#### Cause 4: Control Surface Conflicts

**Solution**: Disable conflicting control surfaces in DAW:

**Logic Pro**:
- **Logic Pro → Control Surfaces → Setup**
- Disable any control surfaces using the same MIDI port

**Ableton Live**:
- **Preferences → Link, Tempo & MIDI**
- In **Control Surface** section, ensure no conflicting surface is assigned to the same port

---

### Issue 3: Latency / Delayed Response

**Symptoms**:
- Noticeable delay (>50ms) between pad press and DAW response
- Audio/MIDI feels "laggy"

**Possible Causes & Solutions**:

#### Cause 1: High Audio Buffer Size

**Solution**: Lower DAW's audio buffer size:

**Logic Pro**:
1. **Logic Pro → Settings → Audio**
2. Reduce **I/O Buffer Size** to 128 or 64 samples
3. Note: Lower buffer = lower latency, but higher CPU usage

**Ableton Live**:
1. **Preferences → Audio**
2. Reduce **Buffer Size** to 128 or 64 samples

**Reaper**:
1. **Preferences → Audio → Device**
2. Reduce **Block size** to 128 or 64 samples

#### Cause 2: Third-Party Virtual MIDI Drivers

**Windows**: loopMIDI adds ~5-10ms latency. This is usually acceptable, but if critical:
- Consider rtpMIDI (network MIDI) for even lower latency
- Ensure loopMIDI is up to date

**macOS**: Use IAC Driver (native, near-zero latency) instead of third-party apps.

**Linux**: Use ALSA virtual ports (native, low latency) instead of JACK MIDI (higher latency).

#### Cause 3: System CPU Load

**Solution**:
1. Close unnecessary applications
2. Freeze/bounce tracks in DAW to reduce CPU usage
3. Increase audio buffer size if CPU is maxed out (trade latency for stability)

#### Cause 4: Conductor Processing Delay

**Diagnosis**: Check Conductor logs for processing time:
```bash
conductor --verbose
```

If you see warnings about slow processing, check:
1. Complex velocity curves (use simpler curves)
2. Conditional actions with many conditions (simplify)
3. Sequence actions with many steps (reduce)

**Solution**: Simplify mappings to reduce processing overhead.

---

### Issue 4: Messages Sent to Wrong Port

**Symptoms**:
- DAW receives MIDI from a different source
- Conductor logs show correct port, but DAW sees different port

**Possible Causes & Solutions**:

#### Cause 1: Multiple Virtual MIDI Ports

**Solution**: List all available MIDI ports and verify exact name:

```bash
# Use Conductor diagnostic tool
cargo run --bin test_midi
```

Update config to use the correct port name (case-sensitive):
```toml
[modes.mappings.action.then_action]
type = "SendMIDI"
port = "IAC Driver Bus 1"  # Not "IAC Driver Bus 2"
```

#### Cause 2: Port Name Changed

**macOS (IAC Driver)**:
- If you renamed the bus in Audio MIDI Setup, update your config

**Windows (loopMIDI)**:
- If you renamed the port in loopMIDI, update your config
- Restart Conductor after renaming

---

### Issue 5: Velocity Not Working

**Symptoms**:
- All MIDI messages sent with same velocity (e.g., always 127)
- Velocity curves not being applied

**Possible Causes & Solutions**:

#### Cause 1: Fixed Velocity in Config

**Problem**: Config specifies a fixed velocity value:
```toml
[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 60
velocity = 127  # Fixed velocity
channel = 0
```

**Solution**: Use a velocity curve to map input velocity:
```toml
[modes.mappings.action.then_action.message]
type = "NoteOn"
note = 60
channel = 0

[modes.mappings.action.then_action.message.velocity_curve]
type = "PassThrough"  # Use controller velocity directly
```

#### Cause 2: Trigger Doesn't Capture Velocity

**Problem**: Trigger type doesn't include velocity information.

**Solution**: Use a trigger that captures velocity:
```toml
[modes.mappings.trigger]
type = "Note"  # Captures velocity
note = 1
# Velocity is automatically available to velocity_curve
```

---

### Issue 6: Virtual Port Not Visible in DAW

**Symptoms**:
- Virtual MIDI port exists (verified via OS tools)
- DAW doesn't list the port in MIDI preferences

**Possible Causes & Solutions**:

#### macOS (IAC Driver)

**Cause**: DAW started before IAC Driver was enabled.

**Solution**:
1. Enable IAC Driver in Audio MIDI Setup
2. Restart the DAW
3. Alternatively: **Logic Pro → Settings → MIDI → Reset MIDI Drivers**

#### Windows (loopMIDI)

**Cause**: DAW doesn't detect dynamically created ports.

**Solution**:
1. Create loopMIDI port **before** launching DAW
2. If DAW is running, restart it after creating the port

#### Linux (ALSA)

**Cause**: ALSA virtual port created after DAW startup.

**Solution**:
1. Load `snd-virmidi` module before launching DAW:
   ```bash
   sudo modprobe snd-virmidi
   ```
2. Restart DAW
3. Alternatively: Use `aconnect` to manually connect ports

---

## Platform-Specific Issues

### macOS Specific

#### IAC Driver "Device is offline" After Reboot

**Problem**: IAC Driver unchecks itself after macOS reboot.

**Solution**:
1. Open **Audio MIDI Setup**
2. Double-click **IAC Driver**
3. Check **"Device is online"**
4. This should persist, but if not, consider using an AppleScript to auto-enable on login

#### Permissions Issues with HID Device

**Problem**: Conductor can't access MIDI controller hardware.

**Solution**:
1. Grant **Input Monitoring** permissions:
   - **System Settings → Privacy & Security → Input Monitoring**
   - Enable for Terminal (if running `cargo run`)
2. Restart Conductor

### Windows Specific

#### loopMIDI Port Disappears

**Problem**: loopMIDI port intermittently disappears.

**Solution**:
1. Ensure loopMIDI is set to **start with Windows**:
   - Right-click loopMIDI tray icon → Settings → **Start with Windows**
2. If port disappears, restart loopMIDI application
3. Restart Conductor daemon

#### Multiple MIDI Drivers Conflict

**Problem**: Multiple virtual MIDI drivers (loopMIDI, rtpMIDI, etc.) cause conflicts.

**Solution**:
1. Use only one virtual MIDI driver at a time
2. Disable/uninstall unused drivers
3. Assign unique port names if using multiple drivers

### Linux Specific

#### ALSA Permissions Denied

**Problem**: Conductor can't access ALSA virtual MIDI ports.

**Solution**:
```bash
# Add user to audio group
sudo usermod -a -G audio $USER

# Log out and log back in

# Verify group membership
groups | grep audio
```

#### JACK MIDI vs ALSA

**Problem**: DAW uses JACK MIDI, but Conductor uses ALSA.

**Solution**:
1. Bridge ALSA to JACK using `a2jmidid`:
   ```bash
   sudo apt-get install a2jmidid
   a2jmidid -e &
   ```
2. Use `qjackctl` to route ALSA virtual port to JACK
3. Alternatively: Use JACK-native virtual MIDI ports instead of ALSA

---

## Advanced Debugging

### Enable Conductor Debug Logging

Run Conductor with verbose logging to see detailed MIDI output:

```bash
conductor --verbose
```

**What to look for**:
- `[DEBUG] Sending MIDI: ...` - Confirms messages are being sent
- `[ERROR] Port not found: ...` - Port name issues
- `[WARN] Slow processing time: ...` - Performance issues

### Monitor MIDI Traffic in DAW

**Logic Pro**:
1. View → Show MIDI Environment
2. Click **Monitor** object
3. Watch for incoming MIDI messages

**Ableton Live**:
1. Preferences → Link, Tempo & MIDI
2. Enable **Track** and **Remote** for the port
3. Create a MIDI track
4. Arm the track for recording
5. Watch the MIDI input meter

**Reaper**:
1. View → MIDI Device Diagnostics
2. Select your virtual MIDI port
3. Watch for incoming messages

### Test MIDI Loopback

Create a loopback test to verify MIDI output is working:

1. **Route virtual output back to Conductor input** (macOS):
   - Audio MIDI Setup → IAC Driver
   - Create two buses: "Conductor Out" and "Conductor In"
   - Configure Conductor to send to "Conductor Out"
   - Configure Conductor to receive from "Conductor In"
   - Use a MIDI routing app to bridge them

2. **Send a test message**:
   ```bash
   # Press a pad and verify it's received on the input
   conductor --verbose
   ```

---

## Getting Help

If you've tried all troubleshooting steps and still have issues:

1. **Gather diagnostic information**:
   ```bash
   # List MIDI ports
   cargo run --bin test_midi > midi_ports.txt

   # Run Conductor with verbose logging
   conductor --verbose 2>&1 | tee conductor_debug.log

   # (Press pads to trigger actions)

   # Stop after 30 seconds
   conductorctl stop
   ```

2. **Check configuration**:
   ```bash
   cat ~/.config/conductor/config.toml
   ```

3. **Report the issue**:
   - GitHub: https://github.com/amiable-dev/conductor/issues
   - Include:
     - Platform (macOS, Windows, Linux)
     - Conductor version (`conductor --version`)
     - DAW name and version
     - MIDI port name
     - Relevant config.toml sections
     - Debug logs (conductor_debug.log)
     - midi_ports.txt

---

## See Also

- [DAW Control Guide](../guides/daw-control.md) - General MIDI output concepts
- [Logic Pro Integration](../examples/logic-pro.md) - Logic Pro-specific setup
- [Ableton Live Integration](../examples/ableton-live.md) - Ableton Live-specific setup
- [Configuration Reference](../configuration/actions.md) - SendMIDI action syntax
