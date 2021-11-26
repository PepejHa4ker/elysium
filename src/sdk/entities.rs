use super::Entity;
use vptr::Virtual;

#[derive(Debug)]
pub struct Entities {
    this: *const (),
}

impl Entities {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn get(&self, index: i32) -> Option<Entity> {
        type Signature = unsafe extern "C" fn(this: *const (), index: i32) -> *const ();

        let method: Signature = unsafe { self.as_ptr().vget(3 * 8) };
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
