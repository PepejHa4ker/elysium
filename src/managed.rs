use core::fmt;
use core::ptr::NonNull;

pub mod handle;

mod sealed {
    pub trait Sealed {}
}

pub trait Handle: sealed::Sealed {}

macro_rules! impl_handle {
    { $ty:ty } => {
        impl sealed::Sealed for $ty {}
        impl Handle for $ty {}

        unsafe impl Send for $ty {}
        unsafe impl Sync for $ty {}
    }
}

impl_handle! { handle::Client }
impl_handle! { handle::Console }
impl_handle! { handle::ConsoleVar }
impl_handle! { handle::Debug }
impl_handle! { handle::Entity }
impl_handle! { handle::EntityList }
impl_handle! { handle::Engine }
impl_handle! { handle::Physics }
impl_handle! { handle::Material }
impl_handle! { handle::MaterialVar }
impl_handle! { handle::Materials }
impl_handle! { handle::ModelInfo }
impl_handle! { handle::ModelRender }
impl_handle! { handle::RayTracer }

#[repr(transparent)]
pub struct Managed<T: Handle + ?Sized>(NonNull<T>);

impl<T: Handle + ?Sized> Managed<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        Some(Self(NonNull::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut T) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const T {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        providence_util::virtual_table(self.as_ptr() as *const ())
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        providence_util::virtual_offset(self.as_ptr() as *const (), offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        providence_util::virtual_entry(self.as_ptr() as *const (), offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        providence_util::relative_offset(self.as_ptr() as *const (), offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        providence_util::relative_entry(self.as_ptr() as *const (), offset)
    }
}

impl<T: Handle + ?Sized> fmt::Debug for Managed<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ptr(), fmt)
    }
}
