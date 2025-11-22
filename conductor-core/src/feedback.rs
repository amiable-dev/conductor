// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::midi_feedback::MidiFeedback;
use crate::mikro_leds::{MikroMK3LEDs, RGB};
use std::error::Error;
use tracing::error;

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
    fn show_long_press_feedback(&mut self, pad: u8, elapsed_ms: u128)
    -> Result<(), Box<dyn Error>>;
    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>>;
}

/// LED lighting schemes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightingScheme {
    Off,
    Static(u8), // Static color based on mode
    Breathing,  // Slow breathing effect
    Pulse,      // Fast pulse effect
    Rainbow,    // Rainbow cycle
    Wave,       // Wave pattern
    Sparkle,    // Random sparkles
    Reactive,   // React to MIDI events only
    VuMeter,    // VU meter style (bottom-up)
    Spiral,     // Spiral pattern
}

impl LightingScheme {
    pub fn parse(s: &str) -> Option<Self> {
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
            "off",
            "static",
            "breathing",
            "pulse",
            "rainbow",
            "wave",
            "sparkle",
            "reactive",
            "vumeter",
            "spiral",
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

    fn show_long_press_feedback(
        &mut self,
        pad: u8,
        _elapsed_ms: u128,
    ) -> Result<(), Box<dyn Error>> {
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

    fn show_long_press_feedback(
        &mut self,
        pad: u8,
        _elapsed_ms: u128,
    ) -> Result<(), Box<dyn Error>> {
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
pub fn create_feedback_device(
    device_name: &str,
    midi_port: Option<usize>,
    enable_hid: bool,
) -> Box<dyn PadFeedback> {
    // Use HID for Mikro MK3 only if explicitly enabled
    if enable_hid
        && device_name.to_lowercase().contains("maschine")
        && device_name.to_lowercase().contains("mikro")
    {
        Box::new(MikroMK3LEDs::new())
    } else if let Some(port) = midi_port {
        let mut midi_fb = MidiFeedback::new();
        if let Err(e) = midi_fb.connect(port) {
            error!("Failed to connect MIDI feedback: {}", e);
        }
        Box::new(midi_fb)
    } else {
        // Fallback to MIDI feedback without connection
        Box::new(MidiFeedback::new())
    }
}

use std::collections::HashMap;
use std::time::Instant;

/// Manager for LED feedback with reactive state tracking
///
/// FeedbackManager wraps a PadFeedback device and provides higher-level
/// feedback management including:
/// - Reactive pad press/release with fade-out tracking
/// - Mode changes with color updates
/// - Lighting scheme switching
/// - Active pad state management
pub struct FeedbackManager {
    device: Box<dyn PadFeedback>,
    current_scheme: LightingScheme,
    reactive_state: HashMap<u8, (Instant, u8)>, // (pad -> (press_time, velocity))
    current_mode: u8,
}

impl FeedbackManager {
    /// Create a new FeedbackManager with the given device
    ///
    /// Defaults to Reactive lighting scheme and mode 0.
    pub fn new(device: Box<dyn PadFeedback>) -> Self {
        Self {
            device,
            current_scheme: LightingScheme::Reactive,
            reactive_state: HashMap::new(),
            current_mode: 0,
        }
    }

    /// Handle a pad press event
    ///
    /// In reactive mode, this records the pad press time and velocity,
    /// and triggers velocity-based LED feedback.
    pub fn on_pad_press(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        // Record reactive state
        self.reactive_state.insert(pad, (Instant::now(), velocity));

        // Show velocity feedback if in reactive mode
        if self.current_scheme == LightingScheme::Reactive {
            self.device.show_velocity_feedback(pad, velocity)?;
        }

        Ok(())
    }

    /// Handle a pad release event
    ///
    /// In reactive mode, the pad remains tracked for fade-out.
    /// It will be removed after the fade period (1 second) during update().
    pub fn on_pad_release(&mut self, _pad: u8) -> Result<(), Box<dyn Error>> {
        // In reactive mode, we keep the pad in state for fade-out
        // It will be removed during update() after 1 second
        Ok(())
    }

    /// Handle a mode change event
    ///
    /// Updates the current mode and applies mode-specific colors to the device.
    pub fn on_mode_change(&mut self, mode: u8, _color: RGB) -> Result<(), Box<dyn Error>> {
        self.current_mode = mode;
        self.device.set_mode_colors(mode)?;
        Ok(())
    }

    /// Set the lighting scheme
    ///
    /// Clears reactive state when switching away from Reactive mode.
    /// Applies the new scheme to the device immediately.
    pub fn set_scheme(&mut self, scheme: LightingScheme) -> Result<(), Box<dyn Error>> {
        // Clear reactive state when switching schemes
        if scheme != LightingScheme::Reactive {
            self.reactive_state.clear();
        }

        self.current_scheme = scheme;
        self.device.run_scheme(&scheme)?;
        Ok(())
    }

    /// Update reactive fade-out state
    ///
    /// Should be called periodically (e.g., in your event loop).
    /// Returns a list of pads that have completed their fade-out period (1 second).
    ///
    /// # Behavior
    /// - For non-Reactive schemes: Returns empty vec
    /// - For Reactive scheme: Removes pads that have been pressed for >1 second
    pub fn update(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Only process updates in reactive mode
        if self.current_scheme != LightingScheme::Reactive {
            return Ok(Vec::new());
        }

        let now = Instant::now();
        let fade_duration = std::time::Duration::from_secs(1);

        // Find pads that have completed fade-out
        let completed: Vec<u8> = self
            .reactive_state
            .iter()
            .filter_map(|(pad, (press_time, _velocity))| {
                if now.duration_since(*press_time) >= fade_duration {
                    Some(*pad)
                } else {
                    None
                }
            })
            .collect();

        // Remove completed pads from state
        for pad in &completed {
            self.reactive_state.remove(pad);
        }

        Ok(completed)
    }

    /// Clear all LED state
    ///
    /// Clears reactive state and sends clear command to device.
    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        self.reactive_state.clear();
        self.device.clear_all()?;
        Ok(())
    }

    /// Get the current lighting scheme
    pub fn current_scheme(&self) -> LightingScheme {
        self.current_scheme
    }

    /// Get the current mode
    pub fn current_mode(&self) -> u8 {
        self.current_mode
    }

    /// Get the number of active pads (in reactive mode)
    pub fn active_pads(&self) -> usize {
        self.reactive_state.len()
    }

    /// Get immutable access to the underlying device
    pub fn device(&self) -> &dyn PadFeedback {
        &*self.device
    }

    /// Get mutable access to the underlying device
    pub fn device_mut(&mut self) -> &mut dyn PadFeedback {
        &mut *self.device
    }
}
