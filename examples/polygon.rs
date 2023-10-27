use std::f64::consts::PI;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

pub fn polygon(tur: &mut TurtleLogo<f64>, sides: usize, color: Color, can: &mut impl Canvas) {
    for _ in 0..sides {
        *tur = tur.forward(1.0, color, can);
        *tur = tur.turn(2.0 * PI / sides as f64);
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(500, 400);
    let mut tur = TurtleLogo::new(Tuple2::new(100.0, 100.0), Tuple2::new(100.0, 0.0));
    canvas.set_background(Color::white());
    polygon(&mut tur, 4, Color::black(), &mut canvas);
    tur = tur.mov(2.0);
    polygon(&mut tur, 5, Color::black(), &mut canvas);
    
    while !canvas.should_close() {
        canvas.display();
    }
}
