use crate::ui::{center_col, center_row, print_menu, print_theme_preview};
use crate::utils::cell::ThemeData;
use crate::utils::input::prompt;
use crate::Settings;
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::stdout;

pub struct Difficulty {
    pub label: &'static str,
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

pub const DIFFICULTIES: &[Difficulty] = &[
    Difficulty { label: "Easy       9×9   10 mines", width: 9,  height: 9,  mines: 10 },
    Difficulty { label: "Medium   16×16   40 mines", width: 16, height: 16, mines: 40 },
    Difficulty { label: "Hard     30×16   99 mines", width: 30, height: 16, mines: 99 },
    Difficulty { label: "Custom",                    width: 0,  height: 0,  mines: 0  },
];

fn next_key_or_resize(redraw: &mut bool) -> Option<KeyCode> {
    if event::poll(std::time::Duration::from_millis(200)).unwrap() {
        match event::read().unwrap() {
            Event::Key(k) => return Some(k.code),
            Event::Resize(_, _) => { *redraw = true; }
            _ => {}
        }
    }
    None
}

pub fn main_menu() -> Option<&'static str> {
    let items = ["Play", "Settings", "Quit"];
    let mut sel = 0usize;
    let mut redraw = true;
    loop {
        if redraw { print_menu(&items, sel, "Main Menu"); redraw = false; }
        if let Some(key) = next_key_or_resize(&mut redraw) {
            match key {
                KeyCode::Up   | KeyCode::Char('w') => { sel = sel.saturating_sub(1); redraw = true; }
                KeyCode::Down | KeyCode::Char('s') => { sel = (sel + 1).min(items.len() - 1); redraw = true; }
                KeyCode::Enter => return Some(items[sel]),
                KeyCode::Char('q') | KeyCode::Esc  => return Some("Quit"),
                _ => {}
            }
        }
    }
}

pub fn difficulty_menu() -> Option<(usize, usize, usize)> {
    let labels: Vec<&str> = DIFFICULTIES.iter().map(|d| d.label).collect();
    let mut sel = 0usize;
    let mut redraw = true;
    loop {
        if redraw { print_menu(&labels, sel, "Select Difficulty"); redraw = false; }
        if let Some(key) = next_key_or_resize(&mut redraw) {
            match key {
                KeyCode::Up   | KeyCode::Char('w') => { sel = sel.saturating_sub(1); redraw = true; }
                KeyCode::Down | KeyCode::Char('s') => { sel = (sel + 1).min(DIFFICULTIES.len() - 1); redraw = true; }
                KeyCode::Enter => {
                    let d = &DIFFICULTIES[sel];
                    if d.label == "Custom" { return custom_input(); }
                    return Some((d.width, d.height, d.mines));
                }
                KeyCode::Char('q') | KeyCode::Esc => return None,
                _ => {}
            }
        }
    }
}

pub fn custom_input() -> Option<(usize, usize, usize)> {
    disable_raw_mode().unwrap();

    let box_width = 20u16;
    let box_height = 6u16;
    let (term_cols, term_rows) = size().unwrap_or((80, 24));
    let col = term_cols.saturating_sub(box_width) / 2;
    let row = term_rows.saturating_sub(box_height) / 2;

    execute!(stdout(), Clear(ClearType::All), cursor::Show, cursor::MoveTo(col, row)).unwrap();

    execute!(stdout(), SetForegroundColor(Color::Cyan), Print("╔══ Custom Game ══╗"), ResetColor).unwrap();
    execute!(stdout(), cursor::MoveTo(col, row + 5), SetForegroundColor(Color::Cyan), Print("╚════════════════╝"), ResetColor).unwrap();

    // move cursor to input lines and show prompts
    execute!(stdout(), cursor::MoveTo(col, row + 1)).unwrap();
    let width  = prompt_at("  Width  ", 9,  col, row + 1);
    let height = prompt_at("  Height ", 9,  col, row + 2);
    let max_mines = (width * height).saturating_sub(1);
    let mines  = prompt_at("  Mines  ", 10, col, row + 3).min(max_mines);

    enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    Some((width, height, mines))
}

fn prompt_at(label: &str, default: usize, col: u16, row: u16) -> usize {
    execute!(stdout(), cursor::MoveTo(col, row)).unwrap();
    prompt(label, default)
}

pub fn settings_menu(settings: &mut Settings, themes: &[ThemeData]) {
    let mut sel = 0usize;
    let mut redraw = true;
    loop {
        if redraw {
            let theme = themes.get(settings.theme_idx);
            let theme_name  = theme.map_or("None", |t| &t.name);
            let theme_label = format!("Theme: {}", theme_name);
            let safe_label  = format!("First Click Safe: {}", if settings.first_click_safe { "ON " } else { "OFF" });
            let items = [theme_label.as_str(), safe_label.as_str(), "Back"];
            print_menu(&items, sel, "Settings");

            // preview below the menu: 3 rows (header + labels + cells)
            // menu occupies: 3 title + 1 blank + 1 subtitle + 1 blank + 3 items + 1 blank + 1 hint = 11 rows
            // add 2 blank rows gap then draw preview centered
            if let Some(t) = theme {
                let preview_cols = 6 * (t.closed.chars().count() as u16 + 1); // 6 cells
                let preview_col = center_col(preview_cols);
                let (_, term_rows) = crossterm::terminal::size().unwrap_or((80, 24));
                let menu_rows = 11u16;
                let preview_row = center_row(menu_rows + 2 + 3) + menu_rows + 2;
                let preview_row = preview_row.min(term_rows.saturating_sub(3));
                print_theme_preview(t, preview_col, preview_row);
            }
            redraw = false;
        }
        if let Some(key) = next_key_or_resize(&mut redraw) {
            match key {
                KeyCode::Up   | KeyCode::Char('w') => { sel = sel.saturating_sub(1); redraw = true; }
                KeyCode::Down | KeyCode::Char('s') => { sel = (sel + 1).min(2); redraw = true; }
                KeyCode::Enter => match sel {
                    0 => { settings.theme_idx = (settings.theme_idx + 1) % themes.len().max(1); redraw = true; }
                    1 => { settings.first_click_safe = !settings.first_click_safe; redraw = true; }
                    _ => return,
                },
                KeyCode::Char('q') | KeyCode::Esc => return,
                _ => {}
            }
        }
    }
}
