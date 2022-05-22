use super::Pad;
use elysium_math::Vec3;

#[repr(C)]
pub struct ActiveChannels {
    pub count: i32,
    pub list: [u16; 128],
}

#[repr(C)]
pub struct Channel {
    _pad0: Pad<260>,
    pub sound_source: i32,
    _pad1: Pad<56>,
    pub origin: Vec3,
    pub direction: Vec3,
    _pad2: Pad<80>,
}
