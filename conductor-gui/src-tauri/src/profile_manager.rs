// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Profile Management System
//!
//! Manages per-app configuration profiles with caching and automatic switching.
//! Profiles allow different MIDI mappings for different applications.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Application profile containing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProfile {
    /// Unique profile identifier
    pub id: String,

    /// Display name for the profile
    pub name: String,

    /// Application bundle IDs this profile applies to
    pub bundle_ids: Vec<String>,

    /// Path to the configuration file
    pub config_path: PathBuf,

    /// Last modified timestamp
    #[serde(skip)]
    #[allow(dead_code)] // Stored for cache invalidation and file watching
    pub last_modified: Option<std::time::SystemTime>,

    /// Whether this is the default/fallback profile
    pub is_default: bool,
}

/// Profile cache entry
struct CachedProfile {
    #[allow(dead_code)] // Stored for future metadata access
    profile: AppProfile,
    config_content: String,
    cached_at: Instant,
}

/// Profile switching result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchResult {
    /// Whether the switch was successful
    pub success: bool,

    /// The profile that was activated
    pub profile_id: Option<String>,

    /// Error message if switch failed
    pub error: Option<String>,

    /// Whether the profile was loaded from cache
    pub from_cache: bool,
}

/// Profile manager state
pub struct ProfileManager {
    /// All registered profiles
    profiles: Arc<RwLock<HashMap<String, AppProfile>>>,

    /// Bundle ID to profile ID mapping
    bundle_map: Arc<RwLock<HashMap<String, String>>>,

    /// Currently active profile
    active_profile: Arc<RwLock<Option<String>>>,

    /// Profile cache
    cache: Arc<RwLock<HashMap<String, CachedProfile>>>,

    /// Cache TTL in seconds
    cache_ttl_secs: u64,

    /// Profiles directory path
    profiles_dir: PathBuf,
}

