//! game hooks

use crate::{SharedOption, STATE};

macro_rules! hooks {
    ($(
        ($ident:ident, $set_ident:ident, $signature:ident)($($arg:ident: $argty:ty),*) -> $output:ty
    ),*) => {
        $(
            #[doc = "`"]
            #[doc = stringify!($ident)]
            #[doc = "` signature."]
            pub type $signature = unsafe extern "C" fn($($arg: $argty),*) -> $output;
        )*

        pub(crate) struct Hooks {
            $(
                $ident: SharedOption<$signature>,
            )*
        }

        impl Hooks {
            pub const fn new() -> Self {
                Self {
                    $(
                        $ident: SharedOption::none(),
                    )*
                }
            }
        }

        $(
            #[doc = "Call the original `"]
            #[doc = stringify!($ident)]
            #[doc = "`"]
            pub fn $ident($($arg: $argty),*) -> $output {
                unsafe { (*STATE.hooks.$ident.as_mut())($($arg),*) }
            }

            #[doc = "Set the original `"]
            #[doc = stringify!($ident)]
            #[doc = "`"]
            pub fn $set_ident($ident: $signature) {
                unsafe { STATE.hooks.$ident.write($ident); }
            }
        )*
    }
}

hooks! {
    (create_move, set_create_move, CreateMove)(this: *const (), sample_time: f32, command: *const ()) -> bool,
    (cl_move, set_cl_move, ClMove)(accumulated_extra_samples: f32, final_tick: bool) -> (),
    (cl_send_move, set_cl_send_move, ClSendMove)() -> (),
    (frame_stage_notify, set_frame_stage_notify, FrameStageNotify)(this: *const (), frame: i32) -> (),
    (write_user_command, set_write_user_command, WriteUserCommand)(buffer: *mut u8, from: *const u8, to: *const u8) -> bool,
    (swap_window, set_swap_window, SwapWindow)(sdl_window: *mut sdl2_sys::SDL_Window) -> (),
    (poll_event, set_poll_event, PollEvent)(sdl_event: *mut sdl2_sys::SDL_Event) -> i32
}
