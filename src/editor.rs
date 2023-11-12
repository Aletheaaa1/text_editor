use crate::terminal::Terminal;
use std::io::Error;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new(),
            cursor_position: Position { x: 0, y: 0 },
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
        Terminal::cursor_position(&Position { x: 0, y: 0 });

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
        Terminal::cursor_position(&self.cursor_position);
    }

    fn process_keypress(&mut self) -> Result<(), Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(&pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: &Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let width = size.width.saturating_sub(1) as usize;
        let height = size.height.saturating_sub(1) as usize;

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height - 1 {
                    y = y.saturating_add(1)
                }
            }
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::PageUp => y = 0,
            Key::PageDown => y = height - 1,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
