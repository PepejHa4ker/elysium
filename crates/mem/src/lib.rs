#![feature(pointer_byte_offsets)]
#![feature(strict_provenance)]

//! Memory related functions.

/// The size of a page.
pub const PAGE_SIZE: usize = 4096;

/// Mask used to obtain a page address from an arbitary address.
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

const UNPROTECTED: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

/// Creates a new pointer with the given address and size.
#[inline]
pub unsafe fn to_absolute<T>(base: *const T, addr: isize, size: usize) -> *const T {
    base.map_addr(|base| (base as isize + addr) as usize)
        .byte_add(size)
}

unsafe fn offset_of<T>(base: *const T) -> isize {
    base.cast::<i32>().read() as isize
}

/// magic
#[inline]
pub unsafe fn to_absolute_with_offset<T>(base: *const T, offset: usize, len: usize) -> *const T {
    let offset_address = base.byte_add(offset);
    let offset = offset_of(offset_address);

    base.byte_offset(offset).byte_add(len)
}

/// Set protection for the page of the given pointer.
#[inline]
pub unsafe fn protect<T>(ptr: *const T, protection: i32) {
    let page = ptr.map_addr(|addr| addr & PAGE_MASK);

    libc::mprotect(page as *mut libc::c_void, PAGE_SIZE, protection);
}

/// Disable protection for the page of the given pointer.
///
/// Convenience function for `protect(ptr, READ | WRITE | EXECUTE)`.
#[inline]
pub unsafe fn unprotect<T>(ptr: *const T) -> i32 {
    protect(ptr, UNPROTECTED);

    libc::PROT_READ | libc::PROT_EXEC
}

#[cfg(test)]
mod tests {
    const CODE: [u8; 6] = [0xFF, 0x25, 0xCA, 0xFC, 0x32, 0x00];
    const ADDRESS: isize = i32::from_le_bytes([0xCA, 0xFC, 0x32, 0x00]) as isize;

    #[test]
    fn to_absolute() {
        unsafe {
            let code = CODE.as_ptr();
            let rip = std::ptr::invalid::<u8>(0);
            let addr = code.byte_add(2).cast::<i32>().read() as isize;
            let dest = super::to_absolute(rip, addr, 6);

            assert_eq!(dest, rip.byte_offset(ADDRESS).byte_add(6));
        }
    }
}
