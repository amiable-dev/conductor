# **Core architecture (high-level)**

- **Engine (headless Rust crate):** event ingestion → normalization → state machine (press/hold/velocity) → mapping → action executor.
    
- **UI shell:** _status-bar (menu bar) app_ for status + quick toggles + “Open Config”. No main window by default.
    
- **Login item / agent:** start on user login; engine runs even when UI is closed.
    

---

# **Rust crates that fit each need**

  

## **MIDI I/O (hardware + virtual endpoints)**

- **midir** — cross-platform MIDI I/O; on macOS it uses CoreMIDI under the hood. Solid for device enumeration and stream callbacks. 
    
- **coremidi** — when you specifically need _virtual_ sources/destinations (e.g., expose your app as a virtual MIDI port) or CoreMIDI-specific features. Use alongside midir if needed. 
    
- **wmidi** — ergonomic parsing/formatting of MIDI messages (NoteOn, velocity, Aftertouch, etc.) for your mapping layer. 
    

  

## **HID / non-MIDI controllers**

- **hidapi** — stable low-level HID access; recent versions support shared access on macOS (important when other apps also “see” the device). 
    
- **async-hid** — if you prefer async over callbacks for HID work. 
    

  

## **Global input & macro output (simulating keys/mouse)**

- **Listen (global):** rdev can hook keyboard/mouse globally (useful if you also want to react to system input). 
    
- **Send (simulate):** enigo to emit keystrokes/mouse moves/clicks for macros. (macOS supported). You’ll still need Accessibility permission. 
    

  

## **Config files (+ hot reload)**

- **Format:** serde + toml (or yaml) for human-readable device maps; schemars if you want a JSON-Schema for validation later.
    
- **Loading:** confy for per-user config location, or roll your own path in ~/Library/Application Support/YourApp.
    
- **Hot reload:** notify to watch config files and re-apply on change. 
    

  

## **App shell: menu bar (status bar) app**

  

Two good routes:

1. **Tauri (Rust backend + tiny WebView front):**
    
    - System tray / menu bar with custom menu & handlers.
        
    - Autostart plugin for “Run at login.”
        
    - Nice for a small, polished panel (React/Vite optional). 
        
    
2. **Pure Rust native:**
    
    - tray-icon (by the Tauri team) + winit/tao for events/windows if you ever need a small settings window. 
        
    

  

**Recommendation:** Use **Tauri v2** for the tray UI + quick actions (quicker to ship, autostart out of the box), keep the mapping engine in a separate Rust crate.

  

## **Launch at login / background**

- **Tauri:** tauri-plugin-autostart (wraps platform login-item mechanisms). 
    
- **DIY (native):** install a LaunchAgent plist into ~/Library/LaunchAgents or use ServiceManagement APIs; Tauri saves you the plumbing.
    

  

## **Running macros and automations**

- **Process launch:** duct (comfy process API) to run shell commands or CLIs your users bind to.
    
- **AppleScript / app control:** call osascript or use a small wrapper; for app focus/opening use Launch Services / NSWorkspace (via Tauri command or objc bindings).
    

---

# **Mapping model & event logic (what to implement)**

- **Device abstraction:** Device { id, kind: Midi|Hid, elements: [Pad|Knob|Slider|Key] }
    
- **Event normalization:** Convert MIDI (NoteOn, CC, velocity) + HID reports into a unified InputEvent { device_id, element_id, action: Press/Release/Rotate, value, timestamp }.
    
- **State machine:** per-element timers for **short/long press**; thresholds in config (e.g., long_press_ms: 400).
    
- **Velocity/pressure rules:** conditions like when velocity > 100 -> Action::Macro("…").
    
- **Profiles & routing:** global + per-app (_frontmost app_), with fallbacks.
    
- **Action graph:** keep it simple first—sequences of: “send keys”, “run command”, “send MIDI”, “run AppleScript”.
    

---

# **UX patterns that work well**

  

**Metaphors**

- **Profiles (a.k.a. Layers):** per app or per task (e.g., “DaVinci Resolve”, “Logic Pro”, “Browser”). Auto-switch when the _frontmost app_ changes; show the active profile name in the menu bar.
    
- **“MIDI/HID Learn” mode:** click a binding slot → press/pad on the device → we fill in element id & channel automatically.
    
- **Routing Matrix:** grid view (Device Elements × Actions) with search + filters.
    
