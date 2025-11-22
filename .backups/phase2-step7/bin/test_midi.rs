// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

// Create a file called test_midi.rs in src/bin/
// src/bin/test_midi.rs

use midir::{MidiInput, MidiOutput};

fn main() {
    println!("=== MIDI Input Ports ===");
    match MidiInput::new("MidiTest") {
        Ok(midi_in) => {
            for (i, p) in midi_in.ports().iter().enumerate() {
                match midi_in.port_name(p) {
                    Ok(name) => println!("{}: {}", i, name),
                    Err(e) => println!("{}: Error getting name: {}", i, e),
                }
            }
            if midi_in.ports().is_empty() {
                println!("No input ports found");
            }
        }
        Err(e) => println!("Error creating MIDI input: {}", e),
    }

    println!("\n=== MIDI Output Ports ===");
    match MidiOutput::new("MidiTest") {
        Ok(midi_out) => {
            for (i, p) in midi_out.ports().iter().enumerate() {
                match midi_out.port_name(p) {
                    Ok(name) => println!("{}: {}", i, name),
                    Err(e) => println!("{}: Error getting name: {}", i, e),
                }
            }
            if midi_out.ports().is_empty() {
                println!("No output ports found");
            }
        }
        Err(e) => println!("Error creating MIDI output: {}", e),
    }

    // Also check virtual ports
    println!("\n=== System Info ===");
    println!("OS: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
}
