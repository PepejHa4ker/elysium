use super::entity::Entity;
use super::hit_group::HitGroup;
use core::ptr::NonNull;
use sdk::{Matrix3x4, Pad, Vector};

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Plane {
    pub normal: Vector,
    pub dist: f32,
    pub kind: u8,
    pub signbits: u8,
    pub _pad0: [u8; 2],
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Surface {
    pub name: *const i8,
    pub index: i16,
    pub flags: u16,
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Trace {
    pub start: Vector,
    pub end: Vector,
    pub plane: Plane,
    pub fraction: f32,
    pub contents: i32,
    pub disp_flags: u32,
    pub all_solid: bool,
    pub start_solid: bool,
    pub fraction_left_solid: f32,
    pub surface: Surface,
    pub hit_group: HitGroup,
    pub physics_bone: i32,
    pub world_surface_index: u16,
    pub entity_hit: *const Entity,
    pub hitbox: i32,
}

impl Trace {
    pub fn new() -> Self {
        unsafe { core::mem::transmute_copy(&[0u8; core::mem::size_of::<Trace>()]) }
    }
}

extern "C" {
    /// Raw handle to the engine's tracer.
    pub type RawTracer;
}

unsafe impl Send for RawTracer {}
unsafe impl Sync for RawTracer {}

#[derive(Debug)]
#[repr(C)]
pub struct Filter {
    _pad0: Pad<16>,
    pub skip: *const (),
}

impl Filter {
    pub fn new(skip: *const ()) -> Self {
        Self {
            _pad0: Pad::zeroed(),
            skip,
        }
    }
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

    pub fn trace(&self, ray: &Ray, mask: u32, filter: *const Filter, trace: &mut Trace) {
        type TraceFn = unsafe extern "C" fn(
            this: *const RawTracer,
            raw: *const Ray,
            mask: u32,
            filter: *const Filter,
            trace: *const Trace,
        );

        unsafe {
            virt::get::<TraceFn>(self.virtual_table(), 5 * 8)(
                self.as_ptr(),
                ray,
                mask,
                filter,
                trace,
            )
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Ray {
    pub start: Vector,
    pub delta: Vector,
    pub start_offset: Vector,
    pub extents: Vector,
    pub world_axis_transform: Matrix3x4,
    pub is_ray: bool,
    pub is_swept: bool,
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
