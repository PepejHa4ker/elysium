use daisy_chain::{Chain, ChainIter};
use elysium_dl::Library;
use std::marker::PhantomData;
use std::{fmt, ptr};

/// An interface.
#[repr(C)]
pub struct Interface<'a> {
    new: unsafe extern "C" fn() -> *mut (),
    name: *const u8,
    next: *mut Interface<'a>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Interface<'a> {
    #[inline]
    pub fn new(&self) -> *mut () {
        let new = self.new;

        unsafe { new() }
    }

    #[inline]
    pub fn name(&self) -> &str {
        unsafe { elysium_sdk::ffi::str_from_ptr_nullable(self.name) }
    }
}

impl<'a> fmt::Debug for Interface<'a> {
    #[inline]
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

#[inline]
fn is_exact(target: &str) -> bool {
    target.chars().rev().take(3).all(char::is_numeric)
}

impl<'a> Interfaces<'a> {
    #[inline]
    pub const unsafe fn from_ptr(head: *mut Interface<'a>) -> Self {
        let inner = Chain::from_ptr(head, next as Next<'a>);

        Self { inner }
    }

    #[inline]
    pub const fn iter(&'a self) -> InterfaceIter<'a> {
        let inner = self.inner.iter();

        InterfaceIter { inner }
    }

    #[inline]
    pub fn get(&'a self, target: &str) -> *mut () {
        let cmp = if is_exact(target) {
            |name: &str, target: &str| name == target
        } else {
            |name: &str, target: &str| {
                let name = unsafe { name.get_unchecked(0..name.len().saturating_sub(3)) };

                name == target
            }
        };

        for interface in self.iter() {
            let name = interface.name();

            if cmp(name, target) {
                return interface.new();
            }
        }

        ptr::null_mut()
    }
}

impl<'a> fmt::Debug for Interfaces<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, fmt)
    }
}

pub struct InterfaceIter<'a> {
    inner: ChainIter<'a, Interface<'a>, Next<'a>>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = &'a Interface<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[inline]
pub fn load_interfaces() -> elysium_sdk::Interfaces {
    unsafe {
        elysium_sdk::Interfaces::from_loader(|interface_kind| {
            let library_kind = interface_kind.library();
            let library = match Library::open(library_kind.as_nul_str()) {
                Some(library) => library,
                None => panic!("Failed to load library: {library_kind:?}"),
            };

            let symbol = match library.symbol("s_pInterfaceRegs\0") {
                Some(interfaces) => interfaces.as_ptr(),
                None => panic!("Failed to find interfaces within library: {library_kind:?}"),
            };

            let symbol = symbol.cast::<*mut Interface<'_>>();
            let interfaces = Interfaces::from_ptr(*symbol);
            let interface = interfaces.get(interface_kind.as_str());

            println!("elysium | loaded interface \x1b[38;5;2m{interface_kind:?}\x1b[m (\x1b[38;5;2m{:?}\x1b[m) within \x1b[38;5;2m{library_kind:?}\x1b[m (\x1b[38;5;2m{:?}\x1b[m) at \x1b[38;5;3m{interface:?}\x1b[m", interface_kind.as_str(), library_kind.as_str());

            interface
        })
    }
}
