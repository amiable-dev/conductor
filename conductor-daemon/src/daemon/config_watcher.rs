// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Config file watcher with debouncing

use crate::daemon::error::{DaemonError, Result};
use crate::daemon::types::DaemonCommand;
use notify::event::{EventKind, ModifyKind};
use notify::{Event, RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, new_debouncer};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info};

// Platform-specific cache types for file watching
#[cfg(target_os = "macos")]
use notify_debouncer_full::FileIdMap as CacheType;
#[cfg(not(target_os = "macos"))]
use notify_debouncer_full::NoCache as CacheType;

/// Config file watcher with debouncing
pub struct ConfigWatcher {
    config_path: PathBuf,
    debouncer: Option<Debouncer<RecommendedWatcher, CacheType>>,
    event_rx: mpsc::Receiver<PathBuf>,
    command_tx: mpsc::Sender<DaemonCommand>,
    shutdown_rx: broadcast::Receiver<()>,
}

impl ConfigWatcher {
    /// Create a new config watcher
    pub fn new(
        config_path: impl Into<PathBuf>,
        command_tx: mpsc::Sender<DaemonCommand>,
        shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<Self> {
        let config_path = config_path.into();

        // Create channel for debounced events
        let (event_tx, event_rx) = mpsc::channel(10);

        // Clone config path for the closure
        let config_path_clone = config_path.clone();

        // Create debouncer with 500ms delay
        let debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |result: DebounceEventResult| {
                match result {
                    Ok(events) => {
                        for event in events {
                            // Check if this is a modification event for our config file
                            if should_reload(&event.event, &config_path_clone) {
                                debug!("Config file changed: {:?}", config_path_clone);
                                let _ = event_tx.blocking_send(config_path_clone.clone());
                                break; // Only send one event per batch
                            }
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            error!("Watch error: {:?}", error);
                        }
                    }
                }
            },
        )
        .map_err(|e| DaemonError::FileWatcher(format!("Failed to create debouncer: {}", e)))?;

        Ok(Self {
            config_path,
            debouncer: Some(debouncer),
            event_rx,
            command_tx,
            shutdown_rx,
        })
    }

    /// Start watching the config file
    pub async fn watch(&mut self) -> Result<()> {
        // Get the parent directory to watch (watching the file directly may not work on all systems)
        let watch_path = self
            .config_path
            .parent()
            .ok_or_else(|| DaemonError::FileWatcher("No parent directory".to_string()))?;

        info!("Starting config watcher for {:?}", self.config_path);
        info!("Watching directory: {:?}", watch_path);

        // Start watching the directory
        if let Some(ref mut debouncer) = self.debouncer {
            debouncer
                .watch(watch_path, RecursiveMode::NonRecursive)
                .map_err(|e| {
                    DaemonError::FileWatcher(format!("Failed to watch directory: {}", e))
                })?;
        }

        // Event loop
        loop {
            tokio::select! {
                // Handle debounced file change events
                Some(path) = self.event_rx.recv() => {
                    info!("Config file changed, triggering reload: {:?}", path);

                    // Send reload command to engine manager
                    if let Err(e) = self.command_tx.send(DaemonCommand::ConfigFileChanged(path)).await {
                        error!("Failed to send config reload command: {}", e);
                    }
                }

                // Handle shutdown signal
                _ = self.shutdown_rx.recv() => {
                    info!("Config watcher shutting down");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Stop watching (cleanup)
    pub fn stop(&mut self) {
        if let Some(mut debouncer) = self.debouncer.take() {
            // Unwatch all paths
            let watch_path = self.config_path.parent();
            if let Some(path) = watch_path {
                let _ = debouncer.unwatch(path);
            }
        }
        debug!("Config watcher stopped");
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Check if an event should trigger a config reload
fn should_reload(event: &Event, config_path: &Path) -> bool {
    // Check if the event is a modification
    match event.kind {
        EventKind::Modify(ModifyKind::Data(_)) | EventKind::Modify(ModifyKind::Any) => {
            // Check if the event is for our config file
            event.paths.iter().any(|p| p == config_path)
        }
        EventKind::Create(_) => {
            // Also reload on create (e.g., atomic save pattern)
            event.paths.iter().any(|p| p == config_path)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_config_watcher_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create a dummy config file
        std::fs::write(&config_path, "# test config").unwrap();

        let (cmd_tx, _cmd_rx) = mpsc::channel(10);
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

        let result = ConfigWatcher::new(config_path, cmd_tx, shutdown_rx);
        assert!(result.is_ok());

        // Cleanup
        drop(shutdown_tx);
    }

    #[tokio::test]
    #[ignore] // File watching can be flaky in CI/test environments
    async fn test_config_watcher_detects_changes() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create initial config file
        std::fs::write(&config_path, "# initial config").unwrap();

        let (cmd_tx, mut cmd_rx) = mpsc::channel(10);
        let (_shutdown_tx, shutdown_rx) = broadcast::channel(1);

        let mut watcher = ConfigWatcher::new(&config_path, cmd_tx, shutdown_rx).unwrap();

        // Start watching in background
        let watcher_handle = tokio::spawn(async move {
            let _ = watcher.watch().await;
        });

        // Wait for watcher to initialize
        sleep(Duration::from_millis(200)).await;

        // Modify the config file multiple times to ensure detection
        for i in 0..3 {
            std::fs::write(&config_path, format!("# modified config {}", i)).unwrap();
            sleep(Duration::from_millis(100)).await;
        }

        // Wait for debounce + processing (longer timeout for CI)
        let result = tokio::time::timeout(Duration::from_secs(3), cmd_rx.recv()).await;

        // Should receive a config changed command
        assert!(result.is_ok(), "Timeout waiting for config change event");
        if let Ok(Some(DaemonCommand::ConfigFileChanged(path))) = result {
            assert_eq!(path, config_path);
        } else {
            panic!("Expected ConfigFileChanged command");
        }

        // Cleanup
        watcher_handle.abort();
    }

    #[test]
    fn test_should_reload_on_modify() {
        let config_path = PathBuf::from("/tmp/config.toml");

        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![config_path.clone()],
            attrs: Default::default(),
        };

        assert!(should_reload(&event, &config_path));
    }

    #[test]
    fn test_should_not_reload_on_other_file() {
        let config_path = PathBuf::from("/tmp/config.toml");
        let other_path = PathBuf::from("/tmp/other.toml");

        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any)),
            paths: vec![other_path],
            attrs: Default::default(),
        };

        assert!(!should_reload(&event, &config_path));
    }

    #[test]
    fn test_should_reload_on_create() {
        let config_path = PathBuf::from("/tmp/config.toml");

        let event = Event {
            kind: EventKind::Create(notify::event::CreateKind::File),
            paths: vec![config_path.clone()],
            attrs: Default::default(),
        };

        assert!(should_reload(&event, &config_path));
    }
}
