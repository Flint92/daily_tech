use crate::buf::buffer::Buffer;
use crate::constant::constants::{NAME, VERSION};
use crate::edit::command::{Direction, EditorCommand};
use crate::edit::documentation::DocumentStatus;
use crate::edit::line::Line;
use crate::edit::terminal::{Position, Size, Terminal};
use std::cmp;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

#[derive(Default)]
pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
    margin_bottom: usize,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            buf: Buffer::default(),
            need_redraw: true,
            size: Size {
                height: size.height.saturating_sub(margin_bottom),
                width: size.width,
            },
            margin_bottom,
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }

    pub fn render(&mut self) {
        if !self.need_redraw || self.size.height == 0 {
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
            EditorCommand::Move(direction) => self.move_text_location(direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
            EditorCommand::Insert(c) => self.insert_char(c),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Enter => self.insert_newline(),
            EditorCommand::Save => self.save(),
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buf) = Buffer::load(file_name) {
            self.buf = buf;
            self.need_redraw = true;
        }
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buf.height(),
            current_line_index: self.text_location.line_index,
            is_modified: self.buf.dirty,
            file_name: format!("{}", self.buf.file_info),
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.size = Size {
            height: to.height.saturating_sub(self.margin_bottom),
            width: to.width,
        };
        self.scroll_text_location_into_view();
        self.need_redraw = true;
    }

    fn save(&mut self) {
        let _ = self.buf.save();
    }

    fn insert_newline(&mut self) {
        self.buf.insert_newline(self.text_location);
        self.move_text_location(Direction::Right);
        self.need_redraw = true;
    }

    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        self.buf.insert_char(character, self.text_location);
        let new_len = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            //move right for an added grapheme (should be the regular case)
            self.move_text_location(Direction::Right);
        }
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
        if offset_changed {
            self.need_redraw = true;
        }
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
        if offset_changed {
            self.need_redraw = true;
        }
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

    fn move_text_location(&mut self, direction: Direction) {
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

    fn backspace(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.move_text_location(Direction::Left);
            self.delete();
        }
    }

    fn delete(&mut self) {
        self.buf.delete(self.text_location);
        self.need_redraw = true;
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
        } else if self.text_location.line_index > 0 {
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
            return String::new();
        }

        let welcome_msg = format!("{NAME} edit -- version {VERSION}");
        let len = welcome_msg.len();

        let remaining_width = width.saturating_sub(1);
        if remaining_width < len {
            return "~".to_string();
        }

        format!("{:<1}{:^remaining_width$}", "~", welcome_msg)
    }
}
