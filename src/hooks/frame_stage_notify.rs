use crate::util::FloatExt;
use crate::{globals, sdk};
use std::lazy::SyncOnceCell;
use std::mem;

pub type Signature = unsafe extern "C" fn(this: *const (), stage: i32);

pub static ORIGINAL: SyncOnceCell<Signature> = SyncOnceCell::new();

pub unsafe fn original(this: *const (), raw_stage: i32) {
    let original = *ORIGINAL.get().unwrap_unchecked();

    original(this, raw_stage);
}

pub fn set_original(original: *const ()) {
    let _ = unsafe { ORIGINAL.set(mem::transmute::<_, Signature>(original)) };
}

pub unsafe extern "C" fn hook(this: *const (), raw_stage: i32) {
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

    if let Some(local_player) = globals::local_player() {
        local_player.view_angle().pitch = f32::down();
    }

    original(this, raw_stage);
}
