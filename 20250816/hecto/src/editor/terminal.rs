use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::fmt::Display;
use std::io::{Write, stdout};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_curr_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Show)
    }

    pub fn print<T: Display>(s: T) -> Result<(), std::io::Error> {
        Self::queue_command(Print(s))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), std::io::Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(cmd: T) -> Result<(), std::io::Error> {
        queue!(stdout(), cmd)
    }
}
