use crate::vtable_validate;
use core::mem;
use frosting::ffi::vtable;
use frosting::option;

pub use class::Class;
pub use classes::Classes;
pub use property::Property;
pub use table::Table;

mod class;
mod classes;
mod property;
mod table;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<8>,
    get_all_classes: unsafe extern "C" fn(this: *const Client) -> *mut Class,
    _pad1: vtable::Pad<1>,
    hud_process_input: unsafe extern "C" fn(),
    hud_update: unsafe extern "C" fn(),
    _pad2: vtable::Pad<4>,
    activate_mouse: unsafe extern "C" fn(),
    _pad3: vtable::Pad<20>,
    frame_stage_notify: unsafe extern "C" fn(this: *const (), frame: i32) -> bool,
    dispatch_user_message: unsafe extern "C" fn(
        this: *const Client,
        message_kind: i32,
        passthrough_flags: i32,
        len: i32,
        data: *const (),
    ) -> bool,
}

vtable_validate! {
    get_all_classes => 8,
    hud_process_input => 10,
    hud_update => 11,
    activate_mouse => 16,
    frame_stage_notify => 37,
    dispatch_user_message => 38,
}

/// Client interface.
#[repr(C)]
pub struct Client {
    vtable: &'static VTable,
}

impl Client {
    #[inline]
    pub fn get_all_classes(&self) -> Classes<'_> {
        let classes = unsafe { (self.vtable.get_all_classes)(self) };

        Classes::new(classes)
    }

    #[inline]
    pub fn dispatch_user_message<'a, D>(
        &self,
        message_kind: i32,
        passthrough_flags: i32,
        data: D,
    ) -> bool
    where
        D: Into<Option<&'a [()]>>,
    {
        let (data, len) = option::to_raw_parts(data);

        unsafe {
            (self.vtable.dispatch_user_message)(
                self,
                message_kind,
                passthrough_flags,
                len as i32,
                data,
            )
        }
    }

    #[inline]
    pub fn client_mode(&self) -> *const u8 {
        unsafe {
            type ClientMode = unsafe extern "C" fn() -> *const u8;

            let hud_process_input = self.vtable.hud_process_input as *const u8;
            let call_client_mode = hud_process_input.byte_add(11);
            let client_mode = elysium_mem::to_absolute_with_offset(call_client_mode, 1, 5);
            let client_mode: ClientMode = mem::transmute(client_mode);

            client_mode()
        }
    }

    #[inline]
    pub fn frame_stage_notify_address(&self) -> *const u8 {
        let frame_stage_notify = &self.vtable.frame_stage_notify
            as *const unsafe extern "C" fn(this: *const (), frame: i32) -> bool;

        frame_stage_notify.cast()
    }

    #[inline]
    pub fn globals(&self) -> *const u8 {
        unsafe {
            let hud_update = self.vtable.hud_update as *const u8;
            let address = hud_update.byte_add(13);
            let globals = elysium_mem::to_absolute_with_offset(address, 3, 7)
                .cast::<*const u8>()
                .read();

            globals
        }
    }

    #[inline]
    pub fn input(&self) -> *const u8 {
        unsafe {
            let activate_mouse = self.vtable.activate_mouse as *const u8;
            let input = elysium_mem::to_absolute_with_offset(activate_mouse, 3, 7)
                .cast::<*const *const u8>()
                .read()
                .read();

            input
        }
    }
}
