use conductor_core::config::types::Config;

#[test]
fn test_backward_compat_without_new_fields() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true

[[modes]]
name = "Default"
color = "blue"

[[modes.mappings]]
description = "Test"

[modes.mappings.trigger]
type = "Note"
note = 60
velocity_min = 1

[modes.mappings.action]
type = "Keystroke"
keys = "space"
modifiers = ["cmd"]
"#;

    let config: Config =
        toml::from_str(toml_str).expect("Failed to parse old config without new fields");
    assert_eq!(config.device.name, "Test Device");
    assert_eq!(config.device.auto_connect, true);
    assert_eq!(config.device.auto_reconnect, true); // default value
    assert_eq!(config.device.port, None); // default value
}

#[test]
fn test_config_with_new_fields() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true
auto_reconnect = false
port = 2

[[modes]]
name = "Default"
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config with new fields");
    assert_eq!(config.device.name, "Test Device");
    assert_eq!(config.device.auto_connect, true);
    assert_eq!(config.device.auto_reconnect, false);
    assert_eq!(config.device.port, Some(2));
}

#[test]
fn test_config_with_only_auto_reconnect() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true
auto_reconnect = false

[[modes]]
name = "Default"
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config");
    assert_eq!(config.device.auto_reconnect, false);
    assert_eq!(config.device.port, None); // still defaults to None
}

#[test]
fn test_config_with_only_port() {
    let toml_str = r#"
[device]
name = "Test Device"
auto_connect = true
port = 5

[[modes]]
name = "Default"
"#;

    let config: Config = toml::from_str(toml_str).expect("Failed to parse config");
    assert_eq!(config.device.auto_reconnect, true); // still defaults to true
    assert_eq!(config.device.port, Some(5));
}
