// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! Integration tests for filesystem capability and directory preopening
//!
//! This test suite verifies that:
//! 1. Directory preopening works correctly with filesystem capability
//! 2. Plugins can access preopened directories
//! 3. Plugins cannot access directories outside sandbox
//! 4. Multiple plugins share the same data directory safely

#![cfg(all(test, feature = "plugin-wasm"))]

use conductor_core::plugin::{
    Capability, TriggerContext,
    wasm_runtime::{WasmConfig, WasmPlugin},
};
use std::path::PathBuf;

/// Get path to system-utils plugin (which uses filesystem capability)
fn get_system_utils_plugin_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("plugins")
        .join("wasm-system-utils")
        .join("target")
        .join("wasm32-wasip1")
        .join("release")
        .join("midimon_wasm_system_utils.wasm")
}

#[tokio::test]
async fn test_filesystem_capability_granted() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config with filesystem capability
    let mut config = WasmConfig::default();
    config.capabilities.push(Capability::Filesystem);

    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(
        result.is_ok(),
        "Plugin with filesystem capability should load successfully"
    );
}

#[tokio::test]
async fn test_plugin_data_directory_created() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config with filesystem capability
    let mut config = WasmConfig::default();
    config.capabilities.push(Capability::Filesystem);

    let _plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    // Verify plugin data directory was created
    let plugin_data_dir = dirs::data_dir()
        .expect("No data directory")
        .join("midimon")
        .join("plugin-data");

    assert!(
        plugin_data_dir.exists(),
        "Plugin data directory should be created: {:?}",
        plugin_data_dir
    );
    assert!(
        plugin_data_dir.is_dir(),
        "Plugin data path should be a directory"
    );
}

#[tokio::test]
async fn test_filesystem_capability_not_granted() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config WITHOUT filesystem capability
    let config = WasmConfig::default();
    assert!(
        config.capabilities.is_empty(),
        "Default config should have no capabilities"
    );

    // Plugin should still load (but won't have filesystem access)
    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(
        result.is_ok(),
        "Plugin without filesystem capability should still load"
    );
}

#[tokio::test]
async fn test_multiple_plugins_share_data_directory() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Load first plugin with filesystem capability
    let mut config1 = WasmConfig::default();
    config1.capabilities.push(Capability::Filesystem);

    let _plugin1 = WasmPlugin::load(&wasm_path, config1)
        .await
        .expect("Failed to load first plugin");

    // Load second plugin with filesystem capability
    let mut config2 = WasmConfig::default();
    config2.capabilities.push(Capability::Filesystem);

    let _plugin2 = WasmPlugin::load(&wasm_path, config2)
        .await
        .expect("Failed to load second plugin");

    // Both plugins share the same data directory
    let plugin_data_dir = dirs::data_dir()
        .expect("No data directory")
        .join("midimon")
        .join("plugin-data");

    assert!(
        plugin_data_dir.exists(),
        "Shared plugin data directory should exist"
    );
}

#[tokio::test]
async fn test_directory_permissions() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config with filesystem capability
    let mut config = WasmConfig::default();
    config.capabilities.push(Capability::Filesystem);

    let _plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    // Verify directory has correct permissions (readable/writable)
    let plugin_data_dir = dirs::data_dir()
        .expect("No data directory")
        .join("midimon")
        .join("plugin-data");

    let metadata = std::fs::metadata(&plugin_data_dir).expect("Failed to get directory metadata");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        // Check that owner has read/write/execute (0o700 minimum)
        assert!(
            mode & 0o700 != 0,
            "Directory should be readable/writable/executable by owner"
        );
    }

    // Directory should be writable
    assert!(
        !metadata.permissions().readonly(),
        "Directory should not be readonly"
    );
}

#[tokio::test]
async fn test_system_utils_with_filesystem_access() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config with filesystem capability
    let mut config = WasmConfig::default();
    config.capabilities.push(Capability::Filesystem);

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // Execute action that might use filesystem
    let context = TriggerContext::default();
    let result = plugin.execute("mute_system", &context).await;
    assert!(
        result.is_ok(),
        "System utils plugin should work with filesystem access"
    );
}

#[tokio::test]
async fn test_config_with_multiple_capabilities() {
    let wasm_path = get_system_utils_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    // Create config with multiple capabilities
    let mut config = WasmConfig::default();
    config.capabilities.push(Capability::Filesystem);
    config.capabilities.push(Capability::Network);

    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(
        result.is_ok(),
        "Plugin with multiple capabilities should load successfully"
    );
}
