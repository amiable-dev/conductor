# Tutorial: Building Dynamic Workflows

Step-by-step guide to combining velocity curves and conditional actions for powerful context-aware MIDI controller mappings.

---

## Overview

This tutorial will teach you to create sophisticated, context-aware workflows that adapt to:
- Time of day (work hours vs evening)
- Active application (DAW vs browser vs IDE)
- Day of week (weekdays vs weekends)
- Current mode
- How hard you hit the pad

By the end, you'll build a complete workflow that transforms your MIDI controller into an intelligent assistant.

---

## Prerequisites

- Conductor installed and configured
- Basic TOML editing skills (or use the GUI)
- A MIDI controller connected
- Understanding of basic triggers and actions

**Estimated Time**: 30-45 minutes

---

## Tutorial Structure

We'll build three progressively complex workflows:

1. **Beginner**: Time-based app launcher
2. **Intermediate**: Velocity-sensitive DAW control
3. **Advanced**: Multi-condition workflow with nested logic

---

## Workflow 1: Time-Based App Launcher (Beginner)

**Goal**: Launch Slack during work hours, Discord after hours.

### Step 1: Create the Basic Mapping

Open your `config.toml` and add a new global mapping:

```toml
[[global_mappings]]
description = "Smart communication launcher"

[global_mappings.trigger]
type = "Note"
note = 8  # Choose your pad number
```

### Step 2: Add the Conditional Action

Add the conditional logic:

```toml
[global_mappings.action]
type = "Conditional"

[global_mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] }
]

[global_mappings.action.then_action]
type = "Launch"
app = "Slack"

[global_mappings.action.else_action]
type = "Launch"
app = "Discord"
```

### Step 3: Test the Workflow

1. Save `config.toml`
2. Config will hot-reload automatically
3. Press your configured pad during work hours (Mon-Fri 9am-5pm)
4. Verify Slack launches
5. Press the same pad outside work hours
6. Verify Discord launches

### Understanding the Logic

```
IF (time is 9am-5pm AND day is Monday-Friday):
  Launch Slack
ELSE:
  Launch Discord
```

### GUI Configuration

If using the GUI:
1. Open Mappings view
2. Click "Add Mapping"
3. Set trigger to Note 8
4. Select action type "Conditional"
5. Select condition type "And"
6. Add two sub-conditions:
   - TimeRange: 09:00 to 17:00
   - DayOfWeek: Select Mon-Fri
7. Configure then_action: Launch → Slack
8. Configure else_action: Launch → Discord
9. Save

---

## Workflow 2: Velocity-Sensitive DAW Control (Intermediate)

**Goal**: Control Logic Pro with velocity-sensitive MIDI notes, boosting soft hits for expressive playing.

### Step 1: Create the Mapping

```toml
[[modes.mappings]]
description = "Expressive MIDI control"

[modes.mappings.trigger]
type = "Note"
note = 1
```

### Step 2: Add Velocity Curve

Make soft hits more audible:

```toml
[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7  # Strong boost for soft notes
```

### Step 3: Add Conditional DAW Control

Only send MIDI if Logic Pro is running:

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "AppRunning"
app_name = "Logic Pro"

[modes.mappings.action.then_action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60  # Middle C
# Velocity is derived from mapped input velocity

[modes.mappings.action.else_action]
type = "Sequence"
actions = [
  { type = "Launch", app = "Logic Pro" },
  { type = "Delay", ms = 2000 },
  { type = "Text", text = "Logic Pro launched, try again" }
]
```

### Step 4: Test the Workflow

1. **With Logic Pro closed**:
   - Press pad → Logic Pro launches
   - Wait 2 seconds → See notification

2. **With Logic Pro running**:
   - Press pad softly → Sends MIDI note with boosted velocity
   - Press pad hard → Sends MIDI note at full velocity
   - Notice soft notes are more audible due to exponential curve

### Understanding the Flow

```
Input Velocity → Exponential Curve (boost soft hits) → Mapped Velocity

IF Logic Pro is running:
  Send MIDI with mapped velocity to IAC Driver
ELSE:
  Launch Logic Pro
  Wait 2 seconds
  Show notification
```

### Velocity Curve Effect

| Input Velocity | Without Curve | With Exponential (0.7) |
|----------------|---------------|------------------------|
| 20 (very soft) | 20            | ~65 (much louder)      |
| 64 (medium)    | 64            | ~85 (slightly louder)  |
| 100 (hard)     | 100           | ~110 (barely affected) |

---

## Workflow 3: Multi-Condition Smart Assistant (Advanced)

**Goal**: Create a single pad that adapts to time, app, and velocity for different actions.

### The Scenario

We want pad 10 to:
- **Work hours + Browser**: Open new tab (velocity doesn't matter)
- **Work hours + DAW**: Send velocity-sensitive MIDI
- **Evening + Any app**: Launch entertainment app based on velocity
  - Soft hit: Launch Spotify
  - Hard hit: Launch Steam

### Step 1: Create the Foundation

```toml
[[global_mappings]]
description = "Adaptive smart pad"

[global_mappings.trigger]
type = "Note"
note = 10
```

### Step 2: Configure Velocity Mapping

Use S-Curve for natural dynamics:

```toml
[global_mappings.velocity_mapping]
type = "Curve"
curve_type = "SCurve"
intensity = 0.5
```

### Step 3: Build the Conditional Logic

First level: Check if work hours:

```toml
[global_mappings.action]
type = "Conditional"

[global_mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] }
]
```

### Step 4: Work Hours Branch (Then Action)

During work hours, check which app is frontmost:

```toml
[global_mappings.action.then_action]
type = "Conditional"

