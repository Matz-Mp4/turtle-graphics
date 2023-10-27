pub mod color;
pub use color::Color;
pub use color::BLACK;
pub use color::WHITE;
pub use color::RED;
pub use color::GREEN;
pub use color::BLUE;

pub mod canvas; 
pub use canvas::Canvas;
pub use canvas::WindowCanvas;
pub use canvas::ImageCanvas;

pub mod tuple;
pub use tuple::Tuple2;
pub use tuple::ITuple2;
pub mod turtle;
pub use turtle::TurtleLogo;
