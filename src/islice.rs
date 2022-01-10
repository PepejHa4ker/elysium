use core::{ops, slice};

/// FFI-safe slice layout compatible with `{ *const T, len: i32 }`.
#[derive(Debug)]
#[repr(C)]
pub struct ISlice<T> {
    data: *const T,
    len: i32,
}

impl<T> ISlice<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.len as usize) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data as *mut T, self.len as usize) }
    }
}

impl<T> ops::Deref for ISlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> ops::DerefMut for ISlice<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
