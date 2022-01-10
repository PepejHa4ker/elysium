use crate::managed::handle;
use sdk::Pad;

/// Ray trace filter.
#[derive(Debug)]
#[repr(C)]
pub struct Filter {
    _pad0: Pad<16>,
    skip_entity: *const handle::Entity,
}

impl Filter {
    /// Skip an entity.
    pub fn new(entity: *const handle::Entity) -> Self {
        Self {
            _pad0: Pad::zeroed(),
            skip_entity: entity,
        }
    }
}
