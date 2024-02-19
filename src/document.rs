use crate::Row;
use std::fs;
use std::io::Error;

pub struct Document {
    lines: Vec<Row>,
    pub filename: String,
}

impl Document {
    pub fn open(filename: &str) -> Result<Document, Error> {
        let mut lines = Vec::new();
        let file_content = fs::read_to_string(filename)?;
        for line in file_content.lines() {
            lines.push(Row::from(line.to_string()));
        }

        Ok(Document {
            lines,
            filename: filename.to_string(),
        })
    }

    pub fn empty() -> Document {
        Document {
            lines: Vec::new(),
            filename: String::from("Unnamed"),
        }
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.lines.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn size(&self) -> usize {
        self.lines.len()
    }
}
