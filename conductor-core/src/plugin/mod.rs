// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin system for extensible actions and triggers
//!
//! This module provides the foundation for MIDIMon's plugin architecture,
//! enabling third-party developers to create custom actions and triggers
//! without modifying the core codebase.
//!
//! # Architecture
//!
//! The plugin system consists of several key components:
//!
//! - **ActionPlugin**: Trait for plugins that provide custom actions
//! - **TriggerPlugin**: Trait for plugins that provide custom event sources (future)
//! - **PluginLoader**: Dynamic library loader for .so/.dylib/.dll files
//! - **PluginMetadata**: Plugin information parsed from manifests
//! - **Capability**: Permission system for plugin security
//!
//! # Example
//!
//! ```rust,no_run
//! use conductor_core::plugin::{ActionPlugin, TriggerContext};
//! use serde_json::Value;
//! use std::error::Error;
//!
//! struct MyPlugin;
//!
//! impl ActionPlugin for MyPlugin {
//!     fn name(&self) -> &str { "my_plugin" }
//!     fn version(&self) -> &str { "1.0.0" }
//!     fn description(&self) -> &str { "My custom plugin" }
//!
//!     fn execute(&mut self, params: Value, _context: TriggerContext) -> Result<(), Box<dyn Error>> {
//!         println!("Plugin executed with params: {}", params);
//!         Ok(())
//!     }
//! }
//! ```

mod action_plugin;
mod capability;
mod discovery;
mod loader;
mod metadata;
mod trigger_plugin;

// WASM plugin runtime (v2.5 - sandboxed execution)
#[cfg(feature = "plugin-wasm")]
pub mod wasm_runtime;

// Plugin signing/verification (v2.7 - cryptographic security)
#[cfg(feature = "plugin-signing")]
pub mod signing;

pub use action_plugin::{ActionPlugin, TriggerContext};
pub use capability::{Capability, RiskLevel};
pub use discovery::{
    DiscoveryError, DiscoveryResult, ManifestCapabilities, ManifestPlugin, PluginDiscovery,
    PluginManifest, PluginRegistry,
};
pub use loader::{LoadedPlugin, PluginLoader, PluginLoaderError, PluginLoaderResult};
pub use metadata::{PluginMetadata, PluginType};
pub use trigger_plugin::{PluginEvent, TriggerPlugin};

// Re-export common types for plugin development
pub use serde_json::Value;
pub use std::error::Error;
