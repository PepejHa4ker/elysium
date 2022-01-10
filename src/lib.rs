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
use sdk::Angle;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod islice;
pub mod managed;
pub mod mem;
pub mod networked;

pub mod client;
pub mod command;
pub mod console;
pub mod consts;
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
pub mod item_kind;
pub mod libraries;
pub mod library;
pub mod material;
pub mod model;
pub mod move_kind;
pub mod movement;
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

struct AtomicAngleInner {
    pitch: AtomicF32,
    yaw: AtomicF32,
}

#[derive(Clone)]
pub struct AtomicAngle(Arc<AtomicAngleInner>);

impl AtomicAngle {
    pub fn new() -> Self {
        Self(Arc::new(AtomicAngleInner {
            pitch: AtomicF32::new(0.0),
            yaw: AtomicF32::new(0.0),
        }))
    }

    pub fn pitch(&self) -> f32 {
        self.0.pitch.load(Ordering::SeqCst)
    }

    pub fn yaw(&self) -> f32 {
        self.0.yaw.load(Ordering::SeqCst)
    }

    pub fn set_pitch(&self, pitch: f32) {
        self.0.pitch.store(pitch, Ordering::SeqCst);
    }

    pub fn set_yaw(&self, yaw: f32) {
        self.0.yaw.store(yaw, Ordering::SeqCst);
    }

    pub fn get(&self) -> Angle {
        Angle::new(self.pitch(), self.yaw())
    }

    pub fn set(&self, angle: Angle) {
        self.set_pitch(angle.pitch);
        self.set_yaw(angle.yaw);
    }
}

const CONTENTS_EMPTY: u32 = 0; // No contents

const CONTENTS_SOLID: u32 = 0x1; // an eye is never valid in a solid
const CONTENTS_WINDOW: u32 = 0x2; // translucent, but not watery: u32 = glass;
const CONTENTS_AUX: u32 = 0x4;
const CONTENTS_GRATE: u32 = 0x8; // alpha-tested "grate" textures. Bullets/sight pass through, but solids don't
const CONTENTS_SLIME: u32 = 0x10;
const CONTENTS_WATER: u32 = 0x20;
const CONTENTS_BLOCKLOS: u32 = 0x40; // block AI line of sight
const CONTENTS_OPAQUE: u32 = 0x80; // things that cannot be seen through: u32 = may be non-solid though;

const CONTENTS_TESTFOGVOLUME: u32 = 0x100;
const CONTENTS_UNUSED: u32 = 0x200;

// unused
// NOTE: If it's visible, grab from the top update LAST_VISIBLE_CONTENTS
// if not visible, then grab from the bottom.
// CONTENTS_OPAQUE SURF_NODRAW count as CONTENTS_OPAQUE: u32 = shadow-casting toolsblocklight textures;
const CONTENTS_BLOCKLIGHT: u32 = 0x400;

const CONTENTS_TEAM1: u32 = 0x800; // per team contents used to differentiate collisions
const CONTENTS_TEAM2: u32 = 0x1000; // between players and objects on different teams

// ignore CONTENTS_OPAQUE on surfaces that have SURF_NODRAW
const CONTENTS_IGNORE_NODRAW_OPAQUE: u32 = 0x2000;

// hits entities which are MOVETYPE_PUSH: u32 = doors, plats, etc.;
const CONTENTS_MOVEABLE: u32 = 0x4000;

// remaining contents are non-visible, and don't eat brushes
const CONTENTS_AREAPORTAL: u32 = 0x8000;

const CONTENTS_PLAYERCLIP: u32 = 0x10000;
const CONTENTS_MONSTERCLIP: u32 = 0x20000;

// currents can be added to any other contents, and may be mixed
const CONTENTS_CURRENT_0: u32 = 0x40000;
const CONTENTS_CURRENT_90: u32 = 0x80000;
const CONTENTS_CURRENT_180: u32 = 0x100000;
const CONTENTS_CURRENT_270: u32 = 0x200000;
const CONTENTS_CURRENT_UP: u32 = 0x400000;
const CONTENTS_CURRENT_DOWN: u32 = 0x800000;

