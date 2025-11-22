# Execute MIDIMon Phase 3: Daemon & Config Hot-Reload

> **⚠️ NOTE**: This was the PLANNING document for Phase 3. Phase 3 is now **COMPLETE**.
>
> **See**: [phase-3-execution-COMPLETED.md](./phase-3-execution-COMPLETED.md) for the actual execution report.
>
> **Key Deviations**:
> - 19 sub-issues created but NOT closed (remain in Backlog/Todo)
> - Released as v1.0.0 (not v1.5.0)
> - Performance exceeded targets by 5-6x (0-10ms vs 50ms)
> - Documentation via man pages + guides (not mdbook site)
> - Grade: A+ (95%) - Production Ready ✅
> - **Action Required**: Close 14 completed issues, mark 3 menu bar issues as future work

---

**Status**: ✅ COMPLETE (Plan below is for reference)
**Phase**: 3 of 5
**Epic**: AMI-107 (Daemon & Config Hot-Reload)
**Target Version**: v1.5.0 (actually released as v1.0.0)
**Duration**: 3-4 weeks (completed ahead of schedule)
**Total Issues**: 15 planned (19 actually created - AMI-133 through AMI-160, AMI-231, AMI-232)
**Start Date**: 2025-12-30
**End Date**: 2026-01-27

---

## Quick Reference

- **Current Version**: v1.0.0 (399 tests, 88% coverage)
- **Previous Phase**: Phase 2 (AMI-106) - Workspace Architecture ✅ Complete
- **Next Phase**: Phase 4 (AMI-108) - Tauri UI Integration
- **Dependencies**: Phase 3 blocked by AMI-106 (Phase 2 completion)

---

## Phase Overview

### Objectives

Phase 3 transforms MIDIMon from a foreground application into a production-ready background daemon service with:

1. **Daemon Architecture**: Long-running background service with proper lifecycle management
2. **Config Hot-Reload**: Zero-downtime configuration updates using file watching
3. **IPC Communication**: Unix socket/named pipe server for external control
4. **CLI Control Tool**: Command-line interface for daemon management
5. **Menu Bar Presence**: Minimal system tray/menu bar status indicator

### Key Deliverables

**Code**:
- `midimon-daemon/` - Background service with lifecycle management
- `midimon-cli/` - Command-line control tool
- IPC server implementation (Unix sockets on macOS/Linux, named pipes on Windows)
- Config watcher with `notify` crate
- Atomic config reload mechanism

**Documentation**:
- Daemon installation/setup guides
- Hot-reload behavior documentation
- CLI command reference
- Platform-specific service registration guides

**Tests**:
- Daemon lifecycle tests
- Config reload tests (valid/invalid scenarios)
- IPC communication tests
- CLI integration tests

### Success Criteria

- [ ] Daemon runs in background reliably
- [ ] Config hot-reload works without restart
- [ ] IPC communication stable
- [ ] CLI control tool fully functional
- [ ] System service installation works (macOS/Linux/Windows)
- [ ] Menu bar shows status and quick actions
- [ ] Zero crashes during config reload
- [ ] Latency still <1ms during reload
- [ ] Documentation site includes daemon guides

---

## Step 1: Analysis & Planning (Days 1-2)

### 1.1 Dependency Analysis

**Linear Dependencies** (from `docs/linear-dependencies.md`):
```
AMI-107 (Phase 3 Epic)
├── Blocked by: AMI-106 (Phase 2 - Workspace Architecture) ✅ Complete
└── Blocks: AMI-108 (Phase 4 - Tauri UI Integration)

Internal Phase 3 Dependencies:
Week 1: Daemon Architecture
├── AMI-XXX: Create midimon-daemon crate
├── AMI-XXX: Implement service lifecycle
├── AMI-XXX: Add IPC server (Unix sockets/Named pipes)
└── AMI-XXX: Platform-specific service registration

Week 2: Config Hot-Reload (depends on Week 1 daemon)
├── AMI-XXX: Implement config file watcher (notify crate)
├── AMI-XXX: Add config validation before reload
├── AMI-XXX: Implement atomic config reload
└── AMI-XXX: Handle reload errors gracefully

Week 3: CLI Control Tool (depends on Week 1 IPC)
├── AMI-XXX: Create daemon control commands
├── AMI-XXX: Implement service install/uninstall
└── AMI-XXX: Add man page documentation

Week 4: Menu Bar Presence (depends on Week 1 daemon)
├── AMI-XXX: Add minimal menu bar icon (tray-icon crate)
├── AMI-XXX: Implement status display
└── AMI-XXX: Platform-specific menu implementations
```

**Architectural Context** (from `docs/architecture.md`):

Phase 3 builds on the v1.0.0 workspace architecture:
```
midimon/
├── midimon-core/      # Pure engine library (v1.0.0) ✅
├── midimon-daemon/    # CLI binaries (v1.0.0) → Background service (v1.5.0)
└── midimon-cli/       # NEW: Daemon control tool
```

**Implementation References**:

1. **`.research/implementation-viewpoint-1.md`**:
   - Daemon architecture with status-bar app
   - Hot-reload with `notify` crate
   - IPC via crossbeam channels
   - Platform-specific service registration (LaunchAgent on macOS)

2. **`.research/implementation-viewpoint-2.md`**:
   - Tauri-based menu bar app approach
   - Config watching with atomic reloads
   - Launch-at-startup mechanisms
   - Frontmost app detection for context-aware profiles

### 1.2 Code Impact Analysis

**Files to Create**:
- `midimon-daemon/src/daemon.rs` - Daemon lifecycle manager
- `midimon-daemon/src/ipc.rs` - IPC server implementation
- `midimon-daemon/src/config_watcher.rs` - Config hot-reload
- `midimon-daemon/src/service.rs` - Platform service registration
- `midimon-cli/` - New CLI control tool crate
- `midimon-cli/src/main.rs` - CLI entry point
- `midimon-cli/src/commands/` - Command implementations

**Files to Modify**:
- `midimon-daemon/src/main.rs` - Convert to daemon mode
- `midimon-core/src/config/loader.rs` - Add reload validation
- `midimon-core/src/lib.rs` - Expose daemon-ready APIs
- `Cargo.toml` - Add workspace member `midimon-cli`

