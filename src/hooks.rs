//! Function hooks.

use crate::{state, Entity};
use core::mem;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use elysium_math::Vec3;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::{MoveKind, ObserverMode};
use elysium_sdk::{Command, Engine, EntityList, Frame, Globals, Input, View};
use iced_elysium_gl::Viewport;
use iced_native::Size;

/// `SDL_GL_SwapWindow` hook.
#[inline(never)]
pub unsafe extern "C" fn swap_window(sdl_window: *mut sdl2_sys::SDL_Window) {
    let mut width = MaybeUninit::uninit();
    let mut height = MaybeUninit::uninit();

    sdl2_sys::SDL_GetWindowSize(sdl_window, width.as_mut_ptr(), height.as_mut_ptr());

    let width = width.assume_init();
    let height = height.assume_init();
    let size = Size::new(width as u32, height as u32);

    state::update_window_size(size);

    let context = state::gl_context();

    // enable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // enable alpha blending to not break our fonts
    context.enable(elysium_gl::BLEND);
    context.blend_func(elysium_gl::SRC_ALPHA, elysium_gl::ONE_MINUS_SRC_ALPHA);

    let viewport = Viewport::with_physical_size(size, 1.0);
    let menu = state::menu(context, viewport.clone());

    //if state::is_menu_open() {
    context.viewport(0, 0, size.width as i32, size.height as i32);

    menu.update(viewport.clone(), state::cursor_position());
    menu.draw(context, viewport);
    //}

    // disable auto-conversion from/to sRGB
    context.enable(elysium_gl::FRAMEBUFFER_SRGB);

    // disable alpha blending to not break vgui fonts
    context.disable(elysium_gl::BLEND);

    state::hooks::swap_window(sdl_window);
}

/// `SDL_PollEvent` hook.
#[inline(never)]
pub unsafe extern "C" fn poll_event(sdl_event: *mut sdl2_sys::SDL_Event) -> i32 {
    let result = state::hooks::poll_event(sdl_event);

    if !state::is_menu_none() {
        let menu = state::menu_unchecked();

        elysium_input::map_event(*sdl_event, |event| {
            use iced_native::{keyboard, mouse, Event};

            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Insert,
                    ..
                }) => state::toggle_menu(),
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Other(4))) => {
                    state::local::toggle_thirdperson()
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    // block input to the game when the menu is open
    if state::is_menu_open() {
        (*sdl_event).type_ = 0;
    }

    result
}

const IN_BULLRUSH: i32 = 1 << 22;
const IN_JUMP: i32 = 1 << 1;
const ON_GROUND: i32 = 1 << 0;

fn fix_movement(command: &mut Command, original_view_angle: Vec3, original_movement: Vec3) {
    let f1 = if original_view_angle.y < 0.0 {
        360.0 + original_view_angle.y
    } else {
        original_view_angle.y
    };

    let f2 = if command.view_angle.y < 0.0 {
        360.0 + command.view_angle.y
    } else {
        command.view_angle.y
    };

    let mut delta_view_angle = if f2 < f1 {
        (f2 - f1).abs()
    } else {
        360.0 - (f1 - f2).abs()
    };

    delta_view_angle = 360.0 - delta_view_angle;

    let (sin, cos) = delta_view_angle.to_radians().sin_cos();
    let (sin_90, cos_90) = (delta_view_angle + 90.0).to_radians().sin_cos();

    command.movement.x = cos * original_movement.x + cos_90 * original_movement.y;
    command.movement.y = sin * original_movement.x + sin_90 * original_movement.y;
}

