use crate::vector::Vec3;
use core::ops::{Add, Mul, Sub};
use vek::vec::repr_simd::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4(pub [[f32; 4]; 3]);

impl Matrix3x4 {
    /// create a new Matrix3x4
    pub const fn new(x: Vec3, y: Vec3, z: Vec3, origin: Vec3) -> Self {
        let data = [
            [x.x, y.x, z.x, origin.x], // x axis
            [x.y, y.y, z.y, origin.y], // y axis
            [x.z, y.z, z.z, origin.z], // z axis
        ];

        Self(data)
    }

    /// zero... lol
    pub const fn zero() -> Self {
        Self::new(Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec3::zero())
    }
}

impl Mul<Self> for Matrix3x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        fn mul(a: [f32; 4], b: [f32; 4]) -> f32 {
            (Vec4::new(a[0], a[1], a[2], a[3]) * Vec4::new(b[0], b[1], b[2], b[3])).sum()
        }

        let mut result = Self::zero();

        for j in 0..4 {
            for i in 0..4 {
                result.0[j][i] = mul(
                    [self.0[j][0], self.0[j][1], self.0[j][2], self.0[j][3]],
                    [rhs.0[0][i], rhs.0[1][i], rhs.0[2][i], rhs.0[3][i]],
                );
            }
        }

        result
    }
}
