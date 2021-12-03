use std::ffi::CString;
use vptr::Virtual;

#[derive(Clone, Debug)]
pub struct Console {
    this: *const (),
}

impl Console {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
    }

    pub fn write(&self, buf: impl Into<Vec<u8>>) {
        type Signature =
            unsafe extern "C" fn(this: *const (), format: *const i8, text: *const i8) -> bool;

        let text = CString::new(buf).unwrap();
        let method: Signature = unsafe { self.as_ptr().vget(27 * 8) };

        unsafe {
            method(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr());
        }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
