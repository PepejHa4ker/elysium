#![feature(const_fn_fn_ptr_basics)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(extern_types)]
#![feature(once_cell)]
#![feature(ptr_metadata)]

use crate::frame::Frame;
use crate::global::Global;
use atomic_float::AtomicF32;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod client;
pub mod command;
pub mod console;
pub mod consts;
pub mod debug;
pub mod engine;
pub mod entity;
pub mod error;
pub mod frame;
pub mod global;
pub mod globals;
pub mod hit_group;
pub mod hooks;
pub mod input;
pub mod interfaces;
pub mod islice;
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod managed;
pub mod material;
pub mod mem;
pub mod model;
pub mod move_kind;
pub mod movement;
pub mod networked;
pub mod pad;
pub mod pattern;
pub mod physics;
pub mod trace;

#[derive(Clone)]
pub struct Choker(Arc<AtomicUsize>);

impl Choker {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub fn reset(&self) {
        self.0.store(0, Ordering::SeqCst);
    }

    pub fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

struct AtomicVec3Inner {
    x: AtomicF32,
    y: AtomicF32,
}

#[derive(Clone)]
pub struct AtomicVec3(Arc<AtomicVec3Inner>);

impl AtomicVec3 {
    pub fn new() -> Self {
        Self(Arc::new(AtomicVec3Inner {
            x: AtomicF32::new(0.0),
            y: AtomicF32::new(0.0),
        }))
    }

    pub fn x(&self) -> f32 {
        self.0.x.load(Ordering::SeqCst)
    }

    pub fn y(&self) -> f32 {
        self.0.y.load(Ordering::SeqCst)
    }

    pub fn set_x(&self, x: f32) {
        self.0.x.store(x, Ordering::SeqCst);
    }

    pub fn set_y(&self, y: f32) {
        self.0.y.store(y, Ordering::SeqCst);
    }

    pub fn get(&self) -> Vec3 {
        Vec3::from_xy(self.x(), self.y())
    }

    pub fn set(&self, angle: Vec3) {
        self.set_x(angle.x);
        self.set_y(angle.y);
    }
}

mod contents;

use providence_math::Vec3;

fn main() -> Result<()> {
    // todo: check we are loaded into counter strike
    //       we get loaded into bash too when executing
    //       counter strike from steam (csgo.sh)

    if library::Library::serverbrowser().is_err() {
        println!("Waiting for Counter Strike to finish loading...");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!("Initializing providence...");

    let global = Global::init()?;
    let global2 = global.clone();
    let global3 = global.clone();

    let choked_packets = Choker::new();

    let original_angle = AtomicVec3::new();
    let thirdperson_angle = AtomicVec3::new();
    let thirdperson_angle2 = thirdperson_angle.clone();

    let tick_start = AtomicI32::new(0);

    global.on_frame(move |frame| {
        if let Some(local_player) = global2.local_player() {
            match frame {
                Frame::RENDER_START => {
                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(1);
                    global2.physics_timescale().set(0.5);
                    //global2.ragdoll_gravity().set(-800.0);

                    // No recoil / no punch.
                    global2.set_aim_punch_angle(local_player.actual_aim_punch_angle());
                    global2.set_view_punch_angle(local_player.actual_view_punch_angle());

                    local_player.set_aim_punch_angle(Vec3::zero());
                    local_player.set_view_punch_angle(Vec3::zero());

                    if global2.input().thirdperson {
                        original_angle.set(local_player.view_angle());
                        local_player.set_view_angle(thirdperson_angle.get());
                    }
                }
                Frame::RENDER_END => {
                    // Restore aim and view punch to not break things.
                    local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    local_player.set_view_punch_angle(global2.view_punch_angle());

                    // Restore local player angle
                    if global2.input().thirdperson {
                        local_player.set_view_angle(original_angle.get());
                    }

                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(0);
                    global2.physics_timescale().set(1.0);
                    //global2.ragdoll_gravity().set(800.0);
                }
                _ => {}
            }
        }

        global2.panorama_blur().set(1);
        global2.cheats().set(1);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        let pure_view_angle = global3.engine().view_angle().to_trusted();
        let pure_forward_move = movement.forward_move;
        let pure_side_move = movement.side_move;

        if movement.local_player.is_dead() {
            movement.view_angle = pure_view_angle;
            movement.forward_move = 0.0;
            movement.side_move = 0.0;

            thirdperson_angle2.set(movement.view_angle);

            return movement;
        }

        movement.do_fast_duck = true;

        thirdperson_angle2.set(movement.view_angle);

        movement
    });

    Ok(())
}

#[ctor::ctor]
fn load() {
    let _ = thread::Builder::new().spawn(move || {
        let _ = main();
    });
}
