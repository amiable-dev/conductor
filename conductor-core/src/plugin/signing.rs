// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Plugin signing and verification (v2.7)
//!
//! This module provides Ed25519-based cryptographic signing and verification
//! for WASM plugins to ensure authenticity and integrity.
//!
//! ## Security Features
//!
//! 1. **Authenticity**: Verify plugins are signed by trusted developers
//! 2. **Integrity**: Detect any tampering with plugin binaries
//! 3. **Trust Chain**: Establish developer identity via public keys
//!
//! ## File Format
//!
//! Plugins are distributed with two files:
//! - `plugin.wasm` - The WebAssembly binary
//! - `plugin.wasm.sig` - JSON signature metadata
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! # #[cfg(feature = "plugin-signing")]
//! # fn example() {
//! use conductor_core::plugin::signing::{sign_plugin, verify_plugin_signature};
//! use std::path::Path;
//!
//! // Generate a 32-byte Ed25519 private key
//! let private_key = [0u8; 32]; // In practice, use proper key generation
//!
//! // Sign a plugin
//! sign_plugin(
//!     Path::new("plugin.wasm"),
//!     &private_key,
//!     "Developer Name",
//!     "dev@example.com",
//! ).expect("Failed to sign plugin");
//!
//! // Verify signature (with empty trusted keys - accepts any signature)
//! verify_plugin_signature(
//!     Path::new("plugin.wasm"),
//!     Path::new("plugin.wasm.sig"),
//!     &Vec::new(), // Empty trusted keys list
//! ).expect("Failed to verify signature");
//! # }
//! ```

#![cfg(feature = "plugin-signing")]

use crate::error::EngineError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Signature metadata stored in `.wasm.sig` files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMetadata {
    /// Signature format version (currently 1)
    pub version: u32,

    /// Signature algorithm (always "Ed25519" for v1)
    pub algorithm: String,

    /// SHA-256 hash of the plugin binary (hex-encoded)
    pub plugin_hash: String,

    /// Size of the plugin binary in bytes
    pub plugin_size: u64,

    /// Public key used for verification (hex-encoded, 32 bytes)
    pub public_key: String,

    /// Ed25519 signature (hex-encoded, 64 bytes)
    pub signature: String,

    /// Timestamp when signature was created (ISO 8601)
    pub signed_at: String,

    /// Developer information
    pub developer: DeveloperInfo,
}

/// Developer information embedded in signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperInfo {
    /// Developer or organization name
    pub name: String,

    /// Contact email address
    pub email: String,
}

/// Trusted key entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedKey {
    /// Developer or organization name
    pub name: String,

    /// Contact email
    pub email: String,

    /// Public key (hex-encoded, 32 bytes)
    pub public_key: String,

    /// When this key was added to trusted list (ISO 8601)
    pub added_at: String,
}

