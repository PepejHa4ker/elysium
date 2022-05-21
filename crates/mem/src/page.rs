//! Control page protection.

/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
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
