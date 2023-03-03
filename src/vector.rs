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
    }

    impl Add for Vector {
        type Output = Self;
        fn add(self, rhs: Self) -> Self { Self { x: self.x+rhs.x, y: self.y+rhs.y } }
    }

    impl Sub for Vector {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self { Self { x: self.x-rhs.x, y: self.y-rhs.y } }
    }

    impl Mul<f32> for Vector {
        type Output = Self;
        fn mul(self, rhs: f32) -> Self { Self { x: self.x*rhs, y: self.y*rhs } }
    }

    impl Div<f32> for Vector {
        type Output = Self;
        fn div(self, rhs: f32) -> Self { Self { x: self.x/rhs, y: self.y/rhs } }
    }
}