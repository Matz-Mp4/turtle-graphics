use crate::{Canvas, Color};

pub struct Window {
    data: Vec<u32>,
    width: usize,
    height: usize,
    window: minifb::Window,
}

impl Window {
    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };
        let data = vec![0; width * height];

        let window =
            minifb::Window::new(name, width, height, options).expect("Failed to create window.");

        Window {
            data,
            width,
            height,
            window,
        }
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn display(&mut self) {
        self.window
            .update_with_buffer(&self.data, self.width, self.height)
            .expect("Failed to update window buffer.");

        
        let (width, height) = self.window.get_size();
        if width != self.width || height != self.height {
            /* self.framebuffer = Framebuffer::new(width, height); */
        }
    }
}

impl Canvas for Window {
    fn width(&self) -> usize {
        self.width
    }


    fn height(&self) -> usize {
        self.height
    }

    fn clear(&mut self) {
        let data = vec![0; self.width * self.height];
        self.data = data;
    }

    fn set_pixel(&mut self, row: usize, col: usize, color: Color) {
        let (r, g, b) = color.color_into_pixel();
        let pixel = Self::from_u8_rgb(r,g,b);
        self.data[row + col * self.width] = pixel;
    }
}
