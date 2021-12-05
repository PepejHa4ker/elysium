#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(link_llvm_intrinsics)]
#![feature(const_mut_refs)]
#![feature(trait_alias)]

use crate::global::Global;
use crate::log::Logger;
use atomic_float::AtomicF32;
use rand::{thread_rng, Rng};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
pub mod global;
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
pub mod movement;
pub mod netvars;
pub mod player_state;
pub mod skybox;
pub mod trace;
pub mod util;

fn main(_logger: Logger) -> Result<()> {
    if library::Library::serverbrowser().is_err() {
        println!("waiting for csgo to load");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    let global = Global::init()?;
    let global2 = global.clone();
    let global3 = global.clone();
    let yaw = Arc::new(AtomicF32::new(0.0));
    let yaw2 = yaw.clone();
    let next_lby_update = Arc::new(AtomicF32::new(0.0));
    let next_lby_update2 = next_lby_update.clone();
    let lby_updated = Arc::new(AtomicBool::new(false));

    global.on_frame(move |_frame| {
        // thirdperson fix
        if global2.input().thirdperson {
            if let Some(local_player) = global2.local_player() {
                local_player.view_angle().pitch = 89.0;
                local_player.view_angle().yaw = yaw.load(Ordering::SeqCst);
            }
        }

        global2.cheats().set(1);
        global2.lost_focus_sleep().set(1);
        global2.panorama_blur().set(1);
        global2.physics_timescale().set(0.5);
        global2.ragdoll_gravity().set(-800.0);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        //movement.send_packet = movement.tick_count % 14 == 0;

        if !movement.local_player.flags().on_ground() {
            movement.in_jump = false;
        }

        if movement.in_duck {
            movement.in_fast_duck = true;
        }

        if movement.in_attack {
            yaw2.store(movement.view_angle.yaw, Ordering::SeqCst);
            lby_updated.store(false, Ordering::SeqCst);
        } else {
            if movement.local_player.velocity().magnitude() > 0.1 {
                next_lby_update2.store(movement.current_time + 0.22, Ordering::SeqCst);
            }

            if movement.current_time >= next_lby_update2.load(Ordering::SeqCst) {
                next_lby_update2.store(movement.current_time + 1.1, Ordering::SeqCst);
                lby_updated.store(!lby_updated.load(Ordering::SeqCst), Ordering::SeqCst);
            }

            movement.view_angle = global3.engine().view_angle();

            let client_yaw = movement.view_angle.yaw + 180.0;

            if lby_updated.load(Ordering::SeqCst) {
                movement.view_angle.yaw = client_yaw - 58.0;
                movement.send_packet = false;

                println!("lby  yaw = {:?}", movement.view_angle.yaw);
            } else if movement.send_packet {
                movement.view_angle.yaw = client_yaw;

                println!("real yaw = {:?}", movement.view_angle.yaw);
            } else {
                movement.view_angle.yaw = client_yaw + 120.0;

                println!("fake yaw = {:?}", movement.view_angle.yaw);
            }

            movement.view_angle.pitch = 89.0;
        }

        if movement.send_packet {
            yaw2.store(movement.view_angle.yaw, Ordering::SeqCst);
        }

        movement
    });

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
