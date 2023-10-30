use crate::tuple::{ITuple2, Tuple2};
use crate::{Canvas, Color};
use num_traits::real::Real;
use num_traits::Zero;

/// A turtle that after each step leaves a trail connecting her previous
/// location to her new location. The TurtleLogo  is represented by a
/// point P = (x,y) and its head is represented  by a vector V = (u,v)
/// where the step size is the length of V, i.e, |V| = √(u² + v²)
#[derive(Copy, Clone, Debug)]
pub struct TurtleLogo<T: Real + Zero> {
    point: Tuple2<T>,
    vector: Tuple2<T>,
}
impl<T: Real + Zero> TurtleLogo<T> {
    ///Create a new TurtleLogo.
    ///
    ///# Example
    ///
    ///```rust,no run
    ///# use turtle_graphics::TurtleLogo;
    ///fn main() {
    ///    let mut tur = TurtleLogo::new((0.0, 0.0), (10.0, 10.0));
    ///    //Instructions with the turtle
    ///}
    ///```
    pub fn new(point: impl ITuple2<T>, vector: impl ITuple2<T>) -> Self {
        let point_alocated = Tuple2::from(point);
        let vector_alocated = Tuple2::from(vector);

        Self {
            point: point_alocated,
            vector: vector_alocated,
        }
    }

    ///Return a reference of a point and the x and y coordinate
    ///can be acess by the method x() and y(), respectively.
    ///
    ///# Example
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///    let tur = TurtleLogo::new((0.0, 0.0), (10.0, 10.0));
    ///    let x = *tur.point().x();
    ///    # assert_eq!(0.0,x);
    ///```
    ///
    pub fn point(&self) -> &Tuple2<T> {
        &self.point
    }
    ///Return a reference of a vector and the x and y coordinate
    ///can be acess by the method x() and y(), respectively.
    ///# Example
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///    let tur = TurtleLogo::new((0.0, 0.0), (10.0, 10.0));
    ///    let x = *tur.vector().x();
    ///    # assert_eq!(10.0,x);
    ///```
    pub fn vector(&self) -> &Tuple2<T> {
        &self.vector
    }

    ///Define a new TurtleLogo with the same point but with a different vector
    ///# Example
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///     let mut tur = TurtleLogo::new((42.0,69.0), (24.0,666.0));
    ///     tur = tur.with_vector((1.0,10.0));
    ///```
    pub fn with_vector(self, vector: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(self.point, vector)
    }
    ///Define a new TurtleLogo with the same vector but with a different point
    ///# Example
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///     let mut tur = TurtleLogo::new((42.0,69.0), (24.0,666.0));
    ///     tur = tur.with_point((0.0,20.0));
    ///```
    pub fn with_point(self, point: impl ITuple2<T>) -> TurtleLogo<T> {
        TurtleLogo::new(point, self.vector)
    }

    ///The turtle moves forward D steps along a straight line from her current position
    ///in the direction of her current heading, and draws a straight line from her initial
    ///position to her final position.
    ///# Example
    ///```rust
    ///# use turtle_graphics::WHITE;
    ///# use turtle_graphics::Tuple2;
    ///# use turtle_graphics::TurtleLogo;
    ///# use turtle_graphics::WindowCanvas;
    ///# use turtle_graphics::Canvas;
    ///     let mut tur = TurtleLogo::new(Tuple2::new(0.0,0.0), Tuple2::new(10.0,10.0));
    ///     let mut canvas = WindowCanvas::new(600,600);
    ///     let mut canvas = WindowCanvas::new(600,600);
    ///     tur = tur.forward(60.0,WHITE, &mut canvas);
    ///
    ///     assert_eq!(600.0, *tur.point().x());
    ///     assert_eq!(600.0, *tur.point().y());
    ///```
    pub fn forward(&mut self, step: T, color: Color, win: &mut impl Canvas) -> Self {
        let a = &self.point;
        let res = self.mov(step);
        win.draw_line::<T>(a, &res.point, color);
        res
    }
    ///The turtle moves forward D steps along a straight line from her current position
    ///in the direction of her current heading.   
    ///# Example
    ///```rust
    ///# use turtle_graphics::Tuple2;
    ///# use turtle_graphics::TurtleLogo;
    ///     let mut tur = TurtleLogo::new(Tuple2::new(0.0,0.0), Tuple2::new(100.0,100.0));
    ///     tur = tur.mov(6.0);
    ///
    ///     assert_eq!(600.0, *tur.point().x());
    ///     assert_eq!(600.0, *tur.point().y());
    ///```
    pub fn mov(&self, step: T) -> Self {
        let x = *self.point.x() + *self.vector.x() * step;
        let y = *self.point.y() + *self.vector.y() * step;
        self.with_point(Tuple2::new(x, y))
    }

