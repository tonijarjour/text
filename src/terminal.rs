use crate::VERSION;
use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::execute;
use std::io::stdout;

pub struct Terminal {
    pub rows: u16,
    pub cols: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Terminal {
    pub fn new() -> Self {
        let Ok((cols, rows)) = crossterm::terminal::size() else { panic!(); };
        Self { rows, cols }
    }

    pub fn set_size(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
    }

    pub fn setup(&self) {
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        execute!(stdout(), Hide).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        self.draw_rows();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        execute!(stdout(), Show).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
    }

    fn draw_rows(&self) {
        let height = self.rows;
        for row in 0..height {
            if row == height / 3 {
                self.draw_welcome();
            } else {
                print!("~");
            }
            execute!(stdout(), MoveToNextLine(1)).unwrap();
        }
    }

    fn draw_welcome(&self) {
        let message = format!("Welcome to Text -- Version {VERSION}");
        let mess_len = message.len();
        let width: usize = self.cols.into();
        let padding = width.saturating_sub(mess_len) / 2;
        let spaces = " ".repeat(padding);
        let mut message = format!("~{spaces}{message}");
        message.truncate(width);
        print!("{message}");
    }

    pub fn exit(&self) {
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
