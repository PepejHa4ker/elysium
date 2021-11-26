#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]
#![feature(const_fn_floating_point_arithmetic)]

use crate::consts::offset;
use crate::interfaces::Interfaces;
use crate::libraries::Libraries;
use crate::log::Logger;
use libc::{RTLD_LOCAL, RTLD_NOLOAD, RTLD_NOW};
use libloading::os::unix;
use parking_lot::RwLock;
use std::ffi::{CString, NulError, OsStr};
use std::lazy::SyncOnceCell;
use std::ptr::NonNull;
use std::time::Duration;
use std::{mem, ptr, thread};
use vptr::{Pointer, Virtual, VirtualMut};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod angle;
pub mod consts;
pub mod error;
pub mod globals;
pub mod hooks;
pub mod interfaces;
pub mod libraries;
pub mod library;
pub mod log;
pub mod sdk;

pub unsafe fn change_ref<'a, 'b, T>(a: &'a T) -> &'b T {
    mem::transmute(a)
}

fn main(logger: Logger) -> Result<()> {
    if library::Library::serverbrowser().is_err() {
        tracing::info!("Waiting for CS:GO to load...");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    tracing::info!("Initialising interfaces...");

    let libraries = Libraries::new()?;
    let interfaces = Interfaces::new(&libraries);

    tracing::info!("{:?}", &interfaces);

    globals::set_console(interfaces.console);

    unsafe { globals::console() }.write("console\n");

    globals::set_engine(interfaces.engine);
    globals::set_entities(interfaces.entities);

    hooks::create_move::set_original(unsafe {
        interfaces
            .client_mode
            .vreplace_protected(hooks::create_move::hook as *mut (), offset::CREATE_MOVE * 8)
    });

    hooks::frame_stage_notify::set_original(unsafe {
        interfaces.client.vreplace_protected(
            hooks::frame_stage_notify::hook as *mut (),
            offset::FRAME_STAGE_NOTIFY * 8,
        )
    });

    let client = unsafe { sdk::Client::from_raw(interfaces.client) };

    sdk::netvars::set(&client);

    /*unsafe {
        return Ok(());

        use crate::consts::library::sdl;
        use vptr::{Pointer, Virtual, VirtualMut};

        let sdl = library::Library::sdl()?;

        let swap_window_symbol = sdl.get::<()>(sdl::SWAPWINDOW) as *mut *const ();
        let swap_window_address = swap_window_symbol.add_bytes(2).to_absolute();
        let swap_window = *swap_window_address;

        let poll_event_symbol = sdl.get::<()>(sdl::POLLEVENT) as *mut *const ();
        let poll_event_address = poll_event_symbol.add_bytes(2).to_absolute();
        let poll_event = *poll_event_address;

        tracing::info!("swap_window = {:0x?}", swap_window);
        tracing::info!("swap_window_address = {:0x?}", swap_window_address);
        tracing::info!("poll_event = {:0x?}", poll_event);
        tracing::info!("poll_event_address = {:0x?}", poll_event_address);

        hooks::sdl::swap_window::set_original(swap_window);
        hooks::sdl::poll_event::set_original(poll_event);

        swap_window_address.replace(hooks::sdl::swap_window::hook as *const ());
        poll_event_address.replace(hooks::sdl::poll_event::hook as *const ());
    }*/

    Ok(())
}

#[ctor::ctor]
fn providence_init() {
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
