use turtle_graphics::{ImageCanvas, Canvas, WHITE};

fn main() {
    let mut canvas = ImageCanvas::new(600,600);
    let a = (0.0,0.0);
    let b = (600.0,600.0);
    canvas.draw_line(&a,&b, WHITE);
    canvas.convert_to_ppm("Pictures/", "canvas.ppm");

}
