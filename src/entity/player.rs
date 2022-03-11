use super::{Entity, Weapon};
use crate::global::Global;
use crate::managed::handle;
use crate::model::Model;
use core::cmp;
use providence_math::Vec3;
use sdk::{AnimationLayer, AnimationState};

const DUCKING: i32 = 1 << 1;
const IN_WATER: i32 = 1 << 9;
const ON_GROUND: i32 = 1 << 0;
const PARTIALLY_ON_GROUND: i32 = 1 << 17;
const WATER_JUMPING: i32 = 1 << 2;

/// A player.
#[derive(Debug)]
#[repr(transparent)]
pub struct Player(Entity);

impl Player {
    pub fn new(ptr: *mut handle::Entity) -> Option<Self> {
        Some(Self(Entity::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Entity) -> Self {
        Self(Entity::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Entity {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub fn as_entity(&self) -> &Entity {
        &self.0
    }

    /// Returns the player's flags.
    fn flags(&self) -> i32 {
        unsafe { self.relative_entry(Global::handle().networked().player.flags) }
    }

    /// If the player's flags has `flag` set.
    fn has_flag(&self, flag: i32) -> bool {
        (self.flags() & flag) != 0
    }

    /// Player's movement kind.
    fn move_kind(&self) -> i32 {
        self.0.move_kind()
    }

    /// Is this player in water.
    pub fn in_water(&self) -> bool {
        self.has_flag(IN_WATER)
    }

    /// Index of this player in the engine.
    pub fn index(&self) -> i32 {
        self.0.index()
    }

    /// Is this player ducking.
    pub fn is_ducking(&self) -> bool {
        self.has_flag(DUCKING)
    }

    /// Is this player dormant.
    pub fn is_dormant(&self) -> bool {
        self.0.is_dormant()
    }

    /// Is this player jumping in water.
    pub fn is_water_jumping(&self) -> bool {
        self.has_flag(WATER_JUMPING)
    }

    /// Is this player on the ground.
    pub fn on_ground(&self) -> bool {
        self.has_flag(ON_GROUND)
    }

    /// If this player is on a ladder.
    pub fn on_ladder(&self) -> bool {
        self.move_kind() == 9
    }

    /// Is this player partially on the ground.
    pub fn partially_on_ground(&self) -> bool {
        self.has_flag(PARTIALLY_ON_GROUND)
    }

    /// Is this player noclipping.
    pub fn is_noclip(&self) -> bool {
        self.move_kind() == 8
    }

    pub fn observer(&self) -> *mut Entity {
        unsafe { self.relative_entry(Global::handle().networked().player.observer) }
    }

    /// Is this player spectating another.
    pub fn is_spectating(&self) -> bool {
        self.observer().is_null()
    }

    /// Get the player's velocity vector.
    pub fn velocity(&self) -> Vec3 {
        unsafe { self.relative_entry(Global::handle().networked().base_player.velocity) }
    }

    /// Get the player's speed (magnitude of velocity vector).
    pub fn speed(&self) -> f32 {
        self.velocity().magnitude()
    }

    fn is_dead_ptr(&self) -> *const u8 {
        unsafe {
            self.relative_offset(Global::handle().networked().base_player.is_dead) as *const u8
        }
    }

    fn view_angle_ptr(&self) -> *const Vec3 {
        unsafe { self.is_dead_ptr().add(4) as *const Vec3 }
    }

    /// Get the player's view angle.
    pub fn view_angle(&self) -> Vec3 {
        unsafe { *self.view_angle_ptr() }
    }

    /// Set the player's view angle.
    pub fn set_view_angle(&self, angle: Vec3) {
        unsafe {
            *(self.view_angle_ptr() as *mut Vec3) = angle;
        }
    }

    pub fn has_helmet(&self) -> bool {
        unsafe { self.relative_entry(Global::handle().networked().player.has_helmet) }
    }

    pub fn is_immune(&self) -> bool {
        unsafe { self.relative_entry(Global::handle().networked().player.is_immune) }
    }

    pub fn lower_body_yaw(&self) -> f32 {
        unsafe { self.relative_entry(Global::handle().networked().player.lower_body_yaw) }
    }

    pub fn is_dead(&self) -> bool {
        unsafe { *(self.is_dead_ptr() as *const bool) }
    }

    pub fn is_scoped(&self) -> bool {
        unsafe { self.relative_entry(Global::handle().networked().player.is_scoped) }
    }

    pub fn eye_angle(&self) -> Vec3 {
        unsafe { self.relative_entry(Global::handle().networked().player.eye_angle) }
    }

    pub fn eye_origin(&self) -> Vec3 {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> Vec3;

        unsafe { self.virtual_entry::<Fn>(348)(self.as_ptr()) }
    }

    pub fn view_offset(&self) -> Vec3 {
        unsafe { self.relative_entry(Global::handle().networked().base_player.view_offset) }
    }

    pub fn has_defuse_kit(&self) -> bool {
        unsafe { self.relative_entry(Global::handle().networked().player.has_defuse_kit) }
    }

    pub fn model(&self) -> Option<&Model> {
        self.0.model()
    }

    pub fn origin(&self) -> Vec3 {
        self.0.origin()
    }

    // Returns a pointer to the aim punch angle.
    pub(crate) fn aim_punch_angle_ptr(&self) -> *mut Vec3 {
        unsafe {
            self.relative_offset(Global::handle().networked().base_player.aim_punch_angle)
                as *mut Vec3
        }
    }

    // Returns the real aim punch angle.
    pub(crate) fn actual_aim_punch_angle(&self) -> Vec3 {
        unsafe { *self.aim_punch_angle_ptr() }
    }

    // Returns a "fixed" aim punch angle.
    pub fn aim_punch_angle(&self) -> Vec3 {
        let global = Global::handle();

        if let Some(local_player) = global.local_player() {
            if self == local_player {
                global.aim_punch_angle()
            } else {
                self.actual_aim_punch_angle()
            }
        } else {
            self.actual_aim_punch_angle()
        }
    }

    // Set the aim punch angle.
    pub fn set_aim_punch_angle(&self, angle: Vec3) {
        unsafe {
            *self.aim_punch_angle_ptr() = angle;
        }
    }

    // Returns a pointer to view punch angle.
    pub(crate) fn view_punch_angle_ptr(&self) -> *mut Vec3 {
        unsafe {
            self.relative_offset(Global::handle().networked().base_player.view_punch_angle)
                as *mut Vec3
        }
    }

    // Returns the real view punch angle.
    pub fn actual_view_punch_angle(&self) -> Vec3 {
        unsafe { *self.view_punch_angle_ptr() }
    }

    // Returns a "fixed" view punch angle.
    pub fn view_punch_angle(&self) -> Vec3 {
        let global = Global::handle();

        if let Some(local_player) = global.local_player() {
            if self == local_player {
                global.view_punch_angle()
            } else {
                self.actual_view_punch_angle()
            }
        } else {
            self.actual_view_punch_angle()
        }
    }

    // Set the real view punch angle.
    pub fn set_view_punch_angle(&self, angle: Vec3) {
        unsafe {
            *self.view_punch_angle_ptr() = angle;
        }
    }

    pub fn health(&self) -> i32 {
        unsafe { self.relative_entry(Global::handle().networked().base_player.health) }
    }

    pub fn money(&self) -> i32 {
        unsafe { self.relative_entry(Global::handle().networked().player.money) }
    }

    pub fn tick_base(&self) -> u32 {
        unsafe { self.relative_entry(Global::handle().networked().base_player.tick_base) }
    }

    fn raw_weapon(&self) -> usize {
        unsafe { self.relative_entry(Global::handle().networked().player.weapon) }
    }

    pub fn weapon(&self) -> Option<Weapon> {
        let handle = Global::handle();

        unsafe {
            let raw = handle
                .entity_list()
                .get((self.raw_weapon() & 0xFFF) as i32)?
                .as_ptr() as *mut handle::Entity;

            Some(Weapon::new_unchecked(raw))
        }
    }

    fn client_animate_ptr(&self) -> *const bool {
        unsafe {
            self.relative_offset(
                Global::handle()
                    .networked()
                    .base_animating
                    .client_side_animation,
            ) as *const bool
        }
    }

    pub fn set_client_animate(&self, animate: bool) {
        unsafe {
            *(self.client_animate_ptr() as *mut bool) = animate;
        }
    }

    pub fn animation_layers(&self) -> &mut [AnimationLayer; 13] {
        unsafe { self.relative_entry(Global::handle().animation_layers() as usize) }
    }

    pub fn animation_state(&self) -> *mut AnimationState {
        unsafe { self.relative_entry(Global::handle().animation_state() as usize) }
    }

    pub fn max_desync_angle(&self) -> f32 {
        let animation_state = self.animation_state();

        if animation_state.is_null() {
            return 0.0;
        }

        let animation_state = unsafe { &mut *animation_state };
        let mut yaw_modifier = (animation_state.stop_to_full_running_fraction * -0.3 - 0.2)
            * animation_state.foot_speed.clamp(0.0, 1.0)
            + 1.0;

        if animation_state.duck_amount > 0.0 {
            yaw_modifier += (animation_state.duck_amount
                * animation_state.foot_speed2.clamp(0.0, 1.0))
                * (0.5 - yaw_modifier);
        }

        animation_state.velocity_subtract_y * yaw_modifier
    }
}

impl cmp::PartialEq<Player> for Player {
    fn eq(&self, other: &Player) -> bool {
        self.index() == other.index()
    }
}

impl cmp::PartialEq<Entity> for Player {
    fn eq(&self, other: &Entity) -> bool {
        self.index() == other.index()
    }
}
