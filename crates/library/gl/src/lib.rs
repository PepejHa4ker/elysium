//! Convenience wrapper around `elysium_dl::Library` for GL methods.
#![feature(core_ffi_c)]

use elysium_dl::Library;

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::os::raw;
use std::os::unix::ffi::OsStrExt;
use std::{fmt, mem, ptr};

// features
pub use consts::{
    BLEND, CLAMP_TO_EDGE, COLOR_BUFFER_BIT, DYNAMIC_DRAW, FRAMEBUFFER_SRGB, LINEAR, MULTISAMPLE,
    ONE, ONE_MINUS_SRC_ALPHA, SCISSOR_TEST, SRC_ALPHA, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER,
    TEXTURE_SWIZZLE_RGBA, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNPACK_ALIGNMENT,
};

// colors
pub use consts::{R8, RED};

// limits
pub use consts::MAX_TEXTURE_SIZE;

// types
pub use consts::{
    ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, TEXTURE0, TEXTURE_2D, UNSIGNED_BYTE, UNSIGNED_INT,
};

// shapes
pub use consts::{TRIANGLES, TRIANGLE_STRIP};

// shaders
pub use consts::{FRAGMENT_SHADER, VERTEX_SHADER};

// values
pub use consts::ZERO;

// compilation
// private since we don't expose API which accepts these
use consts::{COMPILE_STATUS, INFO_LOG_LENGTH, LINK_STATUS};

pub use context::Context;

mod consts;
mod context;

pub(crate) mod macros;

/// The GL library itself.
#[repr(C)]
pub struct Gl {
    library: Library,
}

impl Gl {
    /// Load GL, specifically `libGL.so.1`.
    #[inline]
    pub fn open() -> Option<Self> {
        let library = Library::open_global("libGL.so.1\0")?;

        Some(Self { library })
    }

    #[inline]
    pub unsafe fn get_proc_address<S>(&self, symbol: S) -> *mut ()
    where
        S: AsRef<OsStr>,
    {
        println!(
            "elysium-gl | looking for symbol \x1b[38;5;2m{:?}\x1b[m",
            symbol.as_ref()
        );

        let address = self.glx_get_proc_address(&symbol);

        if !address.is_null() {
            return address;
        }

        println!("elysium-gl | \x1b[38;5;1mglXGetProcAddress returned null\x1b[m");

        match self.library.symbol(symbol.as_ref()) {
            Some(address) => address.as_ptr() as _,
            None => {
                println!("elysium-gl | \x1b[38;5;1mdlsym returned null, aborting\x1b[m");

                panic!(
                    "elysium-gl | unable to find requested symbol {:?}",
                    symbol.as_ref()
                );
            }
        }
    }

    #[inline]
    pub unsafe fn glx_get_proc_address<S>(&self, symbol: S) -> *mut ()
    where
        S: AsRef<OsStr>,
    {
        type Fn = unsafe extern "C" fn(symbol: *const u8) -> *mut ();

        let get_proc_address: Fn = match self.library.symbol("glXGetProcAddress\0") {
            Some(get_proc_address) => mem::transmute(get_proc_address),
            None => return ptr::null_mut(),
        };

        let symbol = match cstr_cow_from_bytes(symbol.as_ref().as_bytes()) {
            Some(symbol) => symbol,
            None => return ptr::null_mut(),
        };

        let symbol = symbol.as_ref().to_bytes_with_nul();

        get_proc_address(symbol.as_ptr())
    }
}

impl fmt::Debug for Gl {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.library, fmt)
    }
}

pub enum PixelPackData<'a> {
    BufferOffset(u32),
    Slice(&'a mut [u8]),
}

pub enum PixelUnpackData<'a> {
    BufferOffset(u32),
    Slice(&'a [u8]),
}

pub enum CompressedPixelUnpackData<'a> {
    BufferRange(core::ops::Range<u32>),
    Slice(&'a [u8]),
}

/// Checks for the last byte and avoids allocating if it is zero.
///
/// Non-last null bytes still result in an error.
pub(crate) fn cstr_cow_from_bytes(slice: &[u8]) -> Option<Cow<'_, CStr>> {
    static ZERO: raw::c_char = 0;

    Some(match slice.last() {
        // Slice out of 0 elements
        None => unsafe { Cow::Borrowed(CStr::from_ptr(&ZERO)) },
        // Slice with trailing 0
        Some(&0) => Cow::Borrowed(CStr::from_bytes_with_nul(slice).ok()?),
        // Slice with no trailing 0
        Some(_) => Cow::Owned(CString::new(slice).ok()?),
    })
}
