//! Trace interface

use core::mem::MaybeUninit;
use elysium_math::Vec3;
use frosting::ffi::vtable;

pub use contents::Contents;
pub use displacement::Displacement;
pub use hit_group::HitGroup;
pub use mask::Mask;
pub use plane::Plane;
pub use ray::Ray;
pub use summary::Summary;
pub use surf::Surf;
pub use surface::Surface;
pub use tex::Tex;

mod contents;
mod displacement;
mod filter;
mod hit_group;
mod mask;
mod plane;
mod ray;
mod summary;
mod surf;
mod surface;
mod tex;

/// A trait used to customize what a trace will yield.
pub trait Filter {
    fn should_hit_entity(&self, entity: *const (), mask: i32) -> bool;
    fn get_trace_kind(&self) -> TraceKind;
}

#[repr(C)]
struct VTable {
    point_contents: unsafe extern "C" fn(
        this: *const Trace,
        position: *const Vec3,
        contents: u32,
        entities: *const *const (),
    ) -> u32,
    _pad0: vtable::Pad<3>,
    clip_to_entity: unsafe extern "C" fn(
        this: *const Trace,
        ray: *const Ray,
        contents: u32,
        filter: *const (),
        entities: *const (),
    ),
    trace: unsafe extern "C" fn(
        this: *const Trace,
        ray: *const Ray,
        contents: u32,
        filter: *const (),
        summary: *mut Summary,
    ),
}

/// Trace engine!!!
#[repr(C)]
pub struct Trace {
    vtable: &'static VTable,
}

impl Trace {
    /// Return contents at a given point.
    pub fn point_contents(&self, position: Vec3, contents: u32, entities: *const *const ()) -> u32 {
        unsafe { (self.vtable.point_contents)(self, &position, contents, entities) }
    }

    /// Clip to the provided entity.
    pub fn clip_to_entity<F>(&self, ray: Ray, contents: u32, filter: F, entities: *const ())
    where
        F: Filter,
    {
        let filter = filter::Filter::new(filter);

        unsafe { (self.vtable.clip_to_entity)(self, &ray, contents, filter.as_ptr(), entities) }
    }

    /// Perform a trace.
    pub fn trace<F>(&self, ray: Ray, contents: u32, filter: F) -> Summary
    where
        F: Filter,
    {
        let filter = filter::Filter::new(filter);
        let mut summary = MaybeUninit::uninit();

        unsafe {
            (self.vtable.trace)(self, &ray, contents, filter.as_ptr(), summary.as_mut_ptr());

            summary.assume_init()
        }
    }
}

/// Kind of trace to perform.
#[repr(i32)]
pub enum TraceKind {
    Everything = 0,
    /// this does not test static props
    WorldOnly = 1,
    /// this version will not test static props
    EntitiesOnly = 2,
    /// everything filter props
    EverythingFilterProps = 3,
}
