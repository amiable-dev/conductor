// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

// This diagnostic tool only uses external crates (hidapi, midir)
// and standard library - no conductor_core imports needed

use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MIKRO_MK3_VENDOR_ID: u16 = 0x17cc;
const MIKRO_MK3_PRODUCT_ID: u16 = 0x1700;
const LED_REPORT_ID: u8 = 0x80;

fn main() -> Result<(), Box<dyn Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Mikro MK3 LED Address Finder                             ║");
    println!("║  Tests different LED addresses to find the right mapping  ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Open HID device
    let api = hidapi::HidApi::new()?;
    let mut hid_device = None;

    for device in api.device_list() {
        if device.vendor_id() == MIKRO_MK3_VENDOR_ID
            && device.product_id() == MIKRO_MK3_PRODUCT_ID
            && device.interface_number() == 0
        {
            hid_device = Some(device.open_device(&api)?);
            break;
        }
    }
    let hid_device = Arc::new(Mutex::new(hid_device.ok_or("Mikro MK3 HID not found")?));
    println!("✓ Connected to Mikro MK3 HID");

    // Capture MIDI notes
    let captured_note = Arc::new(Mutex::new(None::<u8>));
    let mut midi_in = MidiInput::new("LED Tester")?;
    midi_in.ignore(Ignore::None);

    let ports = midi_in.ports();
    let mikro_port = ports
        .iter()
        .find(|p| {
            midi_in
                .port_name(p)
                .unwrap_or_default()
                .contains("Mikro MK3")
        })
        .ok_or("Mikro MK3 MIDI not found")?;

    let captured_note_clone = Arc::clone(&captured_note);
    let _midi_conn = midi_in.connect(
        mikro_port,
        "led-tester",
        move |_stamp, message, _| {
            if message.len() >= 3 && (message[0] & 0xF0) == 0x90 && message[2] > 0 {
                *captured_note_clone.lock().unwrap() = Some(message[1]);
            }
        },
        (),
    )?;

    println!("✓ Connected to Mikro MK3 MIDI");
    println!();

    println!("Instructions:");
    println!("  1. Press a pad on Pad Page A");
    println!("  2. I'll try different LED addresses");
    println!("  3. Tell me when you see RED light up");
    println!();

    loop {
        println!("─────────────────────────────────────────────");
        print!("Press any pad (or 'q' to quit): ");
        io::stdout().flush()?;

        // Wait for pad press
        *captured_note.lock().unwrap() = None;
        let start = std::time::Instant::now();
        let note = loop {
            if start.elapsed() > Duration::from_secs(60) {
                println!("Timeout");
                continue;
            }
            if let Some(n) = *captured_note.lock().unwrap() {
                break n;
            }
            thread::sleep(Duration::from_millis(10));
        };

        println!("Captured MIDI note {}", note);
        println!();
        println!("Testing LED positions for note {}...", note);

        // Try different strategies:
        // Strategy 1: Direct note index (note * 3 for RGB)
        // Strategy 2: Note offset from 12 (* 3 for RGB)
        // Strategy 3: Specific buffer positions

        let test_strategies = vec![
            ("Direct: note * 3", (note * 3) as usize),
            (
                "Offset from 12: (note - 12) * 3",
                ((note - 12) * 3) as usize,
            ),
            ("Offset from 12 + 1", ((note - 12) * 3 + 1) as usize),
            ("Note value as offset", note as usize),
            ("Note + 16", (note + 16) as usize),
        ];

        for (desc, offset) in test_strategies {
            if offset >= 62 {
                continue; // Skip if would overflow RGB triplet
            }

            // Clear all LEDs first
            let mut buffer = vec![LED_REPORT_ID];
            buffer.resize(80, 0);

            // Set this offset to bright RED
            buffer[offset] = 255; // R
            buffer[offset + 1] = 0; // G
            buffer[offset + 2] = 0; // B

            // Pad to 65 bytes and write
            buffer.resize(65, 0);
            if let Ok(dev) = hid_device.lock() {
                let _ = dev.write(&buffer);
            }

            println!("  Testing {} (offset {})...", desc, offset);
            thread::sleep(Duration::from_millis(800));

            // Check if user saw it
            print!("      Did you see RED light? (y/n): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                println!();
                println!("✓✓✓ FOUND IT! ✓✓✓");
                println!("MIDI Note {} → {} (offset {})", note, desc, offset);
                println!();
                break;
            }

            // Turn off
            let mut buffer = vec![LED_REPORT_ID];
            buffer.resize(65, 0);
            if let Ok(dev) = hid_device.lock() {
                let _ = dev.write(&buffer);
            }
        }

        print!("Test another pad? (y/n): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    // Clear all LEDs
    let mut buffer = vec![LED_REPORT_ID];
    buffer.resize(65, 0);
    if let Ok(dev) = hid_device.lock() {
        let _ = dev.write(&buffer);
    }

    println!();
    println!("Done!");

    Ok(())
}
