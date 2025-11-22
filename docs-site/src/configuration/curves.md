# Velocity Mapping Configuration

Complete TOML reference for velocity mapping configuration.

---

## Overview

Velocity mappings transform input velocity (0-127) into output velocity using configurable curves. This allows you to customize the relationship between how hard you hit a pad and what action is triggered.

---

## Configuration Location

Velocity mappings are configured per-mapping in the `velocity_mapping` field:

```toml
[[modes.mappings]]
description = "My mapping"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.velocity_mapping]
# Velocity mapping configuration goes here

[modes.mappings.action]
type = "SendMidi"
# ...
```

---

## Velocity Mapping Types

### Fixed

Output is always the same velocity, regardless of input.

**Fields**:
- `type` (string, required): Must be `"Fixed"`
- `velocity` (integer, required): Output velocity (0-127)

**Example**:
```toml
[velocity_mapping]
type = "Fixed"
velocity = 100
```

**Use Cases**:
- Actions that don't need velocity variation
- Ensure consistent behavior regardless of hit strength
- Testing and debugging

---

### PassThrough

Output velocity equals input velocity (1:1 mapping). This is the default if no velocity mapping is specified.

**Fields**:
- Can be specified as just the string `"PassThrough"` (no fields required)

**Example (short form)**:
```toml
velocity_mapping = "PassThrough"
```

**Example (explicit form)**:
```toml
[velocity_mapping]
type = "PassThrough"
```

**Use Cases**:
- Natural, unmodified velocity response
- Direct MIDI pass-through
- When you want no velocity transformation

---

### Linear

Maps full input range (0-127) to custom output range with linear scaling.

**Fields**:
- `type` (string, required): Must be `"Linear"`
- `min` (integer, required): Minimum output velocity (0-127)
- `max` (integer, required): Maximum output velocity (0-127)

**Behavior**:
- Input 0 → Output `min`
- Input 127 → Output `max`
- Values between are scaled proportionally

**Formula**:
```
output = min + (input / 127) × (max - min)
```

**Example**:
```toml
[velocity_mapping]
type = "Linear"
min = 40   # Softest hit = 40 (instead of 0)
max = 110  # Hardest hit = 110 (instead of 127)
```

**Use Cases**:
- Compress dynamic range (e.g., 0-127 → 60-100)
- Expand subtle playing (e.g., 0-127 → 20-127)
- Shift velocity range up or down
- Make soft hits louder while preventing loud spikes

**Parameter Constraints**:
- `min` must be ≤ `max`
- Both must be in range 0-127
- If `min` = `max`, acts like Fixed mapping

---

### Curve

Applies non-linear transformation with adjustable intensity.

**Fields**:
- `type` (string, required): Must be `"Curve"`
- `curve_type` (string, required): One of `"Exponential"`, `"Logarithmic"`, or `"SCurve"`
- `intensity` (float, required): Curve intensity (0.0 = linear, 1.0 = maximum effect)

**Example**:
```toml
[velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7
```

---

#### Exponential Curve

Makes soft hits louder while preserving hard hits.

**Formula**:
```
output = input^(1 - intensity)
```

**Behavior**:
- `intensity = 0.0`: Linear (no change)
- `intensity = 0.5`: Moderate compression of soft notes
- `intensity = 1.0`: Maximum compression (all inputs → 127)

**Effect**:
- Low input velocities are boosted significantly
- High input velocities remain relatively unchanged
- Creates "compressed" feel that makes subtle playing more audible

**Example**:
```toml
[velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.8  # Strong boost for soft hits
```

**Use Cases**:
- Make soft playing more audible
- Compensate for light touch
- Reduce dynamic range while preserving expressiveness
- MIDI output to DAWs where soft notes get lost

---

#### Logarithmic Curve

Compresses dynamic range, making loud hits quieter relative to soft hits.

**Formula**:
```
normalized_input = input / 127
output = log(1 + intensity × normalized_input) / log(1 + intensity) × 127
```

**Behavior**:
- `intensity = 0.0`: Linear (no change)
- `intensity = 0.5`: Moderate compression
- `intensity = 1.0`: Maximum compression

**Effect**:
- High input velocities are attenuated
- Low input velocities are relatively preserved
- Creates "tamed" feel that prevents aggressive playing from being too loud

**Example**:
```toml
[velocity_mapping]
type = "Curve"
curve_type = "Logarithmic"
intensity = 0.6  # Moderate taming of loud hits
```

**Use Cases**:
- Tame aggressive playing
- Prevent ear-splitting volume spikes
- Smooth out dynamic range for consistent mix
- Volume control actions that need gentle adjustment

---

#### S-Curve (Sigmoid)

Smooth acceleration in middle range with soft and hard extremes less affected.

**Formula**:
```
normalized_input = input / 127
k = intensity × 10 + 0.5
sigmoid = 1 / (1 + exp(-k × (normalized_input - 0.5)))
output = normalize(sigmoid) × 127
```

**Behavior**:
- `intensity = 0.0`: Nearly linear
- `intensity = 0.5`: Balanced S-curve with noticeable "sweet spot"
- `intensity = 1.0`: Sharp transition in midrange

