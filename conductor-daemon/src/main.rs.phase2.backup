// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use chrono::Local;
use clap::{Parser, ValueEnum};
use colored::*;
use crossbeam_channel::{Receiver, bounded};
use midir::{MidiInput, MidiInputConnection};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::{Duration, Instant};

// Import from midimon_core instead of inline modules
use midimon_core::{Config, MidiEvent, mapping::MappingEngine};
use midimon_daemon::ActionExecutor;

/// MIDIMon - MIDI controller mapping system
///
/// Transform MIDI devices into advanced macro pads with velocity sensitivity,
/// long press detection, double-tap, chord detection, and RGB LED feedback.
#[derive(Parser, Debug)]
#[command(name = "midimon")]
#[command(version)]
#[command(about = "MIDI Macro Pad Controller", long_about = None)]
struct Args {
    /// MIDI port index to connect to (list ports with --list)
    #[arg(value_name = "PORT")]
    port: Option<usize>,

    /// Path to configuration file
    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    config: PathBuf,

    /// LED lighting scheme
    #[arg(short, long, value_enum)]
    led: Option<LedScheme>,

    /// Device profile file (.ncmm3 for Native Instruments)
    #[arg(short, long, value_name = "FILE")]
    profile: Option<PathBuf>,

    /// Pad page to use (A-H for Mikro MK3)
    #[arg(long, value_name = "PAGE")]
    pad_page: Option<String>,

    /// List available MIDI ports and exit
    #[arg(short = 'L', long)]
    list: bool,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

/// LED lighting schemes
#[derive(Copy, Clone, Debug, ValueEnum)]
enum LedScheme {
    /// LEDs off
    Off,
    /// Static color based on mode
    Static,
    /// Slow breathing effect
    Breathing,
    /// Fast pulse effect
    Pulse,
    /// Rainbow cycle
    Rainbow,
    /// Wave pattern
    Wave,
    /// Random sparkles
    Sparkle,
    /// React to MIDI events only
    Reactive,
    /// VU meter style (bottom-up)
    VuMeter,
    /// Spiral pattern
    Spiral,
}

pub struct MidiMacroPad {
    config: Config,
    midi_connection: Option<MidiInputConnection<()>>,
    #[allow(dead_code)]
    action_executor: ActionExecutor,
    #[allow(dead_code)]
    mapping_engine: MappingEngine,
    current_mode: Arc<AtomicU8>,
    running: Arc<AtomicBool>,
}

impl MidiMacroPad {
    pub fn new(config_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load(config_path.to_str().unwrap_or("config.toml"))?;

        Ok(Self {
            config,
            midi_connection: None,
            action_executor: ActionExecutor::new(),
            mapping_engine: MappingEngine::new(),
            current_mode: Arc::new(AtomicU8::new(0)),
            running: Arc::new(AtomicBool::new(true)),
        })
    }

    pub fn list_midi_ports(&self) -> Result<(), Box<dyn std::error::Error>> {
        let midi_in = MidiInput::new("MidiMacroPad Scanner")?;

        println!("{}", "Available MIDI input ports:".green().bold());
        println!("{}", "─".repeat(40).dimmed());

        let ports = midi_in.ports();
        for (i, p) in ports.iter().enumerate() {
            let name = midi_in.port_name(p)?;
            println!("  {} {}", format!("[{}]", i).cyan(), name);
        }

        if ports.is_empty() {
            println!("  {}", "No MIDI devices found!".red());
        }

        Ok(())
    }

    pub fn connect(&mut self, port_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let midi_in = MidiInput::new("MidiMacroPad")?;
        let ports = midi_in.ports();

        if port_index >= ports.len() {
            return Err("Invalid port index".into());
        }

        let port = &ports[port_index];
        let port_name = midi_in.port_name(port)?;

        println!("{} {}", "Connecting to:".green(), port_name.yellow());

        let (tx, rx) = bounded::<MidiEvent>(100);
        let _current_mode = Arc::clone(&self.current_mode);

        // Set up MIDI input callback
        let connection = midi_in.connect(
            port,
            "midi-macro-pad",
            move |_timestamp, message, _| {
                let now = Instant::now();
                let event = match message[0] & 0xF0 {
                    0x90 if message[2] > 0 => Some(MidiEvent::NoteOn {
                        note: message[1],
                        velocity: message[2],
                        time: now,
                    }),
                    0x80 | 0x90 => Some(MidiEvent::NoteOff {
                        note: message[1],
                        time: now,
                    }),
                    0xB0 => Some(MidiEvent::ControlChange {
                        cc: message[1],
                        value: message[2],
                        time: now,
                    }),
                    0xC0 => Some(MidiEvent::ProgramChange {
                        program: message[1],
                        time: now,
                    }),
                    _ => None,
                };

                if let Some(event) = event {
                    let _ = tx.try_send(event);
                }
            },
            (),
        )?;

        self.midi_connection = Some(connection);

        // Start event processing thread
        let running = Arc::clone(&self.running);
        let config = self.config.clone();
        let current_mode_thread = Arc::clone(&self.current_mode);

        thread::spawn(move || {
            Self::process_events(rx, config, running, current_mode_thread);
        });

        Ok(())
    }