/// Sign a plugin binary and create signature file
///
/// This creates a `.wasm.sig` file alongside the plugin with Ed25519 signature
/// and metadata. The signature is computed over the SHA-256 hash of the plugin binary.
///
/// # Arguments
///
/// * `plugin_path` - Path to the `.wasm` plugin binary
/// * `private_key_bytes` - Ed25519 private key (64 bytes)
/// * `developer_name` - Name of the developer or organization
/// * `developer_email` - Contact email address
///
/// # Returns
///
/// Returns `Ok(())` if signature file was created successfully,
/// or `EngineError` if signing failed.
///
/// # Example
///
/// ```rust,no_run
/// use conductor_core::plugin::signing::sign_plugin;
/// use std::path::Path;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let private_key = std::fs::read("signing.key")?;
/// sign_plugin(
///     Path::new("plugin.wasm"),
///     &private_key,
///     "Amiable Team",
///     "dev@amiable.com",
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn sign_plugin(
    plugin_path: &Path,
    private_key_bytes: &[u8],
    developer_name: &str,
    developer_email: &str,
) -> Result<(), EngineError> {
    use ed25519_dalek::{Signature, Signer, SigningKey};

    // Read plugin binary
    let plugin_bytes = std::fs::read(plugin_path)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to read plugin: {}", e)))?;

    // Compute SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&plugin_bytes);
    let plugin_hash = hasher.finalize();
    let plugin_hash_hex = hex::encode(plugin_hash);

    // Load signing key (Ed25519 private keys are 32 bytes)
    if private_key_bytes.len() != 32 {
        return Err(EngineError::PluginLoadFailed(format!(
            "Invalid private key size: {} bytes (expected 32)",
            private_key_bytes.len()
        )));
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(private_key_bytes);
    let signing_key = SigningKey::from_bytes(&key_bytes);

    // Sign the hash
    let signature: Signature = signing_key.sign(&plugin_hash);

    // Get public key
    let verifying_key = signing_key.verifying_key();
    let public_key_hex = hex::encode(verifying_key.to_bytes());

    // Create signature metadata
    let sig_metadata = SignatureMetadata {
        version: 1,
        algorithm: "Ed25519".to_string(),
        plugin_hash: plugin_hash_hex,
        plugin_size: plugin_bytes.len() as u64,
        public_key: public_key_hex,
        signature: hex::encode(signature.to_bytes()),
        signed_at: chrono::Utc::now().to_rfc3339(),
        developer: DeveloperInfo {
            name: developer_name.to_string(),
            email: developer_email.to_string(),
        },
    };

    // Write signature file
    let sig_path = plugin_path.with_extension("wasm.sig");
    let sig_json = serde_json::to_string_pretty(&sig_metadata).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to serialize signature: {}", e))
    })?;

    std::fs::write(&sig_path, sig_json).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to write signature file: {}", e))
    })?;

    Ok(())
}

