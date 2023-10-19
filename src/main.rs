use turtle_graphics::Canvas;
use turtle_graphics::Window;

fn test(mut test: impl Canvas) {
    let x = &mut test;
}

fn main() {
    let mut window: Window = Canvas::new(800, 800);

    while !window.should_close() {
        window.clear();
        window.display();
    }
}
