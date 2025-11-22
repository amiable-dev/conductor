// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

// This diagnostic tool only uses external crates (hidapi, midir)
// and standard library - no conductor_core imports needed

use hidapi::{HidApi, HidDevice};
use midir::{Ignore, MidiInput};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Native Instruments Vendor ID
const NI_VENDOR_ID: u16 = 0x17CC;
const MIKRO_MK3_PRODUCT_ID: u16 = 0x1700;
const LED_REPORT_ID: u8 = 0x80;

struct CaptureData {
    midi_note: Option<u8>,
    midi_velocity: Option<u8>,
    hid_buffer: Vec<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("╔════════════════════════════════════════╗");
    println!("║  Mikro MK3 LED Diagnostic Tool        ║");
    println!("║  Press pads to capture MIDI + HID data ║");
    println!("╚════════════════════════════════════════╝\n");

    // Open HID device for LED control
    let api = HidApi::new()?;
    let mut hid_device: Option<HidDevice> = None;

    for device_info in api.device_list() {
        if device_info.vendor_id() == NI_VENDOR_ID
            && device_info.product_id() == MIKRO_MK3_PRODUCT_ID
            && device_info.interface_number() == 0
        {
            println!("✓ Found Mikro MK3 HID interface");
            hid_device = Some(device_info.open_device(&api)?);
            break;
        }
    }

    let hid_device = hid_device.ok_or("Mikro MK3 HID not found")?;

    // Open MIDI input
    let mut midi_in = MidiInput::new("LED Diagnostic MIDI")?;
    midi_in.ignore(Ignore::None);

    let midi_ports = midi_in.ports();
    let mikro_port = midi_ports
        .iter()
        .find(|p| {
            midi_in
                .port_name(p)
                .unwrap_or_default()
                .contains("Mikro MK3")
        })
        .ok_or("Mikro MK3 MIDI port not found")?;

    println!("✓ Found Mikro MK3 MIDI input\n");

    // Shared capture data
    let capture = Arc::new(Mutex::new(CaptureData {
        midi_note: None,
        midi_velocity: None,
        hid_buffer: Vec::new(),
    }));

    let capture_clone = capture.clone();

    // Connect MIDI input
    let _midi_conn = midi_in.connect(
        mikro_port,
        "diagnostic",
        move |_timestamp, message, _| {
            if message.len() >= 3 && (message[0] & 0xF0) == 0x90 && message[2] > 0 {
                let mut cap = capture_clone.lock().unwrap();
                cap.midi_note = Some(message[1]);
                cap.midi_velocity = Some(message[2]);
                println!("\n📥 MIDI: Note {} velocity {}", message[1], message[2]);
            }
        },
        (),
    )?;

    println!("Ready! Press pads one at a time.\n");
    println!("Instructions:");
    println!("1. Press and release a pad");
    println!("2. Wait to see if LED lights up");
    println!("3. Type 'y' if LED lit up, 'n' if not");
    println!("4. Type 'q' to quit\n");

    let mut results: HashMap<u8, bool> = HashMap::new();
    let mut test_count = 0;

    loop {
        println!("\n─────────────────────────────────────");
        println!("Test #{}: Press any pad (or 'q' to quit)", test_count + 1);

        // Clear previous capture
        {
            let mut cap = capture.lock().unwrap();
            cap.midi_note = None;
            cap.midi_velocity = None;
            cap.hid_buffer.clear();
        }

        // Wait for MIDI input
        std::thread::sleep(Duration::from_millis(100));

        let (note, velocity) = loop {
            {
                let cap = capture.lock().unwrap();
                if let (Some(n), Some(v)) = (cap.midi_note, cap.midi_velocity) {
                    break (n, v);
                }
            }
            std::thread::sleep(Duration::from_millis(50));
        };

        println!("\n✓ Captured: Note {} vel {}", note, velocity);

        // Now test LED with different addresses
        println!("\nTesting LED addresses...");

        let test_addresses = [
            (0x1E, "LED_PAD13 (0x1E) - should be pad index 0"),
            (0x21, "LED_PAD14 (0x21) - should be pad index 1"),
            (0x24, "LED_PAD15 (0x24) - should be pad index 2"),
            (0x27, "LED_PAD16 (0x27) - should be pad index 3"),
            (0x2A, "LED_PAD09 (0x2A) - should be pad index 4"),
            (0x2D, "LED_PAD10 (0x2D) - should be pad index 5"),
            (0x30, "LED_PAD11 (0x30) - should be pad index 6"),
            (0x33, "LED_PAD12 (0x33) - should be pad index 7"),
            (0x36, "LED_PAD05 (0x36) - should be pad index 8"),
            (0x39, "LED_PAD06 (0x39) - should be pad index 9"),
            (0x3C, "LED_PAD07 (0x3C) - should be pad index 10"),
            (0x3F, "LED_PAD08 (0x3F) - should be pad index 11"),
            (0x42, "LED_PAD01 (0x42) - should be pad index 12"),
            (0x45, "LED_PAD02 (0x45) - should be pad index 13"),
            (0x48, "LED_PAD03 (0x48) - should be pad index 14"),
            (0x4B, "LED_PAD04 (0x4B) - should be pad index 15"),
        ];

        for (addr, desc) in &test_addresses {
            // Create LED buffer
            let mut buffer = vec![LED_REPORT_ID];
            buffer.resize(80, 0);

            // Set RGB at this address (bright red)
            buffer[*addr as usize] = 255; // R
            buffer[*addr as usize + 1] = 0; // G
            buffer[*addr as usize + 2] = 0; // B

            // Pad to 65 bytes
            buffer.resize(65, 0);

            hid_device.write(&buffer)?;
            std::thread::sleep(Duration::from_millis(300));

            print!("\n  Testing {} - Did you see RED light? (y/n): ", desc);
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().eq_ignore_ascii_case("y") {
                println!(
                    "  ✓✓✓ FOUND IT! Note {} maps to address 0x{:02X} ({})",
                    note, addr, desc
                );
                results.insert(note, true);

                // Turn off
                let mut buffer = vec![LED_REPORT_ID];
                buffer.resize(65, 0);
                hid_device.write(&buffer)?;

                break;
            } else {
                // Turn off and try next
                let mut buffer = vec![LED_REPORT_ID];
                buffer.resize(65, 0);
                hid_device.write(&buffer)?;
            }
        }

        test_count += 1;

        println!("\n\nResults so far:");
        println!("─────────────────────────────────────");
        for note in results.keys() {
            println!("  Note {}: LED address found", note);
        }

        print!("\nContinue testing? (y/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    println!("\n\n╔════════════════════════════════════════╗");
    println!("║  Diagnostic Complete!                  ║");
    println!("╚════════════════════════════════════════╝");
    println!("\nTested {} pads", test_count);
    println!("\nMapping discovered:");
    for (note, _) in results {
        println!("  MIDI Note {} → LED address (found)", note);
    }

    Ok(())
}
