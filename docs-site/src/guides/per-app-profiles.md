# Per-App Profiles

Automatically switch configurations based on the frontmost application. Use different MIDI mappings for VS Code, Photoshop, Final Cut Pro, etc.

## Overview

Per-app profiles enable context-aware MIDI control:

- **Automatic switching**: Profiles activate when you switch apps
- **App-specific mappings**: Different buttons for different workflows
- **Manual override**: Force a specific profile when needed
- **Profile inheritance**: Share common mappings across profiles

## Getting Started

### 1. Enable App Detection

```bash
# macOS: Grant Accessibility permissions
# System Settings → Privacy & Security → Accessibility → Conductor

# Verify app detection is working
conductorctl frontmost-app
```

### 2. Create Profiles

#### Via GUI

1. Open Conductor GUI
2. Navigate to **Devices** tab
3. Click **🔄 Profiles**
4. Click **+ New Profile**
5. Enter profile name (e.g., "vscode-profile")
6. Configure mappings
7. Save profile

#### Via Config File

Create separate profile files in `~/.config/conductor/profiles/`:

```bash
~/.config/conductor/profiles/
├── default.toml          # Fallback profile
├── vscode.toml           # VS Code mappings
├── photoshop.toml        # Photoshop mappings
└── fcpx.toml             # Final Cut Pro mappings
```

### 3. Register App Associations

Map applications to profiles:

#### GUI Method

1. In Profile Manager dialog
2. Click **Associate App**
3. Select profile
4. Choose application from list
5. Click **Save**

#### Config Method

Edit `~/.config/conductor/app_profiles.toml`:

```toml
[app_associations]
"Visual Studio Code" = "vscode"
"Code" = "vscode"  # Alternative app name
"Adobe Photoshop" = "photoshop"
"Final Cut Pro" = "fcpx"
```

## Profile Configuration

### Profile Structure

Each profile is a complete Conductor configuration:

```toml
# profiles/vscode.toml

[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Coding"
color = "blue"

# Code navigation mappings
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 36

[modes.mappings.action]
type = "Keystroke"
modifiers = ["Ctrl"]
keys = "P"  # Quick file open

[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 37

[modes.mappings.action]
type = "Keystroke"
modifiers = ["Ctrl", "Shift"]
keys = "F"  # Find in files

# Debug controls
[[modes.mappings]]
[modes.mappings.trigger]
type = "Note"
note = 38

[modes.mappings.action]
type = "Keystroke"
keys = "F5"  # Start debugging
```

### Shared Mappings

Use global mappings that work across all profiles:

```toml
# In each profile, include common mappings
[[global_mappings]]
[global_mappings.trigger]
type = "EncoderTurn"
cc = 7
direction = "Clockwise"

[global_mappings.action]
type = "VolumeControl"
action = "Up"
```

## App Detection

### Supported Platforms

- **macOS**: Full support via Accessibility API
- **Linux**: Full support via X11/Wayland
- **Windows**: Full support via Win32 API

### App Name Matching

Conductor matches application names flexibly:

```toml
# All of these will match Visual Studio Code
"Visual Studio Code" = "vscode"
"Code" = "vscode"
"code" = "vscode"
"VSCode" = "vscode"
```

### Wildcards

Use wildcards for partial matching:

```toml
"*Adobe*" = "adobe-suite"  # Matches any Adobe app
"Chrome*" = "browser"       # Matches Chrome variants
```

## Profile Switching

### Automatic Switching

When app detection is enabled:

1. Focus changes to new application
2. Conductor detects the frontmost app
3. Looks up associated profile
4. Loads and activates profile
5. LED feedback shows profile change (optional)

Switching latency: ~50ms

### Manual Override

Force a specific profile:

#### GUI Method

1. Click **🔄 Profiles** button
2. Select profile from list
3. Click **Activate**

#### CLI Method

```bash
conductorctl switch-profile vscode
```

### Default Fallback

If no profile matches the current app, Conductor uses:

1. `default` profile (if exists)
2. Main `config.toml`

## Use Cases

### Software Development

```toml
# profiles/dev.toml
# Buttons for common IDE actions

[[modes.mappings]]
# Run tests
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
modifiers = ["Ctrl"]
keys = "T"

[[modes.mappings]]
# Git commit
[modes.mappings.trigger]
type = "Note"
note = 37
[modes.mappings.action]
type = "Shell"
command = "git commit"

[[modes.mappings]]
# Format code
[modes.mappings.trigger]
type = "Note"
note = 38
[modes.mappings.action]
type = "Keystroke"
modifiers = ["Shift", "Alt"]
keys = "F"
```

