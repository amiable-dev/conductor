# Setup Gallery

Visual showcase of real Conductor configurations. Get inspired and download ready-to-use configs!

---

## Hybrid MIDI + Gamepad Setups

### Music Producer's Dream Desk

**Hardware**: Maschine Mikro MK3 (MIDI) + Xbox Series X Controller (Gamepad)
**Software**: Ableton Live 11 + Splice + iZotope RX
**Cost**: $0 additional (repurposed existing gear)

**Setup Description**:
- **MIDI pads (Maschine)**: Velocity-sensitive recording, sample triggering
- **Xbox controller**: DAW navigation, transport controls, mixer view switching
- **Hybrid workflow**: Hands on MIDI for music, thumbs on gamepad for navigation

**Key Features**:
- Velocity Range triggers (soft/medium/hard on same pad)
- Per-app profiles (Ableton vs Splice different mappings)
- RGB LED feedback on Maschine (mode indicators)

**Why This Works**:
> "I keep my hands on the MIDI pads for recording, but use my thumbs on the Xbox controller for transport and navigation. No more reaching for the keyboard mid-performance."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/hybrid/producer-desk)**

---

### Streaming Command Center

**Hardware**: Launchpad Mini (MIDI) + PlayStation DualSense (Gamepad)
**Software**: OBS Studio + Discord + Spotify
**Replaces**: $300 Stream Deck

**Setup Description**:
- **Launchpad grid**: Scene switching, source visibility, filter toggles (with LED feedback)
- **DualSense**: Audio mixing, chat mute, music controls, emergency stop
- **Hybrid power**: Visual grid + ergonomic controller

**Key Features**:
- LED feedback shows active scenes (green = live, red = muted, blue = preview)
- Velocity-sensitive audio fading on DualSense triggers
- Button chords (L1+R1 = mute all, L2+R2 = emergency offline)

**Why This Works**:
> "I have visual confirmation from the Launchpad LEDs for scene switching, and the DualSense triggers let me fade audio smoothly. Best of both worlds."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/hybrid/streaming-center)**

---

## Gamepad-Only Setups

### Budget Stream Deck Alternative (Xbox Controller)

**Hardware**: Xbox One Controller
**Software**: OBS Studio + Discord + Voicemeeter
**Cost**: $0 (reused controller) vs $150-300 Stream Deck
**Savings**: $150-300

**Button Mapping**:
```
Face Buttons:
├─ A: Start/Stop Recording
├─ B: Mute Microphone
├─ X: Scene 1 (Gameplay)
└─ Y: Scene 2 (Webcam)

D-Pad:
├─ Up: Increase Volume
├─ Down: Decrease Volume
├─ Left: Previous Track (Spotify)
└─ Right: Next Track (Spotify)

Shoulders:
├─ LB: Switch to Discord
├─ RB: Switch to OBS
├─ LT (hold): Enable Push-to-Talk
└─ RT (hold): Instant Replay (save last 30s)

Menu Buttons:
├─ Start: Emergency "Be Right Back" scene
└─ Select: Screenshot
```

**Why This Works**:
> "I was about to buy a Stream Deck. Then I realized my Xbox controller has 15+ buttons doing nothing. Configured Conductor in 20 minutes, saved $300."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/streaming/xbox-obs-basic)**

---

### Developer Workflow (PlayStation DualSense)

**Hardware**: PlayStation DualSense Controller
**Software**: VS Code + Terminal + Docker Desktop + GitHub Desktop
**Focus**: Git workflows, build automation, window management

**Button Mapping**:
```
Face Buttons:
├─ Cross: Git Status
├─ Circle: Git Add + Commit
├─ Square: Build Project
└─ Triangle: Run Tests

D-Pad:
├─ Up: Next Workspace
├─ Down: Previous Workspace
├─ Left: Previous Tab
└─ Right: Next Tab

Triggers (Velocity-Sensitive):
├─ L2 (soft): Git Pull
├─ L2 (hard): Git Pull --rebase
├─ R2 (soft): Git Push
└─ R2 (hard): Git Push --force-with-lease

Shoulders:
├─ L1: Launch Terminal
├─ R1: Launch VS Code
├─ L1+R1 (chord): Docker Compose Up
└─ Touchpad Click: Launch GitHub Desktop
```

**Why This Works**:
> "Velocity-sensitive git operations are incredible. Soft press = regular pull/push, hard press = force operations. One button, two functions."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/development/dualsense-vscode)**

