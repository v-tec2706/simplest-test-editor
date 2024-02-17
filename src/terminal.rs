use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Write;
use termion;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

use crate::editor::Position;

pub struct Terminal {
    size: Size,
}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

impl Terminal {
    pub fn default() -> Self {
        let (w, h) = termion::terminal_size().unwrap();
        Terminal {
            size: Size {
                height: h,
                width: w,
            },
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    pub fn move_cursor(position: &Position) {
        print!("{}", cursor::Goto(position.x, position.y));
    }

    pub fn read_key(&self) -> Result<Key, Error> {
        while let Some(key) = io::stdin().keys().next() {
            return key;
        }
        Err(Error::new(ErrorKind::Other, "Failed to read a valid key"))
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }
}
