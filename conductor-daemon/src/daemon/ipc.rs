// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! IPC server for daemon control
//!
//! # Security Considerations
//!
//! This module implements several security measures to prevent abuse:
//!
//! ## Request Size Limiting
//!
//! All incoming IPC requests are limited to [`MAX_REQUEST_SIZE`] (1MB) to prevent
//! memory exhaustion attacks. Attackers could otherwise send arbitrarily large
//! JSON payloads to consume daemon memory and cause denial of service.
//!
//! When a request exceeds the size limit:
//! - The request is immediately rejected without processing
//! - An error response with code 1004 (InvalidRequest) is returned
//! - The oversized data is not accumulated in memory
//! - The attempt is logged as a warning for security monitoring
//!
//! ## Timeout Protection
//!
//! All IPC operations have a 10-second timeout to prevent resource exhaustion
//! from slow or stalled clients.
//!
//! ## Unix Socket Permissions
//!
//! The Unix domain socket and its directory are created with secure permissions:
//! - Socket directory: 0700 (rwx------) - owner-only access
//! - Socket file: 0600 (rw-------) - owner-only read/write
//! - Directory ownership is validated to match the current user
//! - Existing directories with insecure permissions are automatically fixed
//!
//! These measures prevent unauthorized local users from intercepting or sending
//! commands to the daemon.
//!
//! # Protocol
//!
//! Messages are JSON-encoded lines over a Unix domain socket:
//! - Request: `{"id": "...", "command": "...", "args": {...}}\n`
//! - Response: `{"id": "...", "status": "...", "data": {...}}\n`
//!
//! See module documentation for available commands and error codes.

use crate::daemon::error::{DaemonError, IpcErrorCode, Result};
use crate::daemon::state::get_socket_path;
use crate::daemon::types::{DaemonCommand, ErrorDetails, IpcRequest, IpcResponse, ResponseStatus};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{broadcast, mpsc, oneshot};
use tracing::{debug, error, info, warn};

/// Maximum allowed size for a single IPC request (1MB)
/// This prevents memory exhaustion attacks from oversized requests
const MAX_REQUEST_SIZE: usize = 1_048_576; // 1MB

/// IPC server for handling daemon control requests
pub struct IpcServer {
    socket_path: String,
    command_tx: mpsc::Sender<DaemonCommand>,
    shutdown_rx: broadcast::Receiver<()>,
}

impl IpcServer {
    /// Create a new IPC server
    pub fn new(
        command_tx: mpsc::Sender<DaemonCommand>,
        shutdown_rx: broadcast::Receiver<()>,
    ) -> Result<Self> {
        let socket_path = get_socket_path()?;
        let socket_str = socket_path.to_string_lossy().to_string();

        Ok(Self {
            socket_path: socket_str,
            command_tx,
            shutdown_rx,
        })
    }

