use super::UIEngine;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<11>,
    access_ui_engine: unsafe extern "C" fn(this: *const PanoramaUIEngine) -> *const UIEngine,
}

/// Panorama UI Engine.
#[repr(C)]
pub struct PanoramaUIEngine {
    vtable: &'static VTable,
}

impl PanoramaUIEngine {
    #[inline]
    pub fn access_ui_engine(&self) -> *const UIEngine {
        unsafe { (self.vtable.access_ui_engine)(self) }
    }
}
