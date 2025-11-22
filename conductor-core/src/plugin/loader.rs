// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin loader for dynamic library loading
//!
//! This module handles loading plugins from shared libraries (.so/.dylib/.dll)
//! using dynamic symbol resolution. It provides a safe wrapper around libloading
//! with proper error handling and validation.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use thiserror::Error;

use super::{ActionPlugin, PluginMetadata};

/// Plugin loader error types
#[derive(Debug, Error)]
pub enum PluginLoaderError {
    /// Failed to load library
    #[error("Failed to load plugin library: {0}")]
    LoadError(String),

    /// Missing required symbol
    #[error("Plugin missing required symbol '{symbol}': {reason}")]
    MissingSymbol { symbol: String, reason: String },

    /// Invalid plugin (doesn't implement required traits)
    #[error("Invalid plugin: {0}")]
    InvalidPlugin(String),

    /// Plugin version incompatible
    #[error("Plugin version {plugin_version} incompatible with core version {core_version}")]
    IncompatibleVersion {
        plugin_version: String,
        core_version: String,
    },

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Plugin loader result type
pub type PluginLoaderResult<T> = Result<T, PluginLoaderError>;

/// Loaded plugin instance
///
/// Wraps a dynamically loaded plugin with its library handle.
/// The library is kept alive as long as this struct exists.
pub struct LoadedPlugin {
    /// Plugin instance (trait object)
    pub plugin: Box<dyn ActionPlugin>,

    /// Plugin metadata
    pub metadata: PluginMetadata,

    /// Library handle (kept alive to prevent unloading)
    _library: Arc<libloading::Library>,

    /// Path to the plugin binary
    pub path: PathBuf,
}

impl LoadedPlugin {
    /// Get plugin name
    pub fn name(&self) -> &str {
        self.plugin.name()
    }

    /// Get plugin version
    pub fn version(&self) -> &str {
        self.plugin.version()
    }

    /// Get plugin description
    pub fn description(&self) -> &str {
        self.plugin.description()
    }
}

/// Plugin loader
///
/// Loads plugins from shared libraries using dynamic symbol resolution.
/// Plugins must export a `_create_plugin` function that returns a
/// boxed trait object.
///
/// # Example
///
/// ```rust,no_run
/// use conductor_core::plugin::PluginLoader;
/// use std::path::Path;
///
/// let loader = PluginLoader::new();
/// let plugin = loader.load_plugin(Path::new("plugins/http_request.so"))?;
///
/// println!("Loaded plugin: {} v{}", plugin.name(), plugin.version());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct PluginLoader {
    /// Core version for compatibility checking
    core_version: String,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self {
            core_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Load a plugin from a shared library
    ///
    /// # Safety
    ///
    /// This function uses `unsafe` to load dynamic libraries and resolve symbols.
    /// It assumes the plugin binary is valid and exports the required symbols.
    /// The plugin must be compiled with a compatible Rust toolchain.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Library file not found or cannot be loaded
    /// - Required symbol `_create_plugin` not found
    /// - Plugin version incompatible with core
    /// - Plugin validation fails
    pub fn load_plugin(&self, path: &Path) -> PluginLoaderResult<LoadedPlugin> {
        // Verify file exists
        if !path.exists() {
            return Err(PluginLoaderError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Plugin file not found: {}", path.display()),
            )));
        }

        // Load the library
        let library = unsafe {
            libloading::Library::new(path)
                .map_err(|e| PluginLoaderError::LoadError(format!("{}: {}", path.display(), e)))?
        };

        // Resolve the plugin creation function
        // Plugins must export: extern "C" fn _create_plugin() -> *mut dyn ActionPlugin
        let create_plugin: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn ActionPlugin> = unsafe {
            library
                .get(b"_create_plugin")
                .map_err(|e| PluginLoaderError::MissingSymbol {
                    symbol: "_create_plugin".to_string(),
                    reason: e.to_string(),
                })?
        };

        // Create the plugin instance
        let plugin_ptr = unsafe { create_plugin() };
        if plugin_ptr.is_null() {
            return Err(PluginLoaderError::InvalidPlugin(
                "Plugin creation returned null".to_string(),
            ));
        }

        // Convert raw pointer to Box
        let plugin = unsafe { Box::from_raw(plugin_ptr) };

        // Create metadata from plugin
        let metadata = PluginMetadata::new(
            plugin.name().to_string(),
            plugin.version().to_string(),
            plugin.description().to_string(),
            "Unknown".to_string(), // Will be populated from manifest
            "Unknown".to_string(), // Will be populated from manifest
            super::PluginType::Action,
        )
        .with_binary_path(path.to_path_buf())
        .with_capabilities(plugin.capabilities());

        // Check version compatibility (simple check for now)
        // TODO: Implement semantic version comparison
        if !self.is_version_compatible(&metadata.version) {
            return Err(PluginLoaderError::IncompatibleVersion {
                plugin_version: metadata.version.clone(),
                core_version: self.core_version.clone(),
            });
        }

        Ok(LoadedPlugin {
            plugin,
            metadata,
            _library: Arc::new(library),
            path: path.to_path_buf(),
        })
    }

    /// Check if plugin version is compatible with core
    ///
    /// For now, this is a simple check. In the future, implement
    /// semantic version comparison (major version must match).
    fn is_version_compatible(&self, _plugin_version: &str) -> bool {
        // TODO: Implement semantic version comparison
        // For v2.3.0, accept all versions (development mode)
        true
    }

    /// Unload a plugin
    ///
    /// This is done automatically when the LoadedPlugin is dropped,
    /// but can be called explicitly if needed.
    pub fn unload_plugin(&self, _plugin: LoadedPlugin) {
        // Plugin is unloaded when LoadedPlugin is dropped
        // (library handle ref count goes to zero)
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert!(!loader.core_version.is_empty());
    }

    #[test]
    fn test_load_nonexistent_plugin() {
        let loader = PluginLoader::new();
        let result = loader.load_plugin(Path::new("/nonexistent/plugin.so"));
        assert!(result.is_err());

        if let Err(PluginLoaderError::IoError(e)) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("Expected IoError::NotFound");
        }
    }

    #[test]
    fn test_version_compatibility() {
        let loader = PluginLoader::new();
        // For now, all versions are compatible (development mode)
        assert!(loader.is_version_compatible("1.0.0"));
        assert!(loader.is_version_compatible("2.0.0"));
        assert!(loader.is_version_compatible("0.1.0"));
    }

    #[test]
    fn test_loader_default() {
        let loader: PluginLoader = Default::default();
        assert!(!loader.core_version.is_empty());
    }
}
