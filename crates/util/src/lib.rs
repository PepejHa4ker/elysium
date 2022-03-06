pub unsafe fn virtual_table(address: *const ()) -> *const () {
    *(address as *const *const ())
}

pub unsafe fn virtual_offset(address: *const (), offset: usize) -> *const () {
    (virtual_table(address) as *const *const ()).add(offset) as *const ()
}

pub unsafe fn virtual_entry<T>(address: *const (), offset: usize) -> T
where
    T: Sized,
{
    (virtual_offset(address, offset) as *const T).read()
}

pub unsafe fn relative_offset(address: *const (), offset: usize) -> *const () {
    (address as *const u8).add(offset) as *const ()
}

pub unsafe fn relative_entry<T>(address: *const (), offset: usize) -> T
where
    T: Sized,
{
    (relative_offset(address, offset) as *const T).read()
}

/// Returns the dereferenced value of `address` as an `i32`.
pub unsafe fn signed_offset_of(address: *const ()) -> isize {
    *(address as *const i32) as isize
}

pub unsafe fn to_absolute(address: *const ()) -> *const () {
    to_absolute_with_offset(address, 0, 4)
}

pub unsafe fn to_absolute_with_offset(address: *const (), offset: usize, len: usize) -> *const () {
    // We would like to do byte offsets.
    let address = address as *const u8;

    // Determine the address of the signed offset.
    let signed_offset_address = address.add(offset) as *const ();

    // Offset `address` by the signed offset and add `len` (in bytes).
    address
        .offset(signed_offset_of(signed_offset_address))
        .add(len) as *const ()
}

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

/// Determine the page an address lies on.
///
/// Assumes pages are 4,096 bytes.
pub fn page_of(address: usize) -> usize {
    address & PAGE_MASK
}

/// Set the protection of the page which this pointer lies on.
///
/// # Safety
///
/// The conditions of `Pointer::page` apply.
///
pub unsafe fn protect(ptr: *const (), protection: libc::c_int) {
    libc::mprotect(
        page_of(ptr as usize) as *mut libc::c_void,
        PAGE_SIZE,
        protection,
    );
}

/// Remove the protection of the page which this pointer lies on.
///
/// # Safety
///
/// The conditions of `Pointer::page` apply.
///
pub unsafe fn unprotect(ptr: *const ()) -> libc::c_int {
    libc::mprotect(
        page_of(ptr as usize) as *mut libc::c_void,
        PAGE_SIZE,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    );

    libc::PROT_READ | libc::PROT_EXEC
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
