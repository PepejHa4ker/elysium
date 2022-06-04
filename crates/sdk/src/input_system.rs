use super::vtable_validate;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<11>,
    enable_input: unsafe extern "C" fn(this: *const InputSystem, enable: bool),
    _pad1: vtable::Pad<27>,
    reset_input_state: unsafe extern "C" fn(this: *const InputSystem),
}

vtable_validate! {
    enable_input => 11,
    reset_input_state => 39,
}

/// Input System interface.
#[repr(C)]
pub struct InputSystem {
    vtable: &'static VTable,
}

impl InputSystem {
    #[inline]
    pub fn enable_input(&self, enable: bool) {
        unsafe { (self.vtable.enable_input)(self, enable) }
    }

    #[inline]
    pub fn reset_input_state(&self) {
        unsafe { (self.vtable.reset_input_state)(self) }
    }
}
