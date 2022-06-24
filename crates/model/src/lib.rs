use core::ops;
use elysium_math::{Matrix3x4, Vec3};

pub const MAX_BONES: usize = 256;

#[derive(Clone, Copy, Debug)]
pub struct Bones {
    bones: [Matrix3x4; MAX_BONES],
}

impl Bones {
    const ZERO: Matrix3x4 = Matrix3x4::zero();

    pub const fn zero() -> Bones {
        let bones = [Self::ZERO; MAX_BONES];

        Self { bones }
    }

    pub unsafe fn from_raw_parts(data_address: *mut Matrix3x4) -> Bones {
        let bones = *(data_address as *mut [Matrix3x4; MAX_BONES]);

        Self { bones }
    }

    pub fn as_ptr(&self) -> *const Matrix3x4 {
        self.bones.as_ptr() as *const Matrix3x4
    }

    pub fn as_mut_ptr(&mut self) -> *mut Matrix3x4 {
        self.bones.as_mut_ptr() as *mut Matrix3x4
    }

    pub fn get(&self, index: usize) -> Option<&Matrix3x4> {
        if index >= MAX_BONES {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    pub fn get_origin(&self, index: usize) -> Option<Vec3> {
        self.get(index).map(|bone| bone.w_axis())
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &Matrix3x4 {
        &*self.as_ptr().add(index)
    }

    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Matrix3x4 {
        &mut *self.as_mut_ptr().add(index)
    }
}

impl ops::Deref for Bones {
    type Target = [Matrix3x4; MAX_BONES];

    fn deref(&self) -> &Self::Target {
        &self.bones
    }
}
impl ops::DerefMut for Bones {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bones
    }
}
