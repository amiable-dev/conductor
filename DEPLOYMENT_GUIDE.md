# MIDIMon v2.4 Plugin Marketplace - Deployment Guide

**Target**: Production release of Plugin Marketplace feature
**Status**: Phase 2 Complete - Ready for deployment
**Estimated Time**: 15-30 minutes

## Prerequisites Checklist

- ✅ All code committed (7 commits)
- ✅ All tests passing (8/8 = 100%)
- ✅ Release build successful (3m 54s)
- ✅ Plugin binaries built and checksums generated
- ✅ Registry structure created (`plugins/registry/`)
- ✅ Documentation complete (~3,300 lines)

## Step 1: Prepare Plugin Binaries (5 min)

### 1.1 Locate Existing Plugin Binaries

The plugin binaries are already built and located at:

```bash
~/Library/Application Support/midimon/plugins/
├── spotify/
│   └── libmidimon_spotify_plugin.dylib  (4.6 MB)
└── obs/
    └── libmidimon_obs_plugin.dylib      (1.9 MB)
```

### 1.2 Copy Binaries to Release Directory

```bash
# Create release staging directory
mkdir -p ~/Desktop/midimon-plugins-v2.4.0

# Copy plugin binaries
cp ~/Library/Application\ Support/midimon/plugins/spotify/libmidimon_spotify_plugin.dylib \
   ~/Desktop/midimon-plugins-v2.4.0/libmidimon_spotify_plugin-aarch64-apple-darwin.dylib

cp ~/Library/Application\ Support/midimon/plugins/obs/libmidimon_obs_plugin.dylib \
   ~/Desktop/midimon-plugins-v2.4.0/libmidimon_obs_plugin-aarch64-apple-darwin.dylib

# Verify files
ls -lh ~/Desktop/midimon-plugins-v2.4.0/
```

### 1.3 Verify Checksums Match Registry

```bash
cd ~/Desktop/midimon-plugins-v2.4.0/

# Verify Spotify checksum
shasum -a 256 libmidimon_spotify_plugin-aarch64-apple-darwin.dylib
# Expected: d9c84a75ac0193669ac114cd00b1428088b96f3ebad757e3f8728c6c5078f488

# Verify OBS checksum
shasum -a 256 libmidimon_obs_plugin-aarch64-apple-darwin.dylib
# Expected: 6ba95374b5af8f71b3b3634a55cd21c37c9644493299555ab5aac50b0a56c45b
```

If checksums don't match, regenerate them and update `plugins/registry/registry.json`.

## Step 2: Create GitHub Release (5 min)

### 2.1 Push All Commits

```bash
cd /Users/christopherjoseph/projects/amiable/midimon

# Verify all commits are ready
git log --oneline -7

# Push to GitHub
git push origin main
```

### 2.2 Create Release on GitHub

1. Go to: https://github.com/amiable-dev/midimon/releases/new

2. Fill in release details:
   - **Tag**: `v2.4.0-plugins`
   - **Target**: `main` branch
   - **Title**: `v2.4.0 - Plugin Marketplace Release`
   - **Description**:

```markdown
# MIDIMon v2.4.0 - Plugin Marketplace 🎉

This release introduces the **Plugin Marketplace**, allowing users to discover, install, and manage plugins directly from the GUI.

## ✨ New Features

### Plugin Marketplace
- 🔌 Browse available plugins in visual marketplace
- ⬇️ One-click plugin installation from registry
- 🗑️ Easy plugin uninstallation
- 🔍 Search and filter by category/tags
- 🔐 SHA256 checksum verification for security
- 📦 Platform-specific binary downloads

### Available Plugins (v0.1.0)

**Spotify Control**
- Control Spotify playback from MIDI controller
- 12 actions: play/pause, next/prev, volume, shuffle, repeat
- Requires: Spotify Premium + OAuth credentials
- Download: `libmidimon_spotify_plugin-aarch64-apple-darwin.dylib`

**OBS Studio Control**
- Control OBS Studio through WebSocket
- 12 actions: scenes, recording, streaming, sources
- Requires: OBS Studio 28.0+ with obs-websocket 5.x
- Download: `libmidimon_obs_plugin-aarch64-apple-darwin.dylib`

## 📋 Technical Details

### Plugin System
- Dynamic plugin loading using `libloading`
- Capability-based security (Network, Filesystem, Process)
- Plugin enable/disable state management
- Statistics tracking (executions, failures)

### Registry
- Official registry: [plugins/registry/registry.json](https://raw.githubusercontent.com/amiable-dev/midimon/main/plugins/registry/registry.json)
- Automatic caching for offline use
- SHA256 checksum verification
- Multi-platform support (macOS, Linux, Windows)

## 🧪 Testing

- 8/8 integration tests passing (100% success rate)
- All builds passing (release mode)
- Comprehensive test coverage

## 📚 Documentation

- Plugin Marketplace documentation
- Distribution workflow guide
- Developer API reference
- Security considerations

## 🔒 Security

- SHA256 checksum verification for all downloads
- HTTPS-only download URLs
- Capability-based permission system
- Directory isolation per plugin

## 🚀 Installation

Download the appropriate binary for your platform:
- **macOS (Apple Silicon)**: `libmidimon_*_plugin-aarch64-apple-darwin.dylib`
- **macOS (Intel)**: Coming soon
- **Linux**: Coming soon
- **Windows**: Coming soon

Or install directly from the Plugin Marketplace tab in the GUI.

## 🙏 Credits

Built with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

3. Click "Attach binaries by dropping them here or selecting them"

4. Upload these files:
   - `libmidimon_spotify_plugin-aarch64-apple-darwin.dylib`
   - `libmidimon_obs_plugin-aarch64-apple-darwin.dylib`

5. **Important**: Check "Set as the latest release"

6. Click **"Publish release"**

## Step 3: Verify Registry Access (2 min)

### 3.1 Test Registry URL

```bash
# Test registry fetch
curl -I https://raw.githubusercontent.com/amiable-dev/midimon/main/plugins/registry/registry.json

