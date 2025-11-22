# MIDIMon Product Requirements Document
## Universal MIDI Macro Pad System for macOS

**Version**: 2.0 (Modular Architecture)
**Status**: Planning Phase
**Last Updated**: 2025-11-11
**Document Owner**: Product Team

---

## Executive Summary

MIDIMon transforms MIDI controllers into powerful, customizable macro pads for macOS, enabling musicians, developers, streamers, and power users to control their digital workflows with physical hardware. The product bridges the gap between specialized MIDI hardware and general-purpose computing, offering sub-millisecond response times, advanced event detection (velocity sensitivity, long press, chords), and rich visual feedback through RGB LEDs.

### Vision Statement
> To make every MIDI controller a programmable productivity tool, democratizing hardware-based workflow automation for creative professionals and power users.

### Current Status
- **v0.1.0 (Monolithic)**: Fully functional single-binary Rust application
- **Primary Device**: Native Instruments Maschine Mikro MK3 (16 RGB pads, 2 encoders)
- **Key Achievement**: Sub-1ms latency, 5-10MB memory footprint, 10+ trigger types

### Strategic Goals (v2.0)
1. **Preserve Current Implementation**: Tag and maintain working v0.1.0 codebase
2. **Modular Architecture**: Migrate to workspace structure (core engine, daemon, GUI)
3. **Enhanced UX**: Add menu bar app with Tauri-based visual configuration
4. **Ecosystem Expansion**: Support multiple MIDI/HID devices with shared profiles
5. **Community Growth**: Enable profile sharing and template marketplace

---

## Problem Statement

### Current Pain Points

**For Musicians/Producers:**
- MIDI controllers are expensive paperweights when not connected to a DAW
- Limited macro/automation options in live performance scenarios
- No way to map pads to system-level actions or non-music software
- Controller Editor software is device-specific and not extensible

**For Developers/Power Users:**
- External macro pads (Stream Deck, etc.) are expensive ($150-$400)
- Existing MIDI tools lack advanced trigger detection (long press, velocity, chords)
- No macOS-native solution with proper system integration
- Configuration via text files is intimidating for non-technical users

**For Content Creators/Streamers:**
- Need reliable, low-latency triggers for scene switching and effects
- Want physical feedback (LEDs) for mode indication
- Require per-application profile switching
- Limited customization in existing hardware solutions

### Market Opportunity

- **Existing MIDI Hardware**: Millions of unused MIDI controllers worldwide
- **Target Markets**:
  - Music production (15M+ worldwide DAW users)
  - Software development (28M+ developers globally)
  - Content creation (50M+ active creators)
- **Competitive Advantage**: Leverage existing hardware + advanced software features
- **Price Point**: Free/open-source vs. $150-$400 dedicated macro pads

---

## Target Users & Use Cases

### Primary Users
1. **Music Producers** - Repurpose MIDI hardware for DAW control and system shortcuts
2. **Software Developers** - Use pads for IDE shortcuts, build triggers, git operations
3. **Content Creators** - Stream control, scene switching, audio/video triggers
4. **Live Performers** - Reliable macro triggers with visual feedback
5. **Power Users** - Advanced automation and workflow optimization

### Key Use Cases

#### Use Case 1: Music Producer DAW Control
**User**: Alex (see User Personas)
**Goal**: Control Logic Pro without touching mouse/keyboard
**Flow**:
1. Load "Logic Pro" profile when app becomes frontmost
2. Pad 1 (soft press) → Play/Pause
3. Pad 1 (hard press) → Stop and return to start
4. Pad 2 (long press) → Open mixer
5. Encoder 1 → Volume control with LED feedback

#### Use Case 2: Developer Productivity
**User**: Sam (see User Personas)
**Goal**: Quick access to common development tasks
**Flow**:
1. Pad 1 → Run tests (green LED = pass, red LED = fail)
2. Pad 2 → Git commit with terminal prompt
3. Pad 3 (double-tap) → Build and deploy
4. Chord (Pad 1+2) → Emergency: stop all running processes
5. Mode switch via encoder for different project contexts

#### Use Case 3: Live Streaming
**User**: Jordan (see User Personas)
**Goal**: Control OBS scenes and audio effects during stream
**Flow**:
1. Pad 1-8 → Scene switching with mode-color LEDs
2. Pad 9 (long press) → Start/stop recording
3. Pad 10-12 → Audio filters (mute mic, audio ducking, etc.)
4. Encoder → Transition speed control
5. Visual feedback confirms action execution

