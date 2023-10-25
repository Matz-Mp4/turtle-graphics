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
    let mut canvas = WindowCanvas::new(600, 600);
    let mut tur = TurtleLogo::new(Tuple2::new(200.0, 200.0), Tuple2::new(2.0, 2.0));
    canvas.set_background(Color::white());
    /* tur = tur.foward(1.0, Color::black(), &mut canvas); */
    /* show_turtle(&tur); */
    /* tur = tur.turn(PI / 2.0); */
    /* show_turtle(&tur); */
    /* tur = tur.foward(1.0, Color::black(), &mut canvas); */
    /* show_turtle(&tur); */

    /* polygon(&mut tur, 2, Color::black(), &mut canvas); */
    let mut x =1.0;
    let delay = Duration::from_millis(100);
    while !canvas.should_close() {
        tur = tur.foward(x, Color::black(), &mut canvas);
        tur = tur.turn(-PI/6.0);
        canvas.display();
        sleep(delay);
        x+=1.0;
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
