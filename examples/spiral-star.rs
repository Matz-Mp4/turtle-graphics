use std::f64::consts::PI;
use turtle_graphics::Canvas;
use turtle_graphics::Color;
use turtle_graphics::ImageCanvas;
use turtle_graphics::Tuple2;
use turtle_graphics::TurtleLogo;

use turtle_graphics::WHITE;
pub fn star(
    mut tur: TurtleLogo<f64>,
    sides: usize,
    color: Color,
    can: &mut impl Canvas,
) -> TurtleLogo<f64> {
    for _ in 0..sides {
        tur = tur.forward(1.0, color, can);
        tur = tur.turn(4.0 * PI / sides as f64);
    }
    tur
}

fn main() {
    let mut canvas = ImageCanvas::new(500, 500);
    let mut turtle = TurtleLogo::new(Tuple2::new(150.0, 300.0), Tuple2::new(300.0, 0.0));
    canvas.set_background(WHITE);
    let mut star = |tur: TurtleLogo<f64>| star(tur, 5, Color::black(), &mut canvas);
    let mut scale = |tur: TurtleLogo<f64>| tur.scale(1, 0.9, &mut star);
    turtle = turtle.spin(19, -PI / 8.0, &mut scale);
    canvas.convert_to_ppm("Pictures/", "SpiralStar.ppm");
}
