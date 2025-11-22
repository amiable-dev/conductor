// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Tests for the FeedbackManager wrapper struct

use conductor_core::{FeedbackManager, LightingScheme, PadFeedback};
use std::error::Error;
use std::sync::{Arc, Mutex};

/// Mock feedback device for testing
#[derive(Clone)]
struct MockFeedback {
    state: Arc<Mutex<MockState>>,
}

#[derive(Debug)]
struct MockState {
    connected: bool,
    pad_colors: Vec<(u8, u8, u8, u8)>, // (pad, r, g, b)
    pad_velocities: Vec<(u8, u8)>,     // (pad, velocity)
    mode: u8,
    scheme: String,
    cleared: bool,
    long_press_pads: Vec<u8>,
}

impl MockFeedback {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MockState {
                connected: false,
                pad_colors: Vec::new(),
                pad_velocities: Vec::new(),
                mode: 0,
                scheme: String::new(),
                cleared: false,
                long_press_pads: Vec::new(),
            })),
        }
    }
}

impl PadFeedback for MockFeedback {
    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().connected = true;
        Ok(())
    }

    fn set_pad_color(
        &mut self,
        pad: u8,
        color: conductor_core::mikro_leds::RGB,
    ) -> Result<(), Box<dyn Error>> {
        self.state
            .lock()
            .unwrap()
            .pad_colors
            .push((pad, color.r, color.g, color.b));
        Ok(())
    }

    fn set_pad_velocity(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.state
            .lock()
            .unwrap()
            .pad_velocities
            .push((pad, velocity));
        Ok(())
    }

    fn set_mode_colors(&mut self, mode: u8) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().mode = mode;
        Ok(())
    }

    fn show_velocity_feedback(&mut self, pad: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        self.state
            .lock()
            .unwrap()
            .pad_velocities
            .push((pad, velocity));
        Ok(())
    }

    fn flash_pad(
        &mut self,
        _pad: u8,
        _color: conductor_core::mikro_leds::RGB,
        _duration_ms: u64,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn ripple_effect(
        &mut self,
        _start_pad: u8,
        _color: conductor_core::mikro_leds::RGB,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn clear_all(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().cleared = true;
        Ok(())
    }

    fn show_long_press_feedback(
        &mut self,
        pad: u8,
        _elapsed_ms: u128,
    ) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().long_press_pads.push(pad);
        Ok(())
    }

    fn run_scheme(&mut self, scheme: &LightingScheme) -> Result<(), Box<dyn Error>> {
        self.state.lock().unwrap().scheme = format!("{:?}", scheme);
        Ok(())
    }
}

#[test]
fn test_feedback_manager_creation() {
    let mock = MockFeedback::new();
    let manager = FeedbackManager::new(Box::new(mock));

    assert_eq!(manager.current_mode(), 0);
    assert_eq!(manager.current_scheme(), LightingScheme::Reactive);
    assert_eq!(manager.active_pads(), 0);
}

#[test]
fn test_feedback_manager_on_pad_press() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press pad 5 with velocity 100
    manager.on_pad_press(5, 100).unwrap();

    assert_eq!(manager.active_pads(), 1);
}

#[test]
fn test_feedback_manager_on_pad_press_multiple() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press multiple pads
    manager.on_pad_press(0, 80).unwrap();
    manager.on_pad_press(5, 100).unwrap();
    manager.on_pad_press(15, 120).unwrap();

    assert_eq!(manager.active_pads(), 3);
}

#[test]
fn test_feedback_manager_on_mode_change() {
    let mock = MockFeedback::new();
    let mock_state = mock.state.clone();
    let mut manager = FeedbackManager::new(Box::new(mock));

    let color = conductor_core::mikro_leds::RGB { r: 255, g: 0, b: 0 };
    manager.on_mode_change(2, color).unwrap();

    assert_eq!(manager.current_mode(), 2);
    assert_eq!(mock_state.lock().unwrap().mode, 2);
}

#[test]
fn test_feedback_manager_set_scheme() {
    let mock = MockFeedback::new();
    let mock_state = mock.state.clone();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Set to static scheme
    manager.set_scheme(LightingScheme::Static(1)).unwrap();

    assert_eq!(manager.current_scheme(), LightingScheme::Static(1));
    let state = mock_state.lock().unwrap();
    assert!(state.scheme.contains("Static"));
}

#[test]
fn test_feedback_manager_set_scheme_clears_reactive_state() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press some pads in reactive mode
    manager.on_pad_press(0, 80).unwrap();
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 2);

    // Switch to a non-reactive scheme
    manager.set_scheme(LightingScheme::Rainbow).unwrap();

    // Reactive state should be cleared
    assert_eq!(manager.active_pads(), 0);
}

#[test]
fn test_feedback_manager_update_inactive_scheme() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Switch to non-reactive scheme
    manager.set_scheme(LightingScheme::Breathing).unwrap();

    // Update should do nothing and return empty vec
    let completed = manager.update().unwrap();
    assert!(completed.is_empty());
}

#[test]
fn test_feedback_manager_update_reactive_early() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press a pad in reactive mode
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 1);

    // Immediately update - pad should still be fading
    let completed = manager.update().unwrap();
    assert!(completed.is_empty());
    assert_eq!(manager.active_pads(), 1);
}

