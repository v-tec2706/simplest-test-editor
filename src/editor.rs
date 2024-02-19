use crate::document::Document;
use crate::terminal;
use crate::terminal::Size;
use crate::terminal::Terminal;
use std::env;
use std::io;
use std::io::Error;
use termion::color;
use termion::color::Color;
use termion::event::Key;
use termion::raw::IntoRawMode;

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    should_quit: bool,
    document: Document,
    offset: Position,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();
        let doc = if args.len() == 1 {
            Document::open(&args[0]).unwrap_or(Document::empty())
        } else {
            Document::empty()
        };

        Editor {
            terminal: Terminal::default(),
            cursor_position: Position { x: 1, y: 1 },
            should_quit: false,
            document: doc,
            offset: Position { x: 0, y: 0 },
        }
    }

    fn die(&self, error: Error) {
        panic!("{}", error)
    }

    pub fn run(&mut self) -> () {
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                self.die(error);
            }
            if self.should_quit == true {
                break;
            }
            if let Err(err) = self.process_key() {
                self.die(err)
            }
        }
    }

    fn process_key(&mut self) -> Result<(), Error> {
        let key = self.terminal.read_key()?;
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let Size { height, width } = self.terminal.size();
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                //if y < *height {
                y = y.saturating_add(1);
                //}
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                //if x < *width {
                x = x.saturating_add(1);
                //}
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = *height,
            Key::Home => x = 0,
            Key::End => x = *width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let mut new_offset = &mut self.offset;

        if x >= new_offset.x + self.terminal.size().width {
            new_offset.x = x - self.terminal.size().width
        } else if x < new_offset.x {
            new_offset.x = x
        }

        if y >= new_offset.y + self.terminal.size().height {
            new_offset.y = y - self.terminal.size().height
        } else if y < new_offset.y {
            new_offset.y = y
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version v1");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height {
            Terminal::clear_current_line();
            if row == height / 3 && self.document.is_empty() {
                self.draw_welcome_message();
            } else {
                self.draw_doc_line(row + self.offset.y);
            }
        }
        self.draw_message_bar();
    }

    fn draw_doc_line(&self, index: u16) -> Result<(), Error> {
        let start = self.offset.x;
        let end = start + self.terminal.size().width;
        if let Some(line) = self.document.row(index as usize) {
            println!("{}\r", line.render(start as usize, end as usize));
        } else {
            println!("~\r");
        }
        Ok(())
    }

    fn draw_status_bar(&self) {
        Terminal::clear_current_line();
    }

    fn draw_message_bar(&self) {
        Terminal::set_bg_color();
        Terminal::set_fg_color();
        let status_msg = format!(
            "{} | {} lines",
            self.document.filename,
            self.document.size()
        );
        let spaces_to_fulfill = self.terminal.size().width as usize - status_msg.len();
        println!("{}{}\r", status_msg, " ".repeat(spaces_to_fulfill));
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::move_cursor(&Position { x: 1, y: 1 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::move_cursor(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
}
