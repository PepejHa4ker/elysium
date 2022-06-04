use super::Property;
use crate::{ffi, Pad};
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Table {
    properties: (*const Property, i32),
    _pad0: Pad<8>,
    name: *const u8,
    _pad1: Pad<2>,
}

impl Table {
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { ffi::str_from_ptr_nullable(self.name) }
    }

    #[inline]
    pub fn properties(&self) -> &[Property] {
        let (data, len) = self.properties;

        unsafe { ffi::slice_from_i32(data, len) }
    }
}

impl fmt::Debug for Table {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Table")
            .field("properties", &self.properties())
            .field("name", &self.name())
            .finish()
    }
}
