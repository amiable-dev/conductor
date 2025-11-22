# MIDIMon Spotify Plugin

Control Spotify playback through the Spotify Web API using your MIDI controller.

## Features

- **Playback Control**: Play, pause, play/pause toggle, next, previous
- **Volume Control**: Set absolute volume or adjust relative to current level
- **Playlist Management**: Start playing specific playlists
- **Shuffle & Repeat**: Toggle shuffle and cycle through repeat modes
- **Track Management**: Like current track to your Liked Songs
- **Now Playing Info**: Get current track, artist, and album information

## Setup

### 1. Create Spotify Application

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new application
3. Note your **Client ID** and **Client Secret**
4. Add `http://localhost:8888/callback` to Redirect URIs in app settings

### 2. Configure Environment Variables

```bash
export RSPOTIFY_CLIENT_ID="your_client_id_here"
export RSPOTIFY_CLIENT_SECRET="your_client_secret_here"
export RSPOTIFY_REDIRECT_URI="http://localhost:8888/callback"
```

Add these to your shell profile (~/.zshrc, ~/.bashrc) to persist across sessions.

### 3. Authenticate

```bash
# First time setup - opens browser for OAuth flow
midimon spotify auth
```

This creates a cached token that persists between sessions.

### 4. Build Plugin

```bash
cd plugins/midimon-spotify-plugin
cargo build --release
```

The compiled plugin will be at `target/release/libmidimon_spotify_plugin.dylib` (macOS) or `.so` (Linux).

### 5. Install Plugin

Copy the compiled library to your MIDIMon plugins directory:

```bash
# macOS
cp target/release/libmidimon_spotify_plugin.dylib ~/.config/midimon/plugins/

# Linux
cp target/release/libmidimon_spotify_plugin.so ~/.config/midimon/plugins/
```

## Usage in Config

### Basic Playback Control

```toml
[[modes.mappings]]
note = 36  # Pad 1
action = { type = "plugin", plugin = "spotify", data = { type = "play_pause" } }

[[modes.mappings]]
note = 37  # Pad 2
action = { type = "plugin", plugin = "spotify", data = { type = "next_track" } }

[[modes.mappings]]
note = 38  # Pad 3
action = { type = "plugin", plugin = "spotify", data = { type = "previous_track" } }
```

### Volume Control

```toml
# Set specific volume level
[[modes.mappings]]
note = 39  # Pad 4
velocity = { range = [81, 127] }  # Hard press
action = { type = "plugin", plugin = "spotify", data = { type = "set_volume", volume = 100 } }

# Adjust volume relative to current
[[modes.mappings]]
note = 40  # Encoder turn clockwise
action = { type = "plugin", plugin = "spotify", data = { type = "adjust_volume", delta = 5 } }

[[modes.mappings]]
note = 41  # Encoder turn counter-clockwise
action = { type = "plugin", plugin = "spotify", data = { type = "adjust_volume", delta = -5 } }
```

### Playlist Quick Launch

```toml
[[modes.mappings]]
note = 42  # Pad 5
action = { type = "plugin", plugin = "spotify", data = {
    type = "play_playlist",
    uri = "spotify:playlist:37i9dQZF1DXcBWIGoYBM5M"  # Today's Top Hits
} }
```

### Shuffle & Repeat

```toml
[[modes.mappings]]
note = 43  # Pad 6
action = { type = "plugin", plugin = "spotify", data = { type = "toggle_shuffle" } }

[[modes.mappings]]
note = 44  # Pad 7
action = { type = "plugin", plugin = "spotify", data = { type = "cycle_repeat" } }
```

### Like Current Track

```toml
[[modes.mappings]]
note = 45  # Pad 8
action = { type = "plugin", plugin = "spotify", data = { type = "like_current_track" } }
```

## Available Actions

| Action | Parameters | Description |
|--------|------------|-------------|
| `play` | None | Resume playback |
| `pause` | None | Pause playback |
| `play_pause` | None | Toggle play/pause |
| `next_track` | None | Skip to next track |
| `previous_track` | None | Go to previous track |
| `set_volume` | `volume: 0-100` | Set absolute volume |
| `adjust_volume` | `delta: -100 to 100` | Adjust volume relatively |
| `toggle_shuffle` | None | Toggle shuffle on/off |
| `cycle_repeat` | None | Cycle: off → context → track → off |
| `play_playlist` | `uri: String` | Play a specific playlist |
| `like_current_track` | None | Save track to Liked Songs |
| `get_now_playing` | None | Get current track info |

## Finding Playlist URIs

1. Open Spotify
2. Right-click a playlist
3. Share → Copy Spotify URI
4. Use the URI in your config (format: `spotify:playlist:...`)

## Troubleshooting

### "Spotify not authenticated" error

Run the authentication flow:
```bash
midimon spotify auth
```

### Token expired

Tokens auto-refresh. If issues persist, re-authenticate:
```bash
rm ~/.config/midimon/spotify_token_cache
midimon spotify auth
```

### No active device

Ensure Spotify is open and playing on at least one device (desktop app, phone, web player).

### Permission denied errors

Check that your Spotify app has the required scopes:
- `user-read-playback-state`
- `user-modify-playback-state`
- `user-read-currently-playing`
- `user-library-modify`
- `user-library-read`
- `playlist-read-private`

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Debugging

Enable debug logging:
```bash
RUST_LOG=midimon_spotify_plugin=debug midimon
```

## License

MIT License - see LICENSE file for details
