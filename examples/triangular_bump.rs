use std::f64::consts::PI;

use turtle_graphics::{Canvas, Color, BLACK, WHITE};
use turtle_graphics::{TurtleLogo, WindowCanvas};

fn triangular_bump(turtle: &mut TurtleLogo<f64>, color: Color, canvas: &mut WindowCanvas) {
    *turtle = turtle.resize(1.0 / 3.0);
    *turtle = turtle.forward(1.0, color, canvas);
    *turtle = turtle.turn(PI / 3.0);
    *turtle = turtle.forward(1.0, color, canvas);
    *turtle = turtle.turn(-2.0 * PI / 3.0);
    *turtle = turtle.forward(1.0, color, canvas);
    *turtle = turtle.turn(PI / 3.0);
    *turtle = turtle.forward(1.0, color, canvas);
    *turtle = turtle.resize(3.0);
}

fn triangular_bump_fractal(
    level: usize,
    turtle: &mut TurtleLogo<f64>,
    color: Color,
    canvas: &mut WindowCanvas,
) {
    if level == 0 {
        *turtle = turtle.forward(1.0, color, canvas);
    } else {
        *turtle = turtle.resize(1.0 / 3.0);
        triangular_bump_fractal(level - 1, turtle, color, canvas);
        *turtle = turtle.turn(PI / 3.0);
        triangular_bump_fractal(level - 1, turtle, color, canvas);
        *turtle = turtle.turn(-2.0 * PI / 3.0);
        triangular_bump_fractal(level - 1, turtle, color, canvas);
        *turtle = turtle.turn(PI / 3.0);
        triangular_bump_fractal(level - 1, turtle, color, canvas);
        *turtle = turtle.resize(3.0);
        canvas.display();
    }
}

fn main() {
    let mut canvas = WindowCanvas::new(600, 600);
    let mut tur = TurtleLogo::new((000.0, 300.0), (600.0, 000.0));
    /* canvas.set_background(WHITE); */

    triangular_bump_fractal(6, &mut tur, WHITE, &mut canvas);
    while !canvas.should_close() {
        canvas.display();
    }
}
