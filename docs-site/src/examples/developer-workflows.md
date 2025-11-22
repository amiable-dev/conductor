# Developer Workflows

Automate your development environment with one-button git operations, build triggers, and environment switching.

## Overview

Conductor turns gamepads and MIDI controllers into powerful developer tools. Map git workflows, build commands, test runners, and environment management to physical buttons.

**What You'll Learn**:
- One-button git operations (velocity-sensitive)
- Build/test/deploy automation
- Docker and container management
- IDE and terminal shortcuts
- Window management and workspace switching

---

## Quick Start: One-Button Git Workflow

Map all common git operations to a single gamepad button using velocity sensitivity.

```toml
[device]
name = "Developer Setup"
auto_connect = true

[[modes]]
name = "Development"
color = "blue"

# Xbox A Button: Git Operations (Velocity-Sensitive)
[[modes.mappings]]
description = "A Button Soft: Git Status"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
# Soft tap
[modes.mappings.action]
type = "Shell"
command = "cd $(pwd) && git status"

[[modes.mappings]]
description = "A Button (Hold 1s): Git Add + Commit"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 128
duration_ms = 1000
[modes.mappings.action]
type = "Sequence"
actions = [
    { Shell = { command = "git add -A" } },
    { Shell = { command = "git commit -m 'Quick commit'" } }
]

[[modes.mappings]]
description = "A Button (Hold 2s): Commit + Push"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 128
duration_ms = 2000
[modes.mappings.action]
type = "Sequence"
actions = [
    { Shell = { command = "git add -A" } },
    { Shell = { command = "git commit -m 'Auto commit'" } },
    { Shell = { command = "git push" } }
]
```

**Result**: One button for git status (tap), commit (hold 1s), commit+push (hold 2s).

---

## Complete VS Code + Git Setup

Full gamepad integration for Visual Studio Code development.

### Hardware
- **Recommended**: PlayStation DualSense or Xbox Series X controller
- **Alternative**: Any gamepad or MIDI controller

### Configuration

```toml
[device]
name = "VS Code Development"
auto_connect = true

[advanced_settings]
hold_threshold_ms = 800
double_tap_timeout_ms = 300

[[modes]]
name = "VS Code"
color = "blue"

# ========== Git Operations ==========
# Face Buttons: Git Workflow
[[modes.mappings]]
description = "Cross/A: Git Status"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Shell"
command = "git status"

[[modes.mappings]]
description = "Circle/B: Git Add + Commit"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Sequence"
actions = [
    { Shell = { command = "git add -A" } },
    { Shell = { command = "git commit -m 'Update: $(date)'" } }
]

[[modes.mappings]]
description = "Square/X: Git Pull"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Shell"
command = "git pull --rebase"

[[modes.mappings]]
description = "Triangle/Y: Git Push"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Shell"
command = "git push"

# ========== Build & Test ==========
# Shoulders: Build and Test
[[modes.mappings]]
description = "L1/LB: Build Project"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Shell"
command = "npm run build"  # or cargo build, etc.

[[modes.mappings]]
description = "R1/RB: Run Tests"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137
[modes.mappings.action]
type = "Shell"
command = "npm test"

# Triggers: Advanced Build
[[modes.mappings]]
description = "L2/LT: Clean Build"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 132
threshold = 64
[modes.mappings.action]
type = "Sequence"
actions = [
    { Shell = { command = "rm -rf dist node_modules" } },
    { Shell = { command = "npm install" } },
    { Shell = { command = "npm run build" } }
]

[[modes.mappings]]
description = "R2/RT: Deploy"
[modes.mappings.trigger]
type = "GamepadTrigger"
trigger = 133
threshold = 64
[modes.mappings.action]
type = "Shell"
command = "npm run deploy"

# ========== Navigation ==========
# D-Pad: Workspace & Window Management
[[modes.mappings]]
description = "D-Pad Up: Mission Control"
[modes.mappings.trigger]
type = "GamepadButton"
button = 132
[modes.mappings.action]
type = "Keystroke"
keys = "F3"

[[modes.mappings]]
description = "D-Pad Down: Show Desktop"
[modes.mappings.trigger]
type = "GamepadButton"
button = 133
[modes.mappings.action]
type = "Keystroke"
keys = "F11"

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

# ========== App Launching ==========
# Button Chords: Launch Apps
[[modes.mappings]]
description = "L1+R1: Launch Terminal"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [136, 137]
timeout_ms = 50
[modes.mappings.action]
type = "Launch"
application = "Terminal"

[[modes.mappings]]
description = "L2+R2: Launch VS Code"
[modes.mappings.trigger]
type = "GamepadButtonChord"
buttons = [138, 139]
timeout_ms = 50
[modes.mappings.action]
type = "Launch"
application = "Visual Studio Code"
```

