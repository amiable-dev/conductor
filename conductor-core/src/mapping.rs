// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::MidiEvent;
use crate::actions::Action;
use crate::config::{Config, Mapping, Trigger};
use crate::event_processor::ProcessedEvent;
use std::collections::HashMap;
use tracing::{debug, trace};

pub struct MappingEngine {
    mode_mappings: HashMap<u8, Vec<CompiledMapping>>,
    global_mappings: Vec<CompiledMapping>,
}

impl Default for MappingEngine {
    fn default() -> Self {
        Self::new()
    }
}

struct CompiledMapping {
    trigger: CompiledTrigger,
    action: Action,
    description: Option<String>,
}

enum CompiledTrigger {
    Note {
        note: u8,
        velocity_min: u8,
    },
    CC {
        cc: u8,
        value_min: u8,
    },
    NoteChord {
        notes: Vec<u8>,
    },
    // Gamepad triggers (v3.0)
    GamepadButton {
        button: u8,
        velocity_min: u8,
    },
    GamepadButtonChord {
        buttons: Vec<u8>,
    },
    GamepadAnalogStick {
        axis: u8,
        direction: Option<String>,
    },
    GamepadTrigger {
        trigger: u8,
        threshold: u8,
    },
}

impl MappingEngine {
    pub fn new() -> Self {
        Self {
            mode_mappings: HashMap::new(),
            global_mappings: Vec::new(),
        }
    }

    pub fn load_from_config(&mut self, config: &Config) {
        // Load mode-specific mappings
        for (mode_idx, mode) in config.modes.iter().enumerate() {
            let compiled: Vec<CompiledMapping> = mode
                .mappings
                .iter()
                .map(|m| self.compile_mapping(m))
                .collect();

            self.mode_mappings.insert(mode_idx as u8, compiled);
        }

        // Load global mappings
        self.global_mappings = config
            .global_mappings
            .iter()
            .map(|m| self.compile_mapping(m))
            .collect();
    }

    fn compile_mapping(&self, mapping: &Mapping) -> CompiledMapping {
        CompiledMapping {
            trigger: match &mapping.trigger {
                Trigger::Note { note, velocity_min } => CompiledTrigger::Note {
                    note: *note,
                    velocity_min: velocity_min.unwrap_or(1),
                },
                Trigger::CC { cc, value_min } => CompiledTrigger::CC {
                    cc: *cc,
                    value_min: value_min.unwrap_or(0),
                },
                Trigger::NoteChord { notes, .. } => CompiledTrigger::NoteChord {
                    notes: notes.clone(),
                },
                // Gamepad triggers (v3.0)
                Trigger::GamepadButton { button, velocity_min } => CompiledTrigger::GamepadButton {
                    button: *button,
                    velocity_min: velocity_min.unwrap_or(1),
                },
                Trigger::GamepadButtonChord { buttons, .. } => CompiledTrigger::GamepadButtonChord {
                    buttons: buttons.clone(),
                },
                Trigger::GamepadAnalogStick { axis, direction } => CompiledTrigger::GamepadAnalogStick {
                    axis: *axis,
                    direction: direction.clone(),
                },
                Trigger::GamepadTrigger { trigger, threshold } => CompiledTrigger::GamepadTrigger {
                    trigger: *trigger,
                    threshold: threshold.unwrap_or(0),
                },
                // Other trigger types are not yet fully integrated into CompiledTrigger
                // Fall back to a default Note trigger for now
                _ => CompiledTrigger::Note {
                    note: 0,
                    velocity_min: 1,
                },
            },
            action: mapping.action.clone().into(),
            description: mapping.description.clone(),
        }
    }

    pub fn get_action(&self, event: &MidiEvent, mode: u8) -> Option<Action> {
        // Check mode-specific mappings first
        if let Some(mode_mappings) = self.mode_mappings.get(&mode) {
            return self.find_matching_action(event, mode_mappings);
        }

        // Check global mappings
        self.find_matching_action(event, &self.global_mappings)
    }

    /// Get action for a processed event (supports advanced triggers like chords)
    pub fn get_action_for_processed(&self, event: &ProcessedEvent, mode: u8) -> Option<Action> {
        // Check mode-specific mappings first
        if let Some(mode_mappings) = self.mode_mappings.get(&mode) {
            if let Some(action) = self.find_matching_action_for_processed(event, mode_mappings) {
                return Some(action);
            }
        }

        // Check global mappings
        self.find_matching_action_for_processed(event, &self.global_mappings)
    }

