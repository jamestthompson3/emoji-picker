use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Emoji {
    pub keywords: Vec<String>,
    pub char: String,
    pub fitzpatrick_scale: bool,
    pub category: String,
}
/// Data dir is found in the following locations:
/// Linux:   /home/alice/.local/share/emojipicker
/// Windows: C:\Users\Alice\AppData\Roaming\Emoji Picker\emojipicker\data
/// macOS:   /Users/Alice/Library/Application Support/emojipicker
pub fn get_data_dir() -> std::path::PathBuf {
    let project_dirs = ProjectDirs::from("com", "Emoji Picker", "emojipicker").unwrap();
    let data_dir = project_dirs.data_dir();
    data_dir.to_owned()
}

pub fn get_mru_file() -> PathBuf {
    let mut data_dir = get_data_dir();
    data_dir.push("mru.json");
    data_dir
}

pub fn get_mru_file_string() -> Result<String, std::io::Error> {
    let mru_file = get_mru_file();
    let mru_file_string = fs::read_to_string(&mru_file);
    mru_file_string
}
