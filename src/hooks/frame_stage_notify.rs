use crate::entity::Player;
use crate::frame::Frame;
use crate::global::Global;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: Frame);

pub unsafe extern "C" fn hook(this: *const (), frame: Frame) {
    let global = Global::handle();
    let local_player = unsafe {
        let ptr = global
            .entity_list()
            .get_unchecked(global.engine().local_player());

        Player::new(ptr)
    };

    *global.local_player_ptr() = Box::new(local_player);

    let on_frame = &*global.on_frame_ptr();

    on_frame(frame);

    global.frame_stage_notify_original(this, frame);
}