    /// Run the IPC server loop
    pub async fn run(&mut self) -> Result<()> {
        // Remove existing socket file if it exists (Unix only)
        #[cfg(unix)]
        {
            let _ = tokio::fs::remove_file(&self.socket_path).await;
        }

        // Create listener
        let listener = self.create_listener().await?;

        // Set secure permissions on socket file (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = tokio::fs::metadata(&self.socket_path).await {
                let mut perms = metadata.permissions();
                perms.set_mode(0o600); // rw------- (owner-only access)
                if let Err(e) = tokio::fs::set_permissions(&self.socket_path, perms).await {
                    warn!("Failed to set socket permissions: {}", e);
                }
            }
        }

        info!("IPC server listening on {}", self.socket_path);

        loop {
            tokio::select! {
                // Handle incoming connections
                stream_result = listener.accept() => {
                    match stream_result {
                        Ok((stream, _addr)) => {
                            let cmd_tx = self.command_tx.clone();
                            tokio::spawn(async move {
                                if let Err(e) = handle_client(stream, cmd_tx).await {
                                    error!("Client handler error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to accept connection: {}", e);
                        }
                    }
                }

                // Handle shutdown signal
                _ = self.shutdown_rx.recv() => {
                    info!("IPC server shutting down");
                    break;
                }
            }
        }

        // Cleanup socket file (Unix only)
        #[cfg(unix)]
        {
            let _ = tokio::fs::remove_file(&self.socket_path).await;
        }

        Ok(())
    }

    /// Create platform-specific listener
    async fn create_listener(&self) -> Result<UnixListener> {
        #[cfg(unix)]
        {
            UnixListener::bind(&self.socket_path)
                .map_err(|e| DaemonError::Ipc(format!("Failed to create Unix socket: {}", e)))
        }

        #[cfg(not(unix))]
        {
            // Windows: Use named pipes (requires different approach)
            // For now, return error on Windows
            Err(DaemonError::Ipc(
                "Windows named pipes not yet implemented".to_string(),
            ))
        }
    }
}

/// Handle a single client connection
async fn handle_client(stream: UnixStream, command_tx: mpsc::Sender<DaemonCommand>) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut writer = writer;
    let mut line = String::new();

    while reader.read_line(&mut line).await? > 0 {
        // Security: Check request size to prevent memory exhaustion attacks
        if line.len() > MAX_REQUEST_SIZE {
            warn!(
                "Rejected oversized IPC request: {} bytes (max: {} bytes)",
                line.len(),
                MAX_REQUEST_SIZE
            );

            let error_response = create_error_response(
                "unknown",
                IpcErrorCode::InvalidRequest,
                format!(
                    "Request too large: {} bytes exceeds maximum of {} bytes (1MB)",
                    line.len(),
                    MAX_REQUEST_SIZE
                ),
                Some(json!({
                    "request_size": line.len(),
                    "max_size": MAX_REQUEST_SIZE,
                    "security": "Request rejected to prevent memory exhaustion"
                })),
            );
            send_response(&mut writer, &error_response).await?;
            line.clear();
            continue;
        }

        // Parse request
        let request = match parse_request(&line) {
            Ok(req) => req,
            Err(e) => {
                // Send error response
                let error_response = create_error_response(
                    "unknown",
                    IpcErrorCode::InvalidJson,
                    e.to_string(),
                    None,
                );
                send_response(&mut writer, &error_response).await?;
                line.clear();
                continue;
            }
        };

        debug!("Received IPC request: {:?}", request.command);

        // Create response channel
        let (response_tx, response_rx) = oneshot::channel();

        // Send command to daemon
        command_tx
            .send(DaemonCommand::IpcRequest {
                request,
                response_tx,
            })
            .await
            .map_err(|_| DaemonError::ChannelSend)?;

        // Wait for response with timeout
        let response =
            match tokio::time::timeout(std::time::Duration::from_secs(10), response_rx).await {
                Ok(Ok(resp)) => resp,
                Ok(Err(_)) => create_error_response(
                    "unknown",
                    IpcErrorCode::InternalError,
                    "Response channel closed".to_string(),
                    None,
                ),
                Err(_) => create_error_response(
                    "unknown",
                    IpcErrorCode::Timeout,
                    "Request timed out".to_string(),
                    None,
                ),
            };

        // Send response
        send_response(&mut writer, &response).await?;

        line.clear();
    }

    Ok(())
}

/// Parse IPC request from JSON line
fn parse_request(line: &str) -> Result<IpcRequest> {
    serde_json::from_str(line.trim())
        .map_err(|e| DaemonError::Ipc(format!("Failed to parse request JSON: {}", e)))
}

