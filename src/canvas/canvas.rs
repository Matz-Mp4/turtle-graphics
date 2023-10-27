use num_traits::{real::Real, Zero};

use crate::{Color, ITuple2};
use std::{
    convert::{AsMut, AsRef},
    mem::swap,
    ops::{Index, IndexMut},
};

pub trait Canvas {
    fn new(width: usize, height: usize) -> Self
    where
        Self: Sized;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn width_as_mut(&mut self) -> &mut usize;
    fn height_as_mut(&mut self) -> &mut usize;
    fn clear(&mut self);
    fn color_at(&self, row: usize, col: usize) -> &Color;
    fn color_mut_at(&mut self, row: usize, col: usize) -> &mut Color;
    fn set_background(&mut self, color: Color) {
        for row in 0..self.width() {
            for col in 0..self.height() {
                self.set_color(row, col, color);
            }
        }
    }
    fn set_color(&mut self, row: i32, col: i32, color: Color) {
        let temp_row = row.abs() % self.width();
        let temp_col = col.abs() % self.height();
        /* println!("(x,y) ({} , {})", temp_col,temp_row); */
        let c = self.color_mut_at(temp_row as usize, temp_col as usize);
        *c = color;
    }
    fn draw_line<T: Real + Zero>(
        &mut self,
        q1: &impl ITuple2<T>,
        q2: &impl ITuple2<T>,
        color: Color,
    ) where
        Self: Sized,
    {
        let mut y0 = q1.y().to_i32().unwrap();
        let y1 = q2.y().to_i32().unwrap();
        let mut x0 = q1.x().to_i32().unwrap();
        let x1 = q2.x().to_i32().unwrap();

        let mut dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut stepx = 0;
        let mut stepy = 0;

        if dy < 0 {
            dy = -dy;
            stepy = -1;
        } else {
            stepy = 1;
        }

        if dx < 0 {
            dx = -dx;
            stepx = -1;
        } else {
            stepx = 1;
        }
        dy <<= 1;
        dx <<= 1;
        self.set_color(x0, y0, color);
        let height = self.height();
        let width = self.width();

        if dx > dy {
            let mut fraction = dy - (dx >> 1);
            while x0 != x1 {
                if fraction >= 0 {
                    y0 += stepy;
                    fraction -= dx;
                }
                x0 += stepx;
                fraction += dy;
                /* pixel[x0+y0] = pix; */
                self.set_color(x0, y0, color)
            }
        } else {
            let mut fraction = dx - (dy >> 1);
            while y0 != y1 {
                if fraction >= 0 {
                    x0 += stepx;
                    fraction -= dy;
                }
                y0 += stepy;
                fraction += dx;
                self.set_color(x0, y0, color)
            }
        }
    }
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
