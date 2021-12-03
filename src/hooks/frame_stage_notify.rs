use crate::frame::Frame;
use crate::global::Global;

pub type Signature = unsafe extern "C" fn(this: *const (), frame: RawFrame);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RawFrame {
    Undefined = -1,
    Start = 0,
    NetUpdateStart = 1,
    NetUpdatePostDataUpdateStart = 2,
    NetUpdatePostDataUpdateEnd = 3,
    NetUpdateEnd = 4,
    RenderStart = 6,
    RenderEnd = 7,
}

pub unsafe extern "C" fn hook(this: *const (), frame: RawFrame) {
    let global = Global::handle();
    let local_player_index = global.engine().local_player_index();
    let local_player = global.entities().get(local_player_index);

    if let Some(local_player) = local_player {
        *global.local_player_ptr() = Box::new(Some(local_player));
    }

    let on_frame = &*global.on_frame_ptr();

    match frame {
        RawFrame::RenderStart => {
            on_frame(Frame::RenderStart {
                local_player: global.local_player(),
            });
        }
        _ => {}
    }

    global.frame_stage_notify_original(this, frame);
}
