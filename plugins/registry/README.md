# MIDIMon Plugin Registry

This directory contains the official plugin registry for MIDIMon v2.4+.

## Registry File

**`registry.json`** - The main registry file that lists all available plugins with their metadata, download URLs, checksums, and platform support.

### Registry Schema

The registry follows this structure:

```json
{
  "version": "1.0.0",
  "last_updated": "2025-01-18T00:00:00Z",
  "plugins": [/* array of PluginRegistryEntry */],
  "featured_plugins": [/* plugin IDs */],
  "categories": [/* category definitions */]
}
```

### Plugin Entry Schema

Each plugin entry contains:

- **Identification**: `id`, `name`, `version`, `author`, `license`
- **Description**: `description`, `homepage`, `repository`
- **Classification**: `categories`, `tags`
- **Requirements**: `capabilities`, `platforms`, `min_midimon_version`
- **Downloads**: Platform-specific download URLs and checksums
- **Documentation**: `setup_instructions`, `example_config`, `screenshots`, `video_demo`
- **Metrics**: `install_count`, `rating`, `reviews_count`

## Currently Available Plugins

### 1. Spotify Control (v0.1.0)
- **ID**: `spotify`
- **Platforms**: macOS (aarch64, x86_64), Linux, Windows
- **Capabilities**: Network, Storage
- **Categories**: Media, Streaming, Music
- **Actions**: 12 actions for playback control, volume, shuffle, repeat
- **Requirements**: Spotify Premium, OAuth credentials

**Download URLs**:
- macOS (Apple Silicon): `libmidimon_spotify_plugin-aarch64-apple-darwin.dylib`
- macOS (Intel): `libmidimon_spotify_plugin-x86_64-apple-darwin.dylib`
- Linux: `libmidimon_spotify_plugin-x86_64-unknown-linux-gnu.so`
- Windows: `midimon_spotify_plugin-x86_64-pc-windows-msvc.dll`

**SHA256 Checksum (macOS aarch64)**:
```
d9c84a75ac0193669ac114cd00b1428088b96f3ebad757e3f8728c6c5078f488
```

### 2. OBS Studio Control (v0.1.0)
- **ID**: `obs`
- **Platforms**: macOS (aarch64, x86_64), Linux, Windows
- **Capabilities**: Network
- **Categories**: Media, Streaming, Recording
- **Actions**: 12 actions for scene switching, recording, streaming, source control
- **Requirements**: OBS Studio 28.0+, obs-websocket 5.x

**Download URLs**:
- macOS (Apple Silicon): `libmidimon_obs_plugin-aarch64-apple-darwin.dylib`
- macOS (Intel): `libmidimon_obs_plugin-x86_64-apple-darwin.dylib`
- Linux: `libmidimon_obs_plugin-x86_64-unknown-linux-gnu.so`
- Windows: `midimon_obs_plugin-x86_64-pc-windows-msvc.dll`

**SHA256 Checksum (macOS aarch64)**:
```
6ba95374b5af8f71b3b3634a55cd21c37c9644493299555ab5aac50b0a56c45b
```

## Plugin Distribution Workflow

### For Plugin Developers

1. **Build Release Binaries**
   ```bash
   # Build for your platform
   cargo build --release --package midimon-<plugin>-plugin

   # Generate SHA256 checksum
   shasum -a 256 target/release/libmidimon_<plugin>_plugin.dylib
   ```

2. **Create GitHub Release**
   - Tag format: `v<version>-plugins` (e.g., `v2.4.0-plugins`)
   - Upload platform-specific binaries
   - Include README and setup instructions

3. **Update Registry**
   - Add/update entry in `registry.json`
   - Include all platform download URLs
   - Add real SHA256 checksums
   - Update `last_updated` timestamp
   - Increment `version` if schema changes

4. **Submit Pull Request**
   - Fork the midimon repository
   - Update `plugins/registry/registry.json`
   - Submit PR with plugin details
   - Wait for review and approval

### For MIDIMon Maintainers

1. **Review Plugin Submission**
   - Verify plugin code for security issues
   - Test plugin functionality
   - Validate checksums
   - Check documentation completeness

2. **Approve and Merge**
   - Merge PR to main branch
   - Registry automatically updates for all users

3. **Monitor Usage**
   - Track install counts
   - Monitor ratings and reviews
   - Address reported issues

## Registry URL

The official registry is hosted on GitHub at:

```
https://raw.githubusercontent.com/amiable-dev/midimon/main/plugins/registry/registry.json
```

**Note**: The GUI fetches this URL by default. Users can configure custom registry URLs in advanced settings.

## Categories

The registry supports the following plugin categories:

- **Media Control**: Media players and audio/video applications
- **Live Streaming**: Live streaming and broadcasting tools
- **Music Production**: DAW integration and music production
- **Communication**: Chat, messaging, and communication platforms
- **Home Automation**: Smart home and IoT device control
- **Productivity**: Task management and productivity tools
- **Recording**: Audio and video recording tools

## Checksum Verification

All plugin downloads are verified using SHA256 checksums to ensure integrity and prevent tampering.

**Verification Process**:
1. Download plugin binary from URL
2. Calculate SHA256 checksum
3. Compare with registry checksum
4. Reject installation if mismatch

**Generate Checksum**:
```bash
# macOS/Linux
shasum -a 256 plugin.dylib

# Windows (PowerShell)
Get-FileHash plugin.dll -Algorithm SHA256
```

## Platform Detection

The plugin installer automatically detects your platform and selects the correct binary:

- **macOS**: `macos-aarch64` (Apple Silicon) or `macos-x86_64` (Intel)
- **Linux**: `linux-x86_64`
- **Windows**: `windows-x86_64`

## Future Enhancements

Planned features for the registry system:

- [ ] Plugin version management (multiple versions)
- [ ] Automatic update notifications
- [ ] Plugin ratings and reviews
- [ ] Download statistics and trending
- [ ] Community plugin submissions
- [ ] Code signing and verification
- [ ] Mirror support for faster downloads
- [ ] Plugin dependencies management

## Security Considerations

### For Users

- Only install plugins from the official registry
- Verify checksums before installation
- Review plugin capabilities and permissions
- Check plugin ratings and reviews
- Report suspicious plugins immediately

### For Developers

- Never include credentials in plugin code
- Minimize required capabilities
- Provide clear setup instructions
- Document all network requests
- Use HTTPS for all downloads
- Keep dependencies updated
- Respond to security reports promptly

## Support

- **Plugin Issues**: Report to plugin repository
- **Registry Issues**: Report to main midimon repository
- **Security Issues**: Email security@amiable.dev (private)

## License

Registry metadata: MIT License
Individual plugins: See plugin-specific licenses

---

**Last Updated**: 2025-01-18
**Registry Version**: 1.0.0
**Total Plugins**: 2
**Supported Platforms**: macOS, Linux, Windows
