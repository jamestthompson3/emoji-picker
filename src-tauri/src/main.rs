#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct Emoji {
    keywords: Vec<String>,
    char: String,
    fitzpatrick_scale: bool,
    category: String,
}

fn main() {
    tauri::AppBuilder::new()
        .setup(move |webview, _| {
            let mut handle = webview.as_mut();
            let file = File::open(String::from("../build/emoji.json")).unwrap();
            let reader = BufReader::new(file);
            let emoji_list: HashMap<String, Emoji> = serde_json::from_reader(reader).unwrap();
            let curr_dir = env::current_dir().unwrap();
            println!("\x1b[38;5;206m\n{:?}\n\x1b[0m", curr_dir.display());
            tauri::event::listen("search".to_string(), move |data| match data {
                Some(search_term) => {
                    let found_emoji = emoji_list.get(&search_term);
                    match found_emoji {
                        Some(emoji) => {
                            let char = &emoji.char;
                            println!("Sending: {}", char);
                            tauri::event::emit(&mut handle, "result".to_string(), Some(char))
                                .unwrap();
                        }
                        None => {
                            for emoji in emoji_list.values() {
                                if emoji.keywords.contains(&search_term)
                                    || emoji.category == search_term
                                {
                                    println!("Sending: {}", &emoji.char);
                                    tauri::event::emit(
                                        &mut handle,
                                        "result".to_string(),
                                        Some(&emoji.char),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
                None => {}
            });
        })
        .build()
        .run();
}
