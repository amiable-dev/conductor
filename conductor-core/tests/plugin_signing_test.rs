// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! Integration tests for plugin signing and verification
//!
//! This test suite verifies the complete signing workflow:
//! 1. Generate keypair
//! 2. Sign plugin
//! 3. Verify signature
//! 4. Load signed plugin
//! 5. Reject tampered plugins
//! 6. Trust management

#![cfg(all(test, feature = "plugin-wasm", feature = "plugin-signing"))]

use ed25519_dalek::SigningKey;
use conductor_core::plugin::{
    signing::{SignatureMetadata, sign_plugin, verify_plugin_signature},
    wasm_runtime::{WasmConfig, WasmPlugin},
};
use rand::rngs::OsRng;
use std::path::PathBuf;

/// Get path to a test plugin
fn get_test_plugin_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("plugins")
        .join("wasm-spotify")
        .join("target")
        .join("wasm32-wasip1")
        .join("release")
        .join("midimon_wasm_spotify.wasm")
}

/// Create a temporary directory for test files
fn create_temp_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

#[tokio::test]
async fn test_sign_and_verify_workflow() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();
    let public_key = hex::encode(signing_key.verifying_key().to_bytes());

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");
    let test_sig_path = temp_dir.path().join("test_plugin.wasm.sig");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign the plugin
    let result = sign_plugin(
        &test_plugin_path,
        &private_key,
        "Test Developer",
        "test@example.com",
    );
    assert!(result.is_ok(), "Failed to sign plugin: {:?}", result.err());

    // Verify signature file was created
    assert!(test_sig_path.exists(), "Signature file should be created");

    // Verify signature with the public key
    let verify_result = verify_plugin_signature(&test_plugin_path, &test_sig_path, &[public_key]);
    assert!(
        verify_result.is_ok(),
        "Signature verification should succeed"
    );
}

#[tokio::test]
async fn test_load_signed_plugin_with_self_signed_mode() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign the plugin
    sign_plugin(
        &test_plugin_path,
        &private_key,
        "Test Developer",
        "test@example.com",
    )
    .expect("Failed to sign plugin");

    // Load plugin with self-signed mode enabled
    let mut config = WasmConfig::default();
    config.allow_self_signed = true;

    let result = WasmPlugin::load(&test_plugin_path, config).await;
    assert!(
        result.is_ok(),
        "Should load self-signed plugin: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_reject_unsigned_plugin_when_required() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");

    // Copy plugin to temp directory (without signing)
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Try to load with require_signature = true
    let mut config = WasmConfig::default();
    config.require_signature = true;

    let result = WasmPlugin::load(&test_plugin_path, config).await;
    assert!(
        result.is_err(),
        "Should reject unsigned plugin when signature required"
    );
    if let Err(e) = result {
        assert!(e.to_string().contains("signature required"));
    }
}

#[tokio::test]
async fn test_reject_tampered_plugin() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();
    let public_key = hex::encode(signing_key.verifying_key().to_bytes());

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");
    let test_sig_path = temp_dir.path().join("test_plugin.wasm.sig");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign the plugin
    sign_plugin(
        &test_plugin_path,
        &private_key,
        "Test Developer",
        "test@example.com",
    )
    .expect("Failed to sign plugin");

    // Tamper with the plugin (append a byte)
    let mut plugin_bytes = std::fs::read(&test_plugin_path).unwrap();
    plugin_bytes.push(0xFF);
    std::fs::write(&test_plugin_path, plugin_bytes).unwrap();

    // Verify should fail
    let verify_result = verify_plugin_signature(&test_plugin_path, &test_sig_path, &[public_key]);
    assert!(verify_result.is_err(), "Should reject tampered plugin");
    assert!(
        verify_result
            .unwrap_err()
            .to_string()
            .contains("size mismatch")
    );
}

#[tokio::test]
async fn test_reject_invalid_signature() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate two different keys
    let signing_key1 = SigningKey::generate(&mut OsRng);
    let signing_key2 = SigningKey::generate(&mut OsRng);

    let private_key1 = signing_key1.to_bytes();
    let public_key2 = hex::encode(signing_key2.verifying_key().to_bytes());

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");
    let test_sig_path = temp_dir.path().join("test_plugin.wasm.sig");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign with key1
    sign_plugin(
        &test_plugin_path,
        &private_key1,
        "Test Developer",
        "test@example.com",
    )
    .expect("Failed to sign plugin");

    // Try to verify with key2 (wrong key)
    let verify_result = verify_plugin_signature(&test_plugin_path, &test_sig_path, &[public_key2]);
    assert!(
        verify_result.is_err(),
        "Should reject signature from wrong key"
    );
    let err_msg = verify_result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Signature verification failed") || err_msg.contains("untrusted key"),
        "Expected signature verification error, got: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_signature_metadata_format() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");
    let test_sig_path = temp_dir.path().join("test_plugin.wasm.sig");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign the plugin
    sign_plugin(
        &test_plugin_path,
        &private_key,
        "Test Developer",
        "test@example.com",
    )
    .expect("Failed to sign plugin");

    // Read and parse signature metadata
    let sig_json = std::fs::read_to_string(&test_sig_path).expect("Failed to read signature file");

    let metadata: SignatureMetadata =
        serde_json::from_str(&sig_json).expect("Failed to parse signature metadata");

    // Verify metadata fields
    assert_eq!(metadata.version, 1);
    assert_eq!(metadata.algorithm, "Ed25519");
    assert_eq!(metadata.developer.name, "Test Developer");
    assert_eq!(metadata.developer.email, "test@example.com");
    assert!(!metadata.plugin_hash.is_empty());
    assert!(!metadata.signature.is_empty());
    assert!(!metadata.signed_at.is_empty());
}