impl ProfileManager {
    /// Create a new profile manager with default profiles directory
    pub fn new() -> std::io::Result<Self> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to determine config directory",
            )
        })?;

        let profiles_dir = config_dir.join("midimon").join("profiles");

        // Create profiles directory if it doesn't exist
        if !profiles_dir.exists() {
            std::fs::create_dir_all(&profiles_dir)?;
        }

        Ok(Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            bundle_map: Arc::new(RwLock::new(HashMap::new())),
            active_profile: Arc::new(RwLock::new(None)),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl_secs: 300, // 5 minutes default TTL
            profiles_dir,
        })
    }

    /// Create with custom profiles directory
    #[allow(dead_code)] // Part of public API, used in tests
    pub fn with_directory(path: PathBuf) -> std::io::Result<Self> {
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        Ok(Self {
            profiles: Arc::new(RwLock::new(HashMap::new())),
            bundle_map: Arc::new(RwLock::new(HashMap::new())),
            active_profile: Arc::new(RwLock::new(None)),
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl_secs: 300,
            profiles_dir: path,
        })
    }

    /// Register a new profile
    pub async fn register_profile(&self, profile: AppProfile) -> Result<(), String> {
        let profile_id = profile.id.clone();

        // Add to profiles map
        self.profiles
            .write()
            .await
            .insert(profile_id.clone(), profile.clone());

        // Update bundle ID mappings
        let mut bundle_map = self.bundle_map.write().await;
        for bundle_id in &profile.bundle_ids {
            bundle_map.insert(bundle_id.clone(), profile_id.clone());
        }

        Ok(())
    }

    /// Get profile by ID
    pub async fn get_profile(&self, profile_id: &str) -> Option<AppProfile> {
        self.profiles.read().await.get(profile_id).cloned()
    }

    /// Get all registered profiles
    pub async fn list_profiles(&self) -> Vec<AppProfile> {
        self.profiles.read().await.values().cloned().collect()
    }

    /// Find profile for a bundle ID
    pub async fn find_profile_for_app(&self, bundle_id: &str) -> Option<AppProfile> {
        let bundle_map = self.bundle_map.read().await;
        let profile_id = bundle_map.get(bundle_id)?;

        self.profiles.read().await.get(profile_id).cloned()
    }

    /// Get the default profile
    pub async fn get_default_profile(&self) -> Option<AppProfile> {
        self.profiles
            .read()
            .await
            .values()
            .find(|p| p.is_default)
            .cloned()
    }

    /// Load config content for a profile (with caching)
    pub async fn load_profile_config(&self, profile_id: &str) -> Result<String, String> {
        // Check cache first
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(profile_id) {
            let age = cached.cached_at.elapsed();
            if age < Duration::from_secs(self.cache_ttl_secs) {
                return Ok(cached.config_content.clone());
            }
        }
        drop(cache);

        // Load from file
        let profile = self
            .get_profile(profile_id)
            .await
            .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

        let content = std::fs::read_to_string(&profile.config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(
            profile_id.to_string(),
            CachedProfile {
                profile,
                config_content: content.clone(),
                cached_at: Instant::now(),
            },
        );

        Ok(content)
    }

    /// Switch to a profile
    pub async fn switch_to_profile(&self, profile_id: &str) -> Result<SwitchResult, String> {
        // Check if profile exists
        let _profile = self
            .get_profile(profile_id)
            .await
            .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

        // Load config
        let from_cache = {
            let cache = self.cache.read().await;
            cache.contains_key(profile_id)
        };

        match self.load_profile_config(profile_id).await {
            Ok(_config_content) => {
                // Update active profile
                *self.active_profile.write().await = Some(profile_id.to_string());

                // TODO: Send config to daemon for hot-reload
                // This will be implemented when integrating with daemon IPC

                Ok(SwitchResult {
                    success: true,
                    profile_id: Some(profile_id.to_string()),
                    error: None,
                    from_cache,
                })
            }
            Err(e) => Ok(SwitchResult {
                success: false,
                profile_id: None,
                error: Some(e),
                from_cache: false,
            }),
        }
    }

    /// Switch profile based on app bundle ID
    pub async fn switch_for_app(&self, bundle_id: &str) -> Result<SwitchResult, String> {
        // Find profile for app
        let profile = self.find_profile_for_app(bundle_id).await;

        let profile_id = if let Some(p) = profile {
            p.id
        } else {
            // Fall back to default profile
            self.get_default_profile()
                .await
                .ok_or_else(|| "No default profile configured".to_string())?
                .id
        };

        self.switch_to_profile(&profile_id).await
    }

    /// Get currently active profile ID
    pub async fn get_active_profile_id(&self) -> Option<String> {
        self.active_profile.read().await.clone()
    }

    /// Clear the profile cache
    pub async fn clear_cache(&self) {
        self.cache.write().await.clear();
    }

    /// Invalidate specific profile in cache
    #[allow(dead_code)] // Part of cache management API
    pub async fn invalidate_cache(&self, profile_id: &str) {
        self.cache.write().await.remove(profile_id);
    }

    /// Scan profiles directory and auto-register profiles
    pub async fn scan_profiles_directory(&self) -> Result<usize, String> {
        let entries = std::fs::read_dir(&self.profiles_dir)
            .map_err(|e| format!("Failed to read profiles directory: {}", e))?;

        let mut count = 0;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match self.load_profile_from_file(&path).await {
                    Ok(profile) => {
                        self.register_profile(profile).await?;
                        count += 1;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load profile from {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(count)
    }

    /// Load profile metadata from file
    async fn load_profile_from_file(&self, path: &Path) -> Result<AppProfile, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

        let meta: ProfileMetadata =
            toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))?;

        let metadata =
            std::fs::metadata(path).map_err(|e| format!("Failed to read file metadata: {}", e))?;

        Ok(AppProfile {
            id: meta.id.unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            }),
            name: meta.name,
            bundle_ids: meta.bundle_ids,
            config_path: path.to_path_buf(),
            last_modified: metadata.modified().ok(),
            is_default: meta.is_default.unwrap_or(false),
        })
    }

    /// Get profiles directory path
    #[allow(dead_code)] // Part of public API for directory access
    pub fn get_profiles_directory(&self) -> &Path {
        &self.profiles_dir
    }

    /// Export profile to JSON
    pub async fn export_profile_json(&self, profile_id: &str) -> Result<String, String> {
        let profile = self
            .get_profile(profile_id)
            .await
            .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

        serde_json::to_string_pretty(&profile)
            .map_err(|e| format!("Failed to serialize profile: {}", e))
    }

    /// Import profile from JSON
    pub async fn import_profile_json(&self, json: &str) -> Result<String, String> {
        let mut profile: AppProfile =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Generate new ID if conflicts
        let original_id = profile.id.clone();
        let mut counter = 1;
        while self.get_profile(&profile.id).await.is_some() {
            profile.id = format!("{}-{}", original_id, counter);
            counter += 1;
        }

        self.register_profile(profile.clone()).await?;
        Ok(profile.id)
    }

    /// Export profile to TOML file
    pub async fn export_profile_toml(
        &self,
        profile_id: &str,
        output_path: &Path,
    ) -> Result<(), String> {
        let _profile = self
            .get_profile(profile_id)
            .await
            .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

        // Load config content
        let config_content = self.load_profile_config(profile_id).await?;

        // Write to file
        std::fs::write(output_path, config_content)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    /// Import profile from TOML file
    pub async fn import_profile_toml(
        &self,
        path: &Path,
        name: Option<String>,
    ) -> Result<String, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

        // Parse TOML to extract metadata
        let config: toml::Value =
            toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))?;

        // Extract bundle IDs from config if available
        let bundle_ids = if let Some(metadata) = config.get("metadata") {
            if let Some(ids) = metadata.get("bundle_ids") {
                ids.as_array().and_then(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                        .into()
                })
            } else {
                None
            }
        } else {
            None
        }
        .unwrap_or_default();

        // Generate profile
        let profile_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("imported")
            .to_string();

        let profile_name = name.unwrap_or_else(|| profile_id.clone());

        // Copy file to profiles directory
        let dest_path = self.profiles_dir.join(format!("{}.toml", profile_id));
        std::fs::copy(path, &dest_path).map_err(|e| format!("Failed to copy file: {}", e))?;

        let profile = AppProfile {
            id: profile_id.clone(),
            name: profile_name,
            bundle_ids,
            config_path: dest_path,
            last_modified: None,
            is_default: false,
        };

        self.register_profile(profile).await?;
        Ok(profile_id)
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default profile manager")
    }
}

