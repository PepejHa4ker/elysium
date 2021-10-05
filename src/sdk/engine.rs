use std::mem;

#[derive(Debug)]
pub struct Engine {
    this: *const usize,
}

impl Engine {
    pub unsafe fn from_raw(ptr: *const usize) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const usize {
        self.this
    }

    pub fn local_player_index(&self) -> i32 {
        type Signature = unsafe extern "C" fn(this: *const usize) -> i32;

        let method: Signature = unsafe { mem::transmute(vmt::get(self.as_ptr(), 12)) };

        unsafe { method(self.as_ptr()) }
    }
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}
