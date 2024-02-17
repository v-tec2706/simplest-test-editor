use std::cmp::min;

pub struct Row {
    content: String,
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        Row { content: value }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> &str {
        let end = min(end, self.content.len());
        self.content.get(start..end).unwrap_or("FOOR")
    }
}
