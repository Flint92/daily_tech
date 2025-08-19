use crate::editor::terminal;
use crate::editor::terminal::Size;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), std::io::Error> {
        let Size { height, .. } = terminal::Terminal::size()?;

        terminal::Terminal::clear_curr_line()?;
        terminal::Terminal::print("Hello, World!\r\n")?;

        for curr_row in 1..height {
            terminal::Terminal::clear_curr_line()?;

            if curr_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }

            if curr_row.saturating_add(1) < height {
                terminal::Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_msg = format!("{NAME} editor -- version {VERSION}");
        let width = terminal::Terminal::size()?.width as usize;
        let len = welcome_msg.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_msg = format!("~{spaces}{welcome_msg}");
        welcome_msg.truncate(width);
        terminal::Terminal::print(welcome_msg)?;
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        terminal::Terminal::print("~")?;
        Ok(())
    }
}
