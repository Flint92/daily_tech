#[derive(Default, Copy, Clone)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}


impl Location {
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}