    ///The turtle changes the length of its step size (direction vector) by a factor.
    ///# Example
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///# use std::f64::consts::PI;
    ///     let mut tur = TurtleLogo::new((0.0,0.0), (10.0,20.0));
    ///     tur = tur.resize(0.1);
    ///
    ///      assert_eq!(2.0, *tur.vector().y());     
    ///      assert_eq!(1.0, *tur.vector().x());
    ///```
    //
    pub fn resize(&self, size: T) -> Self {
        let x = *self.vector.x() * size;
        let y = *self.vector.y() * size;

        self.with_vector(Tuple2::new(x, y))
    }
    ///The turtle changes her heading by rotating her direction vector in the plane
    /// counterclockwise from her current heading by an angle.
    ///# Example
    ///
    ///```rust
    ///# use turtle_graphics::TurtleLogo;
    ///# use std::f64::consts::PI;
    ///     let mut tur = TurtleLogo::new((30.0,30.0), (10.0,0.0));
    ///     tur = tur.turn(-PI/2.0);
    ///
    ///```
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

    ///The turtle's head resize D steps n times
    ///# Example
    /// ```rust
    ///# use std::f64::consts::PI;
    ///# use turtle_graphics::{Canvas, ImageCanvas, Tuple2, TurtleLogo, WindowCanvas, BLACK, WHITE};
    /// let mut canvas = ImageCanvas::new(500, 500);
    /// let mut turtle = TurtleLogo::new(Tuple2::new(150.0, 300.0), Tuple2::new(300.0, 0.0));
    /// canvas.set_background(WHITE);
    ///
    /// let mut draw_triangle= |mut tur: TurtleLogo<f64>| {
    ///     for _ in 0..4 {
    ///         tur = tur.forward(1.0, WHITE, &mut canvas);
    ///         tur = tur.turn(PI / 3.0);
    ///     }
    ///    tur
    ///  };
    ///
    /// turtle = turtle.scale(2, 3.0, &mut draw_triangle);
    ///```
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
    ///The turtle changes her heading by spin her direction vector n times by an angle
    /// counterclockwise. 
    ///# Example
    /// ```rust
    ///# use std::f64::consts::PI;
    ///# use turtle_graphics::{Canvas, ImageCanvas, Tuple2, TurtleLogo, WindowCanvas, BLACK, WHITE};
    /// let mut canvas = ImageCanvas::new(500, 500);
    /// let mut turtle = TurtleLogo::new(Tuple2::new(150.0, 300.0), Tuple2::new(300.0, 0.0));
    /// canvas.set_background(WHITE);
    /// let mut draw_pentagon = |mut tur: TurtleLogo<f64>| {
    ///     for _ in 0..5 {
    ///         tur = tur.forward(1.0, WHITE, &mut canvas);
    ///         tur = tur.turn(PI / 5.0);
    ///     }
    ///     tur
    /// };
    /// turtle = turtle.spin(10, PI / 6.0, &mut draw_pentagon);
    /// ```
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

   ///The turtle moves forward D steps  n times along a straight line from her current position
   ///in the direction of her current heading.   
   ///# Example
   //// ```rust
   ///# use std::f64::consts::PI;
   ///# use turtle_graphics::{Canvas, ImageCanvas, Tuple2, TurtleLogo, WindowCanvas, BLACK, WHITE};
   /// let mut canvas = ImageCanvas::new(500, 500);
   /// let mut turtle = TurtleLogo::new(Tuple2::new(150.0, 300.0), Tuple2::new(300.0, 0.0));
   /// canvas.set_background(WHITE);
   ///
   /// let mut draw_square = |mut tur: TurtleLogo<f64>| {
   ///     for _ in 0..4 {
   ///         tur = tur.forward(1.0, WHITE, &mut canvas);
   ///         tur = tur.turn(PI / 4.0);
   ///     }
   ///    tur
   ///  };
   ///
   /// turtle = turtle.shift(2, 2.0, &mut draw_square);
   ///```
    pub fn shift<F>(mut self, n: usize, step: T, instructions: &mut F) -> Self
    where
        F: FnMut(TurtleLogo<T>) -> TurtleLogo<T>,
    {
        for _ in 0..n {
            self = instructions(self);
            self = self.mov(step);
        }
        self
    }
}
