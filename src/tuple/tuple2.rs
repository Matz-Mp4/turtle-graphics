use num_traits::{real::Real, One, Zero};
use std::convert::{AsMut, AsRef};

pub trait Tuple2<T: Real + Zero> {
    fn new(x: T, y: T) -> Self
    where
        Self: Sized;
    fn x() -> T
    where
        Self: Sized;
    fn x_as_mut_ref(&mut self) -> &mut T
    where
        Self: Sized;

    fn y_as_mut_ref(&mut self) -> &mut T
    where
        Self: Sized;
    fn zero() -> Self
    where
        Self: Sized,
    {
        Tuple2::new(T::zero(), T::zero())
    }
}

impl<T> AsRef<dyn Tuple2<T>> for dyn Tuple2<T> {
    fn as_ref(&self) -> &(dyn Tuple2<T> + 'static) {
        self
    }
}

impl<T> AsMut<dyn Tuple2<T>> for dyn Tuple2<T> {
    fn as_mut(&mut self) -> &mut (dyn Tuple2<T> + 'static) {
        self
    }
}
