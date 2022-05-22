use super::entity::Entity;
use crate::contents::Contents;
use crate::managed::{handle, Managed};
use core::mem::MaybeUninit;
use core::ptr;
use elysium_math::Vec3;

pub use filter::Filter;
pub use plane::Plane;
pub use ray::Ray;
pub use summary::Summary;
pub use surface::Surface;

mod filter;
mod plane;
mod ray;
mod summary;
mod surface;

/// The ray tracing interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct RayTracer(Managed<handle::RayTracer>);

impl RayTracer {
    pub fn new(ptr: *mut handle::RayTracer) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::RayTracer) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::RayTracer {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub fn point_contents(
        &self,
        position: Vec3,
        contents: Contents,
        entities: *const *const Entity,
    ) -> Contents {
        type Fn = unsafe extern "C" fn(
            this: *const handle::RayTracer,
            position: *const Vec3,
            contents: u32,
            entities: *const *const Entity,
        ) -> Contents;

        unsafe {
            self.virtual_entry::<Fn>(0)(self.as_ptr(), &position, contents.to_u32(), entities)
        }
    }

    pub fn clip_to_entity(
        &self,
        ray: &Ray,
        contents: Contents,
        filter: *const Filter,
        entities: *const Entity,
    ) {
        type Fn = unsafe extern "C" fn(
            this: *const handle::RayTracer,
            ray: *const Ray,
            contents: u32,
            filter: *const Filter,
            entities: *const Entity,
        );

        unsafe {
            self.virtual_entry::<Fn>(4)(self.as_ptr(), ray, contents.to_u32(), filter, entities)
        }
    }

    pub unsafe fn trace_unchecked(
        &self,
        ray: *const Ray,
        contents: Contents,
        filter: *const Filter,
        summary: *mut Summary,
    ) {
        type Fn = unsafe extern "C" fn(
            this: *const handle::RayTracer,
            ray: *const Ray,
            contents: u32,
            filter: *const Filter,
            summary: *mut Summary,
        );

        self.virtual_entry::<Fn>(5)(self.as_ptr(), ray, contents.to_u32(), filter, summary);
    }

    pub unsafe fn trace_filtered_unchecked(
        &self,
        ray: *const Ray,
        contents: Contents,
        filter: *const handle::Entity,
        summary: *mut Summary,
    ) {
        let mut filter = Filter::new(filter);

        self.trace_unchecked(ray, contents, filter.as_mut_ptr(), summary);
    }

    /// Trace a ray, returning a summary of the trace.
    pub fn trace(&self, ray: &Ray, contents: Contents, filter: Option<&Entity>) -> Summary {
        let filter = match filter {
            Some(filter) => filter.as_ptr(),
            None => ptr::null::<()>() as *const handle::Entity,
        };

        unsafe {
            let mut summary = MaybeUninit::<Summary>::uninit();

            self.trace_filtered_unchecked(ray, contents, filter, summary.as_mut_ptr());

            unsafe { summary.assume_init() }
        }
    }

    /// Trace a ray, mutating the `summary` parameter rather than returning it.
    pub fn trace_mut(
        &self,
        ray: &Ray,
        contents: Contents,
        filter: Option<&Entity>,
        summary: &mut Summary,
    ) {
        let filter = match filter {
            Some(filter) => filter.as_ptr(),
            None => ptr::null::<()>() as *const handle::Entity,
        };

        unsafe {
            self.trace_filtered_unchecked(ray, contents, filter, summary);
        }
    }
}
