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
        self.terminal.exit();
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
                _ => Some(()),
            },
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => match key {
                'q' => None,
                _ => Some(()),
            },
            _ => Some(()),
        }
    }
}
