use super::EntityId;
use core::fmt;
use core::ptr::NonNull;
use daisy_chain::Chain;
use vptr::Virtual;

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

#[repr(C)]
pub struct Variant {
    pub data: VariantData,
    pub kind: i32,
}

impl fmt::Debug for Variant {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Variant")
            .field("data", &"<union>")
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RecvProxyData {
    pub recv_prop: Option<&'static RecvProp>,
    pub value: Variant,
    pub element: i32,
    pub object_id: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
pub struct Client {
    this: *const (),
}

impl Client {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn get_all_classes(&self) -> Chain<ClientClass, Next> {
        type Signature = unsafe extern "C" fn(this: *const ()) -> *mut ClientClass;

        let method: Signature = unsafe { self.as_ptr().vget(8 * 8) };

        unsafe { Chain::from_ptr(method(self.as_ptr()), next as Next) }
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}
