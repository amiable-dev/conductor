# Conditional Action Configuration

Complete TOML reference for conditional action configuration.

---

## Overview

Conditional actions allow mappings to execute different actions based on runtime conditions such as time of day, active application, current mode, or day of week. This enables context-aware behavior without manual profile switching.

---

## Configuration Structure

Conditional actions use the `Conditional` action type with three components:

1. **condition**: The condition to evaluate
2. **then_action**: Action to execute if condition is true
3. **else_action**: Optional action to execute if condition is false

```toml
[[modes.mappings]]
description = "Context-aware mapping"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.action]
type = "Conditional"
condition = { /* condition definition */ }
then_action = { /* action if true */ }
else_action = { /* optional action if false */ }
```

---

## Condition Types

### Always

Always evaluates to true (always executes `then_action`).

**Fields**: None (just the string `"Always"`)

**Example**:
```toml
[modes.mappings.action]
type = "Conditional"
condition = "Always"
then_action = { type = "Keystroke", keys = "space", modifiers = [] }
```

**Use Cases**:
- Testing conditional logic
- Default behavior wrapper
- Placeholder for future condition

---

### Never

Always evaluates to false (never executes `then_action`, always uses `else_action` if present).

**Fields**: None (just the string `"Never"`)

**Example**:
```toml
[modes.mappings.action]
type = "Conditional"
condition = "Never"
then_action = { type = "Launch", app = "Disabled App" }
else_action = { type = "Text", text = "This action is disabled" }
```

**Use Cases**:
- Temporarily disable a mapping without deleting it
- Testing else_action logic
- Documented disabled features

---

### TimeRange

Evaluates to true if current time falls within specified range (24-hour format).

**Fields**:
- `type` (string, required): Must be `"TimeRange"`
- `start` (string, required): Start time in `"HH:MM"` format (24-hour)
- `end` (string, required): End time in `"HH:MM"` format (24-hour)

**Behavior**:
- Automatically handles ranges that cross midnight (e.g., `22:00` to `06:00`)
- Time is evaluated when the action is triggered (not when config is loaded)
- Timezone is system local time

**Example**:
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

**Use Cases**:
- Work mode (9am-5pm): Launch productivity apps
- Evening mode (5pm-11pm): Launch entertainment apps
- Night mode (11pm-9am): Disable noisy actions

**Validation**:
- Format must be `HH:MM` (e.g., `"09:00"`, `"23:30"`)
- Hour must be 00-23
- Minute must be 00-59
- Invalid format will cause config load error

---

### DayOfWeek

Evaluates to true if current day matches one of the specified days.

**Fields**:
- `type` (string, required): Must be `"DayOfWeek"`
- `days` (array of integers, required): Day numbers (1=Monday through 7=Sunday)

**Day Numbers**:
- 1 = Monday
- 2 = Tuesday
- 3 = Wednesday
- 4 = Thursday
- 5 = Friday
- 6 = Saturday
- 7 = Sunday

**Example**:
```toml
[modes.mappings.action]
type = "Conditional"

[modes.mappings.action.condition]
type = "DayOfWeek"
days = [1, 2, 3, 4, 5]  # Monday through Friday

[modes.mappings.action.then_action]
type = "Shell"
command = "open ~/Work"

[modes.mappings.action.else_action]
type = "Shell"
command = "open ~/Personal"
```

**Use Cases**:
- Weekday-only shortcuts (work apps)
- Weekend-only shortcuts (gaming, hobbies)
- Different behaviors for different days

**Validation**:
- `days` array must not be empty
- Each day must be 1-7
- Invalid day number will cause config load error

---

### AppRunning

Evaluates to true if a specific application is currently running (process detection).

**Fields**:
- `type` (string, required): Must be `"AppRunning"`
- `app_name` (string, required): Application name to check

**Platform Support**:
- ✅ macOS: Uses `pgrep` (case-insensitive partial match)
- ✅ Linux: Uses `pgrep` (case-insensitive partial match)
- ❌ Windows: Not yet supported

**Matching Behavior**:
- Uses partial string matching
- Case-insensitive
- `"Chrome"` matches `"Google Chrome Helper"`
- `"Logic"` matches `"Logic Pro"`, `"Logic Pro Helper"`, etc.

**Example**:
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

**Use Cases**:
- Smart play/pause (launch DAW if not running, else control it)
- Toggle between apps
- Conditional workflows based on running processes

**Performance**:
- Executes `pgrep` subprocess on each trigger
- Minimal overhead (<10ms typically)
- Cached briefly by system

---

### AppFrontmost

Evaluates to true if a specific application has focus (active window).

**Fields**:
- `type` (string, required): Must be `"AppFrontmost"`
- `app_name` (string, required): Application name to check

**Platform Support**:
- ✅ macOS: Uses NSWorkspace API (exact match)
- ❌ Linux: Not yet supported
- ❌ Windows: Not yet supported

**Matching Behavior**:
- Exact frontmost application name match
- Case-sensitive on macOS
- Checks the actual active window, not just if app is running

