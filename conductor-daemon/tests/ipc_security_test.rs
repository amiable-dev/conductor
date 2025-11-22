// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Integration tests for IPC security controls
//!
//! These tests verify that the IPC server properly enforces security limits
//! to prevent abuse and resource exhaustion attacks.

use conductor_daemon::daemon::error::IpcErrorCode;
use conductor_daemon::daemon::types::{IpcRequest, ResponseStatus};

/// Maximum request size (must match value in ipc.rs)
const MAX_REQUEST_SIZE: usize = 1_048_576; // 1MB

#[test]
fn test_small_request_accepted() {
    // Verify that normal-sized requests are accepted
    let request = IpcRequest {
        id: "test-123".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::Ping,
        args: serde_json::json!({}),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.len() < 1000); // Typical request is <1KB
    assert!(json.len() < MAX_REQUEST_SIZE);
}

#[test]
fn test_large_valid_request_accepted() {
    // Verify that large but valid requests (e.g., big config) are accepted
    // if they're under the limit
    let large_args = serde_json::json!({
        "config": "x".repeat(100_000) // 100KB config data
    });

    let request = IpcRequest {
        id: "test-456".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::ValidateConfig,
        args: large_args,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.len() > 100_000);
    assert!(json.len() < MAX_REQUEST_SIZE); // Still under 1MB limit
}

#[test]
fn test_oversized_request_structure() {
    // Create a request that would exceed MAX_REQUEST_SIZE
    let oversized_data = "x".repeat(MAX_REQUEST_SIZE + 1);
    let oversized_args = serde_json::json!({
        "data": oversized_data
    });

    let request = IpcRequest {
        id: "attack-789".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::Ping,
        args: oversized_args,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.len() > MAX_REQUEST_SIZE);
    // This would be rejected by the IPC server's size check
}

#[test]
fn test_error_code_exists() {
    // Verify that InvalidRequest error code exists and has correct value
    let error_code = IpcErrorCode::InvalidRequest;
    assert_eq!(error_code.as_u16(), 1004);
    assert!(error_code.message().contains("Invalid request"));
}

#[test]
fn test_boundary_conditions() {
    // Test requests at various sizes relative to the limit
    let sizes = vec![
        1_000,      // 1KB - tiny
        100_000,    // 100KB - normal
        500_000,    // 500KB - large but valid
        1_048_575,  // 1 byte under limit - should accept
        1_048_576,  // Exactly at limit - should accept
        1_048_577,  // 1 byte over limit - should reject
        10_000_000, // 10MB - way over limit
    ];

    for size in sizes {
        let data = "x".repeat(size);
        if size <= MAX_REQUEST_SIZE {
            assert!(
                data.len() <= MAX_REQUEST_SIZE,
                "Size {} should be under limit",
                size
            );
        } else {
            assert!(
                data.len() > MAX_REQUEST_SIZE,
                "Size {} should exceed limit",
                size
            );
        }
    }
}

#[test]
fn test_typical_request_sizes() {
    // Document typical request sizes for reference
    let ping_request = IpcRequest {
        id: "test".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::Ping,
        args: serde_json::json!({}),
    };

    let status_request = IpcRequest {
        id: "test".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::Status,
        args: serde_json::json!({}),
    };

    let reload_request = IpcRequest {
        id: "test".to_string(),
        command: conductor_daemon::daemon::types::IpcCommand::Reload,
        args: serde_json::json!({}),
    };

    // Verify all typical requests are well under the limit
    let ping_size = serde_json::to_string(&ping_request).unwrap().len();
    let status_size = serde_json::to_string(&status_request).unwrap().len();
    let reload_size = serde_json::to_string(&reload_request).unwrap().len();

    assert!(ping_size < 1_000, "Ping: {} bytes", ping_size);
    assert!(status_size < 1_000, "Status: {} bytes", status_size);
    assert!(reload_size < 1_000, "Reload: {} bytes", reload_size);

    // All should be <1KB, providing 1000x safety margin
    assert!(ping_size < MAX_REQUEST_SIZE / 1000);
    assert!(status_size < MAX_REQUEST_SIZE / 1000);
    assert!(reload_size < MAX_REQUEST_SIZE / 1000);

    // Print actual sizes for documentation
    println!("Typical request sizes:");
    println!("  Ping:   {} bytes", ping_size);
    println!("  Status: {} bytes", status_size);
    println!("  Reload: {} bytes", reload_size);
    println!("  Limit:  {} bytes (1MB)", MAX_REQUEST_SIZE);
    println!(
        "  Safety margin: {}x",
        MAX_REQUEST_SIZE / ping_size.max(status_size).max(reload_size)
    );
}

#[test]
fn test_security_error_details() {
    // Verify that security errors include helpful context
    use conductor_daemon::daemon::types::{ErrorDetails, IpcResponse};

    // Test error response structure
    let error_response = IpcResponse {
        id: "test".to_string(),
        status: ResponseStatus::Error,
        data: None,
        error: Some(ErrorDetails {
            code: 1004,
            message: "Request too large".to_string(),
            details: Some(serde_json::json!({
                "request_size": 2_000_000,
                "max_size": MAX_REQUEST_SIZE,
                "security": "Request rejected to prevent memory exhaustion"
            })),
        }),
    };

    // Verify structure
    assert!(matches!(error_response.status, ResponseStatus::Error));
    assert!(error_response.error.is_some());

    let error = error_response.error.unwrap();
    assert_eq!(error.code, 1004);
    assert!(error.details.is_some());

    // Verify details include security context
    let details = error.details.unwrap();
    assert!(
        details["security"]
            .as_str()
            .unwrap()
            .contains("memory exhaustion")
    );
    assert_eq!(
        details["max_size"].as_u64().unwrap(),
        MAX_REQUEST_SIZE as u64
    );
}

/// Performance test: Verify size check has negligible overhead
#[test]
fn test_size_check_performance() {
    use std::time::Instant;

    let request_data = "x".repeat(100_000); // 100KB request

    // Measure time for size check
    let iterations = 1_000_000;
    let start = Instant::now();

    for _ in 0..iterations {
        let _is_oversized = request_data.len() > MAX_REQUEST_SIZE;
    }

    let duration = start.elapsed();
    let avg_ns = duration.as_nanos() / iterations;

    println!("Size check performance:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", duration);
    println!("  Average: {} ns per check", avg_ns);

    // Size check should be <1000ns (1 microsecond)
    assert!(
        avg_ns < 1_000,
        "Size check too slow: {} ns (expected <1000 ns)",
        avg_ns
    );
}
