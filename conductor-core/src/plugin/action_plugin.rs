// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! ActionPlugin trait for custom action plugins

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use super::Capability;

/// Trait for custom action plugins
///
/// Plugins that implement this trait can be dynamically loaded and used
/// as actions in MIDIMon mappings. The plugin receives parameters and
/// a trigger context when executed.
///
/// # Example
///
/// ```rust,no_run
/// use conductor_core::plugin::{ActionPlugin, TriggerContext, Capability};
/// use serde_json::Value;
/// use std::error::Error;
///
/// struct EchoPlugin;
///
/// impl ActionPlugin for EchoPlugin {
///     fn name(&self) -> &str {
///         "echo"
///     }
///
///     fn version(&self) -> &str {
///         "1.0.0"
///     }
///
///     fn description(&self) -> &str {
///         "Prints parameters to stdout"
///     }
///
///     fn execute(&mut self, params: Value, context: TriggerContext) -> Result<(), Box<dyn Error>> {
///         println!("Echo: {:?}", params);
///         println!("Velocity: {:?}", context.velocity);
///         Ok(())
///     }
/// }
/// ```
pub trait ActionPlugin: Send + Sync {
    /// Plugin identifier (unique, lowercase, alphanumeric + underscore)
    ///
    /// Must be unique across all plugins. Used in configuration files
    /// to reference this plugin.
    ///
    /// # Format
    /// - Lowercase letters, numbers, and underscores only
    /// - No spaces or special characters
    /// - Example: `"http_request"`, `"spotify_control"`
    fn name(&self) -> &str;

    /// Semantic version (e.g., "1.0.0")
    ///
    /// Used for compatibility checking and update management.
    fn version(&self) -> &str;

    /// Human-readable description
    ///
    /// Displayed in the Plugin Manager UI.
    fn description(&self) -> &str;

    /// Execute the plugin action with given parameters
    ///
    /// This method is called when the action is triggered by a MIDI event.
    /// The plugin receives:
    /// - `params`: JSON object with plugin-specific configuration
    /// - `context`: Information about the trigger event (velocity, mode, timestamp)
    ///
    /// # Errors
    ///
    /// Return an error if the action cannot be executed. The error will be
    /// logged and displayed to the user, but will not crash the daemon.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use conductor_core::plugin::{ActionPlugin, TriggerContext};
    /// # use serde_json::Value;
    /// # use std::error::Error;
    /// # struct MyPlugin;
    /// # impl ActionPlugin for MyPlugin {
    /// #   fn name(&self) -> &str { "my_plugin" }
    /// #   fn version(&self) -> &str { "1.0.0" }
    /// #   fn description(&self) -> &str { "My plugin" }
    /// fn execute(&mut self, params: Value, context: TriggerContext) -> Result<(), Box<dyn Error>> {
    ///     let url = params["url"].as_str().ok_or("Missing 'url' parameter")?;
    ///
    ///     // Use velocity from context if available
    ///     if let Some(velocity) = context.velocity {
    ///         println!("Velocity: {}", velocity);
    ///     }
    ///
    ///     // Execute plugin action...
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn execute(&mut self, params: Value, context: TriggerContext) -> Result<(), Box<dyn Error>>;

    /// Optional: Plugin capabilities (permissions required)
    ///
    /// Returns the capabilities this plugin needs to function.
    /// Used by the permission system to prompt the user for approval.
    ///
    /// Default: No capabilities required (safe plugin)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use conductor_core::plugin::{ActionPlugin, Capability};
    /// # struct HttpPlugin;
    /// # impl ActionPlugin for HttpPlugin {
    /// #   fn name(&self) -> &str { "http" }
    /// #   fn version(&self) -> &str { "1.0.0" }
    /// #   fn description(&self) -> &str { "HTTP plugin" }
    /// #   fn execute(&mut self, _: serde_json::Value, _: conductor_core::plugin::TriggerContext) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    /// fn capabilities(&self) -> Vec<Capability> {
    ///     vec![Capability::Network]
    /// }
    /// # }
    /// ```
    fn capabilities(&self) -> Vec<Capability> {
        vec![]
    }

