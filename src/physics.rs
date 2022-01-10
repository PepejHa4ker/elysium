use crate::managed::{handle, Managed};
use sdk::Pad;

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    pub friction: f32,
    pub elasticity: f32,
    pub density: f32,
    pub thickness: f32,
    pub dampening: f32,
    _pad0: Pad<68>,
    pub penetration_modifier: f32,
    pub damage_modifier: f32,
    pub material: u16,
}

/// Physics.
#[derive(Debug)]
#[repr(transparent)]
pub struct Physics(Managed<handle::Physics>);

impl Physics {
    pub fn new(ptr: *mut handle::Physics) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Physics) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Physics {
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

    pub fn query(&self, index: i32) -> Option<Surface> {
        type Fn = unsafe extern "C" fn(this: *const handle::Physics, index: i32) -> *const Surface;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(5)(self.as_ptr(), index);

            if ptr.is_null() {
                None
            } else {
                Some(ptr.read())
            }
        }
    }
}
