use crate::terminal::Terminal;
use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::{stdout, Write};

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default(),
        }
    }

    pub fn run(&mut self) {
        self.setup();
        self.read_event();
        Self::exit();
    }

    fn read_event(&mut self) {
        while let Ok(event) = event::read() {
            match event {
                Event::Key(keycode) => {
                    if Self::match_keycode(keycode).is_none() {
                        break;
                    }
                }
                Event::Resize(rows, cols) => self.terminal.set_size(rows, cols),
                _ => (),
            }
        }
    }

    const fn match_keycode(keycode: KeyEvent) -> Option<()> {
        match keycode {
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(()),
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

    fn setup(&self) {
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
        execute!(stdout(), MoveTo(0, 0));

        self.draw_rows();
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.rows {
            println!("~\r");
        }
    }

    fn exit() {
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
