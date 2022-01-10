use crate::entity::Player;
use crate::frame::Frame;
use crate::global::Global;
use crate::managed::handle;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: Frame);

pub unsafe extern "C" fn hook(this: *const (), frame: Frame) {
    let global = Global::handle();
    let local_player = global.entity_list().get(global.engine().local_player());

    if let Some(local_player) = local_player {
        let local_player = Player::new_unchecked(local_player.as_ptr() as *mut handle::Entity);

        *global.local_player_ptr() = Box::new(Some(local_player));
    }

    let on_frame = &*global.on_frame_ptr();

    on_frame(frame);

    global.frame_stage_notify_original(this, frame);
}
