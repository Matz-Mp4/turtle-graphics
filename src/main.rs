use std::ops::Index;

use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::ImageCanvas;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;
use turtle_graphics::WindowCanvas;

fn main() {
    let mut canvas = WindowCanvas::new(800, 800);
    let mut tur = TurtleLogo::new(Tuple2::<f64>::zero(), Tuple2::<f64>::zero());
    tur = tur.with_point(Tuple2::new(0.0, 0.0));

    /* tur.foward(Tuple2::new(300.0, 800.0), Color::red(), &mut canvas); */
    /* canvas.convert_to_ppm("./", "teste.ppm"); */
    let mut x = 0.0;
    while !canvas.should_close() {
        tur.foward(Tuple2::new(x, x), Color::red(), &mut canvas);
        canvas.display();
        x += 1.0;
    }
}
