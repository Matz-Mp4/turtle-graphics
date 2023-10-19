use num_traits::{One, Zero};

pub trait Tuple2<T: One + Zero> {
    fn new(x: T, y: T) -> Self
    where
        Self: Sized;
    fn x() -> T
    where
        Self: Sized;
    fn y() -> T
    where
        Self: Sized;
    fn set_x(&mut self, x: T);
    fn set_y(&mut self, y: T);
    fn one() -> Self
    where
        Self: Sized,
    {
        Tuple2::new(One::one(), One::one())
    }
    fn zero() -> Self
    where
        Self: Sized,
    {
        Tuple2::new(Zero::zero(), Zero::zero())
    }
}
