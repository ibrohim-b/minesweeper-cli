use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prefs {
    pub theme_idx: usize,
    pub first_click_safe: bool,
    pub last_difficulty: usize, // index into DIFFICULTIES
}

impl Default for Prefs {
    fn default() -> Self {
        Self { theme_idx: 0, first_click_safe: true, last_difficulty: 0 }
    }
}

fn prefs_path() -> PathBuf {
    let base = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("minesweeper").join("prefs.json")
}

impl Prefs {
    pub fn load() -> Self {
        let path = prefs_path();
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = prefs_path();
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}
