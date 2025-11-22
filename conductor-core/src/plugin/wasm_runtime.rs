// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! WASM plugin runtime for sandboxed plugin execution (v2.5)
//!
//! This module provides a secure, isolated runtime for executing plugins compiled
//! to WebAssembly. WASM plugins run in a sandboxed environment with:
//! - Memory isolation (cannot access daemon memory)
//! - Resource limits (CPU, memory, execution time)
//! - Capability-based permissions (WASI)
//! - Platform independence (same .wasm runs on macOS/Linux/Windows)
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │  MIDIMon Daemon (Rust)              │
//! │  ┌───────────────────────────────┐  │
//! │  │  WASM Runtime (wasmtime)      │  │
//! │  │  ┌─────────────────────────┐  │  │
//! │  │  │  Plugin.wasm            │  │  │
//! │  │  │  - Sandboxed execution  │  │  │
//! │  │  │  - Resource limits      │  │  │
//! │  │  │  - Capability system    │  │  │
//! │  │  └─────────────────────────┘  │  │
//! │  └───────────────────────────────┘  │
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Security Features
//!
//! 1. **Process Isolation**: Plugin runs in separate memory space
//! 2. **Resource Limits**: Configurable limits on memory, CPU, execution time
//! 3. **Capability System**: Fine-grained permission control via WASI
//! 4. **Timeout Protection**: Plugins cannot run indefinitely
//! 5. **Crash Isolation**: Plugin crash doesn't affect daemon
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use conductor_core::plugin::wasm_runtime::{WasmPlugin, WasmConfig};
//! use conductor_core::plugin::types::Capability;
//! use std::path::Path;
//! # use std::collections::HashMap;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = WasmConfig {
//!     max_memory_bytes: 128 * 1024 * 1024, // 128 MB
//!     max_execution_time: std::time::Duration::from_secs(5),
//!     max_fuel: 100_000_000, // 100M instructions
//!     capabilities: vec![Capability::Network],
//!     #[cfg(feature = "plugin-signing")]
//!     require_signature: false,
//!     #[cfg(feature = "plugin-signing")]
//!     allow_self_signed: true,
//!     #[cfg(feature = "plugin-signing")]
//!     trusted_keys_path: None,
//! };
//!
//! let mut plugin = WasmPlugin::load(
//!     Path::new("plugins/spotify/plugin.wasm"),
//!     config,
//! ).await?;
//!
//! // Execute plugin with parameters
//! let params = HashMap::from([("action".to_string(), "play".to_string())]);
//! plugin.execute(serde_json::to_string(&params)?.as_str()).await?;
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "plugin-wasm")]

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;
use wasmtime_wasi::preview1::WasiP1Ctx;

use crate::error::EngineError;
use crate::events::ProcessedEvent;
use crate::plugin::{Capability, PluginMetadata, TriggerContext};

/// Configuration for WASM plugin runtime
#[derive(Debug, Clone)]
pub struct WasmConfig {
    /// Maximum memory in bytes (default: 128 MB)
    pub max_memory_bytes: u64,

    /// Maximum execution time per call (default: 5 seconds)
    pub max_execution_time: Duration,

    /// Maximum fuel (instruction count) per call (default: 100M instructions)
    /// 1 fuel ≈ 1 WASM instruction
    pub max_fuel: u64,

    /// Capabilities granted to this plugin
    pub capabilities: Vec<Capability>,

    /// Require cryptographic signature for plugin loading (default: false for backward compatibility)
    #[cfg(feature = "plugin-signing")]
    pub require_signature: bool,

    /// Allow plugins signed by any key (self-signed), or require trusted keys (default: false)
    #[cfg(feature = "plugin-signing")]
    pub allow_self_signed: bool,

    /// Custom path to trusted_keys.toml file (default: ~/.config/midimon/trusted_keys.toml)
    #[cfg(feature = "plugin-signing")]
    pub trusted_keys_path: Option<std::path::PathBuf>,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: 128 * 1024 * 1024, // 128 MB
            max_execution_time: Duration::from_secs(5),
            max_fuel: 100_000_000, // 100M instructions
            capabilities: Vec::new(),
            #[cfg(feature = "plugin-signing")]
            require_signature: false, // Backward compatible: signatures optional by default
            #[cfg(feature = "plugin-signing")]
            allow_self_signed: false, // Require trusted keys by default
            #[cfg(feature = "plugin-signing")]
            trusted_keys_path: None, // Use default ~/.config/midimon/trusted_keys.toml
        }
    }
}

