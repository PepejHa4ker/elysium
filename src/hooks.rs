pub struct Hooks {
    pub client_cmd: *const usize,
    pub create_move: *const usize,
    pub draw_model_execute: *const usize,
    pub emit_sound: *const usize,
    pub frame_stage_notify: *const usize,
    pub get_viewmodel_fov: *const usize,
    pub is_hltv: *const usize,
    pub paint: *const usize,
    pub override_view: *const usize,
    pub should_draw_crosshair: *const usize,
}

pub struct SdlHooks {
    pub swap_window: fn(sdl_window: *const usize),
    pub swap_window_address: *const usize,
    pub poll_event: fn(sdl_event: *const usize) -> libc::c_int,
    pub poll_event_address: *const usize,
}
