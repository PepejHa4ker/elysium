use crate::entity::Player;
use crate::frame::Frame;
use crate::global::Global;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: Frame);

pub unsafe extern "C" fn hook(this: *const (), frame: Frame) {
    let global = Global::handle();
    let engine = &*elysium_state::engine().cast::<elysium_sdk::Engine>();
    let entity_list = global.entity_list();

    let local_player = entity_list.get_unchecked(engine.local_player_index());
    let local_player = Player::new(local_player);

    *global.local_player_ptr() = Box::new(local_player);

    let on_frame = &*global.on_frame_ptr();

    on_frame(frame);

    global.frame_stage_notify_original(this, frame);
}
