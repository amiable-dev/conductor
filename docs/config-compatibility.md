# Configuration Backward Compatibility Strategy

**Document Version**: 1.0
**Last Updated**: 2025-11-11
**Status**: Active
**Related Issues**: AMI-125, AMI-123, AMI-126

## Overview

This document defines the backward compatibility strategy for MIDIMon's `config.toml` file format across all versions. The goal is to ensure that configuration files created for v0.1.0 (the monolithic implementation) remain fully functional through v0.2.0 (Phase 1 modularization) and v1.0.0 (Phase 2 monorepo migration).

**Core Principle**: Configuration files are the user's primary interface to MIDIMon. Breaking changes to config syntax require major version bumps and must provide automated migration tools.

---

## Compatibility Matrix

### Configuration Sections Stability Guarantees

| Section | v0.1.0 | v0.2.0 (Phase 1) | v1.0.0 (Phase 2) | Stability Level | Notes |
|---------|--------|------------------|------------------|-----------------|-------|
| `[device]` | ✅ | ✅ | ✅ | **Stable** | Core device identification |
| `[device.name]` | ✅ | ✅ | ✅ | **Stable** | Device name for connection |
| `[device.auto_connect]` | ✅ | ✅ | ✅ | **Stable** | Auto-connection behavior |
| `[[modes]]` | ✅ | ✅ | ✅ | **Stable** | Mode definitions |
| `[[modes.mappings]]` | ✅ | ✅ | ✅ | **Stable** | Mode-specific mappings |
| `[[global_mappings]]` | ✅ | ✅ | ✅ | **Stable** | Cross-mode mappings |
| `[advanced_settings]` | ⚠️  | ✅ | ✅ | **Optional** | Added in config_enhanced.toml, optional in v0.1.0 |
| `[advanced_settings.chord_timeout_ms]` | ⚠️  | ✅ | ✅ | **Optional** | Default: 100ms if not specified |
| `[advanced_settings.double_tap_timeout_ms]` | ⚠️  | ✅ | ✅ | **Optional** | Default: 300ms if not specified |
| `[advanced_settings.hold_threshold_ms]` | ⚠️  | ✅ | ✅ | **Optional** | Default: 2000ms if not specified |

**Legend**:
- ✅ **Fully Supported**: Section exists and works identically
- ⚠️  **Optional**: Section is optional and has sensible defaults
- ⛔ **Deprecated**: Section is deprecated but still functional
- 🚫 **Removed**: Section no longer supported (requires migration)

---

## Trigger Types Compatibility

### v0.1.0 Trigger Types (Stable)

All trigger types from v0.1.0 are **guaranteed stable** through v1.0.0:

| Trigger Type | v0.1.0 | v0.2.0 | v1.0.0 | Breaking Changes | Migration Required |
|--------------|--------|--------|--------|------------------|-------------------|
| `Note` | ✅ | ✅ | ✅ | None | No |
| `CC` | ✅ | ✅ | ✅ | None | No |
| `NoteChord` | ✅ | ✅ | ✅ | None | No |
| `VelocityRange` | ⚠️  | ✅ | ✅ | Syntax extension in v0.2.0 | Optional |
| `LongPress` | ⚠️  | ✅ | ✅ | Added in config_enhanced.toml | Optional |
| `DoubleTap` | ⚠️  | ✅ | ✅ | Added in config_enhanced.toml | Optional |
| `EncoderTurn` | ⚠️  | ✅ | ✅ | Added in config_enhanced.toml | Optional |

**Notes**:
- Trigger types marked ⚠️  were introduced in `config_enhanced.toml` but are backward compatible (existing configs without them continue to work)
- All v0.1.0 trigger syntax remains valid in v0.2.0 and v1.0.0
- New optional fields added to existing triggers will have sensible defaults

---

## Action Types Compatibility

### v0.1.0 Action Types (Stable)

All action types from v0.1.0 are **guaranteed stable** through v1.0.0:

