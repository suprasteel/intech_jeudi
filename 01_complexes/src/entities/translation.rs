use super::Norm;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Translation {
    pub x: f64,
    pub y: f64,
}

impl Translation {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Norm for Translation {
    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
