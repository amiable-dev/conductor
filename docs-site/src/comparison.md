# Conductor vs Alternatives

Choosing the right automation tool? Here's how Conductor compares to popular alternatives.

## Feature Comparison

| Feature | Conductor | Stream Deck | Keyboard Maestro | Karabiner |
|---------|---------|-------------|------------------|-----------|
| **Price** | Free (MIT) | $150-300 | $36 | Free |
| **MIDI Support** | ✅ Full | ❌ None | ⚠️ Limited | ❌ None |
| **Gamepad Support** | ✅ Full (v3.0) | ❌ None | ❌ None | ❌ None |
| **Velocity Sensitivity** | ✅ Yes | ❌ No | ❌ No | ❌ No |
| **Hot-Reload** | ✅ <10ms | ❌ No | ⚠️ Slow | ✅ Fast |
| **RGB LED Feedback** | ✅ Yes | ✅ Yes | ❌ No | ❌ No |
| **Per-App Profiles** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| **Response Latency** | <1ms | ~5-10ms | ~2-5ms | <1ms |
| **Platform** | macOS (Linux/Win planned) | macOS/Windows | macOS only | macOS only |
| **Customization** | Unlimited (TOML + GUI) | GUI only | GUI + AppleScript | Complex JSON |
| **Hardware Cost** | $0 (reuse existing) | $150+ (proprietary) | Any keyboard | Any keyboard |

## Winner By Use Case

### 🎹 Music Production → **Conductor**
**Why**: Only tool with full MIDI + velocity sensitivity + gamepad support. Turn existing hardware into professional control surfaces.

**Example**: Use Maschine pads for velocity-sensitive recording while Xbox controller handles DAW navigation.

---

### 🎮 Gaming Hardware → **Conductor**
**Why**: Only automation tool supporting gamepads, racing wheels, flight sticks, and HOTAS controllers.

**Example**: Repurpose your $30 Xbox controller as a 15-button macro pad instead of buying a $150 Stream Deck.

---

### 💰 Budget-Conscious → **Conductor**
**Why**: Free + works with hardware you already own = $0-$300 savings.

**Comparison**:
- **Conductor**: Free (use existing Xbox/MIDI controller)
- **Stream Deck**: $150-300 + proprietary hardware
- **Keyboard Maestro**: $36/license + keyboard only

---

### 🖱️ Plug-and-Play Simplicity → **Stream Deck**
**Why**: Zero configuration, visual GUI, works out of the box.

**Trade-off**: Limited to Stream Deck hardware, no MIDI/gamepad, no velocity sensitivity, expensive.

---

### ⚙️ Maximum Flexibility → **Conductor**
**Why**: Hybrid MIDI+gamepad, open-source, unlimited customization, hot-reload.

**Example**: Combine MIDI pads (recording) + Xbox controller (navigation) + racing wheel pedals (effects) in one workflow. No other tool can do this.

---

### 💻 Keyboard-Only Automation → **Keyboard Maestro** or **Karabiner**
**Why**: Mature, keyboard-focused automation with macOS integration.

**Trade-off**: No MIDI, no gamepads, no velocity sensitivity, slower reload.

---

## Quick Decision Guide

**Choose Conductor if you**:
- ✅ Own a MIDI controller or gamepad
- ✅ Want velocity-sensitive actions (one button = multiple functions)
- ✅ Need hybrid MIDI+gamepad workflows
- ✅ Want to reuse existing hardware (save $150+)
- ✅ Value open-source and customization

**Choose Stream Deck if you**:
- ✅ Want zero-configuration plug-and-play
- ✅ Don't own MIDI/gamepad hardware
- ✅ Prefer visual button displays
- ✅ Budget isn't a concern ($150-300)

**Choose Keyboard Maestro if you**:
- ✅ Only need keyboard-based macros
- ✅ Want mature macOS integration
- ✅ Don't need MIDI/gamepad support

**Choose Karabiner if you**:
- ✅ Need deep keyboard remapping
- ✅ Want free solution (keyboard only)
- ✅ Don't mind complex JSON configuration

## Unique Conductor Advantages

### 1. Velocity Sensitivity (Unmatched)
**No other tool offers this**: Soft/medium/hard press = different actions on the same button.

**Example**: Press pad softly = copy, press hard = paste. One pad, two functions.

### 2. Hybrid Multi-Protocol (v3.0)
**No other tool supports**: MIDI + gamepads simultaneously in one workflow.

**Example**: MIDI pads for recording + Xbox controller for navigation + racing wheel for effects.

### 3. Repurpose Existing Hardware
**Save $150-300**: Use Xbox controller, PlayStation DualSense, MIDI keyboard you already own instead of buying Stream Deck.

### 4. Open Source & Free
**MIT licensed**: No subscriptions, no vendor lock-in, community-driven development.

### 5. Blazing Fast (<1ms latency)
**Lowest latency**: Hot-reload in <10ms, event processing <1ms, daemon architecture.

---

## Migration Guides

### From Stream Deck
1. Export your Stream Deck button layouts
2. Map buttons to gamepad/MIDI IDs in Conductor
3. Import provided template configs
4. Save $150+ by reusing existing hardware

[See Stream Deck Migration Guide →](migration/stream-deck.md)

### From Keyboard Maestro
1. Export keyboard shortcuts list
2. Map shortcuts to MIDI/gamepad triggers
3. Add velocity sensitivity for multi-function buttons
4. Gain MIDI/gamepad support KM doesn't have

[See Keyboard Maestro Migration Guide →](migration/keyboard-maestro.md)

---

## Frequently Compared

### "Can I use Conductor alongside Stream Deck?"
Yes! Conductor and Stream Deck can coexist. Use Stream Deck for visual buttons, Conductor for MIDI/gamepad + velocity sensitivity.

### "Is Conductor harder to configure than Stream Deck?"
**Initial setup**: Conductor requires TOML config or GUI (5-10 min), Stream Deck is plug-and-play (1 min).

**Long-term**: Conductor's hot-reload (<10ms) is faster than Stream Deck's UI reconfiguration. Visual MIDI Learn mode makes config easy.

### "Why not just use Keyboard Maestro?"
Keyboard Maestro is excellent for keyboard-only macros, but lacks:
- MIDI support
- Gamepad support
- Velocity sensitivity
- Fast hot-reload
- Hybrid multi-protocol workflows

Conductor is the only tool supporting MIDI+gamepad+velocity in one workflow.

---

## Try Conductor Free

No commitment, no credit card, no proprietary hardware required.

**[Get Started →](getting-started/quick-start.md)** | **[Download Templates →](guides/device-templates.md)** | **[Join Community →](resources/community.md)**
