use crate::document::Document;
use crate::document::Line;
use crate::terminal::Terminal;
use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveUp};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{self, Clear};
use std::io::stdout;

#[derive(Default)]
struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    fn set_y(&mut self, y: u16) {
        self.y = y;
    }
}

pub struct Editor {
    terminal: Terminal,
    position: Position,
    document: Document,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&String> for Editor {
    fn from(file_path: &String) -> Self {
        Self {
            terminal: Terminal::default(),
            position: Position::default(),
            document: Document::open(file_path).unwrap_or_default(),
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default(),
            position: Position::default(),
            document: Document::default(),
        }
    }

    pub fn run(&mut self) {
        self.terminal.setup();
        if !self.document.lines.is_empty() {
            self.display_document();
        }
        self.read_event();
        Terminal::exit();
    }

    fn draw_line(&self, line: &Line) {
        let end = self.terminal.cols as usize;
        let line = line.render(0, end);
        print!("{line}");
        execute!(stdout(), MoveToNextLine(1)).unwrap();
    }

    fn display_document(&self) {
        execute!(stdout(), Clear(terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        for row in 0..self.terminal.rows {
            if let Some(line) = self.document.get_line(row as usize) {
                self.draw_line(line);
            }
        }
        execute!(stdout(), MoveTo(0, 0)).unwrap();
    }

    fn read_event(&mut self) {
        while let Ok(event) = event::read() {
            match event {
                Event::Key(keycode) => {
                    if self.match_keycode(keycode).is_none() {
                        break;
                    }
                }
                Event::Resize(rows, cols) => self.terminal.set_size(rows, cols),
                _ => (),
            }
        }
    }

    fn match_keycode(&mut self, keycode: KeyEvent) -> Option<()> {
        match keycode {
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
                ..
            } => match key {
                'h' => execute!(stdout(), MoveLeft(1)).map_or(None, |_| {
                    self.position.set_x(self.position.x.saturating_sub(1));
                    Some(())
                }),
                'j' => execute!(stdout(), MoveDown(1)).map_or(None, |_| {
                    let y = self.position.y.saturating_add(1);
                    if y < self.terminal.rows {
                        self.position.set_y(y);
                    }
                    Some(())
                }),
                'k' => execute!(stdout(), MoveUp(1)).map_or(None, |_| {
                    self.position.set_y(self.position.y.saturating_sub(1));
                    Some(())
                }),
                'l' => execute!(stdout(), MoveRight(1)).map_or(None, |_| {
                    let x = self.position.x.saturating_add(1);
                    if x < self.terminal.rows {
                        self.position.set_x(x);
                    }
                    Some(())
                }),
                '0' => execute!(stdout(), MoveTo(0, self.position.y)).map_or(None, |_| {
                    self.position.set_x(0);
                    Some(())
                }),
                '$' => {
                    let last_col = self.terminal.cols - 1;
                    execute!(stdout(), MoveTo(last_col, self.position.y)).map_or(None, |_| {
                        self.position.set_x(last_col);
                        Some(())
                    })
                }
                _ => Some(()),
            },
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => match key {
                'u' => execute!(stdout(), MoveTo(self.position.x, 0)).map_or(None, |_| {
                    self.position.set_y(0);
                    Some(())
                }),
                'd' => {
                    let last_row = self.terminal.rows - 1;
                    execute!(stdout(), MoveTo(self.position.x, last_row)).map_or(None, |_| {
                        self.position.set_y(last_row);
                        Some(())
                    })
                }
                'q' => None,
                _ => Some(()),
            },
            _ => Some(()),
        }
    }
}
