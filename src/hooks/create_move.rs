//use crate::angle::Angle;
use crate::util::FloatExt;
use crate::{globals, sdk};
use std::lazy::SyncOnceCell;
use std::mem;
use vek::Vec2;

pub type Signature =
    extern "C" fn(this: *const (), input_sample_time: f32, command: &mut sdk::Command) -> bool;

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original(
    this: *const (),
    input_sample_time: f32,
    command: &mut sdk::Command,
) -> bool {
    let original = *ORIGINAL.get().unwrap_unchecked();

    original(this, input_sample_time, command)
}

pub fn set_original(original: *const ()) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe fn bunny_hop(command: &mut sdk::Command) {
    let local_player = match globals::local_player() {
        Some(local_player) => local_player,
        None => return,
    };

    let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

    if !flags.on_ground() {
        command.state.0 &= !sdk::input::State::JUMP.0;
    }
}

pub unsafe fn rage_strafe(command: &mut sdk::Command) {
    let local_player = match globals::local_player() {
        Some(local_player) => local_player,
        None => return,
    };

    let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

    if flags.on_ground() {
        return;
    }

    let velocity = local_player.velocity();
    let speed = Vec2::new(velocity.x, velocity.y).magnitude();

    if speed < 1.0 {
        command.state.0 = command.state.0 | sdk::input::State::FORWARD.0;

        return;
    }

    if command.mouse_dx > 1 || command.mouse_dx < -1 {
        command.side_move = if command.mouse_dx < 0 { -450.0 } else { 450.0 };
    } else {
        command.forward_move = 10000.0 / speed;
        command.forward_move = if command.command_number % 2 == 0 {
            -450.0
        } else {
            450.0
        };
        command.side_move = if command.command_number % 2 == 0 {
            -450.0
        } else {
            450.0
        };
    }
}

pub unsafe fn rage_strafe2(command: &mut sdk::Command) {
    let local_player = match globals::local_player() {
        Some(local_player) => local_player,
        None => return,
    };

    let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

    if flags.on_ground() {
        return;
    }

    let speed = local_player.velocity().magnitude();
    let rotation = ((30.0 / speed).sin().to_degrees() * 0.5).min(45.0);

    println!("speed = {:?}", speed);
    println!("rotation = {:?}", rotation);

    if speed < 1.0 {
        command.forward_move = 450.0;
        //command.state.0 = command.state.0 | sdk::input::State::FORWARD.0;

        return;
    }

    if command.mouse_dx > 1 || command.mouse_dx < -1 {
        command.forward_move = if command.mouse_dx < 0 { -450.0 } else { 450.0 };
    } else {
        command.forward_move = 10000.0 / speed;
        command.forward_move = if command.command_number % 2 == 0 {
            -450.0
        } else {
            450.0
        };
        command.side_move = if command.command_number % 2 == 0 {
            -450.0
        } else {
            450.0
        };
    }
}

pub unsafe fn directional_strafe(command: &mut sdk::Command) {
    let local_player = match globals::local_player() {
        Some(local_player) => local_player,
        None => return,
    };

    let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

    if flags.on_ground() {
        return;
    }

    let velocity = local_player.velocity();
    let speed = Vec2::new(velocity.x, velocity.y).magnitude();
    let yaw_velocity = velocity.x.atan2(velocity.y).to_degrees();
    let velocity_delta = (command.view_angles.yaw - yaw_velocity).normalize_yaw();

    globals::console().write(format!("velocity = {:?}\n", &velocity));
    globals::console().write(format!("speed = {:?}\n", &speed));
    globals::console().write(format!("yaw_velocity = {:?}\n", &yaw_velocity));
    globals::console().write(format!("velocity_delta = {:?}\n", &velocity_delta));

    if command.mouse_dx.abs() > 2 {
        command.side_move = if command.mouse_dx < 0 { -450.0 } else { 450.0 };

        return;
    }

    if command.in_backward() {
        command.view_angles.yaw += f32::backward();
    } else if command.in_left() {
        command.view_angles.yaw += f32::left();
    } else if command.in_right() {
        command.view_angles.yaw += f32::right();
    } else {
        //command.state.0 |= sdk::input::State::FORWARD.0;
    }

    if speed == 0.0 || speed.is_nan() || speed.is_infinite() {
        //command.state.0 |= sdk::input::State::FORWARD.0;

        return;
    }

    command.state.0 = 0;
    command.forward_move = (5850.0 / speed).clamp(-450.0, 450.0);
    command.side_move = if velocity_delta > 0.0 { -450.0 } else { 450.0 };
    command.view_angles.yaw = (command.view_angles.yaw - velocity_delta).normalize_yaw();
    command.tick_count = 0;
}

extern "C" {
    #[link_name = "llvm.frameaddress"]
    fn frame_address(depth: i32) -> *const i8;
}

pub unsafe extern "C" fn hook(
    this: *const (),
    input_sample_time: f32,
    command: &mut sdk::Command,
) -> bool {
    let result = original(this, input_sample_time, command);

    if command.tick_count == 0 {
        return true;
    }

    let send_packet = (*(frame_address(0) as *mut *mut bool)).sub(0x18);
    let original_angle = command.view_angles;
    let original_forward = command.forward_move;
    let original_side = command.side_move;

    command.view_angles.pitch = f32::down();

    bunny_hop(command);
    //rage_strafe(command);
    directional_strafe(command);
    //rage_strafe2(command);

    *send_packet = command.tick_count % 14 == 0;

    if *send_packet {
        command.view_angles.yaw += 180.0;
    } else {
        command.view_angles.yaw += 180.0 + 120.0;
    }

    command.view_angles = command.view_angles.normalize();
    //command.view_angles = globals::engine().view_angle();

    let f1 = if original_angle.yaw < 0.0 {
        360.0 + original_angle.yaw
    } else {
        original_angle.yaw
    };

    let f2 = if command.view_angles.yaw < 0.0 {
        360.0 + command.view_angles.yaw
    } else {
        command.view_angles.yaw
    };

    let mut delta_view_angles = if f2 < f1 {
        (f2 - f1).abs()
    } else {
        360.0 - (f2 - f1).abs()
    };

    delta_view_angles = 360.0 - delta_view_angles;

    let delta_radian = delta_view_angles.to_radians();
    let delta_radian_90 = (delta_view_angles + 90.0).to_radians();

    command.forward_move =
        delta_radian.cos() * original_forward + delta_radian_90.cos() * original_side;

    command.side_move =
        delta_radian.sin() * original_forward + delta_radian_90.sin() * original_side;

    return false;
}
