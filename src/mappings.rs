// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::MidiEvent;
use crate::actions::Action;
use crate::config::{Config, Mapping, Trigger};
use colored::Colorize;
use std::collections::HashMap;

pub struct MappingEngine {
    mode_mappings: HashMap<u8, Vec<CompiledMapping>>,
    global_mappings: Vec<CompiledMapping>,
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
        #[allow(dead_code)]
        notes: Vec<u8>,
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
                Trigger::NoteChord { notes } => CompiledTrigger::NoteChord {
                    notes: notes.clone(),
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

    fn find_matching_action(
        &self,
        event: &MidiEvent,
        mappings: &[CompiledMapping],
    ) -> Option<Action> {
        for mapping in mappings {
            if self.trigger_matches(&mapping.trigger, event) {
                if let Some(desc) = &mapping.description {
                    println!("  → {}", desc.green());
                }
                return Some(mapping.action.clone());
            }
        }
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
                MidiEvent::ControlChange { cc: ev_cc, value, .. },
            ) => *cc == *ev_cc && *value >= *value_min,
            _ => false,
        }
    }
}