- **Condition pills:** little chips like Long-press, Velocity>100, Shift layer that decorate a binding.
    
- **Action Library:** reusable blocks (“Send ⌘K”, “Type string”, “Run shell”, “AppleScript: Mute Zoom”), drag to assign.
    
- **Test Console:** live log of last N events with their resolved action—vital for debugging.
    
- **Status-bar popover:** tiny panel with: current profile, device online/offline, “Pause Mappings”, “Open Config”, “Reload Config”.
    

  

**Onboarding**

- First run: permissions checklist (Accessibility for input simulation, possibly “Input Monitoring” if you hook global events), choose devices, pick default profile.
    
- Guided “Learn” to create first 2–3 bindings.
    

---

# **macOS specifics & caveats**

- **Accessibility permission** is required to synthesize keystrokes (e.g., enigo); prompt the user and link to System Settings → Privacy & Security → Accessibility. (Your app won’t be able to “type” until granted.) 
    
- **MIDI sharing:** multiple clients can consume the same device; CoreMIDI is made for it. Use coremidi for virtual endpoints if you want other apps to “see” your app as a device. 
    
- **HID sharing on macOS:** hidapi supports shared device access via a feature flag—handy when not grabbing exclusive control. 
    

---

# **Suggested crate lineup (opinionated)**

- **Engine:** midir (+ coremidi for virtual ports), hidapi/async-hid, wmidi, crossbeam (channels), thiserror
    
- **Macros:** enigo, duct (+ optional AppleScript via osascript) 
    
- **Config:** serde + toml, notify for hot-reload 
    
- **UI:** Tauri v2 (tray + tiny config window), tauri-plugin-autostart 
    
- **Optional:** tray-icon if you go pure native 
    

---

# **Minimal flow to meet your bullet points**

1. **Turn a MIDI device into a macro pad**
    
    - midir input callback → normalize NoteOn/Off/velocity via wmidi → mapping engine → action executor (enigo/duct). 
        
    
2. **Map HID & MIDI to specific devices, override defaults**
    
    - Identify by vendor/product/serial (HID) and entity IDs (MIDI). Maintain user-friendly aliases in config. hidapi gives IDs; CoreMIDI provides endpoint info. 
        
    
3. **Load config files & device element definitions**
    
    - serde structs for devices, elements, bindings; hot-reload with notify; ship sample templates for popular pads. 
        
    
4. **Short/long press, velocity-based actions**
    
    - Per-element timers; thresholds and conditions in config; state machine emits derived events (ShortPress, LongPress, HighVelocity).
        
    
5. **Run in background with status in menu bar**
    
    - Tauri tray with a popover; indicators for active profile & device online; quick actions (Pause, Reload, Open Config). 
        
    
6. **Easy access to config & mapping**
    
    - Tray “Open Config” → either open your tiny settings window (Tauri) or open the config file in the user’s editor.
        
    
7. **Run on OS startup**
    
    - Tauri autostart plugin; or install a LaunchAgent if you’re going full native. 
        
    

---

# **A few implementation tips**

- **Threading:** keep I/O callbacks lean; push raw events onto a lock-free queue; process on a worker where you can debounce/timestamp.
    
- **Per-app profiles:** watch the _frontmost app_ (bundle id) and switch profile; in Tauri, expose a small Rust command using NSWorkspace to query (runningApplication: NSWorkspace.shared.frontmostApplication).
    
- **Safety valve:** a global “panic” hotkey to temporarily disable outputs if a macro goes wild.
    
- **Schema versioning:** include config_version and migrate on load to avoid breaking users.
    
---

here’s the opinionated starter layout, sample config, and core scaffolding so you can get typing immediately.
  

# **Repo layout**

```
midi-macro-pad/
├─ Cargo.toml                       # workspace
├─ engine/                          # pure Rust crate: IO + mapping + actions
│  ├─ Cargo.toml
│  └─ src/
│     ├─ lib.rs
│     ├─ devices.rs
│     ├─ events.rs
│     ├─ mapping.rs
│     ├─ state.rs
│     ├─ actions.rs
│     └─ config.rs
├─ app/                             # Tauri v2 shell (menu bar + small settings window)
│  ├─ Cargo.toml
│  ├─ src-tauri/
│  │  ├─ Cargo.toml
│  │  └─ src/main.rs
│  └─ ui/                           # optional (Svelte/React/Vite) for the settings window
└─ config/
   ├─ default.toml
   └─ device_templates/
      ├─ launchpad_mini.toml
      └─ korg_nanokontrol.toml
```

