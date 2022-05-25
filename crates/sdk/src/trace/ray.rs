use crate::Pad;
use elysium_math::{Vec3, Vec4Aligned};

/// Ray to be traced.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Ray {
    pub start: Vec4Aligned,
    _pad0: Pad<4>,
    pub delta: Vec4Aligned,
    _pad1: Pad<44>,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        let delta = end - start;
        let is_ray = true;
        let is_swept = delta.magnitude() != 0.0;

        let delta = Vec4Aligned::from_xyz(delta.x, delta.y, delta.z);
        let start = Vec4Aligned::from_xyz(start.x, start.y, start.z);

        Self {
            start,
            _pad0: Pad::uninit(),
            delta,
            _pad1: Pad::uninit(),
            is_ray,
            is_swept,
        }
    }
}
