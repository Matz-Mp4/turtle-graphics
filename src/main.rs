use std::f64::consts::PI;
use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::ImageCanvas;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

pub fn polygon(tur: &mut TurtleLogo<f64>, sides: usize, color: Color, can: &mut impl Canvas) {
    for _ in 0..sides {
        /* tur.foward(&Tuple2::new(1.0, 1.0), color, can); */
        /* tur.turn(2.0 * PI / sides as f64); */
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(600, 600);
    let mut tur = TurtleLogo::new(Tuple2::<f64>::zero(), Tuple2::new(1.0, 1.0));
    tur = tur.with_point(Tuple2::new(0.0, 0.0));
    canvas.set_background(Color::white());

    /* canvas.convert_to_ppm("./", "teste.ppm"); */

    tur = tur.foward(100.0, Color::red(), &mut canvas);
    show_turtle(&tur);
    tur = tur.turn(60.0);
    show_turtle(&tur);
    tur = tur.foward(100.0, Color::red(), &mut canvas);

    let mut x = 1.0;
    while !canvas.should_close() {
        /* tur = tur.foward(x, Color::red(), &mut canvas); */
        /* x += 1.0; */
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
