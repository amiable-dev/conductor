// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use hidapi::{HidApi, HidDevice};
use std::error::Error;
use colored::Colorize;

// Native Instruments Vendor ID
const NI_VENDOR_ID: u16 = 0x17CC;
const MIKRO_MK3_PRODUCT_ID: u16 = 0x1700;
const LED_REPORT_ID: u8 = 0x80;

// MK3 uses indexed colors (not RGB!), based on r00tman's driver
const PAD_LED_OFFSET: usize = 39;  // Pads start at buffer[39]

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PadColor {
    Off = 0,
    Red = 1,
    Orange = 2,
    LightOrange = 3,
    WarmYellow = 4,
    Yellow = 5,
    Lime = 6,
    Green = 7,
    Mint = 8,
    Cyan = 9,
    Turquoise = 10,
    Blue = 11,
    Plum = 12,
    Violet = 13,
    Purple = 14,
    Magenta = 15,
    Fuchsia = 16,
    White = 17,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Brightness {
    Off = 0x00,
    Dim = 0x7c,
    Normal = 0x7e,
    Bright = 0x7f,
}

// RGB struct for API compatibility
#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const OFF: RGB = RGB { r: 0, g: 0, b: 0 };
}

pub struct MikroMK3LEDs {
    device: Option<HidDevice>,
    buffer: [u8; 80],
}

impl MikroMK3LEDs {
    pub fn new() -> Self {
        Self {
            device: None,
            buffer: [0; 80],
        }
    }

    fn encode_pad(color: PadColor, brightness: Brightness) -> u8 {
        if matches!(brightness, Brightness::Off) {
            return 0;
        }
        let c = color as u8;
        let b = brightness as u8;
        (c << 2) | (b & 0b11)
    }
    
    fn velocity_to_color(velocity: u8) -> PadColor {
        match velocity {
            0..=39 => PadColor::Green,
            40..=79 => PadColor::Yellow,
            80..=127 => PadColor::Red,
            _ => PadColor::Green,
        }
    }
    
    fn velocity_to_brightness(velocity: u8) -> Brightness {
        if velocity == 0 {
            Brightness::Off
        } else {
            let factor = 0.4 + (velocity as f32 / 127.0) * 0.6;
            if factor < 0.5 {
                Brightness::Dim
            } else if factor < 0.75 {
                Brightness::Normal
            } else {
                Brightness::Bright
            }
        }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        println!("{}", "Connecting to Mikro MK3 LEDs...".cyan());
        
        let api = HidApi::new()?;
        
        for device_info in api.device_list() {
            if device_info.vendor_id() == NI_VENDOR_ID 
               && device_info.product_id() == MIKRO_MK3_PRODUCT_ID {
                
                let interface_number = device_info.interface_number();
                println!("  Found Mikro MK3 on interface {}", interface_number);
                
                if interface_number == 0 {
                    match device_info.open_device(&api) {
                        Ok(dev) => {
                            self.device = Some(dev);
                            println!("{}", "✓ Connected to Mikro MK3 LED interface".green().bold());
                            return Ok(());
                        }
                        Err(e) => {
                            eprintln!("Failed to open Mikro MK3: {}", e);
                        }
                    }
                }
            }
        }
        
        Err("Mikro MK3 not found or could not be opened".into())
    }

    pub fn set_pad_color(&mut self, pad_index: u8, _color: RGB) -> Result<(), Box<dyn Error>> {
        self.set_pad_indexed(pad_index, PadColor::Off, Brightness::Off)
    }
    
    /// Maps logical pad index (0-15) to physical LED position
    /// The MK3 hardware has pads numbered top-to-bottom, but logical indices are bottom-to-top
    /// This creates a vertical flip: rows are swapped (0↔3, 1↔2)
    fn map_pad_to_led_position(pad_index: u8) -> u8 {
        let row = pad_index / 4;        // 0-3 (bottom to top in logical layout)
        let col = pad_index % 4;        // 0-3 (left to right)
        let flipped_row = 3 - row;      // Flip vertically: 0→3, 1→2, 2→1, 3→0
        flipped_row * 4 + col
    }
    
    pub fn set_pad_indexed(&mut self, pad_index: u8, color: PadColor, brightness: Brightness) -> Result<(), Box<dyn Error>> {
        if pad_index >= 16 {
            return Err(format!("Pad index {} out of range (0-15)", pad_index).into());
        }
        
        // Map logical pad index to physical LED position
        let led_position = Self::map_pad_to_led_position(pad_index);
        let offset = PAD_LED_OFFSET + led_position as usize;
        self.buffer[offset] = Self::encode_pad(color, brightness);
        
        eprintln!("LED UPDATE: Pad {} → LED {} (buffer[{}]) = {:?} @ {:?}", 
                  pad_index, led_position, offset, color, brightness);
        
        self.write_buffer()
    }
    
    pub fn set_pad_velocity(&mut self, pad_index: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        let color = Self::velocity_to_color(velocity);
        let brightness = Self::velocity_to_brightness(velocity);
        self.set_pad_indexed(pad_index, color, brightness)
    }

    pub fn show_velocity_feedback(&mut self, pad_index: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.set_pad_velocity(pad_index, velocity)
    }

