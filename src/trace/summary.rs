use super::{Plane, Surface};
use crate::contents::Contents;
use crate::hit_group::HitGroup;
use crate::managed::handle;
use elysium_math::Vec3;

/// Ray trace summary.
#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Summary {
    pub start: Vec3,
    pub end: Vec3,

    pub plane: Plane,

    pub fraction: f32,

    pub contents: Contents,
    pub disp_flags: u32,

    pub all_solid: bool,
    pub start_solid: bool,

    pub fraction_left_solid: f32,

    pub surface: Surface,

    pub hit_group: HitGroup,
    pub physics_bone: i32,

    pub world_surface_index: u16,

    pub entity_hit: *const handle::Entity,

    pub hitbox: i32,
}

impl Summary {
    pub(crate) fn new() -> Self {
        Self {
            start: Vec3::zero(),
            end: Vec3::zero(),
            plane: Plane::new(),
            fraction: 0.0,
            contents: Contents::new(),
            disp_flags: 0,
            all_solid: false,
            start_solid: false,
            fraction_left_solid: 0.0,
            surface: Surface::new(),
            hit_group: HitGroup::Generic,
            physics_bone: 0,
            world_surface_index: 0,
            entity_hit: core::ptr::null::<()>() as *const _,
            hitbox: 0,
        }
    }
}
