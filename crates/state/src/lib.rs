//! Global state.

#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(const_ptr_write)]
#![feature(const_mut_refs)]
#![feature(ptr_const_cast)]

use cache::Players;
use core::mem::ManuallyDrop;
use core::ptr;
use core::ptr::NonNull;
use elysium_math::Vec3;
use elysium_menu::Menu;
use iced_elysium_gl::Viewport;
use iced_native::{Point, Size};
use local::Local;
use material::Materials;

pub use shared::Shared;
pub use shared_box::SharedBox;
pub use shared_option::SharedOption;

mod shared;
mod shared_box;
mod shared_option;

pub mod cache;
pub mod local;
pub mod material;

/// `Client::CreateMove` signature.
pub type CreateMoveFn =
    unsafe extern "C" fn(this: *const (), sample_time: f32, command: *const ()) -> bool;

/// `SDL_GL_SwapWindow` signature.
pub type SwapWindowFn = unsafe extern "C" fn(sdl_window: *mut sdl2_sys::SDL_Window);

/// `SDL_PollEvent` signature.
pub type PollEventFn = unsafe extern "C" fn(sdl_event: *mut sdl2_sys::SDL_Event) -> i32;

struct State {
    gl_library: SharedOption<elysium_gl::Gl<'static>>,
    sdl_library: SharedOption<elysium_sdl::Sdl<'static>>,

    gl_context: Shared<elysium_gl::Context>,

    menu: SharedBox<Menu>,
    menu_open: Shared<bool>,
    cursor_position: Shared<Point>,
    window_size: Shared<Size<u32>>,

    create_move: SharedOption<CreateMoveFn>,
    swap_window: SharedOption<SwapWindowFn>,
    poll_event: SharedOption<PollEventFn>,

    materials: Materials,

    local: Local,

    players: Shared<Players>,
    prediction_time: Shared<f32>,
    send_packet: Shared<*mut bool>,
    tick_count: Shared<i32>,
    view_angle: Shared<Vec3>,

    /// type-erased reference to the game engine interface
    engine: SharedOption<NonNull<u8>>,

    /// type-erased reference to the input interface
    input: SharedOption<NonNull<u8>>,

    /// type-erased reference to the network channel
    network_channel: SharedOption<NonNull<u8>>,

    /// type-erased reference to the trace interface
    trace: SharedOption<NonNull<u8>>,
}

static STATE: ManuallyDrop<State> = ManuallyDrop::new(State {
    gl_library: SharedOption::none(),
    sdl_library: SharedOption::none(),

    gl_context: Shared::new(elysium_gl::Context::NONE),

    menu: SharedBox::none(),
    menu_open: Shared::new(false),
    cursor_position: Shared::new(Point::new(0.0, 0.0)),
    window_size: Shared::new(Size::new(0, 0)),

    create_move: SharedOption::none(),
    poll_event: SharedOption::none(),
    swap_window: SharedOption::none(),

    materials: Materials::new(),

    local: Local::new(),

    players: Shared::new(Players::new()),
    prediction_time: Shared::new(0.0),
    send_packet: Shared::new(ptr::null_mut()),
    tick_count: Shared::new(0),
    view_angle: Shared::new(Vec3::splat(0.0)),

    engine: SharedOption::none(),
    input: SharedOption::none(),
    network_channel: SharedOption::none(),
    trace: SharedOption::none(),
});

/// Returns a reference to the `libGL` loader.
#[inline]
pub unsafe fn gl() -> &'static mut elysium_gl::Gl<'static> {
    STATE.gl_library.as_mut()
}

/// Set the `libGL` loader.
#[inline]
pub fn set_gl(library: elysium_gl::Gl<'static>) {
    unsafe {
        STATE.gl_library.write(library);
    }
}

/// Returns a reference to the `libSDL` loader.
#[inline]
pub fn sdl() -> &'static mut elysium_sdl::Sdl<'static> {
    unsafe { STATE.sdl_library.as_mut() }
}

/// Set the `libSDL` loader.
#[inline]
pub fn set_sdl(library: elysium_sdl::Sdl<'static>) {
    unsafe {
        STATE.sdl_library.write(library);
    }
}

/// Returns a reference to the OpenGL profile context.
#[inline]
pub unsafe fn gl_context() -> &'static elysium_gl::Context {
    &*STATE.gl_context.as_mut()
}

/// Set the OpenGL profile context.
#[inline]
pub fn set_gl_context(context: elysium_gl::Context) {
    unsafe {
        STATE.gl_context.write(context);
    }
}

