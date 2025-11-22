// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Tauri commands for plugin management

use crate::state::AppState;
use conductor_core::plugin::{Capability, PluginMetadata};
use conductor_daemon::plugin_manager::PluginStats;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Plugin metadata for UI (serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadataJson {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub homepage: Option<String>,
    pub license: String,
    pub plugin_type: String,
    pub capabilities: Vec<CapabilityJson>,
    pub enabled: bool,
}

/// Capability info for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityJson {
    pub name: String,
    pub description: String,
    pub risk_level: String,
}

/// Convert Capability to JSON format
fn capability_to_json(cap: &Capability) -> CapabilityJson {
    CapabilityJson {
        name: cap.name().to_string(),
        description: cap.description().to_string(),
        risk_level: format!("{:?}", cap.risk_level()),
    }
}

/// Convert PluginMetadata to JSON format
fn metadata_to_json(metadata: &PluginMetadata) -> PluginMetadataJson {
    PluginMetadataJson {
        name: metadata.name.clone(),
        version: metadata.version.clone(),
        description: metadata.description.clone(),
        author: metadata.author.clone(),
        homepage: metadata.homepage.clone(),
        license: metadata.license.clone(),
        plugin_type: format!("{:?}", metadata.plugin_type),
        capabilities: metadata
            .capabilities
            .iter()
            .map(capability_to_json)
            .collect(),
        enabled: metadata.enabled,
    }
}

/// Discover available plugins
#[tauri::command]
pub async fn plugin_discover(state: State<'_, AppState>) -> Result<usize, String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    let count = pm
        .discover_plugins()
        .map_err(|e| format!("Failed to discover plugins: {}", e))?;

    Ok(count)
}

/// List available plugins
#[tauri::command]
pub async fn plugin_list_available(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let plugin_manager = state.get_plugin_manager().await;
    let pm = plugin_manager.read().await;

    let plugins = pm
        .list_available()
        .map_err(|e| format!("Failed to list available plugins: {}", e))?;

    Ok(plugins)
}

/// List loaded plugins
#[tauri::command]
pub async fn plugin_list_loaded(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let plugin_manager = state.get_plugin_manager().await;
    let pm = plugin_manager.read().await;

    let plugins = pm
        .list_loaded()
        .map_err(|e| format!("Failed to list loaded plugins: {}", e))?;

    Ok(plugins)
}

/// Get plugin metadata
#[tauri::command]
pub async fn plugin_get_metadata(
    plugin_name: String,
    state: State<'_, AppState>,
) -> Result<PluginMetadataJson, String> {
    let plugin_manager = state.get_plugin_manager().await;
    let pm = plugin_manager.read().await;

    let metadata = pm
        .get_metadata(&plugin_name)
        .map_err(|e| format!("Failed to get metadata for {}: {}", plugin_name, e))?;

    Ok(metadata_to_json(&metadata))
}

/// Load a plugin
#[tauri::command]
pub async fn plugin_load(plugin_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    pm.load_plugin(&plugin_name)
        .map_err(|e| format!("Failed to load plugin {}: {}", plugin_name, e))?;

    Ok(())
}

/// Unload a plugin
#[tauri::command]
pub async fn plugin_unload(plugin_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    pm.unload_plugin(&plugin_name)
        .map_err(|e| format!("Failed to unload plugin {}: {}", plugin_name, e))?;

    Ok(())
}

/// Enable a plugin
#[tauri::command]
pub async fn plugin_enable(plugin_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    pm.enable_plugin(&plugin_name)
        .map_err(|e| format!("Failed to enable plugin {}: {}", plugin_name, e))?;

    Ok(())
}

/// Disable a plugin
#[tauri::command]
pub async fn plugin_disable(plugin_name: String, state: State<'_, AppState>) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    pm.disable_plugin(&plugin_name)
        .map_err(|e| format!("Failed to disable plugin {}: {}", plugin_name, e))?;

    Ok(())
}

/// Grant a capability to a plugin
#[tauri::command]
pub async fn plugin_grant_capability(
    plugin_name: String,
    capability: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    // Parse capability string to enum
    let cap = match capability.as_str() {
        "Network Access" => Capability::Network,
        "Filesystem Access" => Capability::Filesystem,
        "Audio Device Access" => Capability::Audio,
        "MIDI Device Access" => Capability::Midi,
        "Process Execution" => Capability::Subprocess,
        "System Control" => Capability::SystemControl,
        _ => return Err(format!("Unknown capability: {}", capability)),
    };

    pm.grant_capability(&plugin_name, cap)
        .map_err(|e| format!("Failed to grant {} to {}: {}", capability, plugin_name, e))?;

    Ok(())
}

/// Revoke a capability from a plugin
#[tauri::command]
pub async fn plugin_revoke_capability(
    plugin_name: String,
    capability: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;

    // Parse capability string to enum
    let cap = match capability.as_str() {
        "Network Access" => Capability::Network,
        "Filesystem Access" => Capability::Filesystem,
        "Audio Device Access" => Capability::Audio,
        "MIDI Device Access" => Capability::Midi,
        "Process Execution" => Capability::Subprocess,
        "System Control" => Capability::SystemControl,
        _ => return Err(format!("Unknown capability: {}", capability)),
    };

    pm.revoke_capability(&plugin_name, &cap).map_err(|e| {
        format!(
            "Failed to revoke {} from {}: {}",
            capability, plugin_name, e
        )
    })?;

    Ok(())
}

