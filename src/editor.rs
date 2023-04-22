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
    x: usize,
    y: usize,
}

impl Position {
    fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}

pub struct Editor {
    terminal: Terminal,
    offset: Position,
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
            offset: Position::default(),
            document: Document::open(file_path).unwrap_or_default(),
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default(),
            offset: Position::default(),
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
        let width = self.terminal.cols as usize;
        let start = self.offset.x;
        let end = start + width;
        let line = line.render(0, end);
        print!("{line}");
        execute!(stdout(), MoveToNextLine(1)).unwrap();
    }

    fn display_document(&self) {
        execute!(stdout(), Clear(terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        for row in 0..self.terminal.rows {
            if let Some(line) = self.document.get_line(row as usize + self.offset.y) {
                self.draw_line(line);
            }
        }

        let (curs_x, curs_y) = self.terminal.get_pos();
        execute!(stdout(), MoveTo(curs_x, curs_y)).unwrap();
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
                    if self.terminal.position.x > 0 {
                        self.terminal
                            .position
                            .set_x(self.terminal.position.x.saturating_sub(1));
                    } else {
                        self.offset.set_x(self.offset.x.saturating_sub(1));
                        self.display_document();
                    }
                    Some(())
                }),
                'j' => execute!(stdout(), MoveDown(1)).map_or(None, |_| {
                    let curs_y = self.terminal.position.y.saturating_add(1);
                    let offs_y = self.offset.y.saturating_add(1);

                    if curs_y < self.terminal.rows {
                        self.terminal.position.set_y(curs_y);
                    } else if offs_y < self.document.lines.len() - self.terminal.rows as usize {
                        self.offset.set_y(offs_y);
                        self.display_document();
                    }
                    Some(())
                }),
                'k' => execute!(stdout(), MoveUp(1)).map_or(None, |_| {
                    if self.terminal.position.y > 0 {
                        self.terminal
                            .position
                            .set_y(self.terminal.position.y.saturating_sub(1));
                    } else {
                        self.offset.set_y(self.offset.y.saturating_sub(1));
                        self.display_document();
                    }

                    Some(())
                }),
                'l' => {
                    let curs_x = self.terminal.position.x.saturating_add(1);
                    //let offs_x = self.offset.x.saturating_add(1);

                    let line_index = self.terminal.position.y as usize + self.offset.y;
                    let line_len = self.document.lines[line_index].len();

                    if curs_x < self.terminal.cols && usize::from(curs_x) < line_len {
                        execute!(stdout(), MoveRight(1)).map_or(None, |_| Some(()));
                        self.terminal.position.set_x(curs_x);
                    }
                    Some(())
                }
                '0' => execute!(stdout(), MoveTo(0, self.terminal.position.y)).map_or(None, |_| {
                    self.terminal.position.set_x(0);
                    Some(())
                }),
                '$' => {
                    let last_col = self.terminal.cols - 1;
                    execute!(stdout(), MoveTo(last_col, self.terminal.position.y)).map_or(
                        None,
                        |_| {
                            self.terminal.position.set_x(last_col);
                            Some(())
                        },
                    )
                }
                _ => Some(()),
            },
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => match key {
                'u' => execute!(stdout(), MoveTo(self.terminal.position.x, 0)).map_or(None, |_| {
                    self.terminal.position.set_y(0);
                    Some(())
                }),
                'd' => {
                    let last_row = self.terminal.rows - 1;
                    execute!(stdout(), MoveTo(self.terminal.position.x, last_row)).map_or(
                        None,
                        |_| {
                            self.terminal.position.set_y(last_row);
                            Some(())
                        },
                    )
                }
                'q' => None,
                _ => Some(()),
            },
            _ => Some(()),
        }
    }
}
