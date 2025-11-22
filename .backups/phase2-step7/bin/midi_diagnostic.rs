// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use colored::Colorize;
use midir::MidiInput;
use std::sync::{Arc, Mutex};
use std::time::Instant;

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

    let event_count_clone = Arc::clone(&event_count);
    let held_notes_clone = Arc::clone(&held_notes);

    let _conn = midi_in.connect(
        port,
        "diagnostic",
        move |_stamp, msg, _| {
            let now = Instant::now();
            let elapsed = now.duration_since(start_time);
            let timestamp = format!("{:6.3}s", elapsed.as_secs_f32());

            let mut count = event_count_clone.lock().unwrap();
            *count += 1;
            print!("{} #{:4} | ", timestamp.dimmed(), count);

            match msg[0] & 0xF0 {
                0x90 => {
                    // Note On
                    let note = msg[1];
                    let velocity = msg[2];
                    let channel = (msg[0] & 0x0F) + 1;

                    if velocity > 0 {
                        // Real note on
                        held_notes_clone.lock().unwrap().insert(note, now);

                        let vel_bar = "█".repeat((velocity as usize * 20) / 127);
                        println!(
                            "{} note={:3} vel={:3} ch={:2} {}",
                            "Note ON ".green().bold(),
                            note,
                            velocity,
                            channel,
                            vel_bar.green()
                        );
                    } else {
                        // Note on with velocity 0 (acts as note off)
                        if let Some(press_time) = held_notes_clone.lock().unwrap().remove(&note) {
                            let duration = now.duration_since(press_time);
                            println!(
                                "{} note={:3} vel=  0 ch={:2} (held {:.3}s)",
                                "Note OFF".yellow().bold(),
                                note,
                                channel,
                                duration.as_secs_f32()
                            );
                        }
                    }
                }

                0x80 => {
                    // Note Off
                    let note = msg[1];
                    let channel = (msg[0] & 0x0F) + 1;

                    if let Some(press_time) = held_notes_clone.lock().unwrap().remove(&note) {
                        let duration = now.duration_since(press_time);
                        println!(
                            "{} note={:3}         ch={:2} (held {:.3}s)",
                            "Note OFF".yellow().bold(),
                            note,
                            channel,
                            duration.as_secs_f32()
                        );
                    } else {
                        println!(
                            "{} note={:3}         ch={:2}",
                            "Note OFF".yellow().bold(),
                            note,
                            channel
                        );
                    }
                }

                0xB0 => {
                    // Control Change
                    let controller = msg[1];
                    let value = msg[2];
                    let channel = (msg[0] & 0x0F) + 1;

                    let val_bar = "▬".repeat((value as usize * 20) / 127);
                    println!(
                        "{}   cc={:3} val={:3} ch={:2} {}",
                        "CC      ".blue().bold(),
                        controller,
                        value,
                        channel,
                        val_bar.blue()
                    );
                }

                0xD0 => {
                    // Channel Aftertouch
                    let pressure = msg[1];
                    let channel = (msg[0] & 0x0F) + 1;

                    let pressure_bar = "▓".repeat((pressure as usize * 20) / 127);
                    println!(
                        "{} pressure={:3}     ch={:2} {}",
                        "Aftertouch".purple().bold(),
                        pressure,
                        channel,
                        pressure_bar.purple()
                    );
                }

                0xE0 => {
                    // Pitch Bend
                    let value = ((msg[2] as u16) << 7) | (msg[1] as u16);
                    let channel = (msg[0] & 0x0F) + 1;
                    let centered = value as i32 - 8192; // Center is 8192

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
                        value,
                        centered,
                        channel,
                        direction
                    );
                }

                0xC0 => {
                    // Program Change
                    let program = msg[1];
                    let channel = (msg[0] & 0x0F) + 1;

                    println!(
                        "{} prog={:3}         ch={:2}",
                        "ProgChg ".cyan().bold(),
                        program,
                        channel
                    );
                }

                0xF0 => {
                    // System messages
                    println!("{} {:02X?}", "System  ".white().bold(), msg);
                }

                _ => {
                    // Unknown message
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
                    format!("{}({:.1}s)", note, duration.as_secs_f32())
                })
                .collect();

            print!(
                "\r{} [{}]    ",
                "Currently held:".cyan(),
                held_list.join(", ").yellow()
            );
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        } else {
            // Clear the line if no notes are held
            print!("\r{:60}", " ");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    }
}
