# Configuration Examples

Real-world configuration patterns for common use cases. Each example is production-ready and can be adapted to your workflow.

## Table of Contents

1. [Basic Workflows](#basic-workflows)
2. [Developer Productivity](#developer-productivity)
3. [Content Creation](#content-creation)
4. [Repetition & Automation](#repetition--automation)
5. [Advanced Patterns](#advanced-patterns)
6. [Hybrid MIDI + Gamepad Configuration](#hybrid-midi--gamepad-configuration-v30)

## Basic Workflows

### Application Launcher

Quick launch frequently used applications:

```toml
[[modes]]
name = "Launcher"
color = "blue"

[[modes.mappings]]
description = "Open Terminal"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Launch"
app = "Terminal"

[[modes.mappings]]
description = "Open VS Code"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

[[modes.mappings]]
description = "Open Browser"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"
```

### Text Snippets

Common text expansions for email and documentation:

```toml
[[modes.mappings]]
description = "Email signature"
[modes.mappings.trigger]
type = "Note"
note = 70
[modes.mappings.action]
type = "Text"
text = """
Best regards,
Chris Joseph
Software Engineer
chris@amiable.dev
"""

[[modes.mappings]]
description = "Code review template"
[modes.mappings.trigger]
type = "Note"
note = 71
[modes.mappings.action]
type = "Text"
text = """
## Review Checklist
- [ ] Code follows style guide
- [ ] Tests are passing
- [ ] Documentation updated
- [ ] No obvious security issues
"""
```

## Developer Productivity

### Git Workflow

Common git operations on a single pad:

```toml
[[modes]]
name = "Git"
color = "green"

# Soft press: git status
[[modes.mappings]]
description = "Git status (soft)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 0, max = 40 }]
[modes.mappings.action]
type = "Shell"
command = "git status"

# Medium press: git add & commit
[[modes.mappings]]
description = "Quick commit (medium)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 41, max = 80 }]
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" }
]

# Hard press: commit and push
[[modes.mappings]]
description = "Commit and push (hard)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 81, max = 127 }]
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git push" }
]
```

### Code Navigation

Navigate code with velocity-based jumps:

```toml
# Jump lines based on velocity
[[modes.mappings]]
description = "Jump down (velocity-based)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 65
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "down" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "down" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "down" } } }
]

# Navigate between functions
[[modes.mappings]]
description = "Next function"
[modes.mappings.trigger]
type = "Note"
note = 66
[modes.mappings.action]
type = "Keystroke"
keys = "down"
modifiers = ["cmd", "shift"]
```

## Content Creation

### Video Editing

Timeline navigation and editing:

```toml
[[modes]]
name = "Video"
color = "purple"

# Play/Pause
[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Keystroke"
keys = "space"

# Frame-by-frame navigation
[[modes.mappings]]
description = "Previous frame"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Keystroke"
keys = "left"

[[modes.mappings]]
description = "Next frame"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Keystroke"
keys = "right"

# Jump 10 frames
[[modes.mappings]]
description = "Jump forward 10 frames"
[modes.mappings.trigger]
type = "Note"
note = 63
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "right" }
```

### Document Formatting

Batch formatting operations:

```toml
# Apply heading and next line
[[modes.mappings]]
description = "Apply heading 2"
[modes.mappings.trigger]
type = "Note"
note = 70
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "home" },  # Start of line
    { type = "Text", text = "## " },
    { type = "Keystroke", keys = "end" },  # End of line
    { type = "Keystroke", keys = "return" },  # New line
]
```

## Repetition & Automation

### Simple Repeats

Scroll through long lists or documents:

```toml
# Scroll down 10 lines
[[modes.mappings]]
description = "Scroll down 10 lines"
[modes.mappings.trigger]
type = "Note"
note = 80
[modes.mappings.action]
type = "Repeat"
count = 10
action = { type = "Keystroke", keys = "down" }

# Slow scroll (with delay)
[[modes.mappings]]
description = "Slow scroll down"
[modes.mappings.trigger]
type = "Note"
note = 81
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 200
action = { type = "Keystroke", keys = "down" }

# Page down 5 times
[[modes.mappings]]
description = "Jump 5 pages down"
[modes.mappings.trigger]
type = "Note"
note = 82
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 300
action = { type = "Keystroke", keys = "pagedown" }
```

### Batch File Processing

Process multiple files with a repeated sequence:

```toml
[[modes.mappings]]
description = "Batch process 3 files"
[modes.mappings.trigger]
type = "LongPress"
note = 85
hold_duration_ms = 1500
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 1000
action = {
    type = "Sequence",
    actions = [
        { type = "Keystroke", keys = "return" },  # Open file
        { type = "Delay", ms = 800 },  # Wait for file to load
        { type = "Keystroke", keys = "e", modifiers = ["cmd"] },  # Export
        { type = "Delay", ms = 500 },
        { type = "Keystroke", keys = "return" },  # Confirm export
        { type = "Delay", ms = 1000 },  # Wait for export
        { type = "Keystroke", keys = "w", modifiers = ["cmd"] },  # Close file
        { type = "Delay", ms = 200 },
        { type = "Keystroke", keys = "down" }  # Next file
    ]
}
```

### Velocity-Based Repeats

Different repeat counts based on pad pressure:

```toml
[[modes.mappings]]
description = "Variable repeat based on velocity"
[modes.mappings.trigger]
type = "VelocityRange"
note = 90
ranges = [
    # Soft: repeat 3 times
    { min = 0, max = 40, action = {
        type = "Repeat",
        count = 3,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Medium: repeat 5 times
    { min = 41, max = 80, action = {
        type = "Repeat",
        count = 5,
        delay_between_ms = 100,
        action = { type = "Keystroke", keys = "down" }
    }},
    # Hard: repeat 10 times
    { min = 81, max = 127, action = {
        type = "Repeat",
        count = 10,
        delay_between_ms = 50,
        action = { type = "Keystroke", keys = "down" }
    }}
]
```

### Nested Repeats

Multiply effect for grid or matrix operations:

```toml
# Navigate a 3x5 grid (15 total moves)
[[modes.mappings]]
description = "Navigate 3x5 grid"
[modes.mappings.trigger]
type = "DoubleTap"
note = 95
max_interval_ms = 300
[modes.mappings.action]
type = "Repeat"
count = 3  # 3 rows
delay_between_ms = 500
action = {
    type = "Sequence",
    actions = [
        # Move right 5 times
        { type = "Repeat", count = 5, delay_between_ms = 100, action = { type = "Keystroke", keys = "right" } },
        # Move down 1
        { type = "Delay", ms = 200 },
        { type = "Keystroke", keys = "down" },
        # Return to start of row
        { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "left" } }
    ]
}
```

### Retry Logic

Attempt operations with error tolerance:

```toml
[[modes.mappings]]
description = "Launch Xcode (retry 3 times)"
[modes.mappings.trigger]
type = "Note"
note = 100
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
stop_on_error = false  # Continue even if already running
action = { type = "Launch", app = "Xcode" }

[[modes.mappings]]
description = "Network test (stop on first failure)"
[modes.mappings.trigger]
type = "Note"
note = 101
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 1000
stop_on_error = true  # Stop if ping fails
action = { type = "Shell", command = "ping -c 1 8.8.8.8" }
```

## Advanced Patterns

### Spotlight Launcher

Use Spotlight for precise app launching:

```toml
[[modes.mappings]]
description = "Spotlight launcher for Terminal"
[modes.mappings.trigger]
type = "Note"
note = 110
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space", modifiers = ["cmd"] },  # Open Spotlight
    { type = "Delay", ms = 200 },  # Wait for Spotlight
    { type = "Text", text = "Terminal" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "return" }
]
```

### Form Automation

Fill complex forms with timing:

```toml
[[modes.mappings]]
description = "Fill registration form"
[modes.mappings.trigger]
type = "LongPress"
note = 115
hold_duration_ms = 2000
[modes.mappings.action]
type = "Sequence"
actions = [
    # First name
    { type = "Text", text = "Chris" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Last name
    { type = "Text", text = "Joseph" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Email
    { type = "Text", text = "chris@amiable.dev" },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "tab" },
    { type = "Delay", ms = 100 },

    # Phone
    { type = "Text", text = "+1-555-123-4567" },
    { type = "Delay", ms = 200 },

    # Submit
    { type = "Keystroke", keys = "return" }
]
```

### Multi-Click Automation

Repeat mouse clicks at intervals:

```toml
[[modes.mappings]]
description = "Click button 5 times"
[modes.mappings.trigger]
type = "DoubleTap"
note = 120
max_interval_ms = 300
[modes.mappings.action]
type = "Repeat"
count = 5
delay_between_ms = 500
action = { type = "MouseClick", button = "Left" }

# Click at specific location repeatedly
[[modes.mappings]]
description = "Click refresh button"
[modes.mappings.trigger]
type = "Note"
note = 121
[modes.mappings.action]
type = "Repeat"
count = 3
delay_between_ms = 2000
action = { type = "MouseClick", button = "Left", x = 1200, y = 100 }
```

### Macro Pad Layout

Complete 16-pad layout for development:

```toml
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Development"
color = "green"

# Row 1: Git operations
[[modes.mappings]]
description = "Git status"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Shell"
command = "git status"

[[modes.mappings]]
description = "Git diff"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Shell"
command = "git diff"

[[modes.mappings]]
description = "Quick commit"
[modes.mappings.trigger]
type = "Note"
note = 62
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Shell", command = "git add -A" },
    { type = "Delay", ms = 100 },
    { type = "Shell", command = "git commit -m 'quick save'" }
]

[[modes.mappings]]
description = "Git push"
[modes.mappings.trigger]
type = "Note"
note = 63
[modes.mappings.action]
type = "Shell"
command = "git push"

# Row 2: Build & test
[[modes.mappings]]
description = "Build project"
[modes.mappings.trigger]
type = "Note"
note = 64
[modes.mappings.action]
type = "Shell"
command = "cargo build"

[[modes.mappings]]
description = "Run tests"
[modes.mappings.trigger]
type = "Note"
note = 65
[modes.mappings.action]
type = "Shell"
command = "cargo test"

[[modes.mappings]]
description = "Run app"
[modes.mappings.trigger]
type = "Note"
note = 66
[modes.mappings.action]
type = "Shell"
command = "cargo run"

[[modes.mappings]]
description = "Format code"
[modes.mappings.trigger]
type = "Note"
note = 67
[modes.mappings.action]
type = "Shell"
command = "cargo fmt"

# Row 3: Navigation
[[modes.mappings]]
description = "Jump down (velocity)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 68
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "down" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "down" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "down" } } }
]

[[modes.mappings]]
description = "Jump up (velocity)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 69
ranges = [
    { min = 0, max = 40, action = { type = "Keystroke", keys = "up" } },
    { min = 41, max = 80, action = { type = "Repeat", count = 5, action = { type = "Keystroke", keys = "up" } } },
    { min = 81, max = 127, action = { type = "Repeat", count = 10, action = { type = "Keystroke", keys = "up" } } }
]

# Row 4: Utilities
[[modes.mappings]]
description = "Open Terminal"
[modes.mappings.trigger]
type = "Note"
note = 72
[modes.mappings.action]
type = "Launch"
app = "Terminal"

[[modes.mappings]]
description = "Open VS Code"
[modes.mappings.trigger]
type = "Note"
note = 73
[modes.mappings.action]
type = "Launch"
app = "Visual Studio Code"

[[modes.mappings]]
description = "Open Browser"
[modes.mappings.trigger]
type = "Note"
note = 74
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"
```

### Context-Aware Actions (Conditional)

Execute different actions based on runtime conditions like active app, time of day, or system state.

#### App-Based Conditional

Different behavior when specific apps are running:

```toml
[[modes.mappings]]
description = "Context-aware play/pause"
[modes.mappings.trigger]
type = "Note"
note = 130
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.apple.Logic" }
]
then_action = { type = "Keystroke", keys = "space" }  # Logic: Space to play/pause
else_action = { type = "Keystroke", keys = "space", modifiers = ["cmd"] }  # System: Media play/pause
```

#### Time-Based Automation

Launch different apps based on work hours:

```toml
[[modes.mappings]]
description = "Time-aware launcher"
[modes.mappings.trigger]
type = "Note"
note = 131
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" },
    { type = "DayOfWeek", days = ["Mon", "Tue", "Wed", "Thu", "Fri"] }
]
operator = "And"
then_action = { type = "Launch", app = "Slack" }  # Work app during work hours
else_action = { type = "Launch", app = "Discord" }  # Personal app otherwise
```

#### Multiple Conditions (OR Logic)

Launch file if any IDE is running:

```toml
[[modes.mappings]]
description = "Open project in active IDE"
[modes.mappings.trigger]
type = "Note"
note = 132
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppRunning", bundle_id = "com.microsoft.VSCode" },
    { type = "AppRunning", bundle_id = "com.jetbrains.IntelliJ" },
    { type = "AppRunning", bundle_id = "com.sublimetext.4" }
]
operator = "Or"  # Any one IDE is enough
then_action = { type = "Keystroke", keys = "o", modifiers = ["cmd"] }  # Cmd+O to open file
else_action = { type = "Launch", app = "Visual Studio Code" }  # Launch IDE if none running
```

#### Modifier-Based Conditional

Hold Shift for alternative behavior:

```toml
[[modes.mappings]]
description = "Delete or force-delete"
[modes.mappings.trigger]
type = "Note"
note = 133
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "ModifierPressed", modifier = "Shift" }
]
then_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd", "option"] }  # Force delete
else_action = { type = "Keystroke", keys = "delete", modifiers = ["cmd"] }  # Normal delete
```

#### Nested Conditionals

Complex decision trees with multiple levels:

```toml
[[modes.mappings]]
description = "Smart launcher with time and app detection"
[modes.mappings.trigger]
type = "LongPress"
note = 135
hold_duration_ms = 1000
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "TimeRange", start = "09:00", end = "17:00" }
]
then_action = {
    type = "Conditional",
    conditions = [
        { type = "AppRunning", bundle_id = "com.apple.Logic" }
    ],
    then_action = { type = "Keystroke", keys = "n", modifiers = ["cmd"] },  # New Logic project
    else_action = { type = "Launch", app = "Logic Pro" }  # Launch Logic
}
else_action = {
    type = "Conditional",
    conditions = [
        { type = "DayOfWeek", days = ["Sat", "Sun"] }
    ],
    then_action = { type = "Launch", app = "Spotify" },  # Weekend music
    else_action = { type = "Launch", app = "Safari" }  # Evening browsing
}
```

#### Launch Only if Not Running

Avoid duplicate app launches:

```toml
[[modes.mappings]]
description = "Launch Spotify or play/pause if running"
[modes.mappings.trigger]
type = "Note"
note = 136
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "AppNotRunning", bundle_id = "com.spotify.client" }
]
then_action = { type = "Launch", app = "Spotify" }
else_action = { type = "Keystroke", keys = "space" }  # Already running, just play/pause
```

#### Mode-Aware Shortcuts

Different actions per mode using global mappings:

```toml
[[global_mappings]]
description = "Mode-aware F5 key"
[global_mappings.trigger]
type = "Note"
note = 137
[global_mappings.action]
type = "Conditional"
conditions = [
    { type = "ModeActive", mode = 1 }  # Development mode
]
then_action = { type = "Shell", command = "npm test" }  # Run tests in dev mode
else_action = { type = "Keystroke", keys = "f5" }  # Refresh in other modes
```

#### Weekend vs Weekday Behavior

Different workflows based on day of week:

```toml
[[modes.mappings]]
description = "Morning routine launcher"
[modes.mappings.trigger]
type = "Note"
note = 138
[modes.mappings.action]
type = "Conditional"
conditions = [
    { type = "DayOfWeek", days = ["Sat", "Sun"] },
    { type = "TimeRange", start = "08:00", end = "12:00" }
]
operator = "And"
then_action = {
    type = "Sequence",
    actions = [
        { type = "Launch", app = "Apple Music" },
        { type = "Delay", ms = 1000 },
        { type = "Launch", app = "Mail" }
    ]
}
else_action = {
    type = "Sequence",
    actions = [
        { type = "Launch", app = "Slack" },
        { type = "Delay", ms = 1000 },
        { type = "Launch", app = "Calendar" },
        { type = "Delay", ms = 500 },
        { type = "Launch", app = "Visual Studio Code" }
    ]
}
```

## Hybrid MIDI + Gamepad Configuration (v3.0+)

Conductor v3.0 introduces support for Game Controllers (HID) alongside MIDI devices, enabling powerful hybrid workflows that combine the velocity-sensitive expressiveness of MIDI controllers with the ergonomic button layouts and analog sticks of gamepads, joysticks, racing wheels, flight sticks, HOTAS setups, and other game controllers.

### Why Hybrid Mode?

**Advantages**:
- **Best of Both Worlds**: MIDI's velocity sensitivity + gamepad's ergonomic buttons and analog sticks
- **No ID Conflicts**: MIDI uses IDs 0-127, gamepad uses 128-255
- **Seamless Integration**: Both devices work simultaneously through unified event stream
- **Device-Specific Strengths**: Use each device for what it does best

**Common Use Cases**:
- Music production with MIDI pads for recording + gamepad for DAW navigation
- Live performance with MIDI keyboard + racing wheel pedals for effects control
- Video editing with MIDI controller for timeline + gamepad for playback controls
- Development with MIDI macro pad + gamepad for shortcuts and window management

### Example 1: Music Production Workflow

**Setup**: Maschine Mikro MK3 (MIDI) + Xbox Controller (Gamepad)

This configuration demonstrates a complete music production workflow using both devices:
- **MIDI Device**: Velocity-sensitive pads for recording, playback control, and instrument triggering
- **Gamepad Device**: Ergonomic navigation, shortcuts, and transport controls

```toml
# Hybrid MIDI + Gamepad Configuration for Music Production
# Device: Maschine Mikro MK3 (MIDI) + Xbox Controller (Gamepad)
# Author: Conductor Examples
# Version: 3.0

[device]
name = "Hybrid Production"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

###########################################
# Mode 0: Production (Default)
###########################################

[[modes]]
name = "Production"
color = "blue"

# ========================================
# MIDI CONTROLLER MAPPINGS (IDs 0-127)
# ========================================

# --- Recording Controls (MIDI Pads) ---

[[modes.mappings]]
description = "Record (soft press)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60  # MIDI Pad 1
ranges = [{ min = 0, max = 40 }]
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Record with count-in (medium press)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 41, max = 80 }]
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "k", modifiers = ["cmd"] },  # Enable count-in
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "r", modifiers = ["cmd"] }   # Start recording
]

[[modes.mappings]]
description = "Punch record (hard press)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [{ min = 81, max = 127 }]
[modes.mappings.action]
type = "Keystroke"
keys = "r"
modifiers = ["cmd", "shift"]

# --- Playback Controls (MIDI Pads) ---

[[modes.mappings]]
description = "Play/Pause"
[modes.mappings.trigger]
type = "Note"
note = 61  # MIDI Pad 2
[modes.mappings.action]
type = "Keystroke"
keys = "space"

[[modes.mappings]]
description = "Stop"
[modes.mappings.trigger]
type = "Note"
note = 62  # MIDI Pad 3
[modes.mappings.action]
type = "Keystroke"
keys = "return"

[[modes.mappings]]
description = "Loop toggle"
[modes.mappings.trigger]
type = "Note"
note = 63  # MIDI Pad 4
[modes.mappings.action]
type = "Keystroke"
keys = "l"
modifiers = ["cmd"]

# --- Track Operations (MIDI Pads) ---

[[modes.mappings]]
description = "New track"
[modes.mappings.trigger]
type = "Note"
note = 64  # MIDI Pad 5
[modes.mappings.action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Duplicate track"
[modes.mappings.trigger]
type = "Note"
note = 65  # MIDI Pad 6
[modes.mappings.action]
type = "Keystroke"
keys = "d"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Delete track"
[modes.mappings.trigger]
type = "LongPress"
note = 66  # MIDI Pad 7 (hold to delete)
hold_duration_ms = 1500
[modes.mappings.action]
type = "Keystroke"
keys = "delete"
modifiers = ["cmd"]

# --- Quick Save (MIDI Pad) ---

[[modes.mappings]]
description = "Quick save"
[modes.mappings.trigger]
type = "Note"
note = 67  # MIDI Pad 8
[modes.mappings.action]
type = "Keystroke"
keys = "s"
modifiers = ["cmd"]

# --- Volume Control (MIDI Encoder) ---

[[modes.mappings]]
description = "Volume up"
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 0
direction = "Clockwise"
[modes.mappings.action]
type = "VolumeControl"
action = "Up"

[[modes.mappings]]
description = "Volume down"
[modes.mappings.trigger]
type = "EncoderTurn"
encoder = 0
direction = "CounterClockwise"
[modes.mappings.action]
type = "VolumeControl"
action = "Down"

# ========================================
# GAMEPAD CONTROLLER MAPPINGS (IDs 128-255)
# ========================================

# --- DAW Shortcuts (Face Buttons) ---

[[modes.mappings]]
description = "Copy (A button)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # A (Xbox) / Cross (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "c"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Paste (B button)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129  # B (Xbox) / Circle (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Undo (X button)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130  # X (Xbox) / Square (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "z"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Redo (Y button)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131  # Y (Xbox) / Triangle (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "z"
modifiers = ["cmd", "shift"]

# --- Track Navigation (D-Pad) ---

[[modes.mappings]]
description = "Previous track"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132  # D-Pad Up
[modes.mappings.action]
type = "Keystroke"
keys = "up"

[[modes.mappings]]
description = "Next track"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133  # D-Pad Down
[modes.mappings.action]
type = "Keystroke"
keys = "down"

[[modes.mappings]]
description = "Jump to start"
[modes.mappings.trigger]
type = "GamepadButton"
button = 134  # D-Pad Left
[modes.mappings.action]
type = "Keystroke"
keys = "return"

[[modes.mappings]]
description = "Jump to end"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135  # D-Pad Right
[modes.mappings.action]
type = "Keystroke"
keys = "end"

# --- Zoom Controls (Shoulder Buttons) ---

[[modes.mappings]]
description = "Zoom out (LB)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136  # LB (Xbox) / L1 (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "-"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Zoom in (RB)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137  # RB (Xbox) / R1 (PS)
[modes.mappings.action]
type = "Keystroke"
keys = "="
modifiers = ["cmd"]

# --- Timeline Scroll (Left Analog Stick) ---

[[modes.mappings]]
description = "Scroll timeline left"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 128  # Left Stick X
direction = "Negative"
threshold = 0.3
[modes.mappings.action]
type = "Keystroke"
keys = "left"

[[modes.mappings]]
description = "Scroll timeline right"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 128  # Left Stick X
direction = "Positive"
threshold = 0.3
[modes.mappings.action]
type = "Keystroke"
keys = "right"

# --- Mixer Navigation (Right Analog Stick) ---

[[modes.mappings]]
description = "Scroll mixer up"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 131  # Right Stick Y
direction = "Negative"
threshold = 0.3
[modes.mappings.action]
type = "Keystroke"
keys = "pageup"

[[modes.mappings]]
description = "Scroll mixer down"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 131  # Right Stick Y
direction = "Positive"
threshold = 0.3
[modes.mappings.action]
type = "Keystroke"
keys = "pagedown"

# --- Quick Actions (Triggers) ---

[[modes.mappings]]
description = "Fade in (LT analog)"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 132  # Left Trigger
direction = "Positive"
threshold = 0.5
[modes.mappings.action]
type = "Keystroke"
keys = "f"
modifiers = ["cmd", "shift"]

[[modes.mappings]]
description = "Fade out (RT analog)"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 133  # Right Trigger
direction = "Positive"
threshold = 0.5
[modes.mappings.action]
type = "Keystroke"
keys = "g"
modifiers = ["cmd", "shift"]

# --- Quick Markers (Stick Clicks) ---

[[modes.mappings]]
description = "Add marker (L3)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 138  # Left Stick Click
[modes.mappings.action]
type = "Keystroke"
keys = "m"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Next marker (R3)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 139  # Right Stick Click
[modes.mappings.action]
type = "Keystroke"
keys = "'"
modifiers = ["cmd"]

# ========================================
# HYBRID CHORD MAPPINGS (MIDI + Gamepad)
# ========================================

[[modes.mappings]]
description = "Emergency save (MIDI Pad 1 + Gamepad Start)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [60, 140]  # MIDI note 60 + Gamepad Start button
timeout_ms = 100
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "s", modifiers = ["cmd"] },
    { type = "Delay", ms = 100 },
    { type = "Text", text = "Project saved!" }
]

###########################################
# Mode 1: Media Control
###########################################

[[modes]]
name = "Media"
color = "purple"

# --- Media Playback (Gamepad Face Buttons) ---

[[modes.mappings]]
description = "Play/Pause (A)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Next track (B)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "right"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Previous track (X)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Keystroke"
keys = "left"
modifiers = ["cmd"]

# --- Volume (MIDI Pads with Velocity) ---

[[modes.mappings]]
description = "Volume control (velocity-based)"
[modes.mappings.trigger]
type = "VelocityRange"
note = 60
ranges = [
    { min = 0, max = 40, action = { type = "VolumeControl", action = "Down" } },
    { min = 41, max = 80, action = { type = "VolumeControl", action = "Set", volume = 50 } },
    { min = 81, max = 127, action = { type = "VolumeControl", action = "Up" } }
]

###########################################
# Mode 2: Navigation
###########################################

[[modes]]
name = "Navigation"
color = "green"

# --- Window Management (Gamepad) ---

[[modes.mappings]]
description = "Switch apps (D-Pad Left/Right)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135  # D-Pad Right
[modes.mappings.action]
type = "Keystroke"
keys = "tab"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Mission Control (D-Pad Up)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "up"
modifiers = ["ctrl"]

# --- Quick Launch (MIDI Pads) ---

[[modes.mappings]]
description = "Launch DAW"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Launch"
app = "Logic Pro"

[[modes.mappings]]
description = "Launch Browser"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Launch"
app = "Google Chrome"

###########################################
# GLOBAL MAPPINGS (All Modes)
###########################################

[[global_mappings]]
description = "Mode switch: Encoder right = next mode"
[global_mappings.trigger]
type = "EncoderTurn"
encoder = 0
direction = "Clockwise"
[global_mappings.action]
type = "ModeChange"
mode = "Next"

[[global_mappings]]
description = "Mode switch: Encoder left = previous mode"
[global_mappings.trigger]
type = "EncoderTurn"
encoder = 0
direction = "CounterClockwise"
[global_mappings.action]
type = "ModeChange"
mode = "Previous"

[[global_mappings]]
description = "Emergency exit (Gamepad Select + Start)"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [141, 140]  # Select + Start
timeout_ms = 50
[global_mappings.action]
type = "Shell"
command = "pkill conductor"

[[global_mappings]]
description = "Quick mute (MIDI Pad 16 + Gamepad B)"
[global_mappings.trigger]
type = "NoteChord"
notes = [75, 129]  # MIDI note 75 + Gamepad B button
timeout_ms = 100
[global_mappings.action]
type = "VolumeControl"
action = "Mute"
```

### Example 2: Live Performance with Racing Wheel

**Setup**: MIDI Keyboard (61 keys) + Logitech G29 Racing Wheel

This creative configuration uses a racing wheel's pedals for real-time effects control during live performance:

```toml
# Hybrid MIDI Keyboard + Racing Wheel Configuration
# Use Case: Live electronic music performance with tactile effects control
# Device: MIDI Keyboard + Racing Wheel (pedals for effects)
# Version: 3.0

[device]
name = "Performance Rig"
auto_connect = true

[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

[[modes]]
name = "Performance"
color = "red"

# ========================================
# MIDI KEYBOARD (Standard note playing)
# ========================================

# Notes 0-127 pass through to DAW for instrument playing
# (Configure pass-through in your DAW)

# --- Quick Octave Shift (MIDI CC or Program Change) ---

[[modes.mappings]]
description = "Octave up"
[modes.mappings.trigger]
type = "Note"
note = 127  # Highest note
[modes.mappings.action]
type = "Keystroke"
keys = "up"
modifiers = ["shift"]

[[modes.mappings]]
description = "Octave down"
[modes.mappings.trigger]
type = "Note"
note = 0  # Lowest note
[modes.mappings.action]
type = "Keystroke"
keys = "down"
modifiers = ["shift"]

# ========================================
# RACING WHEEL PEDALS (Effects Control)
# ========================================

# --- Gas Pedal: Reverb/Delay Mix (Analog Control) ---

[[modes.mappings]]
description = "Increase reverb (gas pedal pressed)"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 133  # Right Trigger (gas pedal)
direction = "Positive"
threshold = 0.2
[modes.mappings.action]
type = "Keystroke"
keys = "1"  # DAW macro: increase reverb send

# --- Brake Pedal: Filter Cutoff (Analog Control) ---

[[modes.mappings]]
description = "Lower filter cutoff (brake pedal pressed)"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 132  # Left Trigger (brake pedal)
direction = "Positive"
threshold = 0.2
[modes.mappings.action]
type = "Keystroke"
keys = "2"  # DAW macro: decrease filter cutoff

# --- Clutch Pedal: Distortion Amount (if 3-pedal wheel) ---

[[modes.mappings]]
description = "Add distortion (clutch pedal)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 143  # Clutch pedal (digital threshold)
[modes.mappings.action]
type = "Keystroke"
keys = "3"  # DAW macro: enable distortion

# --- Wheel Buttons: Scene Launcher ---

[[modes.mappings]]
description = "Launch scene 1 (wheel button 1)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128  # Button on wheel
[modes.mappings.action]
type = "Keystroke"
keys = "1"
modifiers = ["cmd"]

[[modes.mappings]]
description = "Launch scene 2 (wheel button 2)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Keystroke"
keys = "2"
modifiers = ["cmd"]

# --- Shifter Paddles: Loop Control ---

[[modes.mappings]]
description = "Loop start (left paddle)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136  # Left shoulder
[modes.mappings.action]
type = "Keystroke"
keys = "["
modifiers = ["cmd"]

[[modes.mappings]]
description = "Loop end (right paddle)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137  # Right shoulder
[modes.mappings.action]
type = "Keystroke"
keys = "]"
modifiers = ["cmd"]

# --- Hybrid Chord: Panic Stop (Keyboard + Wheel) ---

[[modes.mappings]]
description = "All sound off (MIDI note + wheel center button)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [60, 142]  # Middle C + wheel center/guide button
timeout_ms = 100
[modes.mappings.action]
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "space" },  # Stop playback
    { type = "VolumeControl", action = "Mute" },  # Mute audio
    { type = "Delay", ms = 100 },
    { type = "Text", text = "EMERGENCY STOP ACTIVATED" }
]

[[global_mappings]]
description = "Emergency exit"
[global_mappings.trigger]
type = "GamepadButtonChord"
buttons = [141, 140]  # Select + Start on wheel
timeout_ms = 50
[global_mappings.action]
type = "Shell"
command = "pkill conductor"
```

### Setup Instructions

#### 1. Verify Device Connections

Before starting, ensure both devices are recognized:

```bash
# Check MIDI devices
cargo run --bin test_midi

# Check gamepad devices
cargo run --bin gamepad_diagnostic

# You should see both devices listed
```

#### 2. Configure Hybrid Mode

In your `config.toml`, hybrid mode is enabled automatically when you use both MIDI (0-127) and gamepad (128-255) IDs in your mappings.

#### 3. Test Individual Mappings

Start Conductor and test each device separately:

```bash
# Start in debug mode to see events
DEBUG=1 conductor --foreground

# Test MIDI pads/keys (you'll see note events 0-127)
# Test gamepad buttons (you'll see button events 128-255)
```

#### 4. Mode Switching

The examples above use the MIDI encoder for mode switching. Alternatively, you can use gamepad buttons:

```toml
[[global_mappings]]
description = "Mode switch with gamepad Start button"
[global_mappings.trigger]
type = "GamepadButton"
button = 140  # Start button
[global_mappings.action]
type = "ModeChange"
mode = "Next"
```

### Troubleshooting Hybrid Mode

#### Both Devices Not Responding

1. **Check connection**:
   ```bash
   # List devices
   cargo run --bin test_midi
   cargo run --bin gamepad_diagnostic
   ```

2. **Verify auto_connect**:
   ```toml
   [device]
   auto_connect = true  # Must be enabled
   ```

3. **Check daemon logs**:
   ```bash
   DEBUG=1 conductor --foreground
   # Look for "Connected to MIDI device" and "Connected to gamepad"
   ```

#### Only MIDI or Only Gamepad Working

1. **Verify ID ranges**: MIDI must use 0-127, gamepad must use 128-255
2. **Check for ID conflicts**: No overlapping IDs between devices
3. **Test individually**:
   ```bash
   # MIDI-only config
   cargo run --release 2  # Replace 2 with your MIDI port number

   # Gamepad-only config
   # (Remove MIDI mappings temporarily)
   ```

#### Hybrid Chords Not Triggering

1. **Increase chord timeout**:
   ```toml
   [advanced_settings]
   chord_timeout_ms = 100  # Increase from 50ms
   ```

2. **Check button IDs**: Verify you're using correct MIDI note + gamepad button ID
3. **Use `NoteChord` for MIDI+gamepad**: Not `GamepadButtonChord`

#### Latency Issues

1. **Reduce chord_timeout_ms** for faster single-button response
2. **Check system load**: Hybrid mode uses minimal CPU but check for other processes
3. **Update drivers**: Ensure gamepad drivers are up to date

### Advanced Hybrid Techniques

#### 1. Device-Specific Modes

Create modes optimized for each device:

```toml
[[modes]]
name = "MIDI Focus"  # Heavy MIDI use
color = "blue"
# 80% MIDI mappings, 20% gamepad

[[modes]]
name = "Gamepad Focus"  # Heavy gamepad use
color = "green"
# 20% MIDI mappings, 80% gamepad
```

#### 2. Layered Control

Use one device to modify the other's behavior:

```toml
# Hold gamepad LB to make MIDI pads switch modes instead of triggering actions
[[modes.mappings]]
description = "Mode 1 (MIDI Pad 1 + LB held)"
[modes.mappings.trigger]
type = "NoteChord"
notes = [60, 136]  # Pad 1 + LB
timeout_ms = 50
[modes.mappings.action]
type = "ModeChange"
mode = 1
```

#### 3. Analog Precision Control

Use gamepad analog triggers for precise parameter control:

```toml
# Fine volume control with trigger pressure
[[modes.mappings]]
description = "Precise volume (RT analog)"
[modes.mappings.trigger]
type = "GamepadAxisTurn"
axis = 133  # Right Trigger
direction = "Positive"
threshold = 0.1  # Very sensitive
[modes.mappings.action]
type = "VolumeControl"
action = "Set"
volume = 50  # Map to analog value in future versions
```

### Performance Considerations

**Hybrid Mode Overhead**:
- CPU: <1% additional overhead
- Latency: <1ms additional latency
- Memory: ~2-5MB for gamepad library

**Best Practices**:
1. Use MIDI for velocity-sensitive, musical tasks
2. Use gamepad for navigation, shortcuts, and ergonomic controls
3. Avoid excessive chord mappings (keep under 20 total)
4. Test thoroughly before live use

### See Also

- [Gamepad Support Guide](../guides/gamepad-support.md) - Complete gamepad documentation
- [Gamepad API Reference](../reference/gamepad-api.md) - Technical API details
- [Action Types Reference](../reference/action-types.md) - All available actions
- [Trigger Types](../reference/trigger-types.md) - All trigger configurations

## Performance Tips

### Timing Best Practices

1. **Application Launch**: Wait 1000-2000ms after launching apps
2. **UI Navigation**: Use 100-200ms between UI interactions
3. **Form Fills**: 100ms delay between field navigation
4. **File Operations**: 500-1000ms for save/load operations
5. **Network Operations**: 2000ms+ for remote operations

### Repeat Guidelines

1. **Start Small**: Begin with count=1, increase gradually
2. **Add Delays**: Use `delay_between_ms` for counts >5
3. **Test Incrementally**: Verify each step before combining
4. **Consider UX**: Counts >100 may appear as hang
5. **Monitor Performance**: Watch CPU/memory with large counts

### Debugging

Enable debug mode to see action execution:

```bash
DEBUG=1 cargo run --release 2
```

Common issues:
- **Timing too tight**: Increase `delay_between_ms`
- **Actions not executing**: Check application focus
- **Repeats stopping early**: Check `stop_on_error` setting
- **Slow performance**: Reduce repeat counts or add delays

## See Also

- [Actions Reference](actions.md) - Complete action type documentation
- [Action Types Reference](../reference/action-types.md) - Technical specifications
- [Trigger Types](../reference/trigger-types.md) - Trigger configuration
