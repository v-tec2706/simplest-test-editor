use std::io;
use std::io::Error;
use std::io::ErrorKind;
use termion;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

use crate::editor::Position;

pub struct Terminal {
    height: u16,
    width: u16,
}

impl Terminal {
    pub fn default() -> Self {
        let (h, w) = termion::terminal_size().unwrap();
        Terminal {
            height: h,
            width: w,
        }
    }

    fn size(&self) -> (u16, u16) {
        (self.height, self.width)
    }

    pub fn clear_screen(&self) -> Result<(), Error> {
        let (height, _) = self.size();
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        for i in 1..=height {
            print!("{}~", cursor::Goto(1, i));
        }
        print!("{}", cursor::Goto(1, 1));
        Ok(())
    }

    pub fn move_cursor(position: Position) -> Result<(), Error> {
        print!("{}", cursor::Goto(position.x, position.y));
        Ok(())
    }

    pub fn read_key(&self) -> Result<Key, Error> {
        while let Some(key) = io::stdin().keys().next() {
            return key;
        }
        Err(Error::new(ErrorKind::Other, "Failed to read a valid key"))
    }
}
