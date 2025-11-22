# Context-Aware Mappings

Make your MIDI controller adapt to your workflow by executing different actions based on time, active application, current mode, or day of week.

---

## Overview

Context-aware mappings use **conditional actions** to change behavior dynamically. Instead of always doing the same thing, your controller can:

- Switch profiles based on work hours vs evening
- Route commands to the frontmost application
- Execute different actions on weekdays vs weekends
- Adapt to the current mode

This eliminates the need to manually switch profiles throughout the day.

---

## Basic Conditional Structure

```toml
[[modes.mappings]]
description = "Context-aware action"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.action]
type = "Conditional"
condition = { /* condition goes here */ }
then_action = { /* action if condition is true */ }
else_action = { /* optional action if condition is false */ }
```

---

## Condition Types

### 1. Always
**Always executes the then_action**

```toml
[modes.mappings.action]
type = "Conditional"
condition = "Always"
then_action = { type = "Keystroke", keys = "space", modifiers = [] }
```

**Use Case**: Testing, default behavior, or when you just want the `then_action` wrapper.

---

### 2. Never
**Never executes the then_action (effectively disables the mapping)**

```toml
[modes.mappings.action]
type = "Conditional"
condition = "Never"
then_action = { type = "Launch", app = "Disabled App" }
```

**Use Case**: Temporarily disable a mapping without deleting it.

---

### 3. TimeRange
**Executes only during specific hours (24-hour format)**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "TimeRange"
start = "09:00"
end = "17:00"

[modes.mappings.action.then_action]
type = "Launch"
app = "Slack"

[modes.mappings.action.else_action]
type = "Text"
text = "Outside work hours"
```

**Features**:
- Automatically handles ranges crossing midnight (e.g., `22:00` to `06:00`)
- Time is checked when action is triggered (not when config is loaded)

**Use Cases**:
- Work mode (9am-5pm): Launch productivity apps
- Evening mode (5pm-11pm): Launch entertainment apps
- Night mode (11pm-9am): Disable noisy actions

---

### 4. DayOfWeek
**Executes only on specific days**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "DayOfWeek"
days = [1, 2, 3, 4, 5]  # Monday=1 through Sunday=7

[modes.mappings.action.then_action]
type = "Shell"
command = "open ~/Work"

[modes.mappings.action.else_action]
type = "Shell"
command = "open ~/Personal"
```

**Day Numbers**:
- 1 = Monday
- 2 = Tuesday
- 3 = Wednesday
- 4 = Thursday
- 5 = Friday
- 6 = Saturday
- 7 = Sunday

**Use Cases**:
- Weekday-only shortcuts (work apps)
- Weekend-only shortcuts (gaming, hobbies)
- Different behaviors for different days

---

### 5. AppRunning
**Checks if a specific application is currently running**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "AppRunning"
app_name = "Logic Pro"

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "space"
modifiers = []  # Play/pause in Logic

[modes.mappings.action.else_action]
type = "Launch"
app = "Logic Pro"
```

**Platform Support**:
- ✅ macOS: Uses `pgrep` (case-insensitive)
- ✅ Linux: Uses `pgrep` (case-insensitive)
- ❌ Windows: Not yet supported

**Use Cases**:
- Smart play/pause (launch DAW if not running, play/pause if running)
- Toggle between apps (if browser running, open it; else launch IDE)
- Conditional workflows based on running processes

**Note**: Uses partial string matching. `"Chrome"` matches `"Google Chrome Helper"`.

---

### 6. AppFrontmost
**Checks if a specific application has focus (active window)**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "AppFrontmost"
app_name = "Safari"

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]  # New tab in Safari

[modes.mappings.action.else_action]
type = "Text"
text = "Not in Safari"
```

**Platform Support**:
- ✅ macOS: Uses NSWorkspace API
- ❌ Linux: Not yet supported
- ❌ Windows: Not yet supported

**Use Cases**:
- App-specific shortcuts (browser shortcuts vs IDE shortcuts)
- Context-switching workflows
- Smart key remapping based on frontmost app

**Note**: Checks the actual frontmost application, not just if it's running.

