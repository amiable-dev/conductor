# Spotify Control Plugin

Control Spotify playback via the Spotify Web API using your MIDI controller.

## Description

This plugin demonstrates network-based control using the Spotify Web API. It provides complete playback control through REST HTTP endpoints, showcasing how WASM plugins can integrate with web services.

## Current Status (v2.5)

**Note:** This is a reference implementation for v2.6. WASI Preview1 doesn't provide high-level HTTP APIs yet. This demonstrates the interface design and protocol structure for the Spotify Web API.

In v2.6, the runtime will provide an HTTP wrapper that enables this plugin to work with real Spotify accounts.

## Supported Actions

| Action | Description | Parameters |
|--------|-------------|------------|
| `play_pause` | Toggle play/pause | None |
| `play` | Resume playback | None |
| `pause` | Pause playback | None |
| `next_track` | Skip to next track | None |
| `previous_track` | Go to previous track | None |
| `volume_up` | Increase volume by 10% | None |
| `volume_down` | Decrease volume by 10% | None |
| `set_volume` | Set volume to specific level | `volume_percent` (0-100) |
| `shuffle` | Toggle shuffle mode | None |
| `repeat` | Cycle repeat mode | None |

## Spotify Web API

This plugin uses the [Spotify Web API](https://developer.spotify.com/documentation/web-api/), which requires:

1. **Spotify Premium Account** - Required for playback control
2. **OAuth Token** - For authentication
3. **Active Device** - Spotify must be running on some device

### API Endpoints Used

- `PUT /v1/me/player/play` - Start playback
- `PUT /v1/me/player/pause` - Pause playback
- `POST /v1/me/player/next` - Next track
- `POST /v1/me/player/previous` - Previous track
- `PUT /v1/me/player/volume` - Set volume
- `PUT /v1/me/player/shuffle` - Toggle shuffle
- `PUT /v1/me/player/repeat` - Set repeat mode

## Building

```bash
cargo build --target wasm32-wasip1 --release
```

The compiled plugin will be at:
`target/wasm32-wasip1/release/midimon_wasm_spotify.wasm`

## Testing

```bash
# Unit tests (native Rust)
cargo test

# Integration tests (WASM)
cd ../..
cargo test --package midimon-core --test spotify_wasm_test --features plugin-wasm
```

**Test Results:** 6/6 unit tests passing (100%)

## Plugin Info

- **Name:** spotify_control
- **Version:** 0.2.0
- **Author:** Amiable Team
- **License:** MIT
- **Binary Size:** 68 KB
- **Capabilities:** network
- **Actions:** 10

## v2.6 Requirements

For this plugin to work with real Spotify accounts in v2.6, the runtime needs:

1. **HTTP Client Wrapper**
   - `http_request(method, url, headers, body)` host function
   - OAuth token management
   - Request/response handling

2. **Authentication**
   - OAuth 2.0 flow
   - Access token refresh
   - Secure credential storage

3. **State Management**
   - Current volume tracking (for volume_up/down)
   - Playback state queries (for play_pause toggle)
   - Device selection

## Use Cases

- Control Spotify during music production sessions
- Quick playback control without switching apps
- Volume adjustment during streaming
- Playlist navigation during DJing
- Hands-free music control while gaming

## Comparison to AppleScript Approach

**Previous Approach (v0.1):** Used AppleScript to control Spotify app
- ❌ macOS only
- ❌ Limited API (no shuffle/repeat)
- ❌ Requires Spotify app to be frontmost
- ❌ Process execution overhead

**New Approach (v0.2):** Uses Spotify Web API
- ✅ Cross-platform (works on any OS)
- ✅ Complete API coverage
- ✅ Works with any Spotify device
- ✅ Lower latency
- ✅ More reliable
- ✅ Remote control support

## Development

See the [WASM Plugin Development Guide](../../docs/WASM_PLUGIN_DEVELOPMENT_GUIDE.md) for detailed information on developing plugins.

For the complete Spotify Web API documentation, see: https://developer.spotify.com/documentation/web-api/
