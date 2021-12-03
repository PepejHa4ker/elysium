use super::netvars::Netvar;
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

    pub fn velocity(&self) -> &Vector {
        self.netvar::<Vector>("DT_BasePlayer", "m_vecVelocity[0]")
    }

    pub fn dead_flag(&self) -> &bool {
        self.netvar::<bool>("DT_BasePlayer", "deadflag")
    }

    pub fn view_angle(&self) -> &mut Angle {
        unsafe {
            &mut *((self.netvar_raw("DT_BasePlayer", "deadflag") as *const u8).add(4) as *mut Angle)
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
