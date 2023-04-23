use crate::document::Document;
use crate::document::Line;
use crate::terminal::Terminal;
use crossterm::cursor::{self, Hide, MoveLeft, MoveRight, MoveTo, MoveToNextLine, Show};
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

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    offset: Position,
    document: Document,
}

impl From<&str> for Editor {
    fn from(file_path: &str) -> Self {
        Self {
            terminal: Terminal::default(),
            offset: Position::default(),
            document: Document::open(file_path).unwrap_or_default(),
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        let is_empty = self.document.is_empty();
        self.terminal.setup(is_empty);
        if !is_empty {
            self.display_document();
        }
        self.read_event();
        Terminal::exit();
    }

    fn draw_line(&self, line: &Line) {
        let start = self.offset.x;
        let end = start + self.terminal.cols as usize;
        let line = line.render(start, end);
        print!("{line}");
        execute!(stdout(), MoveToNextLine(1)).unwrap();
    }

    fn display_document(&self) {
        execute!(stdout(), Hide).unwrap();
        execute!(stdout(), Clear(terminal::ClearType::All)).unwrap();
        execute!(stdout(), MoveTo(0, 0)).unwrap();
        for row in 0..self.terminal.rows {
            if let Some(line) = self.document.get_line(row as usize + self.offset.y) {
                self.draw_line(line);
            }
        }

        let (curs_x, curs_y) = self.terminal.get_xy();
        execute!(stdout(), MoveTo(curs_x, curs_y)).unwrap();
        execute!(stdout(), Show).unwrap();
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

    fn go_right(&mut self) -> Option<()> {
        let line_index = self.terminal.position.y as usize + self.offset.y;
        let line_end = self.document.lines[line_index].len().saturating_sub(1);

        let (curs_x, _) = cursor::position().unwrap_or_default();
        if curs_x < self.terminal.cols.saturating_sub(1)
            && self.offset.x.saturating_add(curs_x.into()) < line_end
        {
            self.terminal.position.set_x(curs_x.saturating_add(1));
            execute!(stdout(), MoveRight(1)).ok()
        } else {
            let offs_x = self.offset.x.saturating_add(1);
            if offs_x.saturating_add(curs_x.into()) <= line_end {
                self.offset.set_x(offs_x);
                self.display_document();
            }
            Some(())
        }
    }

    fn go_left(&mut self) -> Option<()> {
        let (curs_x, _) = cursor::position().unwrap_or_default();
        if curs_x > 0 {
            let curs_x = curs_x.saturating_sub(1);
            self.terminal.position.set_x(curs_x);
        } else if self.offset.x > 0 {
            self.offset.set_x(self.offset.x.saturating_sub(1));
            self.display_document();
        }

        execute!(stdout(), MoveLeft(1)).ok()
    }

    fn go_up(&mut self) -> Option<()> {
        if self.terminal.position.y > 0 {
            self.terminal
                .position
                .set_y(self.terminal.position.y.saturating_sub(1));
        } else if self.offset.y > 0 {
            self.offset.set_y(self.offset.y.saturating_sub(1));
        }
        let x_pos = self.line_into_view();
        execute!(stdout(), MoveTo(x_pos, self.terminal.position.y)).ok()
    }

    fn go_down(&mut self) -> Option<()> {
        let curs_y = self.terminal.position.y.saturating_add(1);
        let offs_y = self.offset.y.saturating_add(1);
        if curs_y < self.terminal.rows
            && self.offset.y.saturating_add(curs_y.into()) < self.document.lines.len()
        {
            self.terminal.position.set_y(curs_y);
        } else if curs_y == self.terminal.rows
            && offs_y < self.document.lines.len() - self.terminal.rows as usize
        {
            self.offset.set_y(offs_y);
        }
        let x_pos = self.line_into_view();
        execute!(stdout(), MoveTo(x_pos, self.terminal.position.y)).ok()
    }

    fn line_into_view(&mut self) -> u16 {
        if self.document.is_empty() {
            return 0;
        }

        let line_index = self
            .offset
            .y
            .saturating_add(self.terminal.position.y.into());
        let line_end = self.document.lines[line_index].len().saturating_sub(1);
        let eol_to_offset = self.offset.x.saturating_sub(line_end);
        if eol_to_offset > 0 {
            self.offset
                .set_x(self.offset.x.saturating_sub(eol_to_offset));
        }
        self.display_document();

        std::cmp::min(
            line_end.saturating_sub(self.offset.x),
            self.terminal.position.x.into(),
        )
        .try_into()
        .unwrap()
    }

    fn line_end(&mut self) -> Option<()> {
        self.terminal.position.set_x(0);
        if self.offset.x > 0 {
            self.offset.set_x(0);
            self.display_document();
        }
        execute!(stdout(), MoveTo(0, self.terminal.position.y)).ok()
    }

    fn line_start(&mut self) -> Option<()> {
        let line_index = self.terminal.position.y as usize + self.offset.y;
        let line_len = self.document.lines[line_index].len();
        let edge_pos = self.offset.x.saturating_add(self.terminal.cols.into());
        let offset_needed = line_len.saturating_sub(edge_pos);
        let x_pos = if offset_needed > 0 {
            self.offset
                .set_x(self.offset.x.saturating_add(offset_needed));
            self.display_document();
            self.terminal.cols.saturating_sub(1)
        } else {
            line_len
                .saturating_sub(self.offset.x)
                .saturating_sub(1)
                .try_into()
                .unwrap()
        };
        self.terminal.position.set_x(x_pos);
        execute!(stdout(), MoveTo(x_pos, self.terminal.position.y)).ok()
    }

    fn match_keycode(&mut self, keycode: KeyEvent) -> Option<()> {
        match keycode {
            KeyEvent {
                code: KeyCode::Char(key),
                modifiers: KeyModifiers::NONE,
                ..
            } => match key {
                'h' => self.go_left(),
                'j' => self.go_down(),
                'k' => self.go_up(),
                'l' => self.go_right(),
                '0' => self.line_end(),
                '$' => self.line_start(),
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