---

## Docker & Container Management

```toml
[[modes]]
name = "Docker"
color = "cyan"

# Start/Stop Containers
[[modes.mappings]]
description = "Pad 1: Docker Compose Up"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Shell"
command = "docker-compose up -d"

[[modes.mappings]]
description = "Pad 2: Docker Compose Down"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Shell"
command = "docker-compose down"

# Container Logs
[[modes.mappings]]
description = "Pad 3: View Logs"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Shell"
command = "docker-compose logs -f"

# Cleanup
[[modes.mappings]]
description = "Pad 4 (Hold): Prune System"
[modes.mappings.trigger]
type = "GamepadButtonHold"
button = 131
duration_ms = 2000
[modes.mappings.action]
type = "Shell"
command = "docker system prune -af"
```

---

## Rust Development (Cargo)

```toml
[[modes]]
name = "Rust Dev"
color = "orange"

# Build
[[modes.mappings]]
description = "LB: Cargo Build"
[modes.mappings.trigger]
type = "GamepadButton"
button = 136
[modes.mappings.action]
type = "Shell"
command = "cargo build"

# Test
[[modes.mappings]]
description = "RB: Cargo Test"
[modes.mappings.trigger]
type = "GamepadButton"
button = 137
[modes.mappings.action]
type = "Shell"
command = "cargo test"

# Run
[[modes.mappings]]
description = "A Button: Cargo Run"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Shell"
command = "cargo run"

# Clippy
[[modes.mappings]]
description = "B Button: Cargo Clippy"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Shell"
command = "cargo clippy"

# Format
[[modes.mappings]]
description = "X Button: Cargo Format"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Shell"
command = "cargo fmt"
```

---

## Python Development

```toml
[[modes]]
name = "Python Dev"
color = "yellow"

# Virtual Environment
[[modes.mappings]]
description = "Pad 1: Activate venv"
[modes.mappings.trigger]
type = "GamepadButton"
button = 128
[modes.mappings.action]
type = "Shell"
command = "source venv/bin/activate"

# Run Tests
[[modes.mappings]]
description = "Pad 2: Run Pytest"
[modes.mappings.trigger]
type = "GamepadButton"
button = 129
[modes.mappings.action]
type = "Shell"
command = "pytest"

# Linting
[[modes.mappings]]
description = "Pad 3: Run Flake8"
[modes.mappings.trigger]
type = "GamepadButton"
button = 130
[modes.mappings.action]
type = "Shell"
command = "flake8 ."

# Format
[[modes.mappings]]
description = "Pad 4: Black Format"
[modes.mappings.trigger]
type = "GamepadButton"
button = 131
[modes.mappings.action]
type = "Shell"
command = "black ."
```

---

## Troubleshooting

### Shell Commands Not Executing
- **Problem**: Shell actions don't run or show errors
- **Solution**: Use absolute paths or ensure working directory is correct
- **Example**: `cd /path/to/project && git status` instead of just `git status`

### Environment Variables Not Available
- **Problem**: Shell commands can't find tools (npm, cargo, etc.)
- **Solution**: Source your shell profile in command: `source ~/.zshrc && npm run build`

### Permissions Issues
- **Problem**: "Permission denied" errors
- **Solution**: Ensure scripts are executable: `chmod +x script.sh`

---

## Next Steps

- **[Explore Streaming Examples](streaming.md)** - OBS and streaming platform control
- **[See Automation Examples](automation.md)** - Advanced productivity workflows
- **[Learn Shell Actions](../configuration/actions.md#shell)** - Shell command configuration
- **[Join Developer Community](../resources/community.md)** - Share your dev setup