**New Dependencies**:
```toml
# Daemon & IPC
daemonize = "0.5"          # Unix daemon creation
tokio = { features = ["rt-multi-thread", "net", "sync"] }
interprocess = "2.0"       # Cross-platform IPC (Unix sockets/named pipes)

# Config watching
notify = "6.1"             # File system watcher
notify-debouncer-full = "0.3"  # Debounced events

# Menu bar (optional paths)
tray-icon = "0.14"         # Pure Rust system tray (Tauri team)
# OR
tauri = { version = "2.0", features = ["system-tray"] }  # Full Tauri approach

# Service registration
auto-launch = "0.5"        # Launch at login helper
```

**Test Coverage Plan**:
- Unit tests for config validation before reload
- Integration tests for daemon lifecycle (start, stop, reload, status)
- IPC communication tests (request/response cycle)
- Config watcher tests (modify, move, delete scenarios)
- CLI command tests (mocked IPC)

### 1.3 Risk Assessment

| Risk | Severity | Mitigation |
|------|----------|------------|
| Config reload race conditions | High | Atomic swaps using `Arc<RwLock<Config>>`, thorough testing |
| Platform-specific IPC issues | Medium | Use `interprocess` crate for cross-platform abstractions |
| Daemon crashes during reload | High | Extensive error handling, validation gate before reload |
| Permissions for service install | Medium | Clear documentation, graceful failure messages |
| File watcher high CPU usage | Low | Use debounced watcher, reasonable debounce interval (500ms) |
| Menu bar integration complexity | Medium | Start with minimal implementation (tray-icon), defer rich UI to Phase 4 |

---

## Step 2: Implementation Execution (Days 3-21)

### Week 1: Daemon Architecture (Days 3-7)

#### Issue 1: Create midimon-daemon/ Background Service
**Goal**: Convert `midimon-daemon/src/main.rs` from foreground app to proper daemon

**Implementation**:
```rust
// midimon-daemon/src/daemon.rs
use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tracing::{info, error};

#[derive(Clone)]
pub enum DaemonCommand {
    Reload,
    Stop,
    Status,
}

pub struct Daemon {
    engine: Arc<RwLock<MidiMonEngine>>,
    cmd_tx: broadcast::Sender<DaemonCommand>,
    cmd_rx: broadcast::Receiver<DaemonCommand>,
}

impl Daemon {
    pub fn new(engine: MidiMonEngine) -> Self {
        let (cmd_tx, cmd_rx) = broadcast::channel(10);
        Self {
            engine: Arc::new(RwLock::new(engine)),
            cmd_tx,
            cmd_rx,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting MIDIMon daemon");

        // Start engine in background task
        let engine = Arc::clone(&self.engine);
        tokio::spawn(async move {
            if let Err(e) = engine.write().unwrap().start().await {
                error!("Engine error: {}", e);
            }
        });

        // Command processing loop
        self.command_loop().await
    }

    async fn command_loop(&mut self) -> Result<()> {
        loop {
            match self.cmd_rx.recv().await {
                Ok(DaemonCommand::Reload) => {
                    info!("Reloading configuration");
                    self.reload_config().await?;
                }
                Ok(DaemonCommand::Stop) => {
                    info!("Stopping daemon");
                    break;
                }
                Ok(DaemonCommand::Status) => {
                    // Send status via IPC (handled in ipc.rs)
                }
                Err(e) => {
                    error!("Command channel error: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn reload_config(&self) -> Result<()> {
        let new_config = Config::load()?;
        new_config.validate()?;  // Validation gate

        let mut engine = self.engine.write().unwrap();
        engine.reload_config(new_config)?;

        info!("Configuration reloaded successfully");
        Ok(())
    }

    pub fn command_sender(&self) -> broadcast::Sender<DaemonCommand> {
        self.cmd_tx.clone()
    }
}
```

**Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_daemon_start_stop() {
        let engine = MidiMonEngine::new(Config::default()).unwrap();
        let mut daemon = Daemon::new(engine);

        let cmd_tx = daemon.command_sender();
        tokio::spawn(async move {
            daemon.start().await.unwrap();
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        cmd_tx.send(DaemonCommand::Stop).unwrap();
    }
}
```

#### Issue 2: Implement IPC Server (Unix Sockets / Named Pipes)
**Goal**: Enable external processes to control daemon via IPC

**Implementation**:
```rust
// midimon-daemon/src/ipc.rs
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use tokio::sync::broadcast;

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    Reload,
    Stop,
    Status,
    GetConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub struct IpcServer {
    listener: LocalSocketListener,
    cmd_tx: broadcast::Sender<DaemonCommand>,
}

impl IpcServer {
    pub fn new(cmd_tx: broadcast::Sender<DaemonCommand>) -> Result<Self> {
        let socket_path = Self::socket_path()?;

        // Remove old socket if exists
        let _ = std::fs::remove_file(&socket_path);

        let listener = LocalSocketListener::bind(socket_path)?;
        Ok(Self { listener, cmd_tx })
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("IPC server listening on {}", Self::socket_path()?.display());

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let cmd_tx = self.cmd_tx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream, cmd_tx).await {
                            error!("Client error: {}", e);
                        }
                    });
                }
                Err(e) => error!("Connection error: {}", e),
            }
        }
        Ok(())
    }

    fn socket_path() -> Result<PathBuf> {
        #[cfg(unix)]
        {
            Ok(PathBuf::from("/tmp/midimon.sock"))
        }
        #[cfg(windows)]
        {
            Ok(PathBuf::from(r"\\.\pipe\midimon"))
        }
    }
}

