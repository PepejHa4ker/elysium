use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use core::ptr;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub const fn as_ptr(&self) -> *const f32 {
        ptr::addr_of!(self.x)
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut f32 {
        ptr::addr_of_mut!(self.x)
    }

    /// Create a new `Vec3` from x, y coordinates.
    #[inline]
    pub const fn from_xy(x: f32, y: f32) -> Vec3 {
        Vec3 { x, y, z: 0.0 }
    }

    /// Create a new `Vec3` from x, y, and z coordinates.
    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Create a new `Vec3` from x, y, and z coordinates, discarding w.
    #[inline]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Vec3 {
        let _ = w;

        Vec3 { x, y, z }
    }

    /// Create a new `Vec3` from an array.
    #[inline]
    pub const fn from_array(array: [f32; 3]) -> Vec3 {
        let [x, y, z] = array;

        Vec3 { x, y, z }
    }

    #[inline]
    pub fn from_angle(angle: Vec3) -> Vec3 {
        let meth::Vec2 { x, y } = angle.to_vec2().to_radians();

        let (x_sin, x_cos) = x.sin_cos();
        let (y_sin, y_cos) = y.sin_cos();

        Vec3::from_xyz(x_cos * y_cos, x_cos * y_sin, -x_sin)
    }

    #[inline]
    pub fn vector_angle(vector: Vec3) -> Self {
        let mut pitch;
        let mut yaw;

        if !(vector.x != 0.0 || vector.y != 0.0) {
            pitch = if vector.z > 0.0 { 270.0 } else { 90.0 };
            yaw = 0.0;
        } else {
            pitch = (-vector.z).atan2(vector.magnitude2d()).to_degrees();

            if pitch < 0.0 {
                pitch += 360.0;
            }

            yaw = vector.y.atan2(vector.x).to_degrees();

            if yaw < 0.0 {
                yaw += 360.0;
            }
        }

        Self::from_xy(pitch, yaw)
    }

    /// Vector to angle.
    #[inline]
    pub fn to_angle(self) -> Vec3 {
        let Vec3 { x, y, z } = self;

        let x = (-z).atan2(x.hypot(y));
        let y = y.atan2(x);
        let z = 0.0;

        Vec3::from_xyz(x, y, z)
    }

    #[inline]
    pub fn to_dir(self) -> Self {
        Self::from_xy(self.x, self.y)
    }

    #[inline]
    pub fn normalize(mut self) -> Self {
        let magnitude = self.magnitude();

        if magnitude != 0.0 {
            self.x = self.x / magnitude;
            self.y = self.y / magnitude;
            self.z = self.z / magnitude;
        } else {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 1.0;
        }

        self
    }

    #[inline]
    pub fn normalize_angle(mut self) -> Self {
        while self.x > 89.0 {
            self.x -= 180.0;
        }
        while self.x < -89.0 {
            self.x += 180.0;
        }

        while self.y > 180.0 {
            self.y -= 360.0;
        }
        while self.y < -180.0 {
            self.y += 360.0;
        }

        self
    }

    #[inline]
    pub fn clamp_angle(mut self) -> Self {
        self.x = self.x.clamp(-89.0, 89.0);
        self.y = self.y.clamp(-180.0, 180.0);
        self.z = 0.0;
        self
    }

    #[inline]
    pub fn sanitize_angle(self) -> Self {
        self.normalize_angle().clamp_angle()
    }

    #[inline]
    pub fn angle_vector(&self) -> (Vec3, Vec3, Vec3) {
        let angle = self.to_vec().to_radians();
        let (sin_pitch, cos_pitch) = angle.x.sin_cos();
        let (sin_yaw, cos_yaw) = angle.y.sin_cos();
        let (sin_roll, cos_roll) = angle.z.sin_cos();

        let mut forward = Vec3::zero();
        let mut right = Vec3::zero();
        let mut up = Vec3::zero();

        forward.x = cos_pitch * cos_yaw;
        forward.y = cos_pitch * sin_yaw;
        forward.z = -sin_pitch;

        right.x = -sin_roll * sin_pitch * cos_yaw + cos_roll * sin_yaw;
        right.y = -sin_roll * sin_pitch * sin_yaw - cos_roll * cos_yaw;
        right.z = -sin_roll * cos_pitch;

        up.x = cos_roll * sin_pitch * cos_yaw + sin_roll * sin_yaw;
        up.y = cos_roll * sin_pitch * sin_yaw - sin_roll * cos_yaw;
        up.z = cos_roll * cos_pitch;

        (forward, right, up)
    }

    #[inline]
    pub const fn splat(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }

    /// Create a new `Vec3` with all coordinates set to zero.
    #[inline]
    pub const fn zero() -> Vec3 {
        Vec3::splat(0.0)
    }

    /// Create a new `Vec3` with all coordinates set to one.
    #[inline]
    pub const fn one() -> Vec3 {
        Vec3::splat(1.0)
    }

    #[inline]
    pub const fn distance(self, other: Vec3) -> f32 {
        self.to_vec().distance(other.to_vec())
    }

    #[inline]
    pub const fn distance_squared(self, other: Vec3) -> f32 {
        self.to_vec().distance_squared(other.to_vec())
    }

    #[inline]
    pub const fn distance2d(self, other: Vec3) -> f32 {
        self.to_vec2().distance(other.to_vec2())
    }

    #[inline]
    pub const fn distance2d_squared(self, other: Vec3) -> f32 {
        self.to_vec2().distance_squared(other.to_vec2())
    }

    #[inline]
    pub const fn dot(self, other: Vec3) -> f32 {
        self.to_vec().dot(other.to_vec())
    }

    /// Calculate the magnitude (length).
    #[inline]
    pub const fn magnitude(self) -> f32 {
        self.to_vec().magnitude()
    }

    /// Calculate the magnitude (length) without squaring.
    #[inline]
    pub const fn magnitude_squared(self) -> f32 {
        self.to_vec().magnitude_squared()
    }

    /// Calculate the magnitude (length) of y and x.
    #[inline]
    pub const fn magnitude2d(self) -> f32 {
        self.to_vec2().magnitude()
    }

    /// Calculate the magnitude (length) of y and x without squaring.
    #[inline]
    pub const fn magnitude2d_squared(self) -> f32 {
        self.to_vec2().magnitude_squared()
    }

    #[inline]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[inline]
    pub const fn is_normal(self) -> bool {
        self.x.is_normal() && self.y.is_normal() && self.z.is_normal()
    }

    #[inline]
    const fn from_vec(vec: meth::Vec3<f32>) -> Vec3 {
        Vec3::from_array(vec.to_array())
    }

    #[inline]
    const fn to_vec(self) -> meth::Vec3<f32> {
        let Vec3 { x, y, z } = self;

        meth::Vec3::from_array([x, y, z])
    }

    #[inline]
    const fn to_vec2(self) -> meth::Vec2<f32> {
        let Vec3 { x, y, .. } = self;

        meth::Vec2::from_array([x, y])
    }
}

