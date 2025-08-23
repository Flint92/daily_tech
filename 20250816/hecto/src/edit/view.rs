use crate::buf::buffer::Buffer;
use crate::edit::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        View {
            buf: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }

        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return;
        }

        let vertical_center = height / 3;

        for curr_row in 0..height {
            if let Some(line) = self.buf.lines.get(curr_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(curr_row, truncated_line);
            } else if curr_row == vertical_center && self.buf.is_empty() {
                Self::render_line(curr_row, &Self::render_welcome_message(width));
            } else {
                Self::render_line(curr_row, "~");
            }
        }

        self.need_redraw = false;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buf) = Buffer::load(file_name) {
            self.buf = buf;
            self.need_redraw = true;
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.need_redraw = true;
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
