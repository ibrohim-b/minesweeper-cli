use crate::minesweeper::Minesweeper;
use crate::ui::clear;
use crate::utils::cell::ThemeData;
use crate::Settings;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::size;
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

// board_cols = width * cell_width, total_rows = height + 3 (border top + border bot + status)
fn offsets(ms: &Minesweeper) -> (u16, u16) {
    let (term_cols, term_rows) = size().unwrap_or((80, 24));
    let board_cols = ms.board_display_width() as u16;
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

    let board_cols = ms.board_display_width();
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
        numbers: [" 1 "," 2 "," 3 "," 4 "," 5 "," 6 "," 7 "," 8 "].map(String::from),
    });
    let mut ms = Minesweeper::new(width, height, mines, theme, settings.first_click_safe);
    clear();
    render(&ms);

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => {
                    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                    match key {
                        KeyEvent { code: KeyCode::Up,    modifiers, .. } if modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::SUPER) => ms.move_cursor_to_position((ms.cursor.0, 0)),
                        KeyEvent { code: KeyCode::Down,  modifiers, .. } if modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::SUPER) => ms.move_cursor_to_position((ms.cursor.0, ms.height - 1)),
                        KeyEvent { code: KeyCode::Left,  modifiers, .. } if modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::SUPER) => ms.move_cursor_to_position((0, ms.cursor.1)),
                        KeyEvent { code: KeyCode::Right, modifiers, .. } if modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::SUPER) => ms.move_cursor_to_position((ms.width - 1, ms.cursor.1)),
                        KeyEvent { code: KeyCode::Up    | KeyCode::Char('w'), .. } | KeyEvent { code: KeyCode::Char('k'), .. } if !ctrl => ms.move_cursor(0, -1),
                        KeyEvent { code: KeyCode::Down  | KeyCode::Char('s'), .. } | KeyEvent { code: KeyCode::Char('j'), .. } if !ctrl => ms.move_cursor(0, 1),
                        KeyEvent { code: KeyCode::Left  | KeyCode::Char('a'), .. } | KeyEvent { code: KeyCode::Char('h'), .. } if !ctrl => ms.move_cursor(-1, 0),
                        KeyEvent { code: KeyCode::Right | KeyCode::Char('d'), .. } | KeyEvent { code: KeyCode::Char('l'), .. } if !ctrl => ms.move_cursor(1, 0),
                        KeyEvent { code: KeyCode::Enter | KeyCode::Char(' '), .. } => ms.open(ms.cursor),
                        KeyEvent { code: KeyCode::Char('f') | KeyCode::Char('F'), .. } => ms.toggle_flag(ms.cursor),
                        KeyEvent { code: KeyCode::Char('r') | KeyCode::Char('R'), .. } => { ms.reset(); clear(); },
                        KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc, .. } => return true,
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
