use sdk::{Angle, Vector};

#[derive(Debug)]
#[repr(C)]
pub struct Command {
    vtable: *const (),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angle: Angle,
    pub aim_direction: Angle,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub state: i32,
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
