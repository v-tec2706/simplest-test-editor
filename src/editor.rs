use std::io;
use std::io::Error;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    fn die(&self, error: Error) {
        panic!("{}", error)
    }

    pub fn run(&mut self) -> () {
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        self.print_left_boarded();

        loop {
            if self.should_quit == true {
                println!("Goodbye!");
                return ();
            }
            if let Err(err) = self.process_key() {
                self.die(err)
            }
        }
    }

    fn process_key(&mut self) -> Result<(), Error> {
        let key = self.read_key()?;
        match key {
            Key::Char('q') => self.should_quit = true,
            Key::Char(a) => println!("Entered: {}", a),
            _ => (),
        }
        Ok(())
    }

    fn read_key(&self) -> Result<Key, Error> {
        while let Some(key) = io::stdin().keys().next() {
            return key;
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to read a valid key",
        ))
    }

    fn print_left_boarded(&self) -> Result<(), Error> {
        let (height, _) = termion::terminal_size()?;
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        for i in 1..=height {
            print!("{}~", cursor::Goto(1, i));
        }
        print!("{}", cursor::Goto(1, 1));
        Ok(())
    }
}