    fn process_events(
        rx: Receiver<MidiEvent>,
        config: Config,
        running: Arc<AtomicBool>,
        current_mode: Arc<AtomicU8>,
    ) {
        let mut executor = ActionExecutor::new();
        let mut mapping_engine = MappingEngine::new();

        // Load mappings from config
        mapping_engine.load_from_config(&config);

        while running.load(Ordering::Relaxed) {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    let mode = current_mode.load(Ordering::Relaxed);

                    // Log the event
                    Self::log_event(&event, mode);

                    // Check for mode change
                    if let Some(new_mode) = Self::check_mode_change(&event, &config) {
                        current_mode.store(new_mode, Ordering::Relaxed);
                        println!(
                            "{} {}",
                            "Mode changed to:".cyan().bold(),
                            config
                                .modes
                                .get(new_mode as usize)
                                .map(|m| m.name.as_str())
                                .unwrap_or("Unknown")
                                .yellow()
                        );
                        continue;
                    }

                    // Get and execute action
                    if let Some(action) = mapping_engine.get_action(&event, mode) {
                        executor.execute(action);
                    }
                }
                Err(crossbeam_channel::RecvTimeoutError::Timeout) => continue,
                Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    fn log_event(event: &MidiEvent, mode: u8) {
        let timestamp = Local::now().format("%H:%M:%S%.3f");
        let mode_str = format!("[Mode {}]", mode).cyan();

        let event_str = match event {
            MidiEvent::NoteOn { note, velocity, .. } => {
                format!("Note ON  {:3} vel {:3}", note, velocity).green()
            }
            MidiEvent::NoteOff { note, .. } => format!("Note OFF {:3}        ", note).yellow(),
            MidiEvent::ControlChange { cc, value, .. } => {
                format!("CC       {:3} val {:3}", cc, value).blue()
            }
            MidiEvent::ProgramChange { program, .. } => {
                format!("Program  {:3}        ", program).magenta()
            }
            MidiEvent::Aftertouch { pressure, .. } => {
                format!("Aftertouch   {:3}    ", pressure).cyan()
            }
            MidiEvent::PitchBend { value, .. } => format!("PitchBend    {:5}   ", value).magenta(),
        };

        println!(
            "{} {} {}",
            timestamp.to_string().dimmed(),
            mode_str,
            event_str
        );
    }

    fn check_mode_change(event: &MidiEvent, config: &Config) -> Option<u8> {
        // Use CC 0 for mode changes (bank select)
        match event {
            MidiEvent::ControlChange { cc: 0, value, .. } => {
                if (*value as usize) < config.modes.len() {
                    Some(*value)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "{}",
            "MIDI Macro Pad running. Press Ctrl+C to exit."
                .green()
                .bold()
        );
        println!("{}", "─".repeat(50).dimmed());

        // Wait for Ctrl+C
        let running = Arc::clone(&self.running);
        ctrlc::set_handler(move || {
            running.store(false, Ordering::Relaxed);
        })?;

        while self.running.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
        }

        println!("\n{}", "Shutting down...".yellow());
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Enable debug logging if requested
    if args.debug {
        unsafe {
            std::env::set_var("DEBUG", "1");
        }
        println!("{}", "Debug logging enabled".yellow());
    }

    // Print banner
    println!("{}", "╔══════════════════════════════════╗".cyan().bold());
    println!("{}", "║     MIDI Macro Pad Controller    ║".cyan().bold());
    println!("{}", "╚══════════════════════════════════╝".cyan().bold());
    println!();

    // Show configuration info
    if args.debug {
        println!("{}", format!("Config: {}", args.config.display()).dimmed());
        if let Some(ref profile) = args.profile {
            println!("{}", format!("Profile: {}", profile.display()).dimmed());
        }
        if let Some(ref led) = args.led {
            println!("{}", format!("LED scheme: {:?}", led).dimmed());
        }
        if let Some(ref page) = args.pad_page {
            println!("{}", format!("Pad page: {}", page).dimmed());
        }
        println!();
    }

    // Create MIDI pad with specified config
    let mut pad = MidiMacroPad::new(&args.config)?;

    // List available ports
    pad.list_midi_ports()?;

    // If --list flag is set, exit after listing ports
    if args.list {
        return Ok(());
    }

    // Determine which port to connect to
    let port_index = match args.port {
        Some(port) => port,
        None => {
            // Try to auto-connect to first device
            let midi_in = midir::MidiInput::new("MidiMacroPad Scanner")?;
            let ports = midi_in.ports();
            if ports.is_empty() {
                eprintln!("{}", "No MIDI devices found!".red());
                return Err("No MIDI devices available".into());
            }
            0 // Default to first port
        }
    };

    // Connect to the selected port
    pad.connect(port_index)?;

    // Display LED scheme if specified
    if let Some(led) = args.led {
        println!("{} {:?}", "LED scheme:".cyan(), led);
        // Note: LED scheme integration would go here
        // This would require passing the scheme to the feedback system
    }

    // Display profile info if specified
    if let Some(ref profile) = args.profile {
        println!(
            "{} {}",
            "Using profile:".cyan(),
            profile.display().to_string().yellow()
        );
        if let Some(ref page) = args.pad_page {
            println!("{} {}", "Pad page:".cyan(), page.yellow());
        }
        // Note: Profile loading would go here
        // This would require integrating with the device profile system
    }

    // Run the MIDI pad
    pad.run()?;

    Ok(())
}
