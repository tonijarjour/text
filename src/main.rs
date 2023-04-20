use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::{stdout, Write};

fn main() {
    execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();

    loop {
        if let Event::Key(keycode) = event::read().unwrap() {
            match keycode {
                KeyEvent {
                    code: KeyCode::Char(key),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    print!("{}", key);
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
    }

    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
    crossterm::terminal::disable_raw_mode().ok();
}
