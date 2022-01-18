use crate::bones::Bones;
use crate::client::Client;
use crate::command::Command;
use crate::console::{Console, Var};
use crate::engine::Engine;
use crate::entity::{EntityList, Player};
use crate::frame::Frame;
use crate::globals::Globals;
use crate::hooks;
use crate::hooks::Hook;
use crate::input::Input;
use crate::interfaces::Interfaces;
use crate::libraries::Libraries;
use crate::material::Material;
use crate::model::{DrawModelState, ModelInfo, ModelRender, ModelRenderInfo};
use crate::movement::Movement;
use crate::networked::Networked;
use crate::physics::Physics;
use crate::trace::RayTracer;
use crate::Result;
use core::ptr;
use sdk::Matrix3x4;
use sdk::Vec3;
use std::lazy::SyncOnceCell;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub type OnFrame = Box<dyn Fn(Frame) + 'static>;
pub type OnMove = Box<dyn Fn(Movement) -> Movement + 'static>;

static GLOBAL: SyncOnceCell<Global> = SyncOnceCell::new();

use std::collections::HashMap;

#[repr(C)]
pub struct CachedPlayer {
    bones: Bones,
}

impl CachedPlayer {
    pub fn get_bone(&self, index: usize) -> Option<&Matrix3x4> {
        self.bones.get_bone(index)
    }

    pub fn get_origin(&self, index: usize) -> Option<Vec3> {
        self.bones.get_origin(index)
    }

    pub fn get_head_bone(&self) -> &Matrix3x4 {
        self.bones.get_head_bone()
    }

    pub fn get_head_origin(&self) -> Vec3 {
        self.bones.get_head_origin()
    }
}

pub(crate) struct GlobalRef {
    // The games libraries.
    libraries: Libraries,

    // Source interfaces.
    interfaces: Interfaces,

    // Callbacks exposed to the user of this library.
    on_frame: OnFrame,
    on_move: OnMove,

    // Networked variables.
    networked: Networked,

    // TODO: Don't use Box<>
    // Kept for no recoil / no punch.
    aim_punch_angle: Box<Vec3>,
    view_punch_angle: Box<Vec3>,

    // TODO: Include ping. Fix it?? (Seems to be wrong by 10?).
    tick: AtomicU32,
    last_command_has_been_predicted: AtomicBool,

    // Function hooks.
    create_move: Hook,
    frame_stage_notify: Hook,
    draw_model_execute: Hook,

    // TODO: Don't use Box<>
    //       Don't allocate a new Box<> in frame_stage_notify::hook.
    // Reference to the local player
    local_player: Box<Option<Player>>,

    // Player cache.
    players: HashMap<i32, CachedPlayer>,
}

#[derive(Clone)]
pub struct Global(pub(crate) Arc<GlobalRef>);

unsafe impl Send for GlobalRef {}
unsafe impl Sync for GlobalRef {}

impl Global {
    pub fn init() -> Result<Self> {
        let libraries = Libraries::new()?;
        let interfaces = Interfaces::new(&libraries);
        let networked = Networked::from_client(&interfaces.client);

        let mut create_move = Hook::new(
            interfaces.client.create_move_ptr(),
            hooks::create_move::hook as *const (),
            true,
        );

        let mut frame_stage_notify = Hook::new(
            interfaces.client.frame_stage_notify_ptr(),
            hooks::frame_stage_notify::hook as *const (),
            true,
        );

        let mut draw_model_execute = Hook::new(
            // TODO: Move this to ModelRender.
            unsafe {
                crate::mem::virtual_offset(interfaces.model_render.as_ptr() as *const (), 21)
            },
            hooks::draw_model_execute::hook as *const (),
            true,
        );

        create_move.apply_protected();

        println!("Hooked create_move.");

        frame_stage_notify.apply_protected();

        println!("Hooked frame_stage_notify.");

        draw_model_execute.apply_protected();

        println!("Hooked draw_model_execute.");

        let this = Self(Arc::new(GlobalRef {
            libraries,
            interfaces,

            // Placeholder callbacks.
            on_frame: Box::new(move |_frame| {}),
            on_move: Box::new(move |movement| movement),

            networked,

            aim_punch_angle: Box::new(Vec3::zero()),
            view_punch_angle: Box::new(Vec3::zero()),

            tick: AtomicU32::new(0),
            last_command_has_been_predicted: AtomicBool::new(false),

            // Hooks!
            create_move,
            frame_stage_notify,
            draw_model_execute,

            players: HashMap::new(),

            local_player: Box::new(None),
        }));

        // TODO: Check if it already exists.
        let _ = GLOBAL.set(this.clone());

        Ok(this)
    }

