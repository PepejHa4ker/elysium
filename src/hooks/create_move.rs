use crate::command::Command;

pub type Signature =
    unsafe extern "C" fn(this: *const (), input_sample_time: f32, command: &mut Command) -> bool;

pub const IN_ATTACK: i32 = 1 << 0;
pub const IN_JUMP: i32 = 1 << 1;
pub const IN_DUCK: i32 = 1 << 2;
pub const IN_BULLRUSH: i32 = 1 << 22;

pub unsafe extern "C" fn hook(
    this: *const (),
    input_sample_time: f32,
    command: &mut Command,
) -> bool {
    return false;
}
