// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration test to verify public API exports work correctly

use conductor_core::{
    // Actions
    Action,
    ActionConfig,
    ActionError,

    // Configuration
    Config,
    ConfigError,
    // Device Profiles
    DeviceProfile,
    EncoderDirection,
    // Errors
    EngineError,
    EventProcessor,

    FeedbackError,
    LightingScheme,
    // Mapping
    MappingEngine,
    // Events
    MidiEvent,
    PadPageMapping,

    ProcessedEvent,
    ProfileError,

    Trigger,
    VelocityLevel,
};
use conductor_daemon::ActionExecutor;

#[test]
fn test_all_types_accessible() {
    // This test verifies that all public API types are accessible
    // and can be referenced without errors

    // Type checks - just verify they compile
    let _: Option<Config> = None;
    let _: Option<MidiEvent> = None;
    let _: Option<ProcessedEvent> = None;
    let _: Option<Action> = None;
    let _: Option<Trigger> = None;
    let _: Option<ActionConfig> = None;
    let _: Option<VelocityLevel> = None;
    let _: Option<EncoderDirection> = None;
    let _: Option<LightingScheme> = None;
    let _: Option<DeviceProfile> = None;
    let _: Option<PadPageMapping> = None;
}

#[test]
#[cfg_attr(
    target_os = "linux",
    ignore = "Requires display server for ActionExecutor"
)]
fn test_action_executor_creation() {
    // Verify ActionExecutor can be created
    let _executor = ActionExecutor::new();
}

#[test]
fn test_mapping_engine_creation() {
    // Verify MappingEngine can be created
    let _engine = MappingEngine::new();
}

#[test]
fn test_event_processor_creation() {
    // Verify EventProcessor can be created
    let _processor = EventProcessor::new();
}

#[test]
fn test_velocity_level_enum() {
    // Verify VelocityLevel variants are accessible
    match VelocityLevel::Soft {
        VelocityLevel::Soft => {}
        VelocityLevel::Medium => {}
        VelocityLevel::Hard => {}
    }
}

#[test]
fn test_encoder_direction_enum() {
    // Verify EncoderDirection variants are accessible
    match EncoderDirection::Clockwise {
        EncoderDirection::Clockwise => {}
        EncoderDirection::CounterClockwise => {}
    }
}

#[test]
fn test_lighting_scheme_variants() {
    // Verify LightingScheme has expected variants
    let schemes = LightingScheme::list_all();
    assert!(schemes.contains(&"off"));
    assert!(schemes.contains(&"static"));
    assert!(schemes.contains(&"reactive"));
    assert!(schemes.contains(&"rainbow"));
}

#[test]
fn test_error_types_are_error_trait() {
    // Verify all error types implement std::error::Error
    fn check_error<E: std::error::Error>() {}

    check_error::<EngineError>();
    check_error::<ConfigError>();
    check_error::<ActionError>();
    check_error::<FeedbackError>();
    check_error::<ProfileError>();
}
