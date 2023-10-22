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

    pub fn with_vector(self, vector: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(self.point, vector)
    }

    pub fn with_point(self, point: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(point, self.vector)
    }

    pub fn foward(&mut self, dis: impl ITuple2<T>, color: Color, win: &mut impl Canvas) -> Self {
        win.draw_line::<T>(&self.point, &dis, color);
        self.mov(dis)
    }
    pub fn mov(&self, dis: impl ITuple2<T>) -> Self {
        let x = *self.point.x() + *dis.x();
        let y = *self.point.y() + *dis.y();

        self.with_point(Tuple2::new(x, y))
    }

    pub fn resize(&self, scaling: T) -> Self {
        let x = *self.vector.x() * scaling;
        let y = *self.vector.y() * scaling;

        self.with_vector(Tuple2::new(x, y))
    }

    pub fn turn(&self, angle: T) -> Self {
        let angle_cos = angle.to_radians().cos();
        let angle_sin = angle.to_radians().sin();

        let x = *self.vector.x() * angle_cos - *self.vector.y() * angle_sin;
        let y = *self.vector.y() * angle_cos + *self.vector.x() * angle_sin;
        self.with_vector(Tuple2::new(x, y))
    }
}
