// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Unit tests for src/actions.rs (AMI-256)
//!
//! Comprehensive unit tests covering action parsing, construction,
//! and execution logic to achieve 80%+ code coverage.

use conductor::actions::{Action, KeyCode, ModifierKey, MouseButton};
use conductor::config::ActionConfig;
use conductor_daemon::ActionExecutor;

// ============================================================================
// Action Construction Tests
// ============================================================================

#[test]
fn test_action_from_keystroke_config_simple() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, modifiers } => {
            assert_eq!(keys.len(), 1);
            assert_eq!(modifiers.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_with_single_modifier() {
    let config = ActionConfig::Keystroke {
        keys: "c".to_string(),
        modifiers: vec!["cmd".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, modifiers } => {
            assert_eq!(keys.len(), 1);
            assert_eq!(modifiers.len(), 1);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_with_multiple_modifiers() {
    let config = ActionConfig::Keystroke {
        keys: "v".to_string(),
        modifiers: vec!["cmd".to_string(), "shift".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, modifiers } => {
            assert_eq!(keys.len(), 1);
            assert_eq!(modifiers.len(), 2);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_multiple_keys() {
    let config = ActionConfig::Keystroke {
        keys: "a+b+c".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, modifiers } => {
            assert_eq!(keys.len(), 3, "Should parse 3 keys from 'a+b+c'");
            assert_eq!(modifiers.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_special_keys() {
    let test_cases = vec![
        ("space", 1),
        ("return", 1),
        ("enter", 1),
        ("tab", 1),
        ("escape", 1),
        ("esc", 1),
        ("backspace", 1),
        ("delete", 1),
        ("del", 1),
        ("up", 1),
        ("down", 1),
        ("left", 1),
        ("right", 1),
        ("home", 1),
        ("end", 1),
        ("pageup", 1),
        ("pagedown", 1),
        ("f1", 1),
        ("f5", 1),
        ("f12", 1),
    ];

    for (key_str, expected_count) in test_cases {
        let config = ActionConfig::Keystroke {
            keys: key_str.to_string(),
            modifiers: vec![],
        };

        let action: Action = config.into();

        match action {
            Action::Keystroke { keys, .. } => {
                assert_eq!(keys.len(), expected_count, "Failed for key: {}", key_str);
            }
            _ => panic!("Expected Keystroke action for key: {}", key_str),
        }
    }
}

#[test]
fn test_action_from_keystroke_config_invalid_key() {
    let config = ActionConfig::Keystroke {
        keys: "invalidkey123".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // Invalid keys should be filtered out, resulting in empty vec
            assert_eq!(keys.len(), 0, "Invalid keys should not parse");
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_modifier_variants() {
    let modifier_variants = vec![
        vec!["cmd"],
        vec!["command"],
        vec!["meta"],
        vec!["ctrl"],
        vec!["control"],
        vec!["alt"],
        vec!["option"],
        vec!["shift"],
    ];

    for modifiers_vec in modifier_variants {
        let config = ActionConfig::Keystroke {
            keys: "a".to_string(),
            modifiers: modifiers_vec.iter().map(|s| s.to_string()).collect(),
        };

        let action: Action = config.into();

        match action {
            Action::Keystroke { modifiers, .. } => {
                assert_eq!(
                    modifiers.len(),
                    1,
                    "Failed for modifiers: {:?}",
                    modifiers_vec
                );
            }
            _ => panic!("Expected Keystroke action"),
        }
    }
}

#[test]
fn test_action_from_keystroke_config_invalid_modifier() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec!["invalid_mod".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            assert_eq!(
                modifiers.len(),
                0,
                "Invalid modifiers should be filtered out"
            );
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_keystroke_config_mixed_case() {
    let config = ActionConfig::Keystroke {
        keys: "Space+RETURN+Tab".to_string(),
        modifiers: vec!["CMD".to_string(), "SHIFT".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, modifiers } => {
            assert_eq!(keys.len(), 3, "Should handle mixed case keys");
            assert_eq!(modifiers.len(), 2, "Should handle mixed case modifiers");
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_action_from_text_config() {
    let text = "Hello, World!".to_string();
    let config = ActionConfig::Text { text: text.clone() };

    let action: Action = config.into();

    match action {
        Action::Text(t) => {
            assert_eq!(t, text);
        }
        _ => panic!("Expected Text action"),
    }
}

#[test]
fn test_action_from_text_config_empty() {
    let config = ActionConfig::Text {
        text: "".to_string(),
    };

    let action: Action = config.into();

    match action {
        Action::Text(t) => {
            assert_eq!(t, "");
        }
        _ => panic!("Expected Text action"),
    }
}

#[test]
fn test_action_from_text_config_special_chars() {
    let text = "Test\nWith\tSpecial\r\nChars!@#$%^&*()".to_string();
    let config = ActionConfig::Text { text: text.clone() };

    let action: Action = config.into();

    match action {
        Action::Text(t) => {
            assert_eq!(t, text);
        }
        _ => panic!("Expected Text action"),
    }
}

#[test]
fn test_action_from_launch_config() {
    let app = "Calculator".to_string();
    let config = ActionConfig::Launch { app: app.clone() };

    let action: Action = config.into();

    match action {
        Action::Launch(a) => {
            assert_eq!(a, app);
        }
        _ => panic!("Expected Launch action"),
    }
}

#[test]
fn test_action_from_launch_config_with_path() {
    let app = "/Applications/Calculator.app".to_string();
    let config = ActionConfig::Launch { app: app.clone() };

    let action: Action = config.into();

    match action {
        Action::Launch(a) => {
            assert_eq!(a, app);
        }
        _ => panic!("Expected Launch action"),
    }
}

#[test]
fn test_action_from_shell_config() {
    let command = "echo 'Hello'".to_string();
    let config = ActionConfig::Shell {
        command: command.clone(),
    };

    let action: Action = config.into();

    match action {
        Action::Shell(cmd) => {
            assert_eq!(cmd, command);
        }
        _ => panic!("Expected Shell action"),
    }
}

#[test]
fn test_action_from_shell_config_complex() {
    let command = "ls -la | grep test && echo done".to_string();
    let config = ActionConfig::Shell {
        command: command.clone(),
    };

    let action: Action = config.into();

    match action {
        Action::Shell(cmd) => {
            assert_eq!(cmd, command);
        }
        _ => panic!("Expected Shell action"),
    }
}

#[test]
fn test_action_from_sequence_config_empty() {
    let config = ActionConfig::Sequence { actions: vec![] };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 0);
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_action_from_sequence_config_single() {
    let config = ActionConfig::Sequence {
        actions: vec![ActionConfig::Text {
            text: "test".to_string(),
        }],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 1);
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_action_from_sequence_config_multiple() {
    let config = ActionConfig::Sequence {
        actions: vec![
            ActionConfig::Text {
                text: "Hello".to_string(),
            },
            ActionConfig::Delay { ms: 100 },
            ActionConfig::Text {
                text: "World".to_string(),
            },
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 3);

            // Verify types
            assert!(matches!(actions[0], Action::Text(_)));
            assert!(matches!(actions[1], Action::Delay(_)));
            assert!(matches!(actions[2], Action::Text(_)));
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_action_from_sequence_config_nested() {
    let config = ActionConfig::Sequence {
        actions: vec![
            ActionConfig::Text {
                text: "outer".to_string(),
            },
            ActionConfig::Sequence {
                actions: vec![
                    ActionConfig::Text {
                        text: "inner1".to_string(),
                    },
                    ActionConfig::Text {
                        text: "inner2".to_string(),
                    },
                ],
            },
            ActionConfig::Text {
                text: "final".to_string(),
            },
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 3);

            // Check nested sequence
            if let Action::Sequence(inner) = &actions[1] {
                assert_eq!(inner.len(), 2);
            } else {
                panic!("Expected nested Sequence action");
            }
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_action_from_delay_config() {
    let config = ActionConfig::Delay { ms: 500 };

    let action: Action = config.into();

    match action {
        Action::Delay(ms) => {
            assert_eq!(ms, 500);
        }
        _ => panic!("Expected Delay action"),
    }
}

#[test]
fn test_action_from_delay_config_zero() {
    let config = ActionConfig::Delay { ms: 0 };

    let action: Action = config.into();

    match action {
        Action::Delay(ms) => {
            assert_eq!(ms, 0);
        }
        _ => panic!("Expected Delay action"),
    }
}

#[test]
fn test_action_from_delay_config_large() {
    let config = ActionConfig::Delay { ms: 60000 };

    let action: Action = config.into();

    match action {
        Action::Delay(ms) => {
            assert_eq!(ms, 60000);
        }
        _ => panic!("Expected Delay action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_left() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: None,
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { button, x, y } => {
            assert!(matches!(button, MouseButton::Left));
            assert_eq!(x, None);
            assert_eq!(y, None);
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_right() {
    let config = ActionConfig::MouseClick {
        button: "right".to_string(),
        x: None,
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { button, .. } => {
            assert!(matches!(button, MouseButton::Right));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_middle() {
    let config = ActionConfig::MouseClick {
        button: "middle".to_string(),
        x: None,
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { button, .. } => {
            assert!(matches!(button, MouseButton::Middle));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_with_position() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: Some(100),
        y: Some(200),
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, Some(100));
            assert_eq!(y, Some(200));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_invalid_button() {
    let config = ActionConfig::MouseClick {
        button: "invalid".to_string(),
        x: None,
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { button, .. } => {
            // Invalid button should default to Left
            assert!(matches!(button, MouseButton::Left));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_action_from_mouse_click_config_negative_coords() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: Some(-50),
        y: Some(-100),
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, Some(-50));
            assert_eq!(y, Some(-100));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

// ============================================================================
// Action Clone Tests
// ============================================================================

#[test]
fn test_action_clone_keystroke() {
    let action = Action::Keystroke {
        keys: vec![KeyCode::Unicode('a')],
        modifiers: vec![ModifierKey::Command],
    };

    let cloned = action.clone();

    match (action, cloned) {
        (
            Action::Keystroke {
                keys: k1,
                modifiers: m1,
            },
            Action::Keystroke {
                keys: k2,
                modifiers: m2,
            },
        ) => {
            assert_eq!(k1.len(), k2.len());
            assert_eq!(m1.len(), m2.len());
        }
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_action_clone_text() {
    let action = Action::Text("test".to_string());
    let cloned = action.clone();

    match (action, cloned) {
        (Action::Text(t1), Action::Text(t2)) => {
            assert_eq!(t1, t2);
        }
        _ => panic!("Clone failed"),
    }
}

#[test]
fn test_action_clone_sequence() {
    let action = Action::Sequence(vec![Action::Text("test".to_string()), Action::Delay(100)]);

    let cloned = action.clone();

    match (action, cloned) {
        (Action::Sequence(a1), Action::Sequence(a2)) => {
            assert_eq!(a1.len(), a2.len());
        }
        _ => panic!("Clone failed"),
    }
}

// ============================================================================
// Debug Format Tests
// ============================================================================

#[test]
fn test_action_debug_format_keystroke() {
    let action = Action::Keystroke {
        keys: vec![KeyCode::Unicode('a')],
        modifiers: vec![],
    };

    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Keystroke"));
}

#[test]
fn test_action_debug_format_text() {
    let action = Action::Text("test".to_string());
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Text"));
    assert!(debug_str.contains("test"));
}

#[test]
fn test_action_debug_format_launch() {
    let action = Action::Launch("Calculator".to_string());
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Launch"));
    assert!(debug_str.contains("Calculator"));
}

#[test]
fn test_action_debug_format_shell() {
    let action = Action::Shell("echo test".to_string());
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Shell"));
}

#[test]
fn test_action_debug_format_sequence() {
    let action = Action::Sequence(vec![Action::Text("test".to_string())]);
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Sequence"));
}

#[test]
fn test_action_debug_format_delay() {
    let action = Action::Delay(500);
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("Delay"));
    assert!(debug_str.contains("500"));
}

#[test]
fn test_action_debug_format_mouse_click() {
    let action = Action::MouseClick {
        button: MouseButton::Left,
        x: Some(100),
        y: Some(200),
    };
    let debug_str = format!("{:?}", action);
    assert!(debug_str.contains("MouseClick"));
}

// ============================================================================
// ActionConfig Clone Tests
// ============================================================================

#[test]
fn test_action_config_clone() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec!["cmd".to_string()],
    };

    let cloned = config.clone();

    match (config, cloned) {
        (
            ActionConfig::Keystroke {
                keys: k1,
                modifiers: m1,
            },
            ActionConfig::Keystroke {
                keys: k2,
                modifiers: m2,
            },
        ) => {
            assert_eq!(k1, k2);
            assert_eq!(m1, m2);
        }
        _ => panic!("Clone failed"),
    }
}

// ============================================================================
// Key Parsing Edge Cases
// ============================================================================

#[test]
fn test_parse_keys_with_spaces() {
    let config = ActionConfig::Keystroke {
        keys: "a + b + c".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // Should trim spaces and parse all keys
            assert_eq!(keys.len(), 3);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_keys_empty_string() {
    let config = ActionConfig::Keystroke {
        keys: "".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            assert_eq!(keys.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_keys_only_separators() {
    let config = ActionConfig::Keystroke {
        keys: "+++".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // Empty strings after split should be filtered out
            assert_eq!(keys.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_keys_with_unicode() {
    let config = ActionConfig::Keystroke {
        keys: "😀".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // Multi-byte characters should not parse as single char keys
            assert_eq!(keys.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

// ============================================================================
// Modifier Parsing Edge Cases
// ============================================================================

#[test]
fn test_parse_modifiers_empty_list() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            assert_eq!(modifiers.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_modifiers_duplicate() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec!["cmd".to_string(), "cmd".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            // Both should parse (no deduplication)
            assert_eq!(modifiers.len(), 2);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_modifiers_mixed_valid_invalid() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec![
            "cmd".to_string(),
            "invalid".to_string(),
            "shift".to_string(),
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            // Only valid modifiers should parse
            assert_eq!(modifiers.len(), 2);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

// ============================================================================
// Mouse Button Parsing Tests
// ============================================================================

#[test]
fn test_parse_mouse_button_case_insensitive() {
    let buttons = vec![
        ("LEFT", MouseButton::Left),
        ("Right", MouseButton::Right),
        ("MIDDLE", MouseButton::Middle),
        ("MiDdLe", MouseButton::Middle),
    ];

    for (button_str, expected) in buttons {
        let config = ActionConfig::MouseClick {
            button: button_str.to_string(),
            x: None,
            y: None,
        };

        let action: Action = config.into();

        match action {
            Action::MouseClick { button, .. } => {
                assert!(
                    matches!(
                        (button, expected),
                        (MouseButton::Left, MouseButton::Left)
                            | (MouseButton::Right, MouseButton::Right)
                            | (MouseButton::Middle, MouseButton::Middle)
                    ),
                    "Failed for button: {}",
                    button_str
                );
            }
            _ => panic!("Expected MouseClick action"),
        }
    }
}

#[test]
fn test_parse_mouse_button_empty_string() {
    let config = ActionConfig::MouseClick {
        button: "".to_string(),
        x: None,
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { button, .. } => {
            // Should default to Left
            assert!(matches!(button, MouseButton::Left));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

// ============================================================================
// Complex Sequence Tests
// ============================================================================

#[test]
fn test_deeply_nested_sequences() {
    let config = ActionConfig::Sequence {
        actions: vec![ActionConfig::Sequence {
            actions: vec![ActionConfig::Sequence {
                actions: vec![ActionConfig::Text {
                    text: "deep".to_string(),
                }],
            }],
        }],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(outer) => {
            assert_eq!(outer.len(), 1);
            if let Action::Sequence(middle) = &outer[0] {
                assert_eq!(middle.len(), 1);
                if let Action::Sequence(inner) = &middle[0] {
                    assert_eq!(inner.len(), 1);
                }
            }
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_sequence_with_all_action_types() {
    let config = ActionConfig::Sequence {
        actions: vec![
            ActionConfig::Keystroke {
                keys: "a".to_string(),
                modifiers: vec![],
            },
            ActionConfig::Text {
                text: "test".to_string(),
            },
            ActionConfig::Launch {
                app: "Calculator".to_string(),
            },
            ActionConfig::Shell {
                command: "echo test".to_string(),
            },
            ActionConfig::Delay { ms: 100 },
            ActionConfig::MouseClick {
                button: "left".to_string(),
                x: None,
                y: None,
            },
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 6);
            assert!(matches!(actions[0], Action::Keystroke { .. }));
            assert!(matches!(actions[1], Action::Text(_)));
            assert!(matches!(actions[2], Action::Launch(_)));
            assert!(matches!(actions[3], Action::Shell(_)));
            assert!(matches!(actions[4], Action::Delay(_)));
            assert!(matches!(actions[5], Action::MouseClick { .. }));
        }
        _ => panic!("Expected Sequence action"),
    }
}

// ============================================================================
// F-key Tests
// ============================================================================

#[test]
fn test_all_function_keys() {
    for i in 1..=12 {
        let key_str = format!("f{}", i);
        let config = ActionConfig::Keystroke {
            keys: key_str.clone(),
            modifiers: vec![],
        };

        let action: Action = config.into();

        match action {
            Action::Keystroke { keys, .. } => {
                assert_eq!(keys.len(), 1, "Failed for key: {}", key_str);
            }
            _ => panic!("Expected Keystroke action for {}", key_str),
        }
    }
}

#[test]
fn test_extended_function_keys() {
    // Our domain model now supports F1-F20 (extended range)
    let config = ActionConfig::Keystroke {
        keys: "f13".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // f13 is now valid in our domain model
            assert_eq!(keys.len(), 1);
            assert!(matches!(keys[0], KeyCode::F13));
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_invalid_function_key() {
    let config = ActionConfig::Keystroke {
        keys: "f99".to_string(), // Truly invalid
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            // f99 should not parse as a valid key
            assert_eq!(keys.len(), 0);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

// ============================================================================
// Arrow Key Tests
// ============================================================================

#[test]
fn test_arrow_keys() {
    let arrows = vec!["up", "down", "left", "right"];

    for arrow in arrows {
        let config = ActionConfig::Keystroke {
            keys: arrow.to_string(),
            modifiers: vec![],
        };

        let action: Action = config.into();

        match action {
            Action::Keystroke { keys, .. } => {
                assert_eq!(keys.len(), 1, "Failed for arrow: {}", arrow);
            }
            _ => panic!("Expected Keystroke action for {}", arrow),
        }
    }
}

// ============================================================================
// ActionExecutor Tests (Structure only - no actual execution)
// ============================================================================

#[test]
#[cfg_attr(target_os = "linux", ignore = "Requires display server")]
fn test_action_executor_new() {
    // Test that ActionExecutor can be created
    // Note: We can't test actual execution without mocking enigo
    // Skipped on Linux CI (no display server)
    use conductor_daemon::ActionExecutor;

    let executor = ActionExecutor::new();

    // If we got here without panicking, construction succeeded
    drop(executor);
}

// NOTE: The following tests exercise execution paths for coverage purposes
// They have side effects but are designed to be minimally invasive:
// - Text actions type a single space (safe)
// - Launch actions attempt to open /dev/null (safe no-op on Unix)
// - Shell commands are no-ops (echo to /dev/null, true command)
// - Delays are very short (1ms)
// - Mouse clicks are at coordinate (0,0) without movement
// - Keystrokes simulate pressing a non-printable key (Escape)

#[test]
#[ignore] // Ignored by default - run with --ignored to get full coverage
fn test_execute_text_action_safe() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    let action = Action::Text(" ".to_string()); // Single space - minimal side effect

    // This will actually type a space, but that's relatively safe
    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_delay_action() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    let action = Action::Delay(1); // 1ms delay - no side effects

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_sequence_action() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    let action = Action::Sequence(vec![Action::Delay(1), Action::Delay(1)]);

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
#[cfg(unix)]
fn test_execute_launch_action_safe() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // /dev/null is safe to "open" on Unix systems - it's a no-op
    let action = Action::Launch("/dev/null".to_string());

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
#[cfg(unix)]
fn test_execute_shell_action_safe() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // "true" is a safe no-op command that always succeeds
    let action = Action::Shell("true".to_string());

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_keystroke_action_safe() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // Escape key is safe - it just dismisses dialogs/menus if any are open
    let action = Action::Keystroke {
        keys: vec![KeyCode::Escape],
        modifiers: vec![],
    };

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_keystroke_with_modifiers_safe() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // Tab with no modifiers is relatively safe
    let action = Action::Keystroke {
        keys: vec![KeyCode::Tab],
        modifiers: vec![],
    };

    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_mouse_click_without_position() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // Click at current position (no move) - minimal side effect
    let action = Action::MouseClick {
        button: MouseButton::Left,
        x: None,
        y: None,
    };

    // WARNING: This will click wherever the mouse currently is
    // Only run in controlled test environment
    executor.execute(action, None);
}

#[test]
#[ignore] // Ignored by default
fn test_execute_mouse_click_with_position() {
    use conductor::actions::Action;

    let mut executor = ActionExecutor::new();
    // Click at screen corner (least likely to hit something)
    let action = Action::MouseClick {
        button: MouseButton::Left,
        x: Some(0),
        y: Some(0),
    };

    // WARNING: This will click at (0, 0)
    // Only run in controlled test environment
    executor.execute(action, None);
}

// ============================================================================
// Additional Edge Cases and Validation Tests
// ============================================================================

#[test]
fn test_keystroke_with_numbers() {
    let config = ActionConfig::Keystroke {
        keys: "0+1+2+3+4+5+6+7+8+9".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            assert_eq!(keys.len(), 10, "Should parse all numeric keys");
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_keystroke_with_special_characters() {
    let chars = vec![
        "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "=", "[", "]", "\\", ";", "'", ",",
        ".", "/",
    ];

    for ch in chars {
        let config = ActionConfig::Keystroke {
            keys: ch.to_string(),
            modifiers: vec![],
        };

        let action: Action = config.into();

        match action {
            Action::Keystroke { keys, .. } => {
                assert_eq!(keys.len(), 1, "Failed for character: {}", ch);
            }
            _ => panic!("Expected Keystroke action for {}", ch),
        }
    }
}

#[test]
fn test_keystroke_with_uppercase_letters() {
    let config = ActionConfig::Keystroke {
        keys: "A+B+C+D+E".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            assert_eq!(keys.len(), 5, "Should parse uppercase letters");
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_modifier_all_variants_combined() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec![
            "cmd".to_string(),
            "ctrl".to_string(),
            "alt".to_string(),
            "shift".to_string(),
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            assert_eq!(modifiers.len(), 4, "Should parse all four modifiers");
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_text_with_very_long_string() {
    let long_text = "a".repeat(10000);
    let config = ActionConfig::Text {
        text: long_text.clone(),
    };

    let action: Action = config.into();

    match action {
        Action::Text(t) => {
            assert_eq!(t.len(), 10000);
            assert_eq!(t, long_text);
        }
        _ => panic!("Expected Text action"),
    }
}

#[test]
fn test_text_with_unicode_characters() {
    let text = "Hello 世界 🌍 Привет مرحبا".to_string();
    let config = ActionConfig::Text { text: text.clone() };

    let action: Action = config.into();

    match action {
        Action::Text(t) => {
            assert_eq!(t, text);
        }
        _ => panic!("Expected Text action"),
    }
}

#[test]
fn test_launch_empty_string() {
    let config = ActionConfig::Launch {
        app: "".to_string(),
    };

    let action: Action = config.into();

    match action {
        Action::Launch(a) => {
            assert_eq!(a, "");
        }
        _ => panic!("Expected Launch action"),
    }
}

#[test]
fn test_shell_empty_command() {
    let config = ActionConfig::Shell {
        command: "".to_string(),
    };

    let action: Action = config.into();

    match action {
        Action::Shell(cmd) => {
            assert_eq!(cmd, "");
        }
        _ => panic!("Expected Shell action"),
    }
}

#[test]
fn test_shell_with_pipes_and_redirects() {
    let command = "cat file.txt | grep pattern > output.txt 2>&1".to_string();
    let config = ActionConfig::Shell {
        command: command.clone(),
    };

    let action: Action = config.into();

    match action {
        Action::Shell(cmd) => {
            assert_eq!(cmd, command);
        }
        _ => panic!("Expected Shell action"),
    }
}

#[test]
fn test_sequence_with_100_actions() {
    let mut actions = Vec::new();
    for i in 0..100 {
        actions.push(ActionConfig::Delay { ms: i });
    }

    let config = ActionConfig::Sequence { actions };

    let action: Action = config.into();

    match action {
        Action::Sequence(acts) => {
            assert_eq!(acts.len(), 100);
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_mouse_click_with_zero_coords() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: Some(0),
        y: Some(0),
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, Some(0));
            assert_eq!(y, Some(0));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_mouse_click_with_large_coords() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: Some(i32::MAX),
        y: Some(i32::MAX),
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, Some(i32::MAX));
            assert_eq!(y, Some(i32::MAX));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_mouse_click_partial_coords_x_only() {
    let config = ActionConfig::MouseClick {
        button: "left".to_string(),
        x: Some(100),
        y: None,
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, Some(100));
            assert_eq!(y, None);
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_mouse_click_partial_coords_y_only() {
    let config = ActionConfig::MouseClick {
        button: "right".to_string(),
        x: None,
        y: Some(200),
    };

    let action: Action = config.into();

    match action {
        Action::MouseClick { x, y, .. } => {
            assert_eq!(x, None);
            assert_eq!(y, Some(200));
        }
        _ => panic!("Expected MouseClick action"),
    }
}

#[test]
fn test_delay_boundary_values() {
    let delays = vec![0, 1, 100, 1000, 10000, u64::MAX];

    for ms in delays {
        let config = ActionConfig::Delay { ms };
        let action: Action = config.into();

        match action {
            Action::Delay(d) => {
                assert_eq!(d, ms, "Failed for delay: {}", ms);
            }
            _ => panic!("Expected Delay action for ms: {}", ms),
        }
    }
}

#[test]
fn test_action_config_debug_format() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec!["cmd".to_string()],
    };

    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("Keystroke"));
    assert!(debug_str.contains("keys"));
}

#[test]
fn test_key_parsing_case_sensitivity() {
    // Test that special keys are case insensitive
    let test_pairs = vec![
        ("space", "SPACE"),
        ("return", "RETURN"),
        ("tab", "TAB"),
        ("escape", "ESCAPE"),
    ];

    for (lower, upper) in test_pairs {
        let config_lower = ActionConfig::Keystroke {
            keys: lower.to_string(),
            modifiers: vec![],
        };
        let config_upper = ActionConfig::Keystroke {
            keys: upper.to_string(),
            modifiers: vec![],
        };

        let action_lower: Action = config_lower.into();
        let action_upper: Action = config_upper.into();

        match (action_lower, action_upper) {
            (Action::Keystroke { keys: k1, .. }, Action::Keystroke { keys: k2, .. }) => {
                assert_eq!(k1.len(), k2.len(), "Case sensitivity failed for {}", lower);
            }
            _ => panic!("Expected Keystroke actions"),
        }
    }
}

#[test]
fn test_modifier_parsing_whitespace() {
    let config = ActionConfig::Keystroke {
        keys: "a".to_string(),
        modifiers: vec![" cmd ".to_string(), "  shift  ".to_string()],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { modifiers, .. } => {
            // Whitespace is NOT trimmed - these will fail to parse
            assert_eq!(
                modifiers.len(),
                0,
                "Modifiers with whitespace should not parse"
            );
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_sequence_preserves_order() {
    let config = ActionConfig::Sequence {
        actions: vec![
            ActionConfig::Text {
                text: "first".to_string(),
            },
            ActionConfig::Text {
                text: "second".to_string(),
            },
            ActionConfig::Text {
                text: "third".to_string(),
            },
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 3);

            if let Action::Text(ref t) = actions[0] {
                assert_eq!(t, "first");
            }
            if let Action::Text(ref t) = actions[1] {
                assert_eq!(t, "second");
            }
            if let Action::Text(ref t) = actions[2] {
                assert_eq!(t, "third");
            }
        }
        _ => panic!("Expected Sequence action"),
    }
}

#[test]
fn test_keystroke_with_mixed_special_and_regular() {
    let config = ActionConfig::Keystroke {
        keys: "a+space+b+return+c".to_string(),
        modifiers: vec![],
    };

    let action: Action = config.into();

    match action {
        Action::Keystroke { keys, .. } => {
            assert_eq!(keys.len(), 5);
        }
        _ => panic!("Expected Keystroke action"),
    }
}

#[test]
fn test_parse_return_vs_enter() {
    let config1 = ActionConfig::Keystroke {
        keys: "return".to_string(),
        modifiers: vec![],
    };
    let config2 = ActionConfig::Keystroke {
        keys: "enter".to_string(),
        modifiers: vec![],
    };

    let action1: Action = config1.into();
    let action2: Action = config2.into();

    // Both should parse to the same key
    match (action1, action2) {
        (Action::Keystroke { keys: k1, .. }, Action::Keystroke { keys: k2, .. }) => {
            assert_eq!(k1.len(), 1);
            assert_eq!(k2.len(), 1);
        }
        _ => panic!("Expected Keystroke actions"),
    }
}

#[test]
fn test_parse_escape_vs_esc() {
    let config1 = ActionConfig::Keystroke {
        keys: "escape".to_string(),
        modifiers: vec![],
    };
    let config2 = ActionConfig::Keystroke {
        keys: "esc".to_string(),
        modifiers: vec![],
    };

    let action1: Action = config1.into();
    let action2: Action = config2.into();

    // Both should parse to the same key
    match (action1, action2) {
        (Action::Keystroke { keys: k1, .. }, Action::Keystroke { keys: k2, .. }) => {
            assert_eq!(k1.len(), 1);
            assert_eq!(k2.len(), 1);
        }
        _ => panic!("Expected Keystroke actions"),
    }
}

#[test]
fn test_parse_delete_vs_del() {
    let config1 = ActionConfig::Keystroke {
        keys: "delete".to_string(),
        modifiers: vec![],
    };
    let config2 = ActionConfig::Keystroke {
        keys: "del".to_string(),
        modifiers: vec![],
    };

    let action1: Action = config1.into();
    let action2: Action = config2.into();

    // Both should parse to the same key
    match (action1, action2) {
        (Action::Keystroke { keys: k1, .. }, Action::Keystroke { keys: k2, .. }) => {
            assert_eq!(k1.len(), 1);
            assert_eq!(k2.len(), 1);
        }
        _ => panic!("Expected Keystroke actions"),
    }
}

#[test]
fn test_complex_real_world_sequence() {
    // Simulate a real-world use case: opening an app, waiting, typing, and pressing enter
    let config = ActionConfig::Sequence {
        actions: vec![
            ActionConfig::Launch {
                app: "TextEdit".to_string(),
            },
            ActionConfig::Delay { ms: 1000 },
            ActionConfig::Text {
                text: "Hello, World!".to_string(),
            },
            ActionConfig::Delay { ms: 100 },
            ActionConfig::Keystroke {
                keys: "return".to_string(),
                modifiers: vec![],
            },
            ActionConfig::Keystroke {
                keys: "s".to_string(),
                modifiers: vec!["cmd".to_string()],
            },
        ],
    };

    let action: Action = config.into();

    match action {
        Action::Sequence(actions) => {
            assert_eq!(actions.len(), 6);
            assert!(matches!(actions[0], Action::Launch(_)));
            assert!(matches!(actions[1], Action::Delay(1000)));
            assert!(matches!(actions[2], Action::Text(_)));
            assert!(matches!(actions[3], Action::Delay(100)));
            assert!(matches!(actions[4], Action::Keystroke { .. }));
            assert!(matches!(actions[5], Action::Keystroke { .. }));
        }
        _ => panic!("Expected Sequence action"),
    }
}
