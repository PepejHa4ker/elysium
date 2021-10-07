use crate::consts::library;
use crate::Result;
use daisy_chain::{Chain, ChainIter};
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use std::ffi::{CStr, CString, NulError, OsStr};
use std::fs::OpenOptions;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{fmt, ptr};

/// An interface.
#[repr(C)]
pub struct Interface<'a> {
    new: unsafe extern "C" fn() -> *mut (),
    name: Option<&'a spirit::Str>,
    next: *mut Interface<'a>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Interface<'a> {
    pub fn new(&self) -> *mut () {
        let new = self.new;

        unsafe { new() }
    }

    pub fn name(&self) -> &str {
        self.name.map(spirit::Str::as_str).unwrap_or("")
    }
}

impl<'a> fmt::Debug for Interface<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Interface")
            .field("new", &self.new)
            .field("name", &self.name())
            .finish()
    }
}

type Next<'a> = fn(&Interface<'a>) -> *mut Interface<'a>;

fn next<'a>(interface: &Interface<'a>) -> *mut Interface<'a> {
    interface.next
}

/// Linked list of interfaces.
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

    pub fn get(&'a self, name: &str) -> *mut () {
        for interface in self.iter() {
            let interface_name = interface.name();

            // SAFETY: end index is always valid.
            if unsafe { interface_name.get_unchecked(..interface_name.len().saturating_sub(3)) }
                == name
            {
                let new = interface.new();

                tracing::info!("{} ({}) -> {:?}", &name, &interface_name, new);

                return new;
            }
        }

        ptr::null_mut()
    }

    pub fn get_exact(&'a self, name: &str) -> *mut () {
        for interface in self.iter() {
            let interface_name = interface.name();

            if interface_name == name {
                let new = interface.new();

                tracing::info!("{} ({}) -> {:?}", &name, interface_name, new);

                return new;
            }
        }

        ptr::null_mut()
    }
}

impl<'a> fmt::Debug for Interfaces<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_fmt(format_args!("{:?}", self.inner))
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
pub struct Library {
    lib: unix::Library,
}

impl Library {
    /// Open a library by `name`
    ///
    /// Flags used during dlopen: RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL
    pub fn new(name: &str) -> Result<Self, libloading::Error> {
        let ptr = unsafe {
            unix::Library::open(Some(&name), RTLD_NOLOAD | RTLD_NOW | RTLD_LOCAL)?.into_raw()
        };

        tracing::debug!("{} -> {:?}", &name, ptr);

        let lib = unsafe { unix::Library::from_raw(ptr) };

        Ok(Library { lib })
    }

    pub fn client() -> Result<Self, libloading::Error> {
        Self::new(library::CLIENT)
    }

    pub fn engine() -> Result<Self, libloading::Error> {
        Self::new(library::ENGINE)
    }

    pub fn fs_stdio() -> Result<Self, libloading::Error> {
        Self::new(library::FS_STDIO)
    }

    pub fn inputsystem() -> Result<Self, libloading::Error> {
        Self::new(library::INPUTSYSTEM)
    }

    pub fn localize() -> Result<Self, libloading::Error> {
        Self::new(library::LOCALIZE)
    }

    pub fn matchmaking() -> Result<Self, libloading::Error> {
        Self::new(library::MATCHMAKING)
    }

    pub fn materialsystem() -> Result<Self, libloading::Error> {
        Self::new(library::MATERIALSYSTEM)
    }

    pub fn panorama() -> Result<Self, libloading::Error> {
        Self::new(library::PANORAMA)
    }

    pub fn sdl() -> Result<Self, libloading::Error> {
        Self::new(library::SDL)
    }

    pub fn serverbrowser() -> Result<Self, libloading::Error> {
        Self::new(library::SERVERBROWSER)
    }

    pub fn vguimatsurface() -> Result<Self, libloading::Error> {
        Self::new(library::VGUIMATSURFACE)
    }

    pub fn vgui2() -> Result<Self, libloading::Error> {
        Self::new(library::VGUI2)
    }

    pub fn vphysics() -> Result<Self, libloading::Error> {
        Self::new(library::VPHYSICS)
    }

    pub unsafe fn get<T>(&self, symbol: &[u8]) -> *const T {
        if let Ok(symbol) = self.lib.get::<T>(symbol) {
            symbol.into_raw() as *const T
        } else {
            ptr::null()
        }
    }

    pub fn interfaces(&self) -> Option<Interfaces<'_>> {
        let symbol = unsafe { self.get::<*mut Interface<'_>>(library::INTERFACES.as_bytes()) };

        if symbol.is_null() {
            return None;
        }

        let interfaces = unsafe { Interfaces::from_ptr(*symbol) };

        Some(interfaces)
    }

    pub fn get_interface(&self, interface: &str) -> *mut () {
        if let Some(interfaces) = self.interfaces() {
            interfaces.get(interface)
        } else {
            ptr::null_mut()
        }
    }

    pub fn get_exact_interface(&self, interface: &str) -> *mut () {
        if let Some(interfaces) = self.interfaces() {
            interfaces.get_exact(interface)
        } else {
            ptr::null_mut()
        }
    }
}
