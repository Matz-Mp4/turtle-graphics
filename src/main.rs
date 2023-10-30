use std::f64::consts::PI;

use turtle_graphics::{Canvas, Color, BLACK, WHITE};
use turtle_graphics::{TurtleLogo, WindowCanvas};
fn sierpinski(turtle: &mut TurtleLogo<f64>, canvas: &mut impl Canvas, color: Color, level: usize) {
    if level == 0 {
        poly(turtle, canvas, color, 1.0, 2.0 * PI / 3.0);
    } else {
        for _ in 0..3 {
            *turtle = turtle.resize(0.5);
            sierpinski(turtle, canvas, color, level - 1);
            *turtle = turtle.resize(2.0);
            *turtle = turtle.mov(1.0);
            *turtle = turtle.turn(2.0 * PI / 3.0);
        }
    }
}
fn polygasket(
    turtle: &mut TurtleLogo<f64>,
    canvas: &mut impl Canvas,
    color: Color,
    n: usize,
    level: usize,
) {
    if level == 0 {
        poly(turtle, canvas, color, 1.0, 2.0 * PI / (n as f64));
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
    length: f64,
    angle: f64,
) {
    for _ in 0..5{
        *turtle = turtle.forward(length, color, canvas);
        *turtle = turtle.turn(angle);
    }
}
fn main() {
    let mut canvas = WindowCanvas::new(800, 800);
    let mut tur = TurtleLogo::new((200.0, 150.0), (400.0, 000.0));
    canvas.set_background(WHITE);

    polygasket(&mut tur, &mut canvas, BLACK, 5, 3);

    while !canvas.should_close() {
        canvas.display();
    }
}
