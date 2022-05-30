// https://github.com/HackerPolice/MissedIT/blob/master/src/SDK/INetChannel.h

use super::{ffi, vtable_export, Pad};
use core::mem::MaybeUninit;

#[repr(C)]
struct VTable {
    get_address: unsafe extern "C" fn(this: *const NetworkChannel) -> *const u8,
    get_time: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    get_time_connected: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    get_buffer_len: unsafe extern "C" fn(this: *const NetworkChannel) -> i32,
    get_data_rate: unsafe extern "C" fn(this: *const NetworkChannel) -> i32,
    is_loopback: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    is_timing_out: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    is_playback: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    get_latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_avg_latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_avg_loss: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_avg_choke: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_avg_data: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_avg_packets: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    get_total_data: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> i32,
    get_sequence_number: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> i32,
    is_valid_packet:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> bool,
    get_packet_time:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> f32,
    get_packet_bytes: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        frame_number: i32,
        group: i32,
    ) -> i32,
    get_stream_progress: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        recieved: *mut i32,
        total: *mut i32,
    ) -> bool,
    get_time_since_last_received: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    get_command_interpolation_amount:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> f32,
    get_packet_response_latency: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        frame_number: i32,
        latency: *mut i32,
        choke: *mut i32,
    ),
    get_remote_framerate: unsafe extern "C" fn(
        this: *const NetworkChannel,
        frame_time: *mut f32,
        frame_time_standard_deviation: *mut f32,
    ),
    get_timeout_seconds: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    get_name: unsafe extern "C" fn(this: *const NetworkChannel) -> *const u8,
}

#[allow(dead_code)]
#[allow(invalid_value)]
const VTABLE_VALIDATION: () = {
    let vtable: VTable = unsafe { MaybeUninit::uninit().assume_init() };

    if frosting::offset_of!(vtable.get_avg_latency) != 9 * 8 {
        panic!("invalid vtable.get_avg_latency offset");
    }
};

/// Network Channel.
#[repr(C)]
pub struct NetworkChannel {
    vtable: &'static VTable,
    _pad0: Pad<36>,
    pub choked_packets: i32,
}

#[allow(dead_code)]
#[allow(invalid_value)]
const OBJECT_VALIDATION: () = {
    let object: NetworkChannel = unsafe { MaybeUninit::uninit().assume_init() };

    if frosting::offset_of!(object.choked_packets) != 44 {
        panic!("invalid object.choked_packets offset");
    }
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Flow {
    Outgoing = 0,
    Incoming = 1,
    Both = 2,
}

impl NetworkChannel {
    /// get channel ip address as a string
    #[inline]
    pub fn get_address(&self) -> &str {
        unsafe {
            let ptr = (self.vtable.get_address)(self);

            ffi::str_from_ptr(ptr)
        }
    }

    vtable_export! {
        /// get current network time
        get_time() -> f32,

        /// get connection time in seconds
        get_time_connected() -> f32,

        /// get packet history size
        get_buffer_len() -> i32,

        /// outgoing data rate in bytes/second
        get_data_rate() -> i32,

        /// if loopback channel
        is_loopback() -> bool,

        /// if timing out
        is_timing_out() -> bool,

        /// if demo playback
        is_playback() -> bool,

        /// current latency (rtt), accurate but jittery
        get_latency(flow: Flow) -> f32,

        /// average latency in seconds
        get_avg_latency(flow: Flow) -> f32,

        /// average packet loss (0 to 1)
        get_avg_loss(flow: Flow) -> f32,

        /// average packet choke (0 to 1)
        get_avg_choke(flow: Flow) -> f32,

        /// data flow in bytes/second
        get_avg_data(flow: Flow) -> f32,

        /// average packets/second
        get_avg_packets(flow: Flow) -> f32,

        /// total flow in bytes
        get_total_data(flow: Flow) -> i32,

        /// last sent sequence number
        get_sequence_number(flow: Flow) -> i32,

        /// if packet was not lost/dropped/choked/flushed
        is_valid_packet(flow: Flow, frame_number: i32) -> bool,
    }
}
