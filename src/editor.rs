// src/editor.rs
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use dirs::data_local_dir;
use crate::context::Context;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputMode {
    Input,
    Command
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Input
    }
}

pub struct Editor {
    pub input: String,
    pub mode: InputMode,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            input: String::new(),
            mode: InputMode::Input,
        }
    }

    pub fn push(&mut self, input: char) {
        if self.mode == InputMode::Input {
            self.input.push(input);

            // Save to history
            if let Some(history_path) = get_history_path() {
                let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(history_path)
                .expect("Failed to open history file");

                let line = format!("{}:{}", self.input.len(), self.input);
                file.write_all(line.as_bytes())
                .expect("Failed to write to history file");
            }
        } else if self.mode == InputMode::Command {
            // Handle command mode
        }
    }

    pub fn pop(&mut self) {
        if self.mode == InputMode::Input && !self.input.is_empty() {
            self.input.pop();
        }
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }
}
