use super::entity::Entity;
use super::hit_group::HitGroup;
use core::ptr::NonNull;
use sdk::{Matrix3x4, Vector};

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
    name: *const i8,
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

extern "C" {
    /// Raw handle to the engine's tracer.
    pub type RawTracer;
}

/// The engine's tracer.
#[derive(Debug)]
#[repr(transparent)]
pub struct Tracer(NonNull<RawTracer>);

impl Tracer {
    pub const fn from_raw(raw: *mut RawTracer) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawTracer) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawTracer {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn point_contents(
        &self,
        position: Vector,
        mask: i32,
        entities: *const *const Entity,
    ) -> i32 {
        type PointContents = unsafe extern "C" fn(
            this: *const RawTracer,
            position: *const Vector,
            mask: i32,
            entities: *const *const Entity,
        ) -> i32;

        unsafe {
            virt::get::<PointContents>(self.virtual_table(), 0 * 8)(
                self.as_ptr(),
                &position,
                mask,
                entities,
            )
        }
    }

    pub fn clip_to_entity(
        &self,
        ray: &Ray,
        mask: u32,
        filter: *const usize,
        entities: *const usize,
    ) {
        type ClipToEntity = unsafe extern "C" fn(
            this: *const RawTracer,
            ray: *const Ray,
            mask: u32,
            filter: *const usize,
            entities: *const usize,
        );

        unsafe {
            virt::get::<ClipToEntity>(self.virtual_table(), 3 * 8)(
                self.as_ptr(),
                ray,
                mask,
                filter,
                entities,
            )
        }
    }

    pub fn trace(&self, ray: &Ray, mask: u32, filter: *const usize, entities: *const usize) {
        type Trace = unsafe extern "C" fn(
            this: *const RawTracer,
            raw: *const Ray,
            mask: u32,
            filter: *const usize,
            entities: *const usize,
        );

        unsafe {
            virt::get::<Trace>(self.virtual_table(), 5 * 8)(
                self.as_ptr(),
                ray,
                mask,
                filter,
                entities,
            )
        }
    }
}

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