**Example**:
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

**Use Cases**:
- App-specific shortcuts (browser shortcuts vs IDE shortcuts)
- Context-switching workflows
- Smart key remapping based on frontmost app

**Performance**:
- Native API call (very fast, <1ms)
- No subprocess overhead

---

### ModeIs

Evaluates to true if current mode matches the specified mode name.

**Fields**:
- `type` (string, required): Must be `"ModeIs"`
- `mode` (string, required): Mode name to check (exact match, case-sensitive)

**Example**:
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

**Note**: Mode name must match exactly (case-sensitive). Use mode names as defined in your `[[modes]]` configuration.

---

### And (Logical AND)

Evaluates to true if **all** sub-conditions are true.

**Fields**:
- `type` (string, required): Must be `"And"`
- `conditions` (array, required): Array of condition objects

**Behavior**:
- Short-circuits (stops evaluating as soon as one condition is false)
- Empty array always evaluates to true

**Example**:
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
modifiers = ["cmd", "shift"]  # Search in Slack during work hours
```

**Use Cases**:
- Work mode: weekdays AND business hours AND specific app
- Complex conditions requiring multiple criteria

**Validation**:
- `conditions` array must not be empty
- Each element must be a valid condition

---

### Or (Logical OR)

Evaluates to true if **at least one** sub-condition is true.

**Fields**:
- `type` (string, required): Must be `"Or"`
- `conditions` (array, required): Array of condition objects

**Behavior**:
- Short-circuits (stops evaluating as soon as one condition is true)
- Empty array always evaluates to false

**Example**:
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

**Validation**:
- `conditions` array must not be empty
- Each element must be a valid condition

---

### Not (Logical NOT)

Inverts the result of a condition (true becomes false, false becomes true).

**Fields**:
- `type` (string, required): Must be `"Not"`
- `condition` (object, required): Condition to invert

**Example**:
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

**Validation**:
- `condition` must be a valid condition object

---

## Nested Conditions

Conditions can be nested to arbitrary depth using And/Or/Not operators.

### Example: Complex Work Mode

```toml
[[modes.mappings]]
description = "(Weekday AND work hours) OR (Weekend AND Xcode running)"

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

## Then/Else Actions

### Then Action (Required)

Executed when condition evaluates to true. Can be any action type.

**Example**:
```toml
[modes.mappings.action.then_action]
type = "Keystroke"
keys = "space"
modifiers = []
```

### Else Action (Optional)

Executed when condition evaluates to false. If omitted, no action is taken when condition is false.

**Example**:
```toml
[modes.mappings.action.else_action]
type = "Text"
text = "Condition was false"
```

### Nested Conditionals

Both then_action and else_action can themselves be Conditional actions:

```toml
[modes.mappings.action]
type = "Conditional"
condition = { type = "AppFrontmost", app_name = "Spotify" }

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

**Translation**:
- If Spotify is frontmost: Press space
- Else if Logic Pro is frontmost: Press Return
- Else: Mute volume

---

## Validation Rules

**General**:
- `type` field is required for all conditions (except Always/Never which can be just strings)
- Invalid condition type will cause config load error

**TimeRange**:
- `start` and `end` must be in `HH:MM` format
- Hour must be 00-23, minute must be 00-59

**DayOfWeek**:
- `days` array must contain at least one day
- Each day must be 1-7

**AppRunning/AppFrontmost**:
- `app_name` must be a non-empty string

**ModeIs**:
- `mode` must be a non-empty string
- Mode name is not validated against actual modes (runtime check)

**And/Or**:
- `conditions` array must contain at least one condition
- Each element must be a valid condition

**Not**:
- `condition` must be a valid condition object

---

## Performance Considerations

- **TimeRange/DayOfWeek**: Very fast (system time lookup)
- **ModeIs**: Very fast (string comparison)
- **AppRunning**: Moderate (~10ms, subprocess call to `pgrep`)
- **AppFrontmost**: Very fast (<1ms, native API)
- **And/Or**: Short-circuit evaluation (stops as soon as result is known)
- **Nested conditions**: Evaluated depth-first

**Recommendation**: Keep deeply nested conditions reasonable (<5 levels) to maintain readability and predictable performance.

---

## Examples

### Example 1: Work Hours Profile

Different behavior during work vs personal time:

```toml
[[modes.mappings]]
description = "Slack at work, Discord after hours"

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

### Example 2: Universal Play/Pause

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

---

### Example 3: Smart DAW Control

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

## Related Documentation

- [Guide: Context-Aware Mappings](../guides/context-aware.md) - User guide with tutorials
- [Configuration: Actions](actions.md) - Action types reference
- [Guide: Velocity Curves](../guides/velocity-curves.md) - Combine with velocity mappings

---

**See Also**: The GUI provides a Conditional Action Editor with visual condition type selector, time pickers, day toggles, and support for simple And/Or/Not operators. Complex nested logic can be configured via TOML editing.
