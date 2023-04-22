use std::cmp;

pub struct Line {
    data: String,
}

impl From<&str> for Line {
    fn from(slice: &str) -> Self {
        Self {
            data: String::from(slice),
        }
    }
}

impl Line {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.data.len());
        let start = cmp::min(start, end);
        self.data.get(start..end).unwrap_or_default().to_string()
    }
}

#[derive(Default)]
pub struct Document {
    pub lines: Vec<Line>,
}

impl Document {
    pub fn open() -> Self {
        let mut lines = Vec::new();
        lines.push(Line::from("Hello, World!"));
        Self { lines }
    }

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }
}
