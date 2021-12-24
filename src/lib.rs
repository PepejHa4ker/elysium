#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(const_ptr_is_null)]
#![feature(once_cell)]
#![feature(extern_types)]
#![feature(ptr_metadata)]
#![feature(trait_alias)]

use crate::global::Global;
use atomic_float::AtomicF32;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod client;
pub mod command;
pub mod console;
pub mod consts;
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
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod move_kind;
pub mod movement;
pub mod netvars;
pub mod pad;
pub mod pattern;
pub mod player_state;
pub mod skybox;
pub mod trace;

fn main() -> Result<()> {
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
    let choked_packets = Arc::new(AtomicUsize::new(0));

    global.on_frame(move |_frame| {
        if let Some(local_player) = global2.local_player() {
            // thirdperson fix
            unsafe {
                *(&global2.input().thirdperson as *const bool as *mut bool) = true;
            }

            if global2.input().thirdperson {
                local_player.view_angle().pitch = 89.0;
                local_player.view_angle().yaw = yaw.load(Ordering::SeqCst);
            }

            /*for player in global2.entities().iter() {
                let mut animation_layers = player.animation_layers();

                animation_layers[12].weight = 0.0;
            }*/
        }

        global2.cheats().set(1);
        global2.lost_focus_sleep().set(1);
        global2.panorama_blur().set(1);
        global2.physics_timescale().set(0.5);
        global2.ragdoll_gravity().set(-800.0);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
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

            let client_yaw = movement.view_angle.yaw + 180.0 + 58.0;

            if lby_updated.load(Ordering::SeqCst) {
                movement.view_angle.yaw = client_yaw - 58.0;
                movement.send_packet = false;

                //println!("lby  yaw = {:?}", movement.view_angle.yaw);
            } else if movement.send_packet {
                movement.view_angle.yaw = client_yaw;

                //println!("real yaw = {:?}", movement.view_angle.yaw);
            } else {
                movement.view_angle.yaw = client_yaw + 120.0;

                //println!("fake yaw = {:?}", movement.view_angle.yaw);
            }

            movement.view_angle.pitch = 89.0;
        }

        if !movement.send_packet {
            yaw2.store(movement.view_angle.yaw, Ordering::SeqCst);
        }

        //println!("tick_count = {}", movement.tick_count);

        let choked_packets2 = choked_packets.load(Ordering::SeqCst);

        if choked_packets2 > 6 {
            movement.send_packet = true;
            //movement.tick_count = i32::MAX;
            choked_packets.store(0, Ordering::SeqCst);
        } else {
            movement.send_packet = false;
            choked_packets.store(choked_packets2 + 1, Ordering::SeqCst);
        }

        println!("choked_packets = {:?}", choked_packets2);

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
