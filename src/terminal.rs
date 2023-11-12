use std::{
    io::Error,
    io::{self, stdout, Write},
};

use crate::editor::Position;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{self, event::Key};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    out: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn new() -> Terminal {
        let size = termion::terminal_size().unwrap();

        Terminal {
            size: Size {
                width: size.0,
                height: size.1,
            },
            out: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn read_key() -> Result<Key, Error> {
        io::stdin().lock().keys().next().unwrap()
    }

    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
}