async fn handle_client(
    mut stream: LocalSocketStream,
    cmd_tx: broadcast::Sender<DaemonCommand>,
) -> Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let request: IpcRequest = serde_json::from_str(&line)?;

    let response = match request {
        IpcRequest::Reload => {
            cmd_tx.send(DaemonCommand::Reload)?;
            IpcResponse {
                success: true,
                message: "Reloading configuration".to_string(),
                data: None,
            }
        }
        IpcRequest::Stop => {
            cmd_tx.send(DaemonCommand::Stop)?;
            IpcResponse {
                success: true,
                message: "Stopping daemon".to_string(),
                data: None,
            }
        }
        IpcRequest::Status => {
            IpcResponse {
                success: true,
                message: "Daemon running".to_string(),
                data: Some(serde_json::json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "uptime": "TODO",
                })),
            }
        }
        IpcRequest::GetConfig => {
            // Return current config
            IpcResponse {
                success: true,
                message: "Current configuration".to_string(),
                data: None,  // TODO: serialize config
            }
        }
    };

    let response_json = serde_json::to_string(&response)?;
    stream.write_all(response_json.as_bytes())?;
    stream.write_all(b"\n")?;

    Ok(())
}
```

**Tests**:
```rust
#[tokio::test]
async fn test_ipc_request_response() {
    let (cmd_tx, _cmd_rx) = broadcast::channel(10);
    let mut server = IpcServer::new(cmd_tx).unwrap();

    tokio::spawn(async move {
        server.run().await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect as client
    let socket_path = IpcServer::socket_path().unwrap();
    let mut stream = LocalSocketStream::connect(socket_path).unwrap();

    let request = IpcRequest::Status;
    let request_json = serde_json::to_string(&request).unwrap();
    stream.write_all(request_json.as_bytes()).unwrap();
    stream.write_all(b"\n").unwrap();

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let response: IpcResponse = serde_json::from_str(&line).unwrap();
    assert!(response.success);
}
```

#### Issue 3: Platform-Specific Service Registration
**Goal**: Install daemon as system service (LaunchAgent/systemd/Windows Service)

**Implementation**:
```rust
// midimon-daemon/src/service.rs
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn install_service() -> Result<()> {
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.amiable.midimon</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardErrorPath</key>
    <string>/tmp/midimon.err</string>
    <key>StandardOutPath</key>
    <string>/tmp/midimon.out</string>
</dict>
</plist>"#,
        std::env::current_exe()?.display()
    );

    let plist_path = PathBuf::from(std::env::var("HOME")?)
        .join("Library/LaunchAgents/com.amiable.midimon.plist");

    std::fs::write(&plist_path, plist_content)?;

    // Load service
    Command::new("launchctl")
        .args(&["load", plist_path.to_str().unwrap()])
        .output()?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn install_service() -> Result<()> {
    let service_content = format!(
        r#"[Unit]
Description=MIDIMon MIDI Controller Daemon
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=on-failure
RestartSec=10

[Install]
WantedBy=default.target"#,
        std::env::current_exe()?.display()
    );

    let service_path = PathBuf::from(std::env::var("HOME")?)
        .join(".config/systemd/user/midimon.service");

    std::fs::create_dir_all(service_path.parent().unwrap())?;
    std::fs::write(&service_path, service_content)?;

    // Enable service
    Command::new("systemctl")
        .args(&["--user", "enable", "midimon.service"])
        .output()?;

    Command::new("systemctl")
        .args(&["--user", "start", "midimon.service"])
        .output()?;

    Ok(())
}
```

### Week 2: Config Hot-Reload (Days 8-14)

#### Issue 4: Implement Config File Watcher
**Goal**: Watch config.toml for changes using `notify` crate

**Implementation**:
```rust
// midimon-daemon/src/config_watcher.rs
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, FileIdMap};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;

pub struct ConfigWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    reload_rx: mpsc::Receiver<PathBuf>,
}

impl ConfigWatcher {
    pub fn new(config_path: &Path) -> Result<Self> {
        let (reload_tx, reload_rx) = mpsc::channel(10);
        let config_path_clone = config_path.to_path_buf();

        let debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |result: Result<Vec<DebouncedEvent>, _>| {
                match result {
                    Ok(events) => {
                        for event in events {
                            if event.paths.contains(&config_path_clone) {
                                let _ = reload_tx.blocking_send(config_path_clone.clone());
                            }
                        }
                    }
                    Err(e) => error!("Watch error: {:?}", e),
                }
            },
        )?;

        debouncer.watcher().watch(config_path, RecursiveMode::NonRecursive)?;

