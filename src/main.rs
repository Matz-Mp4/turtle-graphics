use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::WindowCanvas;
use turtle_graphics::ImageCanvas;



fn main() {
    let canvas = ImageCanvas::new(800, 600);
    canvas.convert_to_ppm("./", "test.ppm");
}
