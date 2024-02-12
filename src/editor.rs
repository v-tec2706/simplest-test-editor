use crate::terminal::Terminal;
use std::io;
use std::io::Error;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub struct Editor {
    terminal: Terminal,
    should_quit: bool,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            terminal: Terminal::default(),
            should_quit: false,
        }
    }

    fn die(&self, error: Error) {
        panic!("{}", error)
    }

    pub fn run(&mut self) -> () {
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        self.terminal.clear_screen();

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
        let key = self.terminal.read_key()?;
        match key {
            Key::Char('q') => self.should_quit = true,
            Key::Char(a) => println!("Entered: {}", a),
            _ => (),
        }
        Ok(())
    }
}
