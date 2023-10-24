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

    fn vector_len(&self) -> T {
        let dx = (*self.vector.x() - *self.point.x()).abs();
        let dy = (*self.vector.y() - *self.point.y()).abs();
        let res = dx * dx + dy * dy;
        res.sqrt()
    }

    pub fn with_point(self, point: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(point, self.vector)
    }

    pub fn foward(&mut self, step: T, color: Color, win: &mut impl Canvas) -> Self {
        let a = &self.point;
        let res = self.mov(step);
        win.draw_line::<T>(a, &res.point, color);
        res
    }
    pub fn mov(&self, step: T) -> Self {
        let x = *self.point.x() + *self.vector.x() * step;
        let y = *self.point.y() + *self.vector.y() * step;

        let res = self.with_point(Tuple2::new(x, y));
        res.with_vector(Tuple2::new(x + *self.vector.x(), y + *self.vector.y()))
    }

    pub fn resize(&self, size: T) -> Self {
        let x = *self.vector.x() * size;
        let y = *self.vector.y() * size;

        self.with_vector(Tuple2::new(x, y))
    }

    pub fn turn(&self, angle: T) -> Self {
        let angle_cos = angle.to_radians().cos();
        let angle_sin = angle.to_radians().sin();

        let x = *self.vector.x() * angle_cos - *self.vector.y() * angle_sin;
        let y = *self.vector.y() * angle_cos + *self.vector.x() * angle_sin;
        self.with_vector(Tuple2::new(x, y))
    }

    pub fn spin(&self, n: usize, angle: T) -> Self {
        for _ in 0..n {
            self.turn(angle);
        }
        *self
    }

    pub fn scale(&self, n: usize, scaling: T) -> Self {
        for _ in 0..n {
            self.resize(scaling);
        }
        *self
    }
    pub fn shift(&self, n: usize, step: T) -> Self {
        for _ in 0..n {
            self.mov(step);
        }
        *self
    }
}