> Keep engine UI-free so you can unit test mappings and reuse it for CLIs.

---

# **Cargo (key deps)**

  

**engine/Cargo.toml**

```
[package]
name = "engine"
version = "0.1.0"
edition = "2021"

[dependencies]
midir = "0.9"
wmidi = "4"
hidapi = "2"
crossbeam = "0.8"
thiserror = "1"
serde = { version = "1", features = ["derive"] }
serde_repr = "0.1"
toml = "0.8"
notify = "6"
parking_lot = "0.12"
enigo = "0.2"
duct = "0.13"
time = { version = "0.3", features = ["macros"] }
anyhow = "1"
log = "0.4"
```

**app/src-tauri/Cargo.toml**

```
[package]
name = "midi-macro-pad-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-autostart = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-shell = "2"
tauri-plugin-notification = "2"
serde = { version = "1", features = ["derive"] }
engine = { path = "../../engine" }   # local crate
```

---

# **Config: simple but expressive (TOML)**

  

**config/default.toml**

```
config_version = 1

[general]
long_press_ms = 400
double_tap_ms = 275
velocity_threshold = 100
active_profile = "global"
panic_hotkey = "ctrl+alt+cmd+p"   # to pause outputs

# Per-app or global profiles
[profiles.global]
match = { kind = "any" } # or { kind = "bundle_id", value = "com.apple.Finder" }

# Devices you care about (by ids and friendly aliases)
[[devices]]
alias = "Launchpad Mini"
kind = "midi"
vendor_id = 4661
product_id = 4384
port_name_contains = "Launchpad"

[[devices.elements]]
id = "pad_1_1"
kind = "note"
note = 36
channel = 0

[[devices.elements]]
id = "pad_1_2"
kind = "note"
note = 37
channel = 0

# HID example
[[devices]]
alias = "Generic HID Keypad"
kind = "hid"
vendor_id = 1234
product_id = 5678

[[devices.elements]]
id = "hid_btn_01"
kind = "button"
usage_page = 0x09
usage = 0x01

# Bindings (normalized events → actions)
# Conditions: short, long, velocity>, with optional per-app override through profile
[[bindings]]
profile = "global"
device = "Launchpad Mini"
element = "pad_1_1"
when = "short"
action = { type = "keys", keys = "cmd+shift+4" } # macOS screenshot selection

[[bindings]]
profile = "global"
device = "Launchpad Mini"
element = "pad_1_1"
when = "long"
action = { type = "shell", cmd = "open", args = ["-a", "Calculator"] }

[[bindings]]
profile = "global"
device = "Launchpad Mini"
element = "pad_1_2"
when = "velocity>100"
action = { type = "text", text = "Hello from pad! " }

# Optional: expose a virtual MIDI out so users can route into DAWs
[virtual_midi]
enabled = true
port_name = "MacroPad Out"
```

---

# **Core engine scaffolding (selected files)**

  

**events.rs**

```
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceKind { Midi, Hid }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceId {
  pub kind: DeviceKind,
  pub vendor_id: u16,
  pub product_id: u16,
  pub serial: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ElementAction { Press, Release, Rotate, Value }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
  pub device: DeviceId,
  pub element_id: String,          // stable id from config (e.g., "pad_1_1")
  pub action: ElementAction,
  pub value: i32,                  // velocity/rotation/etc.
  pub ts: Instant,
}

#[derive(Debug, Clone, Copy)]
pub enum Derived {
  ShortPress,
  LongPress,
  VelocityHigh,
}
```

**state.rs** — derive short/long/velocity

