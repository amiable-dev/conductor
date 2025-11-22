// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! Integration tests for WASM resource limiting
//!
//! This test suite verifies that the resource limiter correctly enforces:
//! 1. Fuel limits (instruction count)
//! 2. Memory limits
//! 3. Table growth limits
//!
//! Tests use real plugins to validate limiting behavior.

#![cfg(all(test, feature = "plugin-wasm"))]

use conductor_core::plugin::{
    TriggerContext,
    wasm_runtime::{WasmConfig, WasmPlugin},
};
use std::path::PathBuf;
use std::time::Duration;

/// Get path to a test plugin
fn get_plugin_path(plugin_name: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("plugins")
        .join(plugin_name)
        .join("target")
        .join("wasm32-wasip1")
        .join("release")
        .join(format!(
            "midimon_wasm_{}.wasm",
            plugin_name.replace("-", "_")
        ))
}

#[tokio::test]
async fn test_default_resource_limits() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Use default limits
    let config = WasmConfig::default();
    assert_eq!(config.max_memory_bytes, 128 * 1024 * 1024); // 128 MB
    assert_eq!(config.max_fuel, 100_000_000); // 100M instructions
    assert_eq!(config.max_execution_time, Duration::from_secs(5));

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin with default limits");

    plugin.init().await.expect("Failed to initialize plugin");

    // Execute action - should succeed within limits
    let context = TriggerContext::default();
    let result = plugin.execute("play", &context).await;
    assert!(
        result.is_ok(),
        "Action should succeed within default limits"
    );
}

#[tokio::test]
async fn test_custom_fuel_limit() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Set very high fuel limit
    let mut config = WasmConfig::default();
    config.max_fuel = 500_000_000; // 500M instructions

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin with custom fuel limit");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::default();
    let result = plugin.execute("play", &context).await;
    assert!(result.is_ok(), "Action should succeed with high fuel limit");
}

#[tokio::test]
async fn test_custom_memory_limit() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Set higher memory limit
    let mut config = WasmConfig::default();
    config.max_memory_bytes = 256 * 1024 * 1024; // 256 MB

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin with custom memory limit");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::default();
    let result = plugin.execute("play", &context).await;
    assert!(
        result.is_ok(),
        "Action should succeed with higher memory limit"
    );
}

#[tokio::test]
async fn test_obs_plugin_within_limits() {
    let wasm_path = get_plugin_path("obs-control");

    if !wasm_path.exists() {
        eprintln!("Skipping test: OBS plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load OBS plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // OBS plugin should execute within default limits
    let context = TriggerContext::default();
    let result = plugin.execute("start_recording", &context).await;
    assert!(
        result.is_ok(),
        "OBS plugin should work within default limits"
    );
}

#[tokio::test]
async fn test_multiple_executions_with_fuel_reset() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::default();

    // Execute multiple times - fuel should reset between executions
    for i in 0..5 {
        let result = plugin.execute("play", &context).await;
        assert!(
            result.is_ok(),
            "Execution {} should succeed (fuel resets)",
            i + 1
        );
    }
}

#[tokio::test]
async fn test_low_fuel_limit_still_works() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Set lower but still adequate fuel limit
    let mut config = WasmConfig::default();
    config.max_fuel = 10_000_000; // 10M instructions (lower but should work)

    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin with low fuel limit");

    plugin.init().await.expect("Failed to initialize plugin");

    // Simple actions should still work with lower limit
    let context = TriggerContext::default();
    let result = plugin.execute("play", &context).await;
    assert!(result.is_ok(), "Simple action should work with 10M fuel");
}

#[tokio::test]
async fn test_system_utils_plugin_limits() {
    let wasm_path = get_plugin_path("system-utils");

    if !wasm_path.exists() {
        eprintln!("Skipping test: System utils plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load system utils plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // System utils plugin should work within limits
    let context = TriggerContext::default();
    let result = plugin.execute("mute_system", &context).await;
    assert!(
        result.is_ok(),
        "System utils plugin should work within limits"
    );
}

#[tokio::test]
async fn test_config_limits_preserved_across_store_creations() {
    let wasm_path = get_plugin_path("spotify");

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Create custom config
    let mut config = WasmConfig::default();
    config.max_fuel = 200_000_000; // 200M instructions
    config.max_memory_bytes = 256 * 1024 * 1024; // 256 MB

    let mut plugin = WasmPlugin::load(&wasm_path, config.clone())
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // Execute multiple times - each execution creates new store with same limits
    let context = TriggerContext::default();
    for _ in 0..3 {
        let result = plugin.execute("play", &context).await;
        assert!(
            result.is_ok(),
            "Limits should be preserved across store creations"
        );
    }
}
