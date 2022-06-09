use crate::{vtable_validate, Command, View};
use frosting::ffi::vtable;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<19>,
    override_view: unsafe extern "C" fn(this: *const ClientMode, view: *const View),
    _pad1: vtable::Pad<5>,
    create_move: unsafe extern "C" fn(
        this: *const ClientMode,
        input_sample_time: f32,
        command: *mut Command,
    ) -> bool,
}

vtable_validate! {
    override_view => 19,
    create_move => 25,
}

#[repr(C)]
pub struct ClientMode {
    vtable: &'static VTable,
}

impl ClientMode {
    #[inline]
    pub fn create_move_address(&self) -> *const u8 {
        let create_move = &self.vtable.create_move
            as *const unsafe extern "C" fn(
                this: *const ClientMode,
                input_sample_time: f32,
                command: *mut Command,
            ) -> bool;

        create_move.cast()
    }

    #[inline]
    pub fn override_view_address(&self) -> *const u8 {
        let override_view = &self.vtable.override_view
            as *const unsafe extern "C" fn(this: *const ClientMode, view: *const View);

        override_view.cast()
    }
}
