use turtle_graphics::Window;
use turtle_graphics::Canvas;

fn main() {
    let mut window = Window::new("3D graphics from scratch! (PART 1)", 256, 256);

    while !window.should_close() {
        window.clear();
        window.display();

    }

}
