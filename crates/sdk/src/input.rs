//! Input interace.

use crate::Pad;
use elysium_math::Vec3;
use frosting::ffi::vtable;

pub use button::Button;
pub use joystick::Joystick;
pub use mouse::Mouse;
pub use state::State;

mod button;
mod joystick;
mod mouse;
mod state;

pub const IN_ATTACK: i32 = 1 << 0;
pub const IN_JUMP: i32 = 1 << 1;
pub const IN_DUCK: i32 = 1 << 2;
pub const IN_BULLRUSH: i32 = 1 << 22;
pub const IN_LEFT: i32 = 1 << 9;
pub const IN_RIGHT: i32 = 1 << 10;

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    pub vtable: *const (),
    pub command: i32,
    pub tick_count: i32,
    pub view: Vec3,
    pub aim_direction: Vec3,
    pub movement: Vec3,
    pub state: i32,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Vec3,
    pub head_offset: Vec3,
}

impl Command {
    const fn has(&self, flag: i32) -> bool {
        (self.state & flag) != 0
    }

    pub const fn set(&mut self, flag: i32, value: bool) {
        if value {
            self.state |= flag;
        } else {
            self.state &= !flag;
        }
    }

    pub const fn in_left(&self) -> bool {
        self.has(IN_LEFT)
    }

    pub const fn in_right(&self) -> bool {
        self.has(IN_RIGHT)
    }

    pub const fn in_attack(&self) -> bool {
        self.has(IN_ATTACK)
    }

    pub const fn in_jump(&self) -> bool {
        self.has(IN_JUMP)
    }

    pub const fn in_duck(&self) -> bool {
        self.has(IN_DUCK)
    }

    pub const fn in_fast_duck(&self) -> bool {
        self.has(IN_BULLRUSH)
    }

    pub const fn attack(&mut self, value: bool) {
        self.set(IN_ATTACK, value)
    }

    pub const fn jump(&mut self, value: bool) {
        self.set(IN_JUMP, value)
    }

    pub const fn duck(&mut self, value: bool) {
        self.set(IN_DUCK, value)
    }

    pub const fn fast_duck(&mut self, value: bool) {
        self.set(IN_BULLRUSH, value);
    }
}

#[derive(Debug)]
#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<8>,
    get_user_command:
        unsafe extern "C" fn(this: *const Input, slot: i32, sequence: i32) -> *const Command,
}

#[derive(Debug)]
#[repr(C)]
pub struct Input {
    vtable: &'static VTable,
    _pad0: Pad<8>,
    pub is_track_ir_available: bool,
    pub is_mouse_initialized: bool,
    pub is_mouse_active: bool,
    _pad1: Pad<162>,
    pub thirdperson: bool,
    pub camera_moving_with_mouse: bool,
    pub offset: Vec3,
}

impl Input {
    #[inline]
    pub fn get_user_command(&self, slot: i32, sequence: i32) -> *const Command {
        unsafe { (self.vtable.get_user_command)(self, slot, sequence) }
    }
}
