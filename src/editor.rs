use crate::terminal::Terminal;
use std::{fmt::format, io::Error};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(err)
            }

            if self.should_quit {
                break;
            }

            if let Err(err) = self.process_keypress() {
                die(err);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_position(0, 0);

        if self.should_quit {
            Terminal::clear_screen();
            println!("GoodBye. \r");
        } else {
            self.darw_rows();
        }

        Terminal::flush()
    }

    fn darw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == 0 {
                let welcome_message = format!("Hello Editor -- version {}", VERSION);
                let min_width =
                    std::cmp::min(welcome_message.len(), self.terminal.size().width as usize);
                println!("{}\r", &welcome_message[..min_width]);
            } else {
                println!("~\r");
            }
        }
        Terminal::cursor_position(0, 0);
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        }

        Ok(())
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
