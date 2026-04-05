use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeData {
    pub flag: String,
    pub mine: String,
    pub pressed: String,
    pub closed: String,
    pub opened: String,
    pub exploded: String,
}

use std::fs;

impl ThemeData {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .expect("Failed to read theme file");

        serde_json::from_str(&data)
            .expect("Invalid JSON format")
    }
}