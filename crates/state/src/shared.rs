use core::cell::UnsafeCell;

/// A mutable shared `T`.
pub struct Shared<T>(UnsafeCell<T>);

impl<T> Shared<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }

    #[inline]
    pub const unsafe fn as_mut(&self) -> &mut T {
        &mut *self.0.get()
    }

    #[inline]
    pub const unsafe fn write(&self, value: T) {
        self.as_ptr().write(value);
    }
}

unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}
