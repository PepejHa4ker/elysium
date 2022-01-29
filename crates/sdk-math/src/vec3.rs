use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Create a new `Vec3` from x, y coordinates.
    pub const fn from_xy(x: f32, y: f32) -> Vec3 {
        Vec3 { x, y, z: 0.0 }
    }

    /// Create a new `Vec3` from x, y, and z coordinates.
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create a new `Vec3` from x, y, and z coordinates, discarding w.
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec3 {
        let _ = w;

        Vec3 { x, y, z }
    }

    /// Create a new `Vec3` from an array.
    pub const fn from_array(array: [f32; 3]) -> Vec3 {
        let [x, y, z] = array;

        Vec3 { x, y, z }
    }

    pub fn from_angle(angle: Vec3) -> Vec3 {
        let meth::Vec2 { x, y } = angle.to_vec2().to_radians();

        let (x_sin, x_cos) = x.sin_cos();
        let (y_sin, y_cos) = y.sin_cos();

        Vec3::from_xyz(x_cos * y_cos, x_cos * y_sin, -x_sin)
    }

    pub fn to_angle(self) -> Vec3 {
        let Vec3 { x, y, z } = self;

        Vec3::from_xyz((-z).atan2(x.hypot(y)), y.atan2(x), 0.0)
    }

    pub fn normalize_angle(self) -> Vec3 {
        let Vec3 { x, y, z } = self;

        let x = if x.is_finite() { x % 360.0 } else { 0.0 };
        let y = if y.is_finite() { y % 360.0 } else { 0.0 };
        let z = 0.0;

        Vec3 { x, y, z }
    }

    pub fn normalize_angle_mut(&mut self) {
        *self = (*self).normalize_angle();
    }

    pub const fn splat(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    /// Create a new `Vec3` with all coordinates set to zero.
    pub const fn zero() -> Vec3 {
        Vec3::splat(0.0)
    }

    /// Create a new `Vec3` with all coordinates set to one.
    pub const fn one() -> Vec3 {
        Vec3::splat(1.0)
    }

    pub fn distance(self, other: Vec3) -> f32 {
        self.to_vec().distance(other.to_vec())
    }

    pub fn distance_squared(self, other: Vec3) -> f32 {
        self.to_vec().distance_squared(other.to_vec())
    }

    pub fn distance2d(self, other: Vec3) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }

    pub fn distance2d_squared(self, other: Vec3) -> f32 {
        self.to_vec2().distance_squared(other.to_vec2())
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.to_vec().dot(other.to_vec())
    }

    /// Calculate the magnitude (length).
    pub fn magnitude(self) -> f32 {
        self.to_vec().magnitude()
    }

    /// Calculate the magnitude (length) without squaring.
    pub fn magnitude_squared(self) -> f32 {
        self.to_vec().magnitude_squared()
    }

    /// Calculate the magnitude (length) of y and x.
    pub fn magnitude2d(self) -> f32 {
        self.to_vec2().magnitude()
    }

    /// Calculate the magnitude (length) of y and x without squaring.
    pub fn magnitude2d_squared(self) -> f32 {
        self.to_vec2().magnitude_squared()
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal() && self.z.is_normal()
    }

    fn from_vec(vec: meth::Vec3<f32>) -> Vec3 {
        Vec3::from_array(vec.to_array())
    }

    fn to_vec(self) -> meth::Vec3<f32> {
        let Vec3 { x, y, z } = self;

        meth::Vec3::from_array([x, y, z])
    }

    fn to_vec2(self) -> meth::Vec2<f32> {
        let Vec3 { x, y, .. } = self;

        meth::Vec2::from_array([x, y])
    }
}

macro_rules! impl_op {
    { $ty:ident, $trait:ident, $trait_assign:ident, $fn:ident, $fn_assign:ident, $op:tt } => {
        impl $trait < $ty > for $ty {
            type Output = $ty;

            fn $fn(self, other: $ty) -> $ty {
                $ty::from_vec(self.to_vec() $op other.to_vec())
            }
        }

        impl $trait_assign < $ty > for $ty {
            fn $fn_assign(&mut self, other: $ty) {
                *self = *self $op other;
            }
        }
    }
}

impl_op! { Vec3, Add, AddAssign, add, add_assign, + }
impl_op! { Vec3, Div, DivAssign, div, div_assign, / }
impl_op! { Vec3, Mul, MulAssign, mul, mul_assign, * }
impl_op! { Vec3, Rem, RemAssign, rem, rem_assign, % }
impl_op! { Vec3, Sub, SubAssign, sub, sub_assign, - }
