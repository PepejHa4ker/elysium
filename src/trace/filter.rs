use crate::managed::handle;
use core::mem;

/// Ray trace filter.
#[derive(Debug)]
#[repr(C)]
pub struct Filter {
    virtual_table: *const VTable,
    skip_entity: *const handle::Entity,
}

#[derive(Debug)]
#[repr(C)]
pub struct VTable {
    should_hit_entity:
        unsafe extern "C" fn(this: *const Filter, entity: *const handle::Entity, mask: i32) -> bool,
    get_trace_type: unsafe extern "C" fn(this: *const Filter) -> i32,
}

impl Filter {
    /// Skip an entity.
    pub fn new(entity: *const handle::Entity) -> Self {
        Self {
            virtual_table: Box::into_raw(Box::new(VTable {
                should_hit_entity,
                get_trace_type,
            })),
            skip_entity: entity,
        }
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe {
            mem::drop(&mut Box::from_raw(self.virtual_table as *mut VTable));
        }
    }
}

unsafe extern "C" fn should_hit_entity(
    this: *const Filter,
    entity: *const handle::Entity,
    mask: i32,
) -> bool {
    (*this).skip_entity != entity
}

unsafe extern "C" fn get_trace_type(this: *const Filter) -> i32 {
    // trace everything
    0
}
