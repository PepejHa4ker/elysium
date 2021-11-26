use core::ops::{Add, Mul, Sub};
use vek::vec::repr_simd::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// create a new vector
    pub const fn new(y: f32, x: f32, z: f32) -> Self {
        Self { y, x, z }
    }

    /// zero... lol
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// calculate the magnitude of this vector
    pub fn magnitude(self) -> f32 {
        Vec3::new(self.y, self.x, self.y).magnitude()
    }

    /// calculate the magnitude without squaring
    pub fn magnitude_squared(self) -> f32 {
        Vec3::new(self.y, self.x, self.y).magnitude_squared()
    }

    /// calculate the magnitude of y and x of this vector
    pub fn magnitude2d(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude()
    }

    /// calculate the 2d magnitude without squaring
    pub fn magnitude2d_squared(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude_squared()
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let Vec3 { x, y, z } = Vec3::new(self.y, self.x, self.z) + Vec3::new(rhs.y, rhs.x, rhs.z);

        Self::new(x, y, z)
    }
}

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Vec3 { x, y, z } = Vec3::new(self.y, self.x, self.z) * Vec3::new(rhs.y, rhs.x, rhs.z);

        Self::new(x, y, z)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let Vec3 { x, y, z } = Vec3::new(self.y, self.x, self.z) * Vec3::new(rhs, rhs, rhs);

        Self::new(x, y, z)
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let Vec3 { x, y, z } = Vec3::new(self.y, self.x, self.z) - Vec3::new(rhs.y, rhs.x, rhs.z);

        Self::new(x, y, z)
    }
}
