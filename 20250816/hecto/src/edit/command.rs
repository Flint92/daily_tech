use crate::edit::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
    Backspace,
    Delete,
    Enter,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(EditorCommand::Insert(c))
                },
                (KeyCode::Backspace, _) => Ok(EditorCommand::Backspace),
                (KeyCode::Delete, _) => Ok(EditorCommand::Delete),
                (KeyCode::Tab, _) => Ok(EditorCommand::Insert('\t')),
                (KeyCode::Enter, _) => Ok(EditorCommand::Enter),
                (KeyCode::Up, _) => Ok(EditorCommand::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(EditorCommand::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(EditorCommand::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(EditorCommand::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(EditorCommand::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(EditorCommand::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(EditorCommand::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(EditorCommand::Move(Direction::End)),
                _ => Err(format!("Unsupported key code: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => {
                let width = width_u16 as usize;
                let height = height_u16 as usize;

                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Unsupported event: {event:?}")),
        }
    }
}
