#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod data;
mod events;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

fn main() {
    let data_dir = data::get_data_dir();
    let mru_file = data::get_mru_file();
    match fs::create_dir(&data_dir) {
        Ok(()) => {}
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {}
            _ => panic!("Cannot create data dir"),
        },
    }
    let mru = Path::new(&mru_file);
    if !mru.exists() {
        fs::write(mru, b"{\"recent\": []}").unwrap();
    }
    println!("{:?}", data_dir);
    tauri::AppBuilder::new()
        .setup(move |webview, _| {
            events::send_init(webview.as_mut());
            events::listen_for_search(webview.as_mut());
            events::listen_for_selection();
        })
        .build()
        .run();
}
