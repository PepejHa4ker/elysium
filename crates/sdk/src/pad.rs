use crate::mem;
use core::fmt;

#[repr(C)]
pub struct Pad<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Pad<SIZE> {
    /// Create some uninitialized padding.
    pub const fn uninit() -> Self
    where
        [(); core::mem::size_of::<Self>()]:,
    {
        mem::uninit()
    }

    /// Create some zeroed padding.
    pub const fn zeroed() -> Self
    where
        [(); core::mem::size_of::<Self>()]:,
    {
        mem::zeroed()
    }
}

impl<const SIZE: usize> fmt::Debug for Pad<SIZE> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("< padding >")
    }
}
