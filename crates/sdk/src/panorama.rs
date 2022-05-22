use super::Pad;

pub use engine::UIEngine;
pub use panel::UIPanel;
pub use panorama_engine::PanoramaUIEngine;

mod engine;
mod panel;
mod panorama_engine;

#[repr(C)]
pub struct PanoramaEventRegistration {
    pub args_len: i32,
    _pad0: Pad<4>,
    pub make_event: unsafe extern "C" fn(this: *const ()) -> *const (),
    pub create_event_from_string: unsafe extern "C" fn(
        this: *const (),
        args: *const u8,
        result: *const *const u8,
    ) -> *const (),
    _pad1: Pad<48>,
}
