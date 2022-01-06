use core::ops::{Add, Div, Mul, Sub};
use vek::vec::repr_simd::Vec2;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    /// create a new vector
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// zero... lol
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// calculate the magnitude of y and x of this vector
    pub fn magnitude(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude()
    }

    /// calculate the magnitude without squaring
    pub fn magnitude_squared(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude_squared()
    }

    fn to_vec2(self) -> Vec2<f32> {
        Vec2::new(self.y, self.x)
    }

    fn from_vec2(Vec2 { x, y }: Vec2<f32>) -> Self {
        Self::new(x, y)
    }
}

impl Add<Self> for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() + rhs.to_vec2())
    }
}

impl Add<f32> for Vector2D {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() + Vec2::broadcast(rhs))
    }
}

impl Div<Self> for Vector2D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() / rhs.to_vec2())
    }
}

impl Div<f32> for Vector2D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() / Vec2::broadcast(rhs))
    }
}

impl Mul<Self> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() * rhs.to_vec2())
    }
}

impl Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() * Vec2::broadcast(rhs))
    }
}

impl Sub<Self> for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() - rhs.to_vec2())
    }
}

impl Sub<f32> for Vector2D {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() - Vec2::broadcast(rhs))
    }
}