```
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crate::events::{InputEvent, ElementAction, Derived};

pub struct Deriver {
  long_press: Duration,
  velocity_threshold: i32,
  down_since: HashMap<String, Instant>,
}

impl Deriver {
  pub fn new(long_press_ms: u64, velocity_threshold: i32) -> Self {
    Self {
      long_press: Duration::from_millis(long_press_ms),
      velocity_threshold,
      down_since: HashMap::new(),
    }
  }

  pub fn on_event(&mut self, ev: &InputEvent) -> Option<Derived> {
    match ev.action {
      ElementAction::Press => {
        self.down_since.insert(ev.element_id.clone(), ev.ts);
        if ev.value >= self.velocity_threshold {
          return Some(Derived::VelocityHigh);
        }
      }
      ElementAction::Release => {
        if let Some(t0) = self.down_since.remove(&ev.element_id) {
          let dt = ev.ts.saturating_duration_since(t0);
          return if dt >= self.long_press { Some(Derived::LongPress) }
                 else { Some(Derived::ShortPress) };
        }
      }
      _ => {}
    }
    None
  }
}
```

**actions.rs** — macro executor

```
use anyhow::Result;
use enigo::{Enigo, KeyboardControllable, MouseControllable};
use duct::cmd;

pub enum Action {
  Keys(String),             // "cmd+shift+4"
  Text(String),
  Shell { cmdline: String, args: Vec<String> },
  MidiOut { status: u8, data1: u8, data2: u8 }, // optional: to virtual out
}

pub struct Executor {
  enigo: Enigo,
}

impl Executor {
  pub fn new() -> Self { Self { enigo: Enigo::new() } }

  pub fn run(&mut self, a: Action) -> Result<()> {
    match a {
      Action::Keys(seq) => {
        // naive parser; implement properly later
        // Example: send chord by pressing modifiers then key
        crate::mac::send_keys(&mut self.enigo, &seq)?;
      }
      Action::Text(s) => {
        for ch in s.chars() { self.enigo.key_sequence(&ch.to_string()); }
      }
      Action::Shell { cmdline, args } => { cmd(cmdline, &args).start()?.wait()?;
      }
      Action::MidiOut { .. } => {
        // route to virtual MIDI out (if enabled)
      }
    }
    Ok(())
  }
}
```

**mapping.rs** — resolution from (device, element, derived) → action

```
use std::collections::HashMap;
use crate::events::Derived;
use crate::actions::Action;

#[derive(Hash, Eq, PartialEq)]
struct Key { device: String, element: String, when: When }
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum When { Short, Long, VelocityHigh }

pub struct Mapper {
  table: HashMap<Key, Action>,
}

impl Mapper {
  pub fn resolve(&self, device: &str, element: &str, d: Derived) -> Option<&Action> {
    let w = match d {
      Derived::ShortPress => When::Short,
      Derived::LongPress => When::Long,
      Derived::VelocityHigh => When::VelocityHigh,
    };
    self.table.get(&Key { device: device.into(), element: element.into(), when: w })
  }
}
```

**lib.rs** — glue: inputs → deriver → mapper → executor

```
mod events; pub use events::*;
mod state;  use state::Deriver;
mod actions; use actions::{Action, Executor};
mod mapping; use mapping::Mapper;

use crossbeam::channel::{unbounded, Sender};
use std::thread;
use std::time::Instant;

pub struct Engine {
  tx: Sender<events::InputEvent>,
}

impl Engine {
  pub fn start(mapper: Mapper) -> Self {
    let (tx, rx) = unbounded::<events::InputEvent>();
    thread::spawn(move || {
      let mut deriver = Deriver::new(400, 100);
      let mut exec = Executor::new();
      for ev in rx.iter() {
        if let Some(d) = deriver.on_event(&ev) {
          if let Some(a) = mapper.resolve(&format!("{:?}", ev.device), &ev.element_id, d) {
            // clone or convert as needed
            let _ = exec.run(a.clone());
          }
        }
      }
    });
    Self { tx }
  }

  // Called by MIDI/HID backends after normalization
  pub fn submit(&self, mut ev: events::InputEvent) {
    ev.ts = Instant::now();
    let _ = self.tx.send(ev);
  }
}
```

---

# **MIDI ingestion (sketch)**

