// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! TriggerPlugin trait for custom event sources (future feature)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::time::Instant;

/// Trait for custom trigger plugins (future feature)
///
/// TriggerPlugins allow third-party developers to create new event sources
/// for MIDIMon. Examples include:
/// - Keyboard/mouse events
/// - Web hooks
/// - File system watchers
/// - Network events
/// - Time-based triggers
///
/// # Status
///
/// This trait is defined for v2.3 but will be fully implemented in a future
/// version. Currently, only ActionPlugin is supported.
pub trait TriggerPlugin: Send + Sync {
    /// Plugin identifier (unique, lowercase, alphanumeric + underscore)
    fn name(&self) -> &str;

    /// Semantic version (e.g., "1.0.0")
    fn version(&self) -> &str;

    /// Human-readable description
    fn description(&self) -> &str;

    /// Start the trigger plugin (begin monitoring)
    ///
    /// Called when the plugin is enabled. The plugin should start
    /// monitoring for events and queue them for later retrieval.
    fn start(&mut self) -> Result<(), Box<dyn Error>>;

    /// Stop the trigger plugin (stop monitoring)
    ///
    /// Called when the plugin is disabled or unloaded. The plugin
    /// should stop monitoring and clean up resources.
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;

    /// Poll for events (called periodically by daemon)
    ///
    /// Returns a list of events that have occurred since the last poll.
    /// The daemon will call this method every 10-50ms.
    fn poll_events(&mut self) -> Vec<PluginEvent>;
}

/// Event emitted by trigger plugins
///
/// Represents an event from a trigger plugin that can be mapped
/// to actions in the MIDIMon configuration.
#[derive(Debug, Clone)]
pub struct PluginEvent {
    /// Name of the plugin that generated this event
    pub plugin_name: String,

    /// Type of event (plugin-defined, e.g., "button_press", "webhook_received")
    pub event_type: String,

    /// Event-specific data (JSON object)
    pub data: Value,

    /// Timestamp when the event occurred
    pub timestamp: Instant,
}

impl PluginEvent {
    /// Create a new plugin event
    pub fn new(plugin_name: String, event_type: String, data: Value) -> Self {
        Self {
            plugin_name,
            event_type,
            data,
            timestamp: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_event_creation() {
        let event = PluginEvent::new(
            "test_plugin".to_string(),
            "test_event".to_string(),
            serde_json::json!({ "key": "value" }),
        );

        assert_eq!(event.plugin_name, "test_plugin");
        assert_eq!(event.event_type, "test_event");
        assert_eq!(event.data["key"], "value");
    }
}
