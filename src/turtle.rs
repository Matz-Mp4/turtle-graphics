use crate::Window;
use crate::{tuple::Point2D, Vector2D};

pub struct Turtle<'a> {
    point: Point2D,
    vector: Vector2D,
    win: &'a Window,
}

