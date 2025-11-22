// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use std::error::Error;
use crate::mikro_leds::{MikroMK3LEDs, RGB};
use crate::midi_feedback::MidiFeedback;

/// Unified trait for device feedback (LEDs, visual indicators)
pub trait PadFeedback: Send {
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>>;
    fn set_mode_colors(&mut self, mode: u8) -> Result<(), Box<dyn Error>>;
    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>>;
    fn flash_pad(&mut self, pad: u8, color: RGB, duration_ms: u64) -> Result<(), Box<dyn Error>>;
    fn ripple_effect(&mut self, start_pad: u8, color: RGB) -> Result<(), Box<dyn Error>>;
    fn clear_all(&mut self) -> Result<(), Box<dyn Error>>;
    fn show_long_press_feedback(&mut self, pad: u8, elapsed_ms: u128) -> Result<(), Box<dyn Error>>;
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>>;
}

/// LED lighting schemes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightingScheme {
    Off,
    Static(u8),           // Static color based on mode
    Breathing,            // Slow breathing effect
    Pulse,                // Fast pulse effect
    Rainbow,              // Rainbow cycle
    Wave,                 // Wave pattern
    Sparkle,              // Random sparkles
    Reactive,             // React to MIDI events only
    VuMeter,              // VU meter style (bottom-up)
    Spiral,               // Spiral pattern
}

impl LightingScheme {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(Self::Off),
            "static" => Some(Self::Static(0)),
            "breathing" => Some(Self::Breathing),
            "pulse" => Some(Self::Pulse),
            "rainbow" => Some(Self::Rainbow),
            "wave" => Some(Self::Wave),
            "sparkle" => Some(Self::Sparkle),
            "reactive" => Some(Self::Reactive),
            "vumeter" => Some(Self::VuMeter),
            "spiral" => Some(Self::Spiral),
            _ => None,
        }
    }

    pub fn list_all() -> Vec<&'static str> {
        vec![
            "off", "static", "breathing", "pulse", "rainbow", 
            "wave", "sparkle", "reactive", "vumeter", "spiral"
        ]
    }
}

/// Implementation for Mikro MK3 (HID)
impl PadFeedback for MikroMK3LEDs {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.connect()
    }

    fn set_pad_color(&mut self, pad: u8, color: RGB) -> Result<(), Box<dyn Error>> {
        self.set_pad_color(pad, color)
    }

    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.set_pad_velocity(pad, velocity)
    }

    fn set_mode_colors(&mut self, mode: u8) -> Result<(), Box<dyn Error>> {
        self.set_mode_colors(mode)
    }

    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.show_velocity_feedback(pad, velocity)
    }

    fn flash_pad(&mut self, pad: u8, _color: RGB, _duration_ms: u64) -> Result<(), Box<dyn Error>> {
        self.flash_pad(pad)
    }

    fn ripple_effect(&mut self, start_pad: u8, _color: RGB) -> Result<(), Box<dyn Error>> {
        self.ripple_effect(start_pad)
    }

    fn clear_all(&mut self) -> Result<(), Box<dyn Error>> {
        self.clear_all()
    }

    fn show_long_press_feedback(&mut self, pad: u8, _elapsed_ms: u128) -> Result<(), Box<dyn Error>> {
        self.show_long_press_feedback(pad)
    }

    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>> {
        match scheme {
            LightingScheme::Off => self.clear_all(),
            LightingScheme::Static(mode) => self.set_mode_colors(*mode),
            LightingScheme::Breathing => self.breathing_effect(),
            LightingScheme::Pulse => self.pulse_effect(),
            LightingScheme::Rainbow => self.rainbow_effect(),
            LightingScheme::Wave => self.wave_effect(),
            LightingScheme::Sparkle => self.sparkle_effect(),
            LightingScheme::Reactive => Ok(()), // Reactive only responds to events
            LightingScheme::VuMeter => self.vumeter_effect(),
            LightingScheme::Spiral => self.spiral_effect(),
        }
    }
}

/// Implementation for standard MIDI devices
impl PadFeedback for MidiFeedback {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        // MIDI feedback doesn't need explicit connection setup
        Ok(())
    }

    fn set_pad_color(&mut self, pad: u8, _color: RGB) -> Result<(), Box<dyn Error>> {
        // Send note on with velocity mapped to brightness
        self.send_note_on(pad + 36, 64, 1)?; // C1 = 36
        Ok(())
    }

    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.send_note_on(pad + 36, velocity, 1)?;
        Ok(())
    }

    fn set_mode_colors(&mut self, _mode: u8) -> Result<(), Box<dyn Error>> {
        // MIDI devices typically don't support multi-color LEDs
        Ok(())
    }

    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.send_note_on(pad + 36, velocity, 1)?;
        Ok(())
    }

    fn flash_pad(&mut self, pad: u8, _color: RGB, duration_ms: u64) -> Result<(), Box<dyn Error>> {
        self.flash_pad(pad + 36, 127, duration_ms);
        Ok(())
    }

    fn ripple_effect(&mut self, _start_pad: u8, _color: RGB) -> Result<(), Box<dyn Error>> {
        // Not supported for standard MIDI devices
        Ok(())
    }

    fn clear_all(&mut self) -> Result<(), Box<dyn Error>> {
        // Send note off for all pads
        for pad in 0..16 {
            self.send_note_off(pad + 36, 1)?;
        }
        Ok(())
    }

    fn show_long_press_feedback(&mut self, pad: u8, _elapsed_ms: u128) -> Result<(), Box<dyn Error>> {
        self.send_note_on(pad + 36, 127, 1)?;
        Ok(())
    }

    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>> {
        match scheme {
            LightingScheme::Off => self.clear_all(),
            LightingScheme::Reactive => Ok(()), // Reactive only responds to events
            _ => {
                // Other schemes not supported for basic MIDI devices
                Ok(())
            }
        }
    }
}

/// Factory function to create the appropriate feedback device
pub fn create_feedback_device(device_name: &str, midi_port: Option<usize>, enable_hid: bool) -> Box<dyn PadFeedback> {
    // Use HID for Mikro MK3 only if explicitly enabled
    if enable_hid && device_name.to_lowercase().contains("maschine") && device_name.to_lowercase().contains("mikro") {
        Box::new(MikroMK3LEDs::new())
    } else if let Some(port) = midi_port {
        let mut midi_fb = MidiFeedback::new();
        if let Err(e) = midi_fb.connect(port) {
            eprintln!("Failed to connect MIDI feedback: {}", e);
        }
        Box::new(midi_fb)
    } else {
        // Fallback to MIDI feedback without connection
        Box::new(MidiFeedback::new())
    }
}
