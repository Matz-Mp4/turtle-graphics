use std::{thread::sleep, time::Duration};

use turtle_graphics::{Canvas, TurtleLogo, WindowCanvas, BLACK, WHITE};

fn main() {
    let mut canvas = WindowCanvas::new(500, 700);
    let mut tur = TurtleLogo::new((0.0, 0.0), (1.0, 0.5));
    canvas.set_background(WHITE);

    let x = Duration::from_millis(10);
    while !canvas.should_close() {
        tur = tur.forward(1.0, BLACK, &mut canvas);
        canvas.display();
        sleep(x);
    }
}
