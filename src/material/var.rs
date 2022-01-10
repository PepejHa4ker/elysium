use crate::managed::{handle, Managed};

/// A material variable.
#[derive(Debug)]
#[repr(transparent)]
pub struct MaterialVar(Managed<handle::MaterialVar>);

impl MaterialVar {
    pub fn new(ptr: *mut handle::MaterialVar) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::MaterialVar) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::MaterialVar {
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

    pub fn set_tint(&self, r: f32, g: f32, b: f32) {
        type Fn = unsafe extern "C" fn(this: *const handle::MaterialVar, r: f32, g: f32, b: f32);

        unsafe {
            self.virtual_entry::<Fn>(12)(self.as_ptr(), r, g, b);
        }
    }
}