[global_mappings.action.then_action.condition]
type = "Or"
conditions = [
  { type = "AppFrontmost", app_name = "Safari" },
  { type = "AppFrontmost", app_name = "Chrome" },
  { type = "AppFrontmost", app_name = "Firefox" }
]

[global_mappings.action.then_action.then_action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]  # New tab in browser

[global_mappings.action.then_action.else_action]
type = "Conditional"
condition = { type = "AppRunning", app_name = "Logic Pro" }
then_action = {
  type = "SendMidi",
  port = "IAC Driver Bus 1",
  message_type = "NoteOn",
  channel = 0,
  note = 60
}
else_action = { type = "Text", text = "Work mode: Use browser or DAW" }
```

### Step 5: Evening Branch (Else Action)

Outside work hours, use velocity to choose entertainment:

```toml
[global_mappings.action.else_action]
type = "VelocityRange"
soft_max = 64
soft_action = { type = "Launch", app = "Spotify" }
hard_action = { type = "Launch", app = "Steam" }
```

### Complete Workflow

Here's the full configuration:

```toml
[[global_mappings]]
description = "Adaptive smart pad: Browser/DAW at work, entertainment after hours"

[global_mappings.trigger]
type = "Note"
note = 10

[global_mappings.velocity_mapping]
type = "Curve"
curve_type = "SCurve"
intensity = 0.5

[global_mappings.action]
type = "Conditional"

[global_mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] }
]

# Work hours branch
[global_mappings.action.then_action]
type = "Conditional"

[global_mappings.action.then_action.condition]
type = "Or"
conditions = [
  { type = "AppFrontmost", app_name = "Safari" },
  { type = "AppFrontmost", app_name = "Chrome" },
  { type = "AppFrontmost", app_name = "Firefox" }
]

[global_mappings.action.then_action.then_action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]

[global_mappings.action.then_action.else_action]
type = "Conditional"
condition = { type = "AppRunning", app_name = "Logic Pro" }
then_action = { type = "SendMidi", port = "IAC Driver Bus 1", message_type = "NoteOn", channel = 0, note = 60 }
else_action = { type = "Text", text = "Work mode: Use browser or DAW" }

# Evening branch
[global_mappings.action.else_action]
type = "VelocityRange"
soft_max = 64
soft_action = { type = "Launch", app = "Spotify" }
hard_action = { type = "Launch", app = "Steam" }
```

### Testing the Advanced Workflow

**Scenario 1**: Monday 10am, Safari frontmost
- Press pad → New tab in Safari

**Scenario 2**: Tuesday 2pm, Logic Pro running
- Press pad softly → MIDI note with moderate velocity (S-curve mapped)
- Press pad hard → MIDI note with high velocity

**Scenario 3**: Friday 8pm
- Press pad softly → Spotify launches
- Press pad hard → Steam launches

**Scenario 4**: Monday 10am, VS Code frontmost
- Press pad → "Work mode: Use browser or DAW" notification

### Understanding the Decision Tree

```
Input Velocity → S-Curve → Mapped Velocity

IF (weekday AND 9am-5pm):
  IF (browser is frontmost):
    New tab (Cmd+T)
  ELSE IF (Logic Pro running):
    Send MIDI note (mapped velocity)
  ELSE:
    Show notification
ELSE (evening/weekend):
  IF (soft hit, velocity ≤ 64):
    Launch Spotify
  ELSE (hard hit, velocity > 64):
    Launch Steam
