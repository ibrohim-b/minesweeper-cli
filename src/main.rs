use crate::ui::menu::{difficulty_menu, main_menu, settings_menu};
use crate::ui::game::play;
use crate::utils::cell::load_themes;
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
}

fn main() -> io::Result<()> {
    let themes_dir = std::path::Path::new("themes");
    let themes = load_themes(themes_dir);
    let mut settings = Settings { theme_idx: 0, first_click_safe: true };

    enable_raw_mode()?;
    ui::clear();

    loop {
        match main_menu() {
            Some("Play") => {
                if let Some((w, h, m)) = difficulty_menu() {
                    play(w, h, m, &settings, &themes);
                }
            }
            Some("Settings") => settings_menu(&mut settings, &themes),
            _ => break,
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), cursor::Show, ResetColor, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Thanks for playing!");
    Ok(())
}
