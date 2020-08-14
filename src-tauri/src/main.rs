#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod data;
mod events;
use directories::ProjectDirs;
use std::env;
//use std::fs::File;

fn main() {
    // Linux:   /home/alice/.local/share/emojipicker
    // Windows: C:\Users\Alice\AppData\Roaming\Emoji Picker\emojipicker\data
    // macOS:   /Users/Alice/Library/Application Support/emojipicker
    let project_dirs = ProjectDirs::from("com", "Emoji Picker", "emojipicker").unwrap();
    let data_dir = project_dirs.data_dir();
    println!("{:?}", data_dir);
    tauri::AppBuilder::new()
        .setup(move |webview, _| {
            let curr_dir = env::current_dir().unwrap();
            println!("\x1b[38;5;206m\n{:?}\n\x1b[0m", curr_dir.display());
            events::listen_for_search(webview.as_mut());
            events::listen_for_selection();
        })
        .build()
        .run();
}
