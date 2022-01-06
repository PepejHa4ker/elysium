use crate::global::Global;
use crate::netvars::Netvar;
use core::ptr::NonNull;
use sdk::{Angle, AnimationLayer, AnimationState, Vector};

pub use self::id::EntityId;
pub use self::list::{EntityList, Iter, RawEntityList};
pub use self::weapon::{RawWeapon, Weapon};

mod id;
mod list;
mod weapon;

extern "C" {
    /// Raw handle to an entity.
    pub type RawEntity;
}

unsafe impl Send for RawEntity {}
unsafe impl Sync for RawEntity {}

/// An entity.
#[derive(Debug)]
#[repr(transparent)]
pub struct Entity(NonNull<RawEntity>);

const PLAYER: &str = "DT_CSPlayer";
const BASE_PLAYER: &str = "DT_BasePlayer";
const BASE_ENTITY: &str = "DT_BaseEntity";
const BASE_ANIMATING: &str = "DT_BaseAnimating";

const DUCKING: i32 = 1 << 1;
const IN_WATER: i32 = 1 << 9;
const ON_GROUND: i32 = 1 << 0;
const PARTIALLY_ON_GROUND: i32 = 1 << 18;
const WATER_JUMPING: i32 = 1 << 2;

impl Entity {
    pub const fn from_raw(raw: *mut RawEntity) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawEntity) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawEntity {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const *const u8 {
        unsafe { *(self.as_ptr() as *const *const *const u8) }
    }

    pub unsafe fn get(&self, offset: usize) -> *const u8 {
        (self.as_ptr() as *const u8).add(offset)
    }

    /// Entity's flags.
    #[inline]
    fn flags(&self) -> i32 {
        *self.netvar(BASE_PLAYER, "m_fFlags")
    }

    /// Entity's movement kind.
    #[inline]
    pub fn move_kind(&self) -> i32 {
        unsafe {
            *((self.netvar_raw(BASE_PLAYER, "m_nRenderMode") as *const u8).add(1) as *const i32)
        }
    }

    #[inline]
    fn has_flag(&self, flag: i32) -> bool {
        (self.flags() & flag) != 0
    }

    /// Is this entity ducking.
    #[inline]
    pub fn is_ducking(&self) -> bool {
        self.has_flag(DUCKING)
    }

    /// Is this entity in water.
    #[inline]
    pub fn in_water(&self) -> bool {
        self.has_flag(IN_WATER)
    }

    /// Is this entity jumping in water.
    #[inline]
    pub fn is_water_jumping(&self) -> bool {
        self.has_flag(WATER_JUMPING)
    }

    /// Is this entity on the ground.
    #[inline]
    pub fn on_ground(&self) -> bool {
        self.has_flag(ON_GROUND)
    }

    /// If this entity is on a ladder.
    #[inline]
    pub fn on_ladder(&self) -> bool {
        self.move_kind() == 9
    }

    /// Is this entity partially on the ground.
    #[inline]
    pub fn partially_on_ground(&self) -> bool {
        self.has_flag(PARTIALLY_ON_GROUND)
    }

    /// Is this entity noclipping.
    #[inline]
    pub fn is_noclip(&self) -> bool {
        self.move_kind() == 8
    }

    pub fn observer(&self) -> *mut Entity {
        unsafe { *self.netvar::<*mut Entity>(PLAYER, "m_hObserverTarget") }
    }

    /// Is this entity spectating another.
    #[inline]
    pub fn is_spectating(&self) -> bool {
        self.observer().is_null()
    }

    /// Get the entity's velocity vector.
    pub fn velocity(&self) -> &Vector {
        self.netvar::<Vector>(BASE_PLAYER, "m_vecVelocity[0]")
    }

    /// Get the entity's speed (magnitude of velocity vector).
    pub fn speed(&self) -> f32 {
        self.velocity().magnitude()
    }

    fn is_dead_ptr(&self) -> *const u8 {
        unsafe { self.netvar_raw(BASE_PLAYER, "deadflag") as *const u8 }
    }

    fn view_angle_ptr(&self) -> *const Angle {
        unsafe { self.is_dead_ptr().add(4) as *const Angle }
    }

    /// Get the entity's view angle.
    pub fn view_angle(&self) -> Angle {
        unsafe { *self.view_angle_ptr() }
    }

    /// Set the entity's view angle.
    pub fn set_view_angle(&self, angle: Angle) {
        unsafe {
            *(self.view_angle_ptr() as *mut Angle) = angle;
        }
    }

    pub fn has_helmet(&self) -> bool {
        *self.netvar(PLAYER, "m_bHasHelmet")
    }

    pub fn is_immune(&self) -> bool {
        !*self.netvar::<bool>(PLAYER, "m_bGunGameImmunity")
    }

    pub fn lower_body_yaw(&self) -> f32 {
        *self.netvar(PLAYER, "m_flLowerBodyYawTarget")
    }

    pub fn is_dead(&self) -> bool {
        unsafe { *(self.is_dead_ptr() as *const bool) }
    }

    pub fn is_scoped(&self) -> bool {
        *self.netvar(PLAYER, "m_bIsScoped")
    }

    pub fn lower_body_yaw_target(&self) -> f32 {
        *self.netvar(PLAYER, "m_flLowerBodyYawTarget")
    }

    pub fn eye_angle(&self) -> Angle {
        *self.netvar(PLAYER, "m_angEyeAngles[0]")
    }

    pub fn view_offset(&self) -> Vector {
        *self.netvar(PLAYER, "m_vecViewOffset[0]")
    }

    pub fn has_defuse_kit(&self) -> bool {
        *self.netvar(PLAYER, "m_bHasDefuser")
    }

    pub fn aim_punch(&self) -> Angle {
        *self.netvar(PLAYER, "m_aimPunchAngle")
    }

    pub fn view_punch(&self) -> Angle {
        *self.netvar(PLAYER, "m_viewPunchAngle")
    }

    pub fn health(&self) -> i32 {
        *self.netvar(PLAYER, "m_iHealth")
    }

    pub fn money(&self) -> i32 {
        *self.netvar(PLAYER, "m_iAccount")
    }

    pub fn tick_base(&self) -> u32 {
        *self.netvar(PLAYER, "m_nTickBase")
    }

    fn raw_weapon(&self) -> usize {
        *self.netvar(PLAYER, "m_hActiveWeapon")
    }

    pub fn weapon(&self) -> Option<Weapon> {
        let handle = Global::handle();

        unsafe {
            let raw = handle
                .entity_list()
                .get((self.raw_weapon() & 0xFFF) as i32)?
                .as_ptr() as *mut RawWeapon;

            Some(Weapon::from_raw_unchecked(raw))
        }
    }

    fn client_animate_ptr(&self) -> *const bool {
        unsafe { self.netvar_raw(BASE_ANIMATING, "m_bClientSideAnimation") as *const bool }
    }

    pub fn set_client_animate(&self, animate: bool) {
        unsafe {
            *(self.client_animate_ptr() as *mut bool) = animate;
        }
    }

    pub fn is_player(&self) -> bool {
        unsafe {
            type IsPlayer = unsafe extern "C" fn(this: *const RawEntity) -> bool;

            let is_player: IsPlayer = core::mem::transmute(self.get(157));

            is_player(self.as_ptr())
        }
    }

    pub fn animation_layers(&self) -> &mut [AnimationLayer; 13] {
        unsafe {
            let animlayersptr =
                self.get(Global::handle().animation_layers() as usize) as *mut [AnimationLayer; 13];

            core::mem::transmute(animlayersptr)
        }
    }

    pub fn animation_state(&self) -> *mut AnimationState {
        unsafe {
            *(self.get(Global::handle().animation_state() as usize) as *mut *mut AnimationState)
        }
    }

    pub fn max_desync_angle(&self) -> f32 {
        let animation_state = self.animation_state();

        if animation_state.is_null() {
            return 0.0;
        }

        let animation_state = unsafe { &mut *animation_state };

        println!("{animation_state:?}");

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

impl Netvar for Entity {
    fn as_ptr(&self) -> *const () {
        Entity::as_ptr(self) as _
    }
}
