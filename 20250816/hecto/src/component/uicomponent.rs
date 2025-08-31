use crate::edit::terminal::Size;

pub trait UIComponent {
    // Marks this UI Component as in need of redrawing (or not)
    fn mark_redraw(&mut self, b: bool);

    // Returns true if this UI Component needs to be redrawn
    fn needs_redraw(&self) -> bool;

    // Update the size and mark this UI Component as in need of redrawing
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_redraw(true);
    }

    // Update the size. Needs to be implemented by the UI Component
    fn set_size(&mut self, size: Size);

    // Draw this component if it's visible and in need of redrawing
    fn render(&mut self, origin_y: usize) {
        if self.needs_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }

    // Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error>;
}
