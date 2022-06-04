use super::Table;
use crate::{ffi, Pad};
use core::fmt;
use core::ptr::NonNull;
use elysium_math::Vec3;

#[repr(C)]
pub union VariantData {
    pub as_data: *const u8,
    pub as_f32: f32,
    pub as_i32: i32,
    pub as_i64: i64,
    pub as_vec3: Vec3,
}

#[non_exhaustive]
#[repr(C)]
pub struct Variant {
    pub data: VariantData,
    pub kind: i32,
}

impl fmt::Debug for Variant {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Variant")
            .field("data", &"<union>")
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct RecvProxyData {
    pub recv_prop: Option<&'static Property>,
    pub value: Variant,
    pub element: i32,
    pub object_id: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum PropertyKind {
    Int = 0,
    Float,
    Vector,
    VectorXY,
    String,
    Array,
    DataTable,
}

#[non_exhaustive]
#[repr(C)]
pub struct Property {
    pub name: *const u8,
    pub kind: PropertyKind,
    pub flags: i32,
    pub string_len: i32,
    pub inside_array: bool,
    _pad0: Pad<8>,
    pub array_prop: Option<NonNull<Property>>,
    _pad1: Pad<24>,
    pub data_table: Option<&'static Table>,
    pub offset: i32,
    pub element_stride: i32,
    pub elements: i32,
    pub parent_array_prop_name: *const u8,
}

impl Property {
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { ffi::str_from_ptr_nullable(self.name) }
    }

    #[inline]
    pub fn parent_array_prop_name(&self) -> &str {
        unsafe { ffi::str_from_ptr_nullable(self.parent_array_prop_name) }
    }

    #[inline]
    pub fn data_table(&self) -> Option<&'static Table> {
        if self.kind == PropertyKind::DataTable {
            self.data_table
        } else {
            None
        }
    }
}

impl fmt::Debug for Property {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Property")
            .field("name", &self.name())
            .field("kind", &self.kind)
            .field("flags", &self.flags)
            .field("string_len", &self.string_len)
            .field("inside_array", &self.inside_array)
            .field("array_prop", &self.array_prop)
            .field("data_table", &self.data_table)
            .field("offset", &self.offset)
            .field("element_stride", &self.element_stride)
            .field("elements", &self.elements)
            .field("parent_array_prop_name", &self.parent_array_prop_name)
            .finish()
    }
}
