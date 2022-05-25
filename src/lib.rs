#![feature(const_maybe_uninit_zeroed)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(extern_types)]
#![feature(once_cell)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(ptr_metadata)]

use crate::entity::Entity;
use crate::entity::Player;
use crate::entity::Weapon;
use crate::frame::Frame;
use crate::global::Global;
use core::ptr;
use elysium_math::Vec3;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod client;
pub mod command;
pub mod console;
pub mod consts;
pub mod entity;
pub mod frame;
pub mod global;
pub mod globals;
pub mod hooks;
pub mod interfaces;
pub mod islice;
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod managed;
pub mod material;
pub mod model;
pub mod move_kind;
pub mod movement;
pub mod networked;
pub mod pattern;
pub mod physics;

use elysium_dl::Library;
use std::path::Path;

pub use elysium_state as state;

pub mod hooks2;

#[link_section = ".init_array"]
#[used]
static BOOTSTRAP: unsafe extern "C" fn() = bootstrap;

#[link_section = ".text.startup"]
unsafe extern "C" fn bootstrap() {
    // check the name of the process we're injected into
    let is_csgo = std::env::args()
        .next()
        .and_then(|process_path| {
            let process_path = Path::new(&process_path);
            let process_name = process_path.file_name()?;

            Some(process_name == "csgo_linux64")
        })
        .unwrap_or(false);

    // bail if we're injected into not csgo
    if !is_csgo {
        return;
    }

    // spawn a new thread to prevent blocking csgo
    thread::spawn(main);
}

#[inline]
fn main() {
    // wait for serverbrowser.so to load as it is the last to load.
    frosting::println!("waiting for `serverbrowser_client.so` to load");

    loop {
        if Library::exists("./bin/linux64/serverbrowser_client.so") {
            break;
        }

        thread::sleep(Duration::from_millis(500));
    }

    frosting::println!("`serverbrowser_client.so` loaded");
    frosting::println!("looking for libray: libGL.so.1");

    let gl = match elysium_gl::Gl::open() {
        Some(gl) => gl,
        None => {
            frosting::println!("failed to load GL, aborting");
            return;
        }
    };

    frosting::println!("loaded GL: {:016x?}", gl);

    unsafe {
        let gl_context = elysium_gl::Context::new(|symbol| gl.get_proc_address(symbol).cast());

        state::set_gl_context(gl_context);
        state::set_gl(gl);
    }

    frosting::println!("looking for libray: libSDL-2.0.so.0");

    let sdl = match elysium_sdl::Sdl::open() {
        Some(sdl) => sdl,
        None => {
            frosting::println!("failed to load SDL, aborting");
            return;
        }
    };

    frosting::println!("loaded SDL: {:016x?}", sdl);
    frosting::println!("looking for symbol: `SDL_GL_SwapWindow`");

    let swap_window = match unsafe { sdl.swap_window() } {
        Some(swap_window) => swap_window,
        None => {
            frosting::println!("failed to find symbol `SDL_GL_SwapWindow`, aborting");
            return;
        }
    };

    frosting::println!("`SDL_GL_SwapWindow`: {:016x?}", swap_window);
    frosting::println!("looking for symbol: `SDL_PollEvent`");

    let poll_event = match unsafe { sdl.poll_event() } {
        Some(poll_event) => poll_event,
        None => {
            frosting::println!("failed to find symbol `SDL_PollEvent`, aborting");
            return;
        }
    };

    frosting::println!("found `SDL_PollEvent`: {:016x?}", poll_event);

    unsafe {
        state::set_sdl(sdl);

        frosting::println!("hooking `SDL_GL_SwapWindow`");

        let swap_window = swap_window as *mut state::SwapWindowFn;

        state::set_swap_window(swap_window.replace(hooks2::swap_window));

        frosting::println!("hooking `SDL_PollEvent`");

        let poll_event = poll_event as *mut state::PollEventFn;

        state::set_poll_event(poll_event.replace(hooks2::poll_event));
    }

    println!("initializing providence...");

    let global = Global::init().expect("global");
    let global2 = global.clone();
    let global3 = global.clone();
    let choked = AtomicI32::new(0);

    global.on_frame(move |frame| {
        let input = unsafe { &*elysium_state::input().cast::<elysium_sdk::Input>() };

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

                    // "Fix" the local players view angle and backup the current value.
                    if input.thirdperson {
                        unsafe {
                            let original_view_angle = local_player.view_angle();

                            *elysium_state::local::view_angle() = original_view_angle;

                            local_player.set_view_angle(*elysium_state::view_angle());
                        }
                    }

                    unsafe {
                        let cached_players = &mut *elysium_state::players();
                        let entity_list = global2.entity_list();
                        let client_time = global2.client_time();

                        for index in 1..64 {
                            let bones = &mut cached_players[index as usize].bones;

                            if let Some(entity) = entity_list.get(index) {
                                let bones_ptr = bones.as_mut_ptr();

                                entity.setup_bones(bones_ptr, 128, 0x00000100, client_time);
                                entity.setup_bones(bones_ptr, 128, 0x000FFF00, client_time);
                            } else {
                                *bones = providence_model::Bones::zero();
                            }
                        }

                        let highest_entity_index = entity_list.highest_entity_index();

                        for index in 1..highest_entity_index {
                            let entity = match entity_list.get(index) {
                                Some(entity) => entity,
                                None => continue,
                            };

                            let class = entity.class();

                            if class.is_null() {
                                continue;
                            }

                            let class = &*class;

                            use crate::entity::{EntityId, Fog};

                            if class.entity_id == EntityId::CFogController {
                                //println!("found fog!");

                                let fog = Fog::new_unchecked(entity.as_ptr() as *mut _);

                                *fog.is_enabled() = true;
                                *fog.start() = 1.0;
                                *fog.end() = 10000.0;
                                *fog.far_z() = 10000.0;
                                *fog.density() = 1.0;
                                *fog.color_primary() = 0x00FF00FF;
                            }
                        }
                    }
                }
                Frame::RENDER_END => {
                    // Restore aim and view punch to not break things.
                    local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    local_player.set_view_punch_angle(global2.view_punch_angle());

                    // Restore the local players view_angle.
                    if input.thirdperson {
                        unsafe {
                            local_player.set_view_angle(*elysium_state::local::view_angle());
                        }
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
        let engine = unsafe { &*elysium_state::engine().cast::<elysium_sdk::Engine>() };
        let network_channel = unsafe { &*engine.get_network_channel() }; //unsafe { &*elysium_state::network_channel().cast::<elysium_sdk::NetworkChannel>() };
        let latency = network_channel.get_latency(0);
        let choked_packets = network_channel.choked_packets;
        let level_name = engine.get_level_name();
        let view_angle = engine.get_view_angle();
        let vectors = movement.vectors;
        let punch = movement.local_player.aim_punch_angle() * Vec3::splat(2.0);
        let original_vectors = movement.vectors;
        let side = (movement.tick_count * 2 - 1) as f32;

        println!("level_name = {level_name:?}");
        println!("latency = {latency:?}");
        println!("choked_packets = {choked_packets:?}");

        if movement.send_packet {
            unsafe {
                *elysium_state::view_angle() = movement.view;

                let cached_players = &mut *elysium_state::players();
                let index = movement.local_player.index();
                let bones = &mut cached_players[index as usize].bones;
                let local_player_bones = &mut *elysium_state::local::bones();

                ptr::copy_nonoverlapping(
                    bones.as_ptr(),
                    local_player_bones.as_mut_ptr(),
                    providence_model::MAX_BONES,
                );
            }
        }

        movement
    });
}
