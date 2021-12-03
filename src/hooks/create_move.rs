use crate::command::Command;
use crate::entity::Entity;
use crate::global::Global;
use crate::intrinsics;
use crate::movement::Movement;
use sdk::F32Ext;

pub type Signature =
    unsafe extern "C" fn(this: *const (), input_sample_time: f32, command: &mut Command) -> bool;

/*pub fn bunny_hop(command: &mut Command, send_packet: &mut bool, local_player: &Entity) {
    let flags = local_player.flags();

    if !flags.on_ground() {
        command.state &= !(1 << 1);
    }
}

pub fn directional_strafe(command: &mut Command, send_packet: &mut bool, local_player: &Entity) {
    let flags = local_player.flags();

    if flags.on_ground() {
        return;
    }

    let velocity = local_player.velocity();
    let speed = Vec2::new(velocity.x, velocity.y).magnitude();
    let yaw_velocity = velocity.x.atan2(velocity.y).to_degrees();
    let velocity_delta = (command.view_angle.yaw - yaw_velocity).normalize_yaw();

    if command.mouse_dx.abs() > 2 {
        command.side_move = if command.mouse_dx < 0 { -450.0 } else { 450.0 };

        return;
    }

    if speed == 0.0 || speed.is_nan() || speed.is_infinite() {
        return;
    }

    command.forward_move = (5850.0 / speed).clamp(-450.0, 450.0);
    command.side_move = if velocity_delta > 0.0 { -450.0 } else { 450.0 };
    command.view_angle.yaw = (command.view_angle.yaw - velocity_delta).normalize_yaw();
}

pub fn do_move(command: &mut Command, send_packet: &mut bool, local_player: &Entity) {
    *send_packet = (command.tick_count % 50) - 25 > 25;

    command.view_angle = unsafe { globals::engine().view_angle() };
    command.view_angle.pitch = f32::down();

    if *send_packet {
        command.view_angle.yaw = 180.0;
    } else {
        command.view_angle.yaw = 180.0 + 120.0;
    }

    command.view_angle.yaw += ((command.command_number % 50) - 25) as f32;

    if command.state & (1 << 0) != 0 {
        command.view_angle = unsafe { globals::engine().view_angle() };
    }

    bunny_hop(command, send_packet, local_player);
}*/

pub const IN_JUMP: i32 = 1 << 1;
pub const IN_DUCK: i32 = 1 << 2;
pub const IN_BULLRUSH: i32 = 1 << 22;

pub unsafe extern "C" fn hook(
    this: *const (),
    input_sample_time: f32,
    command: &mut Command,
) -> bool {
    println!("create_move hook");

    let global = Global::handle();
    let result = global.create_move_original(this, input_sample_time, command);

    if command.tick_count == 0 {
        return true;
    }

    let send_packet =
        unsafe { &mut *(*(intrinsics::frame_address(0) as *mut *mut bool)).sub(0x18) };
    let original_angle = command.view_angle;
    let original_forward = command.forward_move;
    let original_side = command.side_move;

    if let Some(local_player) = global.local_player() {
        let on_move = &*global.on_move_ptr();
        let movement = on_move(Movement {
            forward_move: command.forward_move,
            side_move: command.side_move,
            up_move: command.up_move,
            view_angle: command.view_angle,
            send_packet: *send_packet,
            tick_count: command.tick_count,
            in_jump: command.state & IN_JUMP != 0,
            in_duck: command.state & IN_DUCK != 0,
            in_fast_duck: command.state & (IN_DUCK | IN_BULLRUSH) != 0,
            local_player: (local_player as *const Entity).read(),
        });

        command.state &= !(IN_JUMP | IN_DUCK | IN_BULLRUSH);
        command.state |= ((movement.in_jump as u32) << 1) as i32;
        command.state |= (((movement.in_duck | movement.in_fast_duck) as u32) << 2) as i32;
        command.state |= ((movement.in_fast_duck as u32) << 21) as i32;

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

    return false;
}
