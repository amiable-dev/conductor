// Copyright 2025 Amiable Team
// SPDX-License-Identifier: MIT

//! CLI tool for signing and verifying WASM plugins
//!
//! This tool provides commands for:
//! - Generating Ed25519 keypairs
//! - Signing plugins with private keys
//! - Verifying plugin signatures
//! - Managing trusted keys

use std::path::PathBuf;
use std::process;

fn print_usage() {
    println!("midimon-sign - Plugin Signing Tool");
    println!();
    println!("USAGE:");
    println!("  midimon-sign generate-key <output-path>           Generate new Ed25519 keypair");
    println!("  midimon-sign sign <plugin> <key> [options]        Sign a plugin");
    println!("  midimon-sign verify <plugin>                      Verify plugin signature");
    println!("  midimon-sign trust add <public-key> <name>        Add trusted key");
    println!("  midimon-sign trust list                           List trusted keys");
    println!("  midimon-sign trust remove <public-key>            Remove trusted key");
    println!();
    println!("SIGN OPTIONS:");
    println!("  --name <name>        Developer name (required)");
    println!("  --email <email>      Developer email (required)");
    println!();
    println!("EXAMPLES:");
    println!("  # Generate keypair");
    println!("  midimon-sign generate-key ~/.midimon/my-key");
    println!();
    println!("  # Sign a plugin");
    println!("  midimon-sign sign plugin.wasm ~/.midimon/my-key \\");
    println!("    --name \"John Doe\" --email \"john@example.com\"");
    println!();
    println!("  # Verify a plugin");
    println!("  midimon-sign verify plugin.wasm");
    println!();
    println!("  # Add trusted key");
    println!("  midimon-sign trust add abcd1234... \"Official MIDIMon\"");
}

fn generate_keypair(output_path: &str) {
    #[cfg(not(feature = "plugin-signing"))]
    {
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        println!("Generating Ed25519 keypair...");

        let signing_key = SigningKey::generate(&mut OsRng);
        let private_key = signing_key.to_bytes();
        let public_key = signing_key.verifying_key().to_bytes();

        // Write private key
        let private_path = format!("{}.private", output_path);
        std::fs::write(&private_path, &private_key).unwrap_or_else(|e| {
            eprintln!("Error writing private key: {}", e);
            process::exit(1);
        });

        // Write public key
        let public_path = format!("{}.public", output_path);
        std::fs::write(&public_path, hex::encode(&public_key)).unwrap_or_else(|e| {
            eprintln!("Error writing public key: {}", e);
            process::exit(1);
        });

        println!("✓ Keypair generated successfully!");
        println!();
        println!("Private key: {}", private_path);
        println!("Public key:  {}", public_path);
        println!();
        println!("Public key (hex): {}", hex::encode(public_key));
        println!();
        println!("⚠️  Keep your private key secure and never share it!");
    }
}

fn sign_plugin(plugin_path: &str, key_path: &str, name: Option<&str>, email: Option<&str>) {
    #[cfg(not(feature = "plugin-signing"))]
    {
        let _ = (plugin_path, key_path, name, email);
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use conductor_core::plugin::signing::sign_plugin;

        let developer_name = name.unwrap_or_else(|| {
            eprintln!("Error: --name is required");
            process::exit(1);
        });

        let developer_email = email.unwrap_or_else(|| {
            eprintln!("Error: --email is required");
            process::exit(1);
        });

        println!("Signing plugin: {}", plugin_path);
        println!("Developer: {} <{}>", developer_name, developer_email);

        // Read private key
        let key_file = format!("{}.private", key_path);
        let private_key = std::fs::read(&key_file).unwrap_or_else(|e| {
            eprintln!("Error reading private key from {}: {}", key_file, e);
            eprintln!("Try: {}", key_path);

            // Try without .private extension
            std::fs::read(key_path).unwrap_or_else(|e| {
                eprintln!("Error reading private key from {}: {}", key_path, e);
                process::exit(1);
            })
        });

        if private_key.len() != 32 {
            eprintln!(
                "Error: Invalid private key size (expected 32 bytes, got {})",
                private_key.len()
            );
            process::exit(1);
        }

        // Sign plugin
        let plugin_pathbuf = PathBuf::from(plugin_path);
        sign_plugin(
            &plugin_pathbuf,
            &private_key,
            developer_name,
            developer_email,
        )
        .unwrap_or_else(|e| {
            eprintln!("Error signing plugin: {}", e);
            process::exit(1);
        });

        println!("✓ Plugin signed successfully!");
        println!("Signature file: {}.sig", plugin_path);
    }
}

