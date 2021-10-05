use crate::{globals, sdk};
use std::lazy::SyncOnceCell;
use std::mem;

pub type Signature = unsafe extern "C" fn(this: *const usize, stage: i32);

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original() -> Signature {
    *ORIGINAL.get().unwrap_unchecked()
}

pub fn set_original(original: *const usize) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe extern "C" fn hook(this: *const usize, raw_stage: i32) {
    let stage = sdk::FrameStage::new(raw_stage);

    //globals::console().write(format!("stage = {:?}\n", &stage));

    if stage.is_none() {
        tracing::trace!("unknown FrameStage code: {}", raw_stage);
    }

    let local_player_index = globals::engine().local_player_index();

    //globals::console().write(format!("local_player_index = {:?}\n", &local_player_index));

    let local_player = globals::entities().get(local_player_index);

    //globals::console().write(format!("local_player = {:?}\n", &local_player));

    if let Some(local_player) = local_player {
        globals::set_local_player(local_player);
    }

    let original = original();

    original(this, raw_stage);
}