    /// Optional: Initialize plugin (called once after load)
    ///
    /// Use this for one-time setup like creating HTTP clients,
    /// opening connections, loading resources, etc.
    ///
    /// # Errors
    ///
    /// Return an error if initialization fails. The plugin will not be loaded.
    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Optional: Cleanup plugin (called before unload)
    ///
    /// Use this to close connections, flush buffers, release resources, etc.
    ///
    /// # Errors
    ///
    /// Errors are logged but do not prevent shutdown.
    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

/// Trigger context passed to plugin execute()
///
/// Contains information about the event that triggered the action.
/// Plugins can use this context to make velocity-sensitive or
/// mode-aware decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerContext {
    /// MIDI velocity (0-127) if trigger was velocity-sensitive
    ///
    /// `None` for triggers that don't have velocity (e.g., encoder turns)
    pub velocity: Option<u8>,

    /// Current mode index
    ///
    /// Plugins can use this to implement mode-aware behavior.
    pub current_mode: Option<usize>,

    /// Timestamp of trigger event (milliseconds since Unix epoch)
    ///
    /// Use for timing analysis or rate limiting.
    pub timestamp: u64,
}

impl TriggerContext {
    /// Create a new trigger context
    pub fn new() -> Self {
        Self {
            velocity: None,
            current_mode: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    /// Create a trigger context with velocity
    pub fn with_velocity(velocity: u8) -> Self {
        Self {
            velocity: Some(velocity),
            current_mode: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    /// Create a trigger context with velocity and mode
    pub fn with_velocity_and_mode(velocity: u8, mode: usize) -> Self {
        Self {
            velocity: Some(velocity),
            current_mode: Some(mode),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}

impl Default for TriggerContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        execute_count: usize,
    }

    impl ActionPlugin for TestPlugin {
        fn name(&self) -> &str {
            "test_plugin"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn description(&self) -> &str {
            "A test plugin"
        }

        fn execute(
            &mut self,
            _params: Value,
            _context: TriggerContext,
        ) -> Result<(), Box<dyn Error>> {
            self.execute_count += 1;
            Ok(())
        }

        fn capabilities(&self) -> Vec<Capability> {
            vec![Capability::Network]
        }
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = TestPlugin { execute_count: 0 };
        assert_eq!(plugin.name(), "test_plugin");
        assert_eq!(plugin.version(), "1.0.0");
        assert_eq!(plugin.description(), "A test plugin");
        assert_eq!(plugin.capabilities(), vec![Capability::Network]);
    }

    #[test]
    fn test_plugin_execute() {
        let mut plugin = TestPlugin { execute_count: 0 };
        let context = TriggerContext::with_velocity(100);
        let params = serde_json::json!({ "test": "value" });

        assert_eq!(plugin.execute_count, 0);
        plugin.execute(params, context).unwrap();
        assert_eq!(plugin.execute_count, 1);
    }

    #[test]
    fn test_trigger_context_creation() {
        let ctx1 = TriggerContext::new();
        assert_eq!(ctx1.velocity, None);
        assert_eq!(ctx1.current_mode, None);

        let ctx2 = TriggerContext::with_velocity(100);
        assert_eq!(ctx2.velocity, Some(100));
        assert_eq!(ctx2.current_mode, None);

        let ctx3 = TriggerContext::with_velocity_and_mode(85, 2);
        assert_eq!(ctx3.velocity, Some(85));
        assert_eq!(ctx3.current_mode, Some(2));
    }

    #[test]
    fn test_trigger_context_default() {
        let ctx: TriggerContext = Default::default();
        assert_eq!(ctx.velocity, None);
        assert_eq!(ctx.current_mode, None);
    }
}
