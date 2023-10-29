use num_traits::{real::Real, Zero};

use crate::{Color, ITuple2};

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
    ///Define a color of a pixel. The parameter x represents the column
    ///and the parameter represents y the row.
    fn set_color(&mut self, x: i32, y: i32, color: Color) {
        let temp_col = x.abs() % self.width();
        let temp_row = y.abs() % self.height();
        let c = self.color_mut_at(temp_col as usize, temp_row as usize);
        *c = color;
    }
    ///Draw a line using Bresenham's line algorithm
    ///
    ///# Example
    ///```rust
    ///# use turtle_graphics::{ImageCanvas, Canvas, WHITE};
    ///
    ///
    ///    let mut canvas = ImageCanvas::new(600,600);
    ///    let a = (0.0,0.0);
    ///    let b = (600.0,600.0);
    ///    canvas.draw_line(&a,&b, WHITE);
    ///    canvas.convert_to_ppm("", "canvas.ppm");
    ///
    ///```
    /// ![Diagonal Line](https://github.com/Matz-Mp4/turtle-graphics/blob/main/pictures/diagonal_line.png)
    ///
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
        let stepx;
        let stepy;

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
        let height = self.height();
        self.set_color(x0, height - y0, color);
        /* let width = self.width(); */

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
                self.set_color(x0, height - y0, color)
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
                self.set_color(x0, height - y0, color)
            }
        }
    }
}