| Action Type | v0.1.0 | v0.2.0 | v1.0.0 | Breaking Changes | Migration Required |
|-------------|--------|--------|--------|------------------|-------------------|
| `Keystroke` | ✅ | ✅ | ✅ | None | No |
| `Text` | ✅ | ✅ | ✅ | None | No |
| `Launch` | ✅ | ✅ | ✅ | None | No |
| `Shell` | ✅ | ✅ | ✅ | None | No |
| `Sequence` | ✅ | ✅ | ✅ | None | No |
| `Delay` | ✅ | ✅ | ✅ | None | No |
| `MouseClick` | ✅ | ✅ | ✅ | None | No |
| `VolumeControl` | ⚠️  | ✅ | ✅ | Added in config_enhanced.toml | Optional |
| `ModeChange` | ⚠️  | ✅ | ✅ | Added in config_enhanced.toml | Optional |
| `Repeat` | ⚠️  | ✅ | ✅ | Added in examples.md | Optional |
| `Conditional` | ⚠️  | ✅ | ✅ | Added in examples.md | Optional |

**Notes**:
- All v0.1.0 action syntax remains valid
- New action types added in documentation are backward compatible
- Actions may gain optional fields with defaults

---

## Compatibility Guarantees by Version

### v0.1.0 (Monolithic Implementation)

**Status**: Current stable baseline
**Config Format Version**: `1.0`
**Guarantee**: All configs created for v0.1.0 will work in v0.2.0 and v1.0.0

**Supported Sections**:
```toml
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"
[[modes.mappings]]
# Mappings with Note, CC, NoteChord triggers

[[global_mappings]]
# Global mappings
```

**Features**:
- Basic trigger types: `Note`, `CC`, `NoteChord`
- Basic action types: `Keystroke`, `Text`, `Launch`, `Shell`, `Sequence`, `Delay`, `MouseClick`
- Mode system with color support
- Global mappings

---

### v0.2.0 (Phase 1 - Modularization)

**Status**: Planned (Q1 2025)
**Config Format Version**: `1.1` (backward compatible with `1.0`)
**Guarantee**: 100% backward compatible with v0.1.0 configs

**Changes**:
- ✅ **No breaking changes to existing syntax**
- ✅ All v0.1.0 configs work without modification
- ✅ New optional sections added (e.g., `[advanced_settings]`)
- ✅ New optional trigger/action types available
- ✅ Config file location may expand (e.g., `~/.config/midimon/config.toml` in addition to local `config.toml`)

**New Optional Features** (from `config_enhanced.toml` and `examples.md`):
```toml
# Optional: Advanced timing controls (uses defaults if omitted)
[advanced_settings]
chord_timeout_ms = 50
double_tap_timeout_ms = 300
hold_threshold_ms = 2000

# Optional: New trigger types
[modes.mappings.trigger]
type = "VelocityRange"
type = "LongPress"
type = "DoubleTap"
type = "EncoderTurn"

# Optional: New action types
[modes.mappings.action]
type = "VolumeControl"
type = "ModeChange"
type = "Repeat"
type = "Conditional"
```

**Migration**: None required. Existing v0.1.0 configs work as-is.

---

### v1.0.0 (Phase 2 - Monorepo Migration)

**Status**: Planned (Q2 2025)
**Config Format Version**: `2.0` (backward compatible with `1.0` and `1.1`)
**Guarantee**: 100% backward compatible with v0.1.0 and v0.2.0 configs

**Changes**:
- ✅ **No breaking changes to existing syntax**
- ✅ All v0.1.0 and v0.2.0 configs work without modification
- ✅ New optional config location: `~/.config/midimon/config.toml` (preferred)
- ✅ Local `config.toml` still supported for backward compatibility
- ✅ Hot-reload support for config changes (no restart required)
- ✅ Multi-profile support (optional, advanced users)

**New Optional Features**:
```toml
# Optional: Multi-profile support
[profiles]
default = "~/macos-profile.toml"
per_app = [
    { app = "com.apple.Logic", config = "~/logic-profile.toml" },
    { app = "com.microsoft.VSCode", config = "~/vscode-profile.toml" }
]

# Optional: Device profile overrides
[device_profile]
path = "~/.config/midimon/profiles/mikro-mk3.ncmm3"
pad_page = "H"

# Optional: Tauri UI settings (ignored by engine, used by GUI)
[ui]
auto_launch = true
show_tray_icon = true
```

**Migration**: None required. Existing configs work as-is. New features are opt-in.

---

## Deprecation Policy

### Policy Statement

MIDIMon follows semantic versioning (SemVer) for configuration compatibility:

- **Major version (x.0.0)**: May introduce breaking changes with migration tools
- **Minor version (0.x.0)**: Adds features in backward-compatible manner
- **Patch version (0.0.x)**: Bug fixes only, no config changes

