// src/history.rs
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub fn load_history() -> Vec<String> {
    let history_path = get_history_path().expect("Failed to get history path");
    let file = File::open(history_path).ok()?;
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line| line.ok()).collect()
}
