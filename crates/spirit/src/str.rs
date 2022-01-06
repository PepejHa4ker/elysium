use super::Slice;
use core::{fmt, ptr};

#[repr(C)]
pub struct Str {
    inner: Slice<u8>,
}

impl Str {
    /// Create a null terminated string.
    ///
    /// # Safety
    ///
    /// `pointer` must be a valid address that points to a null terminated string.
    pub const unsafe fn new<'a>(ptr: *const u8) -> &'a Self {
        &*(ptr as *const Self)
    }

    /// Create a null terminated array from byte array.
    pub const unsafe fn from_array<'a, const N: usize>(array: [u8; N]) -> &'a Self {
        &*(array.as_ptr() as *const u8 as *const Self)
    }

    /// Create a null terminated string from bytes.
    pub const unsafe fn from_bytes(bytes: &[u8]) -> &Self {
        &*(bytes as *const [u8] as *const Self)
    }

    /// Create a null terminated string from `&str`.
    pub const unsafe fn from_str(string: &str) -> &Self {
        &*(string as *const str as *const Self)
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub const fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    pub const fn as_str(&self) -> &str {
        unsafe { &*ptr::from_raw_parts(self.as_ptr() as *const (), self.len()) }
    }
}

impl fmt::Display for Str {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{}", self.as_str()))
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{:?}", self.as_str()))
    }
}
