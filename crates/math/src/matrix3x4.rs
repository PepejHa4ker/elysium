use crate::Vec3;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4 {
    matrix: [[f32; 4]; 3],
}

impl Matrix3x4 {
    pub const fn splat(value: f32) -> Matrix3x4 {
        let matrix = [[value; 4]; 3];

        Self { matrix }
    }

    pub const fn zero() -> Matrix3x4 {
        Self::splat(0.0)
    }

    pub const fn one() -> Matrix3x4 {
        Self::splat(1.0)
    }

    pub const fn as_ptr(&self) -> *const f32 {
        self.matrix.as_ptr().cast()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.matrix.as_mut_ptr().cast()
    }

    /// Create a matrix where
    ///  x axis = forward
    ///  y axis = left
    ///  z axis = up
    ///  w axis = origin
    pub const fn from_xyzw(x: Vec3, y: Vec3, z: Vec3, w: Vec3) -> Matrix3x4 {
        Self::zero()
            .with_x_axis(x)
            .with_y_axis(y)
            .with_z_axis(z)
            .with_w_axis(w)
    }

    /// Returns the x axis (forward).
    pub const fn x_axis(&self) -> Vec3 {
        let x = self[0][0];
        let y = self[1][0];
        let z = self[2][0];

        Vec3::from_xyz(x, y, z)
    }

    /// Returns the y axis (left).
    pub const fn y_axis(&self) -> Vec3 {
        let x = self[0][1];
        let y = self[1][1];
        let z = self[2][1];

        Vec3::from_xyz(x, y, z)
    }

    /// Returns the z axis (up).
    pub const fn z_axis(&self) -> Vec3 {
        let x = self[0][2];
        let y = self[1][2];
        let z = self[2][2];

        Vec3::from_xyz(x, y, z)
    }

    /// Returns the w axis (origin).
    pub const fn w_axis(&self) -> Vec3 {
        let x = self[0][3];
        let y = self[1][3];
        let z = self[2][3];

        Vec3::from_xyz(x, y, z)
    }

    /// Set the x axis (forward).
    pub const fn with_x_axis(mut self, x: Vec3) -> Matrix3x4 {
        let Vec3 { x, y, z } = x;

        self[0][0] = x;
        self[1][0] = y;
        self[2][0] = z;
        self
    }

    /// Set the y axis (left).
    pub const fn with_y_axis(mut self, y: Vec3) -> Matrix3x4 {
        let Vec3 { x, y, z } = y;

        self[0][1] = x;
        self[1][1] = y;
        self[2][1] = z;
        self
    }

    /// Set the z axis (up).
    pub const fn with_z_axis(mut self, z: Vec3) -> Matrix3x4 {
        let Vec3 { x, y, z } = z;

        self[0][1] = x;
        self[1][1] = y;
        self[2][1] = z;
        self
    }

    /// Set the w axis (orign).
    pub const fn with_w_axis(mut self, w: Vec3) -> Matrix3x4 {
        let Vec3 { x, y, z } = w;

        self[0][1] = x;
        self[1][1] = y;
        self[2][1] = z;
        self
    }
}

impl const Deref for Matrix3x4 {
    type Target = [[f32; 4]; 3];

    fn deref(&self) -> &[[f32; 4]; 3] {
        &self.matrix
    }
}

impl const DerefMut for Matrix3x4 {
    fn deref_mut(&mut self) -> &mut [[f32; 4]; 3] {
        &mut self.matrix
    }
}
