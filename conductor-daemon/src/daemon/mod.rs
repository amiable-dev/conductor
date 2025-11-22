// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Daemon infrastructure for background service operation

pub mod config_watcher;
pub mod engine_manager;
pub mod error;
pub mod ipc;
pub mod menu_bar;
pub mod service;
pub mod state;
pub mod types;

pub use config_watcher::ConfigWatcher;
pub use engine_manager::EngineManager;
pub use error::{DaemonError, IpcErrorCode, Result};
pub use ipc::{IpcClient, IpcServer, create_success_response};
pub use menu_bar::{IconState, MenuAction, MenuBar, MenuBarError};
pub use service::{DaemonService, run_daemon, run_daemon_with_config};
pub use state::{
    ConfigInfo, DaemonInfo, EngineInfo, PersistedState, StateManager, calculate_checksum,
    get_socket_path, get_state_dir,
};
pub use types::{
    DaemonCommand, DaemonStatistics, DeviceStatus, ErrorDetails, ErrorEntry, IpcCommand,
    IpcRequest, IpcResponse, LifecycleState, MenuBarAction, MidiDeviceInfo, ReloadMetrics,
    ResponseStatus,
};
