// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MIKRO_MK3_VENDOR_ID: u16 = 0x17cc;
const MIKRO_MK3_PRODUCT_ID: u16 = 0x1700;

#[derive(Debug, Clone)]
struct PadData {
    physical_pad: u8,
    midi_note: u8,
    hid_pad_index: Option<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  Mikro MK3 Pad Mapping Tool                               ║");
    println!("║  This will capture MIDI note + HID pad index for mapping  ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();

    // Open HID device for reading pad events (with shared access)
    let api = hidapi::HidApi::new()?;
    let mut hid_device = None;

    println!("Searching for Mikro MK3 HID interfaces...");
    for device in api.device_list() {
        if device.vendor_id() == MIKRO_MK3_VENDOR_ID && device.product_id() == MIKRO_MK3_PRODUCT_ID
        {
            println!(
                "  Found interface {}: {}",
                device.interface_number(),
                device.product_string().unwrap_or("Unknown")
            );
            if device.interface_number() == 0 {
                match device.open_device(&api) {
                    Ok(dev) => {
                        println!("  ✓ Opened with shared access");
                        hid_device = Some(dev);
                        break;
                    }
                    Err(e) => {
                        eprintln!("  ✗ Could not open device: {}", e);
                    }
                }
            }
        }
    }

    let hid_device = Arc::new(Mutex::new(
        hid_device.ok_or("Mikro MK3 HID not found or could not open")?,
    ));
    println!("✓ Found Mikro MK3 HID interface");

    // Shared data for captured events
    let captured_midi = Arc::new(Mutex::new(None::<(u8, u8)>)); // (note, velocity)
    let captured_hid = Arc::new(Mutex::new(None::<u8>)); // pad_index

    // Open MIDI input
    let mut midi_in = MidiInput::new("Pad Mapper MIDI")?;
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

    println!("✓ Found Mikro MK3 MIDI input");
    println!();

    // Start MIDI capture thread
    let captured_midi_clone = Arc::clone(&captured_midi);
    let _midi_conn = midi_in.connect(
        mikro_port,
        "pad-mapper",
        move |_stamp, message, _| {
            if message.len() >= 3 && (message[0] & 0xF0) == 0x90 {
                let note = message[1];
                let velocity = message[2];
                if velocity > 0 {
                    *captured_midi_clone.lock().unwrap() = Some((note, velocity));
                }
            }
        },
        (),
    )?;

    // Start HID reading thread
    let captured_hid_clone = Arc::clone(&captured_hid);
    let hid_device_clone = Arc::clone(&hid_device);
    thread::spawn(move || {
        let mut buffer = [0u8; 256];
        loop {
            if let Ok(device) = hid_device_clone.lock()
                && let Ok(size) = device.read_timeout(&mut buffer, 10)
                && size > 0
            {
                eprintln!(
                    "HID Report: ID=0x{:02X}, size={}, data={:02X?}",
                    buffer[0],
                    size,
                    &buffer[0..size.min(16)]
                );

                // MK3 might use different report format - check all reports
                if buffer[0] == 0x20 || buffer[0] == 0x01 {
                    // Try to extract pad index from report
                    for idx in (0..32).step_by(2) {
                        if idx + 1 < buffer.len() {
                            let high_byte = buffer[idx + 1];
                            let pad_idx = (high_byte & 0xF0) >> 4;
                            let low_byte = buffer[idx];
                            let value = (((high_byte & 0x0F) as u16) << 8) | low_byte as u16;

                            if value > 512 {
                                eprintln!(
                                    "  -> Detected pad index: 0x{:02X} (value={})",
                                    pad_idx, value
                                );
                                *captured_hid_clone.lock().unwrap() = Some(pad_idx);
                                break;
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("Ready to capture pad data!");
    println!();
    println!("Instructions:");
    println!("  - Set your device to Pad Page A");
    println!("  - Press each pad when prompted");
    println!("  - I'll capture MIDI note + HID pad index");
    println!();

    let pads_to_test = vec![
        (1, "Bottom-left corner"),
        (2, "Bottom row, 2nd from left"),
        (3, "Bottom row, 3rd from left"),
        (4, "Bottom row, right corner"),
        (5, "2nd row from bottom, left"),
        (6, "2nd row from bottom, 2nd"),
        (7, "2nd row from bottom, 3rd"),
        (8, "2nd row from bottom, right"),
        (9, "2nd row from top, left"),
        (10, "2nd row from top, 2nd"),
        (11, "2nd row from top, 3rd"),
        (12, "2nd row from top, right"),
        (13, "Top row, left corner"),
        (14, "Top row, 2nd from left"),
        (15, "Top row, 3rd from left"),
        (16, "Top row, right corner"),
    ];

    let mut results = Vec::new();

    for (pad_num, description) in pads_to_test {
        println!("─────────────────────────────────────────────");
        println!("Press Pad {} ({})", pad_num, description);
        print!("Waiting... ");
        io::stdout().flush()?;

        // Clear previous captures
        *captured_midi.lock().unwrap() = None;
        *captured_hid.lock().unwrap() = None;

        // Wait for both MIDI and HID data
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > Duration::from_secs(30) {
                println!("⏱ Timeout - skipping this pad");
                break;
            }

            let midi_data = *captured_midi.lock().unwrap();
            let hid_data = *captured_hid.lock().unwrap();

            if let (Some((note, vel)), Some(hid_idx)) = (midi_data, hid_data) {
                println!("✓");
                println!("  MIDI: Note {} velocity {}", note, vel);
                println!("  HID:  Pad index 0x{:02X} ({})", hid_idx, hid_idx);

                results.push(PadData {
                    physical_pad: pad_num,
                    midi_note: note,
                    hid_pad_index: Some(hid_idx),
                });

                thread::sleep(Duration::from_millis(500));
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }
    }

    println!();
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  RESULTS: Pad Mapping                                     ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!();
    println!("Physical Pad | MIDI Note | HID Pad Index");
    println!("-------------|-----------|---------------");
    for data in &results {
        if let Some(hid_idx) = data.hid_pad_index {
            println!(
                "    {:2}       |    {:3}    |   0x{:02X} ({:2})",
                data.physical_pad, data.midi_note, hid_idx, hid_idx
            );
        }
    }

    println!();
    println!("Now we can determine the LED address mapping!");
    println!("The HID pad index should map to specific LED addresses.");

    Ok(())
}
