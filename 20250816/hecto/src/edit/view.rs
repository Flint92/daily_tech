use crate::buf::buffer::Buffer;
use crate::edit::terminal::{Position, Size, Terminal};

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
    pub fn render(&mut self) -> Result<(), std::io::Error> {
        if !self.need_redraw {
            return Ok(());
        }

        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return Ok(());
        }

        let vertical_center = height / 3;

        for curr_row in 0..height {
            if let Some(line) = self.buf.lines.get(curr_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(curr_row, truncated_line)?;
            } else if curr_row == vertical_center && self.buf.is_empty() {
                Self::render_line(curr_row, &Self::render_welcome_message(width))?;
            } else {
                Self::render_line(curr_row, "~")?;
            }
        }

        self.need_redraw = false;

        Ok(())
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

    fn render_line(at: usize, line_text: &str) -> Result<(), std::io::Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clear_curr_line()?;
        Terminal::print(line_text)?;
        Ok(())
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
