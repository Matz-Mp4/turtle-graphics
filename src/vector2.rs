pub mod vector2 {
    use std::ops::{Add, Sub, Mul, Div, Neg};
    
    #[derive(Clone, Copy, Debug)]
    pub struct Vector2 {
        pub x: f64,
        pub y: f64,
    }

    #[allow(dead_code)]
    type Point = Vector2; // type alias

    impl Vector2 {
        pub fn new(x: f64, y: f64) -> Vector2 {
            Vector2 { x, y }
        }

        pub fn length(&self) -> f64 {
            ((self.x * self.x) + (self.y * self.y)).sqrt()
        }

        pub fn dot_product(&self, other: &Vector2) -> f64 {
            (self.x * other.x) + (self.y * other.y)
        }

        pub fn cross_product(&self, other: &Vector2) -> Vector2 {
            Vector2 {
                x: (self.y * other.x) - (self.x * other.y),
                y: (self.x * other.y) - (self.y * other.x),
            }
        }
    }

    // Ops overloading
    // ----------------------------------------------------------------
    impl Add<Vector2> for Vector2 {
        type Output = Vector2;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub<Vector2> for Vector2 {
        type Output = Vector2;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul<Vector2> for Vector2 {
        type Output = Vector2;

        fn mul(self, other: Self) -> Self {
            Self {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

    impl Mul<f64> for Vector2 {
        type Output = Vector2;

        fn mul(self, other: f64) -> Self {
            Self {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    impl Div<Vector2> for Vector2 {
        type Output = Vector2;

        fn div(self, other: Self) -> Self {
            Self {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }

    impl Div<f64> for Vector2 {
        type Output = Vector2;

        fn div(self, other: f64) -> Self {
            Self {
                x: self.x / other,
                y: self.y / other,
            }
        }
    }

    impl Neg for Vector2 {
        type Output = Vector2;

        fn neg(self) -> Self {
            Self {
                x: -self.x,
                y: -self.y
            }
        }
    }

    impl PartialEq for Vector2 {
        fn eq(&self, other: &Vector2) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
}
