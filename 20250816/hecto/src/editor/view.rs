use crate::buf::buffer::Buffer;
use crate::editor::terminal::{Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buf: Buffer,
    need_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        View{
            buf: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {

    pub fn render(&self) -> Result<(), std::io::Error> {
        if self.buf.is_empty() {
            self.render_welcome_message()
        } else {
            self.render_buf()
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buf) = Buffer::load(file_name) {
            self.buf = buf;
        }
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.need_redraw = true;
    }

    fn render_welcome_message(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for curr_row in 0..height {
            Terminal::clear_curr_line()?;

            if curr_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if curr_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn render_buf(&self) -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for curr_row in 0..height {
            Terminal::clear_curr_line()?;

            if let Some(line) = self.buf.lines.get(curr_row) {
                Terminal::print(line)?;
            } else {
                Self::draw_empty_row()?;
            }

            if curr_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_msg.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        Terminal::print(welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
