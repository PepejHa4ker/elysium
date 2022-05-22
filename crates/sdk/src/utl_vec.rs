use super::UtlMem;
use core::ops::{Deref, DerefMut};
use core::slice;

#[repr(C)]
pub struct UtlVec<T> {
    pub mem: UtlMem<T>,
    pub len: i32,
    pub elements: *const T,
}

impl<T> UtlVec<T> {
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.mem.mem, self.len as usize) }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.mem.mem as *mut T, self.len as usize) }
    }
}

impl<T> Deref for UtlVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for UtlVec<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
