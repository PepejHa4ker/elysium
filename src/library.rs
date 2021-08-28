use crate::symbol;
use anyhow::Result;
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use std::ffi::{CStr, CString, NulError, OsStr};
use std::fs::OpenOptions;
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

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

/// Entry within the linked list of interfaces
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Entry<'a> {
    borrow: extern "C" fn() -> Option<NonNull<()>>,
    name: Option<NonNull<libc::c_char>>,
    next: Option<NonNull<Entry<'a>>>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Entry<'a> {
    pub fn borrow<T>(&'a self) -> Option<NonNull<T>> {
        let borrow = self.borrow;

        borrow().map(|ptr| ptr.cast::<T>())
    }

    pub fn raw_name(&'a self) -> Option<&'a CStr> {
        self.name
            .map(|name| unsafe { CStr::from_ptr(name.as_ptr()) })
    }

    pub fn name(&'a self) -> Option<&'a str> {
        self.raw_name().and_then(|name| name.to_str().ok())
    }

    pub fn next(&'a self) -> Option<Entry<'a>> {
        self.next.map(|entry| unsafe { *entry.as_ptr() })
    }
}

/// Linked list of interfaces
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Interfaces<'a> {
    entry: Entry<'a>,
}

impl<'a> Interfaces<'a> {
    pub fn iter(&self) -> InterfaceIter<'a> {
        InterfaceIter {
            entry: Some(self.entry),
        }
    }

    pub fn get<T>(&self, name: &str) -> Option<NonNull<T>> {
        for interface in self.iter() {
            if let Some(interface_name) = interface.name() {
                tracing::debug!("{} -> {:?}", interface_name, interface);

                if interface_name.starts_with(name) {
                    return interface.borrow();
                }
            }
        }

        None
    }
}

pub struct InterfaceIter<'a> {
    entry: Option<Entry<'a>>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.entry?;

        self.entry = unsafe { std::mem::transmute(entry.next()) };

        Some(unsafe { std::mem::transmute(entry) })
    }
}

/// Library wrapper for opening source interfaces easier
pub struct Library<'a> {
    library: unix::Library,
    _phantom: PhantomData<&'a ()>,
}

type Pointer<T> = Option<NonNull<T>>;

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

    fn get<T>(&self, symbol: &str) -> Pointer<T> {
        let pointer = unsafe { self.library.get::<T>(symbol.as_bytes()).ok()?.into_raw() };

        NonNull::new(pointer.cast::<T>())
    }

    pub fn interfaces(&'a self) -> Option<Interfaces<'a>> {
        tracing::debug!("Loading symbol {:?}...", symbol::INTERFACES);

        let symbol = self.get::<Pointer<Interfaces<'a>>>(symbol::INTERFACES)?;
        let pointer = unsafe { *symbol.as_ptr() };
        let interfaces = unsafe { *pointer?.as_ptr() };

        tracing::debug!("Loaded symbol {:?}: {:?}", symbol::INTERFACES, interfaces);

        Some(interfaces)
    }
}
