use crate::data::{get_mru_file, get_mru_file_string, Emoji};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub fn listen_for_search(mut handle: tauri::WebviewMut) {
    let emojifile = include_str!("../../../build/emoji.json");
    let emoji_list: HashMap<String, Emoji> = serde_json::from_str(emojifile).unwrap();
    tauri::event::listen("search".to_string(), move |data| match data {
        Some(search_term) => {
            let found_emoji = emoji_list.get(&search_term);
            match found_emoji {
                Some(emoji) => {
                    let char = &emoji.char;
                    tauri::event::emit(&mut handle, "result".to_string(), Some(vec![char]))
                        .unwrap();
                }
                None => {
                    let mut related_list = Vec::new();
                    for emoji in emoji_list.values() {
                        if emoji.keywords.contains(&search_term) || emoji.category == search_term {
                            related_list.push(emoji.char.clone());
                        }
                    }
                    tauri::event::emit(&mut handle, "result".to_string(), Some(related_list))
                        .unwrap();
                }
            }
        }
        None => {}
    });
}

#[derive(Serialize, Deserialize)]
struct MRU {
    recent: Vec<String>,
}

pub fn listen_for_selection() {
    tauri::event::listen("select".to_string(), move |data| match data {
        Some(selected) => {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(selected.clone()).unwrap();
            let mru_file = get_mru_file();
            let mru_file_string = fs::read_to_string(&mru_file).unwrap();
            let mut mru_cache: MRU = serde_json::from_str(&mru_file_string).unwrap();
            if mru_cache.recent.len() < 15 {
                // check if emoji is already in the cache
                match mru_cache.recent.iter().position(|x| x == selected.as_str()) {
                    Some(idx) => {
                        // if it is in the cache, remove it from it's current position and place
                        // at the beginning of the cache
                        mru_cache.recent.remove(idx);
                        mru_cache.recent.insert(0, selected);
                    }
                    None => {
                        mru_cache.recent.insert(0, selected);
                    }
                }
                fs::write(&mru_file, serde_json::to_string(&mru_cache).unwrap()).unwrap();
            }
        }
        None => {}
    });
}

pub fn send_init(mut handle: tauri::WebviewMut) {
    let mru_file_string = get_mru_file_string().unwrap();
    tauri::event::emit(&mut handle, "loadCache".to_string(), Some(mru_file_string)).unwrap();
}
