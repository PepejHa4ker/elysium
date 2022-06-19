//! Function hooks.

pub use create_move::create_move;
pub use frame_stage_notify::frame_stage_notify;
pub use override_view::override_view;
pub use poll_event::poll_event;
pub use swap_window::swap_window;
pub use write_user_command_delta_to_buffer::write_user_command_delta_to_buffer;

mod create_move;
mod frame_stage_notify;
mod override_view;
mod poll_event;
mod swap_window;
mod write_user_command_delta_to_buffer;

/// `CL_Move` hook.
pub unsafe extern "C" fn cl_move(_accumulated_extra_samples: f32, _final_tick: bool) {}

/// `CL_SendMove` hook.
pub unsafe extern "C" fn cl_send_move(_accumulated_extra_samples: f32, _final_tick: bool) {}
