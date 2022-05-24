use super::Property;
use crate::islice::ISlice;
use core::fmt;
use elysium_sdk::Pad;

#[non_exhaustive]
#[repr(C)]
pub struct Table {
    pub properties: ISlice<Property>,
    _pad0: Pad<8>,
    name: Option<&'static spirit::Str>,
    _pad1: Pad<2>,
}

impl Table {
    pub fn name(&self) -> &'static str {
        self.name.map(|name| name.as_str()).unwrap_or("")
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Table")
            .field("properties", &self.properties.as_slice())
            .field("name", &self.name())
            .finish()
    }
}
