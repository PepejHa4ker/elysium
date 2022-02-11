use sdk::{Matrix3x4, Vec3};

#[repr(C)]
pub struct Bones([Matrix3x4; 256]);

impl Bones {
    pub fn zero() -> Self {
        Self([Matrix3x4::zero(); 256])
    }

    pub fn get_bone(&self, index: usize) -> Option<Matrix3x4> {
        self.0.get(index).map(|bone| *bone)
    }

    pub fn get_origin(&self, index: usize) -> Option<Vec3> {
        self.get_bone(index)
            // SAFETY: Index is always valid.
            .map(|bone| unsafe { bone.get_unchecked(3) })
    }

    pub fn get_head_bone(&self) -> Matrix3x4 {
        // SAFETY: Index is always valid.
        unsafe { self.get_bone(8).unwrap_unchecked() }
    }

    pub fn get_head_origin(&self) -> Vec3 {
        // SAFETY: Index is always valid.
        unsafe { self.get_origin(8).unwrap_unchecked() }
    }
}
