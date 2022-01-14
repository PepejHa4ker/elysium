#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Surface {
    pub name: Option<&'static spirit::Str>,
    pub properties: i16,
    pub flags: u16,
}

impl Surface {
    pub(crate) fn new() -> Self {
        Self {
            name: None,
            properties: 0,
            flags: 0,
        }
    }
}
