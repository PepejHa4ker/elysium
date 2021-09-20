use super::input::State;
use super::{Angle, Vector};

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    vtable: *const (),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: Angle,
    pub aim_direction: Angle,
    pub foward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub state: State,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub head_angles: Angle,
    pub head_offset: Vector,
}

impl Command {
    pub const fn in_attack(&self) -> bool {
        self.state.is_attack()
    }

    pub const fn in_attack2(&self) -> bool {
        self.state.is_attack2()
    }

    pub const fn in_attack3(&self) -> bool {
        self.state.is_attack3()
    }

    pub fn set_tick_count(&mut self, value: i32) {
        self.tick_count = value;
    }
}
