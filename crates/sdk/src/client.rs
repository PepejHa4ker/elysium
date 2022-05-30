use crate::{vtable_export, vtable_validate};
use frosting::ffi::vtable;
use frosting::option;

#[repr(C)]
struct VTable {
    _unknown0: vtable::Pad<8>,
    get_all_classes: unsafe extern "C" fn(this: *const Client) -> *const (),
    _unknown1: vtable::Pad<29>,
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
    dispatch_user_message => 38,
}

/// Client interface.
#[repr(C)]
pub struct Client {
    vtable: &'static VTable,
}

impl Client {
    vtable_export! {
        get_all_classes() -> *const (),
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
}
