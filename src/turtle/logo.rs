use crate::tuple::{ITuple2, Tuple2};
use crate::{Canvas, Color};
use num_traits::real::Real;
use num_traits::Zero;

#[derive(Copy, Clone)]
pub struct TurtleLogo<T: Real + Zero> {
    point: Tuple2<T>,
    vector: Tuple2<T>,
}
impl<T: Real + Zero> TurtleLogo<T> {
    pub fn new(point: impl ITuple2<T>, vector: impl ITuple2<T>) -> Self {
        let point_alocated = Tuple2::from(point);
        let vector_alocated = Tuple2::from(vector);

        Self {
            point: point_alocated,
            vector: vector_alocated,
        }
    }

    pub fn point(&self) -> &Tuple2<T> {
        &self.point
    }

    pub fn vector(&self) -> &Tuple2<T> {
        &self.vector
    }

    pub fn with_vector(self, vector: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(self.point, vector)
    }

    pub fn with_point(self, point: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(point, self.vector)
    }

    pub fn forward(&mut self, step: T, color: Color, win: &mut impl Canvas) -> Self {
        let a = &self.point;
        let res = self.mov(step);
        win.draw_line::<T>(a, &res.point, color);
        res
    }
    pub fn mov(&self, step: T) -> Self {
        let x = *self.point.x() + *self.vector.x() * step;
        let y = *self.point.y() + *self.vector.y() * step;
        self.with_point(Tuple2::new(x.round(), y.round()))
    }

    pub fn resize(&self, size: T) -> Self {
        let x = *self.vector.x() * size;
        let y = *self.vector.y() * size;

        self.with_vector(Tuple2::new(x, y))
    }

    ///angle in radian
    pub fn turn(&self, angle: T) -> Self {
        let angle_cos = angle.cos();
        let angle_sin = angle.sin();

        let dx = *self.vector.x();
        let dy = *self.vector.y();
        let x = dx * angle_cos - dy * angle_sin;
        let y = dy * angle_cos + dx * angle_sin;
        self.with_vector(Tuple2::new(x, y))
    }

    pub fn scale<F>(mut self, n: usize, scaling: T, instructions: &mut F) -> Self
    where
        F: FnMut(TurtleLogo<T>) -> TurtleLogo<T>,
    {
        for _ in 0..n {
            self = instructions(self);
            self = self.resize(scaling);
        }
        self
    }

    pub fn spin<F>(mut self, n: usize, angle: T, instructions: &mut F) -> Self
    where
        F: FnMut(TurtleLogo<T>) -> TurtleLogo<T>,
    {
        for _ in 0..n {
            self = instructions(self);
            self = self.turn(angle);
        }
        self
    }
    pub fn shift<F, B>(mut self, n: usize, step: T, instructions: &mut F) -> Self
    where
        F: FnMut(TurtleLogo<T>) -> TurtleLogo<T>,
    {
        for _ in 0..n {
            self = instructions(self);
            println!("1");
            self = self.mov(step);
        }
        self
    }
}
