use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::{stdout, Write};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

        while let Ok(Event::Key(keycode)) = event::read() {
                match keycode {
                    KeyEvent {
                        code: KeyCode::Char(key),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        print!("{key}");
                        stdout().flush().unwrap()
                    }
                    KeyEvent {
                        code: KeyCode::Char(key),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => match key {
                        'q' => break,
                        _ => (),
                    },
                    _ => (),
                }
            }

        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
