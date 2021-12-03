use super::entity::Entity;
use super::hit_group::HitGroup;
use sdk::{Matrix3x4, Vector};
use vptr::Virtual;

#[derive(Debug)]
#[repr(C)]
pub struct Plane {
    normal: Vector,
    dist: f32,
    kind: u8,
    signbits: u8,
    pad: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    name: *const u8,
    surface_properties: i16,
    flags: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct Trace {
    start: Vector,
    end: Vector,
    plane: Plane,
    fraction: f32,
    contents: i32,
    disp_flags: u32,
    all_solid: bool,
    start_solid: bool,
    fraction_left_solid: f32,
    surface: Surface,
    hit_group: HitGroup,
    physics_bone: i32,
    world_surface_index: u16,
    entity_hit: *const Entity,
    hitbox: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct EngineTrace {
    this: *const (),
}

impl EngineTrace {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
    }

    pub fn get_point_contents(
        &self,
        position: Vector,
        mask: i32,
        entities: *const *const Entity,
    ) -> i32 {
        type Signature =
            unsafe extern "C" fn(this: *const (), *const Vector, i32, *const *const Entity) -> i32;

        let method: Signature = unsafe { self.as_ptr().vget(0 * 8) };

        unsafe { method(self.as_ptr(), &position, mask, entities) }
    }

    pub fn clip_to_entity(
        &self,
        ray: &Ray,
        mask: u32,
        filter: *const usize,
        entities: *const usize,
    ) {
        type Signature =
            unsafe extern "C" fn(this: *const (), *const Ray, u32, *const usize, *const usize);

        let method: Signature = unsafe { self.as_ptr().vget(3 * 8) };

        unsafe { method(self.as_ptr(), ray, mask, filter, entities) }
    }

    pub fn trace(&self, ray: &Ray, mask: u32, filter: *const usize, entities: *const usize) {
        type Signature =
            unsafe extern "C" fn(this: *const (), *const Ray, u32, *const usize, *const usize);

        let method: Signature = unsafe { self.as_ptr().vget(5 * 8) };

        unsafe { method(self.as_ptr(), ray, mask, filter, entities) }
    }
}

unsafe impl Send for EngineTrace {}
unsafe impl Sync for EngineTrace {}

#[derive(Debug)]
#[repr(C)]
pub struct Ray {
    start: Vector,
    delta: Vector,
    start_offset: Vector,
    extents: Vector,
    world_axis_transform: Matrix3x4,
    is_ray: bool,
    is_swept: bool,
}

impl Ray {
    pub fn new(start: Vector, end: Vector) -> Self {
        let delta = end - start;
        let start_offset = Vector::zero();
        let extents = Vector::zero();
        let world_axis_transform = Matrix3x4::zero();
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
        let world_axis_transform = Matrix3x4::zero();
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
