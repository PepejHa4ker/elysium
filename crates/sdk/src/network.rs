use super::Pad;
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<9>,
    get_latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: i32) -> f32,
}

#[repr(C)]
pub struct NetworkChannel {
    vtable: &'static VTable,
    _pad0: Pad<36>,
    pub choked_packets: i32,
}

impl NetworkChannel {
    #[inline]
    pub fn get_latency(&self, flow: i32) -> f32 {
        unsafe { (self.vtable.get_latency)(self, flow) }
    }
}