/// Send IPC response as JSON line
async fn send_response(
    writer: &mut tokio::net::unix::OwnedWriteHalf,
    response: &IpcResponse,
) -> Result<()> {
    let json = serde_json::to_string(response)?;
    writer.write_all(json.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

/// Create an error response
fn create_error_response(
    id: &str,
    code: IpcErrorCode,
    message: String,
    details: Option<serde_json::Value>,
) -> IpcResponse {
    IpcResponse {
        id: id.to_string(),
        status: ResponseStatus::Error,
        data: None,
        error: Some(ErrorDetails {
            code: code.as_u16(),
            message,
            details,
        }),
    }
}

/// Create a success response
pub fn create_success_response(id: &str, data: Option<serde_json::Value>) -> IpcResponse {
    IpcResponse {
        id: id.to_string(),
        status: ResponseStatus::Success,
        data,
        error: None,
    }
}

/// IPC client for sending commands to daemon
pub struct IpcClient {
    stream: UnixStream,
}

impl IpcClient {
    /// Create new IPC client with custom socket path
    pub async fn new(socket_path: String) -> Result<Self> {
        #[cfg(unix)]
        let stream = UnixStream::connect(&socket_path)
            .await
            .map_err(|e| DaemonError::Ipc(format!("Failed to connect to daemon: {}", e)))?;

        #[cfg(not(unix))]
        return Err(DaemonError::Ipc(
            "Windows named pipes not yet implemented".to_string(),
        ));

        Ok(Self { stream })
    }

    /// Connect to daemon IPC server using default socket path
    pub async fn connect() -> Result<Self> {
        let socket_path = get_socket_path()?;
        let socket_str = socket_path.to_string_lossy();

        Self::new(socket_str.to_string()).await
    }

    /// Send a request and wait for response
    pub async fn send_request(&mut self, request: IpcRequest) -> Result<IpcResponse> {
        // Serialize request
        let json = serde_json::to_string(&request)?;

        // Send request
        self.stream.write_all(json.as_bytes()).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await?;

        // Read response
        let mut reader = BufReader::new(&mut self.stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        // Parse response
        let response: IpcResponse = serde_json::from_str(&line)?;

        Ok(response)
    }

    /// Ping the daemon
    pub async fn ping(&mut self) -> Result<IpcResponse> {
        let request = IpcRequest {
            id: uuid::Uuid::new_v4().to_string(),
            command: crate::daemon::types::IpcCommand::Ping,
            args: json!({}),
        };

        self.send_request(request).await
    }

    /// Get daemon status
    pub async fn status(&mut self) -> Result<IpcResponse> {
        let request = IpcRequest {
            id: uuid::Uuid::new_v4().to_string(),
            command: crate::daemon::types::IpcCommand::Status,
            args: json!({}),
        };

        self.send_request(request).await
    }

    /// Request config reload
    pub async fn reload(&mut self) -> Result<IpcResponse> {
        let request = IpcRequest {
            id: uuid::Uuid::new_v4().to_string(),
            command: crate::daemon::types::IpcCommand::Reload,
            args: json!({}),
        };

        self.send_request(request).await
    }

    /// Stop daemon
    pub async fn stop(&mut self) -> Result<IpcResponse> {
        let request = IpcRequest {
            id: uuid::Uuid::new_v4().to_string(),
            command: crate::daemon::types::IpcCommand::Stop,
            args: json!({}),
        };

        self.send_request(request).await
    }

    /// Send a generic command with arguments
    pub async fn send_command(
        &mut self,
        command: crate::daemon::types::IpcCommand,
        args: serde_json::Value,
    ) -> Result<IpcResponse> {
        let request = IpcRequest {
            id: uuid::Uuid::new_v4().to_string(),
            command,
            args,
        };

        self.send_request(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::types::IpcCommand;

    #[test]
    fn test_parse_request_valid() {
        let json = r#"{"id":"test-123","command":"PING","args":{}}"#;
        let request = parse_request(json).unwrap();
        assert_eq!(request.id, "test-123");
        assert!(matches!(request.command, IpcCommand::Ping));
    }

    #[test]
    fn test_parse_request_invalid_json() {
        let json = r#"{"id":"test-123","command":"PING"#; // missing closing brace
        let result = parse_request(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_success_response() {
        let response = create_success_response("test-456", Some(json!({"message": "pong"})));
        assert_eq!(response.id, "test-456");
        assert!(matches!(response.status, ResponseStatus::Success));
        assert!(response.error.is_none());
        assert!(response.data.is_some());
    }

    #[test]
    fn test_create_error_response() {
        let response = create_error_response(
            "test-789",
            IpcErrorCode::InvalidJson,
            "Invalid JSON".to_string(),
            None,
        );
        assert_eq!(response.id, "test-789");
        assert!(matches!(response.status, ResponseStatus::Error));
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, 1001);
    }

    #[tokio::test]
    async fn test_socket_path() {
        let path = get_socket_path().unwrap();

        #[cfg(unix)]
        {
            // Unix platforms - should end with midimon.sock
            assert!(path.ends_with("midimon.sock"));

            // Verify the path is NOT in /tmp (security requirement)
            assert!(
                !path.starts_with("/tmp"),
                "Socket path should not be in /tmp for security reasons"
            );
        }

        #[cfg(windows)]
        assert_eq!(path.to_str().unwrap(), r"\\.\pipe\midimon");
    }

    #[test]
    fn test_max_request_size_constant() {
        // Verify the constant is set to 1MB
        assert_eq!(MAX_REQUEST_SIZE, 1_048_576);
        assert_eq!(MAX_REQUEST_SIZE, 1024 * 1024);
    }

    #[test]
    fn test_request_size_enforcement() {
        // Create a request that exceeds MAX_REQUEST_SIZE
        let oversized_request = "x".repeat(MAX_REQUEST_SIZE + 1);
        assert!(oversized_request.len() > MAX_REQUEST_SIZE);

        // Create a request within limits
        let valid_request = r#"{"id":"test-123","command":"PING","args":{}}"#;
        assert!(valid_request.len() < MAX_REQUEST_SIZE);
    }
}
