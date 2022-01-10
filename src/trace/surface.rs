#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Surface {
    pub name: *const spirit::Str,
    pub index: i16,
    pub flags: u16,
}

impl Surface {
    pub(crate) fn new() -> Self {
        Self {
            name: core::ptr::null::<()>() as *const _,
            index: 0,
            flags: 0,
        }
    }
}
