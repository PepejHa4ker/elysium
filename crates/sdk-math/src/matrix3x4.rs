use crate::{Angle, Vector};
use core::ops::Mul;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4(pub [[f32; 4]; 3]);

// representation
// [ x, y, z, w ]
// [ x, y, z, w ]
// [ x, y, z, w ]
//
// self.0[0] = 1st row
// self.0[4] = 2nd row
// self.0[8] = 3rd row

impl Matrix3x4 {
    /// Create a new `Matrix3x4` from four vectors.
    pub fn new(x: Vector, y: Vector, z: Vector, w: Vector) -> Self {
        Self::zero().with_x(x).with_y(y).with_z(z).with_w(w)
    }

    /// Create a `Matrix3x4` with all `0.0`.
    pub fn zero() -> Self {
        Self::splat(0.0)
    }

    /// Create a `Matrix3x4` with all values set to `value`.
    pub fn splat(value: f32) -> Self {
        Self([[value; 4]; 3])
    }

    /// `x` vector.
    pub fn x(self) -> Vector {
        Vector::new(self.0[0][0], self.0[1][0], self.0[2][0])
    }

    /// `y` vector.
    pub fn y(self) -> Vector {
        Vector::new(self.0[0][1], self.0[1][1], self.0[2][1])
    }

    /// `z` vector.
    pub fn z(self) -> Vector {
        Vector::new(self.0[0][2], self.0[1][2], self.0[2][2])
    }

    /// `w` vector.
    pub fn w(self) -> Vector {
        Vector::new(self.0[0][3], self.0[1][3], self.0[2][3])
    }

    /// Set the `x` vector.
    pub fn with_x(self, vector: Vector) -> Self {
        let mut this = self;

        this.0[0][0] = vector.x;
        this.0[1][0] = vector.y;
        this.0[2][0] = vector.z;
        this
    }

    /// Set the `y` vector.
    pub fn with_y(self, vector: Vector) -> Self {
        let mut this = self;

        this.0[0][1] = vector.x;
        this.0[1][1] = vector.y;
        this.0[2][1] = vector.z;
        this
    }

    /// Set the `z` vector.
    pub fn with_z(self, vector: Vector) -> Self {
        let mut this = self;

        this.0[0][2] = vector.x;
        this.0[1][2] = vector.y;
        this.0[2][2] = vector.z;
        this
    }

    /// Set the `w` vector.
    pub fn with_w(self, vector: Vector) -> Self {
        let mut this = self;

        this.0[0][3] = vector.x;
        this.0[1][3] = vector.y;
        this.0[2][3] = vector.z;
        this
    }

    /// Create a new angled matrix.
    pub fn from_angle(angle: Angle) -> Self {
        let (sin_pitch, cos_pitch) = angle.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = angle.yaw.sin_cos();
        let (sin_roll, cos_roll) = angle.roll.sin_cos();

        let cos_roll_cos_yaw = cos_roll * cos_yaw;
        let cos_roll_sin_yaw = cos_roll * sin_yaw;
        let sin_roll_cos_yaw = sin_roll * cos_yaw;
        let sin_roll_sin_yaw = sin_roll * sin_yaw;

        let x = Vector::new(cos_pitch * cos_yaw, cos_pitch * sin_yaw, -sin_pitch);
        let y = Vector::new(
            sin_pitch * sin_roll_cos_yaw - cos_roll_sin_yaw,
            sin_pitch * sin_roll_sin_yaw + cos_roll_cos_yaw,
            sin_roll * cos_pitch,
        );

        let z = Vector::new(
            sin_pitch * cos_roll_cos_yaw + sin_roll_sin_yaw,
            sin_pitch * cos_roll_sin_yaw - sin_roll_cos_yaw,
            cos_roll * cos_pitch,
        );

        // (yaw * pitch) * roll
        Self::zero().with_x(x).with_y(y).with_z(z)
    }

    fn mul_col(&self, other: &Self, row: usize, col: usize) -> f32 {
        self.0[row][0] * other.0[0][col]
            + self.0[row][1] * other.0[1][col]
            + self.0[row][2] * other.0[2][col]
    }

    fn mul_row(&self, other: &Self, row: usize) -> [f32; 4] {
        [
            self.mul_col(other, row, 0),
            self.mul_col(other, row, 1),
            self.mul_col(other, row, 2),
            self.mul_col(other, row, 3) + self.0[row][3],
        ]
    }
}

impl Mul<Self> for Matrix3x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut output = Self::zero();

        output.0[0] = self.mul_row(&other, 0);
        output.0[1] = self.mul_row(&other, 1);
        output.0[2] = self.mul_row(&other, 2);
        output
    }
}
