use crate::vtable_validate;
use frosting::ffi::vtable;

#[derive(Debug)]
#[repr(C)]
pub struct VTable {
    _pad0: vtable::Pad<3>,
    get: unsafe extern "C" fn(this: *const EntityList, index: i32) -> *const u8,
    from_handle: unsafe extern "C" fn(this: *const EntityList, handle: *const u8) -> *const u8,
    _pad1: vtable::Pad<1>,
    len: unsafe extern "C" fn(this: *const EntityList) -> i32,
}

vtable_validate! {
    get => 3,
    from_handle => 4,
    len => 6,
}

/// Entity list interface.
#[derive(Debug)]
#[repr(C)]
pub struct EntityList {
    vtable: &'static VTable,
}

impl EntityList {
    #[inline]
    pub fn get(&self, index: usize) -> *const u8 {
        unsafe { (self.vtable.get)(self, index as i32) }
    }

    #[inline]
    pub fn from_handle(&self, handle: *const u8) -> *const u8 {
        unsafe { (self.vtable.from_handle)(self, handle) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { (self.vtable.len)(self) as usize }
    }
}
