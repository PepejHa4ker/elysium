use crate::Result;
use std::ffi::CString;
use std::ptr::NonNull;
use std::{mem, ptr};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Console {
    pub vtable_ref: *const *const unsafe extern "C" fn(this: *const ()),
    pub unknown: [u8; 256],
}

impl Console {
    pub fn write(&self, buf: &[u8]) -> Result<()> {
        unsafe {
            type Write =
                unsafe extern "C" fn(this: *const (), format: *const i8, text: *const i8) -> bool;

            let vtable = if self.vtable_ref.is_null() {
                tracing::debug!("console vtable null");

                return Err("vtable null".into());
            } else {
                *self.vtable_ref
            };

            tracing::debug!("console vtable at {:?}", vtable);

            let method = if vtable.add(27).is_null() {
                tracing::debug!("console vtable + 27 null");

                return Err("vtable + 27 null".into());
            } else {
                *vtable.add(27)
            };

            tracing::debug!("console vtable method at {:?}", method);

            let write: Write = mem::transmute(method);
            let text = CString::new(buf).map_err(|_| "invalid string")?;

            tracing::debug!("console write {:?} {:?}", &write, &text);

            write(
                self as *const Self as *const (),
                "%s\0".as_ptr().cast(),
                text.as_ptr(),
            );

            Ok(())
        }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
