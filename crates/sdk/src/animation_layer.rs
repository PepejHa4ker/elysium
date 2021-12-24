use crate::Pad;

#[derive(Debug)]
#[repr(C)]
pub struct AnimationLayer {
    _pad0: Pad<24>,
    pub order: i32,
    pub sequence: i32,
    pub previous_cycle: f32,
    pub weight: f32,
    pub weight_delta_rate: f32,
    pub playback_rate: f32,
    pub cycle: f32,
    /// pointer to the owner of this animation layer
    pub owner: *const (),
    _pad1: Pad<8>,
}
