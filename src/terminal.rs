use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }
}

pub struct Terminal {
    pub rows: u16,
    pub cols: u16,
    pub position: Position,
}

impl Default for Terminal {
    fn default() -> Self {
        let Ok((cols, rows)) = crossterm::terminal::size() else { panic!(); };
        Self {
            rows,
            cols,
            position: Position::default(),
        }
    }
}

impl Terminal {
    pub fn set_size(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
    }

    pub fn get_pos(&self) -> (u16, u16) {
        (self.position.x, self.position.y)
    }

    pub fn setup(&self, is_empty: bool) {
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        if is_empty {
            self.draw_welcome();
        }
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
    }

    fn draw_welcome(&self) {
        let message = format!("Welcome to Text -- v{VERSION}");
        let mess_len = message.len();
        let width: usize = self.cols.into();
        let padding = width.saturating_sub(mess_len) / 2;
        let spaces = " ".repeat(padding);
        let mut message = format!("{spaces}{message}");
        message.truncate(width);
        execute!(stdout(), MoveTo(0, self.rows / 3)).unwrap();
        print!("{message}");
    }

    pub fn exit() {
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
