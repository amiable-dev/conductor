// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Configuration module for MIDIMon
//!
//! This module defines and manages MIDI mapping configurations. It's organized into:
//!
//! - **types**: Data structures representing configuration (Config, Mode, Mapping, Trigger, ActionConfig)
//! - **loader**: Loading, saving, and validating configuration files
//!
//! # Example
//!
//! ```rust,no_run
//! use conductor_core::Config;
//!
//! // Load configuration from a file (creates default if not found)
//! let config = Config::load("config.toml")?;
//!
//! // Validate the configuration
//! config.validate()?;
//!
//! // Save to a new location
//! config.save("backup.toml")?;
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod loader;
pub mod types;

// Re-export types for convenience
pub use types::{
    ActionConfig, AdvancedSettings, Config, DeviceConfig, LoggingConfig, Mapping, Mode, Trigger,
};
