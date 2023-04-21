use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::{stdout, Write};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run() {
        Self::setup();
        Self::read_keys();
        Self::exit();
    }

    fn read_keys() {
        while let Ok(Event::Key(keycode)) = event::read() {
            if Self::match_keycode(keycode).is_none() {
                break;
            }
        }
    }

    fn match_keycode(keycode: KeyEvent) -> Option<()> {
        match keycode {
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                print!("{key}");
                stdout().flush().unwrap();
                Some(())
            }
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

    fn setup() {
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
    }

    fn exit() {
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
