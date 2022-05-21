//! Obtain addresses from instructions.

use core::{fmt, ptr};

/// Represents a `JMP /4` instruction.
///
/// # Example
///
/// ```asm
/// 0000000000037A60 <SDL_GL_SwapWindow>:
///   37A60: FF 25 CA FC 32 00     jmp    *0x32FCCA(%rip)
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C, packed)]
pub struct Jmp4 {
    // should be 0xFF
    opcode: u8,
    // ModR/M
    modrm: u8,
    // disp32
    address: i32,
}

impl Jmp4 {
    /// Construct a new `JMP /4`.
    pub fn new(address: i32) -> Self {
        Self {
            opcode: 0xFF,
            modrm: 0x25,
            address,
        }
    }

    /// Returns the absolute address jumped to relative to `rip`.
    pub unsafe fn to_absolute(&self, rip: *const ()) -> *const () {
        rip.cast::<u8>()
            .offset(self.address())
            .add(1 /* opcode */ + 5 /* modrm */)
            .cast()
    }

    /// Returns the opcode, it is always `0xFF` for `JMP /4`.
    pub fn opcode(&self) -> u8 {
        unsafe { ptr::addr_of!(self.opcode).read() }
    }

    /// Returns the ModR/M, it is always `0x25` for `JMP /4`.
    pub fn modrm(&self) -> u8 {
        unsafe { ptr::addr_of!(self.modrm).read() }
    }

    /// Returns the address.
    pub fn address(&self) -> isize {
        unsafe { ptr::addr_of!(self.address).read() as isize }
    }
}

impl fmt::Debug for Jmp4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Jmp4")
            .field("opcode", &self.opcode())
            .field("modrm", &self.modrm())
            .field("address", &self.address())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    const CODE: [u8; 6] = [0xFF, 0x25, 0xCA, 0xFC, 0x32, 0x00];

    const ADDRESS_32: i32 = i32::from_le_bytes([0xCA, 0xFC, 0x32, 0x00]);
    const ADDRESS: isize = ADDRESS_32 as isize;

    const RIP: *const () = 0x0_usize as *const ();

    #[test]
    fn address() {
        unsafe {
            let jmp = CODE.as_ptr().cast::<Jmp4>().read();

            assert_eq!(jmp.address(), ADDRESS);
        }
    }

    #[test]
    fn layout() {
        unsafe {
            let jmp = CODE.as_ptr().cast::<Jmp4>().read();

            assert_eq!(frosting::offset_of!(jmp.opcode), 0);
            assert_eq!(frosting::offset_of!(jmp.modrm), 1);
            assert_eq!(frosting::offset_of!(jmp.address), 2);
            assert_eq!(mem::size_of::<Jmp4>(), 6);
        }
    }

    #[test]
    fn reinterpret() {
        unsafe {
            let jmp = CODE.as_ptr().cast::<Jmp4>().read();

            assert_eq!(jmp, Jmp4::new(ADDRESS_32));
        }
    }

    #[test]
    fn to_absolute() {
        unsafe {
            let jmp = CODE.as_ptr().cast::<Jmp4>().read();

            assert_eq!(
                jmp.to_absolute(RIP),
                (RIP as isize + ADDRESS + 6) as *const (),
            );
        }
    }
}