```

---

## Best Practices

### 1. Start Simple, Build Complexity

Don't try to build the advanced workflow first. Start with:
1. Single condition
2. Add second condition with And
3. Add nested conditional
4. Add velocity mapping

### 2. Test Each Layer

After adding each condition:
1. Save config
2. Test the new behavior
3. Verify existing behavior still works
4. Add next layer

### 3. Use Descriptive Mappings

```toml
description = "Work hours: Browser tab OR DAW MIDI | Evening: Spotify/Steam"
```

Clear descriptions help when debugging.

### 4. Debug with Text Actions

If behavior is unexpected, temporarily replace actions with Text to see which path executes:

```toml
then_action = { type = "Text", text = "THEN path executed" }
else_action = { type = "Text", text = "ELSE path executed" }
```

### 5. GUI vs TOML

- **Simple conditionals**: Use GUI for visual editing
- **Nested logic**: Use TOML for precision and readability
- **Velocity curves**: Use GUI for real-time preview graph

---

## Common Patterns

### Pattern 1: App-Specific Shortcuts

```toml
condition = { type = "AppFrontmost", app_name = "MyApp" }
then_action = { type = "Keystroke", keys = "s", modifiers = ["cmd"] }
else_action = { type = "Text", text = "Not in MyApp" }
```

### Pattern 2: Time-Based Behavior

```toml
condition = { type = "TimeRange", start = "22:00", end = "08:00" }
then_action = { type = "Text", text = "Quiet hours - action disabled" }
else_action = { /* normal action */ }
```

### Pattern 3: Launch-or-Control

```toml
condition = { type = "AppRunning", app_name = "MyDAW" }
then_action = { /* control command */ }
else_action = { type = "Launch", app = "MyDAW" }
```

### Pattern 4: Velocity-Gated Actions

```toml
[velocity_mapping]
type = "Linear"
min = 50
max = 110

[action]
type = "VelocityRange"
soft_max = 70
soft_action = { /* gentle action */ }
hard_action = { /* aggressive action */ }
```

---

## Troubleshooting

### Problem: Condition Never True

**Check**:
1. Time format is `HH:MM` (24-hour)
2. App name matches exactly (check Activity Monitor on macOS)
3. Day numbers are correct (Monday=1, not 0)
4. Platform support (AppFrontmost is macOS-only)

**Debug**:
```toml
# Add debug text to both branches
then_action = { type = "Text", text = "Condition TRUE" }
else_action = { type = "Text", text = "Condition FALSE" }
```

### Problem: Wrong Action Executes

**Check**:
1. Logical operator (And vs Or)
2. Nesting structure (use proper TOML indentation)
3. Short-circuit evaluation (And stops at first false, Or at first true)

**Debug**:
- Test each sub-condition individually
- Simplify nested logic to isolate issue

### Problem: Velocity Curve Doesn't Feel Right

**Solution**:
1. Open GUI velocity curve preview
2. Adjust intensity in 0.1 increments
3. Try different curve types:
   - Exponential: Boost soft hits
   - Logarithmic: Tame hard hits
   - S-Curve: Natural feel with sweet spot

---

## Next Steps

### Expand Your Workflows

Now that you've mastered dynamic workflows, try:

1. **Per-App Profiles**: Different mappings for different apps
2. **Mode-Based Logic**: Use ModeIs condition for mode-specific behavior
3. **Sequence Actions**: Chain multiple actions with delays
4. **Advanced Velocity**: Combine curves with VelocityRange triggers

### Share Your Workflows

Export your `config.toml` and share with the community:
- GitHub discussions
- Reddit r/midicontrollers
- Discord servers

### Learn More

- [Guide: Context-Aware Mappings](../guides/context-aware.md) - Deep dive into conditionals
- [Guide: Velocity Curves](../guides/velocity-curves.md) - Master velocity mappings
- [Configuration: Actions](../configuration/actions.md) - All available actions
- [Configuration: Triggers](../configuration/triggers.md) - All available triggers

---

## Summary

You've learned to:
- ✅ Combine time and app conditions with And/Or logic
- ✅ Use velocity curves for expressive control
- ✅ Build nested conditional logic
- ✅ Create context-aware workflows
- ✅ Test and debug complex mappings
- ✅ Follow best practices for maintainable configs

**Your MIDI controller is now an intelligent, context-aware assistant that adapts to your workflow automatically.**

---

**Example Configs**: See `examples/` directory for complete workflow examples:
- `work-productivity.toml` - Time-based work/personal split
- `daw-control.toml` - Velocity-sensitive music production
- `gaming-streams.toml` - Entertainment and streaming control
