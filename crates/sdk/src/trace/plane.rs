use crate::Pad;
use elysium_math::Vec3;

/// Extra information about the trace.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
    pub kind: u8,
    pub sign_bits: u8,
    _pad0: Pad<2>,
}
