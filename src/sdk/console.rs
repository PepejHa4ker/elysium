use std::ffi::CString;
use std::mem;

#[derive(Clone)]
pub struct Console {
    this: *const usize,
}

impl Console {
    pub unsafe fn from_raw(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub fn write(&self, buf: impl Into<Vec<u8>>) {
        tracing::info!("console = {:?}", self.this);

        type Signature =
            unsafe extern "C" fn(this: *const usize, format: *const i8, text: *const i8) -> bool;

        let text = CString::new(buf).unwrap();
        let method: Signature = unsafe { mem::transmute(vmt::get(self.as_ptr(), 27)) };

        unsafe {
            method(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr());
        }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