---

### 7. ModeIs
**Checks if the current mode matches**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "ModeIs"
mode = "Development"

[modes.mappings.action.then_action]
type = "Shell"
command = "git status"

[modes.mappings.action.else_action]
type = "Text"
text = "Switch to Development mode first"
```

**Use Cases**:
- Mode-specific behaviors within global mappings
- Validate mode before executing sensitive commands
- Different actions for different modes on same pad

**Note**: Mode name must match exactly (case-sensitive).

---

### 8. And (Logical AND)
**All sub-conditions must be true**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] },
  { type = "AppRunning", app_name = "Slack" }
]

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "s"
modifiers = ["cmd", "shift"]  # Search in Slack during work hours on weekdays
```

**Use Cases**:
- Work mode: weekdays AND business hours AND specific app
- Complex conditions requiring multiple criteria

---

### 9. Or (Logical OR)
**At least one sub-condition must be true**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "Or"
conditions = [
  { type = "AppFrontmost", app_name = "Safari" },
  { type = "AppFrontmost", app_name = "Chrome" },
  { type = "AppFrontmost", app_name = "Firefox" }
]

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "t"
modifiers = ["cmd"]  # New tab in any browser
```

**Use Cases**:
- Action applies to multiple apps
- Alternative conditions (weekend OR evening)

---

### 10. Not (Logical NOT)
**Inverts the result of a condition**

```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "Not"
condition = { type = "AppRunning", app_name = "Music" }

[modes.mappings.action.then_action]
type = "Launch"
app = "Music"

[modes.mappings.action.else_action]
type = "Text"
text = "Music already running"
```

**Use Cases**:
- "If NOT running, launch"
- "If NOT work hours, disable"
- Invert any condition logic

---

## GUI Configuration

The Conditional Action Editor in the GUI provides:

1. **Condition Type Selector**: Dropdown with descriptions
2. **Time Pickers**: Visual time selection for TimeRange
3. **Day Toggles**: Click days for DayOfWeek conditions
4. **Text Inputs**: For app names (AppRunning, AppFrontmost)
5. **Mode Dropdown**: Auto-populated from available modes (ModeIs)
6. **Logical Operators**: Add/remove sub-conditions for And/Or
   - Simple (non-nested) And/Or/Not supported
   - Complex nested logic requires TOML editing
7. **Then/Else Actions**: Full ActionSelector for both branches
8. **Optional Else**: Toggle to add/remove else_action

**To Configure**:
1. Select action type = "Conditional"
2. Choose condition type
3. Fill in condition parameters
4. Configure then_action
5. (Optional) Enable else_action and configure
6. Save

---

## Practical Examples

### Example 1: Work Hours Profile
Different behavior during work hours vs personal time:

```toml
[[modes.mappings]]
description = "Smart launcher - Slack at work, Discord after hours"

[modes.mappings.trigger]
type = "Note"
note = 8

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "And"
conditions = [
  { type = "TimeRange", start = "09:00", end = "17:00" },
  { type = "DayOfWeek", days = [1, 2, 3, 4, 5] }
]

[modes.mappings.action.then_action]
type = "Launch"
app = "Slack"

[modes.mappings.action.else_action]
type = "Launch"
app = "Discord"
```

---

### Example 2: App-Aware Play/Pause
Different shortcuts for different media apps:

```toml
[[modes.mappings]]
description = "Universal play/pause"

