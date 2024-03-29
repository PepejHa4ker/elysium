#![deny(warnings)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_const_cast)]
// src/entity.rs
#![feature(const_ptr_offset_from)]

use elysium_dl::Library;
use elysium_sdk::convar::Vars;
use elysium_sdk::{Client, Console};
use std::path::Path;
use std::time::Duration;
use std::{mem, thread};

pub use elysium_state as state;

pub use entity::Entity;
pub use networked::Networked;

mod entity;
pub mod networked;
//mod simulation;

pub mod hooks;
pub mod library;
pub mod pattern;

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

            if process_name == "csgo_linux64" ||
                // https://github.com/elysian6969/csgo-launcher xoxo
                process_name == "csgo-launcher"
            {
                Some(true)
            } else {
                None
            }
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

    let interfaces = library::load_interfaces();
    let console: &'static Console = unsafe { &*interfaces.convar.cast() };
    let client: &'static Client = unsafe { &*interfaces.client.cast() };
    let globals = client.globals();
    let input = client.input();

    console.write("welcome to elysium\n");

    let vars = Vars::from_loader(|var_kind| {
        let var_nul_name = var_kind.as_nul_str();
        let var_name = var_kind.as_str();
        let address = console.var(var_nul_name);

        println!("elysium | config variable \x1b[38;5;2m{var_name}\x1b[m found at \x1b[38;5;3m{address:?}\x1b[m");

        address
    });

    let networked = Networked::new(client);

    let gl = elysium_gl::Gl::open().expect("libGL");

    println!(
        "elysium | loaded \x1b[38;5;2mlibGL\x1b[m at \x1b[38;5;3m{:?}\x1b[m",
        gl
    );

    let sdl = elysium_sdl::Sdl::open().expect("libSDL");

    println!(
        "elysium | loaded \x1b[38;5;2mlibSDL\x1b[m at \x1b[38;5;3m{:?}\x1b[m",
        sdl
    );

    let swap_window = unsafe { sdl.swap_window().expect("SDL_GL_SwapWindow") };
    let poll_event = unsafe { sdl.poll_event().expect("SDL_PollEvent") };

    let patterns = pattern::Libraries::new();
    let _animation_layers = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::ANIMATION_LAYERS,
                "animation_layers",
            )
            .expect("animation layers");

        address.byte_add(35).cast::<u32>().read()
    };

    let _animation_state = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::ANIMATION_STATE,
                "animation_state",
            )
            .expect("animation state");

        address.byte_add(52).cast::<u32>().read()
    };

    /* pattern is brokey
    * let host_run_frame_input = unsafe {
        let address = patterns
            .address_of(
                "engine_client.so",
                &pattern::HOST_RUN_FRAME_INPUT,
                "host_run_frame_input",
            )
            .expect("host run frame input");

        address
    };*/

    let _cl_move = unsafe {
        let cl_move = patterns
            .address_of("engine_client.so", &pattern::CL_MOVE, "cl_move")
            .expect("cl move");

        let cl_move: state::hooks::ClMove = mem::transmute(cl_move);

        state::hooks::set_cl_move(cl_move);

        cl_move
    };

    /*let write_user_command = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::WRITE_USER_COMMAND,
                "write_user_command",
            )
            .expect("write user command");

        let write_user_command: state::hooks::WriteUserCommand = mem::transmute(address);

        state::hooks::set_write_user_command(write_user_command);

        write_user_command
    };

    let write_user_command_delta_to_buffer = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::WRITE_USER_COMMAND_DELTA_TO_BUFFER,
                "write_user_command_delta_to_buffer",
            )
            .expect("write user command delta to buffer");

        let write_user_command_delta_to_buffer: state::hooks::WriteUserCommand =
            mem::transmute(address);

        write_user_command_delta_to_buffer
    };*/

    unsafe {
        let gl_context = elysium_gl::Context::new(|symbol| gl.get_proc_address(symbol).cast());
        let swap_window = swap_window as *mut state::hooks::SwapWindow;
        let poll_event = poll_event as *mut state::hooks::PollEvent;

        state::set_gl(gl);
        state::set_sdl(sdl);

        state::set_gl_context(gl_context);

        state::set_networked(mem::transmute(networked));
        state::set_vars(mem::transmute(vars));

        state::set_engine(interfaces.engine);
        state::set_entity_list(interfaces.entity_list);
        state::set_globals(globals);
        state::set_input(input);

        // e8 <relative>  call  CL_Move
        // 0x005929d3 - 0x00592910 = 195
        /*{
            let cl_move_hook = hooks::cl_move as usize as *const u8;
            let call_cl_move = host_run_frame_input.byte_offset(195);

            // obtain rip
            let rip = call_cl_move.byte_offset(5);

            // calulate relative
            let relative = cl_move_hook.byte_offset_from(rip);

            // remove protection
            let protection = elysium_mem::unprotect(call_cl_move);

            // replace relative
            /*let original = call_cl_move
            .byte_offset(1)
            .cast::<i32>()
            .as_mut()
            .replace(relative as i32);*/

            //println!("cl_move_hook relative (original) = {original:?}");

            // restore protection
            elysium_mem::protect(call_cl_move, protection);
        }*/

        {
            let address = client
                .create_move_address()
                .as_mut()
                .cast::<state::hooks::CreateMove>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_create_move(address.replace(hooks::create_move));
            println!("elysium | hooked \x1b[38;5;2mCreateMove\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        {
            let address = client
                .frame_stage_notify_address()
                .as_mut()
                .cast::<state::hooks::FrameStageNotify>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_frame_stage_notify(address.replace(hooks::frame_stage_notify));
            println!("elysium | hooked \x1b[38;5;2mFrameStageNotify\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        {
            let address = client
                .override_view_address()
                .as_mut()
                .cast::<state::hooks::OverrideView>();

            // remove protection
            let protection = elysium_mem::unprotect(address);

            state::hooks::set_override_view(address.replace(hooks::override_view));
            println!("elysium | hooked \x1b[38;5;2mOverrideView\x1b[m");

            // restore protection
            elysium_mem::protect(address, protection);
        }

        state::hooks::set_swap_window(swap_window.replace(hooks::swap_window));
        println!("elysium | hooked \x1b[38;5;2mSDL_GL_SwapWindow\x1b[m");

        state::hooks::set_poll_event(poll_event.replace(hooks::poll_event));
        println!("elysium | hooked \x1b[38;5;2mSDL_PollEvent\x1b[m");

        /*{
            #[repr(C, packed)]
            struct Jmp4 {
                jmp: u8,
                rel: i32,
            }

            let hook = hooks::write_user_command_delta_to_buffer as *const u8;
            let base = write_user_command_delta_to_buffer as *const u8;
            let rip = base.byte_add(5);
            let rel = (hook as *const u8).byte_offset_from(rip) as i32;
            let jmp = Jmp4 { jmp: 0xE9, rel };

            // remove protection
            let protection = elysium_mem::unprotect(base);

            // write jmp
            base.as_mut().cast::<Jmp4>().write(jmp);
            println!("elysium | hooked \x1b[38;5;2mWriteUsercmdDeltaToBuffer\x1b[m");

            // restore protection
            elysium_mem::protect(base, protection);
        }*/
    }
}
