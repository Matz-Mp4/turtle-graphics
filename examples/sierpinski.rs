use std::f64::consts::PI;

use turtle_graphics::{Canvas, Color, ImageCanvas, BLACK, WHITE};
use turtle_graphics::{TurtleLogo, WindowCanvas};
fn polygasket(
    turtle: &mut TurtleLogo<f64>,
    canvas: &mut impl Canvas,
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
            *turtle = turtle.mov(1.0);
            *turtle = turtle.turn(2.0 * PI / (n as f64));
        }
    }
}

fn poly(
    turtle: &mut TurtleLogo<f64>,
    canvas: &mut impl Canvas,
    color: Color,
    n: usize,
    length: f64,
    angle: f64,
) {
    for _ in 0..n {
        *turtle = turtle.forward(length, color, canvas);
        *turtle = turtle.turn(angle);
    }
}
fn main() {
    let mut canvas = ImageCanvas::new(600, 600);
    let mut tur = TurtleLogo::new((0.0, 50.0), (600.0, 000.0));
    polygasket(&mut tur, &mut canvas, WHITE, 3, 6);
    canvas.convert_to_ppm("pictures/", "sierpinski.ppm");
}