[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "Or"
conditions = [
  { type = "AppFrontmost", app_name = "Spotify" },
  { type = "AppFrontmost", app_name = "Music" }
]

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "space"
modifiers = []

[modes.mappings.action.else_action]
type = "Conditional"
condition = { type = "AppFrontmost", app_name = "Logic Pro" }
then_action = { type = "Keystroke", keys = "Return", modifiers = [] }
else_action = { type = "VolumeControl", operation = "Mute" }
```

Nested conditionals: Spotify/Music → space, Logic → Return, else → Mute

---

### Example 3: Weekend Gaming Mode
Enable gaming shortcuts only on weekends:

```toml
[[modes.mappings]]
description = "Discord overlay (weekends only)"

[modes.mappings.trigger]
type = "Note"
note = 15

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "DayOfWeek"
days = [6, 7]  # Saturday, Sunday

[modes.mappings.action.then_action]
type = "Keystroke"
keys = "`"
modifiers = ["shift", "cmd"]

# No else_action (does nothing on weekdays)
```

---

### Example 4: Smart DAW Control
Launch DAW if not running, otherwise send MIDI:

```toml
[[modes.mappings]]
description = "Launch or control Logic Pro"

[modes.mappings.trigger]
type = "Note"
note = 5

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
note = 60
velocity = 100

[modes.mappings.action.else_action]
type = "Launch"
app = "Logic Pro"
```

---

## Advanced: Nested Conditions

Combine multiple logical operators for complex logic:

```toml
[[modes.mappings]]
description = "Complex work mode: (weekday AND work hours) OR (weekend AND app running)"

[modes.mappings.trigger]
type = "Note"
note = 10

[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "Or"
conditions = [
  {
    type = "And"
    conditions = [
      { type = "DayOfWeek", days = [1, 2, 3, 4, 5] },
      { type = "TimeRange", start = "09:00", end = "17:00" }
    ]
  },
  {
    type = "And"
    conditions = [
      { type = "DayOfWeek", days = [6, 7] },
      { type = "AppRunning", app_name = "Xcode" }
    ]
  }
]

[modes.mappings.action.then_action]
type = "Shell"
command = "open ~/Code"
```

**Translation**: Open code folder if:
- (Monday-Friday AND 9am-5pm) OR
- (Saturday-Sunday AND Xcode is running)

---

## Tips & Best Practices

### Start Simple
Begin with single conditions:
1. Test TimeRange alone
2. Test AppRunning alone
3. Combine with And/Or once individual conditions work

### Use Descriptive Mappings
```toml
description = "Slack during work (Mon-Fri 9-5), Discord otherwise"
```
Clear descriptions help when debugging.

### Test Edge Cases
- Midnight crossing (TimeRange: 22:00 - 02:00)
- Day transitions (Saturday → Sunday at midnight)
- App name variations ("Chrome" vs "Google Chrome")

### Combine with Velocity Mappings
```toml
[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7

[modes.mappings.action]
type = "Conditional"
# Velocity mapping applies before condition evaluation
```

### Global vs Mode-Specific
- **Global mappings**: Use ModeIs condition to adapt to current mode
- **Mode mappings**: Already scoped to mode, use other conditions

### Performance
Conditions are evaluated each time the trigger fires. Keep deeply nested conditions reasonable (<5 levels deep).

---

## Troubleshooting

### Condition Never True
**Problem**: Action never executes

**Solutions**:
1. Check condition type matches intended logic
2. Verify app name matches process name (`ps aux | grep <app>`)
3. Test time format (must be `HH:MM`, 24-hour)
4. Check day numbers (Monday=1, not 0)
5. Add debug `else_action` to confirm condition is evaluating

### Wrong Action Executes
**Problem**: else_action runs instead of then_action

**Solutions**:
1. Verify condition logic (And vs Or)
2. Check app name capitalization (case-sensitive on some platforms)
3. Test each sub-condition individually
4. Add `type = "Text"` debug actions to see which path executes

### AppFrontmost Not Working
**Problem**: Condition always false

**Solutions**:
1. Verify platform support (macOS only currently)
2. Check app name matches bundle name (use Activity Monitor)
3. Try partial name ("Safari" instead of "com.apple.Safari")

---

## Related Documentation

- [Configuration: Conditionals](../configuration/conditionals.md) - Complete TOML reference
- [Tutorial: Dynamic Workflows](../tutorials/dynamic-workflows.md) - Step-by-step guide
- [Guides: Velocity Curves](velocity-curves.md) - Combine with velocity mappings
- [Configuration: Actions](../configuration/actions.md) - Action types reference

---

**Next**: Try the [Dynamic Workflows Tutorial](../tutorials/dynamic-workflows.md) for a hands-on example combining conditionals and velocity curves.