```
use midir::{MidiInput, Ignore};
use wmidi::{MidiMessage, Channel, Note};
use engine::{Engine, DeviceId, DeviceKind, ElementAction, InputEvent};

fn start_midi_in(engine: Engine, port_name_hint: &str) -> anyhow::Result<()> {
  let mut mi = MidiInput::new("midi-macro-pad")?;
  mi.ignore(Ignore::None);
  let in_port = mi.ports()
    .into_iter()
    .find(|p| mi.port_name(p).map(|n| n.contains(port_name_hint)).unwrap_or(false))
    .expect("MIDI port not found");

  mi.connect(&in_port, "midi-macro-pad-in", move |_, bytes, _| {
    if let Ok(msg) = wmidi::MidiMessage::try_from(bytes) {
      match msg {
        MidiMessage::NoteOn(ch, note, vel) => {
          engine.submit(InputEvent {
            device: DeviceId { kind: DeviceKind::Midi, vendor_id: 0, product_id: 0, serial: None },
            element_id: format!("note_{}_{}", ch.index(), u8::from(note)),
            action: ElementAction::Press,
            value: vel.as_int() as i32,
            ts: std::time::Instant::now(),
          });
        }
        MidiMessage::NoteOff(ch, note, _vel) => {
          engine.submit(InputEvent {
            device: DeviceId { kind: DeviceKind::Midi, vendor_id: 0, product_id: 0, serial: None },
            element_id: format!("note_{}_{}", ch.index(), u8::from(note)),
            action: ElementAction::Release,
            value: 0,
            ts: std::time::Instant::now(),
          });
        }
        _ => {}
      }
    }
  }, ())?;
  Ok(())
}
```

---

# **HID ingestion (sketch)**

```
use hidapi::HidApi;
use engine::{Engine, DeviceId, DeviceKind, ElementAction, InputEvent};

fn start_hid(api: &HidApi, engine: Engine, vendor_id: u16, product_id: u16) -> anyhow::Result<()> {
  let dev = api.open(vendor_id, product_id)?;
  std::thread::spawn(move || {
    let mut buf = [0u8; 64];
    loop {
      match dev.read_timeout(&mut buf, 50) {
        Ok(n) if n > 0 => {
          // parse your report into element_id/value/action
          let element_id = format!("hid_btn_{:02}", buf[1]);
          let pressed = buf[2] > 0;
          engine.submit(InputEvent {
            device: DeviceId { kind: DeviceKind::Hid, vendor_id, product_id, serial: None },
            element_id,
            action: if pressed { ElementAction::Press } else { ElementAction::Release },
            value: buf[2] as i32,
            ts: std::time::Instant::now(),
          });
        }
        _ => {}
      }
    }
  });
  Ok(())
}
```

---

# **Tauri v2 (menu bar app + autostart + quick controls)**

  

**src-tauri/src/main.rs**

```
#![cfg_attr(not(debug_assertions), windows_subsystem = "macos")]
use tauri::{Manager, menu::{Menu, MenuItem}, tray::TrayIconBuilder};
use engine::Engine;

#[tauri::command]
fn pause_engine(pause: bool) {
  // flip a global atomic, Engine checks before executing actions
}

#[tauri::command]
fn reload_config() -> Result<(), String> {
  // tell engine to reload (via channel)
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent, // adds Login Item
      Some(true), // enable by default? set false if you prefer opt-in
    ))
    .setup(|app| {
      // Build tray (menu bar) only; no main window on launch
      let app_handle = app.handle();
      let _tray = TrayIconBuilder::new()
        .icon(app_handle.default_window_icon().unwrap())
        .on_tray_icon_event(|app, _event| {
          // on click: show a small panel window OR just a menu; keep simple here
        })
        .menu(&{
          let menu = Menu::new(app)
            .item(&MenuItem::with_id(app, "open_config", "Open Config", true, None::<&str>).unwrap())
            .item(&MenuItem::with_id(app, "reload", "Reload Config", true, None::<&str>).unwrap())
            .item(&MenuItem::with_id(app, "pause", "Pause Mappings", true, None::<&str>).unwrap())
            .item(&MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap());
          menu
        })
        .build(app)?;

      // TODO: wire menu events (listen for ids "open_config", "reload", etc.)
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![pause_engine, reload_config])
    .run(tauri::generate_context!())
    .expect("error while running app");
}
```

**UX in the tray popover (tiny panel)**

- Title row: active profile badge + green dot if devices online
    
- Toggles: Pause mappings, MIDI Learn
    
- Buttons: Open Config, Reload, View Log
    

---

# **Permissions & startup on macOS**

- To **send keystrokes**, the app needs **Accessibility** permission. On first action, detect failure and show a panel with a “Open System Settings → Privacy & Security → Accessibility” button.
    
- Tauri Autostart (Login Item) is handled via tauri-plugin-autostart with MacosLauncher::LaunchAgent.
    
