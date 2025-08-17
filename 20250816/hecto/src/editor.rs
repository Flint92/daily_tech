mod terminal;

use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::event::{Event, KeyEvent, KeyModifiers, read};
use crate::editor::terminal::{Position, Size};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Editor { should_quit: false }
    }

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
            self.execute_event(event);
        }

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size{height, ..} = terminal::Terminal::size()?;

        for curr_row in 0..height {
            terminal::Terminal::clear_curr_line()?;
            terminal::Terminal::print("~")?;

            if curr_row + 1 < height {
                terminal::Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn execute_event(&mut self, event: Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");

            match code {
                Char('q') if modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        terminal::Terminal::hide_cursor()?;
        if self.should_quit {
            terminal::Terminal::clear_screen()?;
            terminal::Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            terminal::Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        terminal::Terminal::show_cursor()?;
        terminal::Terminal::execute()?;
        Ok(())
    }
}
