// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

//! Device Template System
//!
//! Provides pre-configured templates for popular MIDI controllers.
//! Templates include common mappings and device-specific configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Device template information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTemplate {
    /// Unique template identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Device manufacturer
    pub manufacturer: String,

    /// Device model/series
    pub model: String,

    /// Template description
    pub description: String,

    /// MIDI device name patterns that match this template
    pub midi_patterns: Vec<String>,

    /// Template category (e.g., "pad-controller", "keyboard", "mixer")
    pub category: String,

    /// Example configuration in TOML format
    pub config_template: String,

    /// Thumbnail/icon URL (optional)
    pub thumbnail_url: Option<String>,

    /// Whether this is an official template
    pub is_official: bool,

    /// Template version
    pub version: String,
}

/// Device template registry
pub struct DeviceTemplateRegistry {
    templates: HashMap<String, DeviceTemplate>,
}

impl DeviceTemplateRegistry {
    /// Create a new registry with built-in templates
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };

        // Register built-in templates
        registry.register_builtin_templates();
        registry
    }

    /// Register all built-in device templates
    fn register_builtin_templates(&mut self) {
        // Native Instruments Maschine Mikro MK3
        self.register_template(DeviceTemplate {
            id: "ni-maschine-mikro-mk3".to_string(),
            name: "Maschine Mikro MK3".to_string(),
            manufacturer: "Native Instruments".to_string(),
            model: "Maschine Mikro MK3".to_string(),
            description: "16-pad grid controller with encoder and touch strip. Optimized for beat production and live performance.".to_string(),
            midi_patterns: vec![
                "Maschine Mikro MK3".to_string(),
                "MIDIIN2 (Maschine Mikro MK3)".to_string(),
            ],
            category: "pad-controller".to_string(),
            config_template: include_str!("../templates/maschine-mikro-mk3.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // Novation Launchpad Mini
        self.register_template(DeviceTemplate {
            id: "novation-launchpad-mini".to_string(),
            name: "Launchpad Mini".to_string(),
            manufacturer: "Novation".to_string(),
            model: "Launchpad Mini MK3".to_string(),
            description: "8x8 grid controller with RGB LEDs. Perfect for clip launching and grid-based control.".to_string(),
            midi_patterns: vec![
                "Launchpad Mini".to_string(),
                "Launchpad Mini MK3".to_string(),
            ],
            category: "pad-controller".to_string(),
            config_template: include_str!("../templates/launchpad-mini.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // KORG nanoKONTROL2
        self.register_template(DeviceTemplate {
            id: "korg-nanokontrol2".to_string(),
            name: "nanoKONTROL2".to_string(),
            manufacturer: "KORG".to_string(),
            model: "nanoKONTROL2".to_string(),
            description:
                "Slim-line USB MIDI controller with 8 channels of faders, knobs, and buttons."
                    .to_string(),
            midi_patterns: vec!["nanoKONTROL2".to_string(), "KORG nanoKONTROL2".to_string()],
            category: "mixer-controller".to_string(),
            config_template: include_str!("../templates/nanokontrol2.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // Akai APC Mini
        self.register_template(DeviceTemplate {
            id: "akai-apc-mini".to_string(),
            name: "APC Mini".to_string(),
            manufacturer: "Akai".to_string(),
            model: "APC Mini".to_string(),
            description:
                "Compact 8x8 grid controller with 9 faders. Designed for Ableton Live control."
                    .to_string(),
            midi_patterns: vec!["APC MINI".to_string(), "Akai APC MINI".to_string()],
            category: "pad-controller".to_string(),
            config_template: include_str!("../templates/apc-mini.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // Arturia BeatStep
        self.register_template(DeviceTemplate {
            id: "arturia-beatstep".to_string(),
            name: "BeatStep".to_string(),
            manufacturer: "Arturia".to_string(),
            model: "BeatStep".to_string(),
            description:
                "Pad controller and step sequencer with 16 velocity-sensitive pads and 16 encoders."
                    .to_string(),
            midi_patterns: vec!["Arturia BeatStep".to_string(), "BeatStep".to_string()],
            category: "pad-controller".to_string(),
            config_template: include_str!("../templates/beatstep.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // Generic 25-key MIDI keyboard
        self.register_template(DeviceTemplate {
            id: "generic-midi-keyboard-25".to_string(),
            name: "Generic 25-Key MIDI Keyboard".to_string(),
            manufacturer: "Generic".to_string(),
            model: "25-Key Keyboard".to_string(),
            description: "Basic template for 25-key MIDI keyboards with pitch bend and mod wheel."
                .to_string(),
            midi_patterns: vec!["USB MIDI Keyboard".to_string(), "MIDI Keyboard".to_string()],
            category: "keyboard".to_string(),
            config_template: include_str!("../templates/generic-keyboard-25.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // ========== Gamepad Controllers (v3.0) ==========

        // Xbox Controller (360, One, Series X|S)
        self.register_template(DeviceTemplate {
            id: "xbox-controller".to_string(),
            name: "Xbox Controller".to_string(),
            manufacturer: "Microsoft".to_string(),
            model: "Xbox Controller".to_string(),
            description: "Microsoft Xbox controller (Xbox 360, Xbox One, Xbox Series X|S) with analog triggers and dual sticks. Optimized for desktop navigation, media control, and gaming macros.".to_string(),
            midi_patterns: vec![], // Gamepads don't have MIDI patterns
            category: "gamepad-controller".to_string(),
            config_template: include_str!("../templates/xbox-controller.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // PlayStation Controller (DualShock 4, DualSense)
        self.register_template(DeviceTemplate {
            id: "playstation-controller".to_string(),
            name: "PlayStation Controller".to_string(),
            manufacturer: "Sony".to_string(),
            model: "DualShock/DualSense".to_string(),
            description: "Sony PlayStation controller (DualShock 4, DualSense) with adaptive triggers and haptic feedback support. Includes desktop navigation, media control, and gaming presets.".to_string(),
            midi_patterns: vec![], // Gamepads don't have MIDI patterns
            category: "gamepad-controller".to_string(),
            config_template: include_str!("../templates/playstation-controller.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });

        // Nintendo Switch Pro Controller
        self.register_template(DeviceTemplate {
            id: "switch-pro-controller".to_string(),
            name: "Switch Pro Controller".to_string(),
            manufacturer: "Nintendo".to_string(),
            model: "Switch Pro Controller".to_string(),
            description: "Nintendo Switch Pro Controller with HD rumble and motion controls. Features desktop navigation, media control, and browser control modes.".to_string(),
            midi_patterns: vec![], // Gamepads don't have MIDI patterns
            category: "gamepad-controller".to_string(),
            config_template: include_str!("../templates/switch-pro-controller.toml").to_string(),
            thumbnail_url: None,
            is_official: true,
            version: "1.0.0".to_string(),
        });
    }

    /// Register a template
    pub fn register_template(&mut self, template: DeviceTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Get a template by ID
    pub fn get_template(&self, id: &str) -> Option<&DeviceTemplate> {
        self.templates.get(id)
    }

    /// List all templates
    pub fn list_templates(&self) -> Vec<&DeviceTemplate> {
        self.templates.values().collect()
    }

    /// List templates by category
    pub fn list_by_category(&self, category: &str) -> Vec<&DeviceTemplate> {
        self.templates
            .values()
            .filter(|t| t.category == category)
            .collect()
    }

    /// Find templates matching a MIDI device name
    pub fn find_by_midi_name(&self, midi_name: &str) -> Vec<&DeviceTemplate> {
        self.templates
            .values()
            .filter(|t| {
                t.midi_patterns
                    .iter()
                    .any(|pattern| midi_name.to_lowercase().contains(&pattern.to_lowercase()))
            })
            .collect()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .templates
            .values()
            .map(|t| t.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    /// Create config from template
    pub fn create_config_from_template(&self, template_id: &str) -> Result<String, String> {
        let template = self
            .get_template(template_id)
            .ok_or_else(|| format!("Template not found: {}", template_id))?;

        Ok(template.config_template.clone())
    }
}

impl Default for DeviceTemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = DeviceTemplateRegistry::new();
        assert!(registry.list_templates().len() > 0);
    }

    #[test]
    fn test_get_template() {
        let registry = DeviceTemplateRegistry::new();
        let template = registry.get_template("ni-maschine-mikro-mk3");
        assert!(template.is_some());
        assert_eq!(template.unwrap().manufacturer, "Native Instruments");
    }

    #[test]
    fn test_find_by_midi_name() {
        let registry = DeviceTemplateRegistry::new();
        let results = registry.find_by_midi_name("Maschine Mikro MK3");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "ni-maschine-mikro-mk3");
    }

    #[test]
    fn test_categories() {
        let registry = DeviceTemplateRegistry::new();
        let categories = registry.get_categories();
        assert!(categories.contains(&"pad-controller".to_string()));
        assert!(categories.contains(&"keyboard".to_string()));
    }

    #[test]
    fn test_list_by_category() {
        let registry = DeviceTemplateRegistry::new();
        let pad_controllers = registry.list_by_category("pad-controller");
        assert!(pad_controllers.len() > 0);
    }
}
