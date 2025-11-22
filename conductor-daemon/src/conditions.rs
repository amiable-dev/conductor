// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Condition evaluation for conditional actions (v2.2)
//!
//! This module implements runtime evaluation of conditions for the Conditional action type.
//! Supports time-based, app-based, mode-based, and logical operators.

use chrono::{Datelike, Local, Timelike, Weekday};
use conductor_core::Condition;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::process::Command;

/// Context for condition evaluation
///
/// Provides access to current mode and other runtime state needed for evaluation
#[derive(Debug, Clone, Default)]
pub struct ConditionContext {
    /// Current mode name (e.g., "Default", "Development")
    pub current_mode: Option<String>,
}

impl ConditionContext {
    /// Create a new condition context with the current mode
    pub fn with_mode(mode: String) -> Self {
        Self {
            current_mode: Some(mode),
        }
    }
}

/// Evaluates a condition and returns true/false
///
/// # Arguments
/// * `condition` - The condition to evaluate
/// * `context` - Optional context with current mode and state
///
/// # Returns
/// True if the condition is met, false otherwise
pub fn evaluate_condition(condition: &Condition, context: Option<&ConditionContext>) -> bool {
    match condition {
        Condition::Always => true,
        Condition::Never => false,
        Condition::TimeRange { start, end } => evaluate_time_range(start, end),
        Condition::DayOfWeek { days } => evaluate_day_of_week(days),
        Condition::AppRunning { app_name } => evaluate_app_running(app_name),
        Condition::AppFrontmost { app_name } => evaluate_app_frontmost(app_name),
        Condition::ModeIs { mode } => {
            if let Some(ctx) = context {
                if let Some(current_mode) = &ctx.current_mode {
                    current_mode == mode
                } else {
                    false
                }
            } else {
                false
            }
        }
        Condition::And { conditions } => conditions.iter().all(|c| evaluate_condition(c, context)),
        Condition::Or { conditions } => conditions.iter().any(|c| evaluate_condition(c, context)),
        Condition::Not { condition } => !evaluate_condition(condition, context),
    }
}

/// Evaluates a time range condition
///
/// Format: start and end are in 24-hour format "HH:MM"
/// Returns true if current time is within the range
/// Automatically handles ranges that cross midnight
fn evaluate_time_range(start: &str, end: &str) -> bool {
    // Parse time strings "HH:MM"
    let start_parts: Vec<&str> = start.split(':').collect();
    let end_parts: Vec<&str> = end.split(':').collect();

    if start_parts.len() != 2 || end_parts.len() != 2 {
        eprintln!("Invalid time format: start={}, end={}", start, end);
        return false;
    }

    let start_hour: u32 = start_parts[0].parse().unwrap_or(0);
    let start_min: u32 = start_parts[1].parse().unwrap_or(0);
    let end_hour: u32 = end_parts[0].parse().unwrap_or(0);
    let end_min: u32 = end_parts[1].parse().unwrap_or(0);

    if start_hour >= 24 || start_min >= 60 || end_hour >= 24 || end_min >= 60 {
        eprintln!(
            "Invalid time values: {}:{} - {}:{}",
            start_hour, start_min, end_hour, end_min
        );
        return false;
    }

    let now = Local::now();
    let current_hour = now.hour();
    let current_min = now.minute();

    let current_mins = current_hour * 60 + current_min;
    let start_mins = start_hour * 60 + start_min;
    let end_mins = end_hour * 60 + end_min;

    if start_mins <= end_mins {
        // Normal range (doesn't cross midnight)
        current_mins >= start_mins && current_mins <= end_mins
    } else {
        // Range crosses midnight (e.g., 22:00-02:00)
        current_mins >= start_mins || current_mins <= end_mins
    }
}

/// Evaluates a day of week condition
///
/// Days: Monday=1, Tuesday=2, ..., Sunday=7
/// Returns true if current day is in the list
fn evaluate_day_of_week(days: &[u8]) -> bool {
    if days.is_empty() {
        return false;
    }

    let now = Local::now();
    let weekday = now.weekday();

    // Convert chrono::Weekday to number (1=Monday, 7=Sunday)
    let current_day = match weekday {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 7,
    };

    days.contains(&current_day)
}

/// Evaluates if an application is currently running
///
/// Platform-specific implementation:
/// - macOS: Uses `pgrep` to check for running processes
/// - Linux: Uses `pgrep` to check for running processes
/// - Windows: Not implemented (always returns false)
fn evaluate_app_running(app_name: &str) -> bool {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        // Use pgrep to check if process exists
        let output = Command::new("pgrep")
            .arg("-i") // Case-insensitive
            .arg(app_name)
            .output();

        match output {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        eprintln!("AppRunning condition not supported on this platform");
        let _ = app_name; // Suppress unused variable warning
        false
    }
}

