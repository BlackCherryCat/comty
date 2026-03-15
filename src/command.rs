// src/command.rs
use crate::editor::Editor;

pub enum Command {
    Clear,
    Quit,
    Help,
}

impl Command {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'c' => Some(Command::Clear),
            'q' => Some(Command::Quit),
            'h' => Some(Command::Help),
            _ => None
        }
    }

    pub fn execute(&self, editor: &mut Editor) {
        match self {
            Command::Clear => editor.clear(),
            Command::Quit => std::process::exit(0),
            Command::Help => {
                // Show help message
            }
        }
    }
}
