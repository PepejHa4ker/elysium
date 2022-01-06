use super::{opaque, util};
use core::{fmt, mem, ptr};

#[repr(C)]
pub struct Slice<T> {
    slice: [T; 0],
    opaque: opaque::Opaque,
}

impl<T> Slice<T> {
    /// Create a null terminated slice.
    ///
    /// # Safety
    ///
    /// `pointer` must be a valid address that points to a null terminated array.
    pub const unsafe fn new<'a>(ptr: *const T) -> &'a Self {
        &*(ptr as *const Slice<T>)
    }

    // NOTE: destructors cannot be evaluated at compile-time
    // pub const unsafe fn from_array<'a, const N: usize>(array: [T; N]) -> &'a Self
    // where
    //     T: ~const Drop,
    // {
    //     &*(array.as_ptr() as *const T as *const Self)
    // }

    /// Create a null terminated array from array.
    pub unsafe fn from_array<'a, const N: usize>(array: [T; N]) -> &'a Self {
        &*(array.as_ptr() as *const T as *const Self)
    }

    /// Create a null terminated array from slice.
    pub const unsafe fn from_slice(string: &[T]) -> &Self {
        &*(string as *const [T] as *const Self)
    }

    pub const fn as_ptr(&self) -> *const T {
        self as *const Self as *const T
    }

    pub const fn len(&self) -> usize
    where
        [(); mem::size_of::<T>()]: ,
    {
        util::len(self.as_ptr())
    }

    pub const fn as_slice(&self) -> &[T]
    where
        [(); mem::size_of::<T>()]: ,
    {
        unsafe { &*ptr::from_raw_parts(self.as_ptr() as *const (), self.len()) }
    }
}

impl<T: fmt::Debug> fmt::Debug for Slice<T>
where
    [(); mem::size_of::<T>()]: ,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_list().entries(self.as_slice()).finish()
    }
}