### Deprecation Process

When a configuration feature must be deprecated:

1. **Deprecation Notice** (version N):
   - Feature marked as deprecated in documentation
   - Warning logged when deprecated feature is used
   - Alternative approach documented

2. **Deprecation Period** (version N+1):
   - Feature continues to work with warnings
   - Migration guide published
   - Automated migration tool provided (if feasible)

3. **Removal** (version N+2, major version bump):
   - Feature removed from codebase
   - Config parser errors with helpful migration message
   - Migration tool available as standalone utility

**Example Timeline**:
- v1.0.0: Feature X deprecated, warning added
- v1.1.0: Feature X still works, migration guide published
- v2.0.0: Feature X removed, migration tool available

### Current Deprecations

**As of v0.1.0**: No deprecated features.

**Planned for v0.2.0+**: None currently planned.

---

## Migration Guide Template

When breaking changes are introduced (major version bumps), this template will be used:

### Migration Guide: v{OLD} → v{NEW}

**Release Date**: {DATE}
**Breaking Changes**: {COUNT}
**Automated Migration**: {YES/NO}

#### Breaking Change #1: {Description}

**What changed**:
```toml
# Old syntax (v{OLD})
[old_section]
old_field = "value"

# New syntax (v{NEW})
[new_section]
new_field = "value"
```

**Why it changed**: {Rationale}

**Migration steps**:
1. Manual: Update `[old_section]` to `[new_section]`
2. Automated: Run `midimon migrate config.toml --from {OLD} --to {NEW}`

**Backward compatibility**: {NONE | DEPRECATED | SUPPORTED UNTIL v{X}}

---

## Test Strategy

### Regression Test Suite

All v0.1.0 configuration examples must pass validation in all future versions.

#### Test Cases

| Test ID | Description | Config File | Expected Behavior | v0.1.0 | v0.2.0 | v1.0.0 |
|---------|-------------|-------------|-------------------|--------|--------|--------|
| CFG-001 | Basic config.toml | `config.toml` | Loads without errors | ✅ | ✅ | ✅ |
| CFG-002 | Enhanced config | `config_enhanced.toml` | Loads with all features | ⚠️  | ✅ | ✅ |
| CFG-003 | Minimal device only | `minimal.toml` | Uses defaults for modes | ✅ | ✅ | ✅ |
| CFG-004 | All trigger types | `triggers.toml` | All triggers parsed | ⚠️  | ✅ | ✅ |
| CFG-005 | All action types | `actions.toml` | All actions compiled | ⚠️  | ✅ | ✅ |
| CFG-006 | Complex sequences | `sequences.toml` | Nested actions work | ✅ | ✅ | ✅ |
| CFG-007 | Multiple modes | `multi-mode.toml` | All modes load | ✅ | ✅ | ✅ |
| CFG-008 | Global mappings only | `global-only.toml` | Loads without modes | ✅ | ✅ | ✅ |
| CFG-009 | Legacy v0.1.0 syntax | `legacy-v0.1.0.toml` | Loads without warnings | ✅ | ✅ | ✅ |
| CFG-010 | Invalid syntax | `invalid.toml` | Clear error message | ✅ | ✅ | ✅ |

#### CI/CD Integration

Tests run on:
- Every pull request
- Every commit to `main`
- Pre-release validation
- Nightly builds

**Test Command**:
```bash
cargo test --test config_compatibility
```

**Test Coverage Requirements**:
- 100% coverage for config parsing (`config.rs`)
- 100% coverage for trigger/action validation
- All example configs in `docs-site/src/configuration/examples.md` must parse

---

## Validation Tests

### Unit Tests

Located in: `tests/config_compatibility_test.rs`

**Test Categories**:
1. **Parsing Tests**: Validate TOML syntax parsing
2. **Validation Tests**: Validate semantic correctness
3. **Migration Tests**: Validate automated migration tools
4. **Backward Compatibility Tests**: Validate old configs work
5. **Forward Compatibility Tests**: Validate new configs gracefully degrade

### Integration Tests

**End-to-End Config Loading**:
```rust
#[test]
fn test_v0_1_0_config_loads_in_v0_2_0() {
    let config = Config::load("tests/fixtures/v0.1.0/config.toml").unwrap();
    assert_eq!(config.device.name, "Mikro");
    assert!(config.device.auto_connect);
    assert_eq!(config.modes.len(), 1);
}
```

