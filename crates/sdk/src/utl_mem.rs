#[repr(C)]
pub struct UtlMem<T> {
    pub mem: *const T,
    pub alloc_count: i32,
    pub grow_len: i32,
}