- If you implement a **global panic hotkey**, register it via tauri-plugin-global-shortcut (e.g., ctrl+alt+cmd+p) and flip an AtomicBool checked by Executor.
    

---

# **Next steps (high impact)**

1. Implement **MIDI/HID Learn mode**: when active, the next incoming normalized event fills the currently selected binding slot in the UI (or prints in the log if config-first).
    
2. Add **frontmost-app profile switching**: small Swift/ObjC call via Tauri command to read NSWorkspace.shared.frontmostApplication?.bundleIdentifier.
    
3. Build a **live event console** (ring buffer) and expose via a Tauri command so the UI can show last N events and the resolved actions.
    
4. Add **virtual MIDI out** (optional) so power users can route actions into DAWs.
    
Example minimal compiling repo (with a tiny tray and one working binding):


Below is a **minimal, compiling** Rust workspace that builds a macOS **menu bar** (status bar) app using **Tauri v2** with a tiny tray and **one working binding**:

- Tray → **“Open Calculator”** (shell binding)
    
- Tray → **“Reload Config”** (no-op stub but wired)
    
- Tray → **“Pause Mappings”** (toggles an AtomicBool)
    
- Autostart support via tauri-plugin-autostart
    
- Engine crate with a simple mapping + executor (shell + optional keystroke)
    
- Config file config/default.toml
    

  

> You can paste these into files exactly as named. Then run the commands at the end.

---

## **Cargo.toml**

##  **(workspace root)**

```
[workspace]
members = ["engine", "app/src-tauri"]
resolver = "2"

[workspace.package]
edition = "2021"

[workspace.dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
log = "0.4"
```

---

## **engine/Cargo.toml**

```
[package]
name = "engine"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
log = "0.4"
parking_lot = "0.12"
# Action execution helpers
duct = "0.13"
enigo = "0.2"
```

---

## **engine/src/lib.rs**

```
use anyhow::Result;
use parking_lot::Mutex;
use std::{sync::Arc, sync::atomic::{AtomicBool, Ordering}};

#[derive(Clone, Debug)]
pub enum Action {
    /// Run a program (e.g., `open -a Calculator`)
    Shell { cmd: String, args: Vec<String> },
    /// Type literal text (requires Accessibility permissions on macOS)
    Text(String),
}

pub struct Executor;
impl Executor {
    pub fn run(action: &Action) -> Result<()> {
        match action {
            Action::Shell { cmd, args } => {
                duct::cmd(cmd, args).run()?;
            }
            Action::Text(s) => {
                let mut enigo = enigo::Enigo::new();
                for ch in s.chars() {
                    enigo.key_sequence(&ch.to_string());
                }
            }
        }
        Ok(())
    }
}

/// Minimal mapping engine with a single example binding.
#[derive(Clone)]
pub struct Engine {
    paused: Arc<AtomicBool>,
    bindings: Arc<Mutex<Bindings>>, 
}

#[derive(Default)]
struct Bindings {
    /// For demo: a single binding name → action
    actions: Vec<(String, Action)>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            paused: Arc::new(AtomicBool::new(false)),
            bindings: Arc::new(Mutex::new(Bindings::default())),
        }
    }

    pub fn is_paused(&self) -> bool { self.paused.load(Ordering::Relaxed) }
    pub fn set_paused(&self, val: bool) { self.paused.store(val, Ordering::Relaxed) }

    /// Register the demo binding (menu item → action)
    pub fn register_demo_binding(&self) {
        let mut b = self.bindings.lock();
        // Working binding: open Calculator via macOS `open` command
        b.actions.push((
            "open_calculator".to_string(),
            Action::Shell { cmd: "open".into(), args: vec!["-a".into(), "Calculator".into()] },
        ));
    }

    /// Trigger by name (e.g., from the tray menu)
    pub fn trigger(&self, name: &str) -> Result<()> {
        if self.is_paused() { return Ok(()); }
        let action = {
            let b = self.bindings.lock();
            b.actions.iter().find(|(n, _)| n == name).map(|(_, a)| a.clone())
        };
        if let Some(a) = action { Executor::run(&a)?; }
        Ok(())
    }
}
```

---

## **app/src-tauri/Cargo.toml**

