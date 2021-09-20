use std::ffi::CString;
use std::mem;

#[derive(Clone)]
pub struct Console {
    this: *const usize,
}

impl Console {
    pub fn from_ptr(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    /// Returns a pointer to the underlying interface.
    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub fn write(&self, buf: impl Into<Vec<u8>>) {
        type Write =
            unsafe extern "C" fn(this: *const usize, format: *const i8, text: *const i8) -> bool;

        let method = unsafe { vmt::get(self.as_ptr(), 27) };
        let write: Write = unsafe { mem::transmute(method) };
        let text = CString::new(buf).unwrap();

        unsafe {
            write(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr());
        }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
