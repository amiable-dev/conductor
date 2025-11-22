# Automation & Power User Workflows

Advanced productivity automation for form filling, app switching, window management, and repetitive task elimination.

## Overview

Conductor enables power users to automate complex workflows that traditional macro tools can't handle. Use velocity sensitivity for multi-function buttons, context-aware profiles for app-specific shortcuts, and sequences for multi-step automation.

**What You'll Learn**:
- Form automation (one-button form filling)
- App-specific macro sets with per-app profiles
- Window management and workspace automation
- Context-aware mappings (time-based, app-based)
- Advanced sequences and conditional actions

---

## Quick Start: One-Button Form Filling

Automatically fill web forms with saved data using a single button press.

```toml
[device]
name = "Form Automation"
auto_connect = true

[[modes]]
name = "Form Fill"
color = "green"

# Complete Form Fill (Name, Email, Phone, Address)
[[modes.mappings]]
description = "Pad 1: Fill Contact Form"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Sequence"
actions = [
    { Text = { text = "John Doe" } },
    { Keystroke = { keys = "Tab" } },
    { Text = { text = "john.doe@example.com" } },
    { Keystroke = { keys = "Tab" } },
    { Text = { text = "(555) 123-4567" } },
    { Keystroke = { keys = "Tab" } },
    { Text = { text = "123 Main St, City, ST 12345" } }
]

# Velocity-Sensitive Form Options
[[modes.mappings]]
description = "Pad 2 Soft: Fill Work Email"
[modes.mappings.trigger]
type = "VelocityRange"
note = 37
min_velocity = 0
max_velocity = 40
[modes.mappings.action]
type = "Text"
text = "john.doe@company.com"

[[modes.mappings]]
description = "Pad 2 Hard: Fill Personal Email"
[modes.mappings.trigger]
type = "VelocityRange"
note = 37
min_velocity = 81
max_velocity = 127
[modes.mappings.action]
type = "Text"
text = "john.personal@gmail.com"
```

**Time Saved**: 30-60 seconds per form × 10 forms/day = 5-10 minutes daily

---

## Per-App Profile Switching

Different mappings for different applications automatically.

### Browser (Chrome/Firefox)

```toml
# Browser-Specific Profile
[[modes.mappings]]
description = "A Button: New Tab (Browser)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Conditional"
conditions = [
    {
        type = "AppActive",
        app_name = "Google Chrome",
        action = { Keystroke = { keys = "t", modifiers = ["cmd"] } }
    },
    {
        type = "AppActive",
        app_name = "Firefox",
        action = { Keystroke = { keys = "t", modifiers = ["cmd"] } }
    }
]

# Browser Navigation
[[modes.mappings]]
description = "D-Pad Left: Back"
[modes.mappings.trigger]
type = "GamepadButton"
button = 134
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"
modifiers = ["cmd"]

[[modes.mappings]]
description = "D-Pad Right: Forward"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["cmd"]
```

### Email Client (Mail/Outlook)

```toml
# Email-Specific Actions
[[modes.mappings]]
description = "A: New Email (Mail.app)"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Conditional"
conditions = [
    {
        type = "AppActive",
        app_name = "Mail",
        action = { Keystroke = { keys = "n", modifiers = ["cmd"] } }
    }
]

# Quick Replies
[[modes.mappings]]
description = "Pad 1: Reply with Template"
[modes.mappings.trigger]
type = "Note"
note = 36
[modes.mappings.action]
type = "Sequence"
actions = [
    { Keystroke = { keys = "r", modifiers = ["cmd"] } },  # Reply
    { Delay = { duration_ms = 200 } },
    { Text = { text = "Thanks for your email. I'll get back to you shortly.\n\nBest,\nJohn" } }
]
```

---

## Window Management Automation

Advanced workspace and window control.

### macOS Mission Control & Spaces

```toml
[[modes]]
name = "Window Management"
color = "cyan"

# Workspace Navigation
[[modes.mappings]]
description = "D-Pad Left: Previous Desktop"
[modes.mappings.trigger]
type = "GamepadButton"
button = 134
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"
modifiers = ["ctrl"]

[[modes.mappings]]
description = "D-Pad Right: Next Desktop"
[modes.mappings.trigger]
type = "GamepadButton"
button = 135
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["ctrl"]

# Mission Control
[[modes.mappings]]
description = "D-Pad Up: Mission Control"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "F3"

# Show Desktop
[[modes.mappings]]
description = "D-Pad Down: Show Desktop"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133
[modes.mappings.action]
type = "Keystroke"
keys = "F11"

# Window Snapping (requires Rectangle.app or similar)
[[modes.mappings]]
description = "LB: Snap Left Half"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Keystroke"
keys = "LeftArrow"
modifiers = ["ctrl", "opt"]

[[modes.mappings]]
description = "RB: Snap Right Half"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137
[modes.mappings.action]
type = "Keystroke"
keys = "RightArrow"
modifiers = ["ctrl", "opt"]

# Maximize
[[modes.mappings]]
description = "LB+RB: Maximize Window"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "Return"
modifiers = ["ctrl", "opt"]
```

