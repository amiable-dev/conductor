// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Test backward compatibility layer - verifies old import paths work

// Test old-style imports through the compatibility layer
use conductor::actions::Action;
use conductor::config::{ActionConfig, Config, Trigger};
use conductor::device_profile::{DeviceProfile, PadPageMapping};
use conductor::event_processor::EventProcessor;
use conductor::feedback::LightingScheme;
use conductor::mappings::MappingEngine;
use conductor_daemon::ActionExecutor; // ActionExecutor moved to daemon in Phase 2

#[test]
fn test_config_module_accessible() {
    // Verify config types accessible via old path
    let _: Option<Config> = None;
    let _: Option<Trigger> = None;
    let _: Option<ActionConfig> = None;
}

#[test]
fn test_event_processor_module_accessible() {
    // Verify EventProcessor accessible via old path
    let _processor = EventProcessor::new();
}

#[test]
#[cfg_attr(
    target_os = "linux",
    ignore = "Requires display server for ActionExecutor"
)]
fn test_actions_module_accessible() {
    // Verify actions types accessible via old path
    let _executor = ActionExecutor::new();
    let _: Option<Action> = None;
}

#[test]
fn test_mappings_module_accessible() {
    // Verify MappingEngine accessible via old path
    let _engine = MappingEngine::new();
}

#[test]
fn test_feedback_module_accessible() {
    // Verify feedback types accessible via old path
    let schemes = LightingScheme::list_all();
    assert!(schemes.contains(&"reactive"));
}

#[test]
fn test_device_profile_module_accessible() {
    // Verify device profile types accessible via old path
    let _: Option<DeviceProfile> = None;
    let _: Option<PadPageMapping> = None;
}

#[test]
#[cfg_attr(
    target_os = "linux",
    ignore = "Requires display server for ActionExecutor"
)]
fn test_root_level_imports_work() {
    // Test that root-level re-exports also work
    use conductor::{Config, MidiEvent, ProcessedEvent};
    use conductor_daemon::ActionExecutor;

    let _: Option<Config> = None;
    let _: Option<MidiEvent> = None;
    let _: Option<ProcessedEvent> = None;
    let _executor = ActionExecutor::new();
}
