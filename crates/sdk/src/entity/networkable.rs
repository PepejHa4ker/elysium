use crate::vtable_validate;
use frosting::ffi::vtable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum DataUpdateKind {
    Created = 0,
    Changed = 1,
}

#[derive(Debug)]
#[repr(C)]
struct VTable {
    drop: unsafe extern "C" fn(this: *const Networkable),
    release: unsafe extern "C" fn(this: *const Networkable),
    client_class: unsafe extern "C" fn(this: *const Networkable) -> *const u8,
    _pad1: vtable::Pad<3>,
    pre_data_update: unsafe extern "C" fn(this: *const Networkable, update_kind: DataUpdateKind),
    _pad2: vtable::Pad<2>,
    is_dormant: unsafe extern "C" fn(this: *const Networkable) -> bool,
    index: unsafe extern "C" fn(this: *const Networkable) -> i32,
    _pad3: vtable::Pad<2>,
    set_destroyed_on_recreate_entities: unsafe extern "C" fn(this: *const Networkable),
}

vtable_validate! {
    drop => 0,
    release => 1,
    client_class => 2,
    pre_data_update => 6,
    is_dormant => 9,
    index => 10,
    set_destroyed_on_recreate_entities => 13,
}

#[derive(Debug)]
#[repr(C)]
pub struct Networkable {
    vtable: &'static VTable,
}

impl Networkable {
    #[inline]
    pub fn release(&self) {
        unsafe { (self.vtable.release)(self) }
    }

    #[inline]
    pub fn client_class(&self) -> *const u8 {
        unsafe { (self.vtable.client_class)(self) }
    }

    #[inline]
    pub fn pre_data_update(&self, update_kind: DataUpdateKind) {
        unsafe { (self.vtable.pre_data_update)(self, update_kind) }
    }

    #[inline]
    pub fn is_dormant(&self) -> bool {
        unsafe { (self.vtable.is_dormant)(self) }
    }

    #[inline]
    pub fn index(&self) -> i32 {
        unsafe { (self.vtable.index)(self) }
    }
}
