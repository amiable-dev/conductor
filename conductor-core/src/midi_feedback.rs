// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use tracing::info;

pub struct MidiFeedback {
    connection: Option<MidiOutputConnection>,
}

impl MidiFeedback {
    pub fn new() -> Self {
        Self { connection: None }
    }

    pub fn connect(&mut self, port_index: usize) -> Result<(), Box<dyn Error>> {
        let midi_out = MidiOutput::new("MidiMacroPad Feedback")?;
        let ports = midi_out.ports();

        if port_index >= ports.len() {
            return Err("Invalid output port index".into());
        }

        let port = &ports[port_index];
        let port_name = midi_out.port_name(port)?;
        info!("MIDI feedback connected to: {}", port_name);

        let conn = midi_out.connect(port, "feedback")?;
        self.connection = Some(conn);

        Ok(())
    }

    pub fn send_note_on(
        &mut self,
        note: u8,
        velocity: u8,
        channel: u8,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut conn) = self.connection {
            let msg = [0x90 | (channel - 1), note, velocity];
            conn.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_note_off(&mut self, note: u8, channel: u8) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut conn) = self.connection {
            let msg = [0x80 | (channel - 1), note, 0];
            conn.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_cc(&mut self, cc: u8, value: u8, channel: u8) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut conn) = self.connection {
            let msg = [0xB0 | (channel - 1), cc, value];
            conn.send(&msg)?;
        }
        Ok(())
    }

    pub fn send_sysex(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut conn) = self.connection {
            let mut msg = vec![0xF0]; // SysEx start
            msg.extend_from_slice(data);
            msg.push(0xF7); // SysEx end
            conn.send(&msg)?;
        }
        Ok(())
    }

    // Flash a pad LED (if device supports MIDI note LED feedback)
    pub fn flash_pad(&mut self, note: u8, on_velocity: u8, duration_ms: u64) {
        let _ = self.send_note_on(note, on_velocity, 1);
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(duration_ms));
        });
        // Note: You'd want to send note_off after the delay in a real implementation
    }
}