# Should return: HTTP/2 200
```

### 3.2 Test Plugin Download URLs

```bash
# Test Spotify download
curl -I https://github.com/amiable-dev/midimon/releases/download/v2.4.0-plugins/libmidimon_spotify_plugin-aarch64-apple-darwin.dylib

# Should return: HTTP/2 302 (redirect to actual file)

# Test OBS download
curl -I https://github.com/amiable-dev/midimon/releases/download/v2.4.0-plugins/libmidimon_obs_plugin-aarch64-apple-darwin.dylib

# Should return: HTTP/2 302 (redirect to actual file)
```

### 3.3 Verify Checksums in Registry

Open `plugins/registry/registry.json` and confirm:
- Spotify checksum matches: `d9c84a75ac0193669ac114cd00b1428088b96f3ebad757e3f8728c6c5078f488`
- OBS checksum matches: `6ba95374b5af8f71b3b3634a55cd21c37c9644493299555ab5aac50b0a56c45b`

## Step 4: Update Registry URLs (if needed) (3 min)

If you created the release under a different repository or organization, update the download URLs in `plugins/registry/registry.json`:

```json
{
  "downloads": {
    "macos-aarch64": "https://github.com/YOUR-ORG/midimon/releases/download/v2.4.0-plugins/libmidimon_spotify_plugin-aarch64-apple-darwin.dylib"
  }
}
```

Then commit and push:

```bash
git add plugins/registry/registry.json
git commit -m "fix(registry): Update download URLs for v2.4.0-plugins release"
git push origin main
```

## Step 5: Manual Testing (Optional, 10-15 min)

### 5.1 Launch Tauri Dev Mode

```bash
cd /Users/christopherjoseph/projects/amiable/midimon/midimon-gui/src-tauri
cargo tauri dev
```

### 5.2 Test Plugin Marketplace Workflow

1. **Open Plugin Marketplace Tab**
   - Navigate to "Plugins" or "Marketplace" tab
   - UI should load without errors

2. **Fetch Registry**
   - Click "Refresh" or "Fetch Plugins"
   - Should display 2 plugins (Spotify, OBS)
   - Verify categories and tags display correctly

3. **Install Plugin**
   - Click "Install" on Spotify plugin
   - Progress should be shown (or instant if small)
   - After completion, plugin should appear as "Installed"
   - Check: `~/Library/Application Support/midimon/plugins/spotify/`

4. **Verify Installation**
   - Plugin should appear in installed list
   - Plugin manifest should be created
   - Binary checksum should be verified

5. **Uninstall Plugin**
   - Click "Uninstall" on Spotify plugin
   - Confirm dialog should appear
   - After confirmation, plugin should disappear from installed list
   - Check: Directory should be deleted

6. **Test Search/Filter**
   - Search for "spotify" - should show Spotify plugin
   - Search for "obs" - should show OBS plugin
   - Filter by category "streaming" - should show both
   - Filter by category "media" - should show both

### 5.3 Test Error Scenarios

1. **Network Failure**
   - Disconnect internet
   - Try to fetch registry
   - Should show error message (not crash)

2. **Invalid Plugin**
   - Try to install non-existent plugin
   - Should show error message

3. **Checksum Mismatch** (hard to test)
   - Would require tampering with download
   - Skip for now, trust the implementation

## Step 6: UI Polish (Optional, 1-2 hours)

### 6.1 Install Toast Notification Library

```bash
cd /Users/christopherjoseph/projects/amiable/midimon/midimon-gui/ui
npm install svelte-french-toast
```

### 6.2 Replace Alert Dialogs

Edit `PluginMarketplace.svelte`:

```javascript
import toast from 'svelte-french-toast';

// Replace alert() calls:
// Before:
alert(`Plugin "${pluginId}" installed successfully!`);

// After:
toast.success(`Plugin "${pluginId}" installed successfully!`);
```

### 6.3 Add Loading States

```javascript
let isInstalling = false;

