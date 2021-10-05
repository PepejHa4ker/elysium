use super::Entity;
use std::mem;

#[derive(Debug)]
pub struct Entities {
    this: *const usize,
}

impl Entities {
    pub unsafe fn from_raw(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub fn get(&self, index: i32) -> Option<Entity> {
        type Signature = unsafe extern "C" fn(this: *const usize, index: i32) -> *const usize;

        let method: Signature = unsafe { mem::transmute(vmt::get(self.as_ptr(), 3)) };
        let entity = unsafe { method(self.as_ptr(), index) };

        if entity.is_null() {
            None
        } else {
            Some(unsafe { Entity::from_raw(entity) })
        }
    }
}

unsafe impl Send for Entities {}
unsafe impl Sync for Entities {}
