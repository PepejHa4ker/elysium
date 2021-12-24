use sdk::{Pad, Vector};

pub use self::button::Button;
pub use self::joystick::Joystick;
pub use self::mouse::Mouse;
pub use self::state::State;

mod button;
mod joystick;
mod mouse;
mod state;

#[derive(Debug)]
#[repr(C)]
pub struct Input {
    _pad0: Pad<201>,
    pub thirdperson: bool,
    _pad1: Pad<1>,
    pub offset: Vector,
    _pad2: Pad<18>,
}

impl Input {
    pub unsafe fn from_raw(ptr: *const ()) -> &'static Self {
        &*(ptr as *const Self)
    }
}
