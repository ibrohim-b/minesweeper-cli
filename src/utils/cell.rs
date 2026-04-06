use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeData {
    pub name: String,
    pub closed: String,
    pub opened: String,
    pub flag: String,
    pub mine: String,
    pub exploded: String,
    pub pressed: String,
    pub numbers: [String; 8],
}

impl ThemeData {
    pub fn render_cell(&self, cell: Cell) -> String {
        match cell {
            Cell::Num(n)   => self.numbers[n as usize - 1].clone(),
            Cell::Closed   => self.closed.clone(),
            Cell::Opened   => self.opened.clone(),
            Cell::Flag     => self.flag.clone(),
            Cell::Mine     => self.mine.clone(),
            Cell::Exploded => self.exploded.clone(),
            Cell::Pressed  => self.pressed.clone(),
        }
    }
}

pub fn load_themes(dir: &Path) -> Vec<ThemeData> {
    let Ok(entries) = std::fs::read_dir(dir) else { return vec![] };
    let mut themes: Vec<ThemeData> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |x| x == "json"))
        .filter_map(|e| {
            let text = std::fs::read_to_string(e.path()).ok()?;
            serde_json::from_str(&text).ok()
        })
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Num(u8),
    Flag,
    Mine,
    Pressed,
    Closed,
    Opened,
    Exploded,
}
