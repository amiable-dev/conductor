// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Gamepad event mapping to InputEvent abstraction
//!
//! This module maps gamepad/HID input events (via gilrs) to the protocol-agnostic
//! InputEvent abstraction. It uses button IDs in the 128-255 range to avoid conflicts
//! with MIDI note numbers (0-127).
//!
//! # Button ID Mapping
//!
//! - **Face buttons** (A/B/X/Y): 128-131
//! - **D-Pad**: 132-135 (Up/Down/Left/Right)
//! - **Shoulder buttons** (L1/R1): 136-137
//! - **Stick clicks** (L3/R3): 138-139
//! - **Menu buttons** (Start/Select/Guide): 140-142
//! - **Left stick axes**: Encoder 128 (X), 129 (Y)
//! - **Right stick axes**: Encoder 130 (X), 131 (Y)
//! - **Triggers** (L2/R2): Encoder 132-133 (analog), or Pad 143-144 (digital)
//!
//! # Design Principles
//!
//! - **Non-overlapping IDs**: Gamepad buttons use 128-255 to avoid MIDI 0-127 range
//! - **Analog stick normalization**: -1.0 to 1.0 → 0-127 (MIDI-compatible range)
//! - **Pressure sensitivity**: Analog triggers mapped to velocity/pressure values
//! - **Standard mapping**: Follows SDL2 GameController mappings (gilrs default)

use crate::events::InputEvent;
use std::time::Instant;

/// Gamepad button ID ranges (128-255, non-overlapping with MIDI 0-127)
pub mod button_ids {
    // Face buttons (128-131)
    pub const SOUTH: u8 = 128; // A (Xbox), Cross (PS), B (Nintendo)
    pub const EAST: u8 = 129; // B (Xbox), Circle (PS), A (Nintendo)
    pub const WEST: u8 = 130; // X (Xbox), Square (PS), Y (Nintendo)
    pub const NORTH: u8 = 131; // Y (Xbox), Triangle (PS), X (Nintendo)

    // D-Pad (132-135)
    pub const DPAD_UP: u8 = 132;
    pub const DPAD_DOWN: u8 = 133;
    pub const DPAD_LEFT: u8 = 134;
    pub const DPAD_RIGHT: u8 = 135;

    // Shoulder buttons (136-137)
    pub const LEFT_SHOULDER: u8 = 136; // L1, LB
    pub const RIGHT_SHOULDER: u8 = 137; // R1, RB

    // Stick clicks (138-139)
    pub const LEFT_THUMB: u8 = 138; // L3
    pub const RIGHT_THUMB: u8 = 139; // R3

    // Menu buttons (140-142)
    pub const START: u8 = 140; // Start, Options, +
    pub const SELECT: u8 = 141; // Back, Share, -
    pub const GUIDE: u8 = 142; // Xbox, PS, Home

    // Trigger digital fallback (143-144)
    pub const LEFT_TRIGGER: u8 = 143; // L2, LT (digital threshold)
    pub const RIGHT_TRIGGER: u8 = 144; // R2, RT (digital threshold)
}

/// Encoder/axis ID ranges for analog inputs
pub mod encoder_ids {
    // Analog stick axes (128-131)
    pub const LEFT_STICK_X: u8 = 128;
    pub const LEFT_STICK_Y: u8 = 129;
    pub const RIGHT_STICK_X: u8 = 130;
    pub const RIGHT_STICK_Y: u8 = 131;

    // Trigger axes (132-133)
    pub const LEFT_TRIGGER: u8 = 132; // L2, LT analog value
    pub const RIGHT_TRIGGER: u8 = 133; // R2, RT analog value
}

/// Convert gilrs button to MIDIMon button ID
///
/// Maps gilrs::Button to the 128-255 range for non-overlapping MIDI compatibility.
///
/// # Examples
///
/// ```rust,ignore
/// use gilrs::Button;
/// use conductor_core::gamepad_events::button_to_id;
///
/// let id = button_to_id(Button::South);
/// assert_eq!(id, 128); // SOUTH (A/Cross/B)
/// ```
pub fn button_to_id(button: gilrs::Button) -> u8 {
    use gilrs::Button::*;
    match button {
        South => button_ids::SOUTH,
        East => button_ids::EAST,
        West => button_ids::WEST,
        North => button_ids::NORTH,
        DPadUp => button_ids::DPAD_UP,
        DPadDown => button_ids::DPAD_DOWN,
        DPadLeft => button_ids::DPAD_LEFT,
        DPadRight => button_ids::DPAD_RIGHT,
        LeftTrigger => button_ids::LEFT_SHOULDER, // L1/LB
        RightTrigger => button_ids::RIGHT_SHOULDER, // R1/RB
        LeftTrigger2 => button_ids::LEFT_TRIGGER, // L2/LT digital
        RightTrigger2 => button_ids::RIGHT_TRIGGER, // R2/RT digital
        LeftThumb => button_ids::LEFT_THUMB, // L3
        RightThumb => button_ids::RIGHT_THUMB, // R3
        Start => button_ids::START,
        Select => button_ids::SELECT,
        Mode => button_ids::GUIDE,
        _ => 255, // Unknown buttons map to max ID
    }
}

