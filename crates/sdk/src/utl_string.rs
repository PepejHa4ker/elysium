use super::UtlMem;
use core::ops::{Deref, DerefMut};
use core::{slice, str};

/// Simple string class.
///
/// Source SDK: [tier1/utlstring.h](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/utlstring.h)
#[repr(C)]
pub struct UtlString {
    pub mem: UtlMem<u8>,
    pub len: i32,
}

impl UtlString {
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.mem.mem, self.len as usize) }
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline]
    pub const unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        slice::from_raw_parts_mut(self.mem.mem as *mut u8, self.len as usize)
    }

    #[inline]
    pub const fn as_mut_str(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked_mut(self.as_mut_bytes()) }
    }
}

impl Deref for UtlString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl DerefMut for UtlString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
