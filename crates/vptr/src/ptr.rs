#[inline]
const unsafe fn add_bytes<T>(ptr: *const T, count: usize) -> *const T
where
    T: Sized,
{
    (ptr as *const u8).add(count) as *const T
}

#[inline]
const unsafe fn offset_bytes<T>(ptr: *const T, count: isize) -> *const T
where
    T: Sized,
{
    (ptr as *const u8).offset(count) as *const T
}

#[inline]
const unsafe fn sub_bytes<T>(ptr: *const T, count: usize) -> *const T
where
    T: Sized,
{
    (ptr as *const u8).sub(count) as *const T
}

pub trait Pointer<T> {
    /// Calculate the offset from a pointer in bytes. (convenience for `.offset_bytes(count as isize)`).
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset, **in bytes**, cannot overflow an `isize`.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum must fit in a `usize`.
    ///
    ///
    unsafe fn add_bytes(self, count: usize) -> Self
    where
        T: Sized;

    /// Calculate the offset from a pointer in bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset, **in bytes**, cannot overflow an `isize`.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum, **in bytes** must fit in a usize.
    ///
    ///
    unsafe fn offset_bytes(self, count: isize) -> Self
    where
        T: Sized;

    /// Calculate the offset from a pointer in bytes. (convenience for
    /// `.offset_bytes((count as isize).wrapping_neg())`).
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset cannot exceed `isize::MAX` **bytes**.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum must fit in a usize.
    ///
    ///
    unsafe fn sub_bytes(self, count: usize) -> Self
    where
        T: Sized;

    /// Determine the offset for the relative pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to an `i32`, and be valid for dereferencing.
    unsafe fn relative_offset(self) -> isize;

    /// Convert a relative pointer to an absolute pointer.
    ///
    /// # Safety
    ///
    /// The pointer must uphold the conditions of `.relative_offset()`.
    ///
    /// If the computed page exceeds `isize::MAX` **bytes**, the result is Undefined Behaviour.
    ///
    unsafe fn to_absolute(self) -> Self;

    /// Convert a relative pointer to an absolute pointer.
    ///
    /// # Safety
    ///
    /// The pointer must uphold the conditions of `.relative_offset()`.
    ///
    /// If the computed page exceeds `isize::MAX` **bytes**, the result is Undefined Behaviour.
    ///
    unsafe fn to_offset_absolute(self, offset: usize, len: usize) -> Self;
}

impl<T> const Pointer<T> for *const T {
    #[inline]
    unsafe fn add_bytes(self, count: usize) -> Self
    where
        T: Sized,
    {
        add_bytes(self, count)
    }

    #[inline]
    unsafe fn offset_bytes(self, count: isize) -> Self
    where
        T: Sized,
    {
        offset_bytes(self, count)
    }

    #[inline]
    unsafe fn sub_bytes(self, count: usize) -> Self
    where
        T: Sized,
    {
        sub_bytes(self, count)
    }

    #[inline]
    unsafe fn relative_offset(self) -> isize {
        *(self as *const i32) as isize
    }

    #[inline]
    unsafe fn to_absolute(self) -> Self {
        self.add_bytes(4).offset_bytes(self.relative_offset())
    }

    #[inline]
    unsafe fn to_offset_absolute(self, offset: usize, len: usize) -> Self {
        self.offset_bytes(self.add_bytes(offset).relative_offset())
            .add_bytes(len)
    }
}

impl<T> const Pointer<T> for *mut T {
    #[inline]
    unsafe fn add_bytes(self, count: usize) -> Self
    where
        T: Sized,
    {
        add_bytes(self as *const T, count) as *mut T
    }

    #[inline]
    unsafe fn offset_bytes(self, count: isize) -> Self
    where
        T: Sized,
    {
        offset_bytes(self as *const T, count) as *mut T
    }

    #[inline]
    unsafe fn sub_bytes(self, count: usize) -> Self
    where
        T: Sized,
    {
        sub_bytes(self as *const T, count) as *mut T
    }

    #[inline]
    unsafe fn relative_offset(self) -> isize {
        *(self as *mut i32) as isize
    }

    #[inline]
    unsafe fn to_absolute(self) -> Self {
        self.add_bytes(4).offset_bytes(self.relative_offset())
    }

    #[inline]
    unsafe fn to_offset_absolute(self, offset: usize, len: usize) -> Self {
        self.offset_bytes(self.add_bytes(offset).relative_offset())
            .add_bytes(len)
    }
}
