#[derive(Debug)]
#[repr(C)]
pub struct Globals {
    pub real_time: f32,
    pub frame_count: i32,
    pub absolute_frame_time: f32,
    pub absolute_frame_start_time_standard: f32,
    pub current_time: f32,
    pub frame_time: f32,
    pub max_clients: i32,
    pub tick_count: i32,
    pub interval_per_tick: i32,
    pub interpolation_amount: i32,
    pub simulation_ticks_this_frame: i32,
    pub network_protocol: i32,
    pub save_data: *const (),
    pub is_client: bool,
    pub is_remote_client: bool,
}

impl Globals {
    pub unsafe fn from_raw(ptr: *const ()) -> &'static Self {
        &*(ptr as *const Self)
    }
}
