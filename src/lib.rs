use crate::library::{Entry, Library};
use crate::log::Logger;
use anyhow::Result;
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use std::ffi::{CString, NulError, OsStr};
use std::ptr;
use std::ptr::NonNull;

pub mod interface;
pub mod library;
pub mod log;
pub mod symbol;

fn main() -> Result<()> {
    tracing::info!("Initialising interfaces...");

    let client = Library::new(library::CLIENT)?;
    let engine = Library::new(library::ENGINE)?;
    let materialsystem = Library::new(library::MATERIALSYSTEM)?;
    let vguimatsurface = Library::new(library::VGUIMATSURFACE)?;
    let vgui2 = Library::new(library::VGUI2)?;
    let inputsystem = Library::new(library::INPUTSYSTEM)?;
    let vphysics = Library::new(library::VPHYSICS)?;
    let localize = Library::new(library::LOCALIZE)?;
    let tier0 = Library::new(library::TIER0)?;
    let panorama = Library::new(library::PANORAMA)?;
    let fs_stdio = Library::new(library::FS_STDIO)?;
    let matchmaking = Library::new(library::MATCHMAKING)?;

    let interfaces = materialsystem
        .interfaces()
        .ok_or_else(|| anyhow::anyhow!("no interfaces"))?;

    #[derive(Debug)]
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

        pub fn write(&self, text: &str) -> Result<()> {
            type Write =
                unsafe extern "C" fn(this: *const (), format: *const i8, text: *const i8) -> bool;

            let write: Write = unsafe { std::mem::transmute(*(*self.vtable()).offset(27)) };
            let text = CString::new(text).map_err(|_| anyhow::anyhow!("invalid string"))?;

            tracing::debug!("Console write: {:?}", &write);

            unsafe { write(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr()) };

            Ok(())
        }
    }

    let result: Option<NonNull<Console>> = interfaces.get(interface::VENGINECVAR);
    let console = unsafe {
        result
            .ok_or_else(|| anyhow::anyhow!("no interface"))?
            .as_ref()
    };

    tracing::debug!("Console: {:?}", &console);

    console.write("hello world\n");

    Ok(())
}

#[ctor::ctor]
fn butterscotch_init() {
    use std::thread;

    thread::Builder::new()
        .name(env!("CARGO_PKG_NAME").to_string())
        .spawn(move || {
            let logger = Logger::new();
            let (non_blocking, _guard) = tracing_appender::non_blocking(logger);
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter("trace")
                .with_writer(non_blocking);

            tracing::subscriber::with_default(subscriber.finish(), || {
                tracing::info!("And... we're in!");
                tracing::info!("Main returned: {:?}", main());
            });
        })
        .unwrap();
}
