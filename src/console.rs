use core::ptr::NonNull;
use std::borrow::Cow;
use std::ffi::CStr;

pub use self::var::{Kind, RawVar, Var};

mod var;

extern "C" {
    /// Raw handle to the console.
    pub type RawConsole;
}

unsafe impl Send for RawConsole {}
unsafe impl Sync for RawConsole {}

/// The console.
#[derive(Debug)]
#[repr(transparent)]
pub struct Console(NonNull<RawConsole>);

impl Console {
    pub const fn from_raw(raw: *mut RawConsole) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawConsole) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawConsole {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn var<'a, T, V>(&self, var: V) -> Option<Var<T>>
    where
        T: Kind,
        V: Into<Cow<'a, CStr>>,
    {
        type GetVar = unsafe extern "C" fn(this: *const RawConsole, var: *const i8) -> *mut RawVar;

        unsafe {
            let raw_var = virt::get::<GetVar>(self.virtual_table(), 15 * 8)(
                self.as_ptr(),
                var.into().as_ptr(),
            );

            Var::from_raw(raw_var)
        }
    }

    pub fn write<'a, S>(&self, string: S)
    where
        S: Into<Cow<'a, CStr>>,
    {
        type Write = unsafe extern "C" fn(this: *const RawConsole, fmt: *const i8, txt: *const i8);

        unsafe {
            virt::get::<Write>(self.virtual_table(), 27 * 8)(
                self.as_ptr(),
                b"%s\0".as_ptr().cast(),
                string.into().as_ptr(),
            );
        }
    }
}
