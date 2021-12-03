use crate::frame::Frame;
use crate::globals;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: Frame);

pub unsafe extern "C" fn hook(this: *const (), frame: Frame) {
    tracing::info!("frame_stage_notify hook");

    let global = crate::GLOBAL.get().unwrap_unchecked();
    let local_player_index = globals::engine().local_player_index();
    let local_player = globals::entities().get(local_player_index);

    if let Some(local_player) = local_player {
        if frame == Frame::RenderStart {
            local_player.view_angle().pitch = 89.0;
            local_player.view_angle().yaw = -270.0;
        }

        globals::set_local_player(local_player);
    }

    global.frame_stage_notify_original(this, frame);
}
