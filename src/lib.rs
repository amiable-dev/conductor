// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Conductor Library
//!
//! Multi-protocol input mapping system for MIDI controllers, game controllers,
//! and custom hardware. This is the main library entry point that re-exports
//! types from conductor_core.
//!
//! New code should use conductor_core directly instead of this module.

// Re-export everything from conductor_core
pub use conductor_core::*;

// Module aliases for common imports
pub mod config {
    pub use conductor_core::config::*;
}

pub mod event_processor {
    pub use conductor_core::event_processor::*;
}

pub mod actions {
    pub use conductor_core::actions::*;
    // Note: ActionExecutor moved to conductor-daemon
    // Use `conductor_daemon::ActionExecutor` instead
}

pub mod mappings {
    pub use conductor_core::mapping::*;
}

pub mod feedback {
    pub use conductor_core::feedback::*;
}

pub mod device_profile {
    pub use conductor_core::device::*;
}

// Note: mikro_leds and midi_feedback are private implementation details
// in conductor_core and are not re-exported. Tests should use the public
// PadFeedback trait instead.
