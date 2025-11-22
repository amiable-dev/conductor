// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin metadata structures

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::Capability;

/// Plugin metadata (read from plugin manifest)
///
/// Contains information about a plugin parsed from its `plugin.toml`
/// manifest file. Used by the plugin manager for discovery, validation,
/// and display in the UI.
///
/// # Example plugin.toml
///
/// ```toml
/// [plugin]
/// name = "http_request"
/// version = "1.0.0"
/// description = "Make HTTP requests as actions"
/// author = "MIDIMon Team"
/// homepage = "https://github.com/amiable-dev/midimon-plugins"
/// license = "MIT"
/// type = "action"
/// binary = "http_request.so"
/// checksum = "sha256:abc123..."
///
/// [plugin.capabilities]
/// network = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin identifier (must match ActionPlugin::name())
    pub name: String,

    /// Semantic version
    pub version: String,

    /// Human-readable description
    pub description: String,

    /// Plugin author name or organization
    pub author: String,

    /// Optional homepage URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    /// License identifier (e.g., "MIT", "Apache-2.0")
    pub license: String,

    /// Plugin type: "action" or "trigger"
    #[serde(rename = "type")]
    pub plugin_type: PluginType,

    /// Capabilities required by this plugin
    #[serde(default)]
    pub capabilities: Vec<Capability>,

    // Runtime fields (not in manifest)
    /// Absolute path to plugin binary (.so/.dylib/.dll)
    #[serde(skip)]
    pub binary_path: PathBuf,

    /// SHA256 checksum of binary (hex string)
    #[serde(skip)]
    pub checksum: String,

    /// Ed25519 signature for verification (future feature)
    #[serde(skip)]
    pub signature: Option<String>,

    /// Whether this plugin is currently enabled
    #[serde(skip)]
    pub enabled: bool,
}

/// Plugin type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    /// Action plugin (provides custom actions)
    Action,

    /// Trigger plugin (provides custom event sources)
    Trigger,
}

impl PluginMetadata {
    /// Create a new plugin metadata instance
    pub fn new(
        name: String,
        version: String,
        description: String,
        author: String,
        license: String,
        plugin_type: PluginType,
    ) -> Self {
        Self {
            name,
            version,
            description,
            author,
            homepage: None,
            license,
            plugin_type,
            capabilities: vec![],
            binary_path: PathBuf::new(),
            checksum: String::new(),
            signature: None,
            enabled: false,
        }
    }

    /// Set binary path
    pub fn with_binary_path(mut self, path: PathBuf) -> Self {
        self.binary_path = path;
        self
    }

    /// Set checksum
    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }

    /// Set capabilities
    pub fn with_capabilities(mut self, capabilities: Vec<Capability>) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Check if plugin has dangerous capabilities
    pub fn has_dangerous_capabilities(&self) -> bool {
        self.capabilities.iter().any(|cap| {
            matches!(
                cap,
                Capability::Filesystem | Capability::Subprocess | Capability::SystemControl
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata_creation() {
        let metadata = PluginMetadata::new(
            "test_plugin".to_string(),
            "1.0.0".to_string(),
            "A test plugin".to_string(),
            "Test Author".to_string(),
            "MIT".to_string(),
            PluginType::Action,
        );

        assert_eq!(metadata.name, "test_plugin");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.plugin_type, PluginType::Action);
        assert_eq!(metadata.capabilities.len(), 0);
        assert!(!metadata.enabled);
    }

    #[test]
    fn test_plugin_metadata_builder() {
        let metadata = PluginMetadata::new(
            "test".to_string(),
            "1.0.0".to_string(),
            "Test".to_string(),
            "Author".to_string(),
            "MIT".to_string(),
            PluginType::Action,
        )
        .with_binary_path(PathBuf::from("/path/to/plugin.so"))
        .with_checksum("abc123".to_string())
        .with_capabilities(vec![Capability::Network]);

        assert_eq!(metadata.binary_path, PathBuf::from("/path/to/plugin.so"));
        assert_eq!(metadata.checksum, "abc123");
        assert_eq!(metadata.capabilities, vec![Capability::Network]);
    }

    #[test]
    fn test_dangerous_capabilities_detection() {
        let safe = PluginMetadata::new(
            "safe".to_string(),
            "1.0.0".to_string(),
            "Safe plugin".to_string(),
            "Author".to_string(),
            "MIT".to_string(),
            PluginType::Action,
        )
        .with_capabilities(vec![Capability::Network, Capability::Audio]);

        assert!(!safe.has_dangerous_capabilities());

        let dangerous = PluginMetadata::new(
            "dangerous".to_string(),
            "1.0.0".to_string(),
            "Dangerous plugin".to_string(),
            "Author".to_string(),
            "MIT".to_string(),
            PluginType::Action,
        )
        .with_capabilities(vec![Capability::Filesystem, Capability::Network]);

        assert!(dangerous.has_dangerous_capabilities());
    }

    #[test]
    fn test_plugin_type_serialization() {
        let action_json = serde_json::to_string(&PluginType::Action).unwrap();
        assert_eq!(action_json, "\"action\"");

        let trigger_json = serde_json::to_string(&PluginType::Trigger).unwrap();
        assert_eq!(trigger_json, "\"trigger\"");
    }
}
