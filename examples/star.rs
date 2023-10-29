use std::f64::consts::PI;

use turtle_graphics::BLACK;
use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WHITE;
use turtle_graphics::WindowCanvas;

pub fn star(tur: &mut TurtleLogo<f64>, sides: usize, color: Color, can: &mut impl Canvas) {
    for _ in 0..sides {
        *tur = tur.forward(1.0, color, can);
        *tur = tur.turn(4.0 * PI / sides as f64);
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(500, 500);
    let mut turtle = TurtleLogo::new(Tuple2::new(100.0, 200.0), Tuple2::new(200.0, 0.0));
    canvas.set_background(WHITE);
    star(&mut turtle, 5, BLACK, &mut canvas);

    while !canvas.should_close() {
        canvas.display();
    }
}
