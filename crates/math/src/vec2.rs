use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Create a new `Vec2` from x, y coordinates.
    pub const fn from_xy(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Create a new `Vec2` from x, and y coordinates, discarding z.
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec2 {
        let _ = z;

        Vec2 { x, y }
    }

    /// Create a new `Vec2` from x, and y coordinates, discarding z, and w.
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec2 {
        let _ = z;
        let _ = w;

        Vec2 { x, y }
    }

    /// Create a new `Vec2` from an array.
    pub const fn from_array(array: [f32; 2]) -> Vec2 {
        let [x, y] = array;

        Vec2 { x, y }
    }

    pub const fn splat(value: f32) -> Vec2 {
        Vec2 { x: value, y: value }
    }

    /// Create a new `Vec2` with all coordinates set to zero.
    pub const fn zero() -> Vec2 {
        Vec2::splat(0.0)
    }

    /// Create a new `Vec2` with all coordinates set to one.
    pub const fn one() -> Vec2 {
        Vec2::splat(1.0)
    }

    pub fn distance(self, other: Vec2) -> f32 {
        self.to_vec().distance(other.to_vec())
    }

    pub fn distance_squared(self, other: Vec2) -> f32 {
        self.to_vec().distance_squared(other.to_vec())
    }

    pub fn dot(self, other: Vec2) -> f32 {
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

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal()
    }

    fn from_vec(vec: meth::Vec2<f32>) -> Vec2 {
        Vec2::from_array(vec.to_array())
    }

    fn to_vec(self) -> meth::Vec2<f32> {
        let Vec2 { x, y } = self;

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

impl_op! { Vec2, Add, AddAssign, add, add_assign, + }
impl_op! { Vec2, Div, DivAssign, div, div_assign, / }
impl_op! { Vec2, Mul, MulAssign, mul, mul_assign, * }
impl_op! { Vec2, Rem, RemAssign, rem, rem_assign, % }
impl_op! { Vec2, Sub, SubAssign, sub, sub_assign, - }
