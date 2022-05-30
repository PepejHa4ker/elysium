use super::Pad;
use core::mem::MaybeUninit;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<9>,
    get_latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
}

#[allow(dead_code)]
#[allow(invalid_value)]
const VTABLE_VALIDATION: () = {
    let vtable: VTable = unsafe { MaybeUninit::uninit().assume_init() };

    if frosting::offset_of!(vtable.get_latency) != 9 * 8 {
        panic!("invalid vtable.get_latency offset");
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
    #[inline]
    pub fn get_latency(&self, flow: Flow) -> f32 {
        unsafe { (self.vtable.get_latency)(self, flow) }
    }
}
