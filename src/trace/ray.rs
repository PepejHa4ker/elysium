use core::ptr;
use sdk::{Matrix3x4, Pad, Vector};

/// Ray to be traced.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Ray {
    pub start: Vector,
    _pad0: Pad<4>,
    pub delta: Vector,
    _pad1: Pad<44>,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vector, end: Vector) -> Self {
        let delta = end - start;
        let is_ray = true;
        let is_swept = delta.magnitude() != 0.0;

        Self {
            start,
            _pad0: Pad::zeroed(),
            delta,
            _pad1: Pad::zeroed(),
            is_ray,
            is_swept,
        }
    }
}