fn verify_plugin(plugin_path: &str) {
    #[cfg(not(feature = "plugin-signing"))]
    {
        let _ = plugin_path;
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use conductor_core::plugin::signing::{load_trusted_keys, verify_plugin_signature};
        use std::path::Path;

        let plugin_pathbuf = PathBuf::from(plugin_path);
        let sig_path = plugin_pathbuf.with_extension("wasm.sig");

        if !sig_path.exists() {
            eprintln!("Error: No signature file found at {:?}", sig_path);
            process::exit(1);
        }

        println!("Verifying plugin: {}", plugin_path);
        println!("Signature file: {:?}", sig_path);

        // Load trusted keys
        let trusted_keys = load_trusted_keys().unwrap_or_else(|e| {
            eprintln!("Warning: Could not load trusted keys: {}", e);
            eprintln!("Continuing with empty trust list...");
            Vec::new()
        });

        println!("Trusted keys: {}", trusted_keys.len());

        // Read signature metadata
        let sig_json = std::fs::read_to_string(&sig_path).unwrap_or_else(|e| {
            eprintln!("Error reading signature: {}", e);
            process::exit(1);
        });

        let sig_metadata: conductor_core::plugin::signing::SignatureMetadata =
            serde_json::from_str(&sig_json).unwrap_or_else(|e| {
                eprintln!("Error parsing signature: {}", e);
                process::exit(1);
            });

        println!();
        println!("Signature Details:");
        println!("  Version:     {}", sig_metadata.version);
        println!("  Algorithm:   {}", sig_metadata.algorithm);
        println!("  Signed at:   {}", sig_metadata.signed_at);
        println!(
            "  Developer:   {} <{}>",
            sig_metadata.developer.name, sig_metadata.developer.email
        );
        println!("  Public key:  {}", sig_metadata.public_key);
        println!();

        // Verify signature
        match verify_plugin_signature(Path::new(plugin_path), &sig_path, &trusted_keys) {
            Ok(()) => {
                println!("✓ Signature verified successfully!");
                println!("✓ Plugin signed by trusted key");
            }
            Err(e) => {
                let err_msg = e.to_string();
                if err_msg.contains("untrusted key") {
                    println!("⚠️  Signature is valid but key is not trusted");
                    println!();
                    println!("To trust this key, run:");
                    println!(
                        "  midimon-sign trust add {} \"{}\"",
                        sig_metadata.public_key, sig_metadata.developer.name
                    );
                } else {
                    eprintln!("✗ Signature verification failed: {}", err_msg);
                    process::exit(1);
                }
            }
        }
    }
}

fn trust_add(public_key: &str, name: &str) {
    #[cfg(not(feature = "plugin-signing"))]
    {
        let _ = (public_key, name);
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use conductor_core::plugin::signing::add_trusted_key;

        println!("Adding trusted key:");
        println!("  Name: {}", name);
        println!("  Key:  {}", public_key);

        // Use empty email since it's just a trusted key, not a developer identity
        add_trusted_key(public_key, name, "").unwrap_or_else(|e| {
            eprintln!("Error adding trusted key: {}", e);
            process::exit(1);
        });

        println!("✓ Trusted key added successfully!");
    }
}

