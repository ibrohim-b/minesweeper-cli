pub mod menu;
pub mod game;

use crate::utils::cell::ThemeData;
use crossterm::style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::{stdout, Write};

pub fn clear() {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0), cursor::Hide).unwrap();
}

/// Returns the column that centers `content_width` chars in the terminal.
pub fn center_col(content_width: u16) -> u16 {
    let (term_cols, _) = size().unwrap_or((80, 24));
    term_cols.saturating_sub(content_width) / 2
}

/// Returns the row that vertically centers `content_height` lines in the terminal.
pub fn center_row(content_height: u16) -> u16 {
    let (_, term_rows) = size().unwrap_or((80, 24));
    term_rows.saturating_sub(content_height) / 2
}

pub fn print_title(col: u16, row: u16) {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(col, row),     SetForegroundColor(Color::Cyan), Print("╔═══════════════════════════════╗"), ResetColor).unwrap();
    execute!(out, cursor::MoveTo(col, row + 1), SetForegroundColor(Color::Cyan), Print("║     M I N E S W E E P E R     ║"), ResetColor).unwrap();
    execute!(out, cursor::MoveTo(col, row + 2), SetForegroundColor(Color::Cyan), Print("╚═══════════════════════════════╝"), ResetColor).unwrap();
}

// title = 3 rows + 1 blank + 1 subtitle + 1 blank + items + 1 blank + hint
pub fn print_menu(items: &[&str], selected: usize, title: &str) {
    clear();

    // widest line among all content
    let hint = "↑↓ Navigate   Enter Select   Q Back";
    let title_box_width = 33u16; // inner ╔...╗ width
    let content_width = items.iter()
        .map(|s| s.chars().count() as u16 + 4) // "  ▶ " prefix + "  " suffix
        .chain([hint.chars().count() as u16, title.chars().count() as u16 + 2, title_box_width])
        .max()
        .unwrap_or(40);

    let total_rows = 3 + 1 + 1 + 1 + items.len() as u16 + 1 + 1;
    let col = center_col(content_width);
    let mut row = center_row(total_rows);

    print_title(col, row);
    row += 4; // 3 title lines + 1 blank

    // section title
    execute!(
        stdout(),
        cursor::MoveTo(col, row),
        SetForegroundColor(Color::White),
        SetAttribute(Attribute::Bold),
        Print(title),
        SetAttribute(Attribute::Reset),
        ResetColor,
    ).unwrap();
    row += 2; // title + blank

    for (i, item) in items.iter().enumerate() {
        execute!(stdout(), cursor::MoveTo(col, row + i as u16)).unwrap();
        if i == selected {
            execute!(
                stdout(),
                SetForegroundColor(Color::Black),
                crossterm::style::SetBackgroundColor(Color::Cyan),
                Print(format!("▶ {}  ", item)),
                ResetColor,
            ).unwrap();
        } else {
            execute!(
                stdout(),
                SetForegroundColor(Color::Grey),
                Print(format!("  {}", item)),
                ResetColor,
            ).unwrap();
        }
    }

    row += items.len() as u16 + 1;
    execute!(
        stdout(),
        cursor::MoveTo(col, row),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    ).unwrap();

    stdout().flush().unwrap();
}

pub fn print_theme_preview(theme: &ThemeData, col: u16, row: u16) {
    // two rows: labels on top, cells on bottom
    let cells = [
        ("closed",   &theme.closed),
        ("open",     &theme.opened),
        ("flag",     &theme.flag),
        ("mine",     &theme.mine),
        ("boom",     &theme.exploded),
        ("press",    &theme.pressed),
        ("numbers",  &theme.numbers[0]),
    ];

    let cell_w = theme.closed.chars().count() as u16;
    let col_w  = cell_w.max(5); // at least wide enough for the label

    // header
    execute!(
        stdout(),
        cursor::MoveTo(col, row),
        SetForegroundColor(Color::DarkGrey),
        Print("Preview:"),
        ResetColor,
    ).unwrap();

    // label row
    for (i, (label, _)) in cells.iter().enumerate() {
        let x = col + i as u16 * (col_w + 1);
        execute!(
            stdout(),
            cursor::MoveTo(x, row + 1),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{:width$}", label, width = col_w as usize)),
            ResetColor,
        ).unwrap();
    }

    // cell row
    for (i, (_, symbol)) in cells.iter().enumerate() {
        let x = col + i as u16 * (col_w + 1);
        execute!(
            stdout(),
            cursor::MoveTo(x, row + 2),
            SetForegroundColor(Color::White),
            Print(format!("{:width$}", symbol, width = col_w as usize)),
            ResetColor,
        ).unwrap();
    }

    stdout().flush().unwrap();
}
