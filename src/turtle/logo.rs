use crate::tuple::Tuple2;
use crate::Window;
use num_traits::{One, Zero};

pub struct TurtleLogo<'a, T: Sized + Zero> {
    point: Box<dyn Tuple2<T>>,
    vector: Box<dyn Tuple2<T>>,
    win: &'a Window,
}
