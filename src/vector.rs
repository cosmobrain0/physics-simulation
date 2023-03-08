pub mod vector {
    use std::ops::*;

    #[derive(Debug, Clone, Copy)]
    pub struct Vector {
        pub x: f32,
        pub y: f32
    }
    
    impl Vector {
        pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
        pub fn length(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
        pub fn sqr_length(&self) -> f32 { self.x*self.x + self.y*self.y }
        pub fn angle(&self) -> f32 { f32::atan2(self.y, self.x) }
        pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
        pub fn right() -> Self { Self { x: 1.0, y: 0.0 } }
        pub fn up() -> Self { Self { x: 0.0, y: 1.0 } }
        pub fn rotate(&self, delta: f32) -> Self {
            // matrix is
            // cos -sin
            // sin cos
            let cos = delta.cos();
            let sin = delta.sin();
            Self {
                x: cos*self.x + sin*self.y,
                y: cos*self.y - sin*self.x
            }
        }
        pub fn dot(&self, other: &Vector) -> f32 { self.x*other.x + self.y*other.y }
        pub fn project(&self, project_to: &Vector) -> Vector { *project_to * (self.dot(project_to) / project_to.dot(project_to)) }
        pub fn clockwise_90deg(&self) -> Vector { Vector { x: self.y, y: -self.x } }
        pub fn anticlockwise_90deg(&self) -> Vector { Vector { x: -self.y, y: self.x } }
    }

    impl Add for Vector {
        type Output = Self;
        fn add(self, rhs: Self) -> Self { Self { x: self.x+rhs.x, y: self.y+rhs.y } }
    }

    impl AddAssign for Vector {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl Sub for Vector {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self { Self { x: self.x-rhs.x, y: self.y-rhs.y } }
    }

    impl SubAssign for Vector {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl Mul<f32> for Vector {
        type Output = Self;
        fn mul(self, rhs: f32) -> Self { Self { x: self.x*rhs, y: self.y*rhs } }
    }

    impl MulAssign<f32> for Vector {
        fn mul_assign(&mut self, rhs: f32) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }

    impl Div<f32> for Vector {
        type Output = Self;
        fn div(self, rhs: f32) -> Self { Self { x: self.x/rhs, y: self.y/rhs } }
    }

    impl DivAssign<f32> for Vector {
        fn div_assign(&mut self, rhs: f32) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }
}