/// Verify a plugin's cryptographic signature
///
/// This checks that:
/// 1. Signature file exists and is valid JSON
/// 2. Plugin binary hash matches signature metadata
/// 3. Ed25519 signature is valid for the hash
/// 4. Public key is in the trusted keys list
///
/// # Arguments
///
/// * `plugin_path` - Path to the `.wasm` plugin binary
/// * `sig_path` - Path to the `.wasm.sig` signature file
/// * `trusted_keys` - List of trusted public keys
///
/// # Returns
///
/// Returns `Ok(())` if signature is valid and key is trusted,
/// or `EngineError` if verification fails.
///
/// # Example
///
/// ```rust,no_run
/// use conductor_core::plugin::signing::verify_plugin_signature;
/// use std::path::Path;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let trusted_keys = vec![
///     "a1b2c3d4e5f6...".to_string(),  // Hex-encoded public key
/// ];
///
/// verify_plugin_signature(
///     Path::new("plugin.wasm"),
///     Path::new("plugin.wasm.sig"),
///     &trusted_keys,
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn verify_plugin_signature(
    plugin_path: &Path,
    sig_path: &Path,
    trusted_keys: &[String],
) -> Result<(), EngineError> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    // Read signature metadata
    let sig_json = std::fs::read_to_string(sig_path)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to read signature: {}", e)))?;

    let sig_metadata: SignatureMetadata = serde_json::from_str(&sig_json)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Invalid signature format: {}", e)))?;

    // Verify signature version and algorithm
    if sig_metadata.version != 1 {
        return Err(EngineError::PluginLoadFailed(format!(
            "Unsupported signature version: {}",
            sig_metadata.version
        )));
    }

    if sig_metadata.algorithm != "Ed25519" {
        return Err(EngineError::PluginLoadFailed(format!(
            "Unsupported signature algorithm: {}",
            sig_metadata.algorithm
        )));
    }

    // Read plugin binary
    let plugin_bytes = std::fs::read(plugin_path)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to read plugin: {}", e)))?;

    // Verify file size matches
    if plugin_bytes.len() as u64 != sig_metadata.plugin_size {
        return Err(EngineError::PluginLoadFailed(format!(
            "Plugin size mismatch: expected {} bytes, got {} bytes",
            sig_metadata.plugin_size,
            plugin_bytes.len()
        )));
    }

    // Compute hash
    let mut hasher = Sha256::new();
    hasher.update(&plugin_bytes);
    let computed_hash = hex::encode(hasher.finalize());

    // Verify hash matches
    if computed_hash != sig_metadata.plugin_hash {
        return Err(EngineError::PluginLoadFailed(
            "Plugin hash mismatch - binary may have been tampered with".to_string(),
        ));
    }

    // Decode public key and signature
    let public_key_bytes = hex::decode(&sig_metadata.public_key).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Invalid public key encoding: {}", e))
    })?;

    let signature_bytes = hex::decode(&sig_metadata.signature)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Invalid signature encoding: {}", e)))?;

    // Verify key and signature sizes
    if public_key_bytes.len() != 32 {
        return Err(EngineError::PluginLoadFailed(format!(
            "Invalid public key size: {} bytes (expected 32)",
            public_key_bytes.len()
        )));
    }

    if signature_bytes.len() != 64 {
        return Err(EngineError::PluginLoadFailed(format!(
            "Invalid signature size: {} bytes (expected 64)",
            signature_bytes.len()
        )));
    }

    // Convert to Ed25519 types
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&public_key_bytes);

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(&signature_bytes);

    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Invalid public key format: {}", e)))?;

    let signature = Signature::from_bytes(&sig_array);

    // Verify signature (sign the hash bytes, not the hex string)
    let hash_bytes = hex::decode(&computed_hash)
        .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to decode hash: {}", e)))?;

    verifying_key.verify(&hash_bytes, &signature).map_err(|_| {
        EngineError::PluginLoadFailed(
            "Signature verification failed - invalid signature for this plugin".to_string(),
        )
    })?;

    // Check if public key is trusted
    if !trusted_keys.contains(&sig_metadata.public_key) {
        return Err(EngineError::PluginLoadFailed(format!(
            "Plugin signed by untrusted key. Developer: {} <{}>. \
                     Add this key to trusted_keys.toml to allow this plugin.",
            sig_metadata.developer.name, sig_metadata.developer.email
        )));
    }

    Ok(())
}

/// Load trusted keys from TOML configuration file
///
/// Default path: `~/.config/midimon/trusted_keys.toml`
///
/// # Format
///
/// ```toml
/// [[keys]]
/// name = "Developer Name"
/// email = "dev@example.com"
/// public_key = "a1b2c3d4..."
/// added_at = "2025-01-19T10:00:00Z"
/// ```
pub fn load_trusted_keys() -> Result<Vec<String>, EngineError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| EngineError::PluginLoadFailed("No config directory found".to_string()))?
        .join("midimon");

    let keys_path = config_dir.join("trusted_keys.toml");

    if !keys_path.exists() {
        // Return empty list if file doesn't exist (no trusted keys)
        return Ok(Vec::new());
    }

    let keys_toml = std::fs::read_to_string(&keys_path).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to read trusted keys: {}", e))
    })?;

    #[derive(Deserialize)]
    struct TrustedKeysFile {
        keys: Vec<TrustedKey>,
    }

    let keys_file: TrustedKeysFile = toml::from_str(&keys_toml).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Invalid trusted keys format: {}", e))
    })?;

    Ok(keys_file
        .keys
        .iter()
        .map(|k| k.public_key.clone())
        .collect())
}

