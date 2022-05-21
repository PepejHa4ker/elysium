use providence_math::Vec3;
use sdk2::Pad;

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

impl Plane {
    pub(crate) fn new() -> Self {
        Self {
            normal: Vec3::zero(),
            distance: 0.0,
            kind: 0,
            sign_bits: 0,
            _pad0: Pad::uninit(),
        }
    }
}
