use crate::frame::Frame;
use crate::global::Global;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: Frame);

pub unsafe extern "C" fn hook(this: *const (), frame: Frame) {
    let global = Global::handle();
    let local_player_index = global.engine().local_player_index();
    let local_player = global.entities().get(local_player_index);

    if let Some(local_player) = local_player {
        *global.local_player_ptr() = Box::new(Some(local_player));
    }

    let on_frame = &*global.on_frame_ptr();

    on_frame(frame);

    global.frame_stage_notify_original(this, frame);
}
