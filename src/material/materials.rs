use super::Material;
use crate::managed::{handle, Managed};
use elysium_sdk_material::MaterialKind;

/// Materials interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct Materials(Managed<handle::Materials>);

impl Materials {
    pub fn new(ptr: *mut handle::Materials) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Materials) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Materials {
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

    pub fn create(&self, name: *const u8, settings: *const ()) {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Materials,
            name: *const u8,
            settings: *const (),
        );

        unsafe {
            self.virtual_entry::<Fn>(83)(self.as_ptr(), name, settings);
        }
    }

    /*pub fn find(
        &self,
        kind: MaterialKind,
        texture_group_name: *const u8,
        complain: bool,
        complain_prefix: *const u8,
    ) -> Option<Material> {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Materials,
            name: *const u8,
            texture_group_name: *const u8,
            complain: bool,
            complain_prefix: *const u8,
        ) -> *mut handle::Material;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(84)(
                self.as_ptr(),
                kind.as_ptr(),
                texture_group_name,
                complain,
                complain_prefix,
            );

            Material::new(ptr)
        }
    }*/
}
