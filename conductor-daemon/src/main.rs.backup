// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use chrono::Local;
use colored::*;
use crossbeam_channel::{Receiver, bounded};
use midir::{MidiInput, MidiInputConnection};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::thread;
use std::time::Duration;

mod actions;
mod config;
mod mappings;

use actions::ActionExecutor;
use config::Config;
use mappings::MappingEngine;

#[derive(Debug, Clone)]
pub enum MidiEvent {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    ControlChange { cc: u8, value: u8 },
    ProgramChange { program: u8 },
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
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::load("config.toml")?;

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
                let event = match message[0] & 0xF0 {
                    0x90 if message[2] > 0 => Some(MidiEvent::NoteOn {
                        note: message[1],
                        velocity: message[2],
                    }),
                    0x80 | 0x90 => Some(MidiEvent::NoteOff { note: message[1] }),
                    0xB0 => Some(MidiEvent::ControlChange {
                        cc: message[1],
                        value: message[2],
                    }),
                    0xC0 => Some(MidiEvent::ProgramChange {
                        program: message[1],
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
            MidiEvent::NoteOn { note, velocity } => {
                format!("Note ON  {:3} vel {:3}", note, velocity).green()
            }
            MidiEvent::NoteOff { note } => format!("Note OFF {:3}        ", note).yellow(),
            MidiEvent::ControlChange { cc, value } => {
                format!("CC       {:3} val {:3}", cc, value).blue()
            }
            MidiEvent::ProgramChange { program } => {
                format!("Program  {:3}        ", program).magenta()
            }
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
            MidiEvent::ControlChange { cc: 0, value } => {
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
    println!("{}", "╔══════════════════════════════════╗".cyan().bold());
    println!("{}", "║     MIDI Macro Pad Controller    ║".cyan().bold());
    println!("{}", "╚══════════════════════════════════╝".cyan().bold());
    println!();

    let mut pad = MidiMacroPad::new()?;

    // List available ports
    pad.list_midi_ports()?;

    // Auto-connect to first device or use command line arg
    let port_index = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    pad.connect(port_index)?;
    pad.run()?;

    Ok(())
}
