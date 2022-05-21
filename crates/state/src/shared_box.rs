use super::SharedOption;

/// A mutable shared `Box<T>`.
pub struct SharedBox<T>(SharedOption<Box<T>>);

impl<T> SharedBox<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        Self(SharedOption::new(Box::new(value)))
    }

    #[inline]
    pub const fn none() -> Self {
        Self(SharedOption::none())
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut Option<Box<T>> {
        self.0.as_ptr()
    }

    #[inline]
    pub const unsafe fn as_mut(&self) -> &mut T {
        self.0.as_mut()
    }

    #[inline]
    pub const fn is_none(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub unsafe fn write(&self, value: T) {
        self.0.write(Box::new(value));
    }
}