**Effect**:
- Creates a "sweet spot" in the middle velocity range
- Soft and hard extremes are less affected
- Natural-feeling response with smooth acceleration
- Provides more control in the middle range while preserving extremes

**Example**:
```toml
[velocity_mapping]
type = "Curve"
curve_type = "SCurve"
intensity = 0.5  # Balanced response with sweet spot
```

**Use Cases**:
- Natural-feeling dynamics
- Emphasis on mid-range control
- Smooth transitions between soft and hard
- Expressive MIDI control with gradual acceleration

---

## Intensity Parameter Guide

For Curve mappings, the `intensity` parameter controls how pronounced the curve effect is:

| Intensity | Effect | Typical Use Case |
|-----------|--------|------------------|
| 0.0 - 0.2 | Very subtle | Fine-tuning, slight adjustment |
| 0.3 - 0.5 | Moderate | Noticeable but natural feel |
| 0.6 - 0.8 | Strong | Significant transformation |
| 0.9 - 1.0 | Extreme | Dramatic effect, testing |

**Recommendation**: Start with `0.5` and adjust in increments of 0.1 until you find the desired feel.

---

## Combining with Actions

Velocity mappings apply **before** the action is executed. The transformed velocity is passed to the action.

### SendMidi Action

```toml
[velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7

[action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "NoteOn"
channel = 0
note = 60
# Velocity is derived from the mapped input velocity
```

### VelocityRange Action

Velocity mapping transforms the input **before** VelocityRange thresholds are applied:

```toml
[velocity_mapping]
type = "Linear"
min = 30
max = 120

[action]
type = "VelocityRange"
soft_max = 50    # Applied to mapped velocity (30-120 range)
medium_max = 90
soft_action = { type = "Text", text = "Soft" }
medium_action = { type = "Text", text = "Medium" }
hard_action = { type = "Text", text = "Hard" }
```

---

## Default Behavior

If no `velocity_mapping` is specified, **PassThrough** is used by default:

```toml
[[modes.mappings]]
# No velocity_mapping specified = PassThrough
[modes.mappings.action]
type = "SendMidi"
# Velocity is passed through unchanged
```

---

## Validation Rules

**Type Validation**:
- `type` must be one of: `"Fixed"`, `"PassThrough"`, `"Linear"`, `"Curve"`
- Invalid type will cause config load error

**Fixed Validation**:
- `velocity` must be 0-127
- Missing `velocity` will cause error

**Linear Validation**:
- `min` and `max` must be 0-127
- `min` should be ≤ `max` (not enforced, but illogical if reversed)
- Missing `min` or `max` will cause error

**Curve Validation**:
- `curve_type` must be one of: `"Exponential"`, `"Logarithmic"`, `"SCurve"`
- `intensity` must be 0.0-1.0
- Missing `curve_type` or `intensity` will cause error

---

## Examples

### Example 1: Consistent Launch Action

Ignore velocity variation for application launching:

```toml
[[modes.mappings]]
description = "Launch browser (any hit strength)"

[modes.mappings.trigger]
type = "Note"
note = 5

[modes.mappings.velocity_mapping]
type = "Fixed"
velocity = 100  # Doesn't actually matter for Launch

[modes.mappings.action]
type = "Launch"
app = "Safari"
```

---

### Example 2: Gentle Volume Control

Compress volume adjustments for smoother control:

```toml
[[modes.mappings]]
description = "Volume up (gentle)"

[modes.mappings.trigger]
type = "Note"
note = 10

[modes.mappings.velocity_mapping]
type = "Linear"
min = 60   # Even soft tap has impact
max = 100  # Prevent ear-splitting jumps

[modes.mappings.action]
type = "VolumeControl"
operation = "Up"
```

---

### Example 3: Expressive MIDI Output

Boost soft playing for DAW control:

```toml
[[modes.mappings]]
description = "MIDI note with boosted soft hits"

[modes.mappings.trigger]
type = "Note"
note = 1

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "Exponential"
intensity = 0.7

[modes.mappings.action]
type = "SendMidi"
port = "IAC Driver Bus 1"
message_type = "NoteOn"
channel = 0
note = 60
velocity = 100  # This is overridden by mapped velocity
```

---

### Example 4: Natural Response Curve

S-Curve for smooth, natural feel:

```toml
[[modes.mappings]]
description = "Natural dynamics with sweet spot"

[modes.mappings.trigger]
type = "Note"
note = 3

[modes.mappings.velocity_mapping]
type = "Curve"
curve_type = "SCurve"
intensity = 0.5

[modes.mappings.action]
type = "SendMidi"
port = "Virtual MIDI Port"
message_type = "CC"
channel = 0
controller = 7  # Volume CC
value = 64      # Overridden by mapped velocity
```

---

## Related Documentation

- [Guide: Customizing Velocity Response](../guides/velocity-curves.md) - User guide with tutorials
- [Configuration: Actions](actions.md) - Action types that use velocity
- [Guide: Context-Aware Mappings](../guides/context-aware.md) - Combine with conditionals

---

**See Also**: The GUI provides a visual curve preview graph to help you dial in the perfect response curve. The graph updates in real-time as you adjust parameters.