---

## App Launcher Matrix

Quick launch common apps with button grid.

```toml
# App Launching (Face Buttons)
[[modes.mappings]]
description = "A: Launch Browser"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Launch"
application = "Google Chrome"

[[modes.mappings]]
description = "B: Launch Terminal"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Launch"
application = "Terminal"

[[modes.mappings]]
description = "X: Launch VS Code"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Launch"
application = "Visual Studio Code"

[[modes.mappings]]
description = "Y: Launch Slack"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Launch"
application = "Slack"

# Chord Combinations for More Apps
[[modes.mappings]]
description = "LB+A: Launch Finder"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 128]
timeout_ms = 50
[modes.mappings.action]
type = "Launch"
application = "Finder"

[[modes.mappings]]
description = "LB+B: Launch Spotify"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 129]
timeout_ms = 50
[modes.mappings.action]
type = "Launch"
application = "Spotify"
```

---

## Context-Aware Automation

### Time-Based Mappings

```toml
# Morning Routine (6am-9am)
[[modes.mappings]]
description = "Morning: Launch Email + Calendar"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Conditional"
conditions = [
    {
        type = "TimeRange",
        start_time = "06:00",
        end_time = "09:00",
        action = {
            Sequence = {
                actions = [
                    { Launch = { application = "Mail" } },
                    { Delay = { duration_ms = 500 } },
                    { Launch = { application = "Calendar" } }
                ]
            }
        }
    }
]

# Evening Routine (6pm-11pm)
[[modes.mappings]]
description = "Evening: Launch Spotify + Dimming"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Conditional"
conditions = [
    {
        type = "TimeRange",
        start_time = "18:00",
        end_time = "23:00",
        action = {
            Sequence = {
                actions = [
                    { Launch = { application = "Spotify" } },
                    { Shell = { command = "osascript -e 'tell app \"System Events\" to key code 107'" } }  # F14 for dimming
                ]
            }
        }
    }
]
```

---

## Clipboard Management

Advanced clipboard operations.

```toml
# Clipboard History (requires Alfred or Clipboard Manager)
[[modes.mappings]]
description = "LT: Paste from Clipboard History"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 64
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["cmd", "shift"]  # Alfred clipboard history

# Paste as Plain Text
[[modes.mappings]]
description = "RT: Paste Plain Text"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64
[modes.mappings.action]
type = "Keystroke"
keys = "v"
modifiers = ["cmd", "shift", "opt"]
```

---

## System Automation

### Screenshot & Screen Recording

```toml
# Screenshots
[[modes.mappings]]
description = "Select: Full Screenshot"
[modes.mappings.trigger]
type = "GamepadButton"
button = 141
[modes.mappings.action]
type = "Keystroke"
keys = "3"
modifiers = ["cmd", "shift"]

[[modes.mappings]]
description = "Select (Hold): Selection Screenshot"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 141
duration_ms = 1000
[modes.mappings.action]
type = "Keystroke"
keys = "4"
modifiers = ["cmd", "shift"]

# Screen Recording
[[modes.mappings]]
description = "Start (Hold 2s): Start Recording"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 140
duration_ms = 2000
[modes.mappings.action]
type = "Keystroke"
keys = "5"
modifiers = ["cmd", "shift"]
```

### Lock Screen & Sleep

```toml
[[modes.mappings]]
description = "LB+RB+Start: Lock Screen"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137, 140]
timeout_ms = 50
[modes.mappings.action]
type = "Keystroke"
keys = "q"
modifiers = ["ctrl", "cmd"]
```

---

## Troubleshooting

### Text Not Typing Correctly
- **Problem**: Text actions type gibberish or incomplete
- **Solution**: Add delays between Text and Keystroke actions (100-200ms)
- **Example**: `{ Delay = { duration_ms = 150 } }`

### Per-App Profiles Not Switching
- **Problem**: Same mappings work across all apps
- **Solution**: Ensure app names match exactly (check Activity Monitor)
- **Case-Sensitive**: "Google Chrome" ≠ "google chrome"

### Sequences Executing Out of Order
- **Problem**: Actions in Sequence trigger in wrong order
- **Solution**: Increase delays between actions (try 200-300ms)

---

## Time Savings Calculator

| Task | Manual Time | Automated Time | Daily Frequency | Time Saved |
|------|-------------|----------------|-----------------|------------|
| Form filling | 45s | 2s | 10× | 7m 10s |
| Email reply | 60s | 5s | 15× | 13m 45s |
| App switching | 5s | 1s | 50× | 3m 20s |
| Window management | 8s | 1s | 30× | 3m 30s |
| **Total Daily** | - | - | - | **27m 45s** |
| **Annual** | - | - | - | **115 hours** |

---

## Next Steps

- **[See Developer Workflows](developer-workflows.md)** - Git automation
- **[See Streaming Examples](streaming.md)** - OBS automation
- **[Learn Sequences](../configuration/actions.md#sequence)** - Multi-step actions
- **[Explore Conditionals](../configuration/conditionals.md)** - Context-aware mappings
