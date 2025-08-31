use crate::component::uicomponent::UIComponent;
use crate::edit::terminal::{Size, Terminal};
use std::io::Error;
use std::time::{Duration, Instant};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: String) {
        self.current_message = Message {
            text: new_message,
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.mark_redraw(true)
    }
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, b: bool) {
        self.needs_redraw = b;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
            // Upon expiration, we need to write out "" once to clear the message.
            // To avoid clearing more than necessary,
            // we  keep track of the fact that we've already cleared the expired message once.
            // 1Code has comments. Press enter to view.
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)
    }
}
