use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::Window;

fn color_at(can: Box<dyn Canvas>, row: usize, col: usize) -> Color {
    can[row][col]
}

fn main() {
    let mut window: Box<Window> = Box::new(Canvas::new("Oh my God", 800, 800));
    let x = window.as_ref();

    /* let color = color_at(Box::new(window.clone()), 0, 0); */
    /* let x = window; */

    while !window.should_close() {
        window.clear();
        window.display();
    }
}