---

## Product Goals & Success Metrics

### Goals (6-12 Months)

**G1: Preserve & Document Current Implementation**
- Tag v0.1.0-monolithic with full feature documentation
- Zero breaking changes to existing config.toml format
- 100% feature parity during migration

**G2: Complete Architecture Migration**
- Extract midimon-core as reusable library
- Build midimon-daemon with menu bar integration
- Launch midimon-gui with visual configuration (Tauri v2)
- Maintain <1ms latency and <10MB memory footprint

**G3: Enhance User Experience**
- MIDI Learn mode (click + press pad = auto-configure)
- Hot config reload (no app restart needed)
- Per-app profile switching (frontmost app detection)
- Visual configuration UI for non-technical users

**G4: Expand Device Support**
- Support 5+ popular MIDI controllers (Launchpad, APC, nanoPad)
- Generic device template system for unknown hardware
- HID device support (non-MIDI USB controllers)
- Virtual MIDI output for DAW integration

**G5: Build Community Ecosystem**
- Profile sharing platform (import/export)
- Device template marketplace
- Community-contributed device definitions
- Public API for third-party integrations

### Success Metrics

| Metric | Current (v0.1) | Target (v2.0) | Measurement |
|--------|---------------|---------------|-------------|
| **Performance** |
| Response Latency | <1ms | <1ms | 95th percentile input-to-action |
| Memory Footprint | 5-10MB | <15MB | macOS Activity Monitor |
| CPU Usage (idle) | <1% | <1% | macOS Activity Monitor |
| CPU Usage (active) | <5% | <5% | Under sustained load |
| **Adoption** |
| Active Users | N/A (local) | 1,000 | Monthly active installs |
| GitHub Stars | N/A | 500 | GitHub repository |
| Profile Templates | 1 (Mikro MK3) | 20+ | Community contributions |
| **User Satisfaction** |
| Setup Time | ~30min (tech-savvy) | <10min (all users) | User testing |
| Configuration Ease | 6/10 (TOML editing) | 9/10 (visual UI) | User surveys |
| Documentation Quality | 7/10 | 9/10 | User feedback |
| **Technical** |
| Test Coverage | ~20% | >80% | Automated testing |
| Documentation | Good | Excellent | Comprehensive docs |
| Platform Support | macOS only | macOS + Linux | Multi-platform builds |

---

## Core Features

### Current Features (v0.1.0 - Monolithic)

#### Event Processing Pipeline
**Three-stage architecture:**
1. **MIDI Input** → Raw bytes converted to `MidiEvent` enum
2. **Event Processing** → `MidiEvent` → `ProcessedEvent` (detects patterns)
3. **Mapping & Execution** → `ProcessedEvent` → `Action` → execution

#### Advanced Trigger Types (10+)
- **Note**: Basic note on/off with optional velocity range
- **VelocityRange**: Different actions for soft/medium/hard presses
- **LongPress**: Hold detection (configurable duration, default 2000ms)
- **DoubleTap**: Quick double-tap detection (default 300ms window)
- **NoteChord**: Multiple pads simultaneously (default 100ms window)
- **EncoderTurn**: Rotation with direction detection (CW/CCW)
- **Aftertouch**: Pressure sensitivity
- **PitchBend**: Touch strip control
- **CC**: Control change messages
- **ProgramChange**: Program change handling

#### Action Types (10+)
- **Keystroke**: Keyboard shortcuts with modifiers (⌘⇧⌥⌃)
- **Text**: Type text strings
- **Launch**: Open applications by bundle ID
- **Shell**: Execute shell commands with arguments
- **VolumeControl**: System volume (Up/Down/Mute/Set)
- **ModeChange**: Switch between mapping modes
- **Sequence**: Chain multiple actions with delays
- **Delay**: Timing control between actions
- **MouseClick**: Mouse simulation (left/right/middle)
- **Repeat**: Repeat an action N times
- **Conditional**: Execute based on conditions (app state, time, etc.)

#### LED Feedback System
**10 Lighting Schemes:**
- `off` - All LEDs disabled
- `static` - Mode-based static colors
- `breathing` - Slow breathing effect
- `pulse` - Fast pulse effect
- `rainbow` - Rainbow cycle across pads
- `wave` - Wave pattern animation
- `sparkle` - Random sparkles
- `reactive` - Velocity-based response (green/yellow/red)
- `vumeter` - VU meter style (bottom-up)
- `spiral` - Spiral pattern animation

