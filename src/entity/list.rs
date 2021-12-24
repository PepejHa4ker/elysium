use crate::entity::{Entity, RawEntity};
use crate::global::Global;
use core::ptr::NonNull;

extern "C" {
    /// Raw handle to the engine's entity list.
    pub type RawEntityList;
}

unsafe impl Send for RawEntityList {}
unsafe impl Sync for RawEntityList {}

/// Engine's entity list.
#[derive(Debug)]
#[repr(transparent)]
pub struct EntityList(NonNull<RawEntityList>);

impl EntityList {
    /// Creates a new `EntityList` list if `raw` is non-null.
    pub const fn from_raw(raw: *mut RawEntityList) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawEntityList) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawEntityList {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const *const u8 {
        unsafe { *(self.as_ptr() as *const *const *const u8) }
    }

    pub fn get(&self, index: i32) -> Option<Entity> {
        type Get = unsafe extern "C" fn(*const RawEntityList, i32) -> *mut RawEntity;

        unsafe {
            let raw_entity =
                virt::get::<Get>(self.virtual_table() as *const (), 24)(self.as_ptr(), index);

            Entity::from_raw(raw_entity)
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

/// Iterator over entities within the entity list.
pub struct Iter<'a> {
    entities: &'a EntityList,
    index: i32,
    len: i32,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(entities: &'a EntityList) -> Self {
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
