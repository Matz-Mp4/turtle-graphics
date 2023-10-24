use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::ImageCanvas;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

fn main() {
    let mut canvas = WindowCanvas::new(600, 600);
    let mut tur = TurtleLogo::new(Tuple2::<f64>::zero(), Tuple2::<f64>::zero());
    tur = tur.with_point(Tuple2::new(0.0, 0.0));
    canvas.set_background(Color::white());

    /* tur.foward(Tuple2::new(600.0, 600.0), Color::red(), &mut canvas); */
    /* canvas.convert_to_ppm("./", "teste.ppm"); */
    let mut x = 1.0;
    while !canvas.should_close() {
        tur.foward(Tuple2::new(x, x), Color::black(), &mut canvas);
        canvas.display();
        tur.foward(Tuple2::new(x, 600.0), Color::blue(), &mut canvas);
        x += 1.0;
    }
}
