// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin registry client for discovering and installing plugins
//!
//! This module is only available when the `plugin-registry` feature is enabled.

#[cfg(feature = "plugin-registry")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "plugin-registry")]
use std::collections::HashMap;
#[cfg(feature = "plugin-registry")]
use std::path::{Path, PathBuf};

#[cfg(feature = "plugin-registry")]
use sha2::Digest;

/// Plugin registry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRegistry {
    pub version: String,
    pub last_updated: String,
    pub plugins: Vec<PluginRegistryEntry>,
    #[serde(default)]
    pub featured_plugins: Vec<String>,
    pub categories: Vec<PluginCategory>,
}

/// Individual plugin entry in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRegistryEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub category: String,
    pub tags: Vec<String>,
    pub capabilities: Vec<String>,
    pub download_url: String,
    pub signature_url: String,
    pub checksum: String,
    pub size_bytes: u64,
    pub license: String,
    pub repository: String,
    pub documentation: String,
    pub min_midimon_version: String,
    pub signed: bool,
    pub verified: bool,
}

/// Plugin category metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCategory {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// Plugin registry client
pub struct PluginRegistryClient {
    registry_url: String,
    cache_dir: PathBuf,
}

impl PluginRegistryClient {
    /// Create a new plugin registry client
    pub fn new(registry_url: impl Into<String>, cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            registry_url: registry_url.into(),
            cache_dir: cache_dir.into(),
        }
    }

    /// Create default registry client
    pub fn default_registry() -> Self {
        let cache_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("midimon")
            .join("plugin_cache");

        Self::new(
            "https://raw.githubusercontent.com/amiable-dev/midimon-plugins/main/registry/registry.json",
            cache_dir,
        )
    }

    /// Fetch the latest plugin registry
    pub async fn fetch_registry(&self) -> Result<PluginRegistry, Box<dyn std::error::Error>> {
        let response = reqwest::get(&self.registry_url).await?;
        let registry: PluginRegistry = response.json().await?;

        // Cache the registry locally
        if let Err(e) = self.cache_registry(&registry).await {
            eprintln!("Warning: Failed to cache registry: {}", e);
        }

        Ok(registry)
    }

    /// Load registry from cache
    pub async fn load_cached_registry(&self) -> Result<PluginRegistry, Box<dyn std::error::Error>> {
        let cache_file = self.cache_dir.join("registry.json");
        let contents = tokio::fs::read_to_string(cache_file).await?;
        let registry: PluginRegistry = serde_json::from_str(&contents)?;
        Ok(registry)
    }

    /// Cache registry to disk
    async fn cache_registry(
        &self,
        registry: &PluginRegistry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all(&self.cache_dir).await?;
        let cache_file = self.cache_dir.join("registry.json");
        let json = serde_json::to_string_pretty(registry)?;
        tokio::fs::write(cache_file, json).await?;
        Ok(())
    }

    /// Get plugin by ID
    pub fn find_plugin<'a>(
        &self,
        registry: &'a PluginRegistry,
        plugin_id: &str,
    ) -> Option<&'a PluginRegistryEntry> {
        registry.plugins.iter().find(|p| p.id == plugin_id)
    }

    /// Search plugins by query
    pub fn search_plugins<'a>(
        &self,
        registry: &'a PluginRegistry,
        query: &str,
    ) -> Vec<&'a PluginRegistryEntry> {
        let query_lower = query.to_lowercase();
        registry
            .plugins
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
                    || p.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// Filter plugins by category
    pub fn filter_by_category<'a>(
        &self,
        registry: &'a PluginRegistry,
        category: &str,
    ) -> Vec<&'a PluginRegistryEntry> {
        registry
            .plugins
            .iter()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Get download URL for plugin
    pub fn get_download_url<'a>(&self, plugin: &'a PluginRegistryEntry) -> &'a String {
        &plugin.download_url
    }

    /// Get checksum for plugin
    pub fn get_checksum<'a>(&self, plugin: &'a PluginRegistryEntry) -> &'a String {
        &plugin.checksum
    }

    /// Detect current platform
    fn current_platform(&self) -> String {
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        return "macos-x86_64".to_string();

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return "macos-aarch64".to_string();

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return "linux-x86_64".to_string();

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        return "windows-x86_64".to_string();

        #[cfg(not(any(
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64")
        )))]
        return "unknown".to_string();
    }

    /// Download plugin binary
    pub async fn download_plugin(
        &self,
        plugin: &PluginRegistryEntry,
        destination: impl AsRef<Path>,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let url = self.get_download_url(plugin);

        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;

        // Verify checksum
        let expected_checksum = self.get_checksum(plugin);
        let digest = sha2::Sha256::digest(&bytes);
        let actual_checksum = format!("sha256:{:x}", digest);
        if actual_checksum != *expected_checksum {
            return Err(format!(
                "Checksum mismatch: expected {}, got {}",
                expected_checksum, actual_checksum
            )
            .into());
        }

        // Write to destination
        let dest_path = destination.as_ref().to_path_buf();
        tokio::fs::create_dir_all(dest_path.parent().unwrap()).await?;
        tokio::fs::write(&dest_path, bytes).await?;

        Ok(dest_path)
    }

    /// Install plugin from registry
    pub async fn install_plugin(
        &self,
        plugin_id: &str,
        plugins_dir: impl AsRef<Path>,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Fetch latest registry
        let registry = self.fetch_registry().await?;

        // Find plugin
        let plugin = self
            .find_plugin(&registry, plugin_id)
            .ok_or(format!("Plugin '{}' not found in registry", plugin_id))?;

        // Determine file extension for platform
        let ext = if cfg!(target_os = "macos") {
            "dylib"
        } else if cfg!(target_os = "linux") {
            "so"
        } else if cfg!(target_os = "windows") {
            "dll"
        } else {
            return Err("Unsupported platform".into());
        };

        // Download to plugins directory
        let filename = format!("libmidimon_{}_plugin.{}", plugin.id, ext);
        let dest_path = plugins_dir.as_ref().join(filename);

        println!("Downloading {} v{}...", plugin.name, plugin.version);
        let installed_path = self.download_plugin(plugin, &dest_path).await?;
        println!("Installed {} to {:?}", plugin.name, installed_path);

        Ok(installed_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_platform() {
        let client = PluginRegistryClient::default_registry();
        let platform = client.current_platform();

        // Should be one of the supported platforms
        assert!(matches!(
            platform.as_str(),
            "macos-x86_64" | "macos-aarch64" | "linux-x86_64" | "windows-x86_64"
        ));
    }

    #[test]
    fn test_search_plugins() {
        let registry = PluginRegistry {
            version: "1.0.0".to_string(),
            last_updated: "2025-01-18T00:00:00Z".to_string(),
            plugins: vec![PluginRegistryEntry {
                id: "spotify".to_string(),
                name: "Spotify Control".to_string(),
                description: "Control Spotify playback".to_string(),
                author: "Test".to_string(),
                version: "0.1.0".to_string(),
                category: "media".to_string(),
                tags: vec!["spotify".to_string(), "music".to_string()],
                capabilities: vec!["network".to_string()],
                download_url: "https://example.com/spotify.wasm".to_string(),
                signature_url: "https://example.com/spotify.wasm.sig".to_string(),
                checksum: "abc123".to_string(),
                size_bytes: 1024,
                license: "MIT".to_string(),
                repository: "https://github.com/example/spotify-plugin".to_string(),
                documentation: "https://example.com/docs".to_string(),
                min_midimon_version: "2.3.0".to_string(),
                signed: true,
                verified: true,
            }],
            featured_plugins: vec![],
            categories: vec![],
        };

        let client = PluginRegistryClient::default_registry();
        let results = client.search_plugins(&registry, "spotify");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "spotify");
    }
}
