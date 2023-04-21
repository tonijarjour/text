use crate::terminal::Terminal;
use crossterm::cursor::{MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
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

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        self.terminal.setup();
        self.read_event();
        Terminal::exit();
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
