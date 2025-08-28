use crate::constant::constants::NAME;
use crate::edit::command::EditorCommand;
use crate::edit::statusbar::Statusbar;
use crate::edit::terminal::Terminal;
use crate::edit::view::View;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::io::Error;
use std::panic::{set_hook, take_hook};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: Statusbar,
    title: String,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let curr_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            curr_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self {
            should_quit: false,
            view: View::new(2),
            status_bar: Statusbar::new(1),
            title: String::new(),
        };
        let args: Vec<String> = std::env::args().collect();
        if let Some(filename) = args.get(1) {
            editor.view.load(filename);
        }

        editor.refresh_status();
        Ok(editor)
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.execute_event(event),
                Err(err) => {
                    panic!("Could not read event {err:?}");
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    fn execute_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                    if let EditorCommand::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        } else {
            panic!("Received and discarded unsupported or non-press event.");
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