const CONTENTS_ORIGIN: u32 = 0x1000000; // removed before bsping an entity

const CONTENTS_MONSTER: u32 = 0x2000000; // should never be on a brush, only in game
const CONTENTS_DEBRIS: u32 = 0x4000000;
const CONTENTS_DETAIL: u32 = 0x8000000; // brushes to be added after vis leafs
const CONTENTS_TRANSLUCENT: u32 = 0x10000000; // auto set if any surface has trans
const CONTENTS_LADDER: u32 = 0x20000000;
const CONTENTS_HITBOX: u32 = 0x40000000; // use accurate hitboxes on trace

// NOTE: These are stored in a short in the engine now. Don't use more than 16 bits
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
// everything that is normally solid
const MASK_SOLID: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER | CONTENTS_GRATE;
// everything that blocks player movement
const MASK_PLAYERSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_PLAYERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;

// blocks nc movement
const MASK_NPCSOLID: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTERCLIP
    | CONTENTS_WINDOW
    | CONTENTS_MONSTER
    | CONTENTS_GRATE;

// blocks fluid movement
const MASK_NPCFLUID: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_MONSTERCLIP | CONTENTS_WINDOW | CONTENTS_MONSTER;

// water physics in these contents
const MASK_WATER: u32 = CONTENTS_WATER | CONTENTS_MOVEABLE | CONTENTS_SLIME;
// everything that blocks lighting
const MASK_OPAQUE: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_OPAQUE;
// everything that blocks lighting, but with monsters added.
const MASK_OPAQUE_AND_NPCS: u32 = MASK_OPAQUE | CONTENTS_MONSTER;
// everything that blocks line of sight for AI
const MASK_BLOCKLOS: u32 = CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_BLOCKLOS;
// everything that blocks line of sight for AI plus NPCs
const MASK_BLOCKLOS_AND_NPCS: u32 = MASK_BLOCKLOS | CONTENTS_MONSTER;
// everything that blocks line of sight for players
const MASK_VISIBLE: u32 = MASK_OPAQUE | CONTENTS_IGNORE_NODRAW_OPAQUE;
// everything that blocks line of sight for players, but with monsters added.
const MASK_VISIBLE_AND_NPCS: u32 = MASK_OPAQUE_AND_NPCS | CONTENTS_IGNORE_NODRAW_OPAQUE;
// bullets see these as solid
const MASK_SHOT: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_HITBOX;

// bullets see these as solid, except monsters: u32 = world+brush only;
const MASK_SHOT_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_DEBRIS;

// non-raycasted weapons see this as solid: u32 = includes grates;
const MASK_SHOT_HULL: u32 = CONTENTS_SOLID
    | CONTENTS_MOVEABLE
    | CONTENTS_MONSTER
    | CONTENTS_WINDOW
    | CONTENTS_DEBRIS
    | CONTENTS_GRATE;

// hits solids: u32 = not grates; and passes through everything else
const MASK_SHOT_PORTAL: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTER;

// everything normally solid, except monsters: u32 = world+brush only;
const MASK_SOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_GRATE;

// everything normally solid for player movement, except monsters: u32 = world+brush only;
const MASK_PLAYERSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_PLAYERCLIP | CONTENTS_GRATE;

// everything normally solid for npc movement, except monsters: u32 = world+brush only;
const MASK_NPCSOLID_BRUSHONLY: u32 =
    CONTENTS_SOLID | CONTENTS_MOVEABLE | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;

// just the world, used for route rebuilding
const MASK_NPCWORLDSTATIC: u32 =
    CONTENTS_SOLID | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP | CONTENTS_GRATE;

// just the world, used for route rebuilding
const MASK_NPCWORLDSTATIC_FLUID: u32 = CONTENTS_SOLID | CONTENTS_WINDOW | CONTENTS_MONSTERCLIP;

// These are things that can split areaportals
const MASK_SPLITAREAPORTAL: u32 = CONTENTS_WATER | CONTENTS_SLIME;

