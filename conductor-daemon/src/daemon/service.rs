// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Main daemon service orchestrator

use crate::daemon::config_watcher::ConfigWatcher;
use crate::daemon::engine_manager::EngineManager;
use crate::daemon::error::{DaemonError, Result};
use crate::daemon::ipc::IpcServer;
use crate::daemon::state::{DaemonInfo, PersistedState, StateManager, get_state_dir};
use crate::daemon::types::DaemonCommand;
use conductor_core::Config;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};

/// Main daemon service coordinating all components
pub struct DaemonService {
    config_path: PathBuf,
    state_manager: StateManager,
    command_tx: mpsc::Sender<DaemonCommand>,
    command_rx: Option<mpsc::Receiver<DaemonCommand>>,
    shutdown_tx: broadcast::Sender<()>,
    #[allow(dead_code)] // Reserved for future graceful shutdown coordination
    shutdown_rx: Option<broadcast::Receiver<()>>,
}

impl DaemonService {
    /// Create a new daemon service
    pub fn new(config_path: impl Into<PathBuf>) -> Result<Self> {
        let config_path = config_path.into();
        let state_manager = StateManager::new()?;

        // Create command channel (100 message buffer)
        let (command_tx, command_rx) = mpsc::channel(100);

        // Create shutdown broadcast channel
        let (shutdown_tx, shutdown_rx) = broadcast::channel(10);

        Ok(Self {
            config_path,
            state_manager,
            command_tx,
            command_rx: Some(command_rx),
            shutdown_tx,
            shutdown_rx: Some(shutdown_rx),
        })
    }

    /// Run the daemon service
    pub async fn run(&mut self) -> Result<()> {
        info!("MIDIMon daemon starting");

        // Install panic handler for emergency state save
        self.install_panic_handler();

        // Load initial config
        let config = Config::load(self.config_path.to_str().unwrap_or(""))
            .map_err(|e| DaemonError::Ipc(format!("Failed to load config: {}", e)))?;

        info!("Config loaded from {:?}", self.config_path);

        // Create engine manager
        let command_rx = self
            .command_rx
            .take()
            .ok_or_else(|| DaemonError::Fatal("Command receiver already taken".to_string()))?;

        let mut engine_manager = EngineManager::new(
            config,
            self.config_path.clone(),
            command_rx,
            self.command_tx.clone(),
            self.shutdown_tx.clone(),
        )?;

        // Create IPC server
        let shutdown_rx_ipc = self.shutdown_tx.subscribe();
        let mut ipc_server = IpcServer::new(self.command_tx.clone(), shutdown_rx_ipc)?;

        // Spawn IPC server task
        let ipc_handle = tokio::spawn(async move {
            if let Err(e) = ipc_server.run().await {
                error!("IPC server error: {}", e);
            }
            info!("IPC server stopped");
        });

        // Create config watcher
        let shutdown_rx_watcher = self.shutdown_tx.subscribe();
        let mut config_watcher = ConfigWatcher::new(
            self.config_path.clone(),
            self.command_tx.clone(),
            shutdown_rx_watcher,
        )?;

        // Spawn config watcher task
        let watcher_handle = tokio::spawn(async move {
            if let Err(e) = config_watcher.watch().await {
                error!("Config watcher error: {}", e);
            }
            info!("Config watcher stopped");
        });

        // Note: State persistence will be done manually at shutdown
        // to avoid complex lifetime issues with engine_manager reference

        // Spawn signal handler task
        let command_tx = self.command_tx.clone();
        let signal_handle = tokio::spawn(async move {
            Self::signal_handler(command_tx).await;
        });

        // Run engine manager (blocks until shutdown)
        info!("Starting engine manager");
        let engine_result = engine_manager.run().await;

        // Broadcast shutdown to all tasks
        info!("Broadcasting shutdown signal");
        let _ = self.shutdown_tx.send(());

        // Wait for all tasks to complete
        info!("Waiting for tasks to complete");
        let _ = tokio::join!(ipc_handle, watcher_handle, signal_handle);

        // Final state save
        info!("Saving final daemon state");
        if let Err(e) = self.save_state(&engine_manager).await {
            error!("Failed to save final state: {}", e);
        }

        info!("MIDIMon daemon stopped");

        engine_result
    }

