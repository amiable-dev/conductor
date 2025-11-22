# Getting Support

Need help with MIDIMon? Here's how to get assistance based on your needs.

## Support Channels

### 1. Documentation (Start Here!)

Before requesting support, please check our comprehensive documentation:

- **Documentation Site**: [https://amiable-dev.github.io/midimon/](https://amiable-dev.github.io/midimon/)
  - Getting Started Guide
  - Configuration Reference
  - Troubleshooting Guide
  - Device Compatibility Matrix
- **README**: Quick start and overview
- **CLAUDE.md**: Detailed project instructions and architecture
- **LED_FEEDBACK.md**: LED system documentation

### 2. GitHub Discussions (Questions & Ideas)

For general questions, feature discussions, and community help:

**[GitHub Discussions](https://github.com/amiable-dev/midimon/discussions)**

- **Q&A**: Ask questions and get answers from the community
- **Ideas**: Discuss feature requests and improvements
- **Show and Tell**: Share your configurations and mappings
- **Device Profiles**: Share and discuss device configs
- **Troubleshooting**: Get help debugging issues

**Response Time**: Usually within 24-48 hours (community-driven)

### 3. Discord (Real-Time Chat)

Coming soon! Check GitHub Discussions for updates.

### 4. GitHub Issues (Bugs Only)

**[GitHub Issues](https://github.com/amiable-dev/midimon/issues)**

**Use issues ONLY for**:
- 🐛 Bug reports (unexpected behavior, crashes)
- 🚀 Feature requests (use Feature Request template)
- 📱 Device support requests (use Device Support template)
- 📚 Documentation issues (use Documentation template)

**Do NOT use issues for**:
- ❌ General questions (use Discussions instead)
- ❌ Configuration help (use Discussions instead)
- ❌ "How do I...?" questions (check docs first, then Discussions)

**Response Time**: Critical bugs within 24-48 hours, others within 1 week

## Before Requesting Support

To help us help you faster, please:

### 1. Search First

- Check if your question has already been answered in [Discussions](https://github.com/amiable-dev/midimon/discussions)
- Search existing [Issues](https://github.com/amiable-dev/midimon/issues)
- Read the [Troubleshooting Guide](https://amiable-dev.github.io/midimon/troubleshooting.html)

### 2. Gather Information

Have this information ready:

- **MIDIMon Version**: Run `cargo run --release -- --version` or check your binary
- **Operating System**: macOS version, Linux distro, or Windows version
- **Architecture**: Apple Silicon (M1/M2/M3) or Intel (x86_64)
- **MIDI Device**: Manufacturer and model
- **Config File**: Relevant portions of your `config.toml`
- **Log Output**: Run with `DEBUG=1` for detailed logs

### 3. Run Diagnostic Commands

```bash
# List available MIDI ports
cargo run --release

# Visualize MIDI events
cargo run --bin midi_diagnostic 2

# Test LED functionality
cargo run --bin led_diagnostic

# Check build information
cargo --version
rustc --version
```

## Reporting Bugs

When reporting a bug, please include:

1. **Clear Description**: What went wrong?
2. **Steps to Reproduce**: Exact steps to trigger the bug
3. **Expected Behavior**: What should happen?
4. **Actual Behavior**: What actually happened?
5. **Environment**:
   - OS and version
   - MIDIMon version
   - Rust version (if building from source)
   - MIDI device
6. **Config Snippet**: Relevant `config.toml` section
7. **Log Output**: Run with `DEBUG=1`
8. **Screenshots/Videos**: If applicable

Use our **[Bug Report template](https://github.com/amiable-dev/midimon/issues/new?template=bug_report.yml)** to ensure you include all necessary information.

## Common Issues

### MIDI Device Not Detected

```bash
# Check USB connection
system_profiler SPUSBDataType | grep -i mikro  # macOS

# List MIDI ports
cargo run --release

# Check Audio MIDI Setup
open -a "Audio MIDI Setup"  # macOS
```

**Solutions**:
- Verify USB cable connection
- Check device is powered on
- Restart device and computer
- Try different USB port
- Check Audio MIDI Setup (macOS) or ALSA (Linux)

### LEDs Not Working

**Solutions**:
- Ensure Native Instruments drivers installed (for NI devices)
- Grant Input Monitoring permission on macOS:
  - System Settings → Privacy & Security → Input Monitoring
- Try different LED schemes: `--led reactive` or `--led rainbow`
- Check `DEBUG=1` output for HID connection errors
- Verify device supports LED control (check compatibility matrix)

### Events Not Triggering

```bash
# Verify MIDI events are being received
cargo run --bin midi_diagnostic 2
```

**Solutions**:
- Verify note numbers match your config
- Check velocity/duration thresholds
- Ensure you're in the correct mode (use encoder to switch)
- Check DEBUG=1 output for event processing
- Verify mapping is in active mode or global_mappings

### Permission Errors (macOS)

**Required Permissions**:
- **Input Monitoring**: For HID device access (LED control)
- **Accessibility** (optional): For some keyboard shortcuts

**Grant Permissions**:
1. System Settings → Privacy & Security
2. Input Monitoring → Add Terminal or your IDE
3. Restart application after granting permissions

### Conflicts with Controller Editor

MIDIMon uses shared device mode (macOS) to allow concurrent access with Native Instruments Controller Editor.

**If issues persist**:
- Quit Controller Editor before running MIDIMon
- Restart device after switching applications
- Check `DEBUG=1` for HID device access messages

## Performance Issues

Expected performance:
- **Latency**: <1ms typical
- **Memory**: 5-10MB
- **CPU**: <1% idle, <5% active

If experiencing performance issues:
- Close other MIDI applications
- Use release build: `cargo build --release`
- Check system resource usage
- Report performance bugs with profiling data

## Security Issues

**DO NOT report security vulnerabilities publicly.**

For security issues, please follow our [Security Policy](SECURITY.md):
- Email: (Check SECURITY.md for contact)
- Provide detailed information privately
- Allow time for a fix before public disclosure

## Response Times

| Channel | Typical Response Time | Best For |
|---------|----------------------|----------|
| Documentation | Instant (self-service) | All questions |
| GitHub Discussions | 24-48 hours | Questions, ideas |
| GitHub Issues | 24-48 hours (critical bugs)<br>1 week (others) | Bugs, features |
| Discord | Real-time to 24 hours | Quick questions |

**Note**: MIDIMon is maintained by volunteers. Response times may vary. We appreciate your patience!

## Contributing

If you'd like to help improve MIDIMon:

- **Fix Bugs**: Submit a pull request
- **Add Features**: Discuss in GitHub Discussions first, then PR
- **Improve Docs**: Documentation PRs always welcome
- **Help Others**: Answer questions in Discussions
- **Test Devices**: Report device compatibility

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Community Guidelines

When seeking support:

- ✅ Be respectful and patient
- ✅ Provide complete information
- ✅ Follow up on your questions
- ✅ Help others when you can
- ✅ Give feedback on solutions
- ❌ Don't demand immediate responses
- ❌ Don't post duplicate questions
- ❌ Don't hijack others' threads

See our [Code of Conduct](CODE_OF_CONDUCT.md) for full community guidelines.

## Language Support

- Primary language: English
- Community translations welcome (contribute via PR)

## Additional Resources

- **GitHub Repository**: [https://github.com/amiable-dev/midimon](https://github.com/amiable-dev/midimon)
- **Issue Tracker**: [https://github.com/amiable-dev/midimon/issues](https://github.com/amiable-dev/midimon/issues)
- **Discussions**: [https://github.com/amiable-dev/midimon/discussions](https://github.com/amiable-dev/midimon/discussions)
- **Releases**: [https://github.com/amiable-dev/midimon/releases](https://github.com/amiable-dev/midimon/releases)
- **CHANGELOG**: [CHANGELOG.md](CHANGELOG.md)
- **Roadmap**: [ROADMAP.md](ROADMAP.md)

---

**Thank you for using MIDIMon!** We're here to help make your MIDI controller experience amazing.
