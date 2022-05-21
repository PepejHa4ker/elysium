//! Convienience wrappers around `libloading::os::unix` structures.

use libloading::os::unix;
use std::ffi::OsStr;
use std::fmt;
use std::marker::PhantomData;
use std::os::unix::ffi::OsStrExt;

const FLAGS: libc::c_int = libc::RTLD_NOLOAD /* dont load the library if it isnt already resident */
    | unix::RTLD_LAZY;

const FLAGS_GLOBAL: libc::c_int = libc::RTLD_NOW | unix::RTLD_GLOBAL;

/// Convenience wrapper for `libloading::os::unix::Library`.
pub struct Library<'a> {
    library: unix::Library,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Library<'a> {
    /// Copy the inner library handle.
    #[inline]
    fn copied(&self) -> unix::Library {
        // SAFETY: `unix::Library` is just a handle, aka `*const c_void`
        unsafe {
            let library: *const unix::Library = &self.library;

            library.read()
        }
    }

    /// Obtain an address to this library.
    #[inline]
    pub fn as_ptr(&self) -> *const () {
        self.copied().into_raw().cast()
    }

    /// Open the library, `library`.
    #[inline]
    pub fn open<L>(library: L) -> Option<Self>
    where
        L: AsRef<OsStr>,
    {
        // SAFETY: a null terminator is appended if it isnt present.
        let library = unsafe { unix::Library::open(Some(library), FLAGS).ok()? };
        let _phantom = PhantomData;

        Some(Self { library, _phantom })
    }

    /// Open the library, `library`.
    #[inline]
    pub fn open_global<L>(library: L) -> Option<Self>
    where
        L: AsRef<OsStr>,
    {
        // SAFETY: a null terminator is appended if it isnt present.
        let library = unsafe { unix::Library::open(Some(library), FLAGS_GLOBAL).ok()? };
        let _phantom = PhantomData;

        Some(Self { library, _phantom })
    }

    /// Checks if the library, `library`, is resident.
    #[inline]
    pub fn exists<L>(library: L) -> bool
    where
        L: AsRef<OsStr>,
    {
        Self::open(library).is_some()
    }

    /// Load the current executable.
    #[inline]
    pub fn this() -> Self {
        let library = unix::Library::this();
        let _phantom = PhantomData;

        Self { library, _phantom }
    }

    /// Load the symbol, `symbol`.
    #[inline]
    pub fn symbol<S>(&self, symbol: S) -> Option<Symbol<'a>>
    where
        S: AsRef<OsStr>,
    {
        let symbol = symbol.as_ref().as_bytes();

        // SAFETY: a null terminator is appended if it isnt present.
        if let Ok(address) = unsafe { self.library.get::<usize>(symbol) } {
            let address = address.into_raw();

            if address.is_null() {
                None
            } else {
                Some(Symbol {
                    address: address.cast(),
                    _phantom: PhantomData,
                })
            }
        } else {
            None
        }
    }
}

impl<'a> fmt::Debug for Library<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ptr(), fmt)
    }
}

/// A symbol with a lifetime.
pub struct Symbol<'a> {
    address: *const (),
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Symbol<'a> {
    /// Returns the symbols address.
    #[inline]
    pub fn as_ptr(&self) -> *const () {
        self.address
    }
}
