use crate::F32Ext;
use crate::Vector;
use core::ops::{Add, Div, Mul, Sub};
use vek::vec::repr_simd::{Vec2, Vec3};

/// euler angles
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Angle {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Angle {
    /// create a new angle
    #[inline]
    pub const fn new(pitch: f32, yaw: f32) -> Self {
        Self {
            pitch,
            yaw,
            roll: 0.0,
        }
    }

    /// an angle that points straight ahead
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// an angle that points backward
    #[inline]
    pub const fn backward() -> Self {
        Self::new(0.0, f32::backward())
    }

    /// an angle that points to the left
    #[inline]
    pub const fn left() -> Self {
        Self::new(0.0, f32::left())
    }

    /// an angle that points to the right
    #[inline]
    pub const fn right() -> Self {
        Self::new(0.0, f32::right())
    }

    /// an angle that points directly up
    #[inline]
    pub const fn up() -> Self {
        Self::new(f32::up(), 0.0)
    }

    /// an angle that points directly down
    #[inline]
    pub const fn down() -> Self {
        Self::new(f32::down(), 0.0)
    }

    /// clamp pitch between -89.0 and 89.0
    #[inline]
    pub fn normalize_pitch(self) -> Self {
        Self::new(self.pitch.normalize_pitch(), self.yaw)
    }

    /// clamp yaw between -180.0 and 180.0
    #[inline]
    pub fn normalize_yaw(self) -> Self {
        Self::new(self.pitch, self.yaw.normalize_yaw())
    }

    /// clamp pitch, yaw, and roll as decribed in the other normalize methods
    #[inline]
    pub fn normalize(self) -> Self {
        Self {
            pitch: self.pitch.normalize_pitch(),
            yaw: self.yaw.normalize_yaw(),
            roll: 0.0,
        }
    }

    /// calculate the velocity of this angle (ignores roll)
    #[inline]
    pub fn velocity(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude()
    }

    /// calculate the velocity without squaring (ignores roll)
    #[inline]
    pub fn velocity_squared(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude_squared()
    }

    #[inline]
    fn to_vec2(self) -> Vec2<f32> {
        Vec2::new(self.pitch, self.yaw)
    }

    #[inline]
    fn from_vec2(Vec2 { x, y }: Vec2<f32>) -> Self {
        Self::new(x, y)
    }

    /// calculate the magnitude of this vector
    pub fn magnitude(self) -> f32 {
        Vec3::new(self.pitch, self.yaw, self.roll).magnitude()
    }

    /// calculate the magnitude without squaring
    pub fn magnitude_squared(self) -> f32 {
        Vec3::new(self.pitch, self.yaw, self.roll).magnitude_squared()
    }

    /// calculate the magnitude of y and x of this vector
    pub fn magnitude2d(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude()
    }

    /// calculate the 2d magnitude without squaring
    pub fn magnitude2d_squared(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude_squared()
    }

    pub fn is_finite(self) -> bool {
        self.pitch.is_finite() && self.yaw.is_finite() && self.roll.is_finite()
    }

    pub fn is_normal(self) -> bool {
        self.pitch.is_normal() && self.yaw.is_normal() && self.roll.is_normal()
    }

    pub fn with_angles(src: Vector, dst: Vector) -> Angle {
        let mut angle = Angle::zero();
        let delta = src - dst;
        let hypot = delta.x.hypot(delta.y);

        angle.pitch = (delta.z / hypot).atan().to_degrees();
        angle.yaw = (delta.y / delta.x).atan().to_degrees();
        angle.roll = 0.0;

        if delta.x >= 0.0 {
            angle.yaw += 180.0;
        }

        angle
    }
}

impl Add<Self> for Angle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() + rhs.to_vec2())
    }
}

impl Add<f32> for Angle {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() + rhs)
    }
}

impl Div<Self> for Angle {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() / rhs.to_vec2())
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() / rhs)
    }
}

impl Mul<Self> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() * rhs.to_vec2())
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() * rhs)
    }
}

impl Sub<Self> for Angle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_vec2(self.to_vec2() - rhs.to_vec2())
    }
}

impl Sub<f32> for Angle {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self::from_vec2(self.to_vec2() - rhs)
    }
}
