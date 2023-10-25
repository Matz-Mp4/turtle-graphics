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
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(600, 600);
    let mut tur = TurtleLogo::new(Tuple2::new(300.0, 400.0), Tuple2::new(50.0, 50.0));
    /* tur = tur.foward(1.0, Color::red(), &mut canvas); */
    /* show_turtle(&mut tur); */

    /* tur = tur.foward(100.0, Color::red(), &mut canvas); */
    /* show_turtle(&mut tur); */
    /* tur = tur.turn(90.0); */
    /* show_turtle(&mut tur); */
    /* tur = tur.foward(1.0, Color::red(), &mut canvas); */

    let delay = Duration::from_millis(100);

    /* polygon(&mut tur, 3, Color::white(), &mut canvas); */
    /* polygon(&mut tur, 7, Color::white(), &mut canvas); */
    /* canvas.convert_to_ppm("./", "output.ppm"); */
    let sides = 5;
    /* tur = tur.foward(1.0, Color::red(), &mut canvas); */
    /* tur = tur.turn(30.0); */
    /* show_turtle(&mut tur); */
    polygon(&mut tur, 4, Color::white(), &mut canvas);
    /* polygon(&mut tur, 6, Color::white(), &mut canvas); */
    /* canvas.convert_to_ppm("./", "output.ppm"); */
    let sides = 5;
    while !canvas.should_close() {
        tur = tur.foward(1.0, Color::red(), &mut canvas);
        tur = tur.turn(PI / 12.0);
        canvas.display();
        let delay = Duration::from_millis(100);
        sleep(delay);
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
