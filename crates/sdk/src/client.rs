use crate::{vtable_export, vtable_validate};
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
    _pad3: vtable::Pad<8>,
    create_move:
        unsafe extern "C" fn(this: *const (), sample_time: f32, command: *const ()) -> bool,
    _pad4: vtable::Pad<11>,
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
    create_move => 25,
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
        let hud_process_input = self.vtable.hud_process_input as *const u8;
        let call_get_client_mode = unsafe { hud_process_input.byte_add(11) };

        println!("call client_mode = {:02X?}", unsafe {
            call_get_client_mode.cast::<[u8; 5]>().read()
        });

        let get_client_mode = unsafe { elysium_mem::to_absolute(call_get_client_mode, 1, 5) };
        let get_client_mode =
            unsafe { mem::transmute::<_, unsafe extern "C" fn() -> *const u8>(get_client_mode) };

        unsafe { (get_client_mode)() }
    }

    #[inline]
    pub fn create_move_address(&self) -> *const u8 {
        self.vtable.create_move as *const u8
    }

    #[inline]
    pub fn frame_stage_notify_address(&self) -> *const u8 {
        self.vtable.frame_stage_notify as *const u8
    }

    #[inline]
    pub fn globals(&self) -> *const u8 {
        let hud_update = self.vtable.hud_update as *const u8;
        let rip = unsafe { hud_update.byte_add(13) };

        println!("lea_globals = {:02X?}", unsafe {
            rip.cast::<[u8; 7]>().read()
        });

        // e5 ?? ?? | ?? ?? ?? ??
        let relative = unsafe { rip.byte_add(3).cast::<i32>().read() as isize };

        println!("relative = {relative:?}");

        let globals = unsafe { elysium_mem::to_absolute(rip, relative, 7) };

        println!("globals = {:02X?}", unsafe {
            globals.cast::<[u8; 7]>().read()
        });

        let globals = unsafe { *globals.cast::<*const u8>() };

        globals
    }

    #[inline]
    pub fn input(&self) -> *const u8 {
        let activate_mouse = self.vtable.activate_mouse as *const u8;
        let rip = unsafe { activate_mouse.byte_add(3) };

        println!("call_input = {:02X?}", unsafe {
            rip.cast::<[u8; 5]>().read()
        });

        // e5 ?? ?? ?? ??
        let relative = unsafe { rip.byte_add(1).cast::<i32>().read() as isize };
        let input = unsafe { elysium_mem::to_absolute(rip, relative, 5) };
        let input = unsafe { **input.cast::<*const *const u8>() };

        input
    }
}
