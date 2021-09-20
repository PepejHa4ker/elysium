use crate::Result;
use std::ffi::CString;
use std::ptr::NonNull;
use std::{mem, ptr};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Console {
    this: *const usize,
}

impl Console {
    fn this(&self) -> *const usize {
        self as *const Self as *const usize
    }

    pub fn write(&self, buf: &[u8]) {
        unsafe {
            type Write = unsafe extern "C" fn(
                this: *const usize,
                format: *const i8,
                text: *const i8,
            ) -> bool;

            let method = vmt::get(self.this, 27);
            tracing::debug!("method {:?}", method);
            let write: Write = mem::transmute(method);
            let text = CString::new(buf).unwrap();

            write(self.this(), "%s\0".as_ptr().cast(), text.as_ptr());
        }
    }
}
