use crate::entity::Player;
use crate::global::Global;
use core::mem;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: i32);

pub unsafe extern "C" fn hook(this: *const (), frame: i32) {
    use elysium_state::local;

    let global = Global::handle();
    let engine = &*elysium_state::engine().cast::<elysium_sdk::Engine>();
    let entity_list = global.entity_list();
    let local_player = entity_list.get_unchecked(engine.local_player_index());

    local::set_player(core::ptr::NonNull::new_unchecked(local_player.cast()));

    let local_player = Player::new(local_player);

    *global.local_player_ptr() = Box::new(local_player);

    let on_frame = &*global.on_frame_ptr();

    // TODO: investigate if invalid variants really occur!
    if matches!(frame, 0..=6) {
        let frame = mem::transmute(frame);

        on_frame(frame);
    } else {
        frosting::println!("refused to call on_frame as frame is not a valid variant");
    }

    global.frame_stage_notify_original(this, frame);
}
