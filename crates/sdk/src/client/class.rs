use super::Table;
use crate::entity::EntityId;
use crate::{ffi, Pad};
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Class {
    _pad0: Pad<16>,
    name: *const u8,
    pub table: Option<&'static Table>,
    pub(super) next: *mut Class,
    pub entity_id: EntityId,
}

impl Class {
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { ffi::str_from_ptr_nullable(self.name) }
    }
}

impl fmt::Debug for Class {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Class")
            .field("name", &self.name())
            .field("table", &self.table)
            .field("entity_id", &self.entity_id)
            .finish()
    }
}