/// Returns a reference to the menu structure.
#[inline]
pub fn menu(context: &elysium_gl::Context, viewport: Viewport) -> &'static mut Menu {
    if is_menu_none() {
        frosting::println!("state::menu is None, initialising...");

        unsafe {
            STATE.menu.write(Menu::new(context, viewport));
        }

        frosting::println!("state::menu initialised");
    }

    unsafe { menu_unchecked() }
}

/// Is the menu open?
#[inline]
pub fn is_menu_open() -> bool {
    unsafe { *STATE.menu_open.as_mut() }
}

/// Toggle the open state of the menu.
#[inline]
pub fn toggle_menu() {
    unsafe {
        *STATE.menu_open.as_mut() ^= true;
    }
}

/// Returns a reference to the menu structure, without initialising it if not present.
#[inline]
pub unsafe fn menu_unchecked() -> &'static mut Menu {
    STATE.menu.as_mut()
}

/// Is the menu uninitialized?
#[inline]
pub fn is_menu_none() -> bool {
    STATE.menu.is_none()
}

/// Returns the cursor position.
#[inline]
pub fn cursor_position() -> Point {
    unsafe { *STATE.cursor_position.as_mut() }
}

/// Update the cursor position.
#[inline]
pub fn update_cursor_position(point: Point) {
    unsafe {
        STATE.cursor_position.write(point);
    }
}

/// Returns the window size.
#[inline]
pub fn window_size() -> Size<u32> {
    unsafe { *STATE.window_size.as_mut() }
}

/// Update the cursor position.
#[inline]
pub fn update_window_size(size: Size<u32>) {
    unsafe {
        STATE.window_size.write(size);
    }
}

/// Calls the original `SDL_GL_SwapWindow`.
#[inline]
pub unsafe fn swap_window(window: *mut sdl2_sys::SDL_Window) {
    let swap_window = *STATE.swap_window.as_mut();

    swap_window(window)
}

/// Set the original `SDL_GL_SwapWindow`.
#[inline]
pub fn set_swap_window(swap_window: SwapWindowFn) {
    unsafe {
        STATE.swap_window.write(swap_window);
    }
}

/// Calls the original `SDL_PollEvent`.
#[inline]
pub unsafe fn poll_event(event: *mut sdl2_sys::SDL_Event) -> i32 {
    let poll_event = *STATE.poll_event.as_mut();

    poll_event(event)
}

/// Set the original `SDL_PollEvent`.
#[inline]
pub fn set_poll_event(poll_event: PollEventFn) {
    unsafe {
        STATE.poll_event.write(poll_event);
    }
}

/// Returns a reference to the player cache.
#[inline]
pub unsafe fn players() -> &'static mut Players {
    STATE.players.as_mut()
}

/// Calls the original `Client::CreateMove`.
#[inline]
pub unsafe fn create_move(this: *const (), sample_time: f32, command: *const ()) -> bool {
    let create_move = *STATE.create_move.as_mut();

    create_move(this, sample_time, command)
}

/// Set the original `Client::CreateMove`.
#[inline]
pub fn set_create_move(create_move: CreateMoveFn) {
    unsafe {
        STATE.create_move.write(create_move);
    }
}

/// Return's a reference to engine prediction time.
#[inline]
pub unsafe fn prediction_time() -> &'static mut f32 {
    STATE.prediction_time.as_mut()
}

/// Return's a reference to send_packet.
#[inline]
pub unsafe fn send_packet() -> &'static mut *mut bool {
    STATE.send_packet.as_mut()
}

/// Return's a reference to engine tick count.
#[inline]
pub unsafe fn tick_count() -> &'static mut i32 {
    STATE.tick_count.as_mut()
}

/// Return's a reference to engine view_angle.
#[inline]
pub unsafe fn view_angle() -> &'static mut Vec3 {
    STATE.view_angle.as_mut()
}

#[inline]
pub unsafe fn engine() -> *const u8 {
    STATE.engine.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_engine(engine: *const u8) {
    STATE.engine.write(NonNull::new_unchecked(engine.as_mut()));
}

#[inline]
pub unsafe fn input() -> *const u8 {
    STATE.input.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_input(input: *const u8) {
    STATE.input.write(NonNull::new_unchecked(input.as_mut()));
}

#[inline]
pub unsafe fn network_channel() -> *const u8 {
    STATE.network_channel.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_network_channel(network_channel: *const u8) {
    STATE
        .engine
        .write(NonNull::new_unchecked(network_channel.as_mut()));
}

#[inline]
pub unsafe fn trace() -> *const u8 {
    STATE.trace.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_trace(trace: *const u8) {
    STATE.trace.write(NonNull::new_unchecked(trace.as_mut()));
}
