use crate::minesweeper::Minesweeper;
use crate::ui::clear;
use crate::utils::cell::ThemeData;
use crate::Settings;
use crossterm::event::{self, Event, KeyCode};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::size;
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

// board_cols = width * cell_width, total_rows = height + 3 (border top + border bot + status)
fn offsets(ms: &Minesweeper) -> (u16, u16) {
    let (term_cols, term_rows) = size().unwrap_or((80, 24));
    let board_cols = (ms.width * ms.cell_width()) as u16;
    let total_rows = (ms.height + 3) as u16;
    let col = term_cols.saturating_sub(board_cols) / 2;
    let row = term_rows.saturating_sub(total_rows) / 2;
    (col, row)
}

pub fn render(ms: &Minesweeper) {
    let mut out = stdout();
    let (col, row) = offsets(ms);

    let flags = ms.flagged_cells_count();
    let mines = ms.mines_count();

    let board_cols = ms.width * ms.cell_width();
    // inner width between the corner chars
    let inner = board_cols.saturating_sub(2);
    let label = " MINESWEEPER ";
    // pad label with '═' on both sides to fill inner width
    let label_len = label.chars().count();
    let total_pad = inner.saturating_sub(label_len);
    let pad_left  = total_pad / 2;
    let pad_right = total_pad - pad_left;
    let top = format!("╔{}{}{}╗", "═".repeat(pad_left), label, "═".repeat(pad_right));
    let bot = format!("╚{}╝", "═".repeat(inner));

    // top border
    execute!(
        out,
        cursor::MoveTo(col, row),
        cursor::Hide,
        SetForegroundColor(Color::Cyan),
        Print(&top),
        ResetColor,
    ).unwrap();

    // board rows
    for y in 0..ms.height {
        execute!(out, cursor::MoveTo(col, row + 1 + y as u16)).unwrap();
        execute!(out, Print(ms.render_row(y))).unwrap();
    }

    // bottom border
    execute!(
        out,
        cursor::MoveTo(col, row + 1 + ms.height as u16),
        SetForegroundColor(Color::Cyan),
        Print(&bot),
        ResetColor,
    ).unwrap();

    // status line
    execute!(out, cursor::MoveTo(col, row + 2 + ms.height as u16)).unwrap();
    if ms.is_game_over() {
        execute!(out, SetForegroundColor(Color::Red), Print("💥 Game Over!  R restart  Q menu"), ResetColor).unwrap();
    } else if ms.is_game_finished() {
        execute!(out, SetForegroundColor(Color::Green), Print("🎉 You Win!    R restart  Q menu"), ResetColor).unwrap();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::Yellow),   Print(format!("💣 {}/{}  ", flags, mines)),
            ResetColor,
            SetForegroundColor(Color::DarkGrey), Print("WASD Move  Enter Open  F Flag  R Restart  Q Menu"),
            ResetColor,
        ).unwrap();
    }

    out.flush().unwrap();
}

pub fn play(width: usize, height: usize, mines: usize, settings: &Settings, themes: &[ThemeData]) -> bool {
    let theme = themes.get(settings.theme_idx).cloned().unwrap_or_else(|| ThemeData {
        name: "Fallback".to_string(),
        closed: " # ".to_string(), opened: " . ".to_string(), flag: " F ".to_string(),
        mine: " * ".to_string(), exploded: " X ".to_string(), pressed: " _ ".to_string(),
    });
    let mut ms = Minesweeper::new(width, height, mines, theme, settings.first_click_safe);
    clear();
    render(&ms);

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Up    | KeyCode::Char('w') => ms.move_cursor(0, -1),
                        KeyCode::Down  | KeyCode::Char('s') => ms.move_cursor(0, 1),
                        KeyCode::Left  | KeyCode::Char('a') => ms.move_cursor(-1, 0),
                        KeyCode::Right | KeyCode::Char('d') => ms.move_cursor(1, 0),
                        KeyCode::Enter | KeyCode::Char(' ') => ms.open(ms.cursor),
                        KeyCode::Char('f') | KeyCode::Char('F') => ms.toggle_flag(ms.cursor),
                        KeyCode::Char('r') | KeyCode::Char('R') => { ms.reset(); clear(); },
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => return true,
                        _ => {}
                    }
                    render(&ms);
                }
                Event::Resize(_, _) => { clear(); render(&ms); }
                _ => {}
            }
        }
    }
}
