use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

#[derive(Copy, Clone, Default, Debug)]
#[repr(C, align(16))]
pub struct Vec4Aligned {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4Aligned {
    /// Create a new `Vec4Aligned` from x, y coordinates.
    pub const fn from_xy(x: f32, y: f32) -> Vec4Aligned {
        Vec4Aligned {
            x,
            y,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new `Vec4Aligned` from x, y, and z coordinates.
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec4Aligned {
        Vec4Aligned { x, y, z, w: 0.0 }
    }

    /// Create a new `Vec4Aligned` from x, y, z, and w coordinates.
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec4Aligned {
        Vec4Aligned { x, y, z, w }
    }

    /// Create a new `Vec4Aligned` from an array.
    pub const fn from_array(array: [f32; 4]) -> Vec4Aligned {
        let [x, y, z, w] = array;

        Vec4Aligned { x, y, z, w }
    }

    pub const fn splat(value: f32) -> Vec4Aligned {
        Vec4Aligned {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    /// Create a new `Vec4Aligned` with all coordinates set to zero.
    pub const fn zero() -> Vec4Aligned {
        Vec4Aligned::splat(0.0)
    }

    /// Create a new `Vec4Aligned` with all coordinates set to one.
    pub const fn one() -> Vec4Aligned {
        Vec4Aligned::splat(1.0)
    }

    pub fn distance(self, other: Vec4Aligned) -> f32 {
        self.to_vec().distance(other.to_vec())
    }

    pub fn distance_squared(self, other: Vec4Aligned) -> f32 {
        self.to_vec().distance_squared(other.to_vec())
    }

    pub fn distance2d(self, other: Vec4Aligned) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }

    pub fn distance2d_squared(self, other: Vec4Aligned) -> f32 {
        self.to_vec2().distance_squared(other.to_vec2())
    }

    pub fn dot(self, other: Vec4Aligned) -> f32 {
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
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    pub fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal() && self.z.is_normal() && self.w.is_finite()
    }

    fn from_vec(vec: meth::Vec4<f32>) -> Vec4Aligned {
        Vec4Aligned::from_array(vec.to_array())
    }

    fn to_vec(self) -> meth::Vec4<f32> {
        let Vec4Aligned { x, y, z, w } = self;

        meth::Vec4::from_array([x, y, z, w])
    }

    /*fn from_vec2(vec: meth::Vec2<f32>) -> Vec4Aligned {
        let meth::Vec2 { x, y } = vec;

        Vec4Aligned::from_xy(x, y)
    }*/

    fn to_vec2(self) -> meth::Vec2<f32> {
        let Vec4Aligned { x, y, .. } = self;

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

impl_op! { Vec4Aligned, Add, AddAssign, add, add_assign, + }
impl_op! { Vec4Aligned, Div, DivAssign, div, div_assign, / }
impl_op! { Vec4Aligned, Mul, MulAssign, mul, mul_assign, * }
impl_op! { Vec4Aligned, Rem, RemAssign, rem, rem_assign, % }
impl_op! { Vec4Aligned, Sub, SubAssign, sub, sub_assign, - }
