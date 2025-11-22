# Your First Mapping

Learn how to create custom mappings by building a practical example step-by-step.

## What You'll Build

A simple pad mapping that opens Visual Studio Code when you press pad 1 (Note 60).

## Understanding Mapping Structure

Every mapping has two parts:

```toml
[[modes.mappings]]
trigger = { ... }  # What activates the mapping
action = { ... }   # What happens when activated
```

Think of it as: **"When [trigger], do [action]"**

## Step 1: Open Your Config File

The config file is `config.toml` in your project root:

```bash
cd /path/to/conductor
open config.toml  # macOS
# or
nano config.toml  # Terminal editor
```

## Step 2: Find the Modes Section

Scroll down to find the `[[modes]]` section. It looks like this:

```toml
[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
# Existing mappings here...
```

## Step 3: Add Your Mapping

Add this **at the end** of the `[[modes.mappings]]` section:

```toml
[[modes.mappings]]
trigger = { Note = { note = 60, velocity_min = 0 } }
action = { Launch = { app_path = "/Applications/Visual Studio Code.app" } }
```

**What this means**:
- **trigger**: When pad Note 60 is pressed (any velocity ≥0)
- **action**: Launch Visual Studio Code

## Step 4: Save and Reload

1. **Save** the file (Cmd+S or Ctrl+O in nano)
2. **Stop** Conductor if it's running (Ctrl+C)
3. **Restart** it:

```bash
cargo run --release 2
```

## Step 5: Test It

Press the **first pad** (bottom-left on Mikro MK3). Visual Studio Code should launch!

## Understanding Note Numbers

How do you know which pad is Note 60?

### Quick Reference (Maschine Mikro MK3)

```
Pad Layout (4x4 grid):
┌────┬────┬────┬────┐
│ 60 │ 61 │ 62 │ 63 │  Top row
├────┼────┼────┼────┤
│ 64 │ 65 │ 66 │ 67 │
├────┼────┼────┼────┤
│ 68 │ 69 │ 70 │ 71 │
└────┴────┴────┴────┘
   72   73   74   75    Bottom row
```

### Find Note Numbers for Any Device

Use the pad mapper tool:

```bash
cargo run --bin pad_mapper
```

Then press each pad to see its note number.

## Common Mapping Patterns

### Pattern 1: Keyboard Shortcut

```toml
[[modes.mappings]]
trigger = { Note = { note = 61, velocity_min = 0 } }
action = { Keystroke = { keys = ["Cmd", "T"] } }  # New tab
```

### Pattern 2: Type Text

```toml
[[modes.mappings]]
trigger = { Note = { note = 62, velocity_min = 0 } }
action = { Text = { text = "user@example.com" } }
```

### Pattern 3: Run Shell Command

```toml
[[modes.mappings]]
trigger = { Note = { note = 63, velocity_min = 0 } }
action = { Shell = { command = "open -a Calculator" } }
```

### Pattern 4: Volume Control

```toml
[[modes.mappings]]
trigger = { Note = { note = 64, velocity_min = 0 } }
action = { VolumeControl = "VolumeUp" }
```

## Next Steps

- [Understanding Modes](./modes.md) - Create mode-based workflows
- [Configuration Overview](../configuration/overview.md) - Full config reference
- [All Trigger Types](../reference/trigger-types.md) - Complete trigger reference
- [All Action Types](../reference/action-types.md) - Complete action reference
- [Example Configurations](../configuration/examples.md) - Pre-built configs
