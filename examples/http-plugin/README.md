# HTTP Request Plugin for MIDIMon

An example plugin demonstrating the MIDIMon v2.3 plugin architecture. Makes HTTP requests triggered by MIDI events.

## Features

- **HTTP Methods**: GET, POST, PUT, DELETE
- **Custom Headers**: Add authentication, content-type, etc.
- **JSON Body**: Send structured data
- **Velocity Substitution**: Use `{velocity}` in request body
- **Secure**: Requires Network capability (auto-granted as Low risk)

## Installation

1. Build the plugin:
   ```bash
   cd examples/http-plugin
   cargo build --release
   ```

2. Install to MIDIMon plugins directory:
   ```bash
   mkdir -p ~/.midimon/plugins/http_request
   cp target/release/libmidimon_http_plugin.dylib ~/.midimon/plugins/http_request/
   cp plugin.toml ~/.midimon/plugins/http_request/
   ```

3. Restart MIDIMon or run plugin discovery:
   ```bash
   midimonctl reload
   ```

## Usage

### Basic GET Request

```toml
[[modes.mappings]]
trigger = { Note = { note = 60 } }
action = { Plugin = {
    plugin = "http_request",
    params = {
        url = "https://api.example.com/ping"
    }
}}
```

### POST with JSON Body

```toml
[[modes.mappings]]
trigger = { Note = { note = 61, velocity_range = [0, 127] } }
action = { Plugin = {
    plugin = "http_request",
    params = {
        url = "https://api.example.com/events",
        method = "POST",
        headers = {
            "Content-Type" = "application/json",
            "Authorization" = "Bearer YOUR_TOKEN_HERE"
        },
        body = {
            "event_type" = "pad_press",
            "velocity" = "{velocity}",
            "timestamp" = "2025-01-18T12:00:00Z"
        }
    }
}}
```

### Webhook Notification

```toml
[[modes.mappings]]
trigger = { Note = { note = 62 } }
action = { Plugin = {
    plugin = "http_request",
    params = {
        url = "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
        method = "POST",
        body = {
            "text" = "MIDIMon pad was pressed!",
            "username" = "MIDIMon Bot"
        }
    }
}}
```

## Velocity Substitution

The `{velocity}` placeholder in the request body is replaced with the actual MIDI velocity (0-127):

```json
{
    "before": { "intensity": "{velocity}" },
    "after":  { "intensity": 127 }
}
```

Works in:
- Object values
- Nested objects
- Arrays

## Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `url` | string | ✅ Yes | - | Target URL for the HTTP request |
| `method` | string | No | `"GET"` | HTTP method (GET, POST, PUT, DELETE) |
| `headers` | object | No | `{}` | Custom HTTP headers |
| `body` | object | No | `null` | JSON request body (POST/PUT only) |

## Capabilities

This plugin requires the **Network** capability to make HTTP requests.

**Risk Level**: Low (auto-granted)

The Network capability is considered safe and will be automatically granted when the plugin is loaded. You can revoke this permission if needed through the Plugin Manager UI.

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- Plugin metadata
- Capability requirements
- Velocity substitution logic
- Nested object/array handling

## Example Use Cases

### 1. Home Automation
Trigger smart home devices via HTTP APIs (Philips Hue, LIFX, etc.)

### 2. Notifications
Send alerts to Slack, Discord, or custom webhooks

### 3. Analytics
Log events to analytics platforms

### 4. Integration
Connect to Zapier, IFTTT, or n8n for workflow automation

### 5. Remote Control
Trigger actions on remote servers or IoT devices

## Error Handling

The plugin will:
- Return error if URL is missing
- Return error if HTTP method is unsupported
- Return error if request fails (network error, timeout)
- Return error if response status is not 2xx
- Log all requests to stderr for debugging

## Security Notes

- Always use HTTPS URLs in production
- Never commit API tokens to version control
- Use environment variables or secure vaults for secrets
- Review network logs before sharing config files

## Building from Source

```bash
# Clone the MIDIMon repository
git clone https://github.com/amiable-dev/midimon
cd midimon/examples/http-plugin

# Build
cargo build --release

# The plugin binary will be at:
# target/release/libmidimon_http_plugin.dylib (macOS)
# target/release/libmidimon_http_plugin.so (Linux)
# target/release/midimon_http_plugin.dll (Windows)
```

## License

MIT License - see LICENSE file in the MIDIMon repository

## Contributing

This is an example plugin for demonstration purposes. For production plugins, consider:
- More robust error handling
- Request timeout configuration
- Retry logic
- Response caching
- Rate limiting
- Async requests (non-blocking)

See the MIDIMon Plugin Development Guide for best practices.