/// Convert gilrs axis to MIDIMon encoder ID
///
/// Maps gilrs::Axis to encoder IDs for analog stick and trigger inputs.
///
/// # Examples
///
/// ```rust,ignore
/// use gilrs::Axis;
/// use conductor_core::gamepad_events::axis_to_encoder_id;
///
/// let id = axis_to_encoder_id(Axis::LeftStickX);
/// assert_eq!(id, 128); // LEFT_STICK_X
/// ```
pub fn axis_to_encoder_id(axis: gilrs::Axis) -> u8 {
    use gilrs::Axis::*;
    match axis {
        LeftStickX => encoder_ids::LEFT_STICK_X,
        LeftStickY => encoder_ids::LEFT_STICK_Y,
        RightStickX => encoder_ids::RIGHT_STICK_X,
        RightStickY => encoder_ids::RIGHT_STICK_Y,
        LeftZ => encoder_ids::LEFT_TRIGGER, // L2/LT analog
        RightZ => encoder_ids::RIGHT_TRIGGER, // R2/RT analog
        _ => 255, // Unknown axes
    }
}

/// Normalize analog axis value to MIDI-compatible range
///
/// Converts gilrs axis values (-1.0 to 1.0) to MIDI-compatible 0-127 range.
/// Applies a small deadzone (0.1) to reduce drift noise.
///
/// # Arguments
///
/// * `value` - Raw axis value from gilrs (-1.0 to 1.0)
///
/// # Returns
///
/// MIDI-compatible value (0-127), with 64 as center
///
/// # Examples
///
/// ```rust
/// use conductor_core::gamepad_events::normalize_axis;
///
/// assert_eq!(normalize_axis(0.0), 64); // Center
/// assert_eq!(normalize_axis(1.0), 127); // Max right/up
/// assert_eq!(normalize_axis(-1.0), 0); // Max left/down
/// assert_eq!(normalize_axis(0.05), 64); // Deadzone (< 0.1)
/// ```
pub fn normalize_axis(value: f32) -> u8 {
    const DEADZONE: f32 = 0.1;

    // Apply deadzone - return center (64) if within deadzone
    if value.abs() < DEADZONE {
        return 64;
    }

    // Map -1.0..1.0 → 0..127, with 64 as center
    let normalized = ((value + 1.0) * 63.5).round() as i32;
    normalized.clamp(0, 127) as u8
}

/// Convert gilrs ButtonPressed event to InputEvent
///
/// Maps gamepad button press to PadPressed with velocity 100 (default press strength).
/// Future enhancement: pressure-sensitive buttons could vary velocity.
///
/// # Arguments
///
/// * `button` - gilrs Button that was pressed
///
/// # Returns
///
/// InputEvent::PadPressed with button ID in 128-255 range
pub fn button_pressed_to_input(button: gilrs::Button) -> InputEvent {
    InputEvent::PadPressed {
        pad: button_to_id(button),
        velocity: 100, // Default velocity for digital buttons
        time: Instant::now(),
    }
}

/// Convert gilrs ButtonReleased event to InputEvent
///
/// Maps gamepad button release to PadReleased.
///
/// # Arguments
///
/// * `button` - gilrs Button that was released
///
/// # Returns
///
/// InputEvent::PadReleased with button ID in 128-255 range
pub fn button_released_to_input(button: gilrs::Button) -> InputEvent {
    InputEvent::PadReleased {
        pad: button_to_id(button),
        time: Instant::now(),
    }
}

