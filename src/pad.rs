use core::fmt;

#[repr(C)]
pub struct Pad<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> fmt::Debug for Pad<SIZE> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("< padding >")
    }
}