/// Profile metadata from TOML file
#[derive(Debug, Deserialize)]
struct ProfileMetadata {
    id: Option<String>,
    name: String,
    bundle_ids: Vec<String>,
    is_default: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_profile_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProfileManager::with_directory(temp_dir.path().to_path_buf()).unwrap();

        assert_eq!(manager.list_profiles().await.len(), 0);
        assert!(manager.get_active_profile_id().await.is_none());
    }

    #[tokio::test]
    async fn test_profile_registration() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProfileManager::with_directory(temp_dir.path().to_path_buf()).unwrap();

        let profile = AppProfile {
            id: "test-profile".to_string(),
            name: "Test Profile".to_string(),
            bundle_ids: vec!["com.example.app".to_string()],
            config_path: temp_dir.path().join("test.toml"),
            last_modified: None,
            is_default: false,
        };

        manager.register_profile(profile.clone()).await.unwrap();

        assert_eq!(manager.list_profiles().await.len(), 1);
        let found = manager.find_profile_for_app("com.example.app").await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "test-profile");
    }

    #[tokio::test]
    async fn test_default_profile() {
        let temp_dir = TempDir::new().unwrap();
        let manager = ProfileManager::with_directory(temp_dir.path().to_path_buf()).unwrap();

        let default_profile = AppProfile {
            id: "default".to_string(),
            name: "Default".to_string(),
            bundle_ids: vec![],
            config_path: temp_dir.path().join("default.toml"),
            last_modified: None,
            is_default: true,
        };

        manager.register_profile(default_profile).await.unwrap();

        let found = manager.get_default_profile().await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "default");
    }
}
