use super::{netvars, Vector};
use std::{mem, ptr};
use vmt::PointerExt;

#[derive(Debug)]
pub struct Entity {
    pub this: *const usize,
}

impl Entity {
    pub const unsafe fn from_raw(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub unsafe fn flags_raw(&self) -> *const i32 {
        netvars::offset::<i32>(self.as_ptr(), "DT_BasePlayer", "m_fFlags")
    }

    pub fn flags(&self) -> &i32 {
        unsafe { mem::transmute(self.flags_raw()) }
    }

    pub fn flags_mut(&mut self) -> &mut i32 {
        unsafe { mem::transmute(self.flags_raw()) }
    }

    pub unsafe fn velocity_raw(&self) -> *const Vector {
        netvars::offset::<Vector>(self.as_ptr(), "DT_BasePlayer", "m_vecVelocity[0]")
    }

    pub fn velocity(&self) -> &Vector {
        unsafe { mem::transmute(self.velocity_raw()) }
    }

    pub fn velocity_mut(&mut self) -> &mut Vector {
        unsafe { mem::transmute(self.velocity_raw()) }
    }
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}