macro_rules! impl_op {
    { $ty:ident, $trait:ident, $trait_assign:ident, $fn:ident, $fn_assign:ident, $op:tt } => {
        impl const $trait < $ty > for $ty {
            type Output = $ty;

            fn $fn(self, other: $ty) -> $ty {
                $ty::from_vec(self.to_vec() $op other.to_vec())
            }
        }

        impl const $trait_assign < $ty > for $ty {
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

#[cfg(test)]
pub mod test {
    use super::Vec3;

    #[test]
    fn vec3_from_xyz() {
        assert_eq!(
            Vec3::from_xyz(1.0, 2.0, 3.0),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
    }

    #[test]
    fn vec3_add() {
        assert_eq!(
            Vec3::from_xyz(1.0, 2.0, 3.0) + Vec3::from_xyz(1.0, 2.0, 3.0),
            Vec3::from_xyz(2.0, 4.0, 6.0),
        );
    }

    #[test]
    fn vec3_to_angle() {
        assert_eq!(
            Vec3::from_xyz(89.0, 360.0, 0.0).to_angle(),
            Vec3::from_xyz(0.0, 1.5707964, 0.0),
        );
    }

    #[test]
    fn vec3_to_trusted() {
        assert_eq!(
            Vec3::from_xyz(89.0, 360.0, 0.0).to_trusted(),
            Vec3::from_xyz(89.0, 0.0, 0.0),
        );
    }

    #[test]
    fn vec3_angle_vector() {
        assert_eq!(
            Vec3::from_xyz(89.0, 360.0, 0.0).angle_vector(),
            (Vec3::zero(), Vec3::zero(), Vec3::zero()),
        );
    }
}
