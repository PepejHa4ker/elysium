use crate::{globals, sdk};
use std::lazy::SyncOnceCell;
use std::mem;

pub type Signature = unsafe extern "C" fn(event: *mut ()) -> i32;

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original_unchecked(event: *mut ()) -> i32 {
    let original = *ORIGINAL.get().unwrap_unchecked();

    original(event)
}

pub fn set_original(original: *const ()) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe extern "C" fn hook(event: *mut ()) -> i32 {
    globals::console().write("poll_event\n");

    original_unchecked(event)
}
