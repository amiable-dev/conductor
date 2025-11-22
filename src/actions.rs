// Copyright 2025 Amiable
// SPDX-License-Identifier: MIT

use crate::config::ActionConfig;
use enigo::{Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Action {
    Keystroke {
        keys: Vec<Key>,
        modifiers: Vec<Key>,
    },
    Text(String),
    Launch(String),
    Shell(String),
    Sequence(Vec<Action>),
    Delay(u64),
    MouseClick {
        button: Button,
        x: Option<i32>,
        y: Option<i32>,
    },
}

pub struct ActionExecutor {
    enigo: Enigo,
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(&Settings::default()).unwrap(),
        }
    }

    pub fn execute(&mut self, action: Action) {
        match action {
            Action::Keystroke { keys, modifiers } => {
                self.execute_keystroke(keys, modifiers);
            }
            Action::Text(text) => {
                self.enigo.text(&text).unwrap();
            }
            Action::Launch(app) => {
                self.launch_app(&app);
            }
            Action::Shell(cmd) => {
                self.execute_shell(&cmd);
            }
            Action::Sequence(actions) => {
                for act in actions {
                    self.execute(act);
                    thread::sleep(Duration::from_millis(50));
                }
            }
            Action::Delay(ms) => {
                thread::sleep(Duration::from_millis(ms));
            }
            Action::MouseClick { button, x, y } => {
                if let (Some(x), Some(y)) = (x, y) {
                    self.enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                }
                self.enigo.button(button, Direction::Click).unwrap();
            }
        }
    }

    fn execute_keystroke(&mut self, keys: Vec<Key>, modifiers: Vec<Key>) {
        // Press modifiers
        for modifier in &modifiers {
            self.enigo.key(*modifier, Direction::Press).unwrap();
        }

        // Press keys
        for key in &keys {
            self.enigo.key(*key, Direction::Click).unwrap();
        }

        // Release modifiers
        for modifier in modifiers.iter().rev() {
            self.enigo.key(*modifier, Direction::Release).unwrap();
        }
    }

    fn launch_app(&self, app: &str) {
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg("-a").arg(app).spawn().ok();
        }

        #[cfg(target_os = "linux")]
        {
            Command::new(app).spawn().ok();
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd").args(&["/C", "start", app]).spawn().ok();
        }
    }

    fn execute_shell(&self, cmd: &str) {
        #[cfg(unix)]
        {
            Command::new("sh").arg("-c").arg(cmd).spawn().ok();
        }

        #[cfg(windows)]
        {
            Command::new("cmd").args(&["/C", cmd]).spawn().ok();
        }
    }
}

impl From<ActionConfig> for Action {
    fn from(config: ActionConfig) -> Self {
        match config {
            ActionConfig::Keystroke { keys, modifiers } => Action::Keystroke {
                keys: parse_keys(&keys),
                modifiers: modifiers.iter().flat_map(|m| parse_modifier(m)).collect(),
            },
            ActionConfig::Text { text } => Action::Text(text),
            ActionConfig::Launch { app } => Action::Launch(app),
            ActionConfig::Shell { command } => Action::Shell(command),
            ActionConfig::Sequence { actions } => {
                Action::Sequence(actions.into_iter().map(Into::into).collect())
            }
            ActionConfig::Delay { ms } => Action::Delay(ms),
            ActionConfig::MouseClick { button, x, y } => Action::MouseClick {
                button: parse_mouse_button(&button),
                x,
                y,
            },
        }
    }
}

fn parse_keys(keys: &str) -> Vec<Key> {
    keys.split('+')
        .filter_map(|k| parse_key(k.trim()))
        .collect()
}

fn parse_key(key: &str) -> Option<Key> {
    match key.to_lowercase().as_str() {
        "space" => Some(Key::Space),
        "return" | "enter" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "escape" | "esc" => Some(Key::Escape),
        "backspace" => Some(Key::Backspace),
        "delete" | "del" => Some(Key::Delete),
        "up" => Some(Key::UpArrow),
        "down" => Some(Key::DownArrow),
        "left" => Some(Key::LeftArrow),
        "right" => Some(Key::RightArrow),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Some(Key::Unicode(c))
        }
        _ => None,
    }
}

fn parse_modifier(modifier: &str) -> Option<Key> {
    match modifier.to_lowercase().as_str() {
        "cmd" | "command" | "meta" => Some(Key::Meta),
        "ctrl" | "control" => Some(Key::Control),
        "alt" | "option" => Some(Key::Option),
        "shift" => Some(Key::Shift),
        _ => None,
    }
}

fn parse_mouse_button(button: &str) -> Button {
    match button.to_lowercase().as_str() {
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Left,
    }
}
