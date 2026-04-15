use crate::ui::menu::{difficulty_menu, main_menu, settings_menu};
use crate::ui::game::play;
use crate::utils::cell::load_themes;
use crate::utils::prefs::Prefs;
use crossterm::style::ResetColor;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io;
use std::io::stdout;

pub mod minesweeper;
pub mod utils;
pub mod ui;

#[derive(Clone)]
pub struct Settings {
    pub theme_idx: usize,
    pub first_click_safe: bool,
    pub last_difficulty: usize,
}

impl Settings {
    fn from_prefs(p: &Prefs) -> Self {
        Self { theme_idx: p.theme_idx, first_click_safe: p.first_click_safe, last_difficulty: p.last_difficulty }
    }

    fn to_prefs(&self) -> Prefs {
        Prefs { theme_idx: self.theme_idx, first_click_safe: self.first_click_safe, last_difficulty: self.last_difficulty }
    }
}

fn main() -> io::Result<()> {
    let themes_dir = std::path::Path::new("themes");
    let themes = load_themes(themes_dir);
    let prefs = Prefs::load();
    let mut settings = Settings::from_prefs(&prefs);

    enable_raw_mode()?;
    ui::clear();

    loop {
        match main_menu() {
            Some("Play") => {
                if let Some((idx, w, h, m)) = difficulty_menu(settings.last_difficulty) {
                    settings.last_difficulty = idx;
                    settings.to_prefs().save();
                    play(w, h, m, &settings, &themes);
                }
            }
            Some("Settings") => {
                settings_menu(&mut settings, &themes);
                settings.to_prefs().save();
            }
            _ => break,
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), cursor::Show, ResetColor, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Thanks for playing!");
    Ok(())
}