// UNDONE: This is untested, any moving water
const MASK_CURRENT: u32 = CONTENTS_CURRENT_0
    | CONTENTS_CURRENT_90
    | CONTENTS_CURRENT_180
    | CONTENTS_CURRENT_270
    | CONTENTS_CURRENT_UP
    | CONTENTS_CURRENT_DOWN;

// everything that blocks corpse movement
// UNDONE: Not used yet; / may be deleted
const MASK_DEADSOLID: u32 = CONTENTS_SOLID | CONTENTS_PLAYERCLIP | CONTENTS_WINDOW | CONTENTS_GRATE;

use crate::entity::Entity;
use crate::trace::{Ray, Summary};
use sdk::Vector;

fn trace_to_exit(
    start: Vector,
    direction: Vector,
    enter_summary: &Summary,
    exit_summary: &mut Summary,
    end: &mut Vector,
) -> bool {
    let global = Global::handle();
    let mut distance = 0.0;

    while distance <= 90.0 {
        distance += 4.0;
        *end = start + direction * distance;

        let contents = global.ray_tracer().point_contents(
            *end,
            (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
            ptr::null(),
        );

        if (contents & MASK_SHOT_HULL as i32) != 0 && (contents & CONTENTS_HITBOX as i32) != 0 {
            continue;
        }

        let new_end = *end - (direction * 4.0);

        global.ray_tracer().trace_mut(
            &Ray::new(*end, new_end),
            (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
            None,
            exit_summary,
        );

        if exit_summary.start_solid && (exit_summary.surface.flags & SURF_HITBOX as u16) != 0 {
            let skip_entity = match exit_summary.entity_hit.as_ref() {
                Some(entity) => {
                    Some(unsafe { Entity::new_unchecked(entity.as_ptr() as *mut handle::Entity) })
                }
                None => None,
            };

            global.ray_tracer().trace_mut(
                &Ray::new(*end, start),
                (MASK_SHOT_HULL | CONTENTS_HITBOX) as _,
                skip_entity.as_ref(),
                exit_summary,
            );

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
            if let None = exit_summary.entity_hit {
                return true;
            }

            continue;
        }

        if (exit_summary.surface.flags & SURF_NODRAW as u16) != 0 {
            continue;
        }

        if exit_summary.plane.normal.dot(direction) <= 1.0 {
            *end = *end - (direction * (exit_summary.fraction * 4.0));

            return true;
        }
    }

    false
}

pub struct ShotData {
    pub source: Vector,
    pub enter_summary: Summary,
    pub direction: Vector,
    pub filter: Option<Entity>,
    pub trace_length: f32,
    pub trace_length_remaining: f32,
    pub current_damage: f32,
    pub penetrate_count: i32,
}

impl ShotData {
    pub fn new() -> Self {
        Self {
            source: Vector::zero(),
            enter_summary: Summary::new(),
            direction: Vector::zero(),
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
            .query(self.enter_summary.surface.index as i32)
        {
            Some(surface) => surface,
            None => return true,
        };

        let enter_material = surface.material;
        let enter_penetration_modifier = surface.penetration_modifier;

        self.trace_length += self.trace_length_remaining * self.enter_summary.fraction;
        self.current_damage *= weapon.range_modifier().powf(self.trace_length * 0.002);

        if self.trace_length > 3000.0 || enter_penetration_modifier < 0.1 {
            self.penetrate_count = 0;
        }

        if self.penetrate_count <= 0 {
            return false;
        }

        let mut end = Vector::zero();
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

        let surface = match global.physics().query(exit_summary.surface.index as _) {
            Some(surface) => surface,
            None => return true,
        };

        let exit_material = surface.material;
        let exit_penetration_modifier = surface.penetration_modifier;
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

            let end = self.source + self.direction * self.trace_length_remaining;
            let new_end = end + self.direction * 40.0;

            println!("before tracing");

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

    let thirdperson_angle = AtomicAngle::new();
    let thirdperson_angle2 = thirdperson_angle.clone();

    global.on_frame(move |frame| {
        if let Some(local_player) = global2.local_player() {
            println!("{:?}", frame);

            match frame {
                Frame::RENDER_START => {
                    // no recoil / punch
                    global2.set_aim_punch_angle(local_player.actual_aim_punch_angle());
                    global2.set_view_punch_angle(local_player.actual_view_punch_angle());

                    local_player.set_aim_punch_angle(Angle::zero());
                    local_player.set_view_punch_angle(Angle::zero());

                    if global2.input().thirdperson {
                        local_player.set_view_angle(thirdperson_angle.get());
                    }
                }
                Frame::RENDER_END => {
                    // restore no recoil / punch
                    local_player.set_aim_punch_angle(global2.aim_punch_angle());
                    local_player.set_view_punch_angle(global2.view_punch_angle());
                }
                _ => {}
            }
        }

        global2.cheats().set(1);
        global2.lost_focus_sleep().set(1);
        global2.panorama_blur().set(1);
        global2.physics_timescale().set(0.5);
        global2.ragdoll_gravity().set(-800.0);
        global2.show_impacts().set(2);
    });

    global.on_move(move |mut movement| {
        let max_desync = movement.local_player.max_desync_angle();
        let max_desync = 270.0;
        let eye_yaw_on_send = global3.engine().view_angle().yaw;
        let eye_yaw_on_choke = eye_yaw_on_send + (max_desync * 2.0);
        let real_yaw = eye_yaw_on_send + max_desync;
        let fake_yaw = eye_yaw_on_send - max_desync;

        movement.do_fast_duck = movement.do_duck;

        if !movement.local_player.on_ground() {
            movement.do_jump = false;
        }

        let velocity = movement.local_player.velocity();

        if choked_packets.count() > 1 {
            movement.send_packet = true;
            choked_packets.reset();
        } else {
            movement.send_packet = false;
            choked_packets.increment();
        }

        if !(movement.do_attack
            || movement.local_player.on_ladder()
            || movement.local_player.is_noclip())
        {
            if movement.send_packet {
                movement.view_angle.yaw = eye_yaw_on_send;
            } else {
                movement.view_angle.yaw = eye_yaw_on_choke;
            }

            if movement.side_move.abs() < 5.0 {
                if movement.do_duck {
                    movement.side_move = if (movement.tick_count & 1) != 0 {
                        3.25
                    } else {
                        -3.25
                    };
                } else {
                    movement.side_move = if (movement.tick_count & 1) != 0 {
                        1.1
                    } else {
                        -1.1
                    };
                }
            }
        }

        movement.view_angle = movement.view_angle - (movement.local_player.aim_punch_angle() * 2.0);

        /*print!(
                    "max_desync_angle={:.2?} ",
                    movement.local_player.max_desync_angle()
                );
                print!("money={:.2?} ", movement.local_player.money());
                print!("observer={:.2?} ", movement.local_player.observer());
                print!("on_ground={:.2?} ", movement.local_player.on_ground());
                print!("on_ladder={:.2?} ", movement.local_player.on_ladder());
                print!(
                    "partially_on_ground={:.2?} ",
                    movement.local_player.partially_on_ground()
                );
                print!("speed={:.2?} ", movement.local_player.speed());
                print!("tick_base={:.2?} ", movement.local_player.tick_base());
                print!("velocity={:.2?} ", movement.local_player.velocity());
                print!("view_angle={:.2?} ", movement.local_player.view_angle());
                print!("view_offset={:.2?} ", movement.local_player.view_offset());
        */

        if let Some(weapon) = movement.local_player.weapon() {
            let mut shot_data = ShotData::new();
            // todo fix segfault
            //let damage = shot_data.simulate_shot(&weapon);

            //println!("{damage:?}");

            /*global3.ray_tracer().trace(
                &Ray::new(Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
                MASK_SHOT as _,
                Some(&movement.local_player.as_entity()),
            );*/

            if movement.do_attack == true {
                if let Some(time) = weapon.revolver_cock_time() {
                    if time - 1.0 < movement.client_time {
                        movement.do_attack = false;
                    }
                }
            }
        }

        thirdperson_angle2.set(Angle {
            yaw: if movement.do_attack {
                movement.view_angle.yaw
            } else {
                fake_yaw
            },
            ..movement.view_angle
        });

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
