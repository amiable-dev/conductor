# Device Templates Guide

Device templates provide pre-configured mappings for popular MIDI controllers, letting you get started in seconds instead of hours.

## What are Device Templates?

Device templates are pre-built configuration profiles that include:

- **Note mappings** for all pads/buttons
- **Common actions** (copy/paste, play/pause, volume control)
- **LED configurations** optimized for each device
- **Mode layouts** designed for typical workflows

Instead of manually mapping 16+ pads, load a template and customize only what you need.

## Built-In Templates (v2.0.0)

Conductor includes 6 built-in templates for popular controllers:

### 1. Maschine Mikro MK3
**Full RGB LED support, 16 pads, encoder**

**Pre-configured**:
- 16 velocity-sensitive pads
- Encoder for volume control
- 4 modes (Default, Development, Media, Custom)
- Reactive LED feedback

**Best for**: Music production, software development, general productivity

### 2. Launchpad Mini MK3
**RGB LED grid, 64 pads**

**Pre-configured**:
- 8x8 pad grid
- Scene launch buttons
- Rainbow LED schemes
- Mode selection via top row

**Best for**: Ableton Live control, clip launching, grid-based workflows

### 3. Novation Launchkey Mini MK3
**25 keys, 16 pads, 8 knobs**

**Pre-configured**:
- 16 drum pads
- 8 knobs for parameter control
- Pitch/modulation wheels
- Transport controls

**Best for**: Music production, MIDI sequencing, DAW control

### 4. AKAI MPK Mini MK3
**25 keys, 8 pads, 8 knobs**

**Pre-configured**:
- 8 MPC-style pads
- 8 assignable knobs
- Arpeggiator controls
- 4-way joystick

**Best for**: Beat making, music production, performance

### 5. Korg nanoKONTROL2
**8 faders, 8 knobs, 24 buttons**

**Pre-configured**:
- 8-channel mixer layout
- Transport controls
- Scene/marker buttons
- Fader automation

**Best for**: DAW mixing, transport control, automation

### 6. APC Mini
**64 pads, 9 faders**

**Pre-configured**:
- 8x8 clip launch grid
- Scene launch column
- 9 channel faders
- Shift button combinations

**Best for**: Ableton Live, clip launching, mixing

## Loading a Template

### Via GUI (Recommended)

1. **Open Conductor GUI**

2. **Go to Settings** → **Device Templates**

3. **Select your controller** from the dropdown

4. **Click "Load Template"**

5. **Confirm** the load (replaces current config)

6. **Test** - Press pads to verify mappings

7. **Customize** using MIDI Learn for any changes

### Via CLI

Templates are stored as TOML files in:
```
~/.config/conductor/templates/
```

**Load manually**:
```bash
# Copy template to config location
cp ~/.config/conductor/templates/maschine-mikro-mk3.toml ~/.config/conductor/config.toml

# Reload daemon
conductorctl reload
```

## Template Structure

Templates are standard `config.toml` files with device-specific optimizations:

```toml
[device]
name = "Maschine Mikro MK3"
template = "maschine-mikro-mk3"
auto_connect = true

[led_feedback]
scheme = "reactive"
default_color = [0, 120, 255]  # Blue

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Pad 1 - Copy"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

# ... more mappings
```

## Customizing Templates

After loading a template:

### 1. Use MIDI Learn

The fastest way to modify mappings:

1. Click **Edit** on any mapping
2. Click **Learn** next to the trigger
3. Press the pad/button you want
4. Update the action
5. Save

### 2. Add New Modes

Templates include 2-4 modes. Add more:

1. Go to **Modes** panel
2. Click **Add Mode**
3. Set name and color
4. Add mappings using MIDI Learn

### 3. Adjust LED Schemes

Change LED behavior:

1. Go to **Settings** → **LED Feedback**
2. Select scheme: Reactive, Rainbow, Pulse, etc.
3. Customize colors
4. Save

### 4. Export Customized Template

Save your modifications:

1. Go to **Settings** → **Export Config**
2. Save as new template file
3. Share with others or use on other machines

## Creating Custom Templates

You can create templates for controllers not included:

### Step 1: Map Your Device

1. **Connect device** and use MIDI Learn for all controls
2. **Organize into modes** (Default, Media, Development, etc.)
3. **Configure LED feedback** (if supported)
4. **Test thoroughly** with real workflows

### Step 2: Export Template

1. **Settings** → **Export Config**
2. Save as `my-controller-name.toml`

### Step 3: Add Metadata

Edit the exported file to add template metadata:

```toml
[template]
name = "My Controller"
description = "Template for My MIDI Controller v2"
author = "Your Name"
version = "1.0.0"
created_at = "2025-11-14"
device_ids = ["USB MIDI Device", "My Controller MIDI 1"]
```

### Step 4: Test Template

1. **Reset config** to default
2. **Load your template**
3. **Verify all mappings** work
4. **Check LED behavior**

### Step 5: Share (Optional)

Submit to Conductor template library:

1. Create PR to `config/device_templates/` directory
2. Include template file + documentation
3. Add device to compatibility matrix

## Template Compatibility

Templates are forward-compatible across Conductor versions:

- ✅ v2.0.0 templates work in v2.1.0+
- ⚠️ Newer features may not load in older versions
- ✅ Missing features gracefully ignored

## Troubleshooting

### Template Not Loading

**Symptoms**: "Failed to load template" error

**Solutions**:
1. **Check template file** exists in `~/.config/conductor/templates/`
2. **Validate TOML syntax**: `conductorctl validate --config path/to/template.toml`
3. **Check permissions**: Template file must be readable
4. **View error details** in Event Console

### Wrong Note Numbers

**Symptoms**: Template loads but pads don't trigger actions

**Solutions**:
1. **Check device mode**: Some controllers have multiple MIDI modes
2. **Verify MIDI channel**: Template may use different channel than device
3. **Use MIDI Learn** to detect actual note numbers
4. **Check Event Console** to see incoming MIDI events

### LEDs Not Working

**Symptoms**: Template loads but LEDs don't respond

**Solutions**:
1. **Check device supports RGB**: Some controllers only have single-color LEDs
2. **Verify HID access**: Grant Input Monitoring permission
3. **Try different LED scheme**: Some schemes may not work on all devices
4. **Check template LED config**: May be disabled in template

## Best Practices

### Tip 1: Load Template First

When setting up a new controller:
1. **Load template first** (if available)
2. **Test default mappings**
3. **Customize only what you need**

This saves 90% of setup time.

### Tip 2: Create Per-App Variants

Use templates as a base for per-app profiles:

1. Load template
2. Customize for specific app (Logic Pro, VS Code, etc.)
3. Export as new template
4. Assign to app in Per-App Profiles

### Tip 3: Version Your Templates

When customizing templates:
```toml
[template]
version = "1.0.0"
base_template = "maschine-mikro-mk3"
customized_by = "Your Name"
customized_at = "2025-11-14"
```

This helps track changes over time.

### Tip 4: Test Before Sharing

Before submitting templates:
- ✅ Test all mappings
- ✅ Verify LED feedback
- ✅ Test in multiple apps
- ✅ Document any device-specific quirks

## Next Steps

- [MIDI Learn Mode](../getting-started/midi-learn.md) - Customize templates
- [Per-App Profiles](./per-app-profiles.md) - Create app-specific variants
- [LED System](./led-system.md) - Customize LED behavior
- [Configuration Reference](../configuration/overview.md) - Template file format

---

**Last Updated**: November 14, 2025 (v2.0.0)
