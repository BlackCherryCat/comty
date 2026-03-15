// src/main.rs
use ncurses::{initscr, endwin, noecho, curs_set, wgetch, clear, refresh, mvprintw, KEY_RESIZE};
use std::io;
use std::path::Path;
use crate::editor::Editor;
use crate::utils::{get_history_path, file_exists};

fn main() {
    // Initialize ncurses
    let mut stdscr = initscr();
    noecho();
    curs_set(0); // Hide cursor

    // Initialize editor
    let mut editor = Editor::new();

    // Main loop
    loop {
        // Clear screen
        clear();
        refresh();

        // Display input
        mvprintw(0, 0, "{}", editor.input);
        refresh();

        // Get input
        let ch = wgetch(&mut stdscr);

        match ch {
            KEY_RESIZE => {
                // Handle window resize
                clear();
                refresh();
            },
            27 => break, // ESC key to exit
            _ => {
                if let Some(c) = std::char::from_u32(ch as u32) {
                    editor.push(c);
                }
            }
        }
    }

    // Clean up
    endwin();
}
