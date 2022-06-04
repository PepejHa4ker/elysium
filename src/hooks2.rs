//! Function hooks.

use super::state;
use iced_elysium_gl::Viewport;
use iced_native::Size;

/// `SDL_GL_SwapWindow` hook.
#[inline(never)]
pub unsafe extern "C" fn swap_window(sdl_window: *mut sdl2_sys::SDL_Window) {
    //frosting::println!();

    let mut width = 0;
    let mut height = 0;

    sdl2_sys::SDL_GetWindowSize(sdl_window, &mut width, &mut height);

    let size = Size::new(width as u32, height as u32);

    state::update_window_size(size);

    let context = state::gl_context();

    // enable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // enable alpha blending to not break our fonts
    context.enable(elysium_gl::BLEND);
    context.blend_func(elysium_gl::SRC_ALPHA, elysium_gl::ONE_MINUS_SRC_ALPHA);

    let viewport = Viewport::with_physical_size(size, 1.0);
    let menu = state::menu(context, viewport.clone());

    //if state::is_menu_open() {
    context.viewport(0, 0, size.width as i32, size.height as i32);

    menu.update(viewport.clone(), state::cursor_position());
    menu.draw(context, viewport);
    //}

    // disable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // disable alpha blending to not break vgui fonts
    context.disable(elysium_gl::BLEND);

    state::swap_window(sdl_window);
}

/// `SDL_PollEvent` hook.
#[inline(never)]
pub unsafe extern "C" fn poll_event(sdl_event: *mut sdl2_sys::SDL_Event) -> i32 {
    //frosting::println!();

    let result = state::poll_event(sdl_event);

    if !state::is_menu_none() {
        let menu = state::menu_unchecked();

        elysium_input::map_event(*sdl_event, |event| {
            use iced_native::{keyboard, mouse, Event};

            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Insert,
                    ..
                }) => state::toggle_menu(),
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    // block input to the game when the menu is open
    if state::is_menu_open() {
        (*sdl_event).type_ = 0;
    }

    result
}

use crate::entity::Player;
use core::ptr;
use elysium_sdk::Flow;

/// `CL_Move` hook.
#[inline(never)]
pub unsafe extern "C" fn cl_move(accumulated_extra_samples: f32, final_tick: bool) {
    let engine = &*state::engine().cast::<elysium_sdk::Engine>();

    if !engine.is_connected() {
        return;
    }

    if !state::local::is_player_none() {
        let network_channel = &*engine.get_network_channel();
        let choked_packets = network_channel.choked_packets;
        let level_name = engine.get_level_name();
        let view_angle = engine.get_view_angle();

        let address = network_channel.get_address();
        // segfaults btw
        // let name = network_channel.get_name();
        let avg_outgoing = network_channel.get_latency(Flow::Outgoing);
        let avg_incoming = network_channel.get_latency(Flow::Incoming);

        println!("level_name = {level_name:?}");
        println!("address = {address:?}");
        // println!("name = {name:?}");
        println!("avg_outgoing = {avg_outgoing:?}");
        println!("avg_incoming = {avg_incoming:?}");
        println!("choked_packets = {choked_packets:?}");

        *state::view_angle() = engine.get_view_angle();

        let cached_players = &mut *elysium_state::players();
        let local =
            Player::new(core::mem::transmute(elysium_state::local::player())).expect("player");

        let index = local.index();
        let bones = &mut cached_players[index as usize].bones;
        let mut local_player_bones = elysium_state::local::bones();

        ptr::copy_nonoverlapping(
            bones.as_ptr(),
            local_player_bones.as_mut_ptr(),
            providence_model::MAX_BONES,
        );
    }
}
