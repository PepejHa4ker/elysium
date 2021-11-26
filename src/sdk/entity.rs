use super::netvars::Netvar;
use crate::angle::Angle;
use crate::vector::Vector;

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

    pub fn flags(&self) -> &i32 {
        self.netvar::<i32>("DT_BasePlayer", "m_fFlags")
    }

    pub fn velocity(&self) -> &Vector {
        self.netvar::<Vector>("DT_BasePlayer", "m_vecVelocity[0]")
    }

    pub fn dead_flag(&self) -> &bool {
        self.netvar::<bool>("DT_BasePlayer", "deadflag")
    }

    pub fn view_angle(&self) -> Angle {
        unsafe {
            *((self.netvar_raw("DT_BasePlayer", "deadflag") as *const u8).add(4) as *const Angle)
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
