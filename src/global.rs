use crate::command::Command;
use crate::console::{Console, Var};
use crate::consts::offset;
use crate::engine::Engine;
use crate::entities::Entities;
use crate::entity::Entity;
use crate::frame::Frame;
use crate::globals::Globals;
use crate::hooks;
use crate::input::Input;
use crate::interfaces::Interfaces;
use crate::libraries::Libraries;
use crate::movement::Movement;
use crate::netvars;
use crate::Result;
use core::ptr;
use std::lazy::SyncOnceCell;
use std::sync::Arc;
use vptr::VirtualMut;

pub type OnFrame = Box<dyn Fn(Frame) + 'static>;
pub type OnMove = Box<dyn Fn(Movement) -> Movement + 'static>;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell::new();

pub(crate) struct GlobalRef {
    libraries: Libraries,
    interfaces: Interfaces,
    on_frame: OnFrame,
    on_move: OnMove,
    create_move_original: Box<hooks::create_move::Signature>,
    frame_stage_notify_original: Box<hooks::frame_stage_notify::Signature>,
    local_player: Box<Option<Entity>>,
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

impl Global {
    pub fn init() -> Result<Self> {
        println!("init libs");

        let libraries = Libraries::new()?;

        println!("init interfaces");

        let interfaces = Interfaces::new(&libraries);

        println!("{:?}", &interfaces);

        let this = Self(Arc::new(GlobalRef {
            libraries,
            interfaces,
            on_frame: Box::new(move |frame| {}),
            on_move: Box::new(move |movement| movement),
            create_move_original: Box::new(hooks::create_move::hook),
            frame_stage_notify_original: Box::new(hooks::frame_stage_notify::hook),
            local_player: Box::new(None),
        }));

        println!("created global");

        GLOBAL.set(this.clone());

        println!("set global");

        unsafe {
            ptr::write(
                this.create_move_original_ptr(),
                this.interfaces().client_mode.vreplace_protected(
                    hooks::create_move::hook, // as *const hooks::create_move::Signature as *mut (),
                    offset::CREATE_MOVE * 8,
                ),
            );

            println!("hooked create_move");

            ptr::write(
                this.frame_stage_notify_original_ptr(),
                this.interfaces().client.as_mut_ptr().vreplace_protected(
                    hooks::frame_stage_notify::hook, // as *const hooks::frame_stage_notify::Signature
                    //as *mut (),
                    offset::FRAME_STAGE_NOTIFY * 8,
                ),
            );

            println!("hooked frame_stage_notify");
        }

        let client =
            unsafe { crate::client::Client::from_raw(this.interfaces().client.as_mut_ptr()) };

        netvars::set(&client);

        Ok(this)
    }

    pub fn handle() -> &'static Global {
        unsafe { GLOBAL.get().unwrap_unchecked() }
    }

    pub fn libraries(&self) -> &Libraries {
        &self.0.libraries
    }

    pub fn interfaces(&self) -> &Interfaces {
        &self.0.interfaces
    }

    pub fn globals(&self) -> &Globals {
        self.0.interfaces.globals
    }

    pub fn input(&self) -> &Input {
        self.0.interfaces.input
    }

    pub fn engine(&self) -> &Engine {
        &self.0.interfaces.engine
    }

    pub fn entities(&self) -> &Entities {
        &self.0.interfaces.entities
    }

    pub fn console(&self) -> &Console {
        &self.0.interfaces.console
    }

    pub fn cheats(&self) -> &Var<i32> {
        &self.0.interfaces.cheats
    }

    pub fn ffa(&self) -> &Var<i32> {
        &self.0.interfaces.ffa
    }

    pub fn gravity(&self) -> &Var<f32> {
        &self.0.interfaces.gravity
    }

    pub fn infinite_ammo(&self) -> &Var<i32> {
        &self.0.interfaces.infinite_ammo
    }

    pub fn lost_focus_sleep(&self) -> &Var<i32> {
        &self.0.interfaces.lost_focus_sleep
    }

    pub fn model_stats_overlay(&self) -> &Var<i32> {
        &self.0.interfaces.model_stats_overlay
    }

    pub fn panorama_blur(&self) -> &Var<i32> {
        &self.0.interfaces.panorama_blur
    }

    pub fn physics_timescale(&self) -> &Var<f32> {
        &self.0.interfaces.physics_timescale
    }

    pub fn post_processing(&self) -> &Var<i32> {
        &self.0.interfaces.post_processing
    }

    pub fn ragdoll_gravity(&self) -> &Var<f32> {
        &self.0.interfaces.ragdoll_gravity
    }

    pub fn show_impacts(&self) -> &Var<i32> {
        &self.0.interfaces.show_impacts
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

    pub(crate) fn local_player_ptr(&self) -> *mut Box<Option<Entity>> {
        &self.0.local_player as *const Box<Option<Entity>> as *mut Box<Option<Entity>>
    }

    pub(crate) fn local_player(&self) -> Option<&Entity> {
        (*self.0.local_player).as_ref()
    }

    /// set frame stage notify hook
    pub fn on_frame<F>(&self, f: F)
    where
        F: Fn(Frame) + 'static,
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
