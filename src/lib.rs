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

use elysium_dl::Library;
use elysium_sdk::convar::Vars;
use elysium_sdk::Console;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub use elysium_state as state;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod command;
pub mod consts;
//pub mod entity;
pub mod globals;
pub mod hooks;
pub mod hooks2;
//pub mod interfaces;
pub mod item_kind;
pub mod library;
pub mod managed;
pub mod material;
pub mod model;
pub mod move_kind;
//pub mod movement;
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

    let interfaces = library::load_interfaces();
    let console: &'static Console = unsafe { &*interfaces.convar.cast() };

    console.write("welcome to elysium\n");

    let vars = unsafe {
        Vars::from_loader(|name| {
            let address = console.var(name);

            println!("elysium | config variable \x1b[38;5;2m{name}\x1b[m found at \x1b[38;5;3m{address:?}\x1b[m");

            address
        })
    };

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
    let animation_layers = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::ANIMATION_LAYERS,
                "animation_layers",
            )
            .expect("animation layers");

        address.byte_add(35).cast::<u32>().read()
    };

    let animation_state = unsafe {
        let address = patterns
            .address_of(
                "client_client.so",
                &pattern::ANIMATION_STATE,
                "animation_state",
            )
            .expect("animation state");

        address.byte_add(52).cast::<u32>().read()
    };

    let host_run_frame_input = unsafe {
        let address = patterns
            .address_of(
                "engine_client.so",
                &pattern::HOST_RUN_FRAME_INPUT,
                "host_run_frame_input",
            )
            .expect("host run frame input");

        address
    };

    let cl_move = unsafe {
        let cl_move = patterns
            .address_of("engine_client.so", &pattern::CL_MOVE, "cl_move")
            .expect("cl move");

        let cl_move: elysium_state::ClMoveFn = core::mem::transmute(cl_move);

        elysium_state::set_cl_move(cl_move);

        cl_move
    };

    let write_user_command = unsafe {
        let write_user_command = patterns
            .address_of(
                "client_client.so",
                &pattern::WRITE_USER_COMMAND,
                "write_user_command",
            )
            .expect("write user command");

        let write_user_command: elysium_state::WriteUserCommandFn =
            core::mem::transmute(write_user_command);

        elysium_state::set_write_user_command(write_user_command);

        write_user_command
    };

    unsafe {
        let gl_context = elysium_gl::Context::new(|symbol| gl.get_proc_address(symbol).cast());
        let swap_window = swap_window as *mut state::SwapWindowFn;
        let poll_event = poll_event as *mut state::PollEventFn;

        state::set_gl(gl);
        state::set_sdl(sdl);

        state::set_gl_context(gl_context);

        state::set_swap_window(swap_window.replace(hooks2::swap_window));

        println!("elysium | hooked \x1b[38;5;2mSDL_GL_SwapWindow\x1b[m");

        state::set_poll_event(poll_event.replace(hooks2::poll_event));

        println!("elysium | hooked \x1b[38;5;2mSDL_PollEvent\x1b[m");

        // e8 <relative>  call  CL_Move
        // 0x005929d3 - 0x00592910 = 195
        {
            let cl_move_hook = crate::hooks2::cl_move as usize as *const u8;

            println!(
                "cl_move_hook = {:02X?}",
                cl_move_hook.cast::<[u8; 7]>().read()
            );

            let call_cl_move = host_run_frame_input.byte_offset(195);

            println!(
                "call cl_move (host_run_frame_input + 195) = {:02X?}",
                call_cl_move.cast::<[u8; 5]>().read()
            );

            // obtain rip
            let rip = call_cl_move.byte_offset(5);

            // calulate relative
            let relative = cl_move_hook.byte_offset_from(rip);

            println!("cl_move_hook relative = {relative:?}");

            // remove protection
            let protection = elysium_mem::unprotect(call_cl_move);

            // replace relative
            let original = call_cl_move
                .byte_offset(1)
                .cast::<i32>()
                .as_mut()
                .replace(relative as i32);

            println!("cl_move_hook relative (original) = {original:?}");

            // restore protection
            elysium_mem::protect(call_cl_move, protection);

            println!(
                "call cl_move (host_run_frame_input + 195) (new) = {:02X?}",
                call_cl_move.cast::<[u8; 5]>().read()
            );
        }
    }
}
