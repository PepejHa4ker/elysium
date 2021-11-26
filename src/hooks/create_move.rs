use crate::sdk::Angle;
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

pub fn normalize_pitch(pitch: f32) -> f32 {
    let mut pitch = pitch;

    while pitch > 89.0 {
        pitch -= 180.0;
    }

    while pitch < -89.0 {
        pitch += 180.0;
    }

    pitch
}

pub fn normalize_yaw(yaw: f32) -> f32 {
    let mut yaw = yaw;

    while yaw > 180.0 {
        yaw -= 360.0;
    }

    while yaw < -180.0 {
        yaw += 360.0;
    }

    yaw
}

pub fn normalize_angles(angle: Angle) -> Angle {
    Angle::new(normalize_pitch(angle.x), normalize_yaw(angle.y), 0.0)
}

pub unsafe fn bunny_hop(command: &mut sdk::Command) {
    let local_player = match globals::local_player() {
        Some(local_player) => local_player,
        None => return,
    };

    let flags: sdk::PlayerState = mem::transmute(*local_player.flags());

    if !flags.on_ground() {
        command.state.0 = command.state.0 & !sdk::input::State::JUMP.0;
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
    let velocity_delta = normalize_yaw(command.view_angles.y - yaw_velocity);

    globals::console().write(format!("velocity = {:?}\n", &velocity));
    globals::console().write(format!("speed = {:?}\n", &speed));
    globals::console().write(format!("yaw_velocity = {:?}\n", &yaw_velocity));
    globals::console().write(format!("velocity_delta = {:?}\n", &velocity_delta));

    if command.mouse_dx.abs() > 2 {
        command.side_move = if command.mouse_dx < 0 { -450.0 } else { 450.0 };

        return;
    }

    if command.in_backward() {
        globals::console().write("backward\n");

        command.view_angles.y += -180.0;
    } else if command.in_left() {
        globals::console().write("left\n");

        command.view_angles.y += 90.0;
    } else if command.in_right() {
        globals::console().write("right\n");

        command.view_angles.y += -90.0;
    } else {
        globals::console().write("forward\n");

        command.state.0 = command.state.0 | sdk::input::State::FORWARD.0;
    }

    if speed == 0.0 || speed.is_nan() || speed.is_infinite() {
        command.state.0 = command.state.0 | sdk::input::State::FORWARD.0;

        return;
    }

    command.state.0 = 0;
    command.forward_move = (5850.0 / speed).clamp(-450.0, 450.0);
    command.side_move = if velocity_delta > 0.0 { -450.0 } else { 450.0 };
    command.view_angles.y = normalize_yaw(command.view_angles.y - velocity_delta);
    command.tick_count = 0;
}

pub unsafe extern "C" fn hook(
    this: *const (),
    input_sample_time: f32,
    command: &mut sdk::Command,
) -> bool {
    let result = original(this, input_sample_time, command);

    if command.tick_count != 0 {
        bunny_hop(command);
        rage_strafe(command);
    }

    return result;
}
