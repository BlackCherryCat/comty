// src/utils.rs
use std::path::Path;
use dirs::data_local_dir;

pub fn get_history_path() -> Option<std::path::PathBuf> {
    match data_local_dir() {
        Some(mut dir) => {
            dir.push("comty");
            dir.push("comty_history");
            Some(dir)
        },
        None => {
            let mut path = dirs::home_dir()?;
            path.push(".local/share/comty/comty_history");
            Some(path)
        }
    }
}

pub fn file_exists(file: &str) -> bool {
    Path::new(file).exists()
}
