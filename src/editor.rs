use std::io;
use std::io::Error;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    fn die(&self, error: Error) {
        panic!("{}", error)
    }

    pub fn run(&self) -> () {
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            if let Err(err) = self.process_key() {
                self.die(err)
            }
        }
    }

    fn process_key(&self) -> Result<(), Error> {
        let key = self.read_key()?;
        match key {
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
}
