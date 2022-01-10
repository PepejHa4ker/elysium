use core::ptr;
use sdk::{Matrix3x4, Vector};

/// Ray to be traced.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Ray {
    pub start: Vector,
    pub delta: Vector,
    pub start_offset: Vector,
    pub extents: Vector,
    pub world_axis_transform: *const Matrix3x4,
    pub is_ray: bool,
    pub is_swept: bool,
}

impl Ray {
    pub fn new(start: Vector, end: Vector) -> Self {
        let delta = end - start;
        let start_offset = Vector::zero();
        let extents = Vector::zero();
        let world_axis_transform = ptr::null();
        let is_ray = true;
        let is_swept = delta.magnitude() != 0.0;

        Self {
            start,
            delta,
            start_offset,
            extents,
            world_axis_transform,
            is_ray,
            is_swept,
        }
    }

    pub fn with_extents(start: Vector, end: Vector, min: Vector, max: Vector) -> Self {
        let delta = end - start;
        let start_offset = (max + min) * 5.0;
        let extents = (max - min) * 0.5;
        let world_axis_transform = ptr::null();
        let is_ray = extents.magnitude() < 1e-6;
        let is_swept = delta.magnitude() != 0.0;
        let start = (start + start_offset) * -1.0;

        Self {
            start,
            delta,
            start_offset,
            extents,
            world_axis_transform,
            is_ray,
            is_swept,
        }
    }
}
