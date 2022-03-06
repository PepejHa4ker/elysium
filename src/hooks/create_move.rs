use crate::command::Command;
use crate::entity::Player;
use crate::global::Global;
use crate::movement::Movement;

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
    let global = Global::handle();
    let original_result = global.create_move_original(this, input_sample_time, command);

    if command.command_number == 0 || command.tick_count == 0 {
        return original_result;
    }

    let rbp: *mut *mut bool;

    core::arch::asm!("mov {}, rbp", out(reg) rbp, options(nostack));

    let send_packet = &mut *(*rbp).sub(24);

    if let Some(local_player) = global.local_player() {
        if global.last_command_has_been_predicted() {
            global.set_tick(local_player.tick_base());
        } else {
            global.increment_tick();
        }

        global.set_last_command_has_been_predicted(command.has_been_predicted);

        let on_move = &*global.on_move_ptr();
        let movement = on_move(Movement {
            vectors: command.vectors,
            view: command.view,

            send_packet: *send_packet,

            tick_count: command.tick_count,
            command_number: command.command_number,

            do_attack: command.in_attack(),
            do_jump: command.in_jump(),
            do_duck: command.in_duck(),
            do_fast_duck: command.in_fast_duck(),
            do_left: command.in_left(),
            do_right: command.in_right(),

            local_player: (local_player as *const Player).read(),

            client_time: global.client_time(),
            prediction_time: local_player.tick_base() as f32 * global.interval_per_tick(),
            server_time: global.tick() as f32 * global.interval_per_tick(),
        });

        command.attack(movement.do_attack);
        command.jump(movement.do_jump);
        command.duck(movement.do_duck);
        command.fast_duck(movement.do_fast_duck);

        command.vectors = movement.vectors;
        command.view = movement.view;

        command.command_number = movement.command_number;
        command.tick_count = movement.tick_count;

        *send_packet = movement.send_packet;
    } else {
        println!("create_move called while local_player is null");
    }

    return false;
}