**Device Support:**
- Full RGB control for Maschine Mikro MK3 (HID via hidapi)
- Basic on/off for standard MIDI devices (Note On/Off messages)
- Fade-out effect (1 second after pad release in reactive mode)

#### Mode System
- Multiple modes with independent mapping sets
- Global mappings work across all modes (emergency exit, volume control)
- Mode switching via encoder rotation or pad combinations
- Visual mode indication via LED colors (Blue=Default, Green=Dev, Purple=Media)

#### Device Profile Support
- Load Native Instruments Controller Editor profiles (.ncmm3 XML)
- Auto-detect active pad page (A-H) from incoming MIDI notes
- Manual pad page selection with `--pad-page` option
- Physical pad layout to MIDI note mapping

#### Configuration System
- TOML-based configuration (human-readable)
- Hot-editable (restart required)
- Advanced timing settings (chord timeout, double-tap, hold threshold)
- Per-mode and global mapping definitions

---

### Target Features (v2.0 - Modular)

#### Architecture Changes

**1. Workspace Structure**
```
midimon/
├── midimon-core/       # Pure Rust engine (UI-free, reusable)
├── midimon-daemon/     # Background service with menu bar
├── midimon-gui/        # Tauri v2 visual configuration
└── config/             # Device templates and profiles
```

**Benefits:**
- Core engine reusable in other projects
- Daemon runs at startup without GUI overhead
- GUI launches on-demand for configuration
- Better testability and modularity

#### New Features

**2. Menu Bar Application**
- **Status Indicator**: Active/inactive/processing state
- **Quick Actions**: Enable/disable, current profile, recent mappings
- **Device Status**: Online/offline indicators for connected devices
- **Tray Menu**:
  - Current Profile: [Name] (click to switch)
  - Pause Mappings (toggle)
  - Reload Config (hot reload)
  - Open Configuration...
  - Preferences...
  - Quit

**3. Visual Configuration UI (Tauri v2)**
- **Device Visualization**: Interactive pad/button/encoder layout
- **Drag-and-Drop Mapping**: Drag actions onto pads
- **MIDI Learn Mode**: Click binding slot → press pad → auto-configure
- **Profile Manager**: Create, edit, duplicate, delete profiles
- **Action Library**: Pre-built actions (macros, apps, scripts)
- **Live Event Console**: Real-time event log with resolved actions
- **Per-App Profiles**: Auto-switch based on frontmost application

**4. Hot Config Reload**
- Watch config.toml for changes (using `notify` crate)
- Reload mappings without restarting daemon
- Preserve current state (active pads, timers)
- Visual notification on successful reload

**5. MIDI Learn Workflow**
```
User Flow:
1. Click "Learn" button next to binding slot
2. Press pad/button/encoder on device
3. System captures MIDI message and auto-fills configuration
4. User assigns action (drag from library or create new)
5. Test immediately without restart
```

**6. Per-App Profile Switching**
- Detect frontmost application (bundle ID)
- Auto-switch to app-specific profile
- Fallback to global profile if no match
- Visual indicator in menu bar
- Profile priority system (app > user > global > default)

**7. Device Template System**
- Pre-configured templates for popular controllers:
  - Native Instruments Maschine Mikro MK3
  - Novation Launchpad Mini/Pro
  - Akai APC Mini/40
  - Korg nanoPAD/nanoKONTROL
  - Arturia BeatStep
- User-contributed templates (GitHub repository)
- Visual device layout for each template
- Import/export device definitions

**8. Virtual MIDI Output**
- Expose MIDIMon as virtual MIDI device
- Send MIDI messages from actions
- Enable DAW integration and MIDI routing
- Bidirectional communication with music software

**9. Profile Sharing**
- Export profile to portable format (JSON/TOML)
- Import profiles from community
- Profile marketplace (GitHub-based)
- Version compatibility checking

**10. Advanced Conditions**
- Time-based (e.g., different actions day/night)
- App-running checks (if Spotify running → control playback)
- Modifier keys (Shift/Ctrl held during pad press)
- Variable system (store state between actions)
- Conditional chains (if-then-else logic)

---

## Technical Requirements

### Performance Requirements

| Requirement | Target | Priority |
|-------------|--------|----------|
| Input Latency | <1ms (95th percentile) | P0 |
| Memory Usage | <15MB (daemon + core) | P1 |
| CPU Usage (idle) | <1% on modern Mac | P0 |
| CPU Usage (active) | <5% sustained load | P1 |
| Startup Time | <500ms to functional | P1 |
| Config Reload | <100ms | P2 |
| LED Update Rate | 10fps minimum | P2 |
| MIDI Input Processing | 1000+ events/sec | P1 |

