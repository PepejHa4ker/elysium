// https://github.com/HackerPolice/MissedIT/blob/master/src/SDK/INetChannel.h

use crate::{ffi, object_validate, vtable_export, vtable_validate, Pad};
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
    get_remote_frame_rate: unsafe extern "C" fn(
        this: *const NetworkChannel,
        frame_time: *mut f32,
        frame_time_standard_deviation: *mut f32,
    ),
    get_timeout_seconds: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    get_name: unsafe extern "C" fn(this: *const NetworkChannel) -> *const u8,
}

vtable_validate! {
    get_address => 0,
    get_time => 1,
    get_time_connected => 2,
    get_buffer_len => 3,
    get_data_rate => 4,
    is_loopback => 5,
    is_timing_out => 6,
    is_playback => 7,
    get_latency => 8,
    get_avg_latency => 9,
    get_avg_loss => 10,
    get_avg_choke => 11,
    get_avg_data => 12,
    get_avg_packets => 13,
    get_total_data => 14,
    get_sequence_number => 15,
    is_valid_packet => 16,
    get_packet_time => 17,
    get_packet_bytes => 18,
    get_stream_progress => 19,
    get_time_since_last_received => 20,
    get_command_interpolation_amount => 21,
    get_packet_response_latency => 22,
    get_remote_frame_rate => 23,
    get_timeout_seconds => 24,
    get_name => 25,
}

/// a network channel
#[repr(C)]
pub struct NetworkChannel {
    vtable: &'static VTable,
    _pad0: Pad<36>,
    pub choked_packets: i32,
}

object_validate! {
    NetworkChannel;
    choked_packets => 44,
}

/// network channel flow
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Flow {
    Outgoing = 0,
    Incoming = 1,
    Both = 2,
}

impl NetworkChannel {
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

        /// time when packet was sent
        get_packet_time(flow: Flow, frame_number: i32) -> f32,

        /// group size of this packet
        get_packet_bytes(flow: Flow, frame_number: i32, group: i32) -> i32,

        /// get time since last recieved packet (in seconds)
        get_time_since_last_received() -> f32,

        /// ???
        get_command_interpolation_amount(flow: Flow, frame_number: i32) -> f32,

        /// ???
        get_packet_response_latency(
            flow: Flow,
            frame_number: i32,
            latency: &mut i32,
            choke: &mut i32
        ) -> (),

        /// ???
        get_timeout_seconds() -> f32,
    }

    /// get channel ip address as a string
    #[inline]
    pub fn get_address(&self) -> &str {
        unsafe {
            let ptr = (self.vtable.get_address)(self);

            ffi::str_from_ptr(ptr)
        }
    }

    /// tcp progress if transmitting
    #[inline]
    pub fn get_stream_progress(&self, flow: Flow) -> Option<(i32, i32)> {
        let mut received = MaybeUninit::uninit();
        let mut total = MaybeUninit::uninit();

        unsafe {
            let in_progress = (self.vtable.get_stream_progress)(
                self,
                flow,
                received.as_mut_ptr(),
                total.as_mut_ptr(),
            );

            in_progress.then(|| (received.assume_init(), total.assume_init()))
        }
    }

    /// get channel name
    #[inline]
    pub fn get_remote_frame_rate(&self) -> (f32, f32) {
        let mut frame_time = MaybeUninit::uninit();
        let mut frame_time_standard_deviation = MaybeUninit::uninit();

        unsafe {
            (self.vtable.get_remote_frame_rate)(
                self,
                frame_time.as_mut_ptr(),
                frame_time_standard_deviation.as_mut_ptr(),
            );

            (
                frame_time.assume_init(),
                frame_time_standard_deviation.assume_init(),
            )
        }
    }

    /// get channel name
    #[inline]
    pub fn get_name(&self) -> &str {
        unsafe {
            let ptr = (self.vtable.get_name)(self);

            ffi::str_from_ptr(ptr)
        }
    }
}
