#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(link_llvm_intrinsics)]
#![feature(const_mut_refs)]
#![feature(trait_alias)]

use crate::consts::offset;
use crate::interfaces::Interfaces;
use crate::libraries::Libraries;
use crate::log::Logger;
use std::time::Duration;
use std::{mem, thread};
use vptr::VirtualMut;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod animation_state;
pub mod client;
pub mod command;
pub mod console;
pub mod consts;
pub mod engine;
pub mod entities;
pub mod entity;
pub mod entity_id;
pub mod error;
pub mod frame;
pub mod globals;
pub mod hit_group;
pub mod hooks;
pub mod input;
pub mod interfaces;
pub mod intrinsics;
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod log;
pub mod move_kind;
pub mod netvars;
pub mod player_state;
pub mod skybox;
pub mod trace;
pub mod util;

pub unsafe fn change_ref<'a, 'b, T>(a: &'a T) -> &'b T {
    mem::transmute(a)
}

use crate::command::Command;
use crate::entity::Entity;
use crate::frame::Frame;
use core::ptr;
use std::ptr::NonNull;
use std::sync::Arc;

pub type OnFrame = Box<dyn Fn(Frame) -> Frame + 'static>;
pub type OnMove = Box<dyn Fn(Movement) -> Movement + 'static>;

struct GlobalRef {
    libraries: Libraries,
    interfaces: Interfaces,
    on_frame: OnFrame,
    on_move: OnMove,
    create_move_original: Box<hooks::create_move::Signature>,
    frame_stage_notify_original: Box<hooks::frame_stage_notify::Signature>,
}

#[derive(Clone)]
pub struct Global(pub(crate) Arc<GlobalRef>);

unsafe impl Send for GlobalRef {}
unsafe impl Sync for GlobalRef {}

fn default_on_frame(frame: Frame) -> Frame {
    frame
}

fn default_on_move(movement: Movement) -> Movement {
    movement
}

use std::lazy::SyncOnceCell;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell::new();

impl Global {
    pub fn new() -> Result<Self> {
        tracing::info!("Initialising interfaces...");

        let libraries = Libraries::new()?;
        let interfaces = Interfaces::new(&libraries);

        tracing::info!("{:?}", &interfaces);

        let this = Self(Arc::new(GlobalRef {
            libraries,
            interfaces,
            on_move: Box::new(move |frame| frame),
            on_frame: Box::new(move |movement| movement),
            create_move_original: Box::new(hooks::create_move::hook),
            frame_stage_notify_original: Box::new(hooks::frame_stage_notify::hook),
        }));

        tracing::info!("created global");

        GLOBAL.set(this.clone());

        tracing::info!("set global");

        unsafe {
            ptr::write(
                this.create_move_original_ptr(),
                this.interfaces().client_mode.vreplace_protected(
                    hooks::create_move::hook, // as *const hooks::create_move::Signature as *mut (),
                    offset::CREATE_MOVE * 8,
                ),
            );

            tracing::info!("hooked create_move");

            ptr::write(
                this.frame_stage_notify_original_ptr(),
                this.interfaces().client.as_mut_ptr().vreplace_protected(
                    hooks::frame_stage_notify::hook, // as *const hooks::frame_stage_notify::Signature
                    //as *mut (),
                    offset::FRAME_STAGE_NOTIFY * 8,
                ),
            );

            tracing::info!("hooked frame_stage_notify");
        }

        let client =
            unsafe { crate::client::Client::from_raw(this.interfaces().client.as_mut_ptr()) };

        netvars::set(&client);

        Ok(this)
    }

    pub fn libraries(&self) -> &Libraries {
        &self.0.libraries
    }

    pub fn interfaces(&self) -> &Interfaces {
        &self.0.interfaces
    }

    pub(crate) fn on_frame_ptr(&self) -> *mut OnFrame {
        &self.0.on_frame as *const OnFrame as *mut OnFrame
    }

    pub(crate) fn on_move_ptr(&self) -> *mut OnMove {
        &self.0.on_move as *const OnMove as *mut OnMove
    }

    pub(crate) fn create_move_original_ptr(&self) -> *mut hooks::create_move::Signature {
        &*self.0.create_move_original as *const hooks::create_move::Signature
            as *mut hooks::create_move::Signature
    }

    pub(crate) fn create_move_original(
        &self,
        this: *const (),
        input_sample_time: f32,
        command: &mut Command,
    ) -> bool {
        let original = unsafe { *self.create_move_original_ptr() };

        unsafe { original(this, input_sample_time, command) }
    }

    pub(crate) fn frame_stage_notify_original_ptr(
        &self,
    ) -> *mut hooks::frame_stage_notify::Signature {
        &*self.0.frame_stage_notify_original as *const hooks::frame_stage_notify::Signature
            as *mut hooks::frame_stage_notify::Signature
    }

    pub(crate) fn frame_stage_notify_original(&self, this: *const (), frame: Frame) {
        let original = unsafe { *self.frame_stage_notify_original_ptr() };

        unsafe { original(this, frame) }
    }

    /// set frame stage notify hook
    pub fn on_frame<F>(&self, f: F)
    where
        F: Fn(Frame) -> Frame + 'static,
    {
        unsafe {
            ptr::write(self.on_frame_ptr(), Box::new(f));
        }
    }

    /// set create move hook
    pub fn on_move<F>(&self, f: F)
    where
        F: Fn(Movement) -> Movement + 'static,
    {
        unsafe {
            ptr::write(self.on_move_ptr(), Box::new(f));
        }
    }
}

pub struct Movement {
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub view_angle: sdk::Angle,
    pub tick_count: i32,
    pub send_packet: bool,
    pub local_player: crate::entity::Entity,
}

fn main(logger: Logger) -> Result<()> {
    if library::Library::serverbrowser().is_err() {
        tracing::info!("Waiting for CS:GO to load...");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    let global = Global::new()?;

    tracing::info!("setting user hooks");

    global.on_frame(move |frame| frame);

    tracing::info!("on_frame");

    global.on_move(move |movement| movement);

    tracing::info!("on_move");

    mem::forget(global);

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
