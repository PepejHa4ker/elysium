use super::Entity;
use crate::global::Global;
use crate::managed::handle;

/// A weapon.
#[derive(Debug)]
#[repr(transparent)]
pub struct Fog(Entity);

impl Fog {
    pub fn new(ptr: *mut handle::Entity) -> Option<Self> {
        Some(Self(Entity::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Entity) -> Self {
        Self(Entity::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Entity {
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

    pub fn is_enabled(&self) -> &mut bool {
        unsafe {
            &mut *(self.relative_offset(Global::handle().networked().fog.is_enabled) as *mut bool)
        }
    }

    pub fn start(&self) -> &mut f32 {
        unsafe { &mut *(self.relative_offset(Global::handle().networked().fog.start) as *mut f32) }
    }

    pub fn end(&self) -> &mut f32 {
        unsafe { &mut *(self.relative_offset(Global::handle().networked().fog.end) as *mut f32) }
    }

    pub fn far_z(&self) -> &mut f32 {
        unsafe { &mut *(self.relative_offset(Global::handle().networked().fog.far_z) as *mut f32) }
    }

    pub fn density(&self) -> &mut f32 {
        unsafe {
            &mut *(self.relative_offset(Global::handle().networked().fog.density) as *mut f32)
        }
    }

    pub fn color_primary(&self) -> &mut i32 {
        unsafe {
            &mut *(self.relative_offset(Global::handle().networked().fog.color_primary) as *mut i32)
        }
    }
}
