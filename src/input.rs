use providence_math::Vec3;
use sdk::Pad;

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
    _pad0: Pad<16>,
    pub is_track_ir_available: bool,
    pub is_mouse_initialized: bool,
    pub is_mouse_active: bool,
    _pad1: Pad<162>,
    pub thirdperson: bool,
    pub camera_moving_with_mouse: bool,
    pub offset: Vec3,
}
