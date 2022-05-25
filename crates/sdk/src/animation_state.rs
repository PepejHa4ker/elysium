use crate::pad::Pad;
use core::fmt;

/// Animation state.
#[repr(C)]
pub struct AnimationState {
    _pad0: Pad<164>,
    pub duck_amount: f32,
    _pad1: Pad<80>,
    pub foot_speed: f32,
    pub foot_speed2: f32,
    _pad2: Pad<22>,
    pub stop_to_full_running_fraction: f32,
    _pad3: Pad<532>,
    // p sure wrong offset
    pub velocity_subtract_y: f32,
}

impl fmt::Debug for AnimationState {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AnimationState")
            .field("duck_amount", &self.duck_amount)
            .field("foot_speed", &self.foot_speed)
            .field("foot_speed2", &self.foot_speed2)
            .field(
                "stop_to_full_running_fraction",
                &self.stop_to_full_running_fraction,
            )
            .field("velocity_subtract_y", &self.velocity_subtract_y)
            .finish()
    }
}
