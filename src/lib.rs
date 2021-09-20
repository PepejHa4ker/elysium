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
use std::ptr::NonNull;
use std::{mem, ptr};

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
        .ok_or_else(|| String::from("no interfaces"))?;

    let console = Console::from_ptr(interfaces.get::<usize>(interface::VENGINECVAR));

    console.write("fuck niggers\n");

    logger.set_console(console);

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
                .with_ansi(false)
                .with_level(false)
                .with_max_level(tracing::Level::TRACE)
                .with_writer(non_blocking)
                .without_time();

            tracing::subscriber::with_default(subscriber.finish(), || {
                tracing::info!("And... we're in!");
                tracing::info!("Main returned: {:?}", main(logger));
            });
        })
        .unwrap();
}
