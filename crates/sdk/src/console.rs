use crate::ffi;
use frosting::ffi::vtable;
use std::ffi::OsStr;

#[repr(C)]
pub struct VTable {
    _pad0: vtable::Pad<15>,
    var: unsafe extern "C" fn(this: *const Console, var: *const u8) -> *const (),
    _pad1: vtable::Pad<11>,
    write: unsafe extern "C" fn(this: *const Console, fmt: *const u8, txt: *const u8),
}

#[repr(C)]
pub struct Console {
    vtable: &'static VTable,
}

impl Console {
    #[inline]
    pub fn var<S>(&self, name: S) -> *const ()
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.var)(self, ptr) }
    }

    #[inline]
    pub fn write<S>(&self, string: S)
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(string);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.write)(self, "%s\0".as_ptr(), ptr) }
    }
}
