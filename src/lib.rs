#![feature(const_maybe_uninit_zeroed)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(extern_types)]
#![feature(once_cell)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
#![feature(ptr_metadata)]

use crate::global::Global;
use elysium_dl::Library;
use elysium_sdk::Frame;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub use elysium_state as state;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod command;
pub mod console;
pub mod consts;
pub mod entity;
pub mod global;
pub mod globals;
pub mod hooks;
pub mod hooks2;
pub mod interfaces;
pub mod islice;
pub mod item_kind;
pub mod library;
pub mod managed;
pub mod material;
pub mod model;
pub mod move_kind;
pub mod movement;
pub mod networked;
pub mod pattern;
pub mod physics;

// this is called by glibc after the library is loaded into a process
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
    println!("elysium | waiting for \x1b[38;5;2m`serverbrowser_client.so`\x1b[m to load");

    loop {
        if Library::exists("./bin/linux64/serverbrowser_client.so") {
            break;
        }

        thread::sleep(Duration::from_millis(500));
    }

    println!("elysium | \x1b[38;5;2m`serverbrowser_client.so`\x1b[m loaded, continuing...");
    
    frosting::println!("looking for libray: \x1b[38;5;2m`libGL.so.1`\x1b[m");

    let gl = match elysium_gl::Gl::open() {
        Some(gl) => gl,
        None => {
            frosting::println!("\x1b[38;5;1mfailed to load GL, aborting\x1b[m");
            return;
        }
    };

    frosting::println!("loaded GL: \x1b[38;5;3m{:016x?}\x1b[m", gl);

    unsafe {
        let gl_context = elysium_gl::Context::new(|symbol| gl.get_proc_address(symbol).cast());

        state::set_gl_context(gl_context);
        state::set_gl(gl);
    }

    frosting::println!("looking for libray: \x1b[38;5;2m`libSDL-2.0.so.0`\x1b[m");

    let sdl = match elysium_sdl::Sdl::open() {
        Some(sdl) => sdl,
        None => {
            frosting::println!("\x1b[38;5;1mfailed to load SDL, aborting\x1b[m");
            return;
        }
    };

    frosting::println!("loaded SDL: \x1b[38;5;3m{:016x?}\x1b[m", sdl);
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

    let global = Global::init().expect("global");
    let global2 = global.clone();

    global.on_frame(move |frame| {
        match frame {
            Frame::RenderStart => {
                let vars = &global2.0.interfaces.vars;

                // misc
                vars.cheats.write(true);

                // annoying
                vars.auto_help.write(false);
                vars.show_help.write(false);

                // these disable when true
                vars.engine_sleep.write(true);
                vars.html_motd.write(true);
                vars.freeze_cam.write(true);
                vars.panorama_blur.write(true);

                // p100
                vars.hud.write(false);

                // shadows
                vars.csm.write(false);
                vars.csm_shadows.write(false);
                vars.feet_shadows.write(false);
                vars.prop_shadows.write(false);
                vars.rope_shadows.write(false);
                vars.shadows.write(false);
                vars.skybox3d.write(false);
                vars.viewmodel_shadows.write(false);
                vars.world_shadows.write(false);

                // useless objects
                vars.ropes.write(false);
                vars.sprites.write(false);

                // translucent things
                //vars.translucent_renderables.write(false);
                //vars.translucent_world.write(false);
                vars.water_fog.write(false);

                // overlay
                vars.underwater_overlay.write(false);

                // effects
                vars.blood.write(false);
                vars.decals.write(false);
                vars.jiggle_bones.write(false);
                vars.rain.write(false);

                // phsyics
                vars.physics_timescale.write(0.5);
            }
            _ => {}
        }

        let input = unsafe { &*elysium_state::input().cast::<elysium_sdk::Input>() };

        if let Some(local_player) = global2.local_player() {
            match frame {
                Frame::RenderStart => {
                    use elysium_state::local;

                    // todo refactor
                    let aim_punch_angle = local_player.actual_aim_punch_angle();
                    let view_angle = local_player.view_angle();
                    let view_punch_angle = local_player.actual_view_punch_angle();

                    local::set_aim_punch_angle(aim_punch_angle);
                    local::set_view_angle(view_angle);
                    local::set_view_punch_angle(view_punch_angle);

                    //local_player.set_aim_punch_angle(Vec3::zero());
                    //local_player.set_view_punch_angle(Vec3::zero());

                    // the game doesnt render actual local player angles by default
                    if input.thirdperson {
                        local_player.set_view_angle(*elysium_state::view_angle());
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
                Frame::RenderEnd => {
                    // Restore aim and view punch to not break things.
                    //local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    //local_player.set_view_punch_angle(global2.view_punch_angle());

                    // Restore the local players view_angle.
                    if input.thirdperson {
                        unsafe {
                            local_player.set_view_angle(elysium_state::local::view_angle());
                        }
                    }
                }
                _ => {}
            }
        }
    });
}
