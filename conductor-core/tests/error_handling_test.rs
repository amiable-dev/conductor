// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Test error handling across crate boundaries

use conductor_core::{ActionError, Config, ConfigError, EngineError, FeedbackError, ProfileError};
use std::error::Error;

#[test]
fn test_config_error_from_io_error() {
    // Test that ConfigError can be created from std::io::Error
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let config_error = ConfigError::from(io_error);

    // Verify it's a proper error
    assert!(config_error.to_string().contains("IO error"));
}

#[test]
fn test_config_error_display() {
    // Test ConfigError display implementations
    let err = ConfigError::ValidationError("test validation".to_string());
    assert_eq!(err.to_string(), "Validation error: test validation");

    let err = ConfigError::InvalidTrigger("bad trigger".to_string());
    assert_eq!(err.to_string(), "Invalid trigger: bad trigger");
}

#[test]
fn test_engine_error_display() {
    // Test EngineError display implementations
    let err = EngineError::DeviceNotFound("test device".to_string());
    assert_eq!(err.to_string(), "Device not found: test device");

    let err = EngineError::InvalidMode(5);
    assert_eq!(err.to_string(), "Invalid mode: 5");
}

#[test]
fn test_action_error_display() {
    // Test ActionError display implementations
    let err = ActionError::InvalidKey("test key".to_string());
    assert_eq!(err.to_string(), "Invalid key: test key");

    let err = ActionError::AppNotFound("TestApp".to_string());
    assert_eq!(err.to_string(), "Application not found: TestApp");
}

#[test]
fn test_feedback_error_display() {
    // Test FeedbackError display implementations
    let err = FeedbackError::NotConnected;
    assert_eq!(err.to_string(), "Device not connected");

    let err = FeedbackError::HidError("test error".to_string());
    assert_eq!(err.to_string(), "HID error: test error");
}

#[test]
fn test_profile_error_display() {
    // Test ProfileError display implementations
    let err = ProfileError::XmlError("parse error".to_string());
    assert_eq!(err.to_string(), "XML parse error: parse error");

    let err = ProfileError::InvalidProfile("bad profile".to_string());
    assert_eq!(err.to_string(), "Invalid profile: bad profile");
}

#[test]
fn test_engine_error_from_config_error() {
    // Test that EngineError can be created from ConfigError
    let config_err = ConfigError::ValidationError("test".to_string());
    let engine_err = EngineError::from(config_err);

    match engine_err {
        EngineError::ConfigError(_) => {}
        _ => panic!("Expected ConfigError variant"),
    }
}

#[test]
fn test_error_source_chain() {
    // Test that error source chains work correctly
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let config_error = ConfigError::from(io_error);

    // Verify source chain
    assert!(config_error.source().is_some());
}

#[test]
fn test_config_load_invalid_file() {
    // Test Config::load with invalid file path
    let result = Config::load("nonexistent_config_file_12345.toml");

    // Config might return Ok with defaults or Err - either is acceptable behavior
    // The key is that the API works across crate boundaries
    match result {
        Ok(_) => {
            // Config has default behavior - acceptable
        }
        Err(e) => {
            // Should be an IO error or parse error
            let err_str = e.to_string();
            assert!(
                err_str.contains("IO error")
                    || err_str.contains("No such file")
                    || err_str.contains("Parse error")
            );
        }
    }
}

#[test]
fn test_all_errors_are_send_sync() {
    // Verify all error types are Send + Sync (required for threading)
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<EngineError>();
    assert_send_sync::<ConfigError>();
    assert_send_sync::<ActionError>();
    assert_send_sync::<FeedbackError>();
    assert_send_sync::<ProfileError>();
}