/// Resource limiter for WASM instances
struct PluginResourceLimiter {
    memory_limit: u64,
}

impl ResourceLimiter for PluginResourceLimiter {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> Result<bool> {
        if desired as u64 > self.memory_limit {
            Ok(false) // Deny allocation
        } else {
            Ok(true) // Allow allocation
        }
    }

    fn table_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> anyhow::Result<bool> {
        // Limit table size to prevent DoS
        Ok(desired <= 10000)
    }
}

/// Host state for WASM plugin
struct PluginHostState {
    wasi: WasiP1Ctx,
    limiter: PluginResourceLimiter,
}

/// WASM plugin instance with sandboxed execution
pub struct WasmPlugin {
    engine: Engine,
    module: Module,
    linker: Linker<PluginHostState>,
    config: WasmConfig,
    metadata: Option<PluginMetadata>,
}

impl WasmPlugin {
    /// Load a WASM plugin from file
    ///
    /// This initializes the WASM runtime, loads the module, and sets up
    /// the sandboxed environment with configured resource limits and capabilities.
    ///
    /// If plugin signing is enabled and configured, this will verify the plugin's
    /// cryptographic signature before loading.
    pub async fn load(path: &Path, config: WasmConfig) -> Result<Self, EngineError> {
        // Verify plugin signature if signing feature is enabled
        #[cfg(feature = "plugin-signing")]
        {
            use crate::plugin::signing::{load_trusted_keys, verify_plugin_signature};

            let sig_path = path.with_extension("wasm.sig");

            if sig_path.exists() {
                // Signature file exists - verify it
                let trusted_keys = if config.allow_self_signed {
                    // Allow any key (self-signed)
                    vec![] // Empty list means skip trust check in verify_plugin_signature
                } else {
                    // Load trusted keys from config or default location
                    load_trusted_keys()?
                };

                // Skip trust check if allow_self_signed is true by passing signature's own key
                if config.allow_self_signed {
                    // For self-signed mode, we still verify the signature is valid,
                    // but we extract the public key from the signature and trust it
                    let sig_json = std::fs::read_to_string(&sig_path).map_err(|e| {
                        EngineError::PluginLoadFailed(format!("Failed to read signature: {}", e))
                    })?;
                    let sig_metadata: crate::plugin::signing::SignatureMetadata =
                        serde_json::from_str(&sig_json).map_err(|e| {
                            EngineError::PluginLoadFailed(format!(
                                "Invalid signature format: {}",
                                e
                            ))
                        })?;

                    // Trust the key from the signature itself
                    verify_plugin_signature(path, &sig_path, &[sig_metadata.public_key])?;
                } else {
                    // Normal mode: require trusted keys
                    verify_plugin_signature(path, &sig_path, &trusted_keys)?;
                }
            } else if config.require_signature {
                // Signature required but not found
                return Err(EngineError::PluginLoadFailed(format!(
                    "Plugin signature required but not found: {:?}.sig",
                    path
                )));
            }
            // If signature file doesn't exist and not required, continue loading without verification
        }

        // Configure WASM engine
        let mut engine_config = Config::new();
        engine_config.async_support(true); // Enable async execution
        engine_config.wasm_component_model(false); // Use core WASM for now
        engine_config.consume_fuel(true); // Enable execution metering

        let engine = Engine::new(&engine_config)
            .map_err(|e| EngineError::PluginLoadFailed(e.to_string()))?;

        // Load WASM module
        let module = Module::from_file(&engine, path).map_err(|e| {
            EngineError::PluginLoadFailed(format!("Failed to load WASM module: {}", e))
        })?;

        // Create linker for WASI functions (using preview1 for core modules)
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |state: &mut PluginHostState| {
            &mut state.wasi
        })
        .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to setup WASI: {}", e)))?;

