use super::netvars::Netvar;
use crate::animation_layer::AnimationLayer;
use crate::global::Global;
use crate::player_state::PlayerState;
use sdk::{Angle, Vector};

#[derive(Debug)]
pub struct Entity {
    pub this: *const (),
}

impl Entity {
    pub const unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
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
            &mut *((self.as_ptr() as *const u8).add(Global::handle().animation_layers() as usize)
                as *mut [AnimationLayer; 13])
        }
    }
}

impl Netvar for Entity {
    fn as_ptr(&self) -> *const () {
        self.this
    }
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}