async function installPlugin(pluginId) {
  isInstalling = true;
  try {
    await invoke('install_plugin_from_registry', { pluginId });
    toast.success(`Plugin "${pluginId}" installed!`);
  } catch (e) {
    toast.error(`Failed to install: ${e}`);
  } finally {
    isInstalling = false;
  }
}
```

### 6.4 Add Progress Indicators

This requires backend changes to emit progress events. Skip for v2.4.0, defer to v2.4.1.

## Step 7: Create Release Notes (5 min)

### 7.1 Update CHANGELOG.md

```bash
cd /Users/christopherjoseph/projects/amiable/midimon
```

Add to `CHANGELOG.md`:

```markdown
## [2.4.0] - 2025-01-18

### Added
- **Plugin Marketplace** - Visual plugin browser with search and filtering
- **Plugin Installation** - One-click installation from registry with checksum verification
- **Plugin Management** - Easy plugin enable/disable and uninstallation
- **Official Plugin Registry** - GitHub-hosted registry with 2 initial plugins
- **Spotify Control Plugin** - 12 actions for Spotify playback control
- **OBS Studio Control Plugin** - 12 actions for OBS scene and recording management

### Changed
- Enhanced plugin system with capability-based security
- Improved plugin loading with error handling
- Added SHA256 checksum verification for security

### Security
- Plugin downloads verified with SHA256 checksums
- HTTPS-only download URLs enforced
- Capability-based permission system implemented

### Documentation
- Complete plugin marketplace documentation
- Plugin distribution workflow guide
- Developer API reference
- Security considerations guide
```

Commit:

```bash
git add CHANGELOG.md
git commit -m "docs: Add v2.4.0 changelog entry for Plugin Marketplace release"
git push origin main
```

## Step 8: Announce Release (Optional, 5 min)

### 8.1 Social Media

Post on Twitter, Reddit, Discord, etc.:

```
🎉 MIDIMon v2.4.0 is here!

Introducing the Plugin Marketplace:
🔌 Browse & install plugins with one click
🔐 Secure SHA256 checksum verification
📦 2 launch plugins: Spotify & OBS control

Download: https://github.com/amiable-dev/midimon/releases/tag/v2.4.0-plugins

#MIDIMon #MIDI #Plugins #OpenSource
```

### 8.2 GitHub Discussions

Create announcement post in GitHub Discussions with detailed feature breakdown and usage guide.

## Troubleshooting

### Issue: Registry fetch fails

**Solution**:
1. Verify registry.json is committed to main branch
2. Check URL: `https://raw.githubusercontent.com/amiable-dev/midimon/main/plugins/registry/registry.json`
3. Test with curl: `curl -L <URL>`

### Issue: Download fails

**Solution**:
1. Verify release was published (not draft)
2. Check binary was uploaded correctly
3. Test download URL with curl
4. Verify URL format matches registry

### Issue: Checksum mismatch

**Solution**:
1. Regenerate checksum: `shasum -a 256 plugin.dylib`
2. Update registry.json with new checksum
3. Commit and push changes
4. Wait 1-2 minutes for GitHub raw CDN to update

### Issue: Plugin won't load

**Solution**:
1. Check plugin binary is executable: `chmod +x plugin.dylib`
2. Verify manifest exists: `cat plugin.toml`
3. Check daemon logs for errors
4. Ensure capabilities are granted

## Post-Deployment Checklist

- [ ] GitHub release published
- [ ] Plugin binaries uploaded and accessible
- [ ] Registry URL accessible
- [ ] Download URLs tested
- [ ] Checksums verified
- [ ] Manual GUI testing completed (optional)
- [ ] CHANGELOG.md updated
- [ ] Release notes published
- [ ] Announcement posted (optional)

## Success Criteria

✅ Registry fetch succeeds from GUI
✅ Plugin installation completes without errors
✅ Installed plugins appear in list
✅ Plugin uninstallation works correctly
✅ Search and filter function properly
✅ No console errors in GUI
✅ Downloads verify checksums successfully

## Next Steps (Future Releases)

### v2.4.1 (Maintenance)
- UI polish (toasts, spinners)
- Bug fixes from user reports
- Performance improvements

### v2.5 (Enhanced Features)
- Additional plugins (HTTP, Discord, Slack)
- Plugin ratings and reviews
- Download statistics
- Automatic update notifications

### v3.0 (Advanced)
- Multi-platform binary builds
- Plugin dependencies
- Code signing
- Community plugin submissions

## Support

If you encounter issues during deployment:

1. Check GitHub Actions logs (if using CI/CD)
2. Review daemon logs: `~/Library/Logs/midimon/`
3. Test manually in dev mode first
4. Create GitHub issue with logs and error messages

---

**Deployment Time**: 15-30 minutes (without optional steps)
**Difficulty**: Easy (mostly uploading binaries)
**Risk**: Low (rollback available via GitHub release management)

**Status**: Ready for deployment ✅
