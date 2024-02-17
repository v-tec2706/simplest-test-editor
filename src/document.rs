use crate::Row;
use std::fs;
use std::io::Error;

#[derive(Default)]
pub struct Document {
    lines: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Document, Error> {
        let mut lines = Vec::new();
        let file_content = fs::read_to_string(filename)?;
        for line in file_content.lines() {
            lines.push(Row::from(line.to_string()));
        }

        Ok(Document { lines })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.lines.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