/// Evaluates if an application is frontmost (has focus)
///
/// Platform-specific implementation:
/// - macOS: Uses AppleScript to get frontmost application
/// - Linux: Not implemented (always returns false)
/// - Windows: Not implemented (always returns false)
fn evaluate_app_frontmost(app_name: &str) -> bool {
    #[cfg(target_os = "macos")]
    {
        let script = r#"tell application "System Events" to get name of first application process whose frontmost is true"#;

        let output = Command::new("osascript").arg("-e").arg(script).output();

        match output {
            Ok(result) if result.status.success() => {
                let frontmost_app = String::from_utf8_lossy(&result.stdout);
                let frontmost_app = frontmost_app.trim();

                // Case-insensitive comparison
                frontmost_app.eq_ignore_ascii_case(app_name)
            }
            _ => false,
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        eprintln!("AppFrontmost condition not supported on this platform");
        let _ = app_name; // Suppress unused variable warning
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_condition() {
        assert!(evaluate_condition(&Condition::Always, None));
    }

    #[test]
    fn test_never_condition() {
        assert!(!evaluate_condition(&Condition::Never, None));
    }

    #[test]
    fn test_time_range_normal() {
        // This test will pass if run during 00:00-23:59 (always true)
        let condition = Condition::TimeRange {
            start: "00:00".to_string(),
            end: "23:59".to_string(),
        };
        assert!(evaluate_condition(&condition, None));
    }

    #[test]
    fn test_time_range_midnight_crossing() {
        // Test range that crosses midnight (e.g., 22:00-02:00)
        // We can't reliably test this without mocking time,
        // so we just verify it doesn't panic
        let condition = Condition::TimeRange {
            start: "22:00".to_string(),
            end: "02:00".to_string(),
        };
        // Just ensure it runs without panic
        let _ = evaluate_condition(&condition, None);
    }

    #[test]
    fn test_day_of_week() {
        // Test that today is recognized
        let now = Local::now();
        let weekday = now.weekday();
        let day_num = match weekday {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,
        };

        let condition = Condition::DayOfWeek {
            days: vec![day_num],
        };
        assert!(evaluate_condition(&condition, None));
    }

    #[test]
    fn test_day_of_week_empty() {
        let condition = Condition::DayOfWeek { days: vec![] };
        assert!(!evaluate_condition(&condition, None));
    }

    #[test]
    fn test_mode_is_match() {
        let context = ConditionContext::with_mode("Default".to_string());
        let condition = Condition::ModeIs {
            mode: "Default".to_string(),
        };
        assert!(evaluate_condition(&condition, Some(&context)));
    }

    #[test]
    fn test_mode_is_no_match() {
        let context = ConditionContext::with_mode("Default".to_string());
        let condition = Condition::ModeIs {
            mode: "Development".to_string(),
        };
        assert!(!evaluate_condition(&condition, Some(&context)));
    }

    #[test]
    fn test_mode_is_no_context() {
        let condition = Condition::ModeIs {
            mode: "Default".to_string(),
        };
        assert!(!evaluate_condition(&condition, None));
    }

    #[test]
    fn test_and_operator_all_true() {
        let condition = Condition::And {
            conditions: vec![Condition::Always, Condition::Always],
        };
        assert!(evaluate_condition(&condition, None));
    }

    #[test]
    fn test_and_operator_one_false() {
        let condition = Condition::And {
            conditions: vec![Condition::Always, Condition::Never],
        };
        assert!(!evaluate_condition(&condition, None));
    }

    #[test]
    fn test_or_operator_all_false() {
        let condition = Condition::Or {
            conditions: vec![Condition::Never, Condition::Never],
        };
        assert!(!evaluate_condition(&condition, None));
    }

    #[test]
    fn test_or_operator_one_true() {
        let condition = Condition::Or {
            conditions: vec![Condition::Always, Condition::Never],
        };
        assert!(evaluate_condition(&condition, None));
    }

    #[test]
    fn test_not_operator() {
        let condition = Condition::Not {
            condition: Box::new(Condition::Always),
        };
        assert!(!evaluate_condition(&condition, None));

        let condition = Condition::Not {
            condition: Box::new(Condition::Never),
        };
        assert!(evaluate_condition(&condition, None));
    }

    #[test]
    fn test_complex_nested_conditions() {
        // Test: (Always AND (Never OR Always)) = true
        let condition = Condition::And {
            conditions: vec![
                Condition::Always,
                Condition::Or {
                    conditions: vec![Condition::Never, Condition::Always],
                },
            ],
        };
        assert!(evaluate_condition(&condition, None));
    }
}
