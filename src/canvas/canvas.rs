use crate::Color;
use std::convert::{AsMut, AsRef};

pub trait Canvas {
    fn new(width: usize, height: usize) -> Self where Self: Sized;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn clear(&mut self);
    fn set_pixel(&mut self, row: usize, col: usize, color: Color);
}

impl AsRef<dyn Canvas> for dyn Canvas {
    fn as_ref(&self) -> &(dyn Canvas + 'static) {
        self
    }
}

impl AsMut<dyn Canvas> for dyn Canvas {
    fn as_mut(&mut self) -> &mut (dyn Canvas + 'static) {
        self
    }
}
