use crate::tuple::Tuple2;
use num_traits::Zero;
use crate::Canvas;

pub struct TurtleLogo<'a, T: Sized + Zero> {
    point: Box<dyn Tuple2<T>>,
    vector: Box<dyn Tuple2<T>>,
    win: &'a dyn Canvas,
}
