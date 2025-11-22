// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! MIDIMon Core Engine
//!
//! This module will contain the main MidiMonEngine type that orchestrates
//! MIDI input, event processing, mapping, and action execution.
//!
//! NOTE: Full implementation will be added in Step 6 (API Implementation).
//! For now, this is a placeholder to establish the module structure.

#![allow(dead_code, unused_variables)]

use crate::config::Config;
use crate::error::EngineError;

/// Main engine for MIDI mapping and processing
pub struct MidiMonEngine {
    config: Config,
    running: bool,
}

impl MidiMonEngine {
    /// Create a new engine with the given configuration
    pub fn new(config: Config) -> Result<Self, EngineError> {
        Ok(Self {
            config,
            running: false,
        })
    }

    /// Start the engine (connects to devices and begins processing)
    pub fn start(&mut self) -> Result<(), EngineError> {
        if self.running {
            return Err(EngineError::AlreadyRunning);
        }
        self.running = true;
        // TODO: Implement in Step 6
        Ok(())
    }

    /// Stop the engine
    pub fn stop(&mut self) -> Result<(), EngineError> {
        if !self.running {
            return Err(EngineError::NotRunning);
        }
        self.running = false;
        // TODO: Implement in Step 6
        Ok(())
    }

    /// Check if engine is running
    pub fn is_running(&self) -> bool {
        self.running
    }
}