    fn find_matching_action(
        &self,
        event: &MidiEvent,
        mappings: &[CompiledMapping],
    ) -> Option<Action> {
        for mapping in mappings {
            if self.trigger_matches(&mapping.trigger, event) {
                if let Some(desc) = &mapping.description {
                    debug!(mapping = desc, "Executing mapped action");
                }
                return Some(mapping.action.clone());
            }
        }
        trace!("No mapping found for MIDI event");
        None
    }

    fn find_matching_action_for_processed(
        &self,
        event: &ProcessedEvent,
        mappings: &[CompiledMapping],
    ) -> Option<Action> {
        for mapping in mappings {
            if self.trigger_matches_processed(&mapping.trigger, event) {
                if let Some(desc) = &mapping.description {
                    debug!(mapping = desc, "Executing mapped action for processed event");
                }
                return Some(mapping.action.clone());
            }
        }
        trace!("No mapping found for processed event");
        None
    }

    fn trigger_matches(&self, trigger: &CompiledTrigger, event: &MidiEvent) -> bool {
        match (trigger, event) {
            (
                CompiledTrigger::Note { note, velocity_min },
                MidiEvent::NoteOn {
                    note: ev_note,
                    velocity,
                    ..
                },
            ) => *note == *ev_note && *velocity >= *velocity_min,
            (
                CompiledTrigger::CC { cc, value_min },
                MidiEvent::ControlChange {
                    cc: ev_cc, value, ..
                },
            ) => *cc == *ev_cc && *value >= *value_min,
            _ => false,
        }
    }

    fn trigger_matches_processed(&self, trigger: &CompiledTrigger, event: &ProcessedEvent) -> bool {
        match (trigger, event) {
            (
                CompiledTrigger::NoteChord { notes },
                ProcessedEvent::ChordDetected { notes: detected_notes },
            ) => {
                // Check if all required notes are present in the detected chord
                // Sort both lists for comparison
                let mut required = notes.clone();
                let mut detected = detected_notes.clone();
                required.sort_unstable();
                detected.sort_unstable();

                required == detected
            }
            // Gamepad button press (v3.0)
            (
                CompiledTrigger::GamepadButton { button, velocity_min },
                ProcessedEvent::PadPressed { note, velocity, .. },
            ) => {
                // Gamepad buttons use IDs 128-255 to avoid MIDI conflicts
                *button == *note && *velocity >= *velocity_min && *note >= 128
            }
            // Gamepad button chord (v3.0)
            (
                CompiledTrigger::GamepadButtonChord { buttons },
                ProcessedEvent::ChordDetected { notes: detected_buttons },
            ) => {
                // Check if all required gamepad buttons are present
                // Sort both lists for comparison
                let mut required = buttons.clone();
                let mut detected = detected_buttons.clone();
                required.sort_unstable();
                detected.sort_unstable();

                // Only match if all buttons are in gamepad range (128-255)
                required == detected && required.iter().all(|b| *b >= 128)
            }
            // Gamepad analog stick (v3.0)
            (
                CompiledTrigger::GamepadAnalogStick { axis, direction },
                ProcessedEvent::EncoderTurned { cc, direction: ev_direction, .. },
            ) => {
                // Gamepad analog sticks use axis IDs 128-131
                if *axis != *cc || *cc < 128 || *cc > 131 {
                    return false;
                }

                // Check direction if specified
                match direction {
                    Some(dir) if dir == "Clockwise" => {
                        matches!(ev_direction, crate::event_processor::EncoderDirection::Clockwise)
                    }
                    Some(dir) if dir == "CounterClockwise" => {
                        matches!(ev_direction, crate::event_processor::EncoderDirection::CounterClockwise)
                    }
                    _ => true, // Any direction
                }
            }
            // Gamepad analog trigger (v3.0)
            (
                CompiledTrigger::GamepadTrigger { trigger, threshold },
                ProcessedEvent::EncoderTurned { cc, value, .. },
            ) => {
                // Gamepad analog triggers use IDs 132-133
                *trigger == *cc && *value >= *threshold && (*cc == 132 || *cc == 133)
            }
            _ => false,
        }
    }
}