**Schema Validation**:
```rust
#[test]
fn test_all_trigger_types_valid() {
    let triggers = vec![
        r#"type = "Note"; note = 60"#,
        r#"type = "CC"; cc = 1"#,
        r#"type = "NoteChord"; notes = [60, 64, 67]"#,
        r#"type = "VelocityRange"; note = 60; min_velocity = 1; max_velocity = 127"#,
        r#"type = "LongPress"; note = 60; min_duration_ms = 1000"#,
        r#"type = "DoubleTap"; note = 60; max_interval_ms = 300"#,
        r#"type = "EncoderTurn"; cc = 1; direction = "Clockwise""#,
    ];

    for trigger_toml in triggers {
        let trigger: Trigger = toml::from_str(trigger_toml).unwrap();
        assert!(validate_trigger(&trigger).is_ok());
    }
}
```

**Example Config Tests**:
```rust
#[test]
fn test_all_docs_examples_parse() {
    let example_files = vec![
        "config.toml",
        "config_enhanced.toml",
        "docs-site/src/configuration/examples.md", // Extract TOML blocks
    ];

    for file in example_files {
        let configs = extract_toml_from_file(file);
        for (index, config_str) in configs.iter().enumerate() {
            let result = toml::from_str::<Config>(config_str);
            assert!(result.is_ok(),
                "Failed to parse example {} in {}: {:?}",
                index, file, result.err()
            );
        }
    }
}
```

---

## Future Considerations

### Phase 3+ (Beyond v1.0.0)

**Potential Features Requiring Major Version Bumps**:

1. **Dynamic Scripting Language** (v2.0.0?):
   - Embedded Lua or JavaScript for complex actions
   - Breaking: Action syntax may change significantly
   - Migration: Automated conversion for common patterns

2. **Plugin System** (v2.0.0?):
   - Third-party action/trigger extensions
   - Breaking: Config structure may need plugin declarations
   - Migration: Standard actions remain backward compatible

3. **Cloud Sync** (v1.x.0 - minor):
   - Optional cloud storage for configs
   - Non-breaking: Opt-in feature

4. **Visual Config Editor** (v1.x.0 - minor):
   - GUI-based config editing (Tauri UI)
   - Non-breaking: Generates standard TOML

### Long-Term Compatibility Commitments

- **v0.1.0 configs will work in v1.x.x indefinitely**
- Major version bumps (v2.0.0+) will provide automated migration tools
- Legacy config loaders will be maintained for at least 2 major versions
- Deprecated features will have minimum 6-month notice before removal

---

## Summary

### Key Takeaways

1. **v0.1.0 configs are sacred**: They will work in all v1.x.x versions without modification.
2. **New features are additive**: v0.2.0 and v1.0.0 add optional features without breaking existing syntax.
3. **Deprecation is rare**: Only introduced with major version bumps and 6+ month notice.
4. **Migration tools provided**: If breaking changes are required, automated migration tools will be available.
5. **Test coverage is mandatory**: All configs from documentation must pass validation tests.

### Version Compatibility Summary

```
v0.1.0 config → v0.1.0 engine ✅
v0.1.0 config → v0.2.0 engine ✅ (100% compatible)
v0.1.0 config → v1.0.0 engine ✅ (100% compatible)

v0.2.0 config → v0.1.0 engine ⚠️  (new features ignored, warnings shown)
v0.2.0 config → v0.2.0 engine ✅
v0.2.0 config → v1.0.0 engine ✅ (100% compatible)

v1.0.0 config → v0.1.0 engine ⚠️  (new features ignored, warnings shown)
v1.0.0 config → v0.2.0 engine ⚠️  (new features ignored, warnings shown)
v1.0.0 config → v1.0.0 engine ✅
```

---

## References

- **Related Issues**: AMI-125 (this document), AMI-123 (API design), AMI-126 (Phase 2 checklist)
- **Related Docs**:
  - `/docs-site/src/reference/config-schema.md` - Full config reference
  - `/docs-site/src/configuration/examples.md` - Real-world examples
  - `/CLAUDE.md` - Architecture and migration plan
  - `/config.toml` - v0.1.0 baseline config
  - `/config_enhanced.toml` - v0.2.0 preview config

- **Linear Tracker**: [AMI-125](https://linear.app/amiable/issue/AMI-125)
- **Repository**: https://github.com/amiable-dev/midimon
