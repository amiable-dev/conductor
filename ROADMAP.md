# MIDIMon Roadmap

This document outlines the vision, current state, and planned development phases for MIDIMon.

## Vision

**Transform any MIDI controller into an advanced, context-aware macro pad with professional-grade feedback and timing-based triggers.**

MIDIMon aims to be the most powerful and flexible MIDI controller mapping system for modern workflows, enabling:

- Musicians to control DAWs and effects with velocity-sensitive, multi-layer mappings
- Developers to streamline coding workflows with mode-based hotkey systems
- Content creators to manage streaming, recording, and editing with physical controls
- Power users to replace expensive dedicated macro pads with affordable MIDI controllers

### Target User Segments

1. **Music Producers**: DAW control, plugin automation, live performance triggers
2. **Software Developers**: Build tools, debuggers, Git workflows, IDE shortcuts
3. **Streamers & Content Creators**: OBS control, scene switching, audio routing
4. **Power Users**: System automation, application launchers, window management

### Differentiation

Unlike existing MIDI mapping tools, MIDIMon provides:

- **Advanced Timing**: Long press, double-tap, chord detection out of the box
- **Velocity Sensitivity**: Different actions for soft/medium/hard pad hits
- **Full RGB Feedback**: Not just on/off LEDs, but animated schemes and reactive color
- **Mode System**: Multiple mapping sets, context-aware switching
- **Open Source**: Fully customizable, extensible, community-driven
- **Modern Architecture**: Fast Rust core, hot-reload config, cross-platform

## Current State: v0.1.0-monolithic

### Implemented Features (26 Total)

**Core Triggers (4)**:
- Note on/off
- Velocity range detection
- Encoder rotation (clockwise/counterclockwise)
- Control Change (CC) messages

**Advanced Triggers (5)**:
- Long press (configurable duration)
- Double-tap (configurable window)
- Note chords (multi-note combinations)
- Aftertouch (pressure sensitivity)
- Pitch bend (touch strip)

**Actions (10)**:
- Keystroke with modifiers
- Text typing
- Application launch
- Shell command execution
- Volume control (up/down/mute/set)
- Mode switching
- Action sequences
- Timed delays
- Mouse clicks
- Repeat actions

**LED Feedback (10 Schemes)**:
- Off, Static, Breathing, Pulse
- Rainbow, Wave, Sparkle
- Reactive (velocity-sensitive)
- VU Meter, Spiral

**System Features (7)**:
- Multi-mode mapping system
- Global mappings (work across all modes)
- Device profile support (.ncmm3 from NI Controller Editor)
- Auto-detect active pad page
- HID shared device access (concurrent with Controller Editor)
- Performance: <1ms latency, 5-10MB memory
- Graceful shutdown with cleanup

### Performance Metrics

- **Response Latency**: <1ms typical for MIDI events
- **Memory Footprint**: 5-10MB steady state
- **CPU Usage**: <1% idle, <5% during active use
- **Binary Size**: 3-5MB (release build with LTO)

### Known Limitations

- macOS only (Linux/Windows support planned)
- Single device support (multi-device planned)
- No GUI (config editing requires TOML knowledge)
- Config reload requires restart (hot reload planned)
- No virtual MIDI output (DAW integration limited)

## Development Phases

### Phase 0: Current State Preservation ✅ COMPLETE

**Timeline**: 2025-11-11 (initial release)
**Status**: Complete
**Goal**: Preserve working monolithic implementation before architectural migration

**Completed Work**:
- GitHub repository setup
- Open source documentation (README, LICENSE, CODE_OF_CONDUCT, CONTRIBUTING)
- CI/CD pipeline (GitHub Actions)
- Project governance (GOVERNANCE.md, MAINTAINERS.md, ROADMAP.md, CHANGELOG.md)
- Community infrastructure (issue templates, PR template, SUPPORT.md)
- Documentation site infrastructure (mdBook)
- Developer setup (DEVELOPMENT.md, build scripts, IDE configs)
- Legal compliance (copyright headers, dependency audit)
- Git tag: v0.1.0-monolithic

**Deliverables**:
- All 26 features working and documented
- Comprehensive open source setup
- Release artifacts (binaries, checksums)
- Foundation for community contributions

### Phase 1: Documentation & Test Coverage

**Timeline**: Q1 2025 (Jan-Mar)
**Status**: Planned
**Goal**: Production-ready documentation and comprehensive test coverage

**Key Objectives**:
- Complete documentation site content (user guides, reference, troubleshooting)
- Achieve 80%+ test coverage (unit + integration tests)
- Write device compatibility guides
- Create video tutorials and demos
- Improve error messages and debugging tools
- Performance benchmarking and optimization

**Deliverables**:
- Full documentation site live on GitHub Pages
- Comprehensive test suite with CI integration
- Video demos and tutorials
- Performance baseline established
- Release: v0.2.0

### Phase 2: Monorepo Foundation

**Timeline**: Q2 2025 (Apr-Jun)
**Status**: Planned
**Goal**: Extract core engine into reusable library, establish workspace structure

**Key Objectives**:
- Create `midimon-core` crate (engine abstraction)
- Migrate existing code to workspace structure
- Extract device abstraction layer
- Implement config file watching (hot reload)
- Add frontmost app detection (macOS)
- Maintain 100% backward compatibility

**Deliverables**:
- Workspace: `midimon-core`, `midimon-daemon`
- Hot config reload without restart
- Per-app profile switching (context-aware)
- Release: v0.3.0

