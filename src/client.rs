use crate::entity::EntityId;
use core::fmt;
use core::ptr::NonNull;
use daisy_chain::Chain;

#[non_exhaustive]
#[repr(C)]
pub struct ClientNetworkable;

pub type New = unsafe extern "C" fn(entity: i32, serial: i32) -> *const ClientNetworkable;
pub type NewEvent = unsafe extern "C" fn() -> *const ClientNetworkable;
pub type RecvVarProxy =
    unsafe extern "C" fn(data: *const RecvProxyData, structure: *const usize, out: *const usize);

#[repr(C)]
pub union VariantData {
    pub float: f32,
    pub int: i32,
    pub string: Option<&'static spirit::Str>,
    pub data: *const usize,
    pub vector: [f32; 3],
    pub int64: i64,
}

#[non_exhaustive]
#[repr(C)]
pub struct Variant {
    pub data: VariantData,
    pub kind: i32,
}

/*TODO WHEN#[derive(Debug)]
#[non_exhaustive]
enum Variant {
    Float(f32),
    Int(i32),
    String(Option<&'static spirit::Str),
    Data(*const usize),
    Vector(Vector),
    Int64(i64),
}*/

impl fmt::Debug for Variant {
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
    pub recv_prop: Option<&'static RecvProp>,
    pub value: Variant,
    pub element: i32,
    pub object_id: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum RecvPropKind {
    Int = 0,
    Float,
    Vector,
    VectorXY,
    String,
    Array,
    DataTable,
}

#[derive(Clone, Copy)]
#[non_exhaustive]
#[repr(C)]
pub struct RecvProp {
    pub name: Option<&'static spirit::Str>,
    pub kind: RecvPropKind,
    pub flags: i32,
    pub string_len: i32,
    pub inside_array: bool,
    pub extra_data: Option<NonNull<usize>>,
    pub array_prop: Option<NonNull<RecvProp>>,
    pub array_len_proxy: Option<NonNull<usize>>,
    pub proxy: Option<RecvVarProxy>,
    pub data_table_proxy: Option<NonNull<usize>>,
    pub data_table: Option<&'static RecvTable>,
    pub offset: i32,
    pub element_stride: i32,
    pub elements: i32,
    pub parent_array_prop_name: Option<&'static spirit::Str>,
}

impl RecvProp {
    pub fn name(&self) -> &'static str {
        self.name.map(|name| name.as_str()).unwrap_or("")
    }

    pub fn data_table(&self) -> Option<&'static RecvTable> {
        if self.kind == RecvPropKind::DataTable {
            self.data_table
        } else {
            None
        }
    }
}

impl fmt::Debug for RecvProp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RecvProp")
            .field("name", &self.name())
            .field("kind", &self.kind)
            .field("flags", &self.flags)
            .field("string_len", &self.string_len)
            .field("inside_array", &self.inside_array)
            .field("extra_data", &self.extra_data)
            .field("array_prop", &self.array_prop)
            .field("array_len_proxy", &self.array_len_proxy)
            .field("proxy", &self.proxy)
            .field("data_table_proxy", &self.data_table_proxy)
            .field("data_table", &self.data_table)
            .field("offset", &self.offset)
            .field("element_stride", &self.element_stride)
            .field("elements", &self.elements)
            .field("parent_array_prop_name", &self.parent_array_prop_name)
            .finish()
    }
}

unsafe impl Send for RecvProp {}
unsafe impl Sync for RecvProp {}

#[non_exhaustive]
#[repr(C)]
pub struct RecvTable {
    pub props: *const usize,
    pub props_len: i32,
    pub decoder: Option<NonNull<usize>>,
    pub name: Option<&'static spirit::Str>,
    pub initialized: bool,
    pub in_main_list: bool,
}

impl RecvTable {
    pub fn name(&self) -> &'static str {
        self.name.map(|name| name.as_str()).unwrap_or("")
    }

    pub fn props(&self) -> &'static [RecvProp] {
        unsafe {
            std::slice::from_raw_parts(self.props as *const RecvProp, self.props_len as usize)
        }
    }
}

impl fmt::Debug for RecvTable {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RecvTable")
            .field("props", &self.props())
            .field("decoder", &self.decoder)
            .field("name", &self.name())
            .field("initialized", &self.initialized)
            .field("in_main_list", &self.in_main_list)
            .finish()
    }
}

unsafe impl Send for RecvTable {}
unsafe impl Sync for RecvTable {}

#[non_exhaustive]
#[repr(C)]
pub struct ClientClass {
    pub new: Option<New>,
    pub new_event: Option<&'static NewEvent>,
    pub name: Option<&'static spirit::Str>,
    pub recv_table: Option<&'static RecvTable>,
    pub next: *mut ClientClass,
    pub entity_id: EntityId,
}

impl ClientClass {
    pub fn name(&self) -> &'static str {
        self.name.map(|name| name.as_str()).unwrap_or("")
    }
}

impl fmt::Debug for ClientClass {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ClientClass")
            .field("new", &self.new)
            .field("new_event", &self.new_event)
            .field("name", &self.name())
            .field("recv_table", &self.recv_table)
            .field("entity_id", &self.entity_id)
            .finish()
    }
}

unsafe impl Send for ClientClass {}
unsafe impl Sync for ClientClass {}

type Next = fn(&ClientClass) -> *mut ClientClass;

fn next(class: &ClientClass) -> *mut ClientClass {
    class.next
}

extern "C" {
    /// Raw handle to the client.
    pub type RawClient;
}

unsafe impl Send for RawClient {}
unsafe impl Sync for RawClient {}

/// The client.
#[derive(Debug)]
#[repr(transparent)]
pub struct Client(NonNull<RawClient>);

impl Client {
    pub const fn from_raw(raw: *mut RawClient) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawClient) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawClient {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const *const u8 {
        unsafe { *(self.as_ptr() as *const *const *const u8) }
    }

    pub fn activate_mouse_ptr(&self) -> *const u8 {
        unsafe { *self.virtual_table().add(16) }
    }

    // TODO: Remove Chain<ClientClass, Next>
    pub fn classes(&self) -> Chain<ClientClass, Next> {
        type Classes = unsafe extern "C" fn(this: *const RawClient) -> *mut ClientClass;

        unsafe {
            let classes =
                virt::get::<Classes>(self.virtual_table() as *const (), 64)(self.as_ptr());

            Chain::from_ptr(classes, next as Next)
        }
    }

    pub fn hud_process_input_ptr(&self) -> *const u8 {
        unsafe { *self.virtual_table().add(10) }
    }

    pub fn hud_update_ptr(&self) -> *const u8 {
        unsafe { *self.virtual_table().add(11) }
    }
}
