use crate::Vec3;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x4([f32; 12]);

impl Matrix3x4 {
    pub const fn splat(value: f32) -> Self {
        Self([value; 12])
    }

    pub const fn zero() -> Self {
        Self::splat(0.0)
    }

    pub const fn one() -> Self {
        Self::splat(1.0)
    }

    pub const fn as_ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub const fn get(&self, index: usize) -> Option<Vec3> {
        let index = index;

        if index > 3 {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    pub const unsafe fn get_unchecked(&self, index: usize) -> Vec3 {
        let x = self.as_ptr().add(index).add(0).read();
        let y = self.as_ptr().add(index).add(4).read();
        let z = self.as_ptr().add(index).add(8).read();

        Vec3::from_xyz(x, y, z)
    }
}
