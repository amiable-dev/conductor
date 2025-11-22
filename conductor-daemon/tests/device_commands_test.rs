// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Test device management IPC commands

use conductor_daemon::{IpcCommand, IpcRequest, MidiDeviceInfo};
use serde_json::json;

#[test]
fn test_list_devices_command_serialization() {
    let request = IpcRequest {
        id: "test-1".to_string(),
        command: IpcCommand::ListDevices,
        args: json!({}),
    };

    let json_str = serde_json::to_string(&request).unwrap();
    assert!(json_str.contains("LIST_DEVICES"));
    assert!(json_str.contains("test-1"));

    // Verify deserialization
    let parsed: IpcRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.id, "test-1");
    assert!(matches!(parsed.command, IpcCommand::ListDevices));
}

#[test]
fn test_set_device_command_serialization() {
    let request = IpcRequest {
        id: "test-2".to_string(),
        command: IpcCommand::SetDevice,
        args: json!({"port": 2}),
    };

    let json_str = serde_json::to_string(&request).unwrap();
    assert!(json_str.contains("SET_DEVICE"));
    assert!(json_str.contains("test-2"));

    // Verify deserialization
    let parsed: IpcRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.id, "test-2");
    assert!(matches!(parsed.command, IpcCommand::SetDevice));
    assert_eq!(parsed.args.get("port").and_then(|v| v.as_u64()), Some(2));
}

#[test]
fn test_get_device_command_serialization() {
    let request = IpcRequest {
        id: "test-3".to_string(),
        command: IpcCommand::GetDevice,
        args: json!({}),
    };

    let json_str = serde_json::to_string(&request).unwrap();
    assert!(json_str.contains("GET_DEVICE"));
    assert!(json_str.contains("test-3"));

    // Verify deserialization
    let parsed: IpcRequest = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.id, "test-3");
    assert!(matches!(parsed.command, IpcCommand::GetDevice));
}

#[test]
fn test_midi_device_info_serialization() {
    let device = MidiDeviceInfo {
        port_index: 0,
        port_name: "Maschine Mikro MK3".to_string(),
        manufacturer: Some("Native Instruments".to_string()),
        connected: true,
    };

    // Serialize
    let json_str = serde_json::to_string(&device).unwrap();
    assert!(json_str.contains("Maschine Mikro MK3"));
    assert!(json_str.contains("Native Instruments"));

    // Deserialize
    let parsed: MidiDeviceInfo = serde_json::from_str(&json_str).unwrap();
    assert_eq!(parsed.port_index, 0);
    assert_eq!(parsed.port_name, "Maschine Mikro MK3");
    assert_eq!(parsed.manufacturer, Some("Native Instruments".to_string()));
    assert_eq!(parsed.connected, true);
}

#[test]
fn test_device_list_response_format() {
    let devices = vec![
        MidiDeviceInfo {
            port_index: 0,
            port_name: "Maschine Mikro MK3".to_string(),
            manufacturer: Some("Native Instruments".to_string()),
            connected: true,
        },
        MidiDeviceInfo {
            port_index: 1,
            port_name: "IAC Driver Bus 1".to_string(),
            manufacturer: Some("IAC".to_string()),
            connected: false,
        },
    ];

    let response_data = json!({
        "devices": devices
    });

    // Verify structure
    let devices_array = response_data.get("devices").unwrap().as_array().unwrap();
    assert_eq!(devices_array.len(), 2);

    let first_device = &devices_array[0];
    assert_eq!(
        first_device.get("port_index").and_then(|v| v.as_u64()),
        Some(0)
    );
    assert_eq!(
        first_device.get("port_name").and_then(|v| v.as_str()),
        Some("Maschine Mikro MK3")
    );
    assert_eq!(
        first_device.get("connected").and_then(|v| v.as_bool()),
        Some(true)
    );
}
