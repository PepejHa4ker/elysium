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
use hooks::Hooks;
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
pub mod hooks;
pub mod local;
pub mod material;

struct State {
    gl_library: SharedOption<elysium_gl::Gl>,
    sdl_library: SharedOption<elysium_sdl::Sdl>,

    gl_context: Shared<elysium_gl::Context>,

    menu: SharedBox<Menu>,
    menu_open: Shared<bool>,
    cursor_position: Shared<Point>,
    window_size: Shared<Size<u32>>,

    hooks: Hooks,

    materials: Materials,

    networked: Shared<[u8; 248]>,

    local: Local,

    players: Shared<Players>,
    prediction_time: Shared<f32>,
    send_packet: Shared<*mut bool>,
    tick_count: Shared<i32>,
    view_angle: Shared<Vec3>,

    /// type-erased reference to the game engine interface
    engine: SharedOption<NonNull<u8>>,

    /// type-erased reference to the game entity list interface
    entity_list: SharedOption<NonNull<u8>>,

    /// type-erased reference to the games globals
    globals: SharedOption<NonNull<u8>>,

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

    hooks: Hooks::new(),

    materials: Materials::new(),

    networked: Shared::new([0; 248]),

    local: Local::new(),

    players: Shared::new(Players::new()),
    prediction_time: Shared::new(0.0),
    send_packet: Shared::new(ptr::null_mut()),
    tick_count: Shared::new(0),
    view_angle: Shared::new(Vec3::splat(0.0)),

    engine: SharedOption::none(),
    entity_list: SharedOption::none(),
    globals: SharedOption::none(),
    input: SharedOption::none(),
    network_channel: SharedOption::none(),
    trace: SharedOption::none(),
});

/// Returns a reference to the `libGL` loader.
#[inline]
pub unsafe fn gl() -> &'static mut elysium_gl::Gl {
    STATE.gl_library.as_mut()
}

/// Set the `libGL` loader.
#[inline]
pub fn set_gl(library: elysium_gl::Gl) {
    unsafe {
        STATE.gl_library.write(library);
    }
}

/// Returns a reference to the `libSDL` loader.
#[inline]
pub fn sdl() -> &'static mut elysium_sdl::Sdl {
    unsafe { STATE.sdl_library.as_mut() }
}

/// Set the `libSDL` loader.
#[inline]
pub fn set_sdl(library: elysium_sdl::Sdl) {
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
        unsafe {
            STATE.menu.write(Menu::new(context, viewport));
        }

        println!("elysium | menu has been initialised");
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

/// Returns a reference to the player cache.
#[inline]
pub unsafe fn players() -> &'static mut Players {
    STATE.players.as_mut()
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
pub fn view_angle() -> &'static mut Vec3 {
    unsafe { STATE.view_angle.as_mut() }
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
pub unsafe fn entity_list() -> *const u8 {
    STATE.entity_list.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_entity_list(entity_list: *const u8) {
    STATE
        .entity_list
        .write(NonNull::new_unchecked(entity_list.as_mut()));
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
pub unsafe fn globals() -> *const u8 {
    STATE.globals.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_globals(globals: *const u8) {
    STATE
        .globals
        .write(NonNull::new_unchecked(globals.as_mut()));
}

#[inline]
pub unsafe fn trace() -> *const u8 {
    STATE.trace.as_mut().as_ptr()
}

#[inline]
pub unsafe fn set_trace(trace: *const u8) {
    STATE.trace.write(NonNull::new_unchecked(trace.as_mut()));
}

#[inline]
pub unsafe fn networked() -> *const [u8; 248] {
    STATE.networked.as_mut()
}

#[inline]
pub unsafe fn set_networked(networked: [u8; 248]) {
    STATE.networked.write(networked);
}
