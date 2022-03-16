#![feature(const_fn_fn_ptr_basics)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(extern_types)]
#![feature(once_cell)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(ptr_metadata)]

use crate::contents::Contents;
use crate::entity::Entity;
use crate::entity::Player;
use crate::entity::Weapon;
use crate::frame::Frame;
use crate::global::Global;
use crate::trace::{Ray, Summary};
use atomic_float::AtomicF32;
use core::ptr;
use providence_math::Vec3;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod client;
pub mod command;
pub mod console;
pub mod consts;
pub mod contents;
pub mod debug;
pub mod engine;
pub mod entity;
pub mod error;
pub mod frame;
pub mod global;
pub mod globals;
pub mod hit_group;
pub mod hooks;
pub mod input;
pub mod interfaces;
pub mod islice;
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod managed;
pub mod material;
pub mod model;
pub mod move_kind;
pub mod movement;
pub mod networked;
pub mod pad;
pub mod pattern;
pub mod physics;
pub mod state;
pub mod trace;

fn main() -> Result<()> {
    // todo: check we are loaded into counter strike
    //       we get loaded into bash too when executing
    //       counter strike from steam (csgo.sh)

    if library::Library::serverbrowser().is_err() {
        println!("Waiting for Counter Strike to finish loading...");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!("Initializing providence...");

    let global = Global::init()?;
    let global2 = global.clone();
    let global3 = global.clone();
    let choked = AtomicI32::new(0);

    global.on_frame(move |frame| {
        if let Some(local_player) = global2.local_player() {
            match frame {
                Frame::RENDER_START => {
                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(1);
                    global2.physics_timescale().set(0.5);
                    //global2.ragdoll_gravity().set(-800.0);

                    // No recoil / no punch.
                    global2.set_aim_punch_angle(local_player.actual_aim_punch_angle());
                    global2.set_view_punch_angle(local_player.actual_view_punch_angle());

                    local_player.set_aim_punch_angle(Vec3::zero());
                    local_player.set_view_punch_angle(Vec3::zero());

                    // "Fix" the local players view angle and backup the current value.
                    if global2.input().thirdperson {
                        unsafe {
                            let original_view_angle = local_player.view_angle();

                            *providence_state::original_view_angle() = original_view_angle;

                            local_player.set_view_angle(*providence_state::view_angle());
                        }
                    }

                    unsafe {
                        let cached_players = &mut *providence_state::cached_players();
                        let entity_list = global2.entity_list();
                        let client_time = global2.client_time();

                        for index in 1..64 {
                            let bones = &mut cached_players.players[index as usize].bones;

                            if let Some(entity) = entity_list.get(index) {
                                let bones_ptr = bones.as_mut_ptr();

                                entity.setup_bones(bones_ptr, 128, 0x00000100, client_time);
                                entity.setup_bones(bones_ptr, 128, 0x000FFF00, client_time);
                            } else {
                                *bones = providence_model::Bones::zero();
                            }
                        }

                        let highest_entity_index = entity_list.highest_entity_index();

                        for index in 1..highest_entity_index {
                            let entity = match entity_list.get(index) {
                                Some(entity) => entity,
                                None => continue,
                            };

                            let class = entity.class();

                            if class.is_null() {
                                continue;
                            }

                            let class = &*class;

                            use crate::entity::{EntityId, Fog};

                            if class.entity_id == EntityId::CFogController {
                                println!("found fog!");

                                let fog = Fog::new_unchecked(entity.as_ptr() as *mut _);

                                *fog.is_enabled() = true;
                                *fog.start() = 1.0;
                                *fog.end() = 10000.0;
                                *fog.far_z() = 10000.0;
                                *fog.density() = 1.0;
                                *fog.color_primary() = 0x00FF00FF;
                            }
                        }
                    }
                }
                Frame::RENDER_END => {
                    // Restore aim and view punch to not break things.
                    local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    local_player.set_view_punch_angle(global2.view_punch_angle());

                    // Restore the local players view_angle.
                    if global2.input().thirdperson {
                        unsafe {
                            local_player.set_view_angle(*providence_state::original_view_angle());
                        }
                    }

                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(0);
                    global2.physics_timescale().set(1.0);
                    //global2.ragdoll_gravity().set(800.0);
                }
                _ => {}
            }
        }

        global2.panorama_blur().set(1);
        global2.cheats().set(1);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        let view_angle = global3.engine().view_angle();
        let vectors = movement.vectors;
        let punch = movement.local_player.aim_punch_angle() * Vec3::splat(2.0);
        let original_vectors = movement.vectors;
        let side = (movement.tick_count * 2 - 1) as f32;

        movement.view.x = 89.0;
        movement.view.y += if movement.tick_count % 3 == 0 {
            180.0
        } else {
            -180.0
        };

        if movement.do_attack {
            movement.send_packet = true;
            movement.view = view_angle;

            unsafe {
                if let Some(weapon) = movement.local_player.weapon() {
                    *weapon.next_attack_available_after() = global3.globals().current_time;

                    let entity_list = global3.entity_list();
                    let cached_players = &mut *providence_state::cached_players();
                    let local_player_index = movement.local_player.index();
                    let local_player_eye_origin = movement.local_player.eye_origin();

                    let mut best_fov = 30.0;
                    let mut view_angle = movement.view;
                    //let mut best_target = None;

                    for index in 1..64 {
                        if index == local_player_index {
                            continue;
                        }

                        let bones = &mut cached_players.players[index as usize].bones;

                        let entity = match entity_list.get(index) {
                            Some(entity) => entity,
                            None => continue,
                        };

                        if entity.is_dormant() {
                            continue;
                        }

                        let player = Player::new_unchecked(entity.as_ptr() as *mut _);

                        if player.is_dead() {
                            continue;
                        }

                        println!("{index}'s team is {}", player.as_entity().team());

                        if player.as_entity().team() == movement.local_player.as_entity().team() {
                            continue;
                        }

                        let bone_origin = bones.get_origin(8).unwrap_unchecked();

                        view_angle = calculate_angle(local_player_eye_origin, bone_origin) - punch;
                        view_angle.normalize_in_place();

                        let damage = get_damage(&movement.local_player, &weapon, view_angle);
                    }

                    if view_angle == movement.view {
                        movement.view -= punch;
                    } else {
                        movement.view = view_angle;
                    }
                }
            }
        } else {
            if choked.load(Ordering::Relaxed) > 0 {
                choked.store(0, Ordering::Relaxed);
                movement.send_packet = true;
            } else {
                choked.fetch_add(1, Ordering::Relaxed);
                movement.send_packet = false;
            }
        }

        if movement.do_duck {
            movement.vectors.y = side * 3.25;
        } else {
            movement.vectors.y = side * 1.1;
        }

        if !movement.send_packet {
            movement.view.y += 270.0;
        }

        if movement.send_packet {
            unsafe {
                *providence_state::view_angle() = movement.view;

                let cached_players = &mut *providence_state::cached_players();
                let index = movement.local_player.index();
                let bones = &mut cached_players.players[index as usize].bones;
                let local_player_bones = &mut *providence_state::local_player_bones();

                ptr::copy_nonoverlapping(
                    bones.as_ptr(),
                    local_player_bones.as_mut_ptr(),
                    providence_model::MAX_BONES,
                );
            }
        }

        movement.view.normalize_in_place();
        movement.view.clamp_in_place();

        let mut yaw1 = view_angle.y;
        let mut yaw2 = movement.view.y;

        if yaw1 < 0.0 {
            yaw1 += 360.0;
        }

        if yaw2 < 0.0 {
            yaw2 += 360.0;
        }

        let delta = 360.0
            - if yaw2 < yaw1 {
                (yaw2 - yaw1).abs()
            } else {
                360.0 - (yaw1 - yaw2).abs()
            };

        let (sin, cos) = delta.to_radians().sin_cos();
        let (sin90, cos90) = (delta + 90.0).to_radians().sin_cos();

        movement.vectors.x = cos * original_vectors.x + cos90 * original_vectors.y;
        movement.vectors.y = sin * original_vectors.x + sin90 * original_vectors.y;
        movement
    });

    Ok(())
}

#[ctor::ctor]
fn load() {
    let _ = thread::Builder::new().spawn(move || {
        tracing_subscriber::fmt::init();

        let _ = main();
    });
}

const SURF_LIGHT: u32 = 0x0001; // value will hold the light strength
const SURF_SKY2D: u32 = 0x0002; // don't draw, indicates we should skylight draw 2d sky but not draw the 3D skybox
const SURF_SKY: u32 = 0x0004; // don't draw, but add to skybox
const SURF_WARP: u32 = 0x0008; // turbulent water warp
const SURF_TRANS: u32 = 0x0010;
const SURF_NOPORTAL: u32 = 0x0020; // the surface can not have a portal placed on it
const SURF_TRIGGER: u32 = 0x0040; // FIXME: This is an xbox hack to work around elimination of trigger surfaces, which breaks occluders
const SURF_NODRAW: u32 = 0x0080; // don't bother referencing the texture
const SURF_HINT: u32 = 0x0100; // make a primary bsp splitter
const SURF_SKIP: u32 = 0x0200; // completely ignore, allowing non-closed brushes
const SURF_NOLIGHT: u32 = 0x0400; // Don't calculate light
const SURF_BUMPLIGHT: u32 = 0x0800; // calculate three lightmaps for the surface for bumpmapping
const SURF_NOSHADOWS: u32 = 0x1000; // Don't receive shadows
const SURF_NODECALS: u32 = 0x2000; // Don't receive decals
const SURF_NOPAINT: u32 = SURF_NODECALS; // the surface can not have paint placed on it
const SURF_NOCHOP: u32 = 0x4000; // Don't subdivide patches on this surface
const SURF_HITBOX: u32 = 0x8000; // surface is part of a hitbox
const MASK_ALL: u32 = 0xFFFFFFFF;

fn trace_to_exit(
    start: Vec3,
    direction: Vec3,
    enter_summary: &Summary,
    exit_summary: &mut Summary,
    end: &mut Vec3,
) -> bool {
    println!("{start:?} {direction:?} {enter_summary:?} {exit_summary:?} {end:?}");

    let global = Global::handle();
    let mut distance = 0.0;

    while distance <= 90.0 {
        distance += 4.0;

        *end = start + direction * Vec3::splat(distance);

        let contents = global.ray_tracer().point_contents(
            *end,
            Contents::new().mask_shot_hull().hitbox(),
            ptr::null(),
        );

        if contents.has_mask_shot_hull() && contents.has_hitbox() {
            continue;
        }

        let new_end = *end - (direction * Vec3::splat(4.0));

        global.ray_tracer().trace_mut(
            &Ray::new(*end, new_end),
            Contents::new().mask_shot().hitbox(),
            None,
            exit_summary,
        );

        if exit_summary.start_solid && (exit_summary.surface.flags & SURF_HITBOX as u16) != 0 {
            unsafe {
                global.ray_tracer().trace_filtered_unchecked(
                    &Ray::new(*end, start),
                    Contents::new().mask_shot_hull().hitbox(),
                    exit_summary.entity_hit,
                    exit_summary,
                );
            }

            if (exit_summary.fraction <= 1.0 || exit_summary.all_solid) && !exit_summary.start_solid
            {
                *end = exit_summary.end;
                return true;
            }

            continue;
        }

        if !(exit_summary.fraction <= 1.0 || exit_summary.all_solid || exit_summary.start_solid)
            || exit_summary.start_solid
        {
            if exit_summary.entity_hit.is_null() {
                return true;
            }

            continue;
        }

        if (exit_summary.surface.flags & SURF_NODRAW as u16) != 0 {
            continue;
        }

        if exit_summary.plane.normal.dot(direction) <= 1.0 {
            let fraction = exit_summary.fraction * 4.0;

            *end = *end - (direction * Vec3::splat(fraction));

            return true;
        }
    }

    false
}

#[derive(Debug)]
pub struct ShotData {
    pub source: Vec3,
    pub enter_summary: Summary,
    pub direction: Vec3,
    pub filter: Option<Entity>,
    pub trace_length: f32,
    pub trace_length_remaining: f32,
    pub current_damage: f32,
    pub penetrate_count: i32,
}

impl ShotData {
    pub fn new() -> Self {
        Self {
            source: Vec3::zero(),
            enter_summary: Summary::new(),
            direction: Vec3::zero(),
            filter: None,
            trace_length: 0.0,
            trace_length_remaining: 0.0,
            current_damage: 0.0,
            penetrate_count: 0,
        }
    }

    pub fn handle_bullet_penetration(&mut self, weapon: &Weapon) -> bool {
        let global = Global::handle();
        let surface = match global
            .physics()
            .query(self.enter_summary.surface.properties as i32)
        {
            Some(surface) => surface,
            None => return true,
        };

        let enter_material = surface.properties.material;
        let enter_penetration_modifier = surface.properties.penetration_modifier;

        self.trace_length += self.trace_length_remaining * self.enter_summary.fraction;
        self.current_damage *= weapon.range_modifier().powf(self.trace_length * 0.002);

        if self.trace_length > 3000.0 || enter_penetration_modifier < 0.1 {
            self.penetrate_count = 0;
        }

        if self.penetrate_count <= 0 {
            return false;
        }

        let mut end = Vec3::zero();
        let mut exit_summary = Summary::new();

        if !trace_to_exit(
            /* start */ self.enter_summary.end,
            /* direction */ self.direction,
            /* enter_summary */ &self.enter_summary,
            /* exit_summary */ &mut exit_summary,
            /* end_pos */ &mut end,
        ) {
            return false;
        }

        let surface = match global.physics().query(exit_summary.surface.properties as _) {
            Some(surface) => surface,
            None => return true,
        };

        let exit_material = surface.properties.material;
        let exit_penetration_modifier = surface.properties.penetration_modifier;
        let mut final_damage_modifier: f32 = 0.16;
        let mut combined_penetration_modifier: f32 = 0.0;

        if self.enter_summary.contents.has_grate() || matches!(enter_material, 71 | 89) {
            final_damage_modifier = 0.05;
            combined_penetration_modifier = 3.0;
        } else {
            combined_penetration_modifier =
                (enter_penetration_modifier + exit_penetration_modifier) * 0.5;
        }

        if enter_material == exit_material {
            if matches!(exit_material, 85 | 87) {
                combined_penetration_modifier = 3.0;
            } else if exit_material == 76 {
                combined_penetration_modifier = 2.0;
            }
        }

        let v34 = f32::max(0.0, 1.0 / combined_penetration_modifier);
        let v35 = self.current_damage * final_damage_modifier
            + v34 * 3.0 * f32::max(0.0, (3.0 / weapon.penetration()) * 1.25);

        let mut thickness = (exit_summary.end - self.enter_summary.end).magnitude();

        thickness = (thickness * thickness * v34) / 24.0;

        let lost_damage = f32::max(0.0, v35 + thickness);

        if lost_damage > self.current_damage {
            return false;
        }

        if lost_damage >= 0.0 {
            self.current_damage -= lost_damage;
        }

        if self.current_damage < 1.0 {
            return false;
        }

        self.source = exit_summary.end;
        self.penetrate_count -= 1;

        // cant shoot through this
        true
    }

    pub fn simulate_shot(&mut self, local_player: &Player, weapon: &Weapon) -> bool {
        let global = Global::handle();

        let weapon_damage = weapon.damage();
        let weapon_range = weapon.range();
        let weapon_range_modifier = weapon.range_modifier();
        //let weapon_armor_ratio = weapon.armor_ratio();

        self.penetrate_count = 4;
        self.trace_length = 0.0;
        self.current_damage = weapon_damage;

        while self.penetrate_count > 0 && self.current_damage >= 1.0 {
            println!("current_damage = {}", self.current_damage);

            self.trace_length_remaining = weapon_range - self.trace_length;

            let end = self.source + self.direction * Vec3::splat(self.trace_length_remaining);
            let new_end = end + self.direction * Vec3::splat(4.0);

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, end),
                Contents::new().mask_shot(),
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                Contents::new().mask_shot(),
                self.filter.as_ref(),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                Contents::new().mask_shot(),
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            if self.enter_summary.fraction == 1.0 {
                break;
            }

            if self.enter_summary.hit_group.is_hit() {
                //self.enter_summary.hit_group.damage_multipler(),
                //self.en
                //weapon.armor_ratio()
                //scaleDamage(shotData.enterTrace.hitgroup, player, info->weaponArmorRatio(), shotData.currentDamage);

                return true;
            }

            //if !self.handle_bullet_penetration(weapon) {
            break;
            //}
        }

        false
    }
}

