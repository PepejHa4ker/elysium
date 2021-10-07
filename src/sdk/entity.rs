use super::netvars::Netvar;
use super::Vector;

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
}

impl Netvar for Entity {
    fn as_ptr(&self) -> *const () {
        self.this
    }
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}
