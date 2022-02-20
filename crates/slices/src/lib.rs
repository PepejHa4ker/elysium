#![no_std]

use core::{ops, slice};

/// A slice using an `i32` as the length. (`{ data_address: *const T, len: i32 }`).
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SliceI32<T> {
    data_address: *mut T,
    len: i32,
}

impl<T> SliceI32<T> {
    pub unsafe fn from_raw_parts(data_address: *mut T, len: i32) -> Self {
        Self { data_address, len }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data_address as *const T
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data_address
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    /// Return the length of this slice. Negative length is treated as empty.
    pub fn len(&self) -> usize {
        len.max(0) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> ops::Deref for SliceI32<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> ops::DerefMut for SliceI32<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
