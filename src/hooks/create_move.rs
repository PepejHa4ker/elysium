use crate::command::Command;
use crate::entity::Entity;
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
    let _result = global.create_move_original(this, input_sample_time, command);

    if command.tick_count == 0 {
        return true;
    }

    let rbp: *mut *mut bool;
    core::arch::asm!("mov {}, rbp", out(reg) rbp, options(nostack));
    let send_packet = &mut *(*rbp).sub(24);
    let original_angle = command.view_angle;
    let original_forward = command.forward_move;
    let original_side = command.side_move;

    if let Some(local_player) = global.local_player() {
        //println!("old = {:?}", command);

        if global.last_command_has_been_predicted() {
            global.set_tick(local_player.tick_base());
        } else {
            global.increment_tick();
        }

        global.set_last_command_has_been_predicted(command.has_been_predicted);

        let on_move = &*global.on_move_ptr();
        let movement = on_move(Movement {
            forward_move: command.forward_move,
            side_move: command.side_move,
            up_move: command.up_move,
            view_angle: command.view_angle,
            send_packet: *send_packet,
            tick_count: command.tick_count,
            do_attack: command.in_attack(),
            do_jump: command.in_jump(),
            do_duck: command.in_duck(),
            do_fast_duck: command.in_fast_duck(),
            local_player: (local_player as *const Entity).read(),
            client_time: global.client_time(),
            prediction_time: local_player.tick_base() as f32 * global.interval_per_tick(),
            server_time: global.tick() as f32 * global.interval_per_tick(),
        });

        command.attack(movement.do_attack);
        command.jump(movement.do_jump);
        command.duck(movement.do_duck);
        command.fast_duck(movement.do_fast_duck);

        command.forward_move = movement.forward_move;
        command.side_move = movement.side_move;
        command.up_move = movement.up_move;
        command.view_angle = movement.view_angle;
        command.tick_count = movement.tick_count;

        *send_packet = movement.send_packet;
    }

    let f1 = if original_angle.yaw < 0.0 {
        360.0 + original_angle.yaw
    } else {
        original_angle.yaw
    };

    let f2 = if command.view_angle.yaw < 0.0 {
        360.0 + command.view_angle.yaw
    } else {
        command.view_angle.yaw
    };

    let mut delta_view_angle = if f2 < f1 {
        (f2 - f1).abs()
    } else {
        360.0 - (f2 - f1).abs()
    };

    delta_view_angle = 360.0 - delta_view_angle;

    let delta_radian = delta_view_angle.to_radians();
    let delta_radian_90 = (delta_view_angle + 90.0).to_radians();

    command.forward_move =
        delta_radian.cos() * original_forward + delta_radian_90.cos() * original_side;

    command.side_move =
        delta_radian.sin() * original_forward + delta_radian_90.sin() * original_side;

    //println!("new = {:?}", command);

    return false;
}
