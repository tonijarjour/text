pub struct Terminal {
    pub rows: u16,
    pub cols: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl Terminal {
    pub fn new() -> Self {
        let Ok((cols, rows)) = crossterm::terminal::size() else { panic!(); };
        Self { rows, cols }
    }

    pub fn set_size(&mut self, rows: u16, cols: u16) {
        self.rows = rows;
        self.cols = cols;
    }
}
