#![feature(const_fn_fn_ptr_basics)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(extern_types)]
#![feature(once_cell)]
#![feature(ptr_metadata)]

use crate::entity::Weapon;
use crate::frame::Frame;
use crate::global::Global;
use crate::managed::handle;
use atomic_float::AtomicF32;
use core::ptr;
use sdk::Vec3;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod bones;
pub mod client;
pub mod command;
pub mod console;
pub mod consts;
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
pub mod mem;
pub mod model;
pub mod move_kind;
pub mod movement;
pub mod networked;
pub mod pad;
pub mod pattern;
pub mod physics;
pub mod trace;

#[derive(Clone)]
pub struct Choker(Arc<AtomicUsize>);

impl Choker {
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub fn reset(&self) {
        self.0.store(0, Ordering::SeqCst);
    }

    pub fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    pub fn increment(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

struct AtomicVec3Inner {
    x: AtomicF32,
    y: AtomicF32,
}

#[derive(Clone)]
pub struct AtomicVec3(Arc<AtomicVec3Inner>);

impl AtomicVec3 {
    pub fn new() -> Self {
        Self(Arc::new(AtomicVec3Inner {
            x: AtomicF32::new(0.0),
            y: AtomicF32::new(0.0),
        }))
    }

    pub fn x(&self) -> f32 {
        self.0.x.load(Ordering::SeqCst)
    }

    pub fn y(&self) -> f32 {
        self.0.y.load(Ordering::SeqCst)
    }

    pub fn set_x(&self, x: f32) {
        self.0.x.store(x, Ordering::SeqCst);
    }

    pub fn set_y(&self, y: f32) {
        self.0.y.store(y, Ordering::SeqCst);
    }

    pub fn get(&self) -> Vec3 {
        Vec3::from_xy(self.x(), self.y())
    }

    pub fn set(&self, angle: Vec3) {
        self.set_x(angle.x);
        self.set_y(angle.y);
    }
}

mod contents;

const CONTENTS_SOLID: u32 = 0x1; // an eye is never valid in a solid
const CONTENTS_WINDOW: u32 = 0x2; // translucent, but not watery: u32 = glass;
const CONTENTS_GRATE: u32 = 0x8; // alpha-tested "grate" textures. Bullets/sight pass through, but solids don't

// hits entities which are MOVETYPE_PUSH: u32 = doors, plats, etc.;
const CONTENTS_MOVEABLE: u32 = 0x4000;

const CONTENTS_MONSTER: u32 = 0x2000000; // should never be on a brush, only in game
const CONTENTS_DEBRIS: u32 = 0x4000000;
const CONTENTS_HITBOX: u32 = 0x40000000; // use accurate hitboxes on trace

// NOTE: These are stored in a short in the engine now. Don't use more than 16 bits
const SURF_NODRAW: u32 = 0x0080; // don't bother referencing the texture

const SURF_HITBOX: u32 = 0x8000; // surface is part of a hitbox

// bullets see these as solid
const MASK_SHOT: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_HITBOX;

// non-raycasted weapons see this as solid: u32 = includes grates;
const MASK_SHOT_HULL: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_GRATE;

use crate::entity::Entity;
use crate::trace::{Ray, Summary};

fn trace_to_exit(
    start: Vec3,
    direction: Vec3,
    enter_summary: &Summary,
    exit_summary: &mut Summary,
    end: &mut Vec3,
) -> bool {
    let global = Global::handle();
    let mut distance = 0.0;

    while distance <= 90.0 {
        distance += 4.0;
        *end = start + direction * Vec3::splat(distance);

        let contents = global.ray_tracer().point_contents(
            *end,
            (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
            ptr::null(),
        );

        if (contents & MASK_SHOT_HULL as i32) != 0 && (contents & CONTENTS_HITBOX as i32) != 0 {
            continue;
        }

        let new_end = *end - (direction * Vec3::splat(4.0));

        global.ray_tracer().trace_mut(
            &Ray::new(*end, new_end),
            (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
            None,
            exit_summary,
        );

        if exit_summary.start_solid && (exit_summary.surface.flags & SURF_HITBOX as u16) != 0 {
            let skip_entity = crate::trace::Filter::new(exit_summary.entity_hit);

            unsafe {
                global.ray_tracer().raw_trace(
                    &Ray::new(*end, start),
                    (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
                    &skip_entity,
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
            *end = *end - (direction * Vec3::splat(exit_summary.fraction * 4.0));

            return true;
        }
    }

    false
}

#[derive(Debug)]
#[repr(C)]
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

        if self.enter_summary.surface.properties < 1 {
            return true;
        }

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

        if exit_summary.surface.properties < 1 {
            return true;
        }

        let surface = match global
            .physics()
            .query(exit_summary.surface.properties as i32)
        {
            Some(surface) => surface,
            None => return true,
        };

        let exit_material = surface.properties.material;
        let exit_penetration_modifier = surface.properties.penetration_modifier;
        let mut final_damage_modifier: f32 = 0.16;
        let mut combined_penetration_modifier: f32 = 0.0;

        if (self.enter_summary.contents & CONTENTS_GRATE as i32) != 0
            || matches!(enter_material, 71 | 89)
        {
            final_damage_modifier = 0.05;
            combined_penetration_modifier = 3.0;
        } else {
            combined_penetration_modifier =
                (enter_penetration_modifier + exit_penetration_modifier) * 0.5;
        }

        if enter_material == exit_material {
            if matches!(exit_material, 85 | 87) {
                combined_penetration_modifier = 3.0;
            } else {
                combined_penetration_modifier = 2.0
            }
        }

        let v34 = (1.0 / combined_penetration_modifier).max(0.0);
        let v35 = self.current_damage * final_damage_modifier
            + v34 * 3.0 * (3.0 / weapon.penetration()).max(0.0) * 1.25;

        let mut thickness = (exit_summary.end - self.enter_summary.end).magnitude();

        thickness = (thickness * thickness * v34) / 24.0;

        let lost_damage = (v35 + thickness).max(0.0);

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

    pub fn simulate_shot(&mut self, weapon: &Weapon) -> bool {
        let global = Global::handle();
        let local_player = match global.local_player() {
            Some(local_player) => local_player,
            None => return false,
        };

        self.penetrate_count = 4;
        self.trace_length = 0.0;
        self.current_damage = weapon.damage();

        while self.penetrate_count > 0 && self.current_damage >= 1.0 {
            self.trace_length_remaining = weapon.range() - self.trace_length;

            let end = self.source + self.direction * Vec3::splat(self.trace_length_remaining);
            let new_end = end + self.direction * Vec3::splat(40.0);

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, end),
                MASK_SHOT as _,
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                MASK_SHOT as _,
                self.filter.as_ref(),
                &mut self.enter_summary,
            );

            global.ray_tracer().trace_mut(
                &Ray::new(self.source, new_end),
                MASK_SHOT as _,
                Some(&local_player.as_entity()),
                &mut self.enter_summary,
            );

            if self.enter_summary.fraction == 1.0 {
                break;
            }

            if self.enter_summary.hit_group.is_hit() {
                return true;
            }

            if !self.handle_bullet_penetration(weapon) {
                break;
            }
        }

        false
    }
}

fn main() -> Result<()> {
    if library::Library::serverbrowser().is_err() {
        println!("waiting for csgo to load");

        while library::Library::serverbrowser().is_err() {
            thread::sleep(Duration::from_millis(500));
        }
    }

    let global = Global::init()?;
    let global2 = global.clone();
    let global3 = global.clone();

    let choked_packets = Choker::new();

    let original_angle = AtomicVec3::new();
    let thirdperson_angle = AtomicVec3::new();
    let thirdperson_angle2 = thirdperson_angle.clone();

    global.on_frame(move |frame| {
        if let Some(local_player) = global2.local_player() {
            match frame {
                Frame::RENDER_START => {
                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(1);
                    global2.physics_timescale().set(0.5);
                    global2.ragdoll_gravity().set(-800.0);

                    // No recoil / no punch.
                    global2.set_aim_punch_angle(local_player.actual_aim_punch_angle());
                    global2.set_view_punch_angle(local_player.actual_view_punch_angle());

                    local_player.set_aim_punch_angle(Vec3::zero());
                    local_player.set_view_punch_angle(Vec3::zero());

                    if global2.input().thirdperson {
                        original_angle.set(local_player.view_angle());
                        local_player.set_view_angle(thirdperson_angle.get());
                    }

                    println!("{:?}", global2.globals());
                    println!("{:?}", global2.input());
                }
                Frame::RENDER_END => {
                    // Restore aim and view punch to not break things.
                    local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    local_player.set_view_punch_angle(global2.view_punch_angle());

                    // Restore local player angle
                    if global2.input().thirdperson {
                        local_player.set_view_angle(original_angle.get());
                    }

                    //global2.draw_model_stats_overlay().set(0);
                    global2.lost_focus_sleep().set(0);
                    global2.physics_timescale().set(1.0);
                    global2.ragdoll_gravity().set(800.0);
                }
                _ => {}
            }
        }

        global2.panorama_blur().set(1);
        global2.cheats().set(1);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        let on_ground = movement.local_player.on_ground();
        let on_ladder = movement.local_player.on_ladder();

        let air_move = !on_ground && !on_ladder;
        let ladder_move = on_ladder;

        if air_move {
            let velocity = movement.local_player.velocity();
            let speed = velocity.magnitude();

            movement.forward_move = 10000.0 / speed;
            movement.side_move = if movement.command_number % 2 == 0 {
                -450.0
            } else {
                450.0
            };

            movement.do_jump = false;
        }

        if choked_packets.count() > 9 {
            movement.send_packet = true;
            choked_packets.reset();
        } else {
            movement.send_packet = false;
            choked_packets.increment();
        }

        let view_angle = global3.engine().view_angle();

        movement.view_angle.x = 89.0;
        movement.view_angle.y = view_angle.y + 180.0;

        if movement.do_attack {
            movement.view_angle = view_angle;
        }

        if ladder_move {
            movement.view_angle.x = -89.0;
        }

        movement.view_angle -= movement.local_player.aim_punch_angle() * Vec3::splat(2.0);

        thirdperson_angle2.set(movement.view_angle);

        let original_forward_move = movement.forward_move;
        let original_side_move = movement.side_move;

        let yaw_delta = view_angle.y - movement.view_angle.y;
        let yaw_delta_90 = yaw_delta + 90.0;
        let yaw_delta = yaw_delta.to_radians();
        let yaw_delta_90 = yaw_delta_90.to_radians();

        let (sin, cos) = yaw_delta.sin_cos();
        let (sin_90, cos_90) = yaw_delta_90.sin_cos();

        movement.forward_move = cos * original_forward_move + cos_90 * original_side_move;
        movement.side_move = sin * original_forward_move + sin_90 * original_side_move;

        movement.view_angle.z = movement.view_angle.z.clamp(-89.0, 89.0);
        movement.view_angle.y %= 360.0;
        movement.view_angle.z = 0.0;

        movement.forward_move = movement.forward_move.clamp(-450.0, 450.0);
        movement.side_move = movement.side_move.clamp(-450.0, 450.0);

        movement
    });

    Ok(())
}

#[ctor::ctor]
fn load() {
    let _ = thread::Builder::new().spawn(move || {
        let _ = main();
    });
}
