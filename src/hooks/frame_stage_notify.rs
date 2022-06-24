use crate::{state, Entity};
use core::mem;
use core::ptr::NonNull;
use elysium_math::Vec3;
use elysium_sdk::client::Class;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::EntityId;
use elysium_sdk::{Engine, EntityList, Frame, Globals, Input};

/// `FrameStageNotify` hook.
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

    // misc
    vars.allow_developer.write(true);
    vars.cheats.write(true);
    vars.developer.write(true);

    // useful
    vars.show_grenade_path.write(true);

    // annoying
    vars.auto_help.write(false);
    vars.show_help.write(false);

    // these disable when true
    vars.engine_sleep.write(true);
    vars.html_motd.write(true);
    vars.freeze_cam.write(true);
    vars.panorama_blur.write(true);

    // p100
    vars.hud.write(false);

    // shadows
    vars.csm.write(false);
    vars.csm_shadows.write(false);
    vars.feet_shadows.write(false);
    vars.prop_shadows.write(false);
    vars.rope_shadows.write(false);
    vars.shadows.write(false);
    vars.skybox3d.write(false);
    vars.viewmodel_shadows.write(false);
    vars.world_shadows.write(false);

    // useless objects
    vars.ropes.write(false);
    vars.sprites.write(false);

    // translucent things
    vars.water_fog.write(false);

    // overlay
    vars.underwater_overlay.write(false);

    // effects
    vars.alien_blood.write(false);
    vars.human_blood.write(false);
    vars.decals.write(false);
    vars.jiggle_bones.write(false);
    vars.rain.write(false);

    // phsyics
    vars.physics_timescale.write(0.5);

    if entity.is_null() {
        state::local::set_aim_punch_angle(Vec3::zero());
        state::local::set_player_none();
        state::local::set_view_punch_angle(Vec3::zero());
    } else {
        state::local::set_player(NonNull::new_unchecked(entity.as_mut()));

        let local = &*entity.cast::<Entity>();

        if local.observer_mode().breaks_thirdperson() {
            input.thirdperson = false;
        } else {
            input.thirdperson = state::local::thirdperson();
        }

        match frame {
            Frame::RenderStart => {
                if input.thirdperson {
                    // fix the local player's view_angle when in thirdperson
                    *local.view_angle() = state::local::view_angle();
                    // other players can't see roll, so why should we?
                    local.view_angle().z = 0.0;
                } else {
                    // in cooperation with override_view, this will change the view model's position.
                    if state::local::use_shot_view_angle() != 0.0 {
                        if state::local::use_shot_view_angle() > globals.current_time {
                            *local.view_angle() = state::local::shot_view_angle();
                        } else {
                            *local.view_angle() = *state::view_angle();
                            state::local::set_use_shot_view_angle(0.0);
                        }
                    }

                    // rotate view model
                    local.view_angle().z = -35.0;
                }

                let players = &mut *state::players();
                println!("local player = {local:?}");

                let networkable = &*(local as *const Entity)
                    .byte_add(16)
                    .cast::<elysium_sdk::entity::Networkable>();

                let local_index = networkable.index() as usize;
                println!("local player index = {local_index:?}");
                let local_index = local.index() as usize;

                for index in 1..=64 {
                    if index == local_index {
                        println!("entity {index} is the local player");
                        continue;
                    }

                    let bones = &mut players[index - 1].bones;
                    let entity = entity_list.get(index);

                    if entity.is_null() {
                        println!("entity {index} is null");
                        *bones = providence_model::Bones::zero();
                        continue;
                    }

                    let entity = &*entity.cast::<Entity>();

                    if entity.is_dormant() {
                        println!("entity {index} is dormant");
                        *bones = providence_model::Bones::zero();
                        continue;
                    }

                    entity.setup_bones(&mut bones[0..128], 0x00000100, globals.current_time);

                    entity.setup_bones(&mut bones[0..128], 0x000FFF00, globals.current_time);
                }

                let highest_entity_index = entity_list.len();

                for index in 64..=highest_entity_index {
                    let entity = entity_list.get(index);

                    if entity.is_null() {
                        continue;
                    }

                    let entity = &*entity.cast::<Entity>();

                    let class = entity.client_class();

                    if class.is_null() {
                        continue;
                    }

                    let class = &*class.cast::<Class>();

                    if class.entity_id == EntityId::CFogController {
                        *entity.is_enabled() = true;
                        *entity.start_distance() = 1.0;
                        *entity.end_distance() = 10000.0;
                        *entity.far_z() = 10000.0;
                        *entity.density() = 1.0;
                        *entity.color_primary() = 0xFF0000FF;
                    }
                }
            }
            _ => {
                if input.thirdperson {
                    // restore to the expected value
                    *local.view_angle() = *state::view_angle();
                }
            }
        }
    }

    state::hooks::frame_stage_notify(this, frame as i32);
}
