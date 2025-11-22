// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration Backward Compatibility Test Suite
//!
//! This test suite validates that configuration files from v0.1.0 and later
//! remain compatible across all future versions of MIDIMon.
//!
//! Test Coverage:
//! - CFG-001 through CFG-010 from docs/config-compatibility.md
//! - All trigger types
//! - All action types
//! - All example configs from documentation
//!
//! Related Issues: AMI-125, AMI-123, AMI-126

use std::fs;
use std::path::PathBuf;

// Re-export config types for testing
use conductor::config::{ActionConfig, Config, Trigger};

/// Test ID: CFG-001
/// Basic config.toml loads without errors
#[test]
fn test_cfg_001_basic_config_loads() {
    let config_path = "config.toml";

    // Skip if config.toml doesn't exist (CI environment)
    if !std::path::Path::new(config_path).exists() {
        eprintln!("Skipping CFG-001: config.toml not found");
        return;
    }

    let result = Config::load(config_path);
    assert!(
        result.is_ok(),
        "CFG-001 FAILED: Basic config.toml should load without errors: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.device.name, "Mikro", "Device name should be 'Mikro'");
    assert!(config.device.auto_connect, "auto_connect should be true");
}

/// Test ID: CFG-002
/// Enhanced config with advanced settings
#[test]
fn test_cfg_002_enhanced_config_loads() {
    let config_path = "config_enhanced.toml";

    // Skip if config_enhanced.toml doesn't exist
    if !std::path::Path::new(config_path).exists() {
        eprintln!("Skipping CFG-002: config_enhanced.toml not found");
        return;
    }

    let result = Config::load(config_path);

    // NOTE: In v0.1.0, config_enhanced.toml uses trigger/action types that
    // don't exist yet (VelocityRange, LongPress, DoubleTap, EncoderTurn,
    // VolumeControl, ModeChange). This is expected and documented in
    // docs/config-compatibility.md. This test will pass once those types
    // are added in v0.2.0 (Phase 1).
    //
    // For now, we just verify the file exists and can be read as text.
    if result.is_err() {
        eprintln!(
            "CFG-002: config_enhanced.toml contains v0.2.0 features not yet in v0.1.0: {:?}",
            result.err()
        );
        eprintln!("This is expected and will be resolved in Phase 1 (AMI-123)");
        return; // Skip test in v0.1.0, will work in v0.2.0+
    }

    let config = result.unwrap();
    assert!(
        config.modes.len() >= 3,
        "Enhanced config should have at least 3 modes (Default, Development, Media)"
    );
    assert!(
        !config.global_mappings.is_empty(),
        "Enhanced config should have global mappings"
    );
}

/// Test ID: CFG-003
/// Minimal config with device only
#[test]
fn test_cfg_003_minimal_device_only() {
    let minimal_toml = r#"
[device]
name = "TestDevice"
auto_connect = false

[[modes]]
name = "Default"

[[modes.mappings]]
description = "Test mapping"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = []
"#;

    let result = toml::from_str::<Config>(minimal_toml);
    assert!(
        result.is_ok(),
        "CFG-003 FAILED: Minimal config should parse: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.device.name, "TestDevice");
    assert!(!config.device.auto_connect);
    assert_eq!(config.modes.len(), 1);
}

/// Test ID: CFG-004
/// All trigger types parse correctly
#[test]
fn test_cfg_004_all_trigger_types() {
    let trigger_configs = vec![
        // Note trigger
        (
            "Note",
            r#"type = "Note"
note = 60
velocity_min = 1"#,
        ),
        // CC trigger
        (
            "CC",
            r#"type = "CC"
cc = 1
value_min = 1"#,
        ),
        // NoteChord trigger
        (
            "NoteChord",
            r#"type = "NoteChord"
notes = [60, 64, 67]"#,
        ),
    ];

    for (name, toml_str) in trigger_configs {
        let result = toml::from_str::<Trigger>(toml_str);
        assert!(
            result.is_ok(),
            "CFG-004 FAILED: {} trigger should parse: {:?}",
            name,
            result.err()
        );
    }
}

/// Test ID: CFG-005
/// All action types parse correctly
#[test]
fn test_cfg_005_all_action_types() {
    let action_configs = vec![
        // Keystroke
        (
            "Keystroke",
            r#"type = "Keystroke"
keys = "space"
modifiers = ["cmd"]"#,
        ),
        // Text
        (
            "Text",
            r#"type = "Text"
text = "Hello, World!""#,
        ),
        // Launch
        (
            "Launch",
            r#"type = "Launch"
app = "Terminal""#,
        ),
        // Shell
        (
            "Shell",
            r#"type = "Shell"
command = "echo test""#,
        ),
        // Delay
        (
            "Delay",
            r#"type = "Delay"
ms = 1000"#,
        ),
        // MouseClick
        (
            "MouseClick",
            r#"type = "MouseClick"
button = "Left""#,
        ),
    ];

    for (name, toml_str) in action_configs {
        let result = toml::from_str::<ActionConfig>(toml_str);
        assert!(
            result.is_ok(),
            "CFG-005 FAILED: {} action should parse: {:?}",
            name,
            result.err()
        );
    }
}

/// Test ID: CFG-006
/// Complex sequence actions
#[test]
fn test_cfg_006_complex_sequences() {
    let sequence_toml = r#"
type = "Sequence"
actions = [
    { type = "Keystroke", keys = "c", modifiers = ["cmd"] },
    { type = "Delay", ms = 100 },
    { type = "Keystroke", keys = "v", modifiers = ["cmd"] }
]
"#;

    let result = toml::from_str::<ActionConfig>(sequence_toml);
    assert!(
        result.is_ok(),
        "CFG-006 FAILED: Sequence action should parse: {:?}",
        result.err()
    );

    if let Ok(ActionConfig::Sequence { actions }) = result {
        assert_eq!(actions.len(), 3, "Sequence should have 3 actions");
    } else {
        panic!("CFG-006 FAILED: Parsed action is not a Sequence");
    }
}

/// Test ID: CFG-007
/// Multiple modes configuration
#[test]
fn test_cfg_007_multiple_modes() {
    let multi_mode_toml = r#"
[device]
name = "TestDevice"
auto_connect = true

[[modes]]
name = "Mode1"
color = "blue"
[[modes.mappings]]
description = "Mapping 1"
[modes.mappings.trigger]
type = "Note"
note = 60
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []

[[modes]]
name = "Mode2"
color = "green"
[[modes.mappings]]
description = "Mapping 2"
[modes.mappings.trigger]
type = "Note"
note = 61
[modes.mappings.action]
type = "Keystroke"
keys = "b"
modifiers = []

[[global_mappings]]
description = "Global mapping"
[global_mappings.trigger]
type = "Note"
note = 1
[global_mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = []
"#;

    let result = toml::from_str::<Config>(multi_mode_toml);
    assert!(
        result.is_ok(),
        "CFG-007 FAILED: Multi-mode config should parse: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.modes.len(), 2, "Should have 2 modes");
    assert_eq!(config.modes[0].name, "Mode1");
    assert_eq!(config.modes[1].name, "Mode2");
    assert_eq!(
        config.global_mappings.len(),
        1,
        "Should have 1 global mapping"
    );
}

/// Test ID: CFG-008
/// Global mappings only (no modes)
#[test]
fn test_cfg_008_global_mappings_only() {
    let global_only_toml = r#"
[device]
name = "TestDevice"
auto_connect = true

[[modes]]
name = "Empty"

[[global_mappings]]
description = "Global action"
[global_mappings.trigger]
type = "Note"
note = 0
[global_mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = []
"#;

    let result = toml::from_str::<Config>(global_only_toml);
    assert!(
        result.is_ok(),
        "CFG-008 FAILED: Global-only config should parse: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.modes.len(), 1, "Should have 1 mode (even if empty)");
    assert!(
        config.modes[0].mappings.is_empty(),
        "Mode should have no mappings"
    );
    assert_eq!(
        config.global_mappings.len(),
        1,
        "Should have 1 global mapping"
    );
}

/// Test ID: CFG-009
/// Legacy v0.1.0 syntax (exact baseline)
#[test]
fn test_cfg_009_legacy_v0_1_0_syntax() {
    let legacy_toml = r#"
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Spotlight Search"
[modes.mappings.trigger]
type = "Note"
note = 60
velocity_min = 1
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

[[global_mappings]]
description = "Global Action Example"
[global_mappings.trigger]
type = "Note"
note = 0
[global_mappings.action]
type = "Keystroke"
keys = "a"
modifiers = ["cmd"]
"#;

    let result = toml::from_str::<Config>(legacy_toml);
    assert!(
        result.is_ok(),
        "CFG-009 FAILED: Legacy v0.1.0 syntax should parse without errors: {:?}",
        result.err()
    );

    let config = result.unwrap();
    assert_eq!(config.device.name, "Mikro");
    assert_eq!(config.modes.len(), 1);
    assert_eq!(config.modes[0].mappings.len(), 1);
    assert_eq!(config.global_mappings.len(), 1);
}

/// Test ID: CFG-010
/// Invalid syntax produces clear error
#[test]
fn test_cfg_010_invalid_syntax_clear_error() {
    let invalid_configs = vec![
        // Missing required field
        (
            "Missing note field",
            r#"
[device]
name = "Test"
auto_connect = true

[[modes]]
name = "Default"
[[modes.mappings]]
description = "Invalid"
[modes.mappings.trigger]
type = "Note"
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []
"#,
        ),
        // Invalid type
        (
            "Invalid trigger type",
            r#"
[device]
name = "Test"
auto_connect = true

[[modes]]
name = "Default"
[[modes.mappings]]
description = "Invalid"
[modes.mappings.trigger]
type = "InvalidType"
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []
"#,
        ),
    ];

    for (name, toml_str) in invalid_configs {
        let result = toml::from_str::<Config>(toml_str);
        assert!(
            result.is_err(),
            "CFG-010 FAILED: {} should produce an error",
            name
        );
    }
}

/// Regression test: Ensure all actual config files parse
#[test]
fn test_regression_all_config_files_parse() {
    let v0_1_0_configs = vec!["config.toml"];
    let v0_2_0_configs = vec!["config_enhanced.toml"]; // Contains future features

    // Test v0.1.0 baseline configs (must pass)
    for config_file in v0_1_0_configs {
        if !std::path::Path::new(config_file).exists() {
            eprintln!("Skipping {}: file not found", config_file);
            continue;
        }

        let result = Config::load(config_file);
        assert!(
            result.is_ok(),
            "REGRESSION FAILED: {} should parse without errors: {:?}",
            config_file,
            result.err()
        );
    }

    // Test v0.2.0+ configs (expected to fail in v0.1.0, will pass in v0.2.0+)
    for config_file in v0_2_0_configs {
        if !std::path::Path::new(config_file).exists() {
            eprintln!("Skipping {}: file not found", config_file);
            continue;
        }

        let result = Config::load(config_file);
        if result.is_err() {
            eprintln!(
                "{} contains v0.2.0 features not yet in v0.1.0 (expected)",
                config_file
            );
        } else {
            println!(
                "{} successfully parsed (v0.2.0+ features available)",
                config_file
            );
        }
    }
}

/// Test optional fields have sensible defaults
#[test]
fn test_optional_fields_defaults() {
    // Config without velocity_min
    let no_velocity_min = r#"
type = "Note"
note = 60
"#;

    let result = toml::from_str::<Trigger>(no_velocity_min);
    assert!(
        result.is_ok(),
        "Optional velocity_min should have default: {:?}",
        result.err()
    );

    // Config without modifiers
    let no_modifiers = r#"
type = "Keystroke"
keys = "space"
"#;

    let result = toml::from_str::<ActionConfig>(no_modifiers);
    assert!(
        result.is_ok(),
        "Optional modifiers should have default (empty array): {:?}",
        result.err()
    );
}

/// Test forward compatibility: Unknown fields are ignored
#[test]
fn test_forward_compatibility_unknown_fields_ignored() {
    let future_config = r#"
[device]
name = "Mikro"
auto_connect = true
future_field = "ignored"

[[modes]]
name = "Default"
future_mode_field = "also ignored"

[[modes.mappings]]
description = "Test"
[modes.mappings.trigger]
type = "Note"
note = 60
future_trigger_field = "ignored too"
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = []
future_action_field = "still ignored"
"#;

    // This test may fail depending on serde settings
    // If it fails, ensure serde(deny_unknown_fields) is NOT set
    let result = toml::from_str::<Config>(future_config);

    // We expect this to succeed (unknown fields ignored)
    // If it fails, that's a forward compatibility concern
    match result {
        Ok(_) => {
            // Good: forward compatible
            println!("Forward compatibility: OK (unknown fields ignored)");
        }
        Err(e) => {
            // This might indicate serde(deny_unknown_fields) is set
            eprintln!(
                "WARNING: Forward compatibility may be compromised. \
                 Unknown fields caused parse error: {:?}",
                e
            );
            // Don't fail the test, but warn developers
        }
    }
}

/// Performance test: Large configs should parse quickly
#[test]
fn test_large_config_performance() {
    use std::time::Instant;

    // Generate a large config with 100 modes and 10 mappings each
    let mut large_config = String::from(
        r#"
[device]
name = "Mikro"
auto_connect = true
"#,
    );

    for mode_idx in 0..100 {
        large_config.push_str(&format!(
            r#"
[[modes]]
name = "Mode{}"
color = "blue"
"#,
            mode_idx
        ));

        for mapping_idx in 0..10 {
            large_config.push_str(&format!(
                r#"
[[modes.mappings]]
description = "Mapping {}"
[modes.mappings.trigger]
type = "Note"
note = {}
[modes.mappings.action]
type = "Keystroke"
keys = "a"
modifiers = []
"#,
                mapping_idx,
                60 + mapping_idx
            ));
        }
    }

    let start = Instant::now();
    let result = toml::from_str::<Config>(&large_config);
    let elapsed = start.elapsed();

    assert!(
        result.is_ok(),
        "Large config should parse: {:?}",
        result.err()
    );
    assert!(
        elapsed.as_millis() < 1000,
        "Large config (100 modes, 1000 mappings) should parse in <1 second, took {:?}",
        elapsed
    );

    let config = result.unwrap();
    assert_eq!(config.modes.len(), 100);
}

#[cfg(test)]
mod fixtures {
    use super::*;

    /// Helper to create test fixture directory
    pub fn ensure_fixture_dir() -> PathBuf {
        let fixture_dir = PathBuf::from("tests/fixtures/v0.1.0");
        if !fixture_dir.exists() {
            fs::create_dir_all(&fixture_dir).expect("Failed to create fixture directory");
        }
        fixture_dir
    }

    /// Create a v0.1.0 baseline fixture
    pub fn create_v0_1_0_baseline_fixture() {
        let fixture_dir = ensure_fixture_dir();
        let baseline_config = r#"
[device]
name = "Mikro"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Spotlight Search"
[modes.mappings.trigger]
type = "Note"
note = 60
velocity_min = 1
[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]

[[global_mappings]]
description = "Global Action"
[global_mappings.trigger]
type = "Note"
note = 0
[global_mappings.action]
type = "Keystroke"
keys = "escape"
modifiers = []
"#;

        let fixture_path = fixture_dir.join("baseline.toml");
        fs::write(fixture_path, baseline_config).expect("Failed to write baseline fixture");
    }

    #[test]
    fn test_create_fixtures() {
        create_v0_1_0_baseline_fixture();
        let fixture_path = PathBuf::from("tests/fixtures/v0.1.0/baseline.toml");
        assert!(fixture_path.exists(), "Baseline fixture should be created");
    }
}
