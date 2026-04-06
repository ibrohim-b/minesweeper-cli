use crate::utils::cell::{Cell, ThemeData};
use crate::utils::random::random_range;
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use std::{collections::HashSet, fmt::Display};
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct Minesweeper {
    pub width: usize,
    pub height: usize,
    pub cursor: Position,
    pub theme: ThemeData,
    pub first_click_safe: bool,
    mines: HashSet<Position>,
    mines_count: usize,
    open_cells: HashSet<Position>,
    flagged_cells: HashSet<Position>,
    depressed_cells: HashSet<Position>,
    game_over: bool,
    first_click: bool,
}

type Position = (usize, usize);

impl Minesweeper {
    pub fn new(
        width: usize,
        height: usize,
        mines_count: usize,
        theme: ThemeData,
        first_click_safe: bool,
    ) -> Self {
        Self {
            width,
            height,
            cursor: (0, 0),
            theme,
            first_click_safe,
            mines: HashSet::new(),
            mines_count,
            open_cells: HashSet::new(),
            flagged_cells: HashSet::new(),
            depressed_cells: HashSet::new(),
            game_over: false,
            first_click: true,
        }
    }

    pub fn reset(&mut self) {
        self.cursor = (0, 0);
        self.mines = HashSet::new();
        self.open_cells = HashSet::new();
        self.flagged_cells = HashSet::new();
        self.depressed_cells = HashSet::new();
        self.game_over = false;
        self.first_click = true;
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn mines_count(&self) -> usize {
        self.mines_count
    }

    pub fn flagged_cells_count(&self) -> usize {
        self.flagged_cells.len()
    }

    pub fn is_game_finished(&self) -> bool {
        self.open_cells.len() + self.mines.len() == self.width * self.height
    }

    pub fn open(&mut self, pos: Position) {
        if self.is_game_over() || self.is_game_finished() || self.flagged_cells.contains(&pos) {
            return;
        }

        if self.first_click {
            self.first_click = false;
            let excluded = if self.first_click_safe {
                let mut zone: HashSet<Position> = self.iter_neighbors(pos).collect();
                zone.insert(pos);
                Some(zone)
            } else {
                None
            };
            self.mines = Self::gen_rand_mines(self.width, self.height, self.mines_count, excluded);
        }

        let adj_mines = self.adjacent_mines_count(pos);
        let adj_flags = self.adjacent_flags_count(pos);

        if self.open_cells.contains(&pos) {
            if adj_mines == adj_flags {
                self.depressed_cells.clear();
                self.open_closed_neighbors(pos);
            } else if adj_flags > 0 && adj_flags < adj_mines {
                self.depressed_cells.clear();
                self.depress_neighbors(pos);
            } else {
                self.depressed_cells.clear();
            }
            return;
        }
        self.depressed_cells.clear();

        self.open_cells.insert(pos);

        if self.mines.contains(&pos) {
            self.game_over = true;
            return;
        }

        if adj_mines == 0 {
            let neighbors: Vec<Position> = self.iter_neighbors(pos).collect();
            for neighbor in neighbors {
                if !self.open_cells.contains(&neighbor) && !self.flagged_cells.contains(&neighbor) {
                    self.flood_fill(neighbor);
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.is_game_over() || self.is_game_finished() || self.open_cells.contains(&pos) {
            return;
        }

        if !self.flagged_cells.contains(&pos) {
            self.flagged_cells.insert(pos);
            return;
        }

        self.flagged_cells.remove(&pos);
    }

    pub fn clear_depressed_cells(&mut self) {
        if self.depressed_cells.len() < 1 {
            return;
        }

        self.depressed_cells.clear();
    }

    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        self.depressed_cells.clear();
        let new_x = self.cursor.0 as i32 + dx;
        let new_y = self.cursor.1 as i32 + dy;
        self.cursor.0 = new_x.clamp(0, self.width as i32 - 1) as usize;
        self.cursor.1 = new_y.clamp(0, self.height as i32 - 1) as usize;
    }

    pub fn move_cursor_to_position(&mut self, pos: Position) {
        self.cursor = pos;
    }

    fn gen_rand_mines(
        width: usize,
        height: usize,
        mines_count: usize,
        excluded: Option<HashSet<Position>>,
    ) -> HashSet<Position> {
        let mut mines = HashSet::new();
        while mines.len() < mines_count {
            let pos = (random_range(0, width), random_range(0, height));
            if excluded.as_ref().map_or(true, |e| !e.contains(&pos)) {
                mines.insert(pos);
            }
        }
        mines
    }

    fn open_closed_neighbors(&mut self, pos: Position) {
        let neighbors: Vec<Position> = self.iter_neighbors(pos).collect();
        for neighbor in neighbors {
            if !self.open_cells.contains(&neighbor) && !self.flagged_cells.contains(&neighbor) {
                self.flood_fill(neighbor);
            }
        }
    }

    fn flood_fill(&mut self, pos: Position) {
        if self.open_cells.contains(&pos) || self.flagged_cells.contains(&pos) {
            return;
        }
        self.open_cells.insert(pos);
        if self.mines.contains(&pos) {
            self.game_over = true;
            return;
        }
        if self.adjacent_mines_count(pos) == 0 {
            self.open_closed_neighbors(pos);
        }
    }

    fn depress_neighbors(&mut self, pos: Position) {
        for neighbor in self.iter_neighbors(pos) {
            if !self.open_cells.contains(&neighbor) && !self.flagged_cells.contains(&neighbor) {
                self.depress(neighbor);
            }
        }
    }

    fn depress(&mut self, pos: Position) {
        if self.depressed_cells.contains(&pos) {
            return;
        }
        self.depressed_cells.insert(pos);
    }

    fn adjacent_mines_count(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    fn adjacent_flags_count(&self, pos: (usize, usize)) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.flagged_cells.contains(pos))
            .count() as u8
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> + use<> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }
}

impl Minesweeper {
    pub fn cell_width(&self) -> usize {
        self.theme.closed.chars().count()
    }

    /// Actual terminal columns occupied by one cell (emojis are 2-wide).
    pub fn cell_display_width(&self) -> usize {
        UnicodeWidthStr::width(self.theme.closed.as_str())
    }

    /// Actual terminal columns occupied by the full board.
    pub fn board_display_width(&self) -> usize {
        self.width * self.cell_display_width()
    }

    pub fn render_row(&self, y: usize) -> String {
        let text_theme = self.cell_display_width() == self.cell_width();
        let mut row = String::new();
        for x in 0..self.width {
            let pos: Position = (x, y);
            let is_cursor = pos == self.cursor;
            if is_cursor {
                row.push_str(&format!("{}", SetBackgroundColor(Color::White)));
            }
            let cell = if !self.open_cells.contains(&pos) {
                if self.game_over && self.mines.contains(&pos) {
                    Cell::Mine
                } else if self.flagged_cells.contains(&pos) {
                    Cell::Flag
                } else if self.depressed_cells.contains(&pos) {
                    Cell::Pressed
                } else {
                    Cell::Closed
                }
            } else if self.mines.contains(&pos) {
                Cell::Exploded
            } else {
                let n = self.adjacent_mines_count(pos);
                if n == 0 { Cell::Opened } else { Cell::Num(n) }
            };
            if text_theme {
                if let Cell::Num(n) = cell {
                    let color = Self::num_color(n);
                    row.push_str(&format!("{}{}{}", SetForegroundColor(color), self.theme.render_cell(cell), ResetColor));
                    if is_cursor { row.push_str(&format!("{}", ResetColor)); }
                    continue;
                }
            }
            row.push_str(&self.theme.render_cell(cell));
            if is_cursor {
                row.push_str(&format!("{}", ResetColor));
            }
        }
        row
    }

    fn num_color(n: u8) -> Color {
        match n {
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Red,
            4 => Color::DarkBlue,
            5 => Color::Rgb { r: 128, g: 0, b: 0 },   // burgundy
            6 => Color::Cyan,
            7 => Color::Magenta,                        // violet
            _ => Color::Grey,
        }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos: Position = (x, y);
                let is_cursor = pos == self.cursor;
                if is_cursor {
                    write!(f, "{}", SetBackgroundColor(Color::White))?;
                }
                let cell = if !self.open_cells.contains(&pos) {
                    if self.game_over && self.mines.contains(&pos) {
                        Cell::Mine
                    } else if self.flagged_cells.contains(&pos) {
                        Cell::Flag
                    } else if self.depressed_cells.contains(&pos) {
                        Cell::Pressed
                    } else {
                        Cell::Closed
                    }
                } else if self.mines.contains(&pos) {
                    Cell::Exploded
                } else {
                    let n = self.adjacent_mines_count(pos);
                    if n == 0 { Cell::Opened } else { Cell::Num(n) }
                };
                write!(f, "{}", self.theme.render_cell(cell))?;

                if is_cursor {
                    write!(f, "{}", ResetColor)?;
                }
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}
