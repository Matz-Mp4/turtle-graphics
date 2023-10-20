use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::WindowCanvas;



fn main() {
    let mut window: Box<WindowCanvas> = Box::new(Canvas::new("Oh my God", 800, 800));

    while !window.should_close() {
        window.clear();
        window.display();
    }
}
