use crate::ffi;
use frosting::ffi::vtable;
use std::ffi::OsStr;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<49>,
    get_child_count: unsafe extern "C" fn(this: *const UIPanel) -> i32,
    get_child: unsafe extern "C" fn(this: *const UIPanel, index: i32) -> *const UIPanel,
    _pad1: vtable::Pad<89>,
    has_class: unsafe extern "C" fn(this: *const UIPanel, name: *const u8) -> bool,
    _pad2: vtable::Pad<6>,
    set_has_class: unsafe extern "C" fn(this: *const UIPanel, name: *const u8, has_class: bool),
    _pad3: vtable::Pad<133>,
    get_attribute_f32:
        unsafe extern "C" fn(this: *const UIPanel, name: *const u8, default_value: f32) -> f32,
    _pad4: vtable::Pad<5>,
    set_attribute_f32: unsafe extern "C" fn(this: *const UIPanel, name: *const u8, value: f32),
}

/// A panorama UI panel.
#[repr(C)]
pub struct UIPanel {
    vtable: &'static VTable,
}

impl UIPanel {
    #[inline]
    pub fn get_child_count(&self) -> i32 {
        unsafe { (self.vtable.get_child_count)(self) }
    }

    #[inline]
    pub fn get_child(&self, index: i32) -> *const UIPanel {
        unsafe { (self.vtable.get_child)(self, index) }
    }

    #[inline]
    pub fn has_class<S>(&self, name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.has_class)(self, ptr) }
    }

    #[inline]
    pub fn set_has_class<S>(&self, name: S, has_class: bool)
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.set_has_class)(self, ptr, has_class) }
    }

    #[inline]
    pub fn get_attribute_f32<S>(&self, name: S, default_value: f32) -> f32
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.get_attribute_f32)(self, ptr, default_value) }
    }

    #[inline]
    pub fn set_attribute_f32<S>(&self, name: S, value: f32)
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.set_attribute_f32)(self, ptr, value) }
    }
}
