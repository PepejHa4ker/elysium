use super::netvars;
use std::{mem, ptr};
use vmt::PointerExt;

#[derive(Debug)]
pub struct Entity {
    this: *const usize,
}

impl Entity {
    pub unsafe fn from_raw(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub unsafe fn flags_raw_unchecked(&self) -> *const i32 {
        netvars::offset_unchecked::<i32>(self.as_ptr(), "DT_BasePlayer", "m_fFlags")
    }

    pub unsafe fn flags_raw(&self) -> *const i32 {
        if let Some(ptr) = netvars::offset::<i32>(self.as_ptr(), "DT_BasePlayer", "m_fFlags") {
            ptr
        } else {
            ptr::null()
        }
    }

    pub fn flags(&self) -> &i32 {
        unsafe { mem::transmute(self.flags_raw()) }
    }

    pub fn flags_mut(&self) -> &mut i32 {
        unsafe { mem::transmute(self.flags_raw()) }
    }
}

unsafe impl Send for Entity {}
unsafe impl Sync for Entity {}
