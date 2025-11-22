# Customizing Velocity Response

Learn how to control the relationship between how hard you hit a pad and what action is triggered using velocity mappings.

---

## Overview

Velocity mappings allow you to transform the input velocity (how hard you press a pad) into a different output velocity. This is useful for:

- **Normalizing velocity**: Make soft and hard hits more consistent
- **Amplifying dynamics**: Make subtle differences more pronounced
- **Creating custom response curves**: Match your playing style

---

## Velocity Mapping Types

Conductor supports four velocity mapping types:

### 1. Fixed
**Output is always the same velocity, regardless of input**

```toml
[velocity_mapping]
type = "Fixed"
velocity = 100  # Always outputs velocity 100 (0-127)
```

**Use Cases**:
- Trigger actions that don't need velocity variation
- Ensure consistent behavior regardless of how hard you hit
- Simplify testing and debugging

**Example**: Launch an application with a single tap (any velocity)

---

### 2. PassThrough
**Output velocity = input velocity (1:1 mapping)**

```toml
velocity_mapping = "PassThrough"
```

**Use Cases**:
- Natural, unmodified velocity response
- When you want direct MIDI pass-through
- Default behavior when no mapping specified

**Example**: Control DAW volume with natural dynamics

---

### 3. Linear
**Maps full input range (0-127) to custom output range**

```toml
[velocity_mapping]
type = "Linear"
min = 40   # Minimum output velocity
max = 110  # Maximum output velocity
```

**Behavior**:
- Input 0 → Output `min`
- Input 127 → Output `max`
- Values between are scaled proportionally

**Use Cases**:
- Compress dynamic range (e.g., 0-127 → 60-100)
- Expand subtle playing into wider range (e.g., 0-127 → 20-127)
- Shift velocity range up or down

**Example**: Make soft hits louder while preventing excessively loud hits
```toml
min = 50   # Softest hit = 50 (instead of 0)
max = 110  # Hardest hit = 110 (instead of 127)
```

---

### 4. Curve
**Applies non-linear transformation with adjustable intensity**

```toml
[velocity_mapping]
type = "Curve"
curve_type = "Exponential"  # or "Logarithmic" or "SCurve"
intensity = 0.7             # 0.0 = linear, 1.0 = maximum effect
```

#### Curve Types

**Exponential** (`output = input^(1-intensity)`):
- Makes soft hits louder while preserving hard hits
- Higher intensity = more compression of soft notes
- Great for making subtle playing more audible

**Example**:
```toml
curve_type = "Exponential"
intensity = 0.8  # Boost soft hits significantly
```

**Logarithmic** (`output = log(1 + intensity × input)`):
- Compresses dynamic range
- Makes loud hits quieter relative to soft hits
- Useful for taming aggressive playing

**Example**:
```toml
curve_type = "Logarithmic"
intensity = 0.6  # Moderate compression
```

**S-Curve** (Sigmoid function):
- Smooth acceleration in middle range
- Soft and hard extremes less affected
- Natural-feeling response with "sweet spot"

**Example**:
```toml
curve_type = "SCurve"
intensity = 0.5  # Balanced S-curve
```

---

## GUI Configuration

The GUI provides a visual velocity curve editor with real-time preview:

1. **Open a mapping** in the Mappings view
2. **Locate Velocity Mapping section**
3. **Select mapping type** from dropdown:
   - Fixed
   - Pass-Through
   - Linear
   - Curve
4. **Adjust parameters**:
   - Fixed: Set velocity (0-127)
   - Linear: Set min/max range
   - Curve: Choose curve type and intensity (0.0-1.0)
5. **Preview curve** in the graph:
   - X-axis: Input velocity (0-127)
   - Y-axis: Output velocity (0-127)
   - Diagonal line: 1:1 reference (Pass-Through)
   - Colored line: Your configured curve

The preview updates in real-time as you adjust parameters, allowing you to dial in the perfect response.

---

## Practical Examples

### Example 1: Consistent Application Launch
Make any tap launch an app, regardless of velocity:

```toml
[[modes.mappings]]
description = "Launch browser (any velocity)"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.velocity_mapping]
type = "Fixed"
velocity = 100  # Doesn't matter for Launch action

[modes.mappings.action]
type = "Launch"
app = "Safari"
```

---

### Example 2: Gentle Volume Control
Compress volume adjustments for smoother control:

```toml
[[modes.mappings]]
description = "Gentle volume up"

[modes.mappings.trigger]
type = "Note"
note = 10

[modes.mappings.velocity_mapping]
type = "Linear"
min = 60   # Even softest tap has impact
max = 100  # Prevent ear-splitting jumps

[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

---

### Example 3: Expressive MIDI Control
Boost soft playing for more expressive DAW control:

```toml
[[modes.mappings]]
description = "Expressive note trigger"

[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7  # Make soft notes more audible

[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "NoteOn"
channel = 0
note = 60
# Velocity is passed from the mapped input velocity
```

---

### Example 4: S-Curve for Natural Feel
Create a responsive curve with smooth acceleration:

```toml
[[modes.mappings]]
description = "Natural dynamics"

[modes.mappings.trigger]
type = "Note"
note = 3

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "SCurve"
intensity = 0.5  # Balanced response

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "CC"
channel = 0
controller = 7  # Volume CC
# CC value derived from mapped velocity
```

---

## Tips & Best Practices

### Finding Your Curve
1. **Start with PassThrough** and test your natural playing
2. **Identify issues**:
   - Soft hits not registering? → Try Exponential curve
   - Too much variation? → Try Linear compression
   - Need smoothing? → Try S-Curve
3. **Adjust intensity gradually** (0.3 → 0.5 → 0.7)
4. **Use the preview graph** to visualize the transformation

### Per-Action Tuning
Different actions may need different curves:
- **Launch/Shell**: Fixed (velocity doesn't matter)
- **Volume Control**: Linear compression (0-127 → 60-100)
- **MIDI Output**: Curve (Exponential for expression, Logarithmic for drums)
- **Keystroke**: Usually PassThrough or Fixed

### Testing
After configuring a velocity mapping:
1. **Save your config** (hot-reload applies changes)
2. **Test range**: Hit the pad softly, medium, hard
3. **Check preview graph**: Does curve match your intent?
4. **Iterate**: Adjust intensity or type as needed

---

## Advanced: Velocity-Sensitive Actions

Combine velocity mappings with velocity-sensitive triggers:

```toml
[[modes.mappings]]
description = "Soft hit = text, hard hit = keystroke"

[modes.mappings.trigger]
type = "VelocityRange"
note = 7
soft_max = 40
medium_max = 80

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.6  # Emphasize differences

[modes.mappings.action]
type = "VelocityRange"
soft_action = { type = "Text", text = "Hello" }
medium_action = { type = "Keystroke", keys = "space", modifiers = [] }
hard_action = { type = "Launch", app = "Terminal" }
```

The velocity mapping transforms the input before the VelocityRange thresholds are applied.

---

## Mathematical Details

For those interested in the underlying transformations:

**Exponential**:
```
output = input^(1 - intensity)
```
- `intensity = 0` → linear (no change)
- `intensity = 1` → maximum compression (all inputs → 127)

**Logarithmic**:
```
normalized_input = input / 127
output = log(1 + intensity × normalized_input) / log(1 + intensity) × 127
```
- `intensity = 0` → linear
- `intensity = 1` → maximum compression

**S-Curve** (Sigmoid):
```
normalized_input = input / 127
k = intensity × 10 + 0.5  # Steepness factor
sigmoid = 1 / (1 + e^(-k × (normalized_input - 0.5)))
output = normalize(sigmoid) × 127
```
- Lower `intensity` → gentler curve
- Higher `intensity` → sharper transition in midrange

---

## Related Documentation

- [Configuration: Velocity Mappings](../configuration/curves.md) - Complete TOML reference
- [Context-Aware Mappings](context-aware.md) - Combine with conditionals
- [Configuration: Actions](../configuration/actions.md) - Action types reference
- [Guides: Per-App Profiles](per-app-profiles.md) - Different curves per application

---

**Next**: Learn about [Context-Aware Mappings](context-aware.md) to make your velocity curves change based on time, app, or mode.
