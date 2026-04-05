use crossterm::event::{self, Event, KeyCode};
use std::io::{self, Write};

pub fn next_key() -> Option<KeyCode> {
    if event::poll(std::time::Duration::from_millis(200)).unwrap() {
        if let Event::Key(k) = event::read().unwrap() {
            return Some(k.code);
        }
    }
    None
}

pub fn prompt(label: &str, default: usize) -> usize {
    print!("{} [{}]: ", label, default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(default)
}
