use crate::VERSION;
use crossterm::cursor::{Hide, MoveTo, MoveToNextLine, Show};
use crossterm::execute;
use std::io::stdout;

pub struct Terminal {
    rows: u16,
    cols: u16,
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
        crossterm::terminal::enable_raw_mode().unwrap();

        self.draw_rows();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        execute!(stdout(), Show).unwrap();
    }

    fn draw_rows(&self) {
        let height = self.rows;
        for row in 0..height {
            if row == height / 3 {
                print!("Welcome to Text -- version {}", VERSION);
                execute!(stdout(), MoveToNextLine(1)).unwrap();
            } else {
                println!("~\r");
            }
        }
    }

    pub fn exit(&self) {
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