### Platform Requirements

**Primary Platform: macOS**
- macOS 12 (Monterey) minimum
- macOS 14 (Sonoma) recommended
- Apple Silicon (M1/M2/M3) and Intel support
- Universal binary distribution

**Permissions Required:**
- Accessibility (for keystroke simulation)
- Input Monitoring (for HID device access)
- Microphone (optional, for audio-reactive features)

**System Integration:**
- LaunchAgent for auto-start
- Menu bar application (NSStatusItem)
- Frontmost app detection (NSWorkspace)
- AppleScript execution for system control

**Future Platforms:**
- Linux (Ubuntu 22.04+, Arch, Fedora)
- Windows (10/11) - lower priority

### Security & Privacy

**Data Collection:**
- Zero telemetry by default
- No network connections required
- All data stored locally
- Optional anonymous usage statistics (opt-in)

**Permissions Model:**
- Request only necessary permissions
- Clear explanation for each permission
- Graceful degradation if permissions denied
- User-controlled permission management

**Code Signing:**
- Developer ID signing for macOS distribution
- Notarization for Gatekeeper compliance
- Open-source codebase for community auditing

### Compatibility

**MIDI Standards:**
- MIDI 1.0 specification
- CoreMIDI on macOS
- Standard MIDI note/CC/program change messages
- Virtual MIDI endpoints

**HID Standards:**
- USB HID specification
- Shared device access (macOS specific)
- Custom HID report parsing

**File Formats:**
- TOML for configuration
- JSON for profile export/import
- XML for NI Controller Editor profiles (.ncmm3)
- Markdown for documentation

---

## Migration Strategy (v0.1 → v2.0)

### Phase 1: Preserve & Document (Week 1-2)
**Goal**: Ensure no knowledge loss of current implementation

**Tasks:**
- [x] Create CLAUDE.md with comprehensive architecture documentation
- [x] Update copilot-instructions.md with migration plan
- [ ] Tag current implementation as `v0.1.0-monolithic`
- [ ] Document all trigger types and action types
- [ ] Create user guide for TOML configuration
- [ ] Record demo video of current functionality
- [ ] Run comprehensive testing suite
- [ ] Document known issues and limitations

**Deliverables:**
- Git tag: v0.1.0-monolithic
- Documentation: CLAUDE.md, user guide, API docs
- Test results: All features verified working
- Video: Demo of current capabilities

### Phase 2: Extract Core Engine (Week 3-6)
**Goal**: Create midimon-core as reusable library

**Tasks:**
- [ ] Create workspace Cargo.toml
- [ ] Create midimon-core crate
- [ ] Move device I/O code to core/devices.rs
- [ ] Move event processing to core/events.rs
- [ ] Move mapping engine to core/mapping.rs
- [ ] Move actions to core/actions.rs
- [ ] Move config to core/config.rs
- [ ] Add comprehensive unit tests (>80% coverage)
- [ ] Document public API with rustdoc
- [ ] Verify zero UI dependencies in core

**Success Criteria:**
- Core engine compiles independently
- All existing features work via core API
- Test coverage >80%
- Documentation complete
- Zero breaking changes to config.toml format

### Phase 3: Build Daemon & UI (Week 7-12)
**Goal**: Add menu bar daemon and Tauri GUI

**Tasks:**
- [ ] Create midimon-daemon crate
- [ ] Implement menu bar integration (NSStatusItem)
- [ ] Add LaunchAgent installer
- [ ] Create midimon-gui crate with Tauri v2
- [ ] Build device visualization UI
- [ ] Implement MIDI Learn mode
- [ ] Add profile manager
- [ ] Build action library UI
- [ ] Implement hot config reload
- [ ] Add frontmost app detection
- [ ] Create preferences UI

**Success Criteria:**
- Daemon starts on login
- Menu bar shows status and quick actions
- GUI provides visual configuration
- MIDI Learn works end-to-end
- Config hot-reloads without restart
- Performance targets maintained

### Phase 4: Enhanced Features (Week 13-20)
**Goal**: Add v2.0 exclusive features

**Tasks:**
- [ ] Device template system
- [ ] Profile import/export
- [ ] Virtual MIDI output
- [ ] Advanced conditional logic
- [ ] Live event console
- [ ] Profile marketplace integration
- [ ] Community device templates
- [ ] Linux support (initial)
- [ ] Performance profiling and optimization
- [ ] Beta testing program