fn trust_list() {
    #[cfg(not(feature = "plugin-signing"))]
    {
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use conductor_core::plugin::signing::load_trusted_keys;

        let trusted_keys = load_trusted_keys().unwrap_or_else(|e| {
            eprintln!("Warning: Could not load trusted keys: {}", e);
            Vec::new()
        });

        if trusted_keys.is_empty() {
            println!("No trusted keys configured");
            return;
        }

        println!("Trusted keys ({}):", trusted_keys.len());
        for key in trusted_keys {
            println!("  {}", key);
        }
    }
}

fn trust_remove(public_key: &str) {
    #[cfg(not(feature = "plugin-signing"))]
    {
        let _ = public_key;
        eprintln!("Error: Plugin signing feature not enabled");
        eprintln!("Rebuild with: cargo build --package midimon-daemon --features plugin-signing");
        process::exit(1);
    }

    #[cfg(feature = "plugin-signing")]
    {
        use conductor_core::plugin::signing::{load_trusted_keys, save_trusted_keys};

        let mut trusted_keys = load_trusted_keys().unwrap_or_else(|e| {
            eprintln!("Error loading trusted keys: {}", e);
            process::exit(1);
        });

        let before_count = trusted_keys.len();
        trusted_keys.retain(|k| k != public_key);
        let after_count = trusted_keys.len();

        if before_count == after_count {
            eprintln!("Error: Key not found in trusted list");
            process::exit(1);
        }

        save_trusted_keys(&trusted_keys).unwrap_or_else(|e| {
            eprintln!("Error saving trusted keys: {}", e);
            process::exit(1);
        });

        println!("✓ Trusted key removed successfully!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "generate-key" => {
            if args.len() < 3 {
                eprintln!("Error: Missing output path");
                print_usage();
                process::exit(1);
            }
            generate_keypair(&args[2]);
        }

        "sign" => {
            if args.len() < 4 {
                eprintln!("Error: Missing arguments");
                print_usage();
                process::exit(1);
            }

            let plugin_path = &args[2];
            let key_path = &args[3];

            // Parse optional arguments
            let mut name = None;
            let mut email = None;

            let mut i = 4;
            while i < args.len() {
                match args[i].as_str() {
                    "--name" => {
                        if i + 1 < args.len() {
                            name = Some(args[i + 1].as_str());
                            i += 2;
                        } else {
                            eprintln!("Error: --name requires a value");
                            process::exit(1);
                        }
                    }
                    "--email" => {
                        if i + 1 < args.len() {
                            email = Some(args[i + 1].as_str());
                            i += 2;
                        } else {
                            eprintln!("Error: --email requires a value");
                            process::exit(1);
                        }
                    }
                    _ => {
                        eprintln!("Error: Unknown option: {}", args[i]);
                        process::exit(1);
                    }
                }
            }

            sign_plugin(plugin_path, key_path, name, email);
        }

        "verify" => {
            if args.len() < 3 {
                eprintln!("Error: Missing plugin path");
                print_usage();
                process::exit(1);
            }
            verify_plugin(&args[2]);
        }

        "trust" => {
            if args.len() < 3 {
                eprintln!("Error: Missing trust command");
                print_usage();
                process::exit(1);
            }

            match args[2].as_str() {
                "add" => {
                    if args.len() < 5 {
                        eprintln!("Error: trust add requires <public-key> <name>");
                        process::exit(1);
                    }
                    trust_add(&args[3], &args[4]);
                }
                "list" => {
                    trust_list();
                }
                "remove" => {
                    if args.len() < 4 {
                        eprintln!("Error: trust remove requires <public-key>");
                        process::exit(1);
                    }
                    trust_remove(&args[3]);
                }
                _ => {
                    eprintln!("Error: Unknown trust command: {}", args[2]);
                    print_usage();
                    process::exit(1);
                }
            }
        }

        "--help" | "-h" | "help" => {
            print_usage();
        }

        _ => {
            eprintln!("Error: Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
