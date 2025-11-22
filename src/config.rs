// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub modes: Vec<Mode>,
    #[serde(default)]
    pub global_mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceConfig {
    pub name: String,
    pub auto_connect: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mode {
    pub name: String,
    pub color: Option<String>,
    #[serde(default)]
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mapping {
    pub trigger: Trigger,
    pub action: ActionConfig,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Trigger {
    Note { note: u8, velocity_min: Option<u8> },
    CC { cc: u8, value_min: Option<u8> },
    NoteChord { notes: Vec<u8> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ActionConfig {
    Keystroke {
        keys: String,
        #[serde(default)]
        modifiers: Vec<String>,
    },
    Text {
        text: String,
    },
    Launch {
        app: String,
    },
    Shell {
        command: String,
    },
    Sequence {
        actions: Vec<ActionConfig>,
    },
    Delay {
        ms: u64,
    },
    MouseClick {
        button: String,
        x: Option<i32>,
        y: Option<i32>,
    },
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load from file, otherwise use default
        if std::path::Path::new(path).exists() {
            let contents = std::fs::read_to_string(path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            println!("Config file not found, creating default config...");
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    fn default() -> Self {
        Config {
            device: DeviceConfig {
                name: "Mikro".to_string(),
                auto_connect: true,
            },
            modes: vec![
                Mode {
                    name: "Default".to_string(),
                    color: Some("blue".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: Some(1),
                        },
                        action: ActionConfig::Keystroke {
                            keys: "space".to_string(),
                            modifiers: vec!["cmd".to_string()],
                        },
                        description: Some("Spotlight Search".to_string()),
                    }],
                },
                Mode {
                    name: "Development".to_string(),
                    color: Some("green".to_string()),
                    mappings: vec![Mapping {
                        trigger: Trigger::Note {
                            note: 60,
                            velocity_min: None,
                        },
                        action: ActionConfig::Shell {
                            command: "git status".to_string(),
                        },
                        description: Some("Git status".to_string()),
                    }],
                },
            ],
            global_mappings: vec![],
        }
    }
}