#[test]
fn test_feedback_manager_update_reactive_fade_complete() {
    use std::thread;
    use std::time::Duration;

    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press a pad
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 1);

    // Wait for fade period to complete (1 second + small buffer)
    thread::sleep(Duration::from_millis(1100));

    // Update should show pad as completed
    let completed = manager.update().unwrap();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0], 5);
    assert_eq!(manager.active_pads(), 0);
}

#[test]
fn test_feedback_manager_clear() {
    let mock = MockFeedback::new();
    let mock_state = mock.state.clone();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press some pads
    manager.on_pad_press(0, 80).unwrap();
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 2);

    // Clear
    manager.clear().unwrap();

    assert_eq!(manager.active_pads(), 0);
    assert!(mock_state.lock().unwrap().cleared);
}

#[test]
fn test_feedback_manager_on_pad_release() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press and release
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 1);

    manager.on_pad_release(5).unwrap();
    // Pad should still be tracked for fade-out
    assert_eq!(manager.active_pads(), 1);
}

#[test]
fn test_feedback_manager_multiple_pads_different_velocities() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press multiple pads with different velocities
    manager.on_pad_press(0, 50).unwrap(); // Soft
    manager.on_pad_press(1, 100).unwrap(); // Medium
    manager.on_pad_press(2, 127).unwrap(); // Hard

    assert_eq!(manager.active_pads(), 3);
    assert_eq!(manager.current_scheme(), LightingScheme::Reactive);
}

#[test]
fn test_feedback_manager_scheme_query() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    assert_eq!(manager.current_scheme(), LightingScheme::Reactive);

    manager.set_scheme(LightingScheme::Rainbow).unwrap();
    assert_eq!(manager.current_scheme(), LightingScheme::Rainbow);

    manager.set_scheme(LightingScheme::Off).unwrap();
    assert_eq!(manager.current_scheme(), LightingScheme::Off);
}

#[test]
fn test_feedback_manager_device_access() {
    let mock = MockFeedback::new();
    let manager = FeedbackManager::new(Box::new(mock));

    // Can access the device
    let _device = manager.device();
}

#[test]
fn test_feedback_manager_device_mut_access() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Can mutably access the device
    let _device = manager.device_mut();
}

#[test]
fn test_feedback_manager_sequential_operations() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Create a sequence of operations
    assert_eq!(manager.current_mode(), 0);

    let color = conductor_core::mikro_leds::RGB { r: 0, g: 255, b: 0 };
    manager.on_mode_change(1, color).unwrap();
    assert_eq!(manager.current_mode(), 1);

    manager.on_pad_press(5, 80).unwrap();
    assert_eq!(manager.active_pads(), 1);

    manager.on_pad_press(10, 120).unwrap();
    assert_eq!(manager.active_pads(), 2);

    manager.on_pad_release(5).unwrap();
    // Both pads still active (waiting for fade)
    assert_eq!(manager.active_pads(), 2);

    manager.clear().unwrap();
    assert_eq!(manager.active_pads(), 0);
}

#[test]
fn test_feedback_manager_off_scheme() {
    let mock = MockFeedback::new();
    let mock_state = mock.state.clone();
    let mut manager = FeedbackManager::new(Box::new(mock));

    manager.set_scheme(LightingScheme::Off).unwrap();

    assert_eq!(manager.current_scheme(), LightingScheme::Off);
    let state = mock_state.lock().unwrap();
    assert!(state.scheme.contains("Off"));
}

#[test]
fn test_feedback_manager_pulse_scheme() {
    let mock = MockFeedback::new();
    let mock_state = mock.state.clone();
    let mut manager = FeedbackManager::new(Box::new(mock));

    manager.set_scheme(LightingScheme::Pulse).unwrap();

    assert_eq!(manager.current_scheme(), LightingScheme::Pulse);
    let state = mock_state.lock().unwrap();
    assert!(state.scheme.contains("Pulse"));
}

#[test]
fn test_feedback_manager_reactive_then_breathing_then_reactive() {
    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Start in reactive mode
    assert_eq!(manager.current_scheme(), LightingScheme::Reactive);
    manager.on_pad_press(5, 100).unwrap();
    assert_eq!(manager.active_pads(), 1);

    // Switch to breathing (clears reactive state)
    manager.set_scheme(LightingScheme::Breathing).unwrap();
    assert_eq!(manager.active_pads(), 0);

    // Switch back to reactive
    manager.set_scheme(LightingScheme::Reactive).unwrap();
    assert_eq!(manager.active_pads(), 0);

    // New presses should work
    manager.on_pad_press(10, 80).unwrap();
    assert_eq!(manager.active_pads(), 1);
}

#[test]
fn test_feedback_manager_concurrent_pads_fade() {
    use std::thread;
    use std::time::Duration;

    let mock = MockFeedback::new();
    let mut manager = FeedbackManager::new(Box::new(mock));

    // Press multiple pads at different times
    manager.on_pad_press(0, 80).unwrap();
    thread::sleep(Duration::from_millis(100));

    manager.on_pad_press(1, 90).unwrap();
    thread::sleep(Duration::from_millis(100));

    manager.on_pad_press(2, 100).unwrap();

    assert_eq!(manager.active_pads(), 3);

    // Wait and update
    thread::sleep(Duration::from_millis(1100));

    let completed = manager.update().unwrap();
    // All pads should be completed by now
    assert_eq!(completed.len(), 3);
    assert_eq!(manager.active_pads(), 0);
}