/// Get plugin statistics
#[tauri::command]
pub async fn plugin_get_stats(
    plugin_name: String,
    state: State<'_, AppState>,
) -> Result<PluginStats, String> {
    let plugin_manager = state.get_plugin_manager().await;
    let pm = plugin_manager.read().await;

    let stats = pm
        .get_stats(&plugin_name)
        .map_err(|e| format!("Failed to get stats for {}: {}", plugin_name, e))?;

    Ok(stats)
}

// ============================================================================
// Plugin Marketplace Commands
// ============================================================================

#[cfg(feature = "plugin-registry")]
use conductor_core::plugin_registry::{PluginRegistry, PluginRegistryClient};

/// Fetch the plugin registry from GitHub
#[cfg(feature = "plugin-registry")]
#[tauri::command]
pub async fn fetch_plugin_registry() -> Result<PluginRegistry, String> {
    let cache_dir = dirs::cache_dir()
        .ok_or("Failed to get cache directory")?
        .join("midimon")
        .join("plugin-registry");

    let client = PluginRegistryClient::new(
        "https://raw.githubusercontent.com/amiable-dev/midimon-plugin-registry/main/registry.json",
        cache_dir,
    );

    client
        .fetch_registry()
        .await
        .map_err(|e| format!("Failed to fetch plugin registry: {}", e))
}

/// Install a plugin from the registry
#[cfg(feature = "plugin-registry")]
#[tauri::command]
pub async fn install_plugin_from_registry(
    plugin_id: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let cache_dir = dirs::cache_dir()
        .ok_or("Failed to get cache directory")?
        .join("midimon")
        .join("plugin-registry");

    let client = PluginRegistryClient::new(
        "https://raw.githubusercontent.com/amiable-dev/midimon-plugin-registry/main/registry.json",
        cache_dir,
    );

    // Get plugins directory
    let plugins_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("midimon")
        .join("plugins");

    // Create plugins directory if it doesn't exist
    std::fs::create_dir_all(&plugins_dir)
        .map_err(|e| format!("Failed to create plugins directory: {}", e))?;

    // Install the plugin
    let plugin_path = client
        .install_plugin(&plugin_id, &plugins_dir)
        .await
        .map_err(|e| format!("Failed to install plugin {}: {}", plugin_id, e))?;

    // Auto-discover plugins after installation
    let plugin_manager = state.get_plugin_manager().await;
    let mut pm = plugin_manager.write().await;
    let _ = pm.discover_plugins();

    Ok(plugin_path.to_str().unwrap_or("unknown").to_string())
}

/// Uninstall a plugin (delete from filesystem)
#[tauri::command]
pub async fn uninstall_plugin(
    plugin_name: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Unload first if loaded
    let plugin_manager = state.get_plugin_manager().await;
    {
        let mut pm = plugin_manager.write().await;
        let _ = pm.unload_plugin(&plugin_name);
    }

    // Get plugins directory
    let plugins_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("midimon")
        .join("plugins");

    // Find and delete the plugin file
    let plugin_file = plugins_dir.join(format!("libmidimon_{}_plugin.dylib", plugin_name));

    if plugin_file.exists() {
        std::fs::remove_file(&plugin_file)
            .map_err(|e| format!("Failed to delete plugin file: {}", e))?;
    } else {
        // Try other extensions
        let extensions = vec!["so", "dll"];
        for ext in extensions {
            let plugin_file =
                plugins_dir.join(format!("libmidimon_{}_plugin.{}", plugin_name, ext));
            if plugin_file.exists() {
                std::fs::remove_file(&plugin_file)
                    .map_err(|e| format!("Failed to delete plugin file: {}", e))?;
                break;
            }
        }
    }

    // Rediscover plugins
    {
        let mut pm = plugin_manager.write().await;
        let _ = pm.discover_plugins();
    }

    Ok(())
}

/// List installed plugins (from filesystem)
#[tauri::command]
pub async fn list_installed_plugins() -> Result<Vec<String>, String> {
    let plugins_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("midimon")
        .join("plugins");

    if !plugins_dir.exists() {
        return Ok(Vec::new());
    }

    let mut plugins = Vec::new();
    let entries = std::fs::read_dir(&plugins_dir)
        .map_err(|e| format!("Failed to read plugins directory: {}", e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                if let Some(name) = filename.to_str() {
                    // Extract plugin name from libmidimon_<name>_plugin.{dylib,so,dll}
                    if name.starts_with("libmidimon_") && name.contains("_plugin.") {
                        let plugin_name = name
                            .trim_start_matches("libmidimon_")
                            .split("_plugin.")
                            .next()
                            .unwrap_or("")
                            .to_string();
                        if !plugin_name.is_empty() {
                            plugins.push(plugin_name);
                        }
                    }
                }
            }
        }
    }

    Ok(plugins)
}
