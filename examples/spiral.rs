use std::f64::consts::PI;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

use turtle_graphics::BLACK;
use turtle_graphics::WHITE;

pub fn spiral(tur: &mut TurtleLogo<f64>, sides: usize, ang: f64, scale: f64, color: Color, can: &mut impl Canvas) {
    for _ in 0..sides {
        *tur = tur.forward(1.0, color, can);
        *tur = tur.turn(ang);
        *tur = tur.resize(scale);
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(500, 500);
    let mut turtle = TurtleLogo::new(Tuple2::new(50.0, 200.0), Tuple2::new(400.0, 0.0));
    canvas.set_background(WHITE);
    spiral(&mut turtle, 100, (4.0 * PI) / 5.0, 9.0 / 10.0, BLACK, &mut canvas);

    while !canvas.should_close() {
        canvas.display();
    }
}
