use core::cell::UnsafeCell;

pub struct Shared<T>(UnsafeCell<T>);

impl<T> Shared<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    pub unsafe fn as_mut(&self) -> &mut T {
        &mut *self.0.get()
    }
}

unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}
