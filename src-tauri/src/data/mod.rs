use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Emoji {
    pub keywords: Vec<String>,
    pub char: String,
    pub fitzpatrick_scale: bool,
    pub category: String,
}
