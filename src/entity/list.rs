use crate::entity::Entity;
use crate::global::Global;
use crate::managed::{handle, Managed};

/// Entity list interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct EntityList(Managed<handle::EntityList>);

impl EntityList {
    pub fn new(ptr: *mut handle::EntityList) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::EntityList) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::EntityList {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub fn get(&self, index: i32) -> Option<Entity> {
        type Fn = unsafe extern "C" fn(this: *const handle::EntityList, i32) -> *mut handle::Entity;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(3)(self.as_ptr(), index);

            Entity::new(ptr)
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
