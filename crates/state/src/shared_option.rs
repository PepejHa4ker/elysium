use super::Shared;

/// A mutable shared `Option<T>`.
pub struct SharedOption<T>(Shared<Option<T>>);

impl<T> SharedOption<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(Shared::new(Some(value)))
    }

    #[inline]
    pub const fn none() -> Self {
        Self(Shared::new(None))
    }

    #[inline]
    pub const fn as_ptr(&self) -> *mut Option<T> {
        self.0.as_ptr()
    }

    #[inline]
    pub const unsafe fn as_mut(&self) -> &mut T {
        let reference: &mut Option<T> = self.0.as_mut();

        reference.as_mut().unwrap_unchecked()
    }

    #[inline]
    pub const fn is_none(&self) -> bool {
        unsafe { self.0.as_mut() }.is_none()
    }

    #[inline]
    pub const unsafe fn write(&self, value: T) {
        self.0.write(Some(value));
    }

    #[inline]
    pub const unsafe fn take(&self) -> Option<T> {
        self.0.as_mut().take()
    }
}