### Phase 3: Tauri GUI & Menu Bar

**Timeline**: Q3 2025 (Jul-Sep)
**Status**: Planned
**Goal**: User-friendly GUI for configuration and management

**Key Objectives**:
- Tauri-based menu bar application
- Visual config editor (no TOML knowledge required)
- MIDI Learn mode (click-to-map workflow)
- Live event console for debugging
- Launch-at-startup functionality
- System tray integration

**Deliverables**:
- `midimon-gui` crate (Tauri v2)
- Visual mapping editor
- MIDI Learn workflow
- Auto-start support
- Release: v0.4.0

### Phase 4: Cross-Platform & Multi-Device

**Timeline**: Q4 2025 (Oct-Dec)
**Status**: Planned
**Goal**: Linux and Windows support, multiple simultaneous devices

**Key Objectives**:
- Linux support (X11, Wayland, udev rules)
- Windows support (USB drivers, input simulation)
- Multi-device simultaneous operation
- Device auto-discovery and hot-plug
- Virtual MIDI output (for DAW routing)
- Advanced mapping: velocity curves, MIDI CC output

**Deliverables**:
- Cross-platform builds (Linux, Windows)
- Multi-device support
- Virtual MIDI ports
- Release: v0.5.0

### Phase 5: Polish & v1.0.0 Release

**Timeline**: Q1 2026 (Jan-Mar)
**Status**: Planned
**Goal**: Stable 1.0.0 release with polished UX and ecosystem

**Key Objectives**:
- API stability guarantee (SemVer commitment)
- Comprehensive device profiles library
- Profile sharing platform (community hub)
- Plugin system for custom actions
- Advanced features (macros, conditional logic, app state)
- Performance tuning and optimization
- Security audit and fuzzing

**Deliverables**:
- v1.0.0 stable release
- Public API frozen (SemVer guarantees)
- 50+ device profiles
- Plugin ecosystem foundation
- Professional documentation and marketing site

## Feature Priorities

### P0 (Critical - Required for 1.0)

- Core stability and reliability
- Comprehensive documentation
- Cross-platform support (macOS, Linux, Windows)
- GUI configuration tool
- Hot config reload
- Multi-device support

### P1 (High - Strongly Desired)

- Tauri UI with MIDI Learn
- Virtual MIDI output
- Per-app profile switching
- Advanced mapping features (velocity curves, conditional actions)
- Community profile library
- Performance optimization

### P2 (Medium - Nice to Have)

- Plugin system for custom actions
- Web-based config editor
- Cloud sync for configs
- Mobile companion app (iOS/Android)
- DAW integration plugins
- Advanced LED animations

### P3 (Low - Future Exploration)

- Machine learning for usage optimization
- Voice control integration
- OSC (Open Sound Control) support
- TouchOSC/Lemur integration
- Hardware module (dedicated device)
- Commercial support offerings

## Technical Debt & Refactoring

### Current Known Issues

- LED feedback abstraction could be simplified
- Event processor state machine needs documentation
- Config validation errors could be more helpful
- Test coverage gaps in LED schemes
- Platform-specific code not fully abstracted

### Planned Refactoring

- Extract device I/O into trait-based abstraction
- Simplify event processing pipeline with state machine pattern
- Improve error handling with `thiserror` and better context
- Separate concerns: core engine vs platform integration
- Add property-based testing for event sequences

## Community Goals

### Short Term (6 months)

- 10+ active contributors
- 50+ GitHub stars
- 5+ community-contributed device profiles
- Active GitHub Discussions engagement
- First external PR merged

### Medium Term (1 year)

- 100+ GitHub stars
- 20+ contributors
- 50+ device profiles
- Translation to 3+ languages
- Conference talk or blog post feature

### Long Term (2 years)

- 500+ GitHub stars
- Establish foundation or fiscal sponsor
- Developer ecosystem (plugins, extensions)
- Commercial support/consulting offerings
- Reference implementation for MIDI mapping standards

## Success Metrics

### Adoption

- GitHub stars, forks, downloads
- Active users (telemetry opt-in)
- Community-contributed profiles
- Blog posts and tutorials from users

### Quality

- Test coverage (target: 80%+)
- Bug report velocity (time to first response, time to fix)
- Code review quality (comments, iteration cycles)
- Documentation completeness and accuracy

### Community Health

- Contributor retention rate
- Issue response time
- PR merge time
- Code of Conduct adherence
- Diversity of contributions (not just code)

## Release Cadence

### Current Plan

- **Minor Releases**: Every 4-6 weeks (v0.x.0)
- **Patch Releases**: As needed for critical bugs (v0.x.y)
- **Major Releases**: When breaking changes necessary (v1.0.0)

### Release Process

1. Feature freeze (1 week before release)
2. Beta testing period (3-7 days)
3. Update CHANGELOG.md
4. Tag release and build binaries
5. Publish GitHub release
6. Announce in Discussions, social media

## Long-Term Vision

By v1.0.0 and beyond, MIDIMon will be:

- The **reference implementation** for advanced MIDI controller mapping
- A **platform** for community-driven device support and profiles
- An **ecosystem** with plugins, extensions, and integrations
- A **standard** for velocity-sensitive, timing-aware input mapping
- A **community** of musicians, developers, and creators

---

**Roadmap Version**: 1.0
**Last Updated**: 2025-11-11
**Next Review**: Q1 2025

For detailed implementation planning, see [docs/implementation-roadmap.md](docs/implementation-roadmap.md).
