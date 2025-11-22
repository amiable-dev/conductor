// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin manager for lifecycle management
//!
//! This module provides centralized management of plugins including:
//! - Plugin discovery and loading
//! - Plugin execution with permission checks
//! - Plugin lifecycle management (enable/disable/reload)
//! - Security validation (checksums, capabilities)

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use thiserror::Error;

use conductor_core::plugin::{
    Capability, LoadedPlugin, PluginDiscovery, PluginLoader, PluginLoaderError, PluginMetadata,
    PluginRegistry, TriggerContext,
};
use serde_json::Value;

#[cfg(feature = "plugin-wasm")]
use conductor_core::plugin::wasm_runtime::{WasmConfig, WasmPlugin};

/// Plugin manager error types
#[derive(Debug, Error)]
pub enum PluginManagerError {
    /// Plugin not found
    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),

    /// Plugin loading failed
    #[error("Failed to load plugin: {0}")]
    LoadError(#[from] PluginLoaderError),

    /// Plugin discovery failed
    #[error("Plugin discovery failed: {0}")]
    DiscoveryError(String),

    /// Permission denied
    #[error("Plugin '{plugin}' requires capability '{capability}' which is not granted")]
    PermissionDenied { plugin: String, capability: String },

    /// Plugin execution failed
    #[error("Plugin '{plugin}' execution failed: {reason}")]
    ExecutionError { plugin: String, reason: String },

    /// Plugin disabled
    #[error("Plugin '{0}' is disabled")]
    PluginDisabled(String),

    /// Checksum verification failed
    #[error("Plugin '{plugin}' checksum verification failed: expected {expected}, got {actual}")]
    ChecksumMismatch {
        plugin: String,
        expected: String,
        actual: String,
    },

    /// Lock error
    #[error("Failed to acquire plugin lock: {0}")]
    LockError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Plugin manager result type
pub type PluginManagerResult<T> = Result<T, PluginManagerError>;

/// Plugin execution wrapper (native or WASM)
enum PluginInstance {
    /// Native plugin (.dylib, .so, .dll)
    Native(LoadedPlugin),

    /// WASM plugin (.wasm)
    #[cfg(feature = "plugin-wasm")]
    Wasm(WasmPlugin),
}

impl PluginInstance {
    /// Execute the plugin action (async version for WASM)
    #[allow(dead_code)] // Only used when plugin-wasm feature is enabled
    async fn execute_async(
        &mut self,
        params: Value,
        context: TriggerContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            PluginInstance::Native(plugin) => plugin.plugin.execute(params, context),
            #[cfg(feature = "plugin-wasm")]
            PluginInstance::Wasm(plugin) => {
                // Extract action name from params
                let action = params
                    .get("action")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing 'action' field in params")?;

                plugin
                    .execute(action, &context)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    /// Synchronous execute wrapper
    fn execute_sync(
        &mut self,
        params: Value,
        context: TriggerContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "plugin-wasm")]
        {
            // For WASM plugins, we need to use a runtime
            if matches!(self, PluginInstance::Wasm(_)) {
                return tokio::runtime::Runtime::new()
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
                    .block_on(self.execute_async(params, context));
            }
        }

        // For native plugins, execute synchronously
        match self {
            PluginInstance::Native(plugin) => plugin.plugin.execute(params, context),
            #[cfg(feature = "plugin-wasm")]
            _ => unreachable!(),
        }
    }
}

/// Loaded plugin with execution state
struct ManagedPlugin {
    /// Loaded plugin instance (native or WASM)
    plugin: PluginInstance,

    /// Plugin metadata (for quick access without locking plugin)
    metadata: PluginMetadata,

    /// Whether plugin is currently enabled
    enabled: bool,

    /// Granted capabilities (subset of requested capabilities)
    granted_capabilities: Vec<Capability>,

    /// Execution statistics
    stats: PluginStats,
}

/// Plugin execution statistics
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginStats {
    /// Total executions
    pub executions: u64,

    /// Failed executions
    pub failures: u64,

    /// Last execution timestamp (milliseconds since epoch)
    pub last_execution_ms: u64,

    /// Average execution time (microseconds)
    pub avg_execution_time_us: u64,
}

/// Plugin manager
///
/// Manages all plugins in the system. Thread-safe with interior mutability
/// using RwLock for concurrent access.
///
/// # Example
///
/// ```no_run
/// use conductor_daemon::plugin_manager::PluginManager;
/// use std::path::PathBuf;
///
/// let plugins_dir = PathBuf::from("~/.midimon/plugins");
/// let mut manager = PluginManager::new(plugins_dir);
///
/// // Discover and load plugins
/// manager.discover_plugins()?;
///
/// // Execute plugin action
/// let params = serde_json::json!({
///     "url": "https://api.example.com/notify",
///     "method": "POST"
/// });
/// manager.execute_plugin("http_request", params, None)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct PluginManager {
    /// Plugins directory (~/.midimon/plugins/)
    plugins_dir: PathBuf,

    /// Plugin loader
    loader: PluginLoader,

    /// Loaded plugins indexed by name
    plugins: Arc<RwLock<HashMap<String, ManagedPlugin>>>,

    /// Plugin registry (available but not loaded)
    registry: Arc<RwLock<Option<PluginRegistry>>>,

    /// Auto-grant safe capabilities (Network, Audio, Midi)
    auto_grant_safe: bool,
}

