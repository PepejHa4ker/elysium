use core::mem;
use vptr::Virtual;

#[derive(Debug)]
pub struct Engine {
    this: *const (),
}

impl Engine {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn local_player_index(&self) -> i32 {
        type Signature = unsafe extern "C" fn(this: *const ()) -> i32;

        let method: Signature = unsafe { self.as_ptr().vget(12 * 8) };

        unsafe { method(self.as_ptr()) }
    }
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}
