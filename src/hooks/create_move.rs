use crate::{globals, sdk};
use std::lazy::SyncOnceCell;
use std::mem;

pub type Signature =
    extern "C" fn(this: *const usize, input_sample_time: f32, command: &mut sdk::Command) -> bool;

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original() -> Signature {
    *ORIGINAL.get().unwrap_unchecked()
}

pub fn set_original(original: *const usize) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe extern "C" fn hook(
    this: *const usize,
    input_sample_time: f32,
    command: &mut sdk::Command,
) -> bool {
    let original = original();

    original(this, input_sample_time, command);

    if command.tick_count != 0 {
        globals::console().write(format!("command = {:?}\n", &command));

        let local_player = globals::local_player();

        if let Some(local_player) = local_player {
            let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

            if flags.on_ground() {
                command.state.0 = command.state.0 | sdk::input::State::JUMP.0;
            } else {
                command.state.0 = command.state.0 & !sdk::input::State::JUMP.0;
            }
        }
    }

    return true;
}
