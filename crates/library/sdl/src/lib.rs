//! Convenience wrapper around `elysium_dl::Library` for SDL methods.

use elysium_dl::Library;
use elysium_mem::address::Jmp4;
use std::fmt;

/// The SDL library.
pub struct Sdl<'a> {
    library: Library<'a>,
}

impl<'a> Sdl<'a> {
    /// Load SDL, specifically `libSDL2-2.0.so.0`.
    #[inline]
    pub fn open() -> Option<Self> {
        let library = Library::open("libSDL2-2.0.so.0")?;

        Some(Self { library })
    }

    /// Obtains the absolute address from `JMP /4` located at the `SDL_GL_SwapWindow` symbol.
    ///
    /// ```asm
    /// 0000000000037a60 <SDL_GL_SwapWindow>:
    ///    37a60: ff 25 ca fc 32 00     jmp    *0x32fcca(%rip)
    /// ```
    #[inline]
    pub unsafe fn swap_window(&self) -> Option<*const ()> {
        let symbol = self.library.symbol("SDL_GL_SwapWindow")?;
        let base_address = symbol.as_ptr();
        let jmp = symbol.as_ptr().cast::<Jmp4>().read();

        frosting::println!("resolving absolute address of `SDL_GL_SwapWindow` from JMP /4");
        frosting::println!("rip: {:0x?}", base_address.add(6));
        frosting::println!("opcode: {:0x?}", jmp.opcode());
        frosting::println!("modrm: {:0x?}", jmp.modrm());
        frosting::println!("address: {:0x?}", jmp.address());

        let address = jmp.to_absolute(base_address);

        Some(address)
    }

    /// Obtains the absolute address from the `JMP /4` located at the `SDL_PollEvent` symbol.
    ///
    /// ```asm
    /// 0000000000035eb0 <SDL_PollEvent>:
    ///     35eb0: ff 25 b2 0a 33 00     jmp    *0x330ab2(%rip)
    /// ```
    #[inline]
    pub unsafe fn poll_event(&self) -> Option<*const ()> {
        let symbol = self.library.symbol("SDL_PollEvent")?;
        let base_address = symbol.as_ptr();
        let jmp = symbol.as_ptr().cast::<Jmp4>().read();

        frosting::println!("resolving absolute address of `SDL_PollEvent` from JMP /4");
        frosting::println!("rip: {:0x?}", base_address.add(6));
        frosting::println!("opcode: {:0x?}", jmp.opcode());
        frosting::println!("modrm: {:0x?}", jmp.modrm());
        frosting::println!("address: {:0x?}", jmp.address());

        let address = jmp.to_absolute(base_address);

        Some(address)
    }
}

impl<'a> fmt::Debug for Sdl<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.library, fmt)
    }
}
