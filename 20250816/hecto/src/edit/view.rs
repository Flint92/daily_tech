use crate::buf::buffer::Buffer;
use crate::edit::command::{Direction, EditorCommand};
use crate::edit::line::Line;
use crate::edit::location::Location;
use crate::edit::terminal::{Position, Size, Terminal};
use std::cmp;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        View {
            buf: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if width == 0 || height == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.y;

        for curr_row in 0..height {
            if let Some(line) = self.buf.lines.get(curr_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(curr_row, &line.get(left..right));
            } else if curr_row == vertical_center && self.buf.is_empty() {
                Self::render_line(curr_row, &Self::render_welcome_message(width));
            } else {
                Self::render_line(curr_row, "~");
            }
        }

        self.need_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buf) = Buffer::load(file_name) {
            self.buf = buf;
            self.need_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.need_redraw = true;
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => {
                if x > 0 {
                    x = x.saturating_sub(1);
                } else if y > 0 {
                    y = y.saturating_sub(1);
                    x = self.buf.lines.get(y).map_or(0, Line::len);
                }
            }
            Direction::Right => {
                let width = self.buf.lines.get(y).map_or(0, Line::len);
                if x < width {
                    x = x.saturating_add(1);
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
            }
            Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = self.buf.lines.get(y).map_or(0, Line::len),
        }

        // snap x to valid location
        x = self
            .buf
            .lines
            .get(y)
            .map_or(0, |line| cmp::min(line.len(), x));

        // snap y to valid location
        y = cmp::min(y, self.buf.lines.len());

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { height, width } = self.size;

        let mut offset_changes = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changes = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_sub(1);
            offset_changes = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changes = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_add(width).saturating_sub(1);
            offset_changes = true;
        }

        self.need_redraw = offset_changes;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "failed to render line");
    }

    fn render_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }

        let welcome_msg = format!("{NAME} edit -- version {VERSION}");
        let len = welcome_msg.len();

        if width <= len {
            return "~".to_string();
        }

        let padding = (width.saturating_sub(len).saturating_sub(1)) / 20;
        let padding_str = " ".repeat(padding);

        let mut full_msg = format!("~{padding_str}{welcome_msg}");

        full_msg.truncate(width);
        full_msg
    }
}
