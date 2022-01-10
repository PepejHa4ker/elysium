use super::Table;
use crate::entity::EntityId;
use crate::pad::Pad;
use core::fmt;

#[non_exhaustive]
#[repr(C)]
pub struct Class {
    _pad0: Pad<16>,
    // TODO: Find a better solution.
    name: Option<&'static spirit::Str>,
    pub table: Option<&'static Table>,
    pub(super) next: *mut Class,
    pub entity_id: EntityId,
}

impl Class {
    // TODO: Find a better solution.
    pub fn name(&self) -> &'static str {
        self.name.map(|name| name.as_str()).unwrap_or("")
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Class")
            .field("name", &self.name())
            .field("table", &self.table)
            .field("entity_id", &self.entity_id)
            .finish()
    }
}
