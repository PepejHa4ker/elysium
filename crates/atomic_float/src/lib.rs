use std::mem;
use std::sync::atomic::{AtomicU32, Ordering};

#[repr(transparent)]
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub const fn new(v: f32) -> Self {
        Self(AtomicU32::new(unsafe { mem::transmute(v) }))
    }

    pub fn get_mut(&mut self) -> &mut f32 {
        unsafe { mem::transmute(self.0.get_mut()) }
    }

    pub fn into_inner(self) -> f32 {
        unsafe { mem::transmute(self.0.into_inner()) }
    }

    pub fn load(&self, order: Ordering) -> f32 {
        unsafe { mem::transmute(self.0.load(order)) }
    }

    pub fn store(&self, val: f32, order: Ordering) {
        self.0.store(unsafe { mem::transmute(val) }, order);
    }

    pub fn swap(&self, val: f32, order: Ordering) -> f32 {
        unsafe { mem::transmute(self.0.swap(mem::transmute(val), order)) }
    }
}
