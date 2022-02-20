use crate::entity::Entity;
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

    // `GetClientNetworkable(index)->GetIClientEntity()`
    pub unsafe fn get_unchecked(&self, index: i32) -> *mut handle::Entity {
        type Fn = unsafe extern "C" fn(
            this: *const handle::EntityList,
            index: i32,
        ) -> *mut handle::Entity;

        unsafe { self.virtual_entry::<Fn>(3)(self.as_ptr(), index) }
    }

    pub fn get(&self, index: i32) -> Option<Entity> {
        type Fn =
            unsafe extern "C" fn(this: *const handle::EntityList, index: i32) -> Option<Entity>;

        unsafe { self.virtual_entry::<Fn>(3)(self.as_ptr(), index) }
    }

    pub fn from_handle(&self, handle: *const ()) -> Option<Entity> {
        type Fn = unsafe extern "C" fn(
            this: *const handle::EntityList,
            handle: *const (),
        ) -> Option<Entity>;

        unsafe { self.virtual_entry::<Fn>(4)(self.as_ptr(), handle) }
    }

    pub fn highest_entity_index(&self) -> i32 {
        type Fn = unsafe extern "C" fn(this: *const handle::EntityList) -> i32;

        unsafe { self.virtual_entry::<Fn>(6)(self.as_ptr()) }
    }
}
