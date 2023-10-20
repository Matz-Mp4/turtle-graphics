use crate::Color;
use std::{
    convert::{AsMut, AsRef},
    ops::{Index, IndexMut},
};

pub trait Canvas {
    fn new(name: &str, width: usize, height: usize) -> Self
    where
        Self: Sized;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn width_as_mut(&mut self) -> &mut usize;
    fn height_as_mut(&mut self) -> &mut usize;
    fn clear(&mut self);
    fn color_at(&self, row: usize, col: usize) -> &Color;
    fn color_at_mut(&mut self, row: usize, col: usize) -> &mut Color;
}

//impl AsRef<dyn Canvas> for dyn Canvas {
//    fn as_ref(&self) -> &(dyn Canvas + 'static) {
//        self
//    }
//}
//
//impl AsMut<dyn Canvas> for dyn Canvas {
//    fn as_mut(&mut self) -> &mut (dyn Canvas + 'static) {
//        self
//    }
//}
//
//impl Index<usize> for dyn Canvas {
//    type Output = [Color];
//
//    fn index(&self, row: usize) -> &[Color] {
//        let start = row * self.width();
//        let colors = self.pixels();
//
//        &colors[start..start + self.width()]
//    }
//}
//
//impl IndexMut<usize> for dyn Canvas {
//    fn index_mut(&mut self, row: usize) -> &mut [Color] {
//        let width = self.width();
//        let start = row * width;
//        let colors = self.pixels_as_mut();
//
//        &mut colors[start..start + width]
//    }
//}
