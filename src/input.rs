use sdk::{Vector, Vector2D};
use std::fmt;

pub use self::button::Button;
pub use self::joystick::Joystick;
pub use self::mouse::Mouse;
pub use self::state::State;

mod button;
mod joystick;
mod mouse;
mod state;

#[repr(C)]
pub struct Input {
    _pad0: [u8; 0xB4],
    pub intercepting_mouse: bool,
    pub thirdperson: bool,
    pub moving_with_mouse: bool,
    pub offset: Vector,
    pub distance: Vector,
    pub old_pos: Vector2D,
    pub pos: Vector2D,
    pub is_orthographic: bool,
}

impl Input {
    pub unsafe fn from_raw(ptr: *const ()) -> &'static Self {
        &*(ptr as *const Self)
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Input")
            .field("intercepting_mouse", &self.intercepting_mouse)
            .field("thirdperson", &self.thirdperson)
            .field("moving_with_mouse", &self.moving_with_mouse)
            .field("offset", &self.offset)
            .field("distance", &self.distance)
            .field("old_pos", &self.old_pos)
            .field("pos", &self.pos)
            .field("is_orthographic", &self.is_orthographic)
            .finish()
    }
}
