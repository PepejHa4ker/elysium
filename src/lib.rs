#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]
#![feature(option_result_unwrap_unchecked)]

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
use std::{mem, ptr};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

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
    tracing::info!("Initialising interfaces...");

    let libraries = Libraries::new()?;
    let interfaces = Interfaces::new(&libraries);

    tracing::info!("{:?}", &interfaces);

    globals::set_console(interfaces.console as *const usize);

    unsafe { globals::console() }.write("console\n");

    globals::set_engine(interfaces.engine);
    globals::set_entities(interfaces.entities);

    hooks::create_move::set_original(unsafe {
        vmt::hook(
            interfaces.client_mode,
            hooks::create_move::hook as *const usize,
            offset::CREATE_MOVE,
        )
    });

    hooks::frame_stage_notify::set_original(unsafe {
        vmt::hook(
            interfaces.client,
            hooks::frame_stage_notify::hook as *const usize,
            offset::FRAME_STAGE_NOTIFY,
        )
    });

    let client = unsafe { sdk::Client::from_raw(interfaces.client) };

    sdk::netvars::set(&client);

    let props = sdk::netvars::get_props("DT_BasePlayer");

    for (name, prop) in props.iter().flat_map(|map| map.iter()) {
        tracing::info!("{} -> {:0x?}", name, prop.offset);
    }

    //let offset = sdk::netvars::offset_of("DT_CSPlayer", "m_fFlags");

    //tracing::info!("{}.{} = {:?}", "DT_CSPlayer", "m_fFlags", &offset);

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