---

## Specialized Controllers

### Racing Wheel for Video Editing (Logitech G29)

**Hardware**: Logitech G29 Racing Wheel + Pedals + Shifter
**Software**: DaVinci Resolve / Final Cut Pro
**Unique Factor**: Analog pedal control for timeline/zoom

**Control Mapping**:
```
Pedals (Analog):
├─ Gas: Timeline Playback Speed (0-100%)
├─ Brake: Zoom Level (fine control)
└─ Clutch: Master Volume

Wheel Buttons:
├─ Button 1-4: Mark In/Out, Add Marker, Set Clip
├─ Button 5-8: Cut, Ripple Delete, Slip, Slide
├─ D-Pad: Nudge Frame Left/Right, Track Up/Down
└─ Paddle Shifters: Previous/Next Edit Point

Wheel Rotation:
└─ Scrub Timeline (analog precision)
```

**Why This Works**:
> "Analog pedal control for timeline speed is game-changing. I can smoothly ramp from 10% to 200% playback, which is impossible with keyboard shortcuts. Plus, it's ergonomic—my feet are doing work my hands don't have to."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/video-editing/g29-davinci)**

---

### HOTAS for Productivity (Thrustmaster T.16000M)

**Hardware**: Thrustmaster T.16000M FCS HOTAS
**Software**: macOS productivity apps (Terminal, VS Code, Docker, Postman)
**Repurpose Value**: $150 gaming hardware → $300 productivity tool

**Control Mapping**:
```
Joystick:
├─ Trigger (half-press): Build Project
├─ Trigger (full-press): Build + Run
├─ Thumb Button: Copy
├─ Top Buttons: Paste, Undo, Redo
├─ Hat Switch: Mission Control, Desktop 1-4
└─ Pinkie Switch: Toggle Fullscreen

Throttle:
├─ Base Button 1-6: Launch Apps (Terminal, VS Code, Browser, etc.)
├─ Throttle Hat: Window Snapping (Left/Right/Maximize)
├─ Slider: System Volume
└─ Rocker Switch: Previous/Next Tab
```

**Why This Works**:
> "Dual-stage trigger for build operations is brilliant. Half-pull compiles, full-pull runs. Plus, having 20+ buttons within thumb reach is incredibly efficient."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/productivity/hotas-macos)**

---

## MIDI-Only Setups

### Logic Pro Command Center (Launchpad Mini)

**Hardware**: Novation Launchpad Mini
**Software**: Logic Pro X
**Focus**: 4-mode workflow (Record, Mix, Edit, Perform)

**Mode System**:
```
Mode 1 (Blue): Recording
├─ Row 1: Transport (Play, Stop, Record, Loop)
├─ Row 2: Metronome, Click, Count-in, Pre-roll
├─ Row 3-8: Track Record Arm (8 tracks)

Mode 2 (Green): Mixing
├─ Columns: 8-channel fader control (velocity = level)
├─ Row 1: Mute 8 tracks
├─ Row 2: Solo 8 tracks

Mode 3 (Purple): Editing
├─ Row 1: Cut, Copy, Paste, Delete, Undo, Redo
├─ Row 2: Split, Join, Merge, Bounce
├─ Row 3-8: Marker creation, loop region

Mode 4 (Red): Live Performance
├─ Grid: Trigger scenes, loops, one-shots
├─ LED feedback: Green = playing, Red = stopped
```

**Why This Works**:
> "RGB LED feedback is essential. I see what mode I'm in and what each pad does at a glance. Blue = recording mode, green = mixing, etc."

**[Download Config →](https://github.com/amiable/conductor-configs/tree/main/music/launchpad-logic-pro)**

---

## Share Your Setup

Have an interesting Conductor configuration? Share it with the community!

**[Submit Your Setup →](https://github.com/amiable/conductor/discussions/new?category=show-and-tell)**

Include:
- Photos of your hardware setup
- Description of your workflow
- Config file (TOML)
- Why it works for you

**Featured setups receive**:
- Spot on this page
- Social media shoutout
- Entry in official config repository

---

## More Resources

- **[Success Stories](success-stories.md)** - User testimonials and results
- **[Device Templates](../guides/device-templates.md)** - Pre-built configs for popular controllers
- **[Configuration Examples](../configuration/examples.md)** - Copy-paste ready configs
- **[Community Discussions](https://github.com/amiable/conductor/discussions)** - Ask questions, share ideas
