use crate::component::uicomponent::UIComponent;
use crate::edit::documentation::DocumentStatus;
use crate::edit::terminal::{Size, Terminal};
use std::io::Error;

#[derive(Default)]
pub struct Statusbar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl Statusbar {
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.current_status != new_status {
            self.current_status = new_status;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for Statusbar {
    fn mark_redraw(&mut self, b: bool) {
        self.needs_redraw = b;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        let line_count_str = self.current_status.line_count_to_string();
        let modified_indicator_str = self.current_status.modified_indicator_to_string();

        let beginning = format!(
            "{} - {line_count_str} {modified_indicator_str}",
            self.current_status.file_name
        );

        let pos_indicator_str = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{pos_indicator_str:>remainder_len$}");

        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        Terminal::print_inverted_row(origin_y, &to_print)?;


        Ok(())
    }
}