#[tokio::test]
async fn test_load_unsigned_plugin_when_not_required() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");

    // Copy plugin to temp directory (without signing)
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Load plugin with default config (signatures not required)
    let config = WasmConfig::default();
    assert!(
        !config.require_signature,
        "Default config should not require signatures"
    );

    let result = WasmPlugin::load(&test_plugin_path, config).await;
    assert!(
        result.is_ok(),
        "Should load unsigned plugin when not required: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_multiple_executions_with_signed_plugin() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Sign the plugin
    sign_plugin(
        &test_plugin_path,
        &private_key,
        "Test Developer",
        "test@example.com",
    )
    .expect("Failed to sign plugin");

    // Load plugin with self-signed mode
    let mut config = WasmConfig::default();
    config.allow_self_signed = true;

    let mut plugin = WasmPlugin::load(&test_plugin_path, config)
        .await
        .expect("Failed to load signed plugin");

    plugin.init().await.expect("Failed to initialize plugin");

    // Execute multiple times to ensure signature verification doesn't interfere
    let context = conductor_core::plugin::TriggerContext::default();
    for i in 0..3 {
        let result = plugin.execute("play", &context).await;
        assert!(result.is_ok(), "Execution {} should succeed", i + 1);
    }
}

#[test]
fn test_key_size_validation() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path = temp_dir.path().join("test_plugin.wasm");

    // Copy plugin to temp directory
    std::fs::copy(&plugin_path, &test_plugin_path).expect("Failed to copy plugin");

    // Try to sign with wrong key size
    let wrong_size_key = vec![0u8; 64]; // Wrong size (should be 32)
    let result = sign_plugin(
        &test_plugin_path,
        &wrong_size_key,
        "Test Developer",
        "test@example.com",
    );

    assert!(result.is_err(), "Should reject wrong key size");
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Invalid private key size")
    );
}

#[tokio::test]
async fn test_signature_deterministic() {
    let plugin_path = get_test_plugin_path();

    if !plugin_path.exists() {
        eprintln!("Skipping test: Spotify plugin not built");
        return;
    }

    // Generate signing key
    let signing_key = SigningKey::generate(&mut OsRng);
    let private_key = signing_key.to_bytes();

    // Create temp directory for test
    let temp_dir = create_temp_dir();
    let test_plugin_path1 = temp_dir.path().join("test_plugin1.wasm");
    let test_sig_path1 = temp_dir.path().join("test_plugin1.wasm.sig");
    let test_plugin_path2 = temp_dir.path().join("test_plugin2.wasm");
    let test_sig_path2 = temp_dir.path().join("test_plugin2.wasm.sig");

    // Copy same plugin twice
    std::fs::copy(&plugin_path, &test_plugin_path1).expect("Failed to copy plugin 1");
    std::fs::copy(&plugin_path, &test_plugin_path2).expect("Failed to copy plugin 2");

    // Sign both copies with same key
    sign_plugin(&test_plugin_path1, &private_key, "Dev", "dev@example.com")
        .expect("Failed to sign plugin 1");
    sign_plugin(&test_plugin_path2, &private_key, "Dev", "dev@example.com")
        .expect("Failed to sign plugin 2");

    // Read both signature files
    let sig1_json = std::fs::read_to_string(&test_sig_path1).unwrap();
    let sig2_json = std::fs::read_to_string(&test_sig_path2).unwrap();

    let metadata1: SignatureMetadata = serde_json::from_str(&sig1_json).unwrap();
    let metadata2: SignatureMetadata = serde_json::from_str(&sig2_json).unwrap();

    // Same plugin + same key should produce same hash and signature
    assert_eq!(metadata1.plugin_hash, metadata2.plugin_hash);
    assert_eq!(metadata1.signature, metadata2.signature);
    assert_eq!(metadata1.public_key, metadata2.public_key);
}
