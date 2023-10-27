use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::ImageCanvas;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

pub fn polygon(tur: &mut TurtleLogo<f64>, sides: usize, color: Color, can: &mut impl Canvas) {
    for _ in 0..sides {
        *tur = tur.foward(1.0, color, can);
        *tur = tur.turn(2.0 * PI / sides as f64);
        show_turtle(&tur);
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(500, 400);
    let mut tur = TurtleLogo::new(Tuple2::new(100.0, 200.0), Tuple2::new(100.0, 0.0));
    canvas.set_background(Color::white());
    polygon(&mut tur, 4, Color::black(), &mut canvas);
    tur = tur.mov(2.0);
    polygon(&mut tur, 5, Color::black(), &mut canvas);
    
    while !canvas.should_close() {
        canvas.display();
    }
}

pub fn show_turtle(tur: &TurtleLogo<f64>) {
    let vector = tur.vector();
    let point = tur.point();
    println!(
        "point =  ({},{}) and vector = ({},{})",
        point.x(),
        point.y(),
        vector.x(),
        vector.y()
    );
}
