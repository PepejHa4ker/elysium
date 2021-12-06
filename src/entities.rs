use crate::entity::Entity;
use crate::global::Global;
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

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
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

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

pub struct Iter<'a> {
    entities: &'a Entities,
    index: i32,
    len: i32,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(entities: &'a Entities) -> Self {
        let len = Global::handle().globals().max_clients;

        Self {
            entities,
            index: 0,
            len,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        if self.index > self.len {
            return None;
        }

        match self.entities.get(self.index) {
            Some(entity) => {
                self.index += 1;

                Some(entity)
            }
            None => None,
        }
    }
}

unsafe impl Send for Entities {}
unsafe impl Sync for Entities {}