/// Convert gilrs AxisChanged event to InputEvent
///
/// Maps analog stick and trigger movements to EncoderTurned events.
/// Normalizes -1.0..1.0 range to MIDI-compatible 0-127.
///
/// # Arguments
///
/// * `axis` - gilrs Axis that changed
/// * `value` - Raw axis value (-1.0 to 1.0)
///
/// # Returns
///
/// InputEvent::EncoderTurned with normalized value (0-127)
pub fn axis_changed_to_input(axis: gilrs::Axis, value: f32) -> InputEvent {
    InputEvent::EncoderTurned {
        encoder: axis_to_encoder_id(axis),
        value: normalize_axis(value),
        time: Instant::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_id_ranges() {
        // Face buttons
        assert_eq!(button_ids::SOUTH, 128);
        assert_eq!(button_ids::NORTH, 131);

        // D-Pad
        assert_eq!(button_ids::DPAD_UP, 132);
        assert_eq!(button_ids::DPAD_RIGHT, 135);

        // Shoulders
        assert_eq!(button_ids::LEFT_SHOULDER, 136);
        assert_eq!(button_ids::RIGHT_SHOULDER, 137);

        // Stick clicks
        assert_eq!(button_ids::LEFT_THUMB, 138);
        assert_eq!(button_ids::RIGHT_THUMB, 139);

        // Menu
        assert_eq!(button_ids::START, 140);
        assert_eq!(button_ids::GUIDE, 142);

        // Triggers
        assert_eq!(button_ids::LEFT_TRIGGER, 143);
        assert_eq!(button_ids::RIGHT_TRIGGER, 144);
    }

    #[test]
    fn test_encoder_id_ranges() {
        // Stick axes
        assert_eq!(encoder_ids::LEFT_STICK_X, 128);
        assert_eq!(encoder_ids::RIGHT_STICK_Y, 131);

        // Trigger axes
        assert_eq!(encoder_ids::LEFT_TRIGGER, 132);
        assert_eq!(encoder_ids::RIGHT_TRIGGER, 133);
    }

    #[test]
    fn test_button_to_id() {
        use gilrs::Button::*;

        assert_eq!(button_to_id(South), 128);
        assert_eq!(button_to_id(East), 129);
        assert_eq!(button_to_id(DPadUp), 132);
        assert_eq!(button_to_id(LeftTrigger), 136); // L1/LB
        assert_eq!(button_to_id(LeftThumb), 138); // L3
        assert_eq!(button_to_id(Start), 140);
    }

    #[test]
    fn test_axis_to_encoder_id() {
        use gilrs::Axis::*;

        assert_eq!(axis_to_encoder_id(LeftStickX), 128);
        assert_eq!(axis_to_encoder_id(LeftStickY), 129);
        assert_eq!(axis_to_encoder_id(RightStickX), 130);
        assert_eq!(axis_to_encoder_id(RightStickY), 131);
        assert_eq!(axis_to_encoder_id(LeftZ), 132); // L2 analog
        assert_eq!(axis_to_encoder_id(RightZ), 133); // R2 analog
    }

    #[test]
    fn test_normalize_axis() {
        // Center position
        assert_eq!(normalize_axis(0.0), 64);

        // Max positions
        assert_eq!(normalize_axis(1.0), 127);
        assert_eq!(normalize_axis(-1.0), 0);

        // Half positions
        assert_eq!(normalize_axis(0.5), 95);
        assert_eq!(normalize_axis(-0.5), 32);

        // Deadzone (< 0.1)
        assert_eq!(normalize_axis(0.05), 64);
        assert_eq!(normalize_axis(-0.08), 64);

        // Just outside deadzone
        let outside = normalize_axis(0.11);
        assert!(outside > 64);
    }

    #[test]
    fn test_button_pressed_event() {
        use gilrs::Button;

        let event = button_pressed_to_input(Button::South);

        match event {
            InputEvent::PadPressed { pad, velocity, .. } => {
                assert_eq!(pad, 128); // SOUTH
                assert_eq!(velocity, 100); // Default velocity
            }
            _ => panic!("Expected PadPressed"),
        }
    }

    #[test]
    fn test_button_released_event() {
        use gilrs::Button;

        let event = button_released_to_input(Button::East);

        match event {
            InputEvent::PadReleased { pad, .. } => {
                assert_eq!(pad, 129); // EAST
            }
            _ => panic!("Expected PadReleased"),
        }
    }

    #[test]
    fn test_axis_changed_event() {
        use gilrs::Axis;

        let event = axis_changed_to_input(Axis::LeftStickX, 0.5);

        match event {
            InputEvent::EncoderTurned { encoder, value, .. } => {
                assert_eq!(encoder, 128); // LEFT_STICK_X
                assert_eq!(value, 95); // Normalized 0.5
            }
            _ => panic!("Expected EncoderTurned"),
        }
    }

    #[test]
    fn test_non_overlapping_with_midi() {
        // Ensure all gamepad button IDs are >= 128 (outside MIDI 0-127 range)
        use gilrs::Button::*;

        let buttons = vec![
            South, East, West, North, DPadUp, DPadDown, DPadLeft, DPadRight, LeftTrigger,
            RightTrigger, LeftTrigger2, RightTrigger2, LeftThumb, RightThumb, Start, Select, Mode,
        ];

        for button in buttons {
            let id = button_to_id(button);
            assert!(
                id >= 128,
                "Button {:?} has ID {} which overlaps with MIDI range (0-127)",
                button,
                id
            );
        }
    }
}
