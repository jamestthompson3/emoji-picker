#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
use std::fs::read_to_string;

fn main() {
    tauri::AppBuilder::new()
        .setup(move |_webview, _| {
            tauri::event::listen("read".to_string(), move |_| {
                let file = String::from("../../dist/emoji.txt");
                let s = read_to_string(&file).unwrap();
                println!("{}", s);
            });
        })
        .build()
        .run();
}