### Video Editing

```toml
# profiles/video.toml
# Timeline control and playback

[[modes.mappings]]
# Play/Pause
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
keys = "Space"

[[modes.mappings]]
# Cut clip
[modes.mappings.trigger]
type = "Note"
note = 37
[modes.mappings.action]
type = "Keystroke"
modifiers = ["Cmd"]
keys = "B"

[[modes.mappings]]
# Ripple delete
[modes.mappings.trigger]
type = "LongPress"
note = 37
duration_ms = 1000
[modes.mappings.action]
type = "Keystroke"
modifiers = ["Shift"]
keys = "Delete"
```

### Photo Editing

```toml
# profiles/photo.toml
# Brush size, zoom, undo/redo

[[modes.mappings]]
# Increase brush size
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "Clockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "]"

[[modes.mappings]]
# Decrease brush size
[modes.mappings.trigger]
type = "EncoderTurn"
cc = 1
direction = "CounterClockwise"
[modes.mappings.action]
type = "Keystroke"
keys = "["

[[modes.mappings]]
# Undo
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Keystroke"
modifiers = ["Cmd"]
keys = "Z"
```

## Profile Management

### Export Profiles

Share or back up profiles:

#### GUI Method

1. Profile Manager → Select profile
2. Click **Export**
3. Choose format (TOML/JSON)
4. Save file

#### CLI Method

```bash
conductorctl export-profile vscode > vscode-profile.toml
```

### Import Profiles

Load profiles from files:

#### GUI Method

1. Profile Manager → **Import**
2. Select file
3. Choose name for imported profile
4. Click **Import**

#### CLI Method

```bash
conductorctl import-profile vscode-profile.toml
```

### Profile Validation

Test profiles before activating:

```bash
conductorctl validate-profile vscode
```

## Troubleshooting

### App Not Detected

1. **Check permissions**:
   - macOS: Accessibility permissions granted
   - Linux: Running with sufficient privileges
   - Windows: No UAC blocking

2. **Verify app name**:
   ```bash
   conductorctl frontmost-app
   ```

3. **Check association**:
   - Ensure app name in config matches actual name
   - Use wildcards if app name varies

### Profile Not Switching

1. **Check app detection is enabled**:
   ```bash
   conductorctl status
   ```

2. **Verify profile exists**:
   ```bash
   ls ~/.config/conductor/profiles/
   ```

3. **Test manual switch**:
   ```bash
   conductorctl switch-profile vscode
   ```

4. **Check logs**:
   ```bash
   conductorctl logs | grep profile
   ```

### Wrong Profile Activates

1. Check association priority
2. Verify no conflicting wildcards
3. Review app name matching in logs

## Best Practices

1. **Start with defaults**: Create a base profile with common mappings

2. **Use inheritance**: Share global mappings across profiles

3. **Test thoroughly**: Verify each profile works in target app

4. **Name clearly**: Use descriptive profile names (e.g., "davinci-resolve" not "prof1")

5. **Document mappings**: Add comments in TOML files

6. **Version control**: Keep profiles in git for history

7. **Export regularly**: Back up working profiles

8. **Share templates**: Contribute profiles for popular apps

## Advanced Features

### Conditional Switching

Switch based on multiple criteria:

```toml
[switching_rules]
# Only switch to fcpx profile if specific project is open
[[switching_rules.conditions]]
app = "Final Cut Pro"
window_title = "*MyProject*"
profile = "fcpx-myproject"
```

### Profile Chains

Load multiple profiles in sequence:

```toml
[profile_chain]
base = "default"
overlay = "vscode"
# Loads default, then overlays vscode-specific mappings
```

### Hot Reload

Profiles support hot reload:

1. Edit profile file
2. Save changes
3. Switch to different app and back
4. Profile reloads automatically

## Next Steps

- Learn about [MIDI Learn](../getting-started/midi-learn.md) to quickly create profile mappings
- Set up [device templates](./device-templates.md) for consistent layouts
- Use the [event console](./event-console.md) to test profile mappings
- Explore [LED feedback](./led-system.md) for visual profile indication
