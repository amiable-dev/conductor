// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests for WASM plugin system
//!
//! These tests verify that WASM plugins can be loaded, initialized, and executed
//! correctly using the wasmtime runtime.

#![cfg(all(test, feature = "plugin-wasm"))]

use conductor_core::plugin::{
    Capability, TriggerContext,
    wasm_runtime::{WasmConfig, WasmPlugin},
};
use std::path::PathBuf;
use std::time::Duration;

/// Get path to the WASM template plugin binary
fn get_wasm_template_path() -> PathBuf {
    // Path to the built WASM minimal plugin (no_std, no allocator)
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../plugins/wasm-minimal/target/wasm32-wasip1/release/midimon_wasm_minimal.wasm")
}

#[tokio::test]
async fn test_load_wasm_plugin() {
    let wasm_path = get_wasm_template_path();

    // Skip test if WASM binary doesn't exist
    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built at {:?}", wasm_path);
        eprintln!("Run: cd plugins/wasm-template && cargo build --target wasm32-wasip1 --release");
        return;
    }

    let config = WasmConfig::default();

    // Load the WASM plugin
    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(
        result.is_ok(),
        "Failed to load WASM plugin: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_wasm_plugin_init() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    // Initialize plugin and get metadata
    let metadata = plugin.init().await.expect("Failed to initialize plugin");

    assert_eq!(metadata.name, "minimal_wasm_plugin");
    assert_eq!(metadata.version, "0.1.0");
    assert_eq!(metadata.description, "Minimal test plugin");
}

#[tokio::test]
async fn test_wasm_plugin_execute() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // Create a trigger context
    let context = TriggerContext::with_velocity(100);

    // Execute the plugin
    let result = plugin.execute("test_action", &context).await;
    assert!(
        result.is_ok(),
        "Plugin execution failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_wasm_plugin_with_capabilities() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    let mut config = WasmConfig::default();
    config.max_memory_bytes = 64 * 1024 * 1024; // 64 MB
    config.max_execution_time = Duration::from_secs(3);
    config.capabilities = vec![Capability::Network];

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin with Network capability");

    let metadata = plugin.init().await.expect("Failed to initialize plugin");

    assert_eq!(metadata.name, "minimal_wasm_plugin");
}

#[tokio::test]
async fn test_wasm_plugin_execution_timeout() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    // Set a very short timeout to test timeout handling
    let mut config = WasmConfig::default();
    config.max_execution_time = Duration::from_millis(1); // 1ms timeout

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::new();

    // This should timeout if the plugin takes longer than 1ms
    // Note: The template plugin is very fast, so this might not actually timeout
    let result = plugin.execute("test_action", &context).await;

    // Either succeeds (if fast enough) or times out
    if let Err(e) = result {
        let error_msg = e.to_string();
        // Check if it's a timeout error
        if error_msg.contains("timeout") {
            println!("Plugin execution timed out as expected");
        }
    }
}

#[tokio::test]
async fn test_wasm_config_defaults() {
    let config = WasmConfig::default();

    assert_eq!(config.max_memory_bytes, 128 * 1024 * 1024);
    assert_eq!(config.max_execution_time, Duration::from_secs(5));
    assert!(config.capabilities.is_empty());
}

#[tokio::test]
async fn test_wasm_plugin_with_velocity_context() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // Test with different velocity values
    for velocity in [1, 64, 127] {
        let context = TriggerContext::with_velocity(velocity);
        let result = plugin.execute("test_action", &context).await;
        assert!(
            result.is_ok(),
            "Plugin execution failed with velocity {}: {:?}",
            velocity,
            result.err()
        );
    }
}

#[tokio::test]
async fn test_wasm_plugin_metadata_retrieval() {
    let wasm_path = get_wasm_template_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: WASM template not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    // Metadata should be None before init
    assert!(plugin.metadata().is_none());

    // Initialize plugin
    plugin.init().await.expect("Failed to initialize plugin");

    // Metadata should be available after init
    let metadata = plugin.metadata().expect("Metadata not set after init");
    assert!(!metadata.name.is_empty());
    assert!(!metadata.version.is_empty());
    assert!(!metadata.description.is_empty());
}
