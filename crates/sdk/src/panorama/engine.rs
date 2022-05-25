use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad: vtable::Pad<53>,
    dispatch_event: unsafe extern "C" fn(this: *const UIEngine, event: *const ()),
}

/// Panorama UI Engine.
#[repr(C)]
pub struct UIEngine {
    vtable: &'static VTable,
}

impl UIEngine {
    #[inline]
    pub fn dispatch_event(&self, event: *const ()) {
        unsafe { (self.vtable.dispatch_event)(self, event) }
    }
}
