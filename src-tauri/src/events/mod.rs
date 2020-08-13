use crate::data::Emoji;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn listen_for_search(mut handle: tauri::WebviewMut) {
    let file = File::open(String::from("../build/emoji.json")).unwrap();
    let reader = BufReader::new(file);
    let emoji_list: HashMap<String, Emoji> = serde_json::from_reader(reader).unwrap();
    tauri::event::listen("search".to_string(), move |data| match data {
        Some(search_term) => {
            let found_emoji = emoji_list.get(&search_term);
            match found_emoji {
                Some(emoji) => {
                    let char = &emoji.char;
                    tauri::event::emit(&mut handle, "result".to_string(), Some(char)).unwrap();
                }
                None => {
                    for emoji in emoji_list.values() {
                        if emoji.keywords.contains(&search_term) || emoji.category == search_term {
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
}

pub fn listen_for_selection(handle: tauri::WebviewMut) {
    tauri::event::listen("select".to_string(), move |data| match data {
        Some(selected) => {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(selected).unwrap();
        }
        None => {}
    });
}
