use self::controls::Controls;
use self::scene::Scene;
use crate::{globals, sdk};
use glow;
use glow::*;
use iced_glow::{Backend, Renderer, Settings, Viewport};
use iced_glutin::glutin;
use iced_glutin::glutin::event::{Event, WindowEvent};
use iced_glutin::glutin::event_loop::ControlFlow;
use iced_glutin::{program, Clipboard, Debug, Size};
use iced_winit::conversion;
use iced_winit::winit;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use sdl2_sys::{SDL_SysWMinfo, SDL_Window, SDL_bool};
use std::lazy::SyncOnceCell;
use std::mem;
use std::panic;
use winit::{dpi::PhysicalPosition, event::ModifiersState};

mod controls;
mod scene;

pub type Signature = unsafe extern "C" fn(context: *mut ());

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original_unchecked(context: *mut ()) {
    let original = *ORIGINAL.get().unwrap_unchecked();

    original(context);
}

pub fn set_original(original: *const ()) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe fn do_shit(handle: RawWindowHandle) {
    let handle = match handle {
        RawWindowHandle::Xlib(handle) => handle,
        _ => return,
    };

    // segfault
    // winit::window::Window::from_raw_window_handle(handle);

    use std::sync::Arc;

    let arc = Arc::new(handle);

    globals::console().write(format!("{:?}\n", &arc));

    return;

    let (gl, event_loop, windowed_context, shader_version) = {
        let el = glutin::event_loop::EventLoop::new();

        let wb = glutin::window::WindowBuilder::new()
            .with_title("OpenGL integration example")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();

        unsafe {
            let windowed_context = windowed_context.make_current().unwrap();

            let gl = glow::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            });

            // Enable auto-conversion from/to sRGB
            gl.enable(glow::FRAMEBUFFER_SRGB);

            // Enable alpha blending
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            // Disable multisampling by default
            gl.disable(glow::MULTISAMPLE);

            (gl, el, windowed_context, "#version 100")
        }
    };

    let physical_size = windowed_context.window().inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        windowed_context.window().scale_factor(),
    );

    let mut cursor_position = PhysicalPosition::new(-1.0, -1.0);
    let mut modifiers = ModifiersState::default();
    let mut clipboard = Clipboard::connect(&windowed_context.window());

    let mut renderer = Renderer::new(Backend::new(&gl, Settings::default()));

    let mut debug = Debug::new();

    let controls = Controls::new();
    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        conversion::cursor_position(cursor_position, viewport.scale_factor()),
        &mut renderer,
        &mut debug,
    );

    let mut resized = false;
}

pub unsafe extern "C" fn hook(context: *mut ()) {
    globals::console().write("swap_window\n");

    let window = Window {
        window: context as *mut SDL_Window,
    };

    globals::console().write(format!("{:?}\n", &window));

    let handle = window.raw_window_handle();

    globals::console().write(format!("{:?}\n", &handle));

    let result = panic::catch_unwind(|| {
        do_shit(handle);
    });

    globals::console().write(format!("{:?}\n", result));

    original_unchecked(context);
}

#[derive(Debug)]
pub struct Window {
    pub window: *mut SDL_Window,
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        use sdl2_sys::SDL_SYSWM_TYPE::*;

        let mut wm_info: SDL_SysWMinfo = unsafe { std::mem::zeroed() };

        // Make certain to retrieve version before querying `SDL_GetWindowWMInfo`
        // as that gives an error on certain systems
        unsafe {
            sdl2_sys::SDL_GetVersion(&mut wm_info.version);

            if sdl2_sys::SDL_GetWindowWMInfo(self.window, &mut wm_info) == SDL_bool::SDL_FALSE {
                tracing::error!("Couldn't get SDL window info: {}", sdl2::get_error());

                panic!("fuck");
            }
        }

        match wm_info.subsystem {
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_WAYLAND => {
                use raw_window_handle::unix::WaylandHandle;

                RawWindowHandle::Wayland(WaylandHandle {
                    surface: unsafe { wm_info.info.wl }.surface as *mut libc::c_void,
                    display: unsafe { wm_info.info.wl }.display as *mut libc::c_void,
                    ..WaylandHandle::empty()
                })
            }

            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            SDL_SYSWM_X11 => {
                use raw_window_handle::unix::XlibHandle;

                RawWindowHandle::Xlib(XlibHandle {
                    window: unsafe { wm_info.info.x11 }.window,
                    display: unsafe { wm_info.info.x11 }.display as *mut libc::c_void,
                    ..XlibHandle::empty()
                })
            }
            _ => {
                tracing::error!("Unknown SDL subsystem");

                panic!("fuck");
            }
        }
    }
}
