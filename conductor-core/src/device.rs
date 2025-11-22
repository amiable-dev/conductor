// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// NI Controller Editor Profile (.ncmm3 XML format)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "ni-controller-midi-map")]
pub struct NiControllerProfile {
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "midi-map")]
    pub midi_map: MidiMap,
}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct MidiMap {
    #[serde(rename = "@type")]
    pub map_type: String,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "velocitycurve", default)]
    pub velocity_curve: Option<u8>,

    pub groups: Groups,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Controls {
    #[serde(rename = "button", default)]
    pub buttons: Vec<Control>,

    #[serde(rename = "knob", default)]
    pub knobs: Vec<Control>,

    #[serde(rename = "wheel", default)]
    pub wheels: Vec<Control>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Control {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@version", default)]
    pub version: Option<u8>,

    pub cc: Option<u8>,
    pub controller: Option<u8>, // Some use "controller" instead of "cc"
    pub channel: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Groups {
    #[serde(rename = "current_index", default)]
    pub current_index: Option<u8>,

    #[serde(rename = "group")]
    pub pad_pages: Vec<PadPage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PadPage {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@color-index", default)]
    pub color_index: Option<u8>,

    #[serde(rename = "$value", default)]
    pub children: Vec<PadOrLed>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PadOrLed {
    Pad(Pad),
    Led(Led),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pad {
    #[serde(rename = "@subtype")]
    pub subtype: String, // "trigger" or "pressure"

    #[serde(rename = "@version", default)]
    pub version: Option<u8>,

    #[serde(rename = "@id")]
    pub id: String,

    pub note: Option<u8>,   // For trigger pads
    pub polyat: Option<u8>, // For pressure pads
    pub channel: Option<u8>,
    pub behavior: Option<Behavior>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Behavior {
    #[serde(rename = "@onIfDown", default)]
    pub on_if_down: Option<String>,

    #[serde(rename = "$text")]
    pub mode: String, // "gate", "toggle", "trigger"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Led {
    #[serde(rename = "@version", default)]
    pub version: Option<u8>,

    #[serde(rename = "@id")]
    pub id: String,

    pub display: Option<Display>,
    pub note: u8,
    pub channel: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Display {
    #[serde(rename = "@type", default)]
    pub display_type: Option<u8>,

    pub unit: Option<ColorUnit>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorUnit {
    #[serde(rename = "@color-type", default)]
    pub color_type: Option<u8>,

    #[serde(rename = "@color-mode", default)]
    pub color_mode: Option<u8>,

    #[serde(rename = "@color-on-index", default)]
    pub color_on_index: Option<u8>,

    #[serde(rename = "@color-off-index", default)]
    pub color_off_index: Option<u8>,
}

/// Device profile with computed note-to-pad mappings
#[derive(Debug)]
pub struct DeviceProfile {
    pub name: String,
    pub device_type: String,
    pub pad_pages: Vec<PadPageMapping>,
    note_to_page: HashMap<u8, Vec<usize>>, // Map note -> list of page indices that contain it
}

/// Pad page with pre-computed note-to-pad index mapping
#[derive(Debug, Clone)]
pub struct PadPageMapping {
    pub name: String,
    pub note_to_pad: HashMap<u8, usize>, // MIDI note -> pad index (0-15)
    pub pad_to_note: Vec<u8>,            // pad index -> MIDI note
}

impl DeviceProfile {
    /// Load and parse NI Controller Editor profile from XML file
    pub fn from_ncmm3(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let xml_content = fs::read_to_string(path)?;
        let profile: NiControllerProfile = quick_xml::de::from_str(&xml_content)?;

        let mut pad_pages = Vec::new();
        let mut note_to_page: HashMap<u8, Vec<usize>> = HashMap::new();

        for page in &profile.midi_map.groups.pad_pages {
            let mut note_to_pad = HashMap::new();
            let mut pad_to_note = Vec::new();

            // Extract trigger pads (not pressure pads) from children
            let mut trigger_pads: Vec<_> = page
                .children
                .iter()
                .filter_map(|child| {
                    if let PadOrLed::Pad(pad) = child
                        && pad.subtype == "trigger"
                        && pad.note.is_some()
                    {
                        return Some(pad);
                    }
                    None
                })
                .collect();

            // Sort by pad ID to get correct ordering (Pad1, Pad2, ... Pad16)
            trigger_pads.sort_by_key(|p| {
                p.id.strip_prefix("Pad")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0)
            });

            for (pad_idx, pad) in trigger_pads.iter().enumerate() {
                if let Some(note) = pad.note {
                    note_to_pad.insert(note, pad_idx);
                    pad_to_note.push(note);

                    // Track which page(s) contain this note
                    note_to_page.entry(note).or_default().push(pad_pages.len());
                }
            }

            pad_pages.push(PadPageMapping {
                name: page.name.clone(),
                note_to_pad,
                pad_to_note,
            });
        }

        Ok(DeviceProfile {
            name: profile.midi_map.name.clone(),
            device_type: profile.midi_map.map_type.clone(),
            pad_pages,
            note_to_page,
        })
    }

    /// Get pad page by name (e.g., "Pad Page A")
    pub fn get_page_by_name(&self, name: &str) -> Option<&PadPageMapping> {
        self.pad_pages.iter().find(|p| p.name == name)
    }

    /// Get pad page by letter (e.g., "A", "B", "C")
    pub fn get_page_by_letter(&self, letter: char) -> Option<&PadPageMapping> {
        let name = format!("Pad Page {}", letter.to_ascii_uppercase());
        self.get_page_by_name(&name)
    }

    /// Get pad page by index (0 = A, 1 = B, etc.)
    pub fn get_page_by_index(&self, index: usize) -> Option<&PadPageMapping> {
        self.pad_pages.get(index)
    }

    /// Auto-detect which pad page contains a note (returns first match if multiple)
    pub fn detect_page_for_note(&self, note: u8) -> Option<&PadPageMapping> {
        self.note_to_page
            .get(&note)
            .and_then(|pages| pages.first())
            .and_then(|&idx| self.pad_pages.get(idx))
    }

    /// Get all pad pages that contain a specific note
    pub fn get_pages_for_note(&self, note: u8) -> Vec<&PadPageMapping> {
        self.note_to_page
            .get(&note)
            .map(|pages| {
                pages
                    .iter()
                    .filter_map(|&idx| self.pad_pages.get(idx))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl PadPageMapping {
    /// Convert MIDI note to pad index (0-15)
    pub fn note_to_pad_index(&self, note: u8) -> Option<usize> {
        self.note_to_pad.get(&note).copied()
    }

    /// Convert pad index to MIDI note
    pub fn pad_index_to_note(&self, pad_index: usize) -> Option<u8> {
        self.pad_to_note.get(pad_index).copied()
    }

    /// Get note range for this pad page
    pub fn note_range(&self) -> Option<(u8, u8)> {
        let min = *self.pad_to_note.first()?;
        let max = *self.pad_to_note.last()?;
        Some((min, max))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_page_a_mapping() {
        // Pad Page A should map notes 12-27 to pads 0-15
        let mut mapping = PadPageMapping {
            name: "Pad Page A".to_string(),
            note_to_pad: HashMap::new(),
            pad_to_note: Vec::new(),
        };

        for (pad_idx, note) in (12..=27).enumerate() {
            mapping.note_to_pad.insert(note, pad_idx);
            mapping.pad_to_note.push(note);
        }

        assert_eq!(mapping.note_to_pad_index(12), Some(0));
        assert_eq!(mapping.note_to_pad_index(16), Some(4));
        assert_eq!(mapping.note_to_pad_index(27), Some(15));
        assert_eq!(mapping.note_to_pad_index(11), None);
        assert_eq!(mapping.note_to_pad_index(28), None);

        assert_eq!(mapping.note_range(), Some((12, 27)));
    }
}