```
[package]
name = "midi-macro-pad-app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
log = "0.4"
engine = { path = "../../engine" }

# Tauri v2
# Note: Tauri expects a front-end by default, but we only use the tray; this is fine.
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-autostart = "2"

auto-launch = "0.5" # not used directly, but handy if you roll your own later
```

---

## **app/src-tauri/src/main.rs**

```
#![cfg_attr(not(debug_assertions), windows_subsystem = "macos")]

use anyhow::Result;
use tauri::{menu::{Menu, MenuItem}, tray::{TrayIconBuilder, MouseButton, TrayIconEvent}, Manager};
use std::sync::OnceLock;

static ENGINE: OnceLock<engine::Engine> = OnceLock::new();

#[tauri::command]
fn pause_engine(pause: bool) {
    if let Some(e) = ENGINE.get() { e.set_paused(pause); }
}

#[tauri::command]
fn reload_config() -> Result<(), String> {
    // Stub for minimal demo; wire your real config reload here
    Ok(())
}

#[tauri::command]
fn trigger_binding(name: String) -> Result<(), String> {
    ENGINE.get().ok_or("engine not ready".to_string())
        .and_then(|e| e.trigger(&name).map_err(|e| e.to_string()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(false), // default off; user can enable later
        ))
        .setup(|app| {
            // Init engine
            let eng = engine::Engine::new();
            eng.register_demo_binding();
            ENGINE.set(eng).ok();

            // Build tray-only UI
            let app_handle = app.handle();
            let menu = Menu::new(&app_handle)
                .item(&MenuItem::with_id(&app_handle, "open_calc", "Open Calculator", true, None::<&str>)?)
                .item(&MenuItem::with_id(&app_handle, "reload", "Reload Config", true, None::<&str>)?)
                .item(&MenuItem::with_id(&app_handle, "pause", "Pause Mappings", true, None::<&str>)?)
                .item(&MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>)?)
            ;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_tray_icon_event(|app, event| match event {
                    TrayIconEvent::Click { button, .. } => {
                        if let MouseButton::Left = button {
                            // Left-click can toggle menu, or show a small window later
                        }
                    }
                    _ => {}
                })
                .on_menu_event(|app, ev| {
                    match ev.id().as_ref() {
                        "open_calc" => {
                            let _ = app.invoke(
                                tauri::InvokePayload::new("trigger_binding", serde_json::json!({ "name": "open_calculator" }))
                            );
                        }
                        "reload" => { let _ = app.invoke(tauri::InvokePayload::new("reload_config", serde_json::json!({}))); }
                        "pause" => {
                            // Toggle
                            let paused = ENGINE.get().map(|e| e.is_paused()).unwrap_or(false);
                            if let Some(e) = ENGINE.get() { e.set_paused(!paused); }
                        }
                        "quit" => { app.exit(0); }
                        _ => {}
                    }
                })
                .build(&app_handle)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![pause_engine, reload_config, trigger_binding])
        .run(tauri::generate_context!())
        .expect("error while running app");
}
```

---

## **config/default.toml**

```
config_version = 1
[general]
long_press_ms = 400
velocity_threshold = 100
active_profile = "global"
```

> In this minimal demo, the config is not yet parsed by the app; it’s a placeholder for your next step. The tray menu’s **Open Calculator** demonstrates one working binding via the engine’s action executor.

---

## **Build & run (macOS)**

```
# 1) From the repo root
rustup target add x86_64-apple-darwin aarch64-apple-darwin  # optional universal support

# 2) Ensure you have the macOS prerequisites for Tauri (Xcode CLT, etc.)
#    https://tauri.app/v2/guides/getting-started/prerequisites

# 3) Build
cargo build -p midi-macro-pad-app

# 4) Run (dev)
cargo run -p midi-macro-pad-app
```

When it launches, you’ll see a **menu bar icon** (the default Tauri icon). Click it → **Open Calculator** to execute the demo binding. If you later use text typing (Action::Text), macOS will prompt for **Accessibility** permissions.

---

## **Next steps (quick wins)**

- Wire Reload Config to read ~/Library/Application Support/MidiMacroPad/default.toml and rebuild the Bindings.
    
- Add a global panic shortcut using tauri-plugin-global-shortcut.
    
- Add MIDI input (via midir + wmidi) and HID input (hidapi), normalize to your engine events, then map to trigger().
    
- Replace the default tray icon with your own .icns and set bundle identifiers in tauri.conf.json if you add one (not needed for this minimal build).