    fn write_buffer(&self) -> Result<(), Box<dyn Error>> {
        if self.device.is_none() {
            return Ok(());
        }
        
        let device = self.device.as_ref().unwrap();
        
        let mut data = vec![LED_REPORT_ID];
        data.extend_from_slice(&self.buffer);
        
        eprintln!("LED UPDATE: Writing {} bytes (pads: {:02X?})", 
            data.len(), 
            &self.buffer[PAD_LED_OFFSET..PAD_LED_OFFSET+16]
        );
        
        match device.write(&data) {
            Ok(bytes_written) => {
                eprintln!("LED UPDATE: Successfully wrote {} bytes", bytes_written);
                Ok(())
            },
            Err(e) => {
                eprintln!("LED ERROR: Failed to write: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn set_mode_colors(&mut self, _mode: u8) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    pub fn flash_pad(&mut self, _pad_index: u8) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    pub fn ripple_effect(&mut self, _start_pad: u8) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    pub fn show_long_press_feedback(&mut self, _pad_index: u8) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    pub fn set_pad_colors(&mut self, colors: &[RGB; 16]) -> Result<(), Box<dyn Error>> {
        // Convert RGB to indexed colors (map to nearest available color)
        for (i, _color) in colors.iter().enumerate() {
            // For now, just set all pads to dim white
            self.buffer[PAD_LED_OFFSET + i] = Self::encode_pad(PadColor::White, Brightness::Dim);
        }
        self.write_buffer()
    }
    
    pub fn clear_all(&mut self) -> Result<(), Box<dyn Error>> {
        // Clear all pads
        for i in 0..16 {
            self.buffer[PAD_LED_OFFSET + i] = 0;
        }
        self.write_buffer()
    }
    
    pub fn breathing_effect(&mut self) -> Result<(), Box<dyn Error>> {
        for i in 0..16 {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(PadColor::Blue, Brightness::Dim);
        }
        self.write_buffer()
    }
    
    pub fn pulse_effect(&mut self) -> Result<(), Box<dyn Error>> {
        for i in 0..16 {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(PadColor::Cyan, Brightness::Normal);
        }
        self.write_buffer()
    }
    
    pub fn rainbow_effect(&mut self) -> Result<(), Box<dyn Error>> {
        let colors = [
            PadColor::Red, PadColor::Orange, PadColor::Yellow, PadColor::Green,
            PadColor::Cyan, PadColor::Blue, PadColor::Purple, PadColor::Magenta,
            PadColor::Red, PadColor::Orange, PadColor::Yellow, PadColor::Green,
            PadColor::Cyan, PadColor::Blue, PadColor::Purple, PadColor::Magenta,
        ];
        for (i, &color) in colors.iter().enumerate() {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(color, Brightness::Normal);
        }
        self.write_buffer()
    }
    
    pub fn wave_effect(&mut self) -> Result<(), Box<dyn Error>> {
        let brightnesses = [
            Brightness::Dim, Brightness::Normal, Brightness::Bright, Brightness::Bright,
            Brightness::Bright, Brightness::Bright, Brightness::Normal, Brightness::Dim,
            Brightness::Dim, Brightness::Normal, Brightness::Bright, Brightness::Bright,
            Brightness::Bright, Brightness::Bright, Brightness::Normal, Brightness::Dim,
        ];
        for (i, &brightness) in brightnesses.iter().enumerate() {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(PadColor::Blue, brightness);
        }
        self.write_buffer()
    }
    
    pub fn sparkle_effect(&mut self) -> Result<(), Box<dyn Error>> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for i in 0..16 {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            if rng.gen_bool(0.2) {
                let brightness = if rng.gen_bool(0.5) { Brightness::Normal } else { Brightness::Bright };
                self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(PadColor::White, brightness);
            } else {
                self.buffer[PAD_LED_OFFSET + led_pos] = 0;
            }
        }
        self.write_buffer()
    }
    
    pub fn vumeter_effect(&mut self) -> Result<(), Box<dyn Error>> {
        let colors = [
            PadColor::Green, PadColor::Green, PadColor::Green, PadColor::Green,
            PadColor::Green, PadColor::Green, PadColor::Yellow, PadColor::Yellow,
            PadColor::Yellow, PadColor::Yellow, PadColor::Orange, PadColor::Orange,
            PadColor::Red, PadColor::Red, PadColor::Red, PadColor::Red,
        ];
        for (i, &color) in colors.iter().enumerate() {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(color, Brightness::Dim);
        }
        self.write_buffer()
    }
    
    pub fn spiral_effect(&mut self) -> Result<(), Box<dyn Error>> {
        let pattern = [
            (PadColor::Purple, Brightness::Bright),
            (PadColor::Purple, Brightness::Normal),
            (PadColor::Purple, Brightness::Normal),
            (PadColor::Purple, Brightness::Dim),
            (PadColor::Magenta, Brightness::Dim),
            (PadColor::Off, Brightness::Off),
            (PadColor::Off, Brightness::Off),
            (PadColor::Blue, Brightness::Dim),
            (PadColor::Magenta, Brightness::Normal),
            (PadColor::Off, Brightness::Off),
            (PadColor::Off, Brightness::Off),
            (PadColor::Blue, Brightness::Normal),
            (PadColor::Magenta, Brightness::Normal),
            (PadColor::Cyan, Brightness::Normal),
            (PadColor::Cyan, Brightness::Normal),
            (PadColor::Blue, Brightness::Normal),
        ];
        for (i, &(color, brightness)) in pattern.iter().enumerate() {
            let led_pos = Self::map_pad_to_led_position(i as u8) as usize;
            self.buffer[PAD_LED_OFFSET + led_pos] = Self::encode_pad(color, brightness);
        }
        self.write_buffer()
    }
}
