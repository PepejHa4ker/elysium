#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]

use crate::console::Console;
use crate::library::Library;
use crate::log::Logger;
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use std::ffi::{CString, NulError, OsStr};
use std::ptr;
use std::ptr::NonNull;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod console;
pub mod error;
pub mod hooks;
pub mod interface;
pub mod library;
pub mod log;
pub mod sdk;
pub mod symbol;

fn main(logger: Logger) -> Result<()> {
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
        .ok_or(String::from("no interfaces"))?;

    let result = interfaces.get::<*const Console>(interface::VENGINECVAR);
    let console = unsafe { **result };

    tracing::debug!("console {:?}", &console);

    console.write("nigger".as_bytes());

    Ok(())
}

#[ctor::ctor]
fn providence_init() {
    use std::thread;

    thread::Builder::new()
        .name(env!("CARGO_PKG_NAME").to_string())
        .spawn(move || {
            let logger = Logger::new();
            let (non_blocking, _guard) = tracing_appender::non_blocking(logger.clone());
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter("trace")
                .with_writer(non_blocking);

            tracing::subscriber::with_default(subscriber.finish(), || {
                tracing::info!("And... we're in!");
                tracing::info!("Main returned: {:?}", main(logger));
            });
        })
        .unwrap();
}
