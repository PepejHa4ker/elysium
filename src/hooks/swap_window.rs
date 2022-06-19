use crate::state;
use core::mem::MaybeUninit;
use iced_elysium_gl::Viewport;
use iced_native::Size;

/// `SDL_GL_SwapWindow` hook.
pub unsafe extern "C" fn swap_window(sdl_window: *mut sdl2_sys::SDL_Window) {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();

    sdl2_sys::SDL_GetWindowSize(sdl_window, width.as_mut_ptr(), height.as_mut_ptr());

    let width = width.assume_init();
    let height = height.assume_init();
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

    if state::is_menu_open() {
        context.viewport(0, 0, size.width as i32, size.height as i32);

        menu.update(viewport.clone(), state::cursor_position());
        menu.draw(context, viewport);
    }

    // disable auto-conversion from/to sRGB
    context.disable(elysium_gl::FRAMEBUFFER_SRGB);

    // disable alpha blending to not break vgui fonts
    context.disable(elysium_gl::BLEND);

    state::hooks::swap_window(sdl_window);
}