    pub fn handle() -> &'static Global {
        unsafe { GLOBAL.get().unwrap_unchecked() }
    }

    /// Returns a reference to the networked variable manager.
    pub fn networked<'networked>(&'networked self) -> &'networked Networked {
        &self.0.networked
    }

    /// Current client time.
    pub fn client_time(&self) -> f32 {
        self.0.interfaces.globals.current_time
    }

    /// The interval (in seconds) that one tick takes.
    ///
    /// 1 second / 64 ticks = 0.015625 seconds
    /// 1 second / 128 ticks = 0.0078125 seconds
    pub fn interval_per_tick(&self) -> f32 {
        self.0.interfaces.globals.interval_per_tick
    }

    pub fn tick(&self) -> u32 {
        self.0.tick.load(Ordering::SeqCst)
    }

    pub fn set_tick(&self, tick: u32) {
        self.0.tick.store(tick, Ordering::SeqCst);
    }

    pub fn increment_tick(&self) {
        self.0.tick.fetch_add(1, Ordering::SeqCst);
    }

    pub fn last_command_has_been_predicted(&self) -> bool {
        self.0
            .last_command_has_been_predicted
            .load(Ordering::SeqCst)
    }

    pub fn set_last_command_has_been_predicted(&self, predicted: bool) {
        self.0
            .last_command_has_been_predicted
            .store(predicted, Ordering::SeqCst);
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

    pub fn physics(&self) -> &Physics {
        &self.0.interfaces.physics
    }

    pub fn input(&self) -> &Input {
        self.0.interfaces.input
    }

    pub fn engine(&self) -> &Engine {
        &self.0.interfaces.engine
    }

    pub fn entity_list(&self) -> &EntityList {
        &self.0.interfaces.entity_list
    }

    pub fn ray_tracer(&self) -> &RayTracer {
        &self.0.interfaces.tracer
    }

    pub fn client(&self) -> &Client {
        &self.0.interfaces.client
    }

    pub fn model_render(&self) -> &ModelRender {
        &self.0.interfaces.model_render
    }

    pub fn model_info(&self) -> &ModelInfo {
        &self.0.interfaces.model_info
    }

    pub fn console(&self) -> &Console {
        &self.0.interfaces.console
    }

    pub fn animation_layers(&self) -> u32 {
        self.0.interfaces.animation_layers
    }

    pub fn animation_state(&self) -> u32 {
        self.0.interfaces.animation_state
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
        &self.0.create_move.original as *const *const () as *const hooks::create_move::Signature
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
        &self.0.frame_stage_notify.original as *const *const ()
            as *const hooks::frame_stage_notify::Signature
            as *mut hooks::frame_stage_notify::Signature
    }

    pub(crate) fn frame_stage_notify_original(&self, this: *const (), frame: Frame) {
        let original = unsafe { *self.frame_stage_notify_original_ptr() };

        unsafe { original(this, frame) }
    }

    pub(crate) fn draw_model_execute_original_ptr(
        &self,
    ) -> *mut hooks::draw_model_execute::Signature {
        &self.0.draw_model_execute.original as *const *const ()
            as *const hooks::draw_model_execute::Signature
            as *mut hooks::draw_model_execute::Signature
    }

    pub(crate) fn draw_model_execute_original(
        &self,
        this: *const (),
        context: *const (),
        state: *const DrawModelState,
        info: *const ModelRenderInfo,
        bone_to_world: *const Matrix3x4,
    ) {
        let original = unsafe { *self.draw_model_execute_original_ptr() };

        unsafe { original(this, context, state, info, bone_to_world) }
    }

    pub(crate) fn local_player_ptr(&self) -> *mut Box<Option<Player>> {
        &self.0.local_player as *const Box<Option<Player>> as *mut Box<Option<Player>>
    }

    pub(crate) fn local_player(&self) -> Option<&Player> {
        (*self.0.local_player).as_ref()
    }

    pub(crate) fn aim_punch_angle_ptr(&self) -> *mut Vec3 {
        unsafe { &mut *(&*self.0.aim_punch_angle as *const Vec3 as *mut Vec3) }
    }

    pub(crate) fn aim_punch_angle(&self) -> Vec3 {
        unsafe { *self.aim_punch_angle_ptr() }
    }

    pub(crate) fn set_aim_punch_angle(&self, angle: Vec3) {
        unsafe {
            *self.aim_punch_angle_ptr() = angle;
        }
    }

    pub(crate) fn view_punch_angle_ptr(&self) -> *mut Vec3 {
        unsafe { &mut *(&*self.0.view_punch_angle as *const Vec3 as *mut Vec3) }
    }

    pub(crate) fn view_punch_angle(&self) -> Vec3 {
        unsafe { *self.view_punch_angle_ptr() }
    }

    pub(crate) fn set_view_punch_angle(&self, angle: Vec3) {
        unsafe {
            *self.view_punch_angle_ptr() = angle;
        }
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

    pub fn flat_material(&self) -> &Material {
        &self.0.interfaces.flat
    }
}
