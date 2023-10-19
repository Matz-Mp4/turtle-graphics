use crate::tuple::Tuple2;
use crate::Window;
use num_traits::{One, Zero};

pub struct Turtle<'a, T: Sized + One + Zero> {
    point: Box<dyn Tuple2<T>>,
    vector: Box<dyn Tuple2<T>>,
    win: &'a Window,
}
