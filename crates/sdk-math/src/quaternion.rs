use crate::F32Ext;
use core::ops::{Add, Div, Mul, Sub};
use vek::vec::repr_simd::Vec2;

/// quaternion
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Angle {
    /// create a new quaternion
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// creates a new quaternion with all elements set to zero
    pub const fn zero() -> Self {
        Self::from_xyzw(0.0, 0.0, 0.0, 0.0)
    }

    /// an angle that points backward
    pub const fn backward() -> Self {
        Self::new(0.0, f32::backward())
    }

    /// an angle that points to the left
    pub const fn left() -> Self {
        Self::new(0.0, f32::left())
    }

    /// an angle that points to the right
    pub const fn right() -> Self {
        Self::new(0.0, f32::right())
    }

    /// an angle that points directly up
    pub const fn up() -> Self {
        Self::new(f32::up(), 0.0)
    }

    /// an angle that points directly down
    pub const fn down() -> Self {
        Self::new(f32::down(), 0.0)
    }

    /// clamp pitch between -89.0 and 89.0
    pub fn normalize_pitch(self) -> Self {
        Self::new(self.pitch.normalize_pitch(), self.yaw)
    }

    /// clamp yaw between -180.0 and 180.0
    pub fn normalize_yaw(self) -> Self {
        Self::new(self.pitch, self.yaw.normalize_yaw())
    }

    /// clamp pitch, yaw, and roll as decribed in the other normalize methods
    pub fn normalize(self) -> Self {
        Self {
            pitch: self.pitch.normalize_pitch(),
            yaw: self.yaw.normalize_yaw(),
            roll: 0.0,
        }
    }

    /// calculate the velocity of this angle (ignores roll)
    pub fn velocity(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude()
    }

    /// calculate the velocity without squaring (ignores roll)
    pub fn velocity_squared(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude_squared()
    }

    fn to_vec2(self) -> Vec2<f32> {
        Vec2::new(self.pitch, self.yaw)
    }

    fn from_vec2(Vec2 { x, y }: Vec2<f32>) -> Self {
        Self::new(x, y)
    }
}

impl Add<Self> for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() + rhs.to_vec2())
    }
}

impl Add<f32> for Angle {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() + Vec2::broadcast(rhs))
    }
}

impl Div<Self> for Angle {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() / rhs.to_vec2())
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() / Vec2::broadcast(rhs))
    }
}

impl Mul<Self> for Angle {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() * rhs.to_vec2())
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() * Vec2::broadcast(rhs))
    }
}

impl Sub<Self> for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() - rhs.to_vec2())
    }
}

impl Sub<f32> for Angle {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() - Vec2::broadcast(rhs))
    }
}
