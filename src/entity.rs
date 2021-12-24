use super::netvars::Netvar;
use crate::global::Global;
use crate::player_state::PlayerState;
use core::ptr::NonNull;
use sdk::{Angle, AnimationLayer, AnimationState, Vector};

pub use self::id::EntityId;
pub use self::list::{EntityList, RawEntityList};

mod id;
mod list;

extern "C" {
    pub type RawEntity;
}

unsafe impl Send for RawEntity {}
unsafe impl Sync for RawEntity {}

#[derive(Debug)]
#[repr(transparent)]
pub struct Entity(NonNull<RawEntity>);

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

    pub fn flags(&self) -> &PlayerState {
        self.netvar::<PlayerState>("DT_BasePlayer", "m_fFlags")
    }

    pub fn dead_flag(&self) -> &bool {
        self.netvar::<bool>("DT_BasePlayer", "deadflag")
    }

    pub fn tick_base(&self) -> &u32 {
        self.netvar::<u32>("DT_BasePlayer", "m_nTickBase")
    }

    pub fn velocity(&self) -> &Vector {
        self.netvar::<Vector>("DT_BasePlayer", "m_vecVelocity[0]")
    }

    pub fn view_angle(&self) -> &mut Angle {
        unsafe {
            &mut *((self.netvar_raw("DT_BasePlayer", "deadflag") as *const u8).add(4) as *mut Angle)
        }
    }

    pub fn animation_layers(&self) -> &mut [AnimationLayer; 13] {
        unsafe {
            let animlayersptr =
                self.get(Global::handle().animation_layers() as usize) as *mut [AnimationLayer; 13];

            core::mem::transmute(animlayersptr)
        }
    }

    pub fn animation_state(&self) -> Option<&mut AnimationState> {
        unsafe {
            let animstateptrptr =
                self.get(Global::handle().animation_state() as usize) as *mut *mut AnimationState;

            if animstateptrptr.is_null() {
                return None;
            }

            let animstateptr = *animstateptrptr;

            if animstateptr.is_null() {
                return None;
            }

            core::mem::transmute(animstateptr)
        }
    }
}

impl Netvar for Entity {
    fn as_ptr(&self) -> *const () {
        Entity::as_ptr(self) as _
    }
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}
