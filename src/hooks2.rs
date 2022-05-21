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

    context.viewport(0, 0, size.width as i32, size.height as i32);

    menu.update(viewport.clone(), state::cursor_position());
    menu.draw(context, viewport);

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
            use iced_native::mouse;
            use iced_native::Event;

            match &event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    //(*sdl_event).type_ = 0;

    result
}