        Ok(Self {
            _debouncer: debouncer,
            reload_rx,
        })
    }

    pub async fn wait_for_change(&mut self) -> Option<PathBuf> {
        self.reload_rx.recv().await
    }
}
```

**Tests**:
```rust
#[tokio::test]
async fn test_config_watcher_detects_changes() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    std::fs::write(&config_path, "test = 1").unwrap();

    let mut watcher = ConfigWatcher::new(&config_path).unwrap();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        std::fs::write(&config_path, "test = 2").unwrap();
    });

    let changed_path = tokio::time::timeout(
        Duration::from_secs(2),
        watcher.wait_for_change()
    ).await.unwrap();

    assert!(changed_path.is_some());
}
```

#### Issue 5: Add Config Validation Before Reload
**Goal**: Validate new config before applying to prevent daemon crashes

**Implementation**:
```rust
// midimon-core/src/config/loader.rs (additions)
impl Config {
    /// Validate config and return detailed errors
    pub fn validate_detailed(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();

        // Existing validation
        if let Err(e) = self.validate() {
            errors.push(e);
        }

        // Additional validation for hot-reload safety
        for mode in &self.modes {
            for mapping in &mode.mappings {
                // Validate action is executable
                match &mapping.action {
                    ActionConfig::Launch { app } => {
                        #[cfg(target_os = "macos")]
                        {
                            let check = std::process::Command::new("mdfind")
                                .arg(format!("kMDItemCFBundleIdentifier = '{}'", app))
                                .output();

                            if let Ok(output) = check {
                                if output.stdout.is_empty() {
                                    errors.push(ConfigError::ValidationError(
                                        format!("Application '{}' not found", app)
                                    ));
                                }
                            }
                        }
                    }
                    ActionConfig::Shell { command } => {
                        // Check command exists in PATH
                        let cmd = command.split_whitespace().next().unwrap_or("");
                        if which::which(cmd).is_err() {
                            errors.push(ConfigError::ValidationError(
                                format!("Command '{}' not found in PATH", cmd)
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

#### Issue 6: Implement Atomic Config Reload
**Goal**: Reload config without interrupting active MIDI processing

**Implementation**:
```rust
// midimon-core/src/lib.rs (engine integration)
use parking_lot::RwLock;

pub struct MidiMonEngine {
    config: Arc<RwLock<Config>>,
    mapping_engine: Arc<RwLock<MappingEngine>>,
    // ... other fields
}

impl MidiMonEngine {
    pub fn reload_config(&mut self, new_config: Config) -> Result<()> {
        // Validate first (fail fast)
        new_config.validate_detailed()
            .map_err(|errors| {
                let msg = errors.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ");
                anyhow::anyhow!("Config validation failed: {}", msg)
            })?;

        // Create new mapping engine from config
        let new_mapping_engine = MappingEngine::from_config(&new_config)?;

        // Atomic swap (RwLock write takes exclusive lock)
        {
            *self.config.write() = new_config;
            *self.mapping_engine.write() = new_mapping_engine;
        }

        info!("Config reloaded successfully");
        Ok(())
    }
}
```

**Tests**:
```rust
#[test]
fn test_atomic_config_reload() {
    let config1 = Config::default_config();
    let mut engine = MidiMonEngine::new(config1).unwrap();

    let config2 = Config {
        modes: vec![Mode {
            name: "NewMode".to_string(),
            color: None,
            mappings: vec![],
        }],
        ..Config::default()
    };

    // Should not panic during reload
    engine.reload_config(config2).unwrap();

    let config = engine.config.read();
    assert_eq!(config.modes[0].name, "NewMode");
}

#[test]
fn test_reload_rejects_invalid_config() {
    let config1 = Config::default_config();
    let mut engine = MidiMonEngine::new(config1).unwrap();

    let invalid_config = Config {
        modes: vec![
            Mode { name: "Mode1".to_string(), color: None, mappings: vec![] },
            Mode { name: "Mode1".to_string(), color: None, mappings: vec![] },  // Duplicate
        ],
        ..Config::default()
    };

    let result = engine.reload_config(invalid_config);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Duplicate mode name"));
}
```

#### Issue 7: Handle Reload Errors Gracefully
**Goal**: Log errors, notify user, keep daemon running with old config

**Implementation**:
```rust
// midimon-daemon/src/daemon.rs (update reload_config)
impl Daemon {
    async fn reload_config(&self) -> Result<()> {
        match Config::load() {
            Ok(new_config) => {
                match new_config.validate_detailed() {
                    Ok(()) => {
                        let mut engine = self.engine.write().unwrap();
                        match engine.reload_config(new_config) {
                            Ok(()) => {
                                info!("✓ Configuration reloaded successfully");
                                // Send notification (if enabled)
                                #[cfg(feature = "notifications")]
                                self.send_notification("Config reloaded", "MIDIMon configuration updated successfully");
                                Ok(())
                            }
                            Err(e) => {
                                error!("✗ Config reload failed: {}", e);
                                self.send_notification("Config reload failed", &e.to_string());
                                Err(e)
                            }
                        }
                    }
                    Err(errors) => {
                        let msg = errors.iter()
                            .map(|e| format!("  • {}", e))
                            .collect::<Vec<_>>()
                            .join("\n");
                        error!("✗ Config validation failed:\n{}", msg);
                        self.send_notification("Config validation failed", &msg);
                        Err(anyhow::anyhow!("Validation errors: {}", msg))
                    }
                }
            }
            Err(e) => {
                error!("✗ Failed to load config file: {}", e);
                self.send_notification("Config load failed", &e.to_string());
                Err(e)
            }
        }
    }

    #[cfg(feature = "notifications")]
    fn send_notification(&self, title: &str, body: &str) {
        // Platform-specific notification (notify-rust crate)
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("osascript")
                .arg("-e")
                .arg(format!(
                    r#"display notification "{}" with title "{}""#,
                    body, title
                ))
                .output();
        }
    }
}
```

### Week 3: CLI Control Tool (Days 15-18)

#### Issue 8: Create midimon-cli/ Crate
**Goal**: New CLI tool for daemon control

**Implementation**:
```toml
# midimon-cli/Cargo.toml
[package]
name = "midimon-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
midimon-core = { path = "../midimon-core" }
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
interprocess = "2.0"
colored = "2.1"
```

```rust
// midimon-cli/src/main.rs
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "midimon")]
#[command(about = "MIDIMon daemon control tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the daemon
    Start,
    /// Stop the daemon
    Stop,
    /// Reload configuration
    Reload,
    /// Show daemon status
    Status,
    /// Install as system service
    Install,
    /// Uninstall system service
    Uninstall,
    /// Validate configuration file
    Validate {
        /// Path to config file (default: ~/.config/midimon/config.toml)
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => cmd_start()?,
        Commands::Stop => cmd_stop()?,
        Commands::Reload => cmd_reload()?,
        Commands::Status => cmd_status()?,
        Commands::Install => cmd_install()?,
        Commands::Uninstall => cmd_uninstall()?,
        Commands::Validate { config } => cmd_validate(config)?,
    }

    Ok(())
}
```

```rust
// midimon-cli/src/commands/mod.rs
mod start;
mod stop;
mod reload;
mod status;
mod service;
mod validate;

pub use start::cmd_start;
pub use stop::cmd_stop;
pub use reload::cmd_reload;
pub use status::cmd_status;
pub use service::{cmd_install, cmd_uninstall};
pub use validate::cmd_validate;
```

```rust
// midimon-cli/src/commands/reload.rs
use interprocess::local_socket::LocalSocketStream;
use std::io::{BufRead, BufReader, Write};

pub fn cmd_reload() -> Result<()> {
    println!("{}", "Reloading configuration...".cyan());

    let socket_path = get_socket_path()?;
    let mut stream = LocalSocketStream::connect(socket_path)
        .context("Failed to connect to daemon. Is it running?")?;

    let request = IpcRequest::Reload;
    let request_json = serde_json::to_string(&request)?;
    stream.write_all(request_json.as_bytes())?;
    stream.write_all(b"\n")?;

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    let response: IpcResponse = serde_json::from_str(&line)?;

    if response.success {
        println!("{} {}", "✓".green(), response.message);
    } else {
        eprintln!("{} {}", "✗".red(), response.message);
        std::process::exit(1);
    }

    Ok(())
}
```

**Tests**:
```rust
// midimon-cli/tests/integration_test.rs
#[test]
fn test_cli_status() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--bin", "midimon-cli", "--", "status"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Daemon status"));
}
```

#### Issue 9: Add Man Page Documentation
**Goal**: Create man page for `midimon-cli`

**Implementation**:
```bash
# docs/man/midimon.1
.TH MIDIMON 1 "January 2026" "MIDIMon v1.5.0" "User Commands"
.SH NAME
midimon \- MIDI controller daemon control tool
.SH SYNOPSIS
.B midimon
[\fICOMMAND\fR] [\fIOPTIONS\fR]
.SH DESCRIPTION
\fBmidimon\fR is a command-line tool for controlling the MIDIMon daemon, which transforms MIDI controllers into advanced macro pads with velocity sensitivity, long press detection, and full LED feedback.
.SH COMMANDS
.TP
.B start
Start the MIDIMon daemon
.TP
.B stop
Stop the MIDIMon daemon
.TP
.B reload
Reload the configuration file without restarting
.TP
.B status
Show daemon status and configuration
.TP
.B install
Install MIDIMon as a system service (runs at startup)
.TP
.B uninstall
Remove MIDIMon from system services
.TP
.B validate [\-c CONFIG]
Validate configuration file syntax
.SH OPTIONS
.TP
.B \-c, \-\-config PATH
Path to configuration file (default: ~/.config/midimon/config.toml)
.TP
.B \-h, \-\-help
Display help information
.TP
.B \-V, \-\-version
Display version information
.SH FILES
.TP
.I ~/.config/midimon/config.toml
User configuration file
.TP
.I /tmp/midimon.sock
Unix socket for IPC (macOS/Linux)
.TP
.I ~/Library/LaunchAgents/com.amiable.midimon.plist
macOS LaunchAgent plist
.SH EXAMPLES
.TP
Start the daemon:
.B midimon start
.TP
Reload configuration after editing:
.B midimon reload
.TP
Validate config before applying:
.B midimon validate
.SH SEE ALSO
Full documentation: https://docs.midimon.dev
.SH AUTHOR
Amiable <contact@amiable.dev>
```

### Week 4: Menu Bar Presence (Days 19-21)

#### Issue 10: Add Minimal Menu Bar Icon
**Goal**: System tray icon with status and quick actions

**Implementation**:
```rust
// midimon-daemon/src/menu_bar.rs
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, PredefinedMenuItem}};
use std::sync::Arc;

pub struct MenuBar {
    _tray: TrayIcon,
    cmd_tx: broadcast::Sender<DaemonCommand>,
}

impl MenuBar {
    pub fn new(cmd_tx: broadcast::Sender<DaemonCommand>) -> Result<Self> {
        let tray_menu = Menu::new();

        let status_item = MenuItem::new("MIDIMon Active", true, None);
        status_item.set_enabled(false);  // Display only
        tray_menu.append(&status_item)?;

        tray_menu.append(&PredefinedMenuItem::separator())?;

        let reload_item = MenuItem::new("Reload Config", true, None);
        let configure_item = MenuItem::new("Open Config File", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        tray_menu.append(&reload_item)?;
        tray_menu.append(&configure_item)?;
        tray_menu.append(&PredefinedMenuItem::separator())?;
        tray_menu.append(&quit_item)?;

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("MIDIMon Daemon")
            .with_icon(load_icon()?)
            .build()?;

        // Menu event handler
        let cmd_tx_clone = cmd_tx.clone();
        tray.set_menu_item_handler(move |item| {
            match item.id() {
                "reload" => {
                    let _ = cmd_tx_clone.send(DaemonCommand::Reload);
                }
                "configure" => {
                    let config_path = Config::config_path().unwrap();
                    #[cfg(target_os = "macos")]
                    {
                        let _ = std::process::Command::new("open")
                            .arg(&config_path)
                            .spawn();
                    }
                }
                "quit" => {
                    let _ = cmd_tx_clone.send(DaemonCommand::Stop);
                }
                _ => {}
            }
        });

        Ok(Self {
            _tray: tray,
            cmd_tx,
        })
    }
}

fn load_icon() -> Result<Icon> {
    // Load embedded icon
    let icon_data = include_bytes!("../resources/icon.png");
    Icon::from_rgba(icon_data.to_vec(), 32, 32)
}
```

**Integration** (midimon-daemon/src/main.rs):
```rust
#[tokio::main]
async fn main() -> Result<()> {
    // ... logging setup ...

    let config = Config::load()?;
    let engine = MidiMonEngine::new(config)?;
    let mut daemon = Daemon::new(engine);

    let cmd_tx = daemon.command_sender();

    // Start IPC server
    let mut ipc_server = IpcServer::new(cmd_tx.clone())?;
    tokio::spawn(async move {
        ipc_server.run().await.unwrap();
    });

    // Start config watcher
    let config_path = Config::config_path()?;
    let mut watcher = ConfigWatcher::new(&config_path)?;
    let cmd_tx_watcher = cmd_tx.clone();
    tokio::spawn(async move {
        while let Some(_) = watcher.wait_for_change().await {
            let _ = cmd_tx_watcher.send(DaemonCommand::Reload);
        }
    });

    // Start menu bar (on main thread - macOS requires it)
    let _menu_bar = MenuBar::new(cmd_tx.clone())?;

    // Start daemon
    daemon.start().await?;

    Ok(())
}
```

---

## Step 3: Testing & Validation (Days 22-24)

### Test Suite Checklist

#### Unit Tests
- [ ] `daemon.rs`: Daemon lifecycle (start, stop, reload)
- [ ] `ipc.rs`: IPC request/response handling
- [ ] `config_watcher.rs`: File watcher detects changes
- [ ] `config/loader.rs`: Validation before reload
- [ ] `service.rs`: Service installation (mocked)

#### Integration Tests
- [ ] End-to-end daemon start → IPC reload → config change → reload success
- [ ] Config validation failure prevents reload
- [ ] IPC communication under load (concurrent requests)
- [ ] CLI commands (start, stop, reload, status)
- [ ] Config watcher with multiple rapid changes (debouncing)

#### Manual Testing Scenarios
1. **Fresh Install**:
   ```bash
   cargo build --release
   ./target/release/midimon-cli install
   ./target/release/midimon-cli status
   # Verify: Daemon running, config loaded
   ```

2. **Config Hot-Reload**:
   ```bash
   # Terminal 1: Watch logs
   tail -f ~/.local/share/midimon/logs/midimon.log

   # Terminal 2: Modify config
   vim ~/.config/midimon/config.toml
   # Change a mapping, save

   # Verify: Logs show "Configuration reloaded successfully"
   # Verify: New mapping works immediately
   ```

3. **Invalid Config Rejection**:
   ```bash
   # Introduce duplicate mode name in config
   ./target/release/midimon-cli validate
   # Verify: Shows validation errors

   # Save invalid config
   ./target/release/midimon-cli reload
   # Verify: Reload fails, daemon keeps running with old config
   ```

4. **Service Persistence**:
   ```bash
   ./target/release/midimon-cli install
   sudo reboot
   # After reboot:
   ./target/release/midimon-cli status
   # Verify: Daemon auto-started
   ```

5. **Menu Bar Interaction**:
   - Click menu bar icon
   - Select "Reload Config" → Verify reload
   - Select "Open Config File" → Verify editor opens
   - Select "Quit" → Verify daemon stops

#### Performance Tests
- [ ] Config reload latency: <50ms
- [ ] MIDI event processing during reload: No dropped events
- [ ] Memory usage before/after reload: No leaks
- [ ] CPU usage during config watch: <1%

---

## Step 4: Completion & Documentation (Days 25-28)

### Documentation Site Updates

#### 4.1 Getting Started Guide
**File**: `docs-site/src/getting-started/daemon.md`

```markdown
# Running MIDIMon as a Daemon

MIDIMon v1.5.0+ can run as a background daemon service, providing always-on MIDI controller functionality with hot-reloadable configuration.

## Installation

### Install as System Service

```bash
midimon install
```

This installs MIDIMon to run automatically at system startup:
- **macOS**: Creates LaunchAgent in `~/Library/LaunchAgents/`
- **Linux**: Creates systemd user service
- **Windows**: Installs Windows Service

### Verify Installation

```bash
midimon status
```

Expected output:
```
✓ Daemon running
  Version: v1.5.0
  Uptime: 2h 15m 32s
  Config: ~/.config/midimon/config.toml
  Active mode: Default
```

## Usage

### Start/Stop

```bash
# Start daemon (if not running)
midimon start

# Stop daemon
midimon stop
```

### Reload Configuration

After editing `config.toml`, reload without restarting:

```bash
midimon reload
```

Or simply save the file - the daemon watches for changes automatically.

### Validate Config

Before applying changes, validate syntax:

```bash
midimon validate
```

## Menu Bar Icon

The daemon displays a status icon in your system tray/menu bar:

- **Click**: Quick status view
- **Right-click**: Menu with actions
  - Reload Config
  - Open Config File
  - Quit

## Troubleshooting

### Daemon Won't Start

```bash
# Check logs
tail -f ~/.local/share/midimon/logs/midimon.log

# Check service status (macOS)
launchctl list | grep midimon

# Check service status (Linux)
systemctl --user status midimon
```

### Config Reload Fails

The daemon validates configs before applying. If reload fails:

1. Check validation errors: `midimon validate`
2. Review log file for details
3. Fix issues, then `midimon reload`

The daemon continues running with the previous valid config.

### Permissions Issues

MIDIMon requires:
- **macOS**: Input Monitoring permission (System Settings → Privacy & Security)
- **Linux**: Access to `/dev/midi*` devices (add user to `audio` group)

## See Also

- [Configuration Hot-Reload Guide](../configuration/hot-reload.md)
- [CLI Reference](../reference/cli-commands.md)
```

#### 4.2 Hot-Reload Configuration Guide
**File**: `docs-site/src/configuration/hot-reload.md`

```markdown
# Configuration Hot-Reload

MIDIMon v1.5.0+ supports zero-downtime configuration hot-reloading. Edit your config file and changes apply immediately without restarting the daemon.

## How It Works

1. **File Watcher**: Daemon monitors `config.toml` for changes
2. **Validation Gate**: New config validated before applying
3. **Atomic Swap**: Valid config replaces old atomically
4. **Error Handling**: Invalid configs rejected, old config preserved

## Triggering Reload

### Automatic (File Save)

Simply edit and save `config.toml`:

```bash
vim ~/.config/midimon/config.toml
# Make changes, :wq
# Reload happens automatically within 500ms
```

### Manual (CLI)

```bash
midimon reload
```

### Menu Bar

Click tray icon → "Reload Config"

## Validation

Before applying, the daemon validates:
- TOML syntax
- Required fields present
- Unique mode names
- Valid trigger/action types
- Note/CC ranges (0-127)
- Key/modifier names
- Application bundle IDs (macOS)

### Pre-Validation

Check config before saving:

```bash
midimon validate
```

Example output:
```
✓ Config valid: ~/.config/midimon/config.toml
  Modes: 3
  Global mappings: 2
  Total mappings: 15
```

Or with errors:
```
✗ Config validation failed:
  • Duplicate mode name: 'Development'
  • Invalid modifier: 'invalid_mod' (must be: cmd, shift, alt, ctrl, fn)
  • Note number out of range: 128 (must be 0-127)
```

## What Can't Be Hot-Reloaded

The following require daemon restart:
- Device connections (new MIDI/HID devices)
- Logging configuration
- Advanced settings (timings, debounce)

## Best Practices

1. **Validate First**: Run `midimon validate` before saving
2. **Small Changes**: Test incremental changes
3. **Backup**: Keep `config.toml.backup` before major edits
4. **Watch Logs**: Monitor `~/.local/share/midimon/logs/midimon.log` during reload

## Troubleshooting

### Reload Doesn't Apply

Check file watcher is active:
```bash
midimon status
# Should show: "Config watcher: Active"
```

### Syntax Errors

TOML syntax errors prevent reload:
```
✗ Failed to parse config: expected `.`, `=` at line 15
```

Use a TOML validator: https://www.toml-lint.com/

### Missing Application

macOS checks if apps exist:
```
✗ Application 'com.nonexistent.app' not found
```

Find bundle ID: `osascript -e 'id of app "AppName"'`

## See Also

- [Configuration Reference](../reference/config-toml.md)
- [Daemon Guide](../getting-started/daemon.md)
```

#### 4.3 CLI Reference
**File**: `docs-site/src/reference/cli-commands.md`

```markdown
# CLI Command Reference

MIDIMon CLI (`midimon`) controls the background daemon service.

## Commands

### `midimon start`

Start the MIDIMon daemon (if not already running).

**Usage**:
```bash
midimon start
```

**Example**:
```bash
$ midimon start
✓ Daemon started successfully
  PID: 12345
  Config: ~/.config/midimon/config.toml
```

### `midimon stop`

Stop the MIDIMon daemon gracefully.

**Usage**:
```bash
midimon stop
```

**Example**:
```bash
$ midimon stop
✓ Daemon stopped
```

### `midimon reload`

Reload configuration without restarting the daemon.

**Usage**:
```bash
midimon reload
```

**Example**:
```bash
$ midimon reload
Reloading configuration...
✓ Configuration reloaded successfully
```

### `midimon status`

Show daemon status and configuration summary.

**Usage**:
```bash
midimon status
```

**Example**:
```bash
$ midimon status
✓ Daemon running
  Version: v1.5.0
  Uptime: 1d 3h 25m
  Config: ~/.config/midimon/config.toml
  Active mode: Development
  Connected devices: 1
    • Maschine Mikro MK3 (HID)
```

### `midimon install`

Install MIDIMon as a system service (runs at startup).

**Usage**:
```bash
midimon install
```

**Platform-specific**:
- **macOS**: Creates `~/Library/LaunchAgents/com.amiable.midimon.plist`
- **Linux**: Creates `~/.config/systemd/user/midimon.service`
- **Windows**: Installs Windows Service

**Example**:
```bash
$ midimon install
✓ Service installed successfully
  Location: ~/Library/LaunchAgents/com.amiable.midimon.plist
  Status: Loaded and running
```

### `midimon uninstall`

Remove MIDIMon from system services.

**Usage**:
```bash
midimon uninstall
```

**Example**:
```bash
$ midimon uninstall
✓ Service uninstalled
  Daemon stopped
  Launch agent removed
```

### `midimon validate`

Validate configuration file without applying.

**Usage**:
```bash
midimon validate [--config PATH]
```

**Options**:
- `-c, --config PATH`: Path to config file (default: `~/.config/midimon/config.toml`)

**Example (valid)**:
```bash
$ midimon validate
✓ Config valid: ~/.config/midimon/config.toml
  Modes: 3
  Global mappings: 2
  Total mappings: 15
```

**Example (invalid)**:
```bash
$ midimon validate
✗ Config validation failed:
  • Duplicate mode name: 'Development'
  • Invalid note number: 128 (must be 0-127)
```

## Global Options

- `-h, --help`: Show help information
- `-V, --version`: Show version

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Config validation failed
- `3`: Daemon not running
- `4`: Permission denied

## Environment Variables

- `MIDIMON_CONFIG`: Override config file path
- `MIDIMON_LOG`: Override log level (`debug`, `info`, `warn`, `error`)

## See Also

- [Daemon Guide](../getting-started/daemon.md)
- [Configuration Hot-Reload](../configuration/hot-reload.md)
- Man page: `man midimon`
```

### 4.4 Changelog Update

**File**: `CHANGELOG.md`

```markdown
## [1.5.0] - 2026-01-27

### Added - Phase 3: Daemon & Config Hot-Reload

**Daemon Architecture**:
- Background daemon service with proper lifecycle management
- IPC server using Unix sockets (macOS/Linux) and named pipes (Windows)
- Platform-specific service registration (LaunchAgent/systemd/Windows Service)
- Graceful shutdown and error recovery

**Config Hot-Reload**:
- File watcher using `notify` crate with 500ms debouncing
- Validation gate prevents invalid configs from crashing daemon
- Atomic config swap using `Arc<RwLock<Config>>`
- Detailed validation errors reported to user
- Desktop notifications on reload success/failure

**CLI Control Tool**:
- New `midimon-cli` binary for daemon control
- Commands: `start`, `stop`, `reload`, `status`, `install`, `uninstall`, `validate`
- Colored output with status indicators
- Man page documentation included
- Exit codes for scripting integration

**Menu Bar Presence**:
- Minimal system tray/menu bar icon using `tray-icon` crate
- Quick actions: Reload Config, Open Config File, Quit
- Status display (version, uptime, active mode)
- Cross-platform support (macOS/Linux/Windows)

**Documentation**:
- Complete daemon installation guide
- Configuration hot-reload guide with best practices
- CLI command reference
- Platform-specific setup instructions
- Troubleshooting section for common issues

### Changed
- `midimon-daemon/src/main.rs` converted to daemon mode
- `midimon-core` engine now supports atomic config reload
- Config validation enhanced with application/command checks

### Performance
- Config reload latency: <50ms
- Zero MIDI event drops during reload
- Memory-safe atomic swaps (no leaks)
- CPU usage <1% during file watching

### Testing
- 45+ new tests for daemon lifecycle, IPC, config watcher
- Integration tests for CLI commands
- Manual testing scenarios documented
- Cross-platform validation (macOS/Linux)

---

## [1.0.0] - 2025-11-13 (Previous Release)

Phase 2: Workspace Architecture - Complete
...
```

### 4.5 README Update

**File**: `README.md` (top section)

```markdown
# MIDIMon v1.5.0

> Transform MIDI controllers into advanced macro pads with velocity sensitivity, hot-reloadable configs, and LED feedback

[![Build Status](https://img.shields.io/github/workflow/status/amiable/midimon/CI)](https://github.com/amiable/midimon/actions)
[![Version](https://img.shields.io/badge/version-1.5.0-blue)](https://github.com/amiable/midimon/releases/tag/v1.5.0)
[![Tests](https://img.shields.io/badge/tests-444%20passing-brightgreen)](./docs/testing-report.md)
[![Coverage](https://img.shields.io/badge/coverage-89%25-brightgreen)](./docs/testing-report.md)

## ✨ Features

- 🎹 **MIDI Controller Mapping**: Transform any MIDI device into a powerful macro pad
- ⚡ **Hot-Reloadable Config**: Edit mappings and reload instantly without restart (v1.5.0+)
- 🔄 **Background Daemon**: Always-on service with menu bar control (v1.5.0+)
- 🎛️ **Advanced Triggers**: Velocity ranges, long press, double-tap, chords
- 💡 **Full RGB LED Feedback**: 10 lighting schemes (reactive, rainbow, VU meter)
- 🔧 **CLI Control**: Manage daemon via `midimon` command-line tool (v1.5.0+)
- 🖥️ **Multiple Modes**: Switch mapping sets with encoder rotation
- 📝 **TOML Config**: Human-readable configuration format

## 🚀 Quick Start

### Installation

```bash
# Build from source
cargo build --release

# Install as system service
./target/release/midimon install

# Verify daemon is running
./target/release/midimon status
```

### Configuration

Edit `~/.config/midimon/config.toml`:

```toml
[[modes]]
name = "Development"

[[modes.mappings]]
trigger = { type = "Note", note = 36 }
action = { type = "Keystroke", keys = "cmd+space", modifiers = [] }
description = "Spotlight Search"
```

**Save the file and reload** (automatic or manual):

```bash
midimon reload
```

### Usage

```bash
# Control daemon
midimon start          # Start daemon
midimon stop           # Stop daemon
midimon reload         # Reload config
midimon status         # Show status

# Validate config before applying
midimon validate
```

**Menu bar**: Click tray icon for quick actions (Reload, Open Config, Quit)

## 📚 Documentation

- [Getting Started Guide](./docs-site/src/getting-started/)
- [Daemon & Hot-Reload](./docs-site/src/getting-started/daemon.md)
- [Configuration Reference](./docs-site/src/reference/config-toml.md)
- [CLI Commands](./docs-site/src/reference/cli-commands.md)
- [Architecture Overview](./docs/architecture.md)

## 🏗️ Architecture

MIDIMon v1.5.0 uses a 3-crate workspace architecture:

```
midimon/
├── midimon-core/      # Pure Rust engine library (UI-free)
├── midimon-daemon/    # Background daemon service
└── midimon-cli/       # Command-line control tool
```

**Key capabilities**:
- **Daemon mode**: Runs in background, controlled via IPC
- **Hot-reload**: Config changes apply instantly (atomic swaps)
- **Menu bar**: System tray icon with quick actions
- **Service integration**: LaunchAgent/systemd/Windows Service

See [Architecture Documentation](./docs/architecture.md) for details.

...
```

### Definition of Done Validation

Before marking Phase 3 complete, verify all criteria:

#### ✅ Code Quality
- [ ] All new code follows Rust idioms and project conventions
- [ ] No compiler warnings (clippy clean)
- [ ] All dependencies justified and documented
- [ ] Error handling comprehensive (no unwrap/expect in prod code)

#### ✅ Testing
- [ ] Unit tests pass: `cargo test --package midimon-daemon`
- [ ] Unit tests pass: `cargo test --package midimon-cli`
- [ ] Integration tests pass: `cargo test --workspace`
- [ ] Manual testing scenarios completed (6 scenarios in Step 3)
- [ ] Coverage maintained at ≥88%

#### ✅ Documentation
- [ ] All public APIs documented with rustdoc
- [ ] CHANGELOG.md updated with Phase 3 changes
- [ ] README.md updated with daemon/CLI usage
- [ ] Documentation site updated:
  - [ ] `docs-site/src/getting-started/daemon.md` created
  - [ ] `docs-site/src/configuration/hot-reload.md` created
  - [ ] `docs-site/src/reference/cli-commands.md` updated
  - [ ] `mdbook build` succeeds locally

#### ✅ Performance
- [ ] Config reload latency <50ms
- [ ] No MIDI events dropped during reload
- [ ] Memory usage stable (no leaks detected)
- [ ] CPU usage <1% during file watching

#### ✅ Platform Support
- [ ] Tested on macOS (primary platform)
- [ ] Tested on Linux (secondary)
- [ ] Windows support validated (or known limitations documented)

#### ✅ Release Artifacts
- [ ] Man page installed: `man midimon` works
- [ ] Service files functional (LaunchAgent/systemd)
- [ ] Binary installed to system path
- [ ] Example configs in place

#### ✅ Linear Tracking
- [ ] All 15 Phase 3 issues created in Linear
- [ ] All issues marked "Done" status
- [ ] Epic AMI-107 marked complete
- [ ] Completion comment added with metrics

#### ✅ Git Hygiene
- [ ] All work on `phase-3/daemon-hot-reload` branch
- [ ] Commits follow conventional format
- [ ] PR created with detailed description
- [ ] CI/CD pipeline passes

---

## Post-Completion Tasks

### 1. Linear Update
```bash
# Use Linear MCP to update epic
# Mark AMI-107 status = "Done"
# Add completion comment with metrics
```

### 2. GitHub Release
```bash
git tag -a v1.5.0 -m "Phase 3: Daemon & Config Hot-Reload"
git push origin v1.5.0

# Create GitHub release with:
# - Binaries (midimon-daemon, midimon-cli)
# - Changelog excerpt
# - Installation instructions
```

### 3. Documentation Deployment
```bash
cd docs-site
mdbook build
# Deploy to https://docs.midimon.dev
```

### 4. Announcement
- Update project README on GitHub
- Announce v1.5.0 in project channels
- Highlight key features: daemon mode, hot-reload, CLI

---

## Next Phase Preview

**Phase 4: Tauri UI Integration (AMI-108)**
- Graphical config editor with device visualization
- MIDI Learn mode (click binding → press device)
- Real-time event console for debugging
- Profile management UI
- Frontmost app detection for per-app profiles

**Target Version**: v2.0.0
**Duration**: 4-5 weeks

---

## Appendix: Tool Reference

### Build Commands
```bash
# Build workspace
cargo build --workspace --release

# Build daemon only
cargo build --package midimon-daemon --release

# Build CLI only
cargo build --package midimon-cli --release
```

### Test Commands
```bash
# All tests
cargo test --workspace

# Daemon tests
cargo test --package midimon-daemon

# CLI tests
cargo test --package midimon-cli

# Integration tests
cargo test --test '*'
```

### Benchmarks
```bash
# Config reload latency
cargo bench --bench config_reload

# IPC throughput
cargo bench --bench ipc_performance
```

### Documentation
```bash
# Generate rustdoc
cargo doc --open --no-deps

# Build mdbook site
cd docs-site && mdbook build

# Serve locally
mdbook serve --open
```

### Maintenance
```bash
# Check for outdated deps
cargo outdated

# Update deps (patch versions)
cargo update

# Audit security
cargo audit

# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features
```

---

**Phase 3 Execution Guide v1.0**
**Last Updated**: 2026-01-27
**Maintained By**: Amiable Development Team
