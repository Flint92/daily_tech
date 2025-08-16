mod terminal;

use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::event::{Event, KeyEvent, KeyModifiers, read};

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
        let height = terminal::Terminal::size()?.1;

        for curr_row in 0..height {
            print!("~");

            if curr_row + 1 < height {
                print!("\r\n");
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
        if self.should_quit {
            terminal::Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            terminal::Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
}