    /// Signal handler task
    async fn signal_handler(command_tx: mpsc::Sender<DaemonCommand>) {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{SignalKind, signal};

            let mut sigterm =
                signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
            let mut sigint =
                signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");
            let mut sighup =
                signal(SignalKind::hangup()).expect("Failed to install SIGHUP handler");

            tokio::select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM, initiating graceful shutdown");
                    let _ = command_tx.send(DaemonCommand::Shutdown).await;
                }
                _ = sigint.recv() => {
                    info!("Received SIGINT, initiating graceful shutdown");
                    let _ = command_tx.send(DaemonCommand::Shutdown).await;
                }
                _ = sighup.recv() => {
                    info!("Received SIGHUP, reloading configuration");
                    let _ = command_tx.send(DaemonCommand::ConfigFileChanged(PathBuf::new())).await;
                }
            }
        }

        #[cfg(not(unix))]
        {
            // Windows: Use ctrl_c handler
            if let Err(e) = tokio::signal::ctrl_c().await {
                error!("Failed to listen for ctrl-c: {}", e);
                return;
            }
            info!("Received Ctrl-C, initiating graceful shutdown");
            let _ = command_tx.send(DaemonCommand::Shutdown).await;
        }
    }

    // Note: Periodic state persistence can be added later if needed
    // For now, state is saved only at shutdown to avoid lifetime complexity

    /// Save daemon state
    async fn save_state(&self, engine_manager: &EngineManager) -> Result<()> {
        Self::save_state_internal(&self.state_manager, engine_manager, &self.config_path).await
    }

    /// Internal state save implementation
    async fn save_state_internal(
        state_manager: &StateManager,
        engine_manager: &EngineManager,
        _config_path: &PathBuf,
    ) -> Result<()> {
        let daemon_info = DaemonInfo {
            lifecycle_state: engine_manager.get_state().await,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            pid: process::id(),
        };

        let config_info = engine_manager.get_config_info().await?;
        let engine_info = engine_manager.get_engine_info().await;
        let statistics = engine_manager.get_statistics().await;
        let last_errors = engine_manager.get_recent_errors().await;

        let state = PersistedState::new(
            daemon_info,
            config_info,
            engine_info,
            statistics,
            last_errors,
        );

        state_manager.save(state).await
    }

    /// Install panic handler for emergency state save
    fn install_panic_handler(&self) {
        let state_manager = self.state_manager.clone();

        std::panic::set_hook(Box::new(move |panic_info| {
            error!("PANIC: {}", panic_info);

            // Try to save state before exit
            if let Some(state) = tokio::runtime::Handle::try_current()
                .ok()
                .and_then(|handle| handle.block_on(async { state_manager.get().await }))
                && let Err(e) = state_manager.save_emergency(&state)
            {
                eprintln!("Failed to save emergency state: {}", e);
            }
        }));
    }

    /// Get state manager reference
    pub fn state_manager(&self) -> &StateManager {
        &self.state_manager
    }

    /// Get command sender for external control
    pub fn command_sender(&self) -> mpsc::Sender<DaemonCommand> {
        self.command_tx.clone()
    }
}

/// Run daemon with default config path
pub async fn run_daemon() -> Result<()> {
    // Determine config path
    let config_path = get_state_dir()?.join("config.toml");

    let mut daemon = DaemonService::new(config_path)?;
    daemon.run().await
}

/// Run daemon with custom config path
pub async fn run_daemon_with_config(config_path: impl Into<PathBuf>) -> Result<()> {
    let mut daemon = DaemonService::new(config_path)?;
    daemon.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_daemon_service_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create a minimal config file
        std::fs::write(
            &config_path,
            r#"
            [device]
            name = "Test"
            auto_connect = true

            [[modes]]
            name = "Default"
            color = "blue"
        "#,
        )
        .unwrap();

        let result = DaemonService::new(config_path);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_state_manager_access() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        std::fs::write(
            &config_path,
            r#"
            [device]
            name = "Test"
            auto_connect = true

            [[modes]]
            name = "Default"
            color = "blue"
        "#,
        )
        .unwrap();

        let daemon = DaemonService::new(config_path).unwrap();
        let state_manager = daemon.state_manager();

        // Should be able to save/load state
        let state = state_manager.get().await;
        assert!(state.is_none()); // No state saved yet
    }

    #[test]
    fn test_command_sender() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        std::fs::write(
            &config_path,
            r#"
            [device]
            name = "Test"
            auto_connect = true

            [[modes]]
            name = "Default"
            color = "blue"
        "#,
        )
        .unwrap();

        let daemon = DaemonService::new(config_path).unwrap();
        let _sender = daemon.command_sender();
        // Should be able to get command sender for external control
    }
}
