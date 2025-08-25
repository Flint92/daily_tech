use crate::buf::buffer::Buffer;
use crate::edit::command::{Direction, EditorCommand};
use crate::edit::line::Line;
use crate::edit::terminal::{Position, Size, Terminal};
use std::cmp;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        View {
            buf: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
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
        let top = self.scroll_offset.row;

        for curr_row in 0..height {
            if let Some(line) = self.buf.lines.get(curr_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(curr_row, &line.get_visible_graphemes(left..right));
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

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_text_location_into_view();
        self.need_redraw = true;
    }

    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        self.need_redraw = self.need_redraw || offset_changed;
    }

    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        self.need_redraw = self.need_redraw || offset_changed;
    }

    fn scroll_text_location_into_view(&mut self) {
        let Position { col, row } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buf.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_line(),
            Direction::End => self.move_to_end_line(),
        }

        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme()
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        let line_width = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);

        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index = self.text_location.grapheme_index.saturating_add(1);
        } else {
            self.move_to_start_line();
            self.move_down(1)
        }
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index = self.text_location.grapheme_index.saturating_sub(1);
        } else {
            self.move_up(1);
            self.move_to_end_line()
        }
    }

    fn move_to_start_line(&mut self) {
        self.text_location.grapheme_index = 0
    }

    fn move_to_end_line(&mut self) {
        self.text_location.grapheme_index = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count)
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                cmp::min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = cmp::min(self.text_location.line_index, self.buf.height());
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
