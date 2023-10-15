use crate::Color;

pub trait Canvas {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn clear(&mut self);
    fn set_pixel(&mut self, row: usize, col: usize, color: Color);
}