#[inline]
unsafe fn do_create_move(command: &mut Command) {
    if command.tick_count == 0 || state::local::is_player_none() {
        return;
    }

    let local = &*state::local::player().as_ptr().cast::<Entity>();

    // can you dont when spectatng
    if local.observer_mode() != ObserverMode::None {
        return;
    }

    // can you dont when on ladder or in noclip
    if matches!(local.move_kind(), 8 | 9) {
        return;
    }

    if (command.state & IN_JUMP) != 0 {
        if (local.flags() & ON_GROUND) == 0 {
            command.state &= !IN_JUMP;
        }
    }

    if (local.flags() & ON_GROUND) == 0 {
        let side = if command.command % 2 != 0 { 1.0 } else { -1.0 };
        let velocity = local.velocity();
        let magnitude = velocity.magnitude2d();
        let ideal_strafe = (15.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0);
        let mut wish_angle = command.view_angle;
        let strafe_dir = command.movement.to_dir();
        let strafe_dir_yaw_offset = strafe_dir.y.atan2(strafe_dir.x).to_degrees();

        wish_angle.y -= strafe_dir_yaw_offset;

        let mut wish_angle = wish_angle.sanitize_angle();
        let yaw_delta = libm::remainderf(wish_angle.y - state::local::old_yaw(), 360.0);
        let abs_yaw_delta = yaw_delta.abs();

        state::local::set_old_yaw(wish_angle.y);

        let vars = &*state::vars().cast::<Vars>();
        let horizontal_speed = vars.horizontal_speed.read();

        if abs_yaw_delta <= ideal_strafe || abs_yaw_delta >= 30.0 {
            let velocity_dir = Vec3::vector_angle(velocity);
            let velocity_yaw_delta = libm::remainderf(wish_angle.y - velocity_dir.y, 360.0);
            let retrack = (30.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0) * 2.0;

            if velocity_yaw_delta <= retrack || magnitude <= 15.0 {
                if -retrack <= velocity_yaw_delta || magnitude <= 15.0 {
                    wish_angle.y += side * ideal_strafe;
                    command.movement.y = horizontal_speed * side;
                } else {
                    wish_angle.y = velocity_dir.y - retrack;
                    command.movement.y = horizontal_speed;
                }
            } else {
                wish_angle.y = velocity_dir.y + retrack;
                command.movement.y = -horizontal_speed;
            }
        } else if yaw_delta > 0.0 {
            command.movement.y = -horizontal_speed;
        } else if yaw_delta < 0.0 {
            command.movement.y = horizontal_speed
        }

        command.movement.x = 0.0;

        fix_movement(command, wish_angle, command.movement);
    }

    command.state |= IN_BULLRUSH;
}

/// `CreateMove` hook.
#[inline(never)]
pub unsafe extern "C" fn create_move(
    this: *const u8,
    input_sample_time: f32,
    command: *mut u8,
) -> bool {
    state::hooks::create_move(this, input_sample_time, command);

    let command = &mut *command.cast::<Command>();

    do_create_move(command);

    state::local::set_view_angle(command.view_angle);

    false
}

/// `CL_Move` hook.
#[inline(never)]
pub unsafe extern "C" fn cl_move(_accumulated_extra_samples: f32, _final_tick: bool) {}

#[inline(never)]
pub unsafe extern "C" fn frame_stage_notify(this: *const u8, frame: i32) {
    let engine = &*state::engine().cast::<Engine>();
    let entity_list = &*state::entity_list().cast::<EntityList>();
    let globals = &*state::globals().cast::<Globals>();
    let input = &mut *state::input().as_mut().cast::<Input>();
    let vars = &*state::vars().cast::<Vars>();

    *state::view_angle() = engine.view_angle();

    let frame: Frame = mem::transmute(frame);
    let index = engine.local_player_index();
    let entity = entity_list.get(index);

    vars.cheats.write(true);
    vars.panorama_blur.write(true);
    vars.hud.write(false);

    if entity.is_null() {
        state::local::set_aim_punch_angle(Vec3::zero());
        state::local::set_player_none();
        state::local::set_view_punch_angle(Vec3::zero());
    } else {
        state::local::set_player(NonNull::new_unchecked(entity.as_mut()));

        let entity = &*entity.cast::<Entity>();

        if entity.observer_mode().breaks_thirdperson() {
            input.thirdperson = false;
        } else {
            input.thirdperson = state::local::thirdperson();
        }

        match frame {
            Frame::RenderStart => {
                if input.thirdperson {
                    // fix the local player's view_angle when in thirdperson
                    *entity.view_angle() = state::local::view_angle();
                } else {
                    // in cooperation with override_view, this will change the view model's position.
                    if state::local::use_shot_view_angle() != 0.0 {
                        if state::local::use_shot_view_angle() > globals.current_time {
                            *entity.view_angle() = state::local::shot_view_angle();
                        } else {
                            *entity.view_angle() = *state::view_angle();
                            state::local::set_use_shot_view_angle(0.0);
                        }
                    }

                    // rotate view model
                    entity.view_angle().z = -90.0;
                }
            }
            _ => {
                if input.thirdperson {
                    // restore to the expected value
                    *entity.view_angle() = *state::view_angle();
                }
            }
        }
    }

    state::hooks::frame_stage_notify(this, frame as i32);
}

#[inline(never)]
pub unsafe extern "C" fn override_view(this: *const u8, view: *mut u8) {
    let view = &mut *view.cast::<View>();

    view.angle = *state::view_angle();

    state::hooks::override_view(this, (view as *mut View).cast());
}

#[inline(never)]
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
