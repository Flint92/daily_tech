mod terminal;

use crate::editor::terminal::{Position, Size};
use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, read};
use std::cmp::min;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        terminal::Terminal::initialize().unwrap();
        let result = self.repl();
        terminal::Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.execute_event(event)?;
        }

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = terminal::Terminal::size()?;

        for curr_row in 0..height {
            terminal::Terminal::clear_curr_line()?;

            if curr_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if curr_row + 1 < height {
                terminal::Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let width = terminal::Terminal::size()?.width as usize;
        let len = welcome_msg.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        terminal::Terminal::print(welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        terminal::Terminal::print("~")?;
        Ok(())
    }

    fn execute_event(&mut self, event: Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home => self.move_point(code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = terminal::Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        terminal::Terminal::hide_caret()?;
        terminal::Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            terminal::Terminal::clear_screen()?;
            terminal::Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            terminal::Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        terminal::Terminal::show_caret()?;
        terminal::Terminal::execute()?;
        Ok(())
    }
}
