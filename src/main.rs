use std::f64::consts::PI;

use turtle_graphics::{Canvas, Color, BLACK, WHITE};
use turtle_graphics::{TurtleLogo, WindowCanvas};
fn polygasket(
    turtle: &mut TurtleLogo<f64>,
    canvas: &mut WindowCanvas,
    color: Color,
    n: usize,
    level: usize,
) {
    if level == 0 {
        poly(turtle, canvas, color, n, 1.0, 2.0 * PI / (n as f64));
    } else {
        for _ in 0..n {
            *turtle = turtle.resize(0.5);
            polygasket(turtle, canvas, color, n, level - 1);
            *turtle = turtle.resize(2.0);
            canvas.display();
            *turtle = turtle.mov(1.0);
            canvas.display();
            *turtle = turtle.turn(2.0 * PI / (n as f64));
        }
    }
}

fn poly(
    turtle: &mut TurtleLogo<f64>,
    canvas: &mut WindowCanvas,
    color: Color,
    n: usize,
    length: f64,
    angle: f64,
) {
    for _ in 0..n {
        *turtle = turtle.forward(length, color, canvas);
        canvas.display();
        *turtle = turtle.turn(angle);
    }
}
fn main() {
    let mut canvas = WindowCanvas::new(1000, 600);
    let mut tur = TurtleLogo::new((400.0, 100.0), (200.0, 000.0));
    canvas.set_background(WHITE);

    /* polygasket(&mut tur, &mut canvas, BLACK, 5, 3); */

    polygasket(&mut tur, &mut canvas, BLACK, 7, 2);
    while !canvas.should_close() {
        canvas.display();
    }
}
