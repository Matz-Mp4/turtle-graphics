use std::ops::{Index, IndexMut};

use crate::{Canvas, Color};

pub struct WindowCanvas {
    data: Vec<Color>,
    width: usize,
    height: usize,
    window: minifb::Window,
}
impl Canvas for WindowCanvas {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn clear(&mut self) {
        let data = vec![Color::black(); self.width * self.height];
        self.data = data;
    }

    fn color_at(&self, row: usize, col: usize) -> &Color {
        /* &self.data[row + col *  self.width ] */

        &self.data[col * self.width + row]
    }

    fn color_mut_at(&mut self, row: usize, col: usize) -> &mut Color {


        /* &mut self.data[row * self.height+ col ] */
        &mut self.data[col * self.width + row]
    }
   

    fn new(width: usize, height: usize) -> Self
    where
        Self: Sized,
    {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };
        let data = vec![Color::black(); width * height];

        let window = minifb::Window::new("TurtleGraphics", width, height, options)
            .expect("Failed to create window.");

        WindowCanvas {
            data,
            width,
            height,
            window,
        }
    }

    fn width_as_mut(&mut self) -> &mut usize {
        &mut self.width
    }

    fn height_as_mut(&mut self) -> &mut usize {
        &mut self.height
    }
}
impl WindowCanvas {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn display(&mut self) {
        let mut data_conv = vec![0 as u32; self.width * self.height];
        for (i, color) in self.data.iter().enumerate() {
            let (r, g, b) = color.color_into_pixel();
            data_conv[i] = WindowCanvas::from_u8_rgb(r, g, b);
        }
        self.window
            .update_with_buffer(&data_conv, self.width, self.height)
            .expect("Failed to update window buffer.");

        let (width, height) = self.window.get_size();
        if width != self.width || height != self.height {
            /* self.framebuffer = Framebuffer::new(width, height); */
        }
    }
}

impl Index<usize> for WindowCanvas{
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.width;

        &self.data[start..start + self.width]
    }
}

impl IndexMut<usize> for WindowCanvas{
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.width;

        &mut self.data[start..start + self.width]

    }
}
