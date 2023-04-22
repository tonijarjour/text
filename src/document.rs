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
    pub fn open(file_path: &String) -> Result<Self, std::io::Error> {
        let file = std::fs::read_to_string(file_path)?;
        let mut lines = Vec::new();
        for line in file.lines() {
            lines.push(Line::from(line));
        }

        Ok(Self { lines })
    }

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
    }
}
