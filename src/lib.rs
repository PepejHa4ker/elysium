#![feature(const_trait_impl)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(once_cell)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(link_llvm_intrinsics)]
#![feature(const_mut_refs)]
#![feature(trait_alias)]

use crate::consts::offset;
use crate::frame::Frame;
use crate::global::Global;
use crate::log::Logger;
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::{mem, thread};

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

fn main(logger: Logger) -> Result<()> {
    if library::Library::serverbrowser().is_err() {
        println!("waiting for csgo to load");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Arc;

    let global = Global::init()?;
    let global2 = global.clone();
    let global3 = global.clone();
    let yaw = Arc::new(AtomicI32::new(0));
    let yaw2 = yaw.clone();

    global.on_frame(move |frame| {
        // thirdperson fix
        if global2.input().thirdperson {
            if let Some(local_player) = global2.local_player() {
                local_player.view_angle().pitch = 89.0;
                local_player.view_angle().yaw = yaw.load(Ordering::SeqCst) as f32;
            }
        }

        //global2.cheats().set(0);
        global2.lost_focus_sleep().set(1);
        global2.panorama_blur().set(1);
        global2.physics_timescale().set(0.5);
        global2.ragdoll_gravity().set(-800.0);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        movement.send_packet = movement.tick_count % 14 == 0;

        if !movement.local_player.flags().on_ground() {
            movement.in_jump = false;
        }

        if movement.in_duck {
            movement.in_fast_duck = true;
        }

        if !movement.in_attack {
            let mut rng = thread_rng();
            let real: i32 =
                if movement.tick_count % 2 == 0 { -1 } else { 1 } * rng.gen_range(32..=58);
            let fake: i32 = 58;

            movement.view_angle = global3.engine().view_angle();
            movement.view_angle.yaw += 180.0;

            if movement.send_packet {
                movement.view_angle.yaw = real as f32;
            } else {
                movement.view_angle.yaw = (real + (fake * 2)) as f32;
                yaw2.store(movement.view_angle.yaw as i32, Ordering::SeqCst);
            }

            movement.view_angle.pitch = 89.0;
        }

        println!("{:?}", &movement);

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
