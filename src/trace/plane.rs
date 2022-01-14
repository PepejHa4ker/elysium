use sdk::{Pad, Vector};

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Plane {
    pub normal: Vector,
    pub distance: f32,
    pub kind: u8,
    pub sign_bits: u8,
    _pad0: Pad<2>,
}

impl Plane {
    pub(crate) fn new() -> Self {
        Self {
            normal: Vector::zero(),
            distance: 0.0,
            kind: 0,
            sign_bits: 0,
            _pad0: Pad::uninit(),
        }
    }
}