/// Add a public key to the trusted keys list
///
/// # Arguments
///
/// * `public_key` - Hex-encoded public key (32 bytes)
/// * `name` - Developer or organization name
/// * `email` - Contact email address
pub fn add_trusted_key(public_key: &str, name: &str, email: &str) -> Result<(), EngineError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| EngineError::PluginLoadFailed("No config directory found".to_string()))?
        .join("midimon");

    std::fs::create_dir_all(&config_dir).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to create config directory: {}", e))
    })?;

    let keys_path = config_dir.join("trusted_keys.toml");

    #[derive(Serialize, Deserialize)]
    struct TrustedKeysFile {
        keys: Vec<TrustedKey>,
    }

    // Load existing keys or create new file
    let mut keys_file = if keys_path.exists() {
        let keys_toml = std::fs::read_to_string(&keys_path).map_err(|e| {
            EngineError::PluginLoadFailed(format!("Failed to read trusted keys: {}", e))
        })?;

        toml::from_str(&keys_toml).map_err(|e| {
            EngineError::PluginLoadFailed(format!("Invalid trusted keys format: {}", e))
        })?
    } else {
        TrustedKeysFile { keys: Vec::new() }
    };

    // Check if key already exists
    if keys_file.keys.iter().any(|k| k.public_key == public_key) {
        return Err(EngineError::PluginLoadFailed(format!(
            "Public key already trusted: {}",
            public_key
        )));
    }

    // Add new key
    keys_file.keys.push(TrustedKey {
        name: name.to_string(),
        email: email.to_string(),
        public_key: public_key.to_string(),
        added_at: chrono::Utc::now().to_rfc3339(),
    });

    // Save updated file
    let keys_toml = toml::to_string_pretty(&keys_file).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to serialize trusted keys: {}", e))
    })?;

    std::fs::write(&keys_path, keys_toml).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to write trusted keys: {}", e))
    })?;

    Ok(())
}

/// Save trusted keys to config file
///
/// # Arguments
///
/// * `public_keys` - List of public key hex strings to save
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if saving fails
pub fn save_trusted_keys(public_keys: &[String]) -> Result<(), EngineError> {
    use serde::{Deserialize, Serialize};

    let config_dir = dirs::config_dir()
        .ok_or_else(|| EngineError::PluginLoadFailed("No config directory found".to_string()))?
        .join("midimon");

    std::fs::create_dir_all(&config_dir).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to create config directory: {}", e))
    })?;

    let keys_path = config_dir.join("trusted_keys.toml");

    #[derive(Serialize, Deserialize)]
    struct TrustedKeysFile {
        keys: Vec<TrustedKey>,
    }

    // Convert public key strings to TrustedKey structs
    let keys_file = TrustedKeysFile {
        keys: public_keys
            .iter()
            .map(|k| TrustedKey {
                name: String::new(), // Name not preserved when just saving keys
                email: String::new(),
                public_key: k.clone(),
                added_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect(),
    };

    // Save file
    let keys_toml = toml::to_string_pretty(&keys_file).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to serialize trusted keys: {}", e))
    })?;

    std::fs::write(&keys_path, keys_toml).map_err(|e| {
        EngineError::PluginLoadFailed(format!("Failed to write trusted keys: {}", e))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_signature_metadata_serialization() {
        let metadata = SignatureMetadata {
            version: 1,
            algorithm: "Ed25519".to_string(),
            plugin_hash: "a1b2c3d4".to_string(),
            plugin_size: 1000,
            public_key: "e5f6g7h8".to_string(),
            signature: "i9j0k1l2".to_string(),
            signed_at: "2025-01-19T10:00:00Z".to_string(),
            developer: DeveloperInfo {
                name: "Test Developer".to_string(),
                email: "test@example.com".to_string(),
            },
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: SignatureMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.algorithm, "Ed25519");
        assert_eq!(deserialized.developer.name, "Test Developer");
    }

    #[test]
    fn test_key_generation() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        assert_eq!(signing_key.to_bytes().len(), 32); // Ed25519 secret key is 32 bytes
        assert_eq!(verifying_key.to_bytes().len(), 32); // Ed25519 public key is 32 bytes
    }
}