fn calculate_angle(src: Vec3, dst: Vec3) -> Vec3 {
    let delta = src - dst;
    let hypot = (delta.x * delta.x + delta.y * delta.y).sqrt();

    let x = (delta.z / hypot).atan().to_degrees();
    let mut y = (delta.y / delta.x).atan().to_degrees();
    let z = 0.0;

    if delta.x >= 0.0 {
        y += 180.0;
    }

    Vec3::from_xyz(x, y, z)
}

fn angle_vector(angle: &Vec3, forward: &mut Vec3) {
    let x = angle.x.to_radians();
    let y = angle.y.to_radians();

    let (x_sin, x_cos) = x.sin_cos();
    let (y_sin, y_cos) = y.sin_cos();

    forward.x = x_cos * y_cos;
    forward.y = x_cos * y_sin;
    forward.z = -x_sin;
}

fn get_damage(local_player: &Player, weapon: &Weapon, destination: Vec3) -> f32 {
    let mut shot_data = ShotData::new();

    shot_data.source = local_player.eye_origin();
    shot_data.filter = Some(unsafe { Entity::new_unchecked(local_player.as_ptr() as *mut _) });

    let angle = calculate_angle(shot_data.source, destination);

    angle_vector(&angle, &mut shot_data.direction);

    shot_data.direction.normalize_in_place();

    if shot_data.simulate_shot(local_player, weapon) {
        shot_data.current_damage
    } else {
        -1.0
    }
}