impl PluginManager {
    /// Create a new plugin manager
    ///
    /// # Arguments
    ///
    /// * `plugins_dir` - Path to plugins directory (e.g., ~/.midimon/plugins/)
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self {
            plugins_dir,
            loader: PluginLoader::new(),
            plugins: Arc::new(RwLock::new(HashMap::new())),
            registry: Arc::new(RwLock::new(None)),
            auto_grant_safe: true, // Auto-grant safe capabilities by default
        }
    }

    /// Set whether to auto-grant safe capabilities
    ///
    /// Safe capabilities: Network, Audio, Midi (risk level: Low)
    /// Dangerous capabilities: Filesystem, Subprocess, SystemControl (require explicit grant)
    pub fn set_auto_grant_safe(&mut self, auto_grant: bool) {
        self.auto_grant_safe = auto_grant;
    }

    /// Discover available plugins
    ///
    /// Scans the plugins directory for plugin manifests and builds a registry.
    /// Does not load plugins - use `load_plugin()` to load specific plugins.
    ///
    /// # Errors
    ///
    /// Returns error if directory scanning fails or manifests are invalid.
    pub fn discover_plugins(&mut self) -> PluginManagerResult<usize> {
        let discovery = PluginDiscovery::new(self.plugins_dir.clone());
        let registry = discovery
            .scan()
            .map_err(|e| PluginManagerError::DiscoveryError(e.to_string()))?;

        let count = registry.count();

        // Update registry
        let mut reg = self
            .registry
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;
        *reg = Some(registry);

        Ok(count)
    }

    /// Get list of available plugin names
    pub fn list_available(&self) -> PluginManagerResult<Vec<String>> {
        let reg = self
            .registry
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(registry) = reg.as_ref() {
            Ok(registry.list_names())
        } else {
            Ok(Vec::new())
        }
    }

    /// Get list of loaded plugin names
    pub fn list_loaded(&self) -> PluginManagerResult<Vec<String>> {
        let plugins = self
            .plugins
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        Ok(plugins.keys().cloned().collect())
    }

    /// Get plugin metadata
    pub fn get_metadata(&self, plugin_name: &str) -> PluginManagerResult<PluginMetadata> {
        let plugins = self
            .plugins
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get(plugin_name) {
            return Ok(managed.metadata.clone());
        }

        // Check registry if not loaded
        let reg = self
            .registry
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(registry) = reg.as_ref()
            && let Some(metadata) = registry.get(plugin_name)
        {
            return Ok(metadata.clone());
        }

        Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
    }

    /// Load a plugin by name
    ///
    /// Loads the plugin binary and initializes it. Capabilities are granted
    /// based on auto_grant_safe setting.
    ///
    /// Supports both native (.dylib/.so/.dll) and WASM (.wasm) plugins.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Plugin not found in registry
    /// - Plugin binary cannot be loaded
    /// - Plugin initialization fails
    pub fn load_plugin(&mut self, plugin_name: &str) -> PluginManagerResult<()> {
        // Get metadata from registry
        let reg = self
            .registry
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        let registry = reg
            .as_ref()
            .ok_or_else(|| PluginManagerError::PluginNotFound(plugin_name.to_string()))?;

        let metadata = registry
            .get(plugin_name)
            .ok_or_else(|| PluginManagerError::PluginNotFound(plugin_name.to_string()))?
            .clone(); // Clone metadata to release lock

        drop(reg); // Release read lock

        let binary_path = metadata.binary_path.clone();

        // Verify checksum if present in metadata
        Self::verify_checksum(&binary_path, &metadata.checksum)?;

        // Detect plugin type based on file extension
        let is_wasm = binary_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "wasm")
            .unwrap_or(false);

        let plugin_instance = if is_wasm {
            #[cfg(feature = "plugin-wasm")]
            {
                // Load WASM plugin
                let config = WasmConfig {
                    max_memory_bytes: 128 * 1024 * 1024, // 128 MB
                    max_execution_time: std::time::Duration::from_secs(5),
                    max_fuel: 100_000_000, // 100M instructions
                    capabilities: metadata.capabilities.clone(),
                    #[cfg(feature = "plugin-signing")]
                    require_signature: false, // Backward compatible - don't require signatures
                    #[cfg(feature = "plugin-signing")]
                    allow_self_signed: true, // Allow self-signed plugins
                    #[cfg(feature = "plugin-signing")]
                    trusted_keys_path: None, // Use default path
                };

                let mut wasm_plugin = tokio::runtime::Runtime::new()
                    .map_err(|e| {
                        PluginManagerError::LoadError(PluginLoaderError::LoadError(e.to_string()))
                    })?
                    .block_on(WasmPlugin::load(&binary_path, config))
                    .map_err(|e| {
                        PluginManagerError::LoadError(PluginLoaderError::LoadError(e.to_string()))
                    })?;

                // Initialize WASM plugin
                tokio::runtime::Runtime::new()
                    .map_err(|e| {
                        PluginManagerError::LoadError(PluginLoaderError::LoadError(e.to_string()))
                    })?
                    .block_on(wasm_plugin.init())
                    .map_err(|e| PluginManagerError::ExecutionError {
                        plugin: plugin_name.to_string(),
                        reason: format!("WASM initialization failed: {}", e),
                    })?;

                PluginInstance::Wasm(wasm_plugin)
            }
            #[cfg(not(feature = "plugin-wasm"))]
            {
                return Err(PluginManagerError::LoadError(PluginLoaderError::LoadError(
                    "WASM plugins not supported (compile with --features plugin-wasm)".to_string(),
                )));
            }
        } else {
            // Load native plugin
            let mut loaded_plugin = self.loader.load_plugin(&binary_path)?;

            // Initialize plugin
            loaded_plugin
                .plugin
                .initialize()
                .map_err(|e| PluginManagerError::ExecutionError {
                    plugin: plugin_name.to_string(),
                    reason: format!("Initialization failed: {}", e),
                })?;

            PluginInstance::Native(loaded_plugin)
        };

        // Determine granted capabilities
        let granted_capabilities = if self.auto_grant_safe {
            metadata
                .capabilities
                .iter()
                .filter(|cap| cap.risk_level().is_safe())
                .cloned()
                .collect()
        } else {
            Vec::new() // No capabilities granted by default
        };

        // Create managed plugin
        let managed = ManagedPlugin {
            plugin: plugin_instance,
            metadata: metadata.clone(),
            enabled: true,
            granted_capabilities,
            stats: PluginStats::default(),
        };

        // Store in plugins map
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        plugins.insert(plugin_name.to_string(), managed);

        Ok(())
    }

    /// Unload a plugin
    ///
    /// Calls shutdown() on the plugin and removes it from memory.
    pub fn unload_plugin(&mut self, plugin_name: &str) -> PluginManagerResult<()> {
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.remove(plugin_name) {
            // Call shutdown (only for native plugins)
            match managed.plugin {
                PluginInstance::Native(mut plugin) => {
                    let _ = plugin.plugin.shutdown(); // Ignore shutdown errors
                }
                #[cfg(feature = "plugin-wasm")]
                PluginInstance::Wasm(_) => {
                    // WASM plugins don't need explicit shutdown
                }
            }
            Ok(())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Enable a plugin
    pub fn enable_plugin(&mut self, plugin_name: &str) -> PluginManagerResult<()> {
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get_mut(plugin_name) {
            managed.enabled = true;
            Ok(())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Disable a plugin
    pub fn disable_plugin(&mut self, plugin_name: &str) -> PluginManagerResult<()> {
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get_mut(plugin_name) {
            managed.enabled = false;
            Ok(())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Grant a capability to a plugin
    pub fn grant_capability(
        &mut self,
        plugin_name: &str,
        capability: Capability,
    ) -> PluginManagerResult<()> {
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get_mut(plugin_name) {
            if !managed.granted_capabilities.contains(&capability) {
                managed.granted_capabilities.push(capability);
            }
            Ok(())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Revoke a capability from a plugin
    pub fn revoke_capability(
        &mut self,
        plugin_name: &str,
        capability: &Capability,
    ) -> PluginManagerResult<()> {
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get_mut(plugin_name) {
            managed.granted_capabilities.retain(|c| c != capability);
            Ok(())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Execute a plugin action
    ///
    /// # Arguments
    ///
    /// * `plugin_name` - Name of the plugin to execute
    /// * `params` - Plugin-specific parameters (JSON object)
    /// * `context` - Optional trigger context (velocity, mode, etc.)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Plugin not found or disabled
    /// - Plugin requires capabilities that haven't been granted
    /// - Plugin execution fails
    pub fn execute_plugin(
        &mut self,
        plugin_name: &str,
        params: Value,
        context: Option<TriggerContext>,
    ) -> PluginManagerResult<()> {
        let start = std::time::Instant::now();

        // Get plugin (read lock)
        let mut plugins = self
            .plugins
            .write()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        let managed = plugins
            .get_mut(plugin_name)
            .ok_or_else(|| PluginManagerError::PluginNotFound(plugin_name.to_string()))?;

        // Check if enabled
        if !managed.enabled {
            return Err(PluginManagerError::PluginDisabled(plugin_name.to_string()));
        }

        // Check capabilities
        for required_cap in &managed.metadata.capabilities {
            if !managed.granted_capabilities.contains(required_cap) {
                return Err(PluginManagerError::PermissionDenied {
                    plugin: plugin_name.to_string(),
                    capability: required_cap.name().to_string(),
                });
            }
        }

        // Execute plugin (synchronous wrapper for both native and WASM)
        let result = managed
            .plugin
            .execute_sync(params, context.unwrap_or_default());

        // Update statistics
        managed.stats.executions += 1;
        let elapsed_us = start.elapsed().as_micros() as u64;

        if result.is_err() {
            managed.stats.failures += 1;
        }

        // Update average execution time (running average)
        if managed.stats.executions == 1 {
            managed.stats.avg_execution_time_us = elapsed_us;
        } else {
            managed.stats.avg_execution_time_us =
                (managed.stats.avg_execution_time_us * (managed.stats.executions - 1) + elapsed_us)
                    / managed.stats.executions;
        }

        managed.stats.last_execution_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Return result
        result.map_err(|e| PluginManagerError::ExecutionError {
            plugin: plugin_name.to_string(),
            reason: e.to_string(),
        })
    }

    /// Get plugin statistics
    pub fn get_stats(&self, plugin_name: &str) -> PluginManagerResult<PluginStats> {
        let plugins = self
            .plugins
            .read()
            .map_err(|e| PluginManagerError::LockError(e.to_string()))?;

        if let Some(managed) = plugins.get(plugin_name) {
            Ok(managed.stats.clone())
        } else {
            Err(PluginManagerError::PluginNotFound(plugin_name.to_string()))
        }
    }

    /// Get plugins directory path
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }

    /// Calculate SHA256 checksum of a file
    ///
    /// Returns hex-encoded checksum string
    fn calculate_checksum(path: &Path) -> PluginManagerResult<String> {
        let bytes = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    /// Verify plugin binary checksum
    ///
    /// Compares the calculated checksum against the metadata checksum.
    /// If metadata has no checksum (empty string), verification is skipped.
    fn verify_checksum(binary_path: &Path, expected_checksum: &str) -> PluginManagerResult<()> {
        // Skip verification if no checksum in metadata
        if expected_checksum.is_empty() {
            return Ok(());
        }

        let actual_checksum = Self::calculate_checksum(binary_path)?;

        if actual_checksum != expected_checksum {
            return Err(PluginManagerError::ChecksumMismatch {
                plugin: binary_path.display().to_string(),
                expected: expected_checksum.to_string(),
                actual: actual_checksum,
            });
        }

        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        let plugins_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".midimon")
            .join("plugins");

        Self::new(plugins_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let plugins_dir = PathBuf::from("/tmp/plugins");
        let manager = PluginManager::new(plugins_dir.clone());

        assert_eq!(manager.plugins_dir(), plugins_dir.as_path());
    }

    #[test]
    fn test_plugin_manager_default() {
        let manager = PluginManager::default();
        assert!(manager.plugins_dir().to_string_lossy().contains(".midimon"));
    }

    #[test]
    fn test_list_available_empty() {
        let manager = PluginManager::default();
        let available = manager.list_available().unwrap();
        assert_eq!(available.len(), 0);
    }

    #[test]
    fn test_list_loaded_empty() {
        let manager = PluginManager::default();
        let loaded = manager.list_loaded().unwrap();
        assert_eq!(loaded.len(), 0);
    }

    #[test]
    fn test_get_metadata_not_found() {
        let manager = PluginManager::default();
        let result = manager.get_metadata("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_plugin_not_found() {
        let mut manager = PluginManager::default();
        let result = manager.load_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_plugin_not_found() {
        let mut manager = PluginManager::default();
        let result = manager.unload_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_plugin_not_found() {
        let mut manager = PluginManager::default();
        let result = manager.enable_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_disable_plugin_not_found() {
        let mut manager = PluginManager::default();
        let result = manager.disable_plugin("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_plugin_not_found() {
        let mut manager = PluginManager::default();
        let result = manager.execute_plugin("nonexistent", Value::Null, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_grant_safe() {
        let mut manager = PluginManager::default();
        assert!(manager.auto_grant_safe);

        manager.set_auto_grant_safe(false);
        assert!(!manager.auto_grant_safe);
    }
}
