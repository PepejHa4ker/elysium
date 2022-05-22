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

#[repr(C)]
pub struct Client {
    vtable: &'static VTable,
}

impl Client {
    pub fn get_all_clases(&self) -> *const () {
        unsafe { (self.vtable.get_all_classes)(self) }
    }

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