**Success Criteria:**
- 10+ device templates available
- Profile sharing functional
- Virtual MIDI tested with DAWs
- Community contributing templates
- Beta users providing feedback

---

## Risk Analysis

### Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Breaking changes during migration | High | Medium | Comprehensive testing, feature parity checks |
| Performance degradation | High | Low | Continuous benchmarking, profiling |
| Tauri learning curve | Medium | Medium | Prototype early, community support |
| HID device compatibility | Medium | Medium | Broad device testing, fallback modes |
| macOS API changes | Medium | Low | Test on multiple macOS versions |
| Community adoption | High | Medium | Clear documentation, easy onboarding |

### Product Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| User confusion during migration | Medium | High | Clear migration guide, side-by-side support |
| Loss of current users | Medium | Medium | Maintain v0.1 branch, easy rollback |
| UI complexity overwhelms users | High | Medium | Simple defaults, progressive disclosure |
| Insufficient device support | High | Medium | Template system, community contributions |
| Competitive alternatives emerge | Medium | Low | Open source advantage, community building |

---

## Open Questions

1. **Distribution Model**: Mac App Store vs. GitHub releases vs. Homebrew?
2. **Licensing**: MIT vs. Apache 2.0 vs. GPL for community contributions?
3. **Monetization**: Donations, paid support, enterprise features, marketplace fees?
4. **Profile Format**: Keep TOML or switch to JSON for better UI integration?
5. **Device Discovery**: Auto-detect and suggest templates vs. manual selection?
6. **Update Mechanism**: Auto-update (Sparkle/Tauri updater) vs. manual downloads?
7. **Multi-device**: Support multiple controllers simultaneously? Priority?
8. **Windows Support**: Timeline and resource allocation for Windows port?
9. **Mobile Companion**: iOS/Android app for remote control? Future consideration?
10. **Cloud Sync**: Sync profiles across multiple Macs? Privacy implications?

---

## Timeline & Milestones

### 2025 Roadmap

**Q1 2025 (Jan-Mar): Foundation**
- ✅ Document current implementation (CLAUDE.md)
- ✅ Create PRD and personas (this document)
- [ ] Tag v0.1.0-monolithic
- [ ] Complete Phase 1 (Preserve & Document)

**Q2 2025 (Apr-Jun): Core Migration**
- [ ] Complete Phase 2 (Extract Core Engine)
- [ ] Alpha release: midimon-core library
- [ ] Begin Phase 3 (Daemon development)

**Q3 2025 (Jul-Sep): UI Development**
- [ ] Complete Phase 3 (Daemon & UI)
- [ ] Beta release: v2.0-beta1 with visual configuration
- [ ] Community testing program

**Q4 2025 (Oct-Dec): Enhancement & Launch**
- [ ] Complete Phase 4 (Enhanced Features)
- [ ] v2.0 Release Candidate
- [ ] Launch v2.0 with device templates and profile sharing
- [ ] Marketing push and community building

---

## Appendices

### A. Related Documents
- **User Personas**: See `docs/personas.md`
- **Feature Specifications**: See `docs/features.md`
- **Technical Architecture**: See `docs/architecture.md`
- **Current Implementation**: See `CLAUDE.md`
- **Research Proposals**: See `.research/implementation-viewpoint-1.md` and `viewpoint-2.md`

### B. References
- **Current Codebase**: `/src/*.rs` - Working implementation
- **Configuration**: `config.toml` - Example mappings
- **LED Documentation**: `LED_FEEDBACK.md` - Feedback system details
- **Diagnostic Tools**: `/src/bin/*.rs` - Testing utilities

### C. Glossary
- **DAW**: Digital Audio Workstation (e.g., Logic Pro, Ableton Live)
- **HID**: Human Interface Device (USB protocol)
- **MIDI**: Musical Instrument Digital Interface
- **CC**: Control Change (MIDI message type)
- **Pad**: Physical button on MIDI controller
- **Profile**: Set of mappings for specific use case
- **Mode**: Sub-profile within a profile (e.g., Default, Development, Media)
- **Trigger**: Event that activates a mapping (e.g., pad press, long press)
- **Action**: Response to a trigger (e.g., keystroke, shell command)
- **Template**: Device-specific configuration (pad layout, MIDI mappings)

---

**Document History:**
- v1.0 (2025-11-11): Initial PRD creation based on current implementation
- Future: Version history will be maintained here
