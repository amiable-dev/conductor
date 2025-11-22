// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration test for Spotify WASM plugin

#![cfg(all(test, feature = "plugin-wasm"))]

use conductor_core::plugin::{
    TriggerContext,
    wasm_runtime::{WasmConfig, WasmPlugin},
};
use std::path::PathBuf;

/// Get path to the Spotify WASM plugin binary
fn get_spotify_plugin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../plugins/wasm-spotify/target/wasm32-wasip1/release/midimon_wasm_spotify.wasm")
}

#[tokio::test]
async fn test_load_spotify_plugin() {
    let wasm_path = get_spotify_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built at {:?}", wasm_path);
        eprintln!("Run: cd plugins/wasm-spotify && cargo build --target wasm32-wasip1 --release");
        return;
    }

    let config = WasmConfig::default();
    let result = WasmPlugin::load(&wasm_path, config).await;
    assert!(
        result.is_ok(),
        "Failed to load Spotify plugin: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_spotify_plugin_metadata() {
    let wasm_path = get_spotify_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    let metadata = plugin.init().await.expect("Failed to initialize plugin");

    assert_eq!(metadata.name, "spotify_control");
    assert_eq!(metadata.version, "0.2.0");
    assert_eq!(metadata.author, "Amiable Team");
    assert_eq!(metadata.license, "MIT");
    assert!(metadata.description.contains("Spotify"));

    // Verify capabilities (Network capability required for Spotify Web API)
    assert!(
        !metadata.capabilities.is_empty(),
        "Spotify plugin should request Network capability"
    );
}

#[tokio::test]
async fn test_spotify_play_pause_action() {
    let wasm_path = get_spotify_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::new();
    let result = plugin.execute("play_pause", &context).await;

    assert!(
        result.is_ok(),
        "play_pause action failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_spotify_all_actions() {
    let wasm_path = get_spotify_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::with_velocity(100);

    // Test all 10 supported actions
    let actions = vec![
        "play_pause",
        "play",
        "pause",
        "next_track",
        "previous_track",
        "volume_up",
        "volume_down",
        "shuffle",
        "repeat",
    ];

    for action in actions {
        let result = plugin.execute(action, &context).await;
        assert!(
            result.is_ok(),
            "Action '{}' failed: {:?}",
            action,
            result.err()
        );
    }
}

#[tokio::test]
async fn test_spotify_unknown_action() {
    let wasm_path = get_spotify_plugin_path();

    if !wasm_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    let config = WasmConfig::default();
    let mut plugin = WasmPlugin::load(&wasm_path, config)
        .await
        .expect("Failed to load plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    let context = TriggerContext::new();
    let result = plugin.execute("unknown_action", &context).await;

    // Should fail with error code 3 (action execution failed)
    assert!(result.is_err(), "Unknown action should fail");
}
