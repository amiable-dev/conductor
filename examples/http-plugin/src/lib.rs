// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! HTTP Request Plugin for MIDIMon
//!
//! This example plugin demonstrates how to create a MIDIMon plugin that makes
//! HTTP requests. It showcases the plugin API, capability system, and parameter
//! handling.
//!
//! # Features
//!
//! - Make HTTP GET/POST/PUT/DELETE requests
//! - Custom headers support
//! - JSON request body support
//! - Velocity-based parameter substitution
//! - Capability: Network
//!
//! # Example Configuration
//!
//! ```toml
//! [[modes.mappings]]
//! trigger = { Note = { note = 60, velocity_range = [0, 127] } }
//! action = { Plugin = {
//!     plugin = "http_request",
//!     params = {
//!         url = "https://api.example.com/notify",
//!         method = "POST",
//!         headers = {
//!             "Content-Type" = "application/json",
//!             "Authorization" = "Bearer YOUR_TOKEN"
//!         },
//!         body = {
//!             "event" = "pad_pressed",
//!             "velocity" = "{velocity}"
//!         }
//!     }
//! }}
//! ```

use midimon_core::plugin::{ActionPlugin, Capability, TriggerContext};
use serde_json::Value;
use std::error::Error;

/// HTTP Request Plugin
///
/// Makes HTTP requests with configurable method, headers, and body.
pub struct HttpRequestPlugin;

impl ActionPlugin for HttpRequestPlugin {
    fn name(&self) -> &str {
        "http_request"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn description(&self) -> &str {
        "Make HTTP requests (GET, POST, PUT, DELETE) with custom headers and JSON body"
    }

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::Network]
    }

    fn execute(&mut self, params: Value, context: TriggerContext) -> Result<(), Box<dyn Error>> {
        // Parse parameters
        let url = params["url"]
            .as_str()
            .ok_or("Missing 'url' parameter")?;

        let method = params["method"]
            .as_str()
            .unwrap_or("GET")
            .to_uppercase();

        // Build HTTP client
        let client = reqwest::blocking::Client::new();

        // Substitute velocity in body if present
        let body = if let Some(body_value) = params.get("body") {
            substitute_velocity(body_value.clone(), &context)
        } else {
            Value::Null
        };

        // Build request based on method
        let request = match method.as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url).json(&body),
            "PUT" => client.put(url).json(&body),
            "DELETE" => client.delete(url),
            _ => return Err(format!("Unsupported HTTP method: {}", method).into()),
        };

        // Add headers if present
        let request = if let Some(headers) = params.get("headers").and_then(|h| h.as_object()) {
            let mut req = request;
            for (key, value) in headers {
                if let Some(val_str) = value.as_str() {
                    req = req.header(key, val_str);
                }
            }
            req
        } else {
            request
        };

        // Send request
        let response = request.send()?;

        // Check status
        if !response.status().is_success() {
            return Err(format!("HTTP request failed with status: {}", response.status()).into());
        }

        // Log success
        eprintln!(
            "[http_request] {} {} - Status: {}",
            method,
            url,
            response.status()
        );

        Ok(())
    }

    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        eprintln!("[http_request] Plugin initialized");
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        eprintln!("[http_request] Plugin shutdown");
        Ok(())
    }
}

/// Substitute {velocity} placeholders in JSON with actual velocity value
fn substitute_velocity(mut value: Value, context: &TriggerContext) -> Value {
    match &mut value {
        Value::String(s) => {
            if s == "{velocity}" {
                if let Some(vel) = context.velocity {
                    Value::Number(vel.into())
                } else {
                    Value::Number(100.into()) // Default velocity
                }
            } else {
                value
            }
        }
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                *v = substitute_velocity(v.clone(), context);
            }
            value
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                *item = substitute_velocity(item.clone(), context);
            }
            value
        }
        _ => value,
    }
}

/// Plugin creation function
///
/// This function is called by the PluginLoader to create an instance of the plugin.
/// It must be exported with C ABI and return a raw pointer to a boxed trait object.
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn ActionPlugin {
    let plugin = HttpRequestPlugin;
    Box::into_raw(Box::new(plugin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = HttpRequestPlugin;
        assert_eq!(plugin.name(), "http_request");
        assert_eq!(plugin.version(), "1.0.0");
        assert!(!plugin.description().is_empty());
    }

    #[test]
    fn test_capabilities() {
        let plugin = HttpRequestPlugin;
        let caps = plugin.capabilities();
        assert_eq!(caps.len(), 1);
        assert_eq!(caps[0], Capability::Network);
    }

    #[test]
    fn test_velocity_substitution() {
        let context = TriggerContext {
            velocity: Some(127),
            current_mode: None,
            timestamp: std::time::Instant::now(),
        };

        let input = serde_json::json!({
            "event": "test",
            "velocity": "{velocity}"
        });

        let output = substitute_velocity(input, &context);
        assert_eq!(output["velocity"], 127);
    }

    #[test]
    fn test_velocity_substitution_nested() {
        let context = TriggerContext {
            velocity: Some(64),
            current_mode: None,
            timestamp: std::time::Instant::now(),
        };

        let input = serde_json::json!({
            "outer": {
                "inner": {
                    "velocity": "{velocity}"
                }
            }
        });

        let output = substitute_velocity(input, &context);
        assert_eq!(output["outer"]["inner"]["velocity"], 64);
    }

    #[test]
    fn test_velocity_substitution_array() {
        let context = TriggerContext {
            velocity: Some(100),
            current_mode: None,
            timestamp: std::time::Instant::now(),
        };

        let input = serde_json::json!([
            "{velocity}",
            "normal_string",
            { "vel": "{velocity}" }
        ]);

        let output = substitute_velocity(input, &context);
        assert_eq!(output[0], 100);
        assert_eq!(output[1], "normal_string");
        assert_eq!(output[2]["vel"], 100);
    }
}