        Ok(WasmPlugin {
            engine,
            module,
            linker,
            config,
            metadata: None,
        })
    }

    /// Initialize plugin and retrieve metadata
    ///
    /// This calls the `init` export to get plugin metadata (name, version, etc.)
    pub async fn init(&mut self) -> Result<PluginMetadata, EngineError> {
        let mut store = self.create_store()?;

        // Instantiate module
        let instance = self
            .linker
            .instantiate_async(&mut store, &self.module)
            .await
            .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to instantiate: {}", e)))?;

        // Call init() to get metadata
        // Note: init() returns u64 with ptr in high 32 bits, len in low 32 bits
        let init_func = instance
            .get_typed_func::<(), u64>(&mut store, "init")
            .map_err(|e| EngineError::PluginLoadFailed(format!("Missing init export: {}", e)))?;

        let packed = init_func
            .call_async(&mut store, ())
            .await
            .map_err(|e| EngineError::PluginLoadFailed(format!("init() failed: {}", e)))?;

        // Unpack ptr and len from u64
        let ptr = (packed >> 32) as u32;
        let len = (packed & 0xFFFFFFFF) as u32;

        // Read metadata from WASM memory
        let metadata_json = self.read_string_from_memory(&instance, &mut store, ptr, len)?;

        let metadata: PluginMetadata = serde_json::from_str(&metadata_json)
            .map_err(|e| EngineError::PluginLoadFailed(format!("Invalid metadata: {}", e)))?;

        self.metadata = Some(metadata.clone());
        Ok(metadata)
    }

    /// Execute plugin action
    ///
    /// This calls the `execute` export with the action name and trigger context,
    /// enforcing timeout and resource limits.
    pub async fn execute(&self, action: &str, context: &TriggerContext) -> Result<(), EngineError> {
        let mut store = self.create_store()?;

        // Instantiate module
        let instance = self
            .linker
            .instantiate_async(&mut store, &self.module)
            .await
            .map_err(|e| {
                EngineError::PluginExecutionFailed(format!("Failed to instantiate: {}", e))
            })?;

        // Serialize request
        let request = ActionRequest {
            action: action.to_string(),
            context: context.clone(),
        };
        let request_json = serde_json::to_string(&request).map_err(|e| {
            EngineError::PluginExecutionFailed(format!("Serialization failed: {}", e))
        })?;

        // Write request to WASM memory
        let (ptr, len) = self
            .write_string_to_memory(&instance, &mut store, &request_json)
            .await?;

        // Call execute(ptr, len) -> result_code
        let execute_func = instance
            .get_typed_func::<(u32, u32), i32>(&mut store, "execute")
            .map_err(|e| {
                EngineError::PluginExecutionFailed(format!("Missing execute export: {}", e))
            })?;

        // Execute with timeout
        let timeout = self.config.max_execution_time;
        let result = tokio::time::timeout(timeout, execute_func.call_async(&mut store, (ptr, len)))
            .await
            .map_err(|_| {
                EngineError::PluginExecutionFailed(format!(
                    "Execution timeout ({}s)",
                    timeout.as_secs()
                ))
            })?
            .map_err(|e| EngineError::PluginExecutionFailed(format!("Execution failed: {}", e)))?;

        if result != 0 {
            return Err(EngineError::PluginExecutionFailed(format!(
                "Plugin returned error code: {}",
                result
            )));
        }

        Ok(())
    }

    /// Get plugin metadata
    pub fn metadata(&self) -> Option<&PluginMetadata> {
        self.metadata.as_ref()
    }

    // --- Private helper methods ---

    /// Create a new store with WASI context and resource limits
    fn create_store(&self) -> Result<Store<PluginHostState>, EngineError> {
        // Build WASI context with capabilities
        let mut wasi_builder = WasiCtxBuilder::new();
        wasi_builder.inherit_stdio();
        wasi_builder.inherit_args();

        // Grant filesystem access if capability is present
        if self.config.capabilities.contains(&Capability::Filesystem) {
            // Create plugin-specific data directory
            let plugin_data_dir = dirs::data_dir()
                .ok_or_else(|| EngineError::PluginLoadFailed("No data directory".to_string()))?
                .join("midimon")
                .join("plugin-data");

            std::fs::create_dir_all(&plugin_data_dir).map_err(|e| {
                EngineError::PluginLoadFailed(format!("Failed to create plugin data dir: {}", e))
            })?;

            // Preopen directory with read/write access (wasmtime v26 API)
            // This allows the plugin to access only this specific directory
            use wasmtime_wasi::DirPerms;
            use wasmtime_wasi::FilePerms;

            let dir_perms = DirPerms::all();
            let file_perms = FilePerms::all();

            wasi_builder
                .preopened_dir(
                    plugin_data_dir,
                    "/", // Mount at root of WASI filesystem
                    dir_perms,
                    file_perms,
                )
                .map_err(|e| {
                    EngineError::PluginLoadFailed(format!("Failed to preopen directory: {}", e))
                })?;
        }

        // Network capability is implicit in WASI (TCP/UDP sockets)
        // We don't need to explicitly enable it

        let wasi_ctx = wasi_builder.build_p1();
        let limiter = PluginResourceLimiter {
            memory_limit: self.config.max_memory_bytes,
        };
        let host_state = PluginHostState {
            wasi: wasi_ctx,
            limiter,
        };
        let mut store = Store::new(&self.engine, host_state);

        // Set resource limiter (wasmtime v26 API)
        // This enforces memory and table growth limits to prevent DoS attacks
        store.limiter(|state| &mut state.limiter);

        // Set fuel limit (instruction count limit from config)
        // 1 fuel ≈ 1 WASM instruction
        // NOTE: Fuel must be enabled in Config before creating engine (done in load())
        store
            .set_fuel(self.config.max_fuel)
            .map_err(|e| EngineError::PluginLoadFailed(format!("Failed to set fuel: {}", e)))?;

        Ok(store)
    }

    /// Write string to WASM linear memory
    async fn write_string_to_memory(
        &self,
        instance: &Instance,
        store: &mut Store<PluginHostState>,
        data: &str,
    ) -> Result<(u32, u32), EngineError> {
        let memory = instance
            .get_memory(&mut *store, "memory")
            .ok_or_else(|| EngineError::PluginExecutionFailed("No memory export".to_string()))?;

        // Allocate memory in WASM
        let alloc_func = instance
            .get_typed_func::<u32, u32>(&mut *store, "alloc")
            .map_err(|_| EngineError::PluginExecutionFailed("No alloc export".to_string()))?;

        let len = data.len() as u32;
        let ptr = alloc_func
            .call_async(&mut *store, len)
            .await
            .map_err(|e| EngineError::PluginExecutionFailed(format!("Allocation failed: {}", e)))?;

        // Write data to memory
        memory
            .write(&mut *store, ptr as usize, data.as_bytes())
            .map_err(|e| {
                EngineError::PluginExecutionFailed(format!("Memory write failed: {}", e))
            })?;

        Ok((ptr, len))
    }

    /// Read string from WASM linear memory
    fn read_string_from_memory(
        &self,
        instance: &Instance,
        store: &mut Store<PluginHostState>,
        ptr: u32,
        len: u32,
    ) -> Result<String, EngineError> {
        let memory = instance
            .get_memory(&mut *store, "memory")
            .ok_or_else(|| EngineError::PluginExecutionFailed("No memory export".to_string()))?;

        let mut buffer = vec![0u8; len as usize];
        memory
            .read(&*store, ptr as usize, &mut buffer)
            .map_err(|e| {
                EngineError::PluginExecutionFailed(format!("Memory read failed: {}", e))
            })?;

        String::from_utf8(buffer)
            .map_err(|e| EngineError::PluginExecutionFailed(format!("Invalid UTF-8: {}", e)))
    }
}

/// Request structure for plugin execution
#[derive(Debug, Serialize, Deserialize)]
struct ActionRequest {
    pub action: String,
    pub context: TriggerContext,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_config_default() {
        let config = WasmConfig::default();
        assert_eq!(config.max_memory_bytes, 128 * 1024 * 1024);
        assert_eq!(config.max_execution_time, Duration::from_secs(5));
        assert!(config.capabilities.is_empty());
    }

    #[test]
    fn test_resource_limiter() {
        let mut limiter = PluginResourceLimiter {
            memory_limit: 1024 * 1024, // 1 MB
        };

        // Should allow allocations under limit
        assert!(limiter.memory_growing(0, 512 * 1024, None).unwrap());

        // Should deny allocations over limit
        assert!(!limiter.memory_growing(0, 2 * 1024 * 1024, None).unwrap());
    }
}
