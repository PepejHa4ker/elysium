use crate::symbol;
use crate::Result;
use daisy_chain::{Chain, ChainIter};
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use std::ffi::{CStr, CString, NulError, OsStr};
use std::fs::OpenOptions;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, ptr};

pub const CLIENT: &str = "./csgo/bin/linux64/client_client.so\0";
pub const ENGINE: &str = "./bin/linux64/engine_client.so\0";
pub const FS_STDIO: &str = "./bin/linux64/filesystem_stdio_client.so\0";
pub const INPUTSYSTEM: &str = "./bin/linux64/inputsystem_client.so\0";
pub const TIER0: &str = "./bin/linux64/libtier0_client.so\0";
pub const LOCALIZE: &str = "./bin/linux64/localize_client.so\0";
pub const MATCHMAKING: &str = "./csgo/bin/linux64/matchmaking_client.so\0";
pub const MATERIALSYSTEM: &str = "./bin/linux64/materialsystem_client.so\0";
pub const PANORAMA: &str = "./bin/linux64/panorama_client.so\0";
pub const VGUIMATSURFACE: &str = "./bin/linux64/vguimatsurface_client.so\0";
pub const VGUI2: &str = "./bin/linux64/vgui2_client.so\0";
pub const VPHYSICS: &str = "./bin/linux64/vphysics_client.so\0";

/// An interface.
#[repr(C)]
pub struct Interface<'a> {
    new: unsafe extern "C" fn() -> *mut usize,
    name: Option<&'a spirit::Str>,
    next: *mut Interface<'a>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Interface<'a> {
    pub fn new<T>(&self) -> *const T {
        let new = self.new;

        unsafe { new() as *const T }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.map(spirit::Str::as_str)
    }
}

impl<'a> fmt::Debug for Interface<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Interface")
            .field("new", &self.new)
            .field("name", &self.name)
            .finish()
    }
}

type Next<'a> = fn(&Interface<'a>) -> *mut Interface<'a>;

fn next<'a>(interface: &Interface<'a>) -> *mut Interface<'a> {
    interface.next
}

/// Linked list of interfaces.
#[derive(Debug)]
pub struct Interfaces<'a> {
    inner: Chain<Interface<'a>, Next<'a>>,
}

impl<'a> Interfaces<'a> {
    pub const unsafe fn from_ptr(head: *mut Interface<'a>) -> Self {
        let inner = Chain::from_ptr(head, next as Next<'a>);

        Self { inner }
    }

    pub const fn iter(&'a self) -> InterfaceIter<'a> {
        let inner = self.inner.iter();

        InterfaceIter { inner }
    }

    pub fn get<T>(&'a self, name: &str) -> *const T {
        for interface in self.iter() {
            if let Some(interface_name) = interface.name() {
                if interface_name.starts_with(name) {
                    tracing::debug!("{:?} matches {:?}", &interface, &name);

                    return interface.new();
                }
            }
        }

        ptr::null()
    }
}

pub struct InterfaceIter<'a> {
    inner: ChainIter<'a, Interface<'a>, Next<'a>>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = &'a Interface<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Library wrapper for opening source interfaces easier
pub struct Library<'a> {
    library: unix::Library,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Library<'a> {
    /// Open a library by `name`
    ///
    /// Flags used during dlopen: RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL
    pub fn new(library: impl AsRef<OsStr> + 'a) -> Result<Library<'a>, libloading::Error> {
        let library_name = library.as_ref();

        tracing::debug!("Opening library {:?}...", &library_name);

        let library = unsafe {
            unix::Library::open(Some(&library_name), RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL)?
        };

        tracing::debug!(
            "Opening of library {:?}, succeeded! ({:?})",
            &library_name,
            &library
        );

        Ok(Library {
            library,
            _phantom: PhantomData,
        })
    }

    unsafe fn get<T>(&self, symbol: &str) -> *const T {
        if let Ok(symbol) = self.library.get::<T>(symbol.as_bytes()) {
            symbol.into_raw() as *const T
        } else {
            ptr::null()
        }
    }

    pub fn interfaces(&'a self) -> Option<Interfaces<'a>> {
        let symbol = unsafe { self.get::<*mut Interface<'a>>(symbol::INTERFACES) };

        if symbol.is_null() {
            return None;
        }

        let interfaces = unsafe { Interfaces::from_ptr(*symbol) };

        tracing::debug!("{:?}", &interfaces);

        Some(interfaces)
    }
}
