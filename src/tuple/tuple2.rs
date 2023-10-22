use num_traits::{real::Real, One, Zero};
use std::{
    convert::{AsMut, AsRef, From},
    ops::{Index, IndexMut},
};

pub trait ITuple2<T: Real> {
    fn new(x: T, y: T) -> Self
    where
        Self: Sized;
    fn x(&self) -> &T;
    fn y(&self) -> &T;
}

#[derive(Copy, Clone)]
pub struct Tuple2<T: Real + Zero> {
    x: T,
    y: T,
}

impl<T: Real + Zero> ITuple2<T> for Tuple2<T> {
    fn new(x: T, y: T) -> Self
    where
        Self: Sized,
    {
        Tuple2::new(x, y)
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl<T: Real + Zero> Tuple2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Tuple2<T> {
        Tuple2::new(T::zero(), T::zero())
    }

    pub fn from(value: impl ITuple2<T>) -> Self {
        Tuple2::new(*value.x(), *value.y())
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }
    pub fn y(&self) -> &T {
        &self.y
    }
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl<T: Real + Zero> Index<usize> for Tuple2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let i = index % 2;
        if i == 0 {
            self.x()
        } else {
            self.y()
        }
    }
}

impl<T: Real + Zero> IndexMut<usize> for Tuple2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index % 2;
        if i == 0 {
            self.x_mut()
        } else {
            self.y_mut()
        }
    }
}

//
//impl<T> AsRef<dyn Tuple2<T>> for dyn Tuple2<T> {
//    fn as_ref(&self) -> &(dyn Tuple2<T> + 'static) {
//        self
//    }
//}
//
//impl<T> AsMut<dyn Tuple2<T>> for dyn Tuple2<T> {
//    fn as_mut(&mut self) -> &mut (dyn Tuple2<T> + 'static) {
//        self
//    }
//}
