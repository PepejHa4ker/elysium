use crate::state;
use core::mem::MaybeUninit;
use elysium_sdk::{Command, Input};

/// `WriteUserCommandDeltaToBuffer` hook.
pub unsafe extern "C" fn write_user_command_delta_to_buffer(
    _this: *const u8,
    slot: i32,
    buffer: *mut u8,
    from: i32,
    to: i32,
    _new_command: u8,
) -> bool {
    let mut zero_command = MaybeUninit::<Command>::zeroed();
    let zero_command = zero_command.as_mut_ptr();
    let input = &*state::input().cast::<Input>();

    let from_command = if from == -1 {
        zero_command
    } else {
        let from_command = input.get_user_command(slot, from).as_mut();

        if from_command.is_null() {
            zero_command
        } else {
            from_command
        }
    };

    let to_command = input.get_user_command(slot, to).as_mut();
    let to_command = if to_command.is_null() {
        zero_command
    } else {
        to_command
    };

    let from_command = from_command.cast();
    let to_command = to_command.as_const().cast();

    state::hooks::write_user_command(buffer, to_command, from_command);

    true
}
