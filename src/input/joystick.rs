use std::ops::BitAnd;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Joystick(u32);

impl Joystick {
    pub const AXIS_X: Self = Self::new(0);
    pub const AXIS_Y: Self = Self::new(1);
    pub const AXIS_Z: Self = Self::new(2);
    pub const AXIS_R: Self = Self::new(3);
    pub const AXIS_U: Self = Self::new(4);
    pub const AXIS_V: Self = Self::new(5);

    pub const MAX_AXES: u32 = 6;
    pub const MAX_JOYSTICKS: u32 = 4;

    pub const MAX_COUNT: u32 = 32;
    pub const POV_COUNT: u32 = 4;
    pub const AXIS_COUNT: u32 = 12;

    const fn new(state: u32) -> Self {
        Self(state)
    }
}
