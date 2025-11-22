// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Core Engine
//!
//! Pure Rust MIDI mapping engine with zero UI dependencies.
//!
//! This library provides the core functionality for processing MIDI events,
//! mapping them to actions, and executing those actions. It's designed to be
//! embedded in applications that need MIDI controller mapping capabilities.
//!
//! This crate is used by:
//! - **midimon-daemon**: Background service with config hot-reload (v1.0.0+)
//! - **midimon**: Legacy direct-run application (deprecated in v1.0.0)
//! - External applications needing MIDI mapping capabilities
//!
//! # Architecture
//!
//! The engine follows a three-stage processing pipeline:
//!
//! 1. **MIDI Input** → [`MidiEvent`] (raw MIDI bytes converted to structured events)
//! 2. **Event Processing** → [`ProcessedEvent`] (adds timing, velocity, chord detection)
//! 3. **Mapping & Execution** → [`Action`] (matches events to actions and executes them)
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use conductor_core::{Config, MappingEngine, EventProcessor};
//!
//! // Load configuration
//! let config = Config::load("config.toml").expect("Failed to load config");
//!
//! // Create engine components
//! let mut event_processor = EventProcessor::new();
//! let mut mapping_engine = MappingEngine::new();
//!
//! // Process MIDI events (in your event loop)
//! // let midi_event = ...; // from your MIDI input
//! // let processed = event_processor.process(midi_event);
//! // let action = mapping_engine.map_event(&processed, &config);
//! // Action execution is handled by midimon-daemon's ActionExecutor
//! ```
//!
//! # Features
//!
//! ## Trigger Types
//!
//! - **Note**: Basic note on/off with optional velocity range
//! - **VelocityRange**: Different actions for soft/medium/hard presses
//! - **LongPress**: Hold detection with configurable duration
//! - **DoubleTap**: Quick double-tap detection
//! - **NoteChord**: Multiple notes pressed simultaneously
//! - **EncoderTurn**: Encoder rotation with direction
//! - **Aftertouch**: Pressure sensitivity
//! - **PitchBend**: Touch strip control
//! - **CC**: Control change messages
//!
//! ## Action Types
//!
//! - **Keystroke**: Keyboard shortcuts with modifiers
//! - **Text**: Type text strings
//! - **Launch**: Open applications
//! - **Shell**: Execute shell commands
//! - **VolumeControl**: System volume control
//! - **ModeChange**: Switch between mapping modes
//! - **Sequence**: Chain multiple actions
//! - **Delay**: Timing control
//! - **MouseClick**: Mouse simulation
//! - **Repeat**: Repeat an action N times
//! - **Conditional**: Conditional execution
//!
//! ## System Features
//!
//! - **Multi-mode operation**: Switch between different mapping sets
//! - **Global mappings**: Work across all modes
//! - **Device profiles**: Support for device-specific configurations
//! - **LED feedback**: RGB LED control via HID or MIDI
//! - **Zero UI dependencies**: Pure engine library
//!
//! # Examples
//!
//! See the `midimon-daemon` package for a complete CLI implementation.

#![allow(dead_code, unused_variables, unused_imports)]
// TODO: Re-enable missing_docs after adding documentation to all public items
#![allow(missing_docs)]

// Public modules
pub mod actions;
pub mod config;
pub mod device;
pub mod engine;
pub mod error;
pub mod event_processor;
pub mod events;
pub mod gamepad_events; // Gamepad/HID input mapping (v3.0)
pub mod feedback;
pub mod mapping; // Public for advanced event processing
pub mod midi_output; // MIDI output management (v2.1)
pub mod velocity; // Velocity mapping calculations (v2.2)

// Private modules (implementation details)
pub mod logging;
mod midi_feedback;
pub mod mikro_leds; // Structured logging with tracing

// Re-exports for convenience

// Engine
pub use engine::MidiMonEngine;

// Configuration
pub use config::{ActionConfig, Config, DeviceConfig, LoggingConfig, Mapping, Mode, Trigger};

// Events
pub use event_processor::EventProcessor;
pub use events::{EncoderDirection, InputEvent, MidiEvent, ProcessedEvent, VelocityLevel};

// Actions (ActionExecutor moved to midimon-daemon in Phase 2 security refactor)
// Domain-specific types for platform-independent action representation
pub use actions::{
    Action, Condition, KeyCode, MidiMessageParams, MidiMessageType, ModifierKey, MouseButton,
    VelocityCurve, VelocityMapping, VolumeOperation,
};

// Feedback
pub use feedback::{FeedbackManager, LightingScheme, PadFeedback};

// Device Profiles
pub use device::{DeviceProfile, PadPageMapping};

// Errors
pub use error::{ActionError, ConfigError, EngineError, FeedbackError, ProfileError};

// Mapping
pub use mapping::MappingEngine;

// MIDI Output (v2.1)
pub use midi_output::{MidiMessage, MidiOutputManager};

// Plugin System (v2.3)
pub mod plugin;

// Plugin Registry (v2.4)
#[cfg(feature = "plugin-registry")]
pub mod plugin_registry;
