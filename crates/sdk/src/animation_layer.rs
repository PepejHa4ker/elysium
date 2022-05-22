use crate::Pad;
use core::fmt;

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
    _pad1: Pad<16>,
}

impl fmt::Debug for AnimationLayer {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AnimationLayer")
            .field("order", &self.order)
            .field("sequence", &self.sequence)
            .field("previous_cycle", &self.previous_cycle)
            .field("weight", &self.weight)
            .field("weight_delta_rate", &self.weight_delta_rate)
            .field("playback_rate", &self.playback_rate)
            .field("cycle", &self.cycle)
            .finish()
    }
}
