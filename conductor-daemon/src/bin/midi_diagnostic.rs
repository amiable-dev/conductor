// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use colored::Colorize;
use midi_msg::{ChannelVoiceMsg, MidiMsg};
use midir::MidiInput;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Convert MIDI note number (0-127) to musical note name (e.g., "C4", "A#3")
fn note_to_name(note: u8) -> String {
    const NOTE_NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let octave = (note / 12) as i32 - 1; // MIDI note 60 = C4
    let note_name = NOTE_NAMES[(note % 12) as usize];
    format!("{}{}", note_name, octave)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        "╔══════════════════════════════════════╗".cyan().bold()
    );
    println!(
        "{}",
        "║      MIDI Diagnostic Tool            ║".cyan().bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════╝".cyan().bold()
    );
    println!();

    let midi_in = MidiInput::new("Diagnostic")?;
    let ports = midi_in.ports();

    // List all ports
    println!("{}", "Available MIDI Ports:".green().bold());
    println!("{}", "─".repeat(40).dimmed());
    for (i, port) in ports.iter().enumerate() {
        let name = midi_in.port_name(port)?;
        println!("  {} {}", format!("[{}]", i).cyan(), name);
    }
    println!();

    // Find Mikro MK3 or use command line argument
    let port_index = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<usize>().ok())
        .or_else(|| {
            ports
                .iter()
                .position(|p| midi_in.port_name(p).unwrap_or_default().contains("Mikro"))
        });

    let port = match port_index {
        Some(idx) if idx < ports.len() => &ports[idx],
        _ => {
            eprintln!("{}", "No Mikro MK3 found and no valid port specified".red());
            eprintln!("Usage: {} [port_number]", std::env::args().next().unwrap());
            return Ok(());
        }
    };

    let port_name = midi_in.port_name(port)?;
    println!("{} {}", "Connecting to:".green(), port_name.yellow());
    println!();

    let start_time = Instant::now();
    let event_count = Arc::new(Mutex::new(0u32));
    let held_notes = Arc::new(Mutex::new(std::collections::HashMap::new()));
    let status_line_active = Arc::new(Mutex::new(false));

    let event_count_clone = Arc::clone(&event_count);
    let held_notes_clone = Arc::clone(&held_notes);
    let status_line_active_clone = Arc::clone(&status_line_active);

    let _conn = midi_in.connect(
        port,
        "diagnostic",
        move |midi_timestamp, msg, _| {
            let now = Instant::now();
            let elapsed = now.duration_since(start_time);

            // Show both MIDI timestamp (microseconds from device) and elapsed time
            let timestamp = format!(
                "{:6.3}s (MIDI: {:10}μs)",
                elapsed.as_secs_f32(),
                midi_timestamp
            );

            // Clear status line if it's active
            let mut status_active = status_line_active_clone.lock().unwrap();
            if *status_active {
                print!("\r{:80}\r", ""); // Clear the line
                *status_active = false;
            }
            drop(status_active);

            let mut count = event_count_clone.lock().unwrap();
            *count += 1;
            print!("{} #{:4} | ", timestamp.dimmed(), count);

            // Parse MIDI message using midi-msg library
            match MidiMsg::from_midi(msg) {
                Ok((
                    MidiMsg::ChannelVoice {
                        channel,
                        msg: voice_msg,
                    },
                    _,
                ))
                | Ok((
                    MidiMsg::RunningChannelVoice {
                        channel,
                        msg: voice_msg,
                    },
                    _,
                )) => {
                    let ch = channel as u8 + 1; // Display as 1-based

                    match voice_msg {
                        ChannelVoiceMsg::NoteOn { note, velocity } => {
                            let note_name = note_to_name(note);

                            if velocity > 0 {
                                // Real note on
                                held_notes_clone.lock().unwrap().insert(note, now);

                                let vel_bar = "█".repeat((velocity as usize * 20) / 127);
                                println!(
                                    "{} {:>3} ({:3}) vel={:3} ch={:2} {}",
                                    "Note ON ".green().bold(),
                                    note_name.cyan(),
                                    note,
                                    velocity,
                                    ch,
                                    vel_bar.green()
                                );
                            } else {
                                // Note on with velocity 0 (acts as note off)
                                if let Some(press_time) =
                                    held_notes_clone.lock().unwrap().remove(&note)
                                {
                                    let duration = now.duration_since(press_time);
                                    println!(
                                        "{} {:>3} ({:3}) vel=  0 ch={:2} (held {:.3}s)",
                                        "Note OFF".yellow().bold(),
                                        note_name.cyan(),
                                        note,
                                        ch,
                                        duration.as_secs_f32()
                                    );
                                }
                            }
                        }

                        ChannelVoiceMsg::NoteOff { note, velocity: _ } => {
                            let note_name = note_to_name(note);

                            if let Some(press_time) = held_notes_clone.lock().unwrap().remove(&note)
                            {
                                let duration = now.duration_since(press_time);
                                println!(
                                    "{} {:>3} ({:3})         ch={:2} (held {:.3}s)",
                                    "Note OFF".yellow().bold(),
                                    note_name.cyan(),
                                    note,
                                    ch,
                                    duration.as_secs_f32()
                                );
                            } else {
                                println!(
                                    "{} {:>3} ({:3})         ch={:2}",
                                    "Note OFF".yellow().bold(),
                                    note_name.cyan(),
                                    note,
                                    ch
                                );
                            }
                        }

                        ChannelVoiceMsg::ControlChange { control } => {
                            // Extract control and value from ControlChange enum
                            use midi_msg::ControlChange;
                            if let ControlChange::CC { control: cc, value } = control {
                                let val_bar = "▬".repeat((value as usize * 20) / 127);
                                println!(
                                    "{}   cc={:3} val={:3} ch={:2} {}",
                                    "CC      ".blue().bold(),
                                    cc,
                                    value,
                                    ch,
                                    val_bar.blue()
                                );
                            } else {
                                // For other ControlChange variants, just show raw bytes
                                println!("{} {:02X?}", "CC      ".blue().bold(), msg);
                            }
                        }

                        ChannelVoiceMsg::PolyPressure { note, pressure } => {
                            let note_name = note_to_name(note);
                            let pressure_bar = "▓".repeat((pressure as usize * 20) / 127);
                            println!(
                                "{} {:>3} ({:3}) pres={:3} ch={:2} {}",
                                "PolyAT  ".purple().bold(),
                                note_name.cyan(),
                                note,
                                pressure,
                                ch,
                                pressure_bar.purple()
                            );
                        }

                        ChannelVoiceMsg::ChannelPressure { pressure } => {
                            let pressure_bar = "▓".repeat((pressure as usize * 20) / 127);
                            println!(
                                "{} pres={:3}         ch={:2} {}",
                                "ChanAT  ".purple().bold(),
                                pressure,
                                ch,
                                pressure_bar.purple()
                            );
                        }

                        ChannelVoiceMsg::PitchBend { bend } => {
                            let centered = bend as i32 - 8192; // Center is 8192

                            let direction = if centered > 0 {
                                "↑"
                            } else if centered < 0 {
                                "↓"
                            } else {
                                "◯"
                            };
                            println!(
                                "{} value={:5} ({:+6}) ch={:2} {}",
                                "PitchBend".magenta().bold(),
                                bend,
                                centered,
                                ch,
                                direction
                            );
                        }

                        ChannelVoiceMsg::ProgramChange { program } => {
                            println!(
                                "{} prog={:3}         ch={:2}",
                                "ProgChg ".cyan().bold(),
                                program,
                                ch
                            );
                        }

                        _ => {
                            // Other voice messages (HighResNoteOn, HighResNoteOff, etc.)
                            println!("{} {:02X?}", "Voice   ".cyan().bold(), msg);
                        }
                    }
                }

                Ok((MidiMsg::SystemCommon { .. }, _)) | Ok((MidiMsg::SystemRealTime { .. }, _)) => {
                    println!("{} {:02X?}", "System  ".white().bold(), msg);
                }

                _ => {
                    println!("{} {:02X?}", "Unknown ".red().bold(), msg);
                }
            }
        },
        (),
    )?;

    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!("{}", "Listening for MIDI events...".green());
    println!("{}", "Press Ctrl+C to exit".yellow());
    println!();
    println!("{}", "Try these tests:".cyan().bold());
    println!("  • Press pads with different velocities");
    println!("  • Hold pads down for different durations");
    println!("  • Try double-tapping pads quickly");
    println!("  • Press multiple pads simultaneously (chords)");
    println!("  • Turn encoders slowly and quickly");
    println!("  • Use the touch strip");
    println!("  • Test Shift + pad combinations");
    println!();
    println!("{}", "═".repeat(50).dimmed());
    println!();

    // Keep the program running
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Show currently held notes
        let held = held_notes.lock().unwrap();
        if !held.is_empty() {
            let held_list: Vec<String> = held
                .iter()
                .map(|(note, press_time)| {
                    let duration = Instant::now().duration_since(*press_time);
                    format!("{}({:.1}s)", note_to_name(*note), duration.as_secs_f32())
                })
                .collect();

            print!(
                "\r{} [{}]    ",
                "Currently held:".cyan(),
                held_list.join(", ").yellow()
            );
            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            // Mark that status line is active
            *status_line_active.lock().unwrap() = true;
        } else {
            // Clear the line if no notes are held
            let mut status_active = status_line_active.lock().unwrap();
            if *status_active {
                print!("\r{:80}\r", " ");
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                *status_active = false;
            }
        }
    }
}
