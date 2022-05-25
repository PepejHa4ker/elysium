/// A growable memory structure.
///
/// Source SDK: [tier1/utlmemory.h](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/utlmemory.h)
#[doc(alias = "CUtlMemory")]
#[repr(C)]
pub struct UtlMem<T> {
    pub mem: *const T,
    pub alloc_count: i32,
    pub grow_len: i32,
}
