use crate::vector::Vector;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4(pub [[f32; 4]; 3]);

impl Matrix3x4 {
    /// create a new Matrix3x4
    pub const fn new(x: Vector, y: Vector, z: Vector, origin: Vector) -> Self {
        let data = [
            [x.x, y.x, z.x, origin.x], // x axis
            [x.y, y.y, z.y, origin.y], // y axis
            [x.z, y.z, z.z, origin.z], // z axis
        ];

        Self(data)
    }

    /// zero... lol
    pub const fn zero() -> Self {
        Self::new(
            Vector::zero(),
            Vector::zero(),
            Vector::zero(),
            Vector::zero(),
        )
    }
}
