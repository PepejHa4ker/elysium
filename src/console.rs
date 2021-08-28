use std::ffi::CString;
use std::mem;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Console {
    vtable: *const (),
}

impl Console {
    pub const fn as_ptr(&self) -> *const () {
        self as *const Self as _
    }

    pub const fn vtable(&self) -> *const *const *const () {
        self.as_ptr() as _
    }

    pub fn write(&self, buf: &[u8]) -> anyhow::Result<()> {
        type Write =
            unsafe extern "C" fn(this: *const (), format: *const i8, text: *const i8) -> bool;

        let write: Write = unsafe { mem::transmute(*(*self.vtable()).offset(27)) };
        let text = CString::new(buf).map_err(|_| anyhow::anyhow!("invalid string"))?;

        tracing::debug!("Console write: {:?}", &write);

        unsafe { write(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr()) };

        Ok(())
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
