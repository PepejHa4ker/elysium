use core::ptr;
use vptr::VirtualMut;

pub mod create_move;
pub mod draw_model_execute;
pub mod frame_stage_notify;

pub struct Hook<T: Copy> {
    address: *const u8,
    original: T,
    replacement: T,
}

impl<T: Copy> Hook<T> {
    pub fn new(address: *const u8, original: T, replacement: T) -> Self {
        Self {
            address,
            original,
            replacement,
        }
    }

    pub fn address(&self) -> *mut u8 {
        self.address as *mut u8
    }

    pub fn replace(&mut self) {
        unsafe {
            self.original = self
                .address()
                .vreplace_protected(ptr::read(&self.replacement), 0);
        }
    }

    pub fn restore(&self) {
        unsafe {
            self.address()
                .vreplace_protected(ptr::read(&self.original), 0);
        }
    }
}

impl<T: Copy> Drop for Hook<T> {
    fn drop(&mut self) {
        self.restore();
    